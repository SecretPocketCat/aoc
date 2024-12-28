pub mod dir;
mod ext;
mod grid;
pub mod iter;

pub use ext::*;
pub use grid::*;

pub mod prelude {
    pub use glam::{IVec2, UVec2};

    pub use crate::dir::*;
    pub use crate::ext::*;
    pub use crate::grid::builder::*;
    pub use crate::grid::*;
    pub use crate::iter::*;

    pub mod pathfinding {
        pub use pathfinding::directed::astar::{astar, astar_bag};
    }
}
