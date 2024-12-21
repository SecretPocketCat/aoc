use std::collections::{BinaryHeap, HashMap, HashSet};

use glam::{IVec2, UVec2};
use iter::grid_iter;
use tracing::warn;

pub const DIRS_4: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];

#[derive(PartialEq, Eq, Debug)]
struct WeightedPoint {
    coords: UVec2,
    priority: u32,
}
impl WeightedPoint {
    fn new(coords: UVec2, priority: u32) -> Self {
        Self { coords, priority }
    }
}
impl PartialOrd for WeightedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for WeightedPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reversed order to use a min heap
        self.priority.cmp(&other.priority).reverse()
    }
}

mod iter {
    use glam::UVec2;

    pub struct GridIterator {
        grid_size: UVec2,
        index: UVec2,
    }
    impl Iterator for GridIterator {
        type Item = UVec2;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index.y >= self.grid_size.y {
                None
            } else {
                let next = self.index;
                self.index.x += 1;
                if self.index.x == self.grid_size.x {
                    self.index = (0, self.index.y + 1).into();
                }
                Some(next)
            }
        }
    }

    pub fn grid_iter(grid_size: impl Into<UVec2>) -> GridIterator {
        GridIterator {
            grid_size: grid_size.into(),
            index: UVec2::ZERO,
        }
    }

    #[cfg(test)]
    mod tests {
        use glam::UVec2;

        use super::grid_iter;

        #[test]
        fn iter() {
            let size = UVec2::new(5, 3);
            let tiles: Vec<_> = grid_iter(size).collect();
            assert_eq!(
                tiles,
                [
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (0, 1),
                    (1, 1),
                    (2, 1),
                    (3, 1),
                    (4, 1),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                ]
                .map(Into::into)
            );
        }
    }
}

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
    pub fn neighbours(&self, tile: UVec2) -> Vec<UVec2> {
        DIRS_4
            .iter()
            .filter_map(|d| self.move_target(tile, *d).map(|(c, _)| c))
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

    /// a* impl is based on [this redblobgames article](https://www.redblobgames.com/pathfinding/a-star/implementation.html#python-astar)
    // todo: might wanna try [Jump point search](https://harablog.wordpress.com/2011/09/07/jump-point-search/) - a symmetry pruning optimization for a* when pathfinding on uniform-cost grids
    #[must_use]
    pub fn find_path_astar(
        &self,
        start: impl Into<UVec2>,
        end: impl Into<UVec2>,
    ) -> Option<Vec<UVec2>> {
        let start = start.into();
        let end = end.into();
        if !self.walkable_tiles.contains_key(&start) || !self.walkable_tiles.contains_key(&end) {
            return None;
        }
        if start == end {
            return Some(vec![start]);
        }
        let mut frontier = BinaryHeap::new();
        frontier.push(WeightedPoint::new(start, 0));
        let capacity = (self.size.element_product() / 4).max(8) as usize;
        let mut came_from = HashMap::with_capacity(capacity);
        came_from.insert(start, None);
        let mut cost = HashMap::with_capacity(capacity);
        cost.insert(start, 0);
        while let Some(current) = frontier.pop() {
            warn!(?current);
            if current.coords == end {
                let mut from = current.coords;
                let mut res = vec![from];
                while let Some(Some(prev)) = came_from.get(&from) {
                    res.push(*prev);
                    from = *prev;
                }
                res.reverse();
                return Some(res);
            }

            let current_cost = cost.get(&current.coords).copied().unwrap_or_default();
            for neighbour_coords in self.neighbours(current.coords) {
                let neighbour_cost = current_cost + 1;
                if cost
                    .get(&neighbour_coords)
                    // cost doesn't yet exist or is lower than the stored one
                    .map_or(true, |current_cost| neighbour_cost < *current_cost)
                {
                    cost.insert(neighbour_coords, neighbour_cost);
                    let prio = neighbour_cost + neighbour_coords.manhattan_distance(end);
                    frontier.push(WeightedPoint::new(neighbour_coords, prio));
                    came_from.insert(neighbour_coords, Some(current.coords));
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

pub trait UVec2Ext {
    fn manhattan_distance(&self, other: UVec2) -> u32;
}
impl UVec2Ext for UVec2 {
    #[must_use]
    fn manhattan_distance(&self, other: UVec2) -> u32 {
        (self.as_ivec2() - other.as_ivec2()).abs().element_sum() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    #[test_case(UVec2::ONE, UVec2::ONE => 0)]
    #[test_case(UVec2::ZERO, UVec2::ONE => 2)]
    #[test_case(UVec2::ONE, UVec2::ZERO => 2)]
    #[test_case(UVec2::new(1, 2), UVec2::new(2, 1) => 2)]
    #[traced_test]
    fn manhattan(a: UVec2, b: UVec2) -> u32 {
        a.manhattan_distance(b)
    }

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
