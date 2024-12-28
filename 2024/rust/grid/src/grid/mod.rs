use glam::{IVec2, UVec2};
use pathfinding::directed::astar::astar;
use std::collections::{HashMap, HashSet};

use crate::{dir::DIRS_4, iter::grid_iter, UVec2Ext};

pub mod builder;

#[derive(Debug)]
pub struct NodePath<TNode> {
    pub path: Vec<TNode>,
    pub cost: u32,
}

#[derive(Debug, Default)]
pub struct NodePaths<TNode> {
    pub paths: Vec<Vec<TNode>>,
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
        astar(
            &start,
            |node| self.neighbours(*node).into_iter().map(|n| (n.tile, 1)),
            |node| node.manhattan_distance(end),
            |n| *n == end,
        )
        .map(|node_path| node_path.0)
    }

    pub fn print_debug_map<TFnFormatWalkable: FnMut(UVec2) -> Option<char>>(
        &self,
        mut format_walkable: TFnFormatWalkable,
    ) {
        let size = self.size() + UVec2::ONE;
        let mut dbg_map = String::with_capacity(size.element_product() as _);
        let x_axis = (0..size.x)
            .map(|i| (i % 10).to_string())
            .collect::<String>();
        dbg_map.push_str(&format!("  {}\n", &x_axis));
        dbg_map.push_str(" 0");
        let mut prev_y = 0;
        for tile in grid_iter(size) {
            if tile.y != prev_y {
                prev_y = tile.y;
                dbg_map.push_str(&format!("{:2}", tile.y - 1));
                dbg_map.push('\n');
                dbg_map.push_str(&format!("{:2}", tile.y));
            }
            if self.walkable_tiles().contains_key(&tile) {
                dbg_map.push(format_walkable(tile).unwrap_or('.'));
            } else {
                dbg_map.push('#');
            }
        }
        dbg_map.push_str(&format!("\n {}", &x_axis));
        println!("{dbg_map}");
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
