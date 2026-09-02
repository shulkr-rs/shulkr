use crate::util::HashMap;
use indexmap::IndexSet;
use std::hash::Hash;

struct LeveledPriorityQueue<Node> {
    level_count: usize,
    queues: Vec<IndexSet<Node>>,
    first_queued_level: usize,
}

impl<Node: Eq + Hash + Copy> LeveledPriorityQueue<Node> {
    fn new(level_count: usize) -> Self {
        Self {
            level_count,
            queues: (0..level_count).map(|_| IndexSet::new()).collect(),
            first_queued_level: level_count,
        }
    }

    fn is_empty(&self) -> bool {
        self.first_queued_level >= self.level_count
    }

    fn remove_first(&mut self) -> Node {
        let queue = &mut self.queues[self.first_queued_level];
        let node = *queue.get_index(0).expect("caller checked is_empty first");
        queue.shift_remove_index(0);
        if queue.is_empty() {
            self.check_first_queued_level(self.level_count);
        }
        node
    }

    fn dequeue(&mut self, node: Node, key: usize, upper_bound: usize) {
        let queue = &mut self.queues[key];
        queue.shift_remove(&node);
        if queue.is_empty() && self.first_queued_level == key {
            self.check_first_queued_level(upper_bound);
        }
    }

    fn enqueue(&mut self, node: Node, key: usize) {
        self.queues[key].insert(node);
        if self.first_queued_level > key {
            self.first_queued_level = key;
        }
    }

    fn check_first_queued_level(&mut self, upper_bound: usize) {
        let old_level = self.first_queued_level;
        self.first_queued_level = upper_bound;
        for i in (old_level + 1)..upper_bound {
            if !self.queues[i].is_empty() {
                self.first_queued_level = i;
                break;
            }
        }
    }
}

pub struct GraphState<Node> {
    level_count: i32,
    priority_queue: LeveledPriorityQueue<Node>,
    computed_levels: HashMap<Node, i32>,
    has_work: bool,
}

impl<Node: Eq + Hash + Copy> GraphState<Node> {
    pub fn level_count(&self) -> i32 {
        self.level_count
    }

    pub fn new(level_count: i32) -> Self {
        assert!(
            level_count < 254,
            "level count must be < 254 (matches vanilla's byte-packed computed-level storage headroom)"
        );
        Self {
            level_count,
            priority_queue: LeveledPriorityQueue::new(level_count as usize),
            computed_levels: HashMap::default(),
            has_work: false,
        }
    }
}

pub trait DynamicGraph {
    type Node: Eq + Hash + Copy;

    fn state(&mut self) -> &mut GraphState<Self::Node>;
    fn state_ref(&self) -> &GraphState<Self::Node>;

    fn is_source(&self, node: Self::Node) -> bool;

    fn get_level(&self, node: Self::Node) -> i32;
    fn set_level(&mut self, node: Self::Node, level: i32);
    fn compute_level_from_neighbor(&self, from: Self::Node, to: Self::Node, from_level: i32)
    -> i32;
    fn get_computed_level(
        &self,
        node: Self::Node,
        known_parent: Self::Node,
        known_level_from_parent: i32,
    ) -> i32;
    fn check_neighbors_after_update(&mut self, node: Self::Node, level: i32, only_decrease: bool);

    fn calculate_priority(&self, level: i32, computed_level: i32) -> i32 {
        level
            .min(computed_level)
            .min(self.state_ref().level_count - 1)
    }

    fn check_node(&mut self, node: Self::Node) {
        let level_count = self.state_ref().level_count;
        self.check_edge(node, node, level_count - 1, false);
    }

    fn check_edge(
        &mut self,
        from: Self::Node,
        to: Self::Node,
        new_level_from: i32,
        only_decreased: bool,
    ) {
        let level_to = self.get_level(to);
        let old_computed_level = self.state_ref().computed_levels.get(&to).copied();
        self.check_edge_inner(
            from,
            to,
            new_level_from,
            level_to,
            old_computed_level,
            only_decreased,
        );
        let state = self.state();
        state.has_work = !state.priority_queue.is_empty();
    }

    fn check_edge_inner(
        &mut self,
        from: Self::Node,
        to: Self::Node,
        new_level_from: i32,
        level_to: i32,
        old_computed_level: Option<i32>,
        only_decreased: bool,
    ) {
        if self.is_source(to) {
            return;
        }
        let level_count = self.state_ref().level_count;
        let new_level_from = new_level_from.clamp(0, level_count - 1);
        let level_to = level_to.clamp(0, level_count - 1);
        let was_consistent = old_computed_level.is_none();
        let old_computed_level = old_computed_level.unwrap_or(level_to);

        let new_computed_level = if only_decreased {
            old_computed_level.min(new_level_from)
        } else {
            self.get_computed_level(to, from, new_level_from)
                .clamp(0, level_count - 1)
        };

        let old_priority = self.calculate_priority(level_to, old_computed_level);
        if level_to != new_computed_level {
            let new_priority = self.calculate_priority(level_to, new_computed_level);
            if old_priority != new_priority && !was_consistent {
                self.state().priority_queue.dequeue(
                    to,
                    old_priority as usize,
                    new_priority as usize,
                );
            }
            let state = self.state();
            state.priority_queue.enqueue(to, new_priority as usize);
            state.computed_levels.insert(to, new_computed_level);
        } else if !was_consistent {
            let state = self.state();
            let level_count = state.level_count;
            state
                .priority_queue
                .dequeue(to, old_priority as usize, level_count as usize);
            state.computed_levels.remove(&to);
        }
    }

    fn check_neighbor(
        &mut self,
        from: Self::Node,
        to: Self::Node,
        level: i32,
        only_decreased: bool,
    ) {
        let level_count = self.state_ref().level_count;
        let stored_old_computed = self.state_ref().computed_levels.get(&to).copied();
        let level_from = self
            .compute_level_from_neighbor(from, to, level)
            .clamp(0, level_count - 1);

        if only_decreased {
            let level_to = self.get_level(to);
            self.check_edge_inner(
                from,
                to,
                level_from,
                level_to,
                stored_old_computed,
                only_decreased,
            );
        } else {
            let was_consistent = stored_old_computed.is_none();
            let old_computed_level = match stored_old_computed {
                Some(l) => l,
                None => self.get_level(to).clamp(0, level_count - 1),
            };

            if level_from == old_computed_level {
                let level_to = if was_consistent {
                    old_computed_level
                } else {
                    self.get_level(to)
                };
                self.check_edge_inner(
                    from,
                    to,
                    level_count - 1,
                    level_to,
                    stored_old_computed,
                    only_decreased,
                );
            }
        }
    }

    fn has_work(&self) -> bool {
        self.state_ref().has_work
    }

    fn run_updates(&mut self, mut count: i32) -> i32 {
        if self.state_ref().priority_queue.is_empty() {
            return count;
        }
        while !self.state_ref().priority_queue.is_empty() && count > 0 {
            count -= 1;
            let node = self.state().priority_queue.remove_first();
            let level_count = self.state_ref().level_count;
            let level = self.get_level(node).clamp(0, level_count - 1);
            let computed_level = self
                .state()
                .computed_levels
                .remove(&node)
                .expect("a node popped off the priority queue always has a computed level");

            if computed_level < level {
                self.set_level(node, computed_level);
                self.check_neighbors_after_update(node, computed_level, true);
            } else if computed_level > level {
                self.set_level(node, level_count - 1);
                if computed_level != level_count - 1 {
                    let priority = self.calculate_priority(level_count - 1, computed_level);
                    let state = self.state();
                    state.priority_queue.enqueue(node, priority as usize);
                    state.computed_levels.insert(node, computed_level);
                }
                self.check_neighbors_after_update(node, level, false);
            }
        }
        let state = self.state();
        state.has_work = !state.priority_queue.is_empty();
        count
    }

    fn remove_from_queue(&mut self, node: Self::Node) {
        let computed_level = self.state().computed_levels.remove(&node);
        if let Some(computed_level) = computed_level {
            let level = self.get_level(node);
            let priority = self.calculate_priority(level, computed_level);
            let state = self.state();
            let level_count = state.level_count;
            state
                .priority_queue
                .dequeue(node, priority as usize, level_count as usize);
            state.has_work = !state.priority_queue.is_empty();
        }
    }
}
