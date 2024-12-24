pub mod solution {
    use anyhow::Context;
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Edge<'a>(&'a str, &'a str);

    #[derive(Default, Debug)]
    struct Graph<'a> {
        edges: HashSet<Edge<'a>>,
        neighbours: HashMap<&'a str, HashSet<&'a str>>,
        search_nodes: HashSet<&'a str>,
    }
    impl<'a> Graph<'a> {
        fn push_edge(&mut self, a: &'a str, b: &'a str) {
            self.neighbours
                .entry(a)
                .and_modify(|edges| {
                    edges.insert(b);
                })
                .or_insert_with(|| [b].into());
            self.edges.insert(Edge(a, b));
        }

        fn add_undirected_edge(&mut self, a: &'a str, b: &'a str) {
            self.push_edge(a, b);
            self.push_edge(b, a);
        }

        fn neighbours(&self, node: &str) -> &HashSet<&'a str> {
            &self.neighbours[node]
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let all_edges = parse_edges(input);
        let all_edges = &all_edges;
        let lan_count: HashSet<_> = all_edges
            .search_nodes
            .iter()
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
        let all_edges = parse_edges(input);
        let mut max = 0;
        let mut res = None;
        'search_nodes: for n in all_edges
            .neighbours
            .keys()
            .sorted_unstable_by_key(|n| all_edges.neighbours(n).len())
        {
            let neighbours = all_edges.neighbours(n);
            if neighbours.len() <= max {
                break;
            }
            for i in ((max + 1)..=neighbours.len()).rev() {
                if let Some(mut complete_subgraph) =
                    neighbours.iter().combinations(i).find(|subset| {
                        subset
                            .iter()
                            .tuple_combinations::<(_, _)>()
                            .all(|(a, b)| all_edges.edges.contains(&Edge(a, b)))
                    })
                {
                    max = complete_subgraph.len();
                    complete_subgraph.push(n);
                    res = Some(complete_subgraph.iter().sorted_unstable().join(","));
                    continue 'search_nodes;
                }
            }
        }
        res.context("Found valid subgraph")
    }

    fn parse_edges(input: &str) -> Graph {
        input.lines().fold(Graph::default(), |mut edges, l| {
            let (a, b) = l.split_once('-').expect("Valid edge");
            edges.add_undirected_edge(a, b);
            edges
                .search_nodes
                .extend([a, b].iter().filter(|n| n.starts_with('t')));
            edges
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "7";
    const EXPECTED_B: &str = "co,de,ka,ta";

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
