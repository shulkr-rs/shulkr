use super::graph::{DynamicGraph, GraphState};

pub trait ChunkGraphSource {
    fn get_level(&self, pos: (i32, i32)) -> i32;
    fn set_level(&mut self, pos: (i32, i32), level: i32);
    fn get_level_from_source(&self, pos: (i32, i32)) -> i32;
}

pub struct ChunkGraph<S: ChunkGraphSource> {
    state: GraphState<Option<(i32, i32)>>,
    pub source: S,
}

impl<S: ChunkGraphSource> ChunkGraph<S> {
    pub fn new(level_count: i32, source: S) -> Self {
        Self {
            state: GraphState::new(level_count),
            source,
        }
    }

    pub fn update(&mut self, pos: (i32, i32), new_level_from: i32, only_decreased: bool) {
        self.check_edge(None, Some(pos), new_level_from, only_decreased);
    }
}

fn neighbors(pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    (-1..=1).flat_map(move |dx| (-1..=1).map(move |dz| (pos.0 + dx, pos.1 + dz)))
}

impl<S: ChunkGraphSource> DynamicGraph for ChunkGraph<S> {
    type Node = Option<(i32, i32)>;

    fn state(&mut self) -> &mut GraphState<Self::Node> {
        &mut self.state
    }

    fn state_ref(&self) -> &GraphState<Self::Node> {
        &self.state
    }

    fn is_source(&self, node: Self::Node) -> bool {
        node.is_none()
    }

    fn get_level(&self, node: Self::Node) -> i32 {
        match node {
            None => 0,
            Some(pos) => self.source.get_level(pos),
        }
    }

    fn set_level(&mut self, node: Self::Node, level: i32) {
        if let Some(pos) = node {
            self.source.set_level(pos, level);
        }
    }

    fn compute_level_from_neighbor(
        &self,
        from: Self::Node,
        to: Self::Node,
        from_level: i32,
    ) -> i32 {
        match from {
            None => self.get_level_from_source_node(to),
            Some(_) => from_level + 1,
        }
    }

    fn get_computed_level(
        &self,
        node: Self::Node,
        known_parent: Self::Node,
        known_level_from_parent: i32,
    ) -> i32 {
        let Some(pos) = node else {
            return known_level_from_parent;
        };
        let mut computed = known_level_from_parent;
        for neighbor_pos in neighbors(pos) {
            let neighbor = if neighbor_pos == pos {
                None
            } else {
                Some(neighbor_pos)
            };
            if neighbor != known_parent {
                let cost =
                    self.compute_level_from_neighbor(neighbor, node, self.get_level(neighbor));
                computed = computed.min(cost);
                if computed == 0 {
                    return 0;
                }
            }
        }
        computed
    }

    fn check_neighbors_after_update(&mut self, node: Self::Node, level: i32, only_decrease: bool) {
        let level_count = self.state_ref().level_count();
        if !only_decrease || level < level_count - 2 {
            let Some(pos) = node else { return };
            for neighbor_pos in neighbors(pos) {
                if neighbor_pos != pos {
                    self.check_neighbor(node, Some(neighbor_pos), level, only_decrease);
                }
            }
        }
    }
}

impl<S: ChunkGraphSource> ChunkGraph<S> {
    fn get_level_from_source_node(&self, node: Option<(i32, i32)>) -> i32 {
        match node {
            Some(pos) => self.source.get_level_from_source(pos),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::HashMap;

    struct FixedSource {
        levels: HashMap<(i32, i32), i32>,
        source_pos: (i32, i32),
        max_level: i32,
    }

    impl ChunkGraphSource for FixedSource {
        fn get_level(&self, pos: (i32, i32)) -> i32 {
            *self.levels.get(&pos).unwrap_or(&self.max_level)
        }

        fn set_level(&mut self, pos: (i32, i32), level: i32) {
            if level >= self.max_level {
                self.levels.remove(&pos);
            } else {
                self.levels.insert(pos, level);
            }
        }

        fn get_level_from_source(&self, pos: (i32, i32)) -> i32 {
            if pos == self.source_pos {
                0
            } else {
                self.max_level
            }
        }
    }

    fn chebyshev(a: (i32, i32), b: (i32, i32)) -> i32 {
        (a.0 - b.0).abs().max((a.1 - b.1).abs())
    }

    #[test]
    fn single_source_propagates_as_chebyshev_rings() {
        let max_level = 12;
        let mut graph = ChunkGraph::new(
            max_level + 1,
            FixedSource {
                levels: HashMap::default(),
                source_pos: (0, 0),
                max_level,
            },
        );

        graph.update((0, 0), 0, false);
        graph.run_updates(i32::MAX);

        for radius in 0..max_level {
            for dz in -radius..=radius {
                for dx in -radius..=radius {
                    let pos = (dx, dz);
                    let expected = chebyshev((0, 0), pos);
                    if expected <= max_level {
                        assert_eq!(
                            graph.source.get_level(pos),
                            expected,
                            "chunk {pos:?} should be at level {expected} (Chebyshev distance from source)"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn moving_the_source_updates_incrementally() {
        let max_level = 8;
        let mut graph = ChunkGraph::new(
            max_level + 1,
            FixedSource {
                levels: HashMap::default(),
                source_pos: (0, 0),
                max_level,
            },
        );

        graph.update((0, 0), 0, false);
        graph.run_updates(i32::MAX);
        assert_eq!(graph.source.get_level((3, 0)), 3);

        graph.source.source_pos = (5, 5);
        graph.update((0, 0), max_level, false);
        graph.update((5, 5), 0, false);
        graph.run_updates(i32::MAX);

        assert_eq!(graph.source.get_level((5, 5)), 0);
        assert_eq!(graph.source.get_level((7, 5)), 2);
        assert_eq!(graph.source.get_level((0, 0)), 5);

        graph.update((20, 20), 0, false);
        graph.run_updates(i32::MAX);
        graph.update((20, 20), max_level, false);
        graph.run_updates(i32::MAX);
        assert!(!graph.source.levels.contains_key(&(20, 20)));
    }

    #[test]
    fn bounded_run_updates_makes_partial_progress_without_blocking() {
        let max_level = 20;
        let mut graph = ChunkGraph::new(
            max_level + 1,
            FixedSource {
                levels: HashMap::default(),
                source_pos: (0, 0),
                max_level,
            },
        );

        graph.update((0, 0), 0, false);

        let mut calls = 0;
        while graph.has_work() {
            graph.run_updates(4);
            calls += 1;
            assert!(
                calls < 10_000,
                "did not converge in a reasonable number of bounded calls"
            );
        }
        assert!(
            calls > 1,
            "should have taken more than one call with a tiny budget"
        );

        assert_eq!(graph.source.get_level((3, 3)), 3);
        assert_eq!(graph.source.get_level((10, 0)), 10);
    }
}
