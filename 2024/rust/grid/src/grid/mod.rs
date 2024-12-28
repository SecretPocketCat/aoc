use glam::{IVec2, UVec2};
use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt, hash,
};

use crate::{dir::DIRS_4, iter::grid_iter, UVec2Ext};

pub mod builder;

#[derive(PartialEq, Eq, Debug)]
struct OrderedNode<T: hash::Hash + fmt::Debug = UVec2> {
    node: T,
    priority: u32,
}
impl<T: hash::Hash + fmt::Debug> OrderedNode<T> {
    fn new(node: T, priority: u32) -> Self {
        Self { node, priority }
    }
}
impl<T: hash::Hash + Eq + fmt::Debug> PartialOrd for OrderedNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: hash::Hash + Eq + fmt::Debug> Ord for OrderedNode<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // reversed order to use a min heap
        self.priority.cmp(&other.priority).reverse()
    }
}

#[derive(Debug)]
pub struct NodePath<TNode> {
    pub path: Vec<TNode>,
    pub cost: u32,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Neigbour {
    pub tile: UVec2,
    pub direction: IVec2,
}
impl Neigbour {
    #[must_use]
    pub fn new(tile: UVec2, direction: IVec2) -> Self {
        Self { tile, direction }
    }
}

#[derive(Debug)]
pub struct Grid<T = ()> {
    size: UVec2,
    walkable_tiles: HashMap<UVec2, T>,
}
impl<T> Grid<T> {
    pub fn new(walkable_tiles: impl Into<HashMap<UVec2, T>>, size: impl Into<UVec2>) -> Self {
        Self {
            walkable_tiles: walkable_tiles.into(),
            size: size.into(),
        }
    }

    pub fn from_walkable_tiles<IntoV, IntoT>(
        walkable_tiles: impl Iterator<Item = (IntoV, IntoT)>,
        size: impl Into<UVec2>,
    ) -> Self
    where
        IntoV: Into<UVec2>,
        IntoT: Into<T>,
    {
        Self {
            walkable_tiles: walkable_tiles
                .map(|(coords, val)| (coords.into(), val.into()))
                .collect(),
            size: size.into(),
        }
    }

    #[must_use]
    pub fn size(&self) -> UVec2 {
        self.size
    }

    #[must_use]
    pub fn walkable_tiles(&self) -> &HashMap<UVec2, T> {
        &self.walkable_tiles
    }

    #[must_use]
    pub fn move_target(&self, pos: UVec2, dir: IVec2) -> Option<(UVec2, &T)> {
        let target = pos.as_ivec2() + dir;
        if !self.within_bounds(target) {
            return None;
        }
        let target = target.as_uvec2();
        self.walkable_tiles.get(&target).map(|c| (target, c))
    }

    #[must_use]
    pub fn move_tile(pos: UVec2, dir: IVec2) -> IVec2 {
        pos.as_ivec2() + dir
    }

    #[must_use]
    pub fn move_within_bounds(&self, pos: UVec2, dir: IVec2) -> bool {
        let target = Self::move_tile(pos, dir);
        self.within_bounds(target)
    }

    #[must_use]
    pub fn within_bounds(&self, tile: IVec2) -> bool {
        tile.min_element() >= 0 && tile.x < self.size.x as _ && tile.y < self.size.y as _
    }

    #[must_use]
    pub fn neighbours(&self, tile: UVec2) -> Vec<Neigbour> {
        DIRS_4
            .iter()
            .filter_map(|d| {
                self.move_target(tile, *d)
                    .map(|(c, _)| Neigbour::new(c, *d))
            })
            .collect()
    }

    #[must_use]
    pub fn obstacle_neighbours(&self, tile: UVec2) -> Vec<Neigbour> {
        DIRS_4
            .iter()
            .filter_map(|d| {
                let target = Self::move_tile(tile, *d);
                if self.move_within_bounds(tile, *d) {
                    Some(Neigbour::new(target.as_uvec2(), *d))
                } else {
                    None
                }
            })
            .collect()
    }

    #[must_use]
    pub fn find_path_astar(
        &self,
        start: impl Into<UVec2>,
        end: impl Into<UVec2>,
    ) -> Option<Vec<UVec2>> {
        let start = start.into();
        let end = end.into();
        Self::find_path_astar_with_successors(
            start,
            &end,
            |node| self.neighbours(*node).into_iter().map(|n| (n.tile, 1)),
            |node| node.manhattan_distance(end),
            |n| *n == end,
        )
        .map(|node_path| node_path.path)
    }

    /// a* impl is based on [this redblobgames article](https://www.redblobgames.com/pathfinding/a-star/implementation.html#python-astar)
    // todo: might wanna try [Jump point search](https://harablog.wordpress.com/2011/09/07/jump-point-search/) - a symmetry pruning optimization for a* when pathfinding on uniform-cost grids
    #[must_use]
    pub fn find_path_astar_with_successors<
        TNode,
        TFnSuccessors,
        TSuccessors,
        TFNHeurestic,
        TFnReachedEnd,
    >(
        start: TNode,
        end: &TNode,
        mut successors: TFnSuccessors,
        mut heuristic: TFNHeurestic,
        mut reached_end: TFnReachedEnd,
    ) -> Option<NodePath<TNode>>
    where
        TNode: Eq + fmt::Debug + hash::Hash + Clone,
        TFnSuccessors: FnMut(&TNode) -> TSuccessors,
        TSuccessors: IntoIterator<Item = (TNode, u32)>,
        TFNHeurestic: FnMut(&TNode) -> u32,
        TFnReachedEnd: FnMut(&TNode) -> bool,
    {
        if &start == end {
            return Some(NodePath {
                path: vec![start],
                cost: 0,
            });
        }
        let mut frontier = BinaryHeap::new();
        frontier.push(OrderedNode::new(start.clone(), 0));
        let mut came_from: HashMap<TNode, Option<TNode>> = HashMap::new();
        came_from.insert(start.clone(), None);
        let mut cost = HashMap::new();
        cost.insert(start, 0);
        while let Some(current) = frontier.pop() {
            tracing::trace!(?current);
            let current_cost = cost.get(&current.node).copied().unwrap_or_default();
            if reached_end(&current.node) {
                let mut from = &current.node;
                let mut res = vec![from.clone()];
                while let Some(Some(prev)) = came_from.get(from) {
                    res.push((*prev).clone());
                    from = prev;
                }
                res.reverse();
                return Some(NodePath {
                    path: res,
                    cost: current_cost,
                });
            }
            for (succesor_node, mut successor_cost) in successors(&current.node) {
                successor_cost += current_cost;
                if cost
                    .get(&succesor_node)
                    // cost doesn't yet exist or is lower than the stored one
                    .map_or(true, |stored_successor_cost| {
                        successor_cost < *stored_successor_cost
                    })
                {
                    let prio = successor_cost + heuristic(&succesor_node);
                    frontier.push(OrderedNode::new(succesor_node.clone(), prio));
                    came_from.insert(succesor_node.clone(), Some(current.node.clone()));
                    cost.insert(succesor_node, successor_cost);
                }
            }
        }
        None
    }
}
impl<T: Default> Grid<T> {
    pub fn from_size(size: impl Into<UVec2>) -> Self {
        let size = size.into();
        let walkable_tiles = grid_iter(size).map(|c| (c, T::default())).collect();
        Self {
            size,
            walkable_tiles,
        }
    }

    pub fn from_obstacles(obstacles: impl Into<HashSet<UVec2>>, size: impl Into<UVec2>) -> Self {
        let size = size.into();
        let obstacles = obstacles.into();
        let walkable_tiles = grid_iter(size)
            .filter(|c| !obstacles.contains(c))
            .map(|c| (c, T::default()))
            .collect();
        Self {
            size,
            walkable_tiles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    #[test_case(UVec2::ZERO, UVec2::ZERO => Some(vec![UVec2::ZERO]))]
    #[test_case(UVec2::ZERO, UVec2::ONE => None)]
    #[test_case(UVec2::ONE, UVec2::ZERO => None)]
    #[test_case(UVec2::ONE, UVec2::ONE => None)]
    #[test_case(UVec2::ZERO, UVec2::X => Some(vec![UVec2::ZERO, UVec2::X]))]
    #[test_case(UVec2::ZERO, UVec2::splat(2) => Some(vec![
        (0,0).into(),
        (1,0).into(),
        (2,0).into(),
        (2,1).into(),
        (2,2).into(),
    ]))]
    #[test_case(UVec2::splat(2), UVec2::ZERO => Some(vec![
        (2,2).into(),
        (2,1).into(),
        (2,0).into(),
        (1,0).into(),
        (0,0).into(),
    ]))]
    #[traced_test]
    fn astar(start: UVec2, end: UVec2) -> Option<Vec<UVec2>> {
        let grid = Grid::<()>::from_obstacles([UVec2::ONE], (3, 3));
        grid.find_path_astar(start, end)
    }
}
