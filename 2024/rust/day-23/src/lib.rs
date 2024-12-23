pub mod solution {
    use std::collections::{HashMap, HashSet};

    #[derive(Default, Debug)]
    struct Edges<'a>(HashMap<&'a str, HashSet<&'a str>>);
    impl<'a> Edges<'a> {
        fn push_edge(&mut self, a: &'a str, b: &'a str) {
            self.0
                .entry(a)
                .and_modify(|edges| {
                    edges.insert(b);
                })
                .or_insert_with(|| [b].into());
        }

        fn add_undirected_edge(&mut self, a: &'a str, b: &'a str) {
            self.push_edge(a, b);
            self.push_edge(b, a);
        }

        fn neighbours(&self, node: &str) -> &HashSet<&'a str> {
            &self.0[node]
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let (all_edges, search_nodes) = input.lines().fold(
            (Edges::default(), HashSet::<&str>::new()),
            |(mut edges, mut search_nodes), l| {
                let (a, b) = l.split_once('-').expect("Valid edge");
                edges.add_undirected_edge(a, b);
                search_nodes.extend([a, b].iter().filter(|n| n.starts_with('t')));
                (edges, search_nodes)
            },
        );
        let all_edges = &all_edges;
        let lan_count: HashSet<_> = search_nodes
            .into_iter()
            .flat_map(|n| {
                let node_edges = all_edges.neighbours(n);
                node_edges.iter().flat_map(move |n1| {
                    all_edges.neighbours(n1).iter().filter_map(move |n2| {
                        if node_edges.contains(n2) {
                            let mut nodes = [n, n1, n2];
                            nodes.sort_unstable();
                            Some(nodes)
                        } else {
                            None
                        }
                    })
                })
            })
            .collect();
        Ok(lan_count.len().to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "7";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_23_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_23_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
