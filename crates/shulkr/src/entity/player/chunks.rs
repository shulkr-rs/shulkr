use super::Player;
use crate::{
    entity::EntityLike,
    protocol::packet::{
        ChunkBatchFinishedPacket, ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket,
        UnloadChunkPacket,
    },
    util::{HashMap, HashSet},
    world::chunk::Chunk,
};
use std::{sync::atomic::Ordering, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TrackState {
    Pending,
    Viewing,
}

type TrackedDiff = (Vec<(i32, i32)>, Vec<(i32, i32)>);

fn diff_tracked(
    tracked: &mut HashMap<(i32, i32), TrackState>,
    desired: &HashSet<(i32, i32)>,
) -> TrackedDiff {
    let mut added = Vec::new();
    for &pos in desired {
        if let std::collections::hash_map::Entry::Vacant(entry) = tracked.entry(pos) {
            entry.insert(TrackState::Pending);
            added.push(pos);
        }
    }
    let stale = tracked
        .keys()
        .copied()
        .filter(|pos| !desired.contains(pos))
        .collect();
    (added, stale)
}

pub(super) type SyncChunk = Chunk;

pub struct ChunkQueue {
    pub queue: Vec<SyncChunk>,
    pub target_cpt: f32,
    pub pending_chunks: f32,
    pub max_lead: i32,
    pub lead: i32,
}

impl ChunkQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            target_cpt: 9.,
            pending_chunks: 0.,
            max_lead: 1,
            lead: 0,
        }
    }

    pub fn enqueue(&mut self, chunk: SyncChunk) {
        self.queue.push(chunk);
    }

    pub fn cancel(&mut self, cx: i32, cz: i32) -> bool {
        let before = self.queue.len();
        self.queue
            .retain(|chunk| chunk.x() != cx || chunk.z() != cz);
        self.queue.len() != before
    }

    pub fn sort_by_distance_to(&mut self, from: (i32, i32)) {
        self.queue.sort_by_key(|chunk| {
            let dx = chunk.x() - from.0;
            let dz = chunk.z() - from.1;
            std::cmp::Reverse(dx * dx + dz * dz)
        });
    }

    pub fn dequeue(&mut self) -> Option<SyncChunk> {
        self.queue.pop()
    }
}

impl Player {
    pub(super) fn process_view_changes(&self) {
        let center = Chunk::to_chunk_pos(self.position());
        let view_distance = self.view_distance();

        {
            let mut last_view = self.0.last_view.lock();
            if *last_view == Some((center, view_distance)) {
                return;
            }
            *last_view = Some((center, view_distance));
        }

        let desired: HashSet<(i32, i32)> = Chunk::chunks_in_range(center, view_distance)
            .into_iter()
            .collect();

        let (added, stale) = diff_tracked(&mut self.0.tracked_chunks.lock(), &desired);

        for pos in stale {
            self.untrack_chunk(pos.0, pos.1);
        }

        if !added.is_empty() {
            self.0.pending_loads.lock().extend(added);
            self.0.pending_notify.notify_one();
        }

        self.ensure_load_dispatcher();
    }

    fn view_distance(&self) -> i32 {
        self.0.connection.view_distance()
    }

    pub(super) fn untrack_chunk(&self, cx: i32, cz: i32) {
        let Some(state) = self.0.tracked_chunks.lock().remove(&(cx, cz)) else {
            return;
        };

        if state == TrackState::Pending {
            self.0.pending_loads.lock().remove(&(cx, cz));
            return;
        }

        if !self.0.chunk_queue.lock().cancel(cx, cz) {
            self.send_packet(&UnloadChunkPacket {
                chunk_x: cx,
                chunk_z: cz,
            });
        }
        self.world().remove_viewer(cx, cz);
    }

    fn send_chunk(&self, chunk: SyncChunk) {
        let mut queue = self.0.chunk_queue.lock();
        queue.enqueue(chunk);
    }

    pub(super) fn send_pending_chunks(&self) {
        let mut queue = self.0.chunk_queue.lock();

        if queue.queue.is_empty() || queue.lead >= queue.max_lead {
            return;
        }

        let per_tick = queue.target_cpt;
        queue.pending_chunks = (queue.pending_chunks + per_tick).min(per_tick.max(1.));
        if queue.pending_chunks < 1. {
            return;
        }

        let quota = queue.pending_chunks as usize;
        let batch_size = if self.is_local() {
            queue.queue.len()
        } else {
            queue.queue.len().min(quota)
        };

        let center = Chunk::to_chunk_pos(self.position());
        queue.sort_by_distance_to(center);

        self.send_packet(&ChunkBatchStartPacket {});

        let mut sent = 0;
        while sent < batch_size
            && let Some(chunk) = queue.dequeue()
        {
            let packet: ChunkDataAndUpdateLightPacket = (&chunk).into();
            self.send_packet(&packet);
            sent += 1;
        }

        queue.pending_chunks -= sent as f32;
        self.send_packet(&ChunkBatchFinishedPacket {
            batch_size: sent as i32,
        });
        queue.lead += 1;
    }

    fn is_local(&self) -> bool {
        self.addr().ip().is_loopback()
    }

    fn take_nearest_pending(&self) -> Option<(i32, i32)> {
        let (center_x, center_z) = Chunk::to_chunk_pos(self.position());
        let mut pending = self.0.pending_loads.lock();
        let nearest = pending.iter().copied().min_by_key(|&(cx, cz)| {
            let (dx, dz) = ((cx - center_x) as i64, (cz - center_z) as i64);
            dx * dx + dz * dz
        })?;
        pending.remove(&nearest);
        Some(nearest)
    }

    pub fn set_view_distance(&self, view_distance: i32) {
        self.0.connection.set_view_distance(view_distance);
    }

    fn ensure_load_dispatcher(&self) {
        if self.0.dispatcher_started.swap(true, Ordering::AcqRel) {
            return;
        }

        let this = self.clone();
        tokio::spawn(async move {
            while !this.0.connection.closed() {
                let notified = this.0.pending_notify.notified();

                let Some(pos) = this.take_nearest_pending() else {
                    let _ = tokio::time::timeout(Duration::from_millis(500), notified).await;
                    continue;
                };

                let permit = this.world().acquire_load_permit().await;

                let this = this.clone();
                tokio::spawn(async move {
                    let world = this.world();
                    let chunk = loop {
                        let chunk = world.load_chunk_async(pos.0, pos.1).await;

                        let mut tracked = this.0.tracked_chunks.lock();
                        if !matches!(tracked.get(&pos), Some(TrackState::Pending)) {
                            return;
                        }

                        if world.add_viewer(pos.0, pos.1) {
                            tracked.insert(pos, TrackState::Viewing);
                            break chunk;
                        }
                    };

                    drop(permit);
                    this.send_chunk(chunk);
                });
            }
        });
    }
}
