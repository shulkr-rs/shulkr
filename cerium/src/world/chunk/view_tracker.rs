use super::{ChunkGraph, ChunkGraphSource, DynamicGraph};
use std::collections::HashMap;

struct ViewSource {
    levels: HashMap<(i32, i32), i32>,
    players_per_chunk: HashMap<(i32, i32), u32>,
    max_level: i32,
    view_distance: i32,
    touched: std::collections::HashSet<(i32, i32)>,
}

impl ChunkGraphSource for ViewSource {
    fn get_level(&self, pos: (i32, i32)) -> i32 {
        *self.levels.get(&pos).unwrap_or(&self.max_level)
    }

    fn set_level(&mut self, pos: (i32, i32), level: i32) {
        if level >= self.max_level {
            self.levels.remove(&pos);
        } else {
            self.levels.insert(pos, level);
        }
        self.touched.insert(pos);
    }

    fn get_level_from_source(&self, pos: (i32, i32)) -> i32 {
        if self.players_per_chunk.get(&pos).copied().unwrap_or(0) > 0 {
            0
        } else {
            self.max_level
        }
    }
}

pub struct ViewTracker {
    graph: ChunkGraph<ViewSource>,
    reported_visible: std::collections::HashSet<(i32, i32)>,
}

impl ViewTracker {
    pub fn new(view_distance: i32, max_level: i32) -> Self {
        Self {
            graph: ChunkGraph::new(
                max_level + 1,
                ViewSource {
                    levels: HashMap::new(),
                    players_per_chunk: HashMap::new(),
                    max_level,
                    view_distance,
                    touched: std::collections::HashSet::new(),
                },
            ),
            reported_visible: std::collections::HashSet::new(),
        }
    }

    pub fn add_player(&mut self, pos: (i32, i32)) {
        *self.graph.source.players_per_chunk.entry(pos).or_insert(0) += 1;
        self.graph.update(pos, 0, true);
    }

    pub fn remove_player(&mut self, pos: (i32, i32)) {
        if let Some(count) = self.graph.source.players_per_chunk.get_mut(&pos) {
            *count -= 1;
            if *count == 0 {
                self.graph.source.players_per_chunk.remove(&pos);
            }
        }
        let max_level = self.graph.source.max_level;
        self.graph.update(pos, max_level, false);
    }

    pub fn move_player(&mut self, old: (i32, i32), new: (i32, i32)) {
        if old != new {
            self.remove_player(old);
            self.add_player(new);
        }
    }

    pub fn run_updates(&mut self) -> Vec<((i32, i32), bool)> {
        self.graph.source.touched.clear();
        self.graph.run_updates(i32::MAX);

        let view_distance = self.graph.source.view_distance;
        let mut changes = Vec::new();
        for pos in std::mem::take(&mut self.graph.source.touched) {
            let is_visible = self.graph.source.get_level(pos) <= view_distance;
            let was_visible = self.reported_visible.contains(&pos);
            if was_visible != is_visible {
                changes.push((pos, is_visible));
                if is_visible {
                    self.reported_visible.insert(pos);
                } else {
                    self.reported_visible.remove(&pos);
                }
            }
        }
        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_entering_reveals_chunks_within_view_distance() {
        let mut tracker = ViewTracker::new(4, 40);
        tracker.add_player((0, 0));
        let changes = tracker.run_updates();

        let visible: Vec<_> = changes.into_iter().filter(|&(_, visible)| visible).map(|(p, _)| p).collect();
        assert!(visible.contains(&(0, 0)));
        assert!(visible.contains(&(4, 0)));
        assert!(!visible.contains(&(5, 0)), "chunk beyond view distance should not become visible");
        assert_eq!(visible.len(), 9 * 9, "a 4-radius Chebyshev square is 9x9 chunks");
    }

    #[test]
    fn moving_out_of_range_hides_previously_visible_chunks_and_reveals_new_ones() {
        let mut tracker = ViewTracker::new(2, 40);
        tracker.add_player((0, 0));
        tracker.run_updates();

        tracker.move_player((0, 0), (20, 0));
        let changes = tracker.run_updates();

        let hidden: Vec<_> = changes.iter().filter(|&&(_, visible)| !visible).map(|&(p, _)| p).collect();
        let shown: Vec<_> = changes.iter().filter(|&&(_, visible)| visible).map(|&(p, _)| p).collect();

        assert!(hidden.contains(&(0, 0)), "old position should be hidden after moving far away");
        assert!(shown.contains(&(20, 0)), "new position should become visible");
        assert!(!shown.contains(&(0, 0)), "old position should not also appear as newly shown");
    }

    #[test]
    fn two_players_near_each_other_keep_the_area_visible_until_both_leave() {
        let mut tracker = ViewTracker::new(2, 40);
        tracker.add_player((0, 0));
        tracker.add_player((1, 0));
        tracker.run_updates();

        tracker.remove_player((0, 0));
        let changes = tracker.run_updates();
        assert!(
            !changes.iter().any(|&(p, visible)| p == (0, 0) && !visible),
            "chunk (0,0) should stay visible: player at (1,0) is still within range of it"
        );
    }
}
