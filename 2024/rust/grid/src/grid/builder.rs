use std::{collections::HashSet, marker::PhantomData, str::Lines};

use bon::bon;
use derive_more::derive::{Display, Error};
use glam::UVec2;

use super::Grid;

pub struct BuiltGrid<T = ()> {
    pub grid: Grid<T>,
    pub start_tile: Option<UVec2>,
    pub end_tile: Option<UVec2>,
}

#[derive(Debug, Error, Display)]
pub enum BuildGridError {
    EmptyGrid,
    MissingStartTile,
    #[display("Multiple start tiles: {_0} and {_1}")]
    MultipleStartTiles(UVec2, UVec2),
    MissingEndTile,
    #[display("Multiple end tiles: {_0} and {_1}")]
    MultipleEndTiles(UVec2, UVec2),
}

pub struct GridBuilder<TValue = (), TProcessCtx = ()>(
    PhantomData<TValue>,
    PhantomData<TProcessCtx>,
);

#[bon]
impl<TValue: Default> GridBuilder<TValue> {
    #[builder]
    pub fn build_obstacle_grid(
        input: &str,
        obstacle: char,
        start_character: Option<char>,
        end_character: Option<char>,
    ) -> Result<BuiltGrid<TValue>, BuildGridError> {
        Self::build_obstacle_grid_from_lines_impl(
            &mut input.lines(),
            obstacle,
            &mut (),
            None,
            start_character,
            end_character,
        )
    }

    #[builder]
    pub fn build_obstacle_grid_from_lines(
        lines: &mut Lines<'_>,
        obstacle: char,
        start_character: Option<char>,
        end_character: Option<char>,
    ) -> Result<BuiltGrid<TValue>, BuildGridError> {
        Self::build_obstacle_grid_from_lines_impl(
            lines,
            obstacle,
            &mut (),
            None,
            start_character,
            end_character,
        )
    }
}

#[bon]
impl<TValue: Default, TProcessCtx> GridBuilder<TValue, TProcessCtx> {
    #[builder]
    pub fn build_obstacle_grid_from_lines_with_processing<'a>(
        lines: &'_ mut Lines<'a>,
        obstacle: char,
        process_ctx: &mut TProcessCtx,
        process_tile: fn(&mut TProcessCtx, char, UVec2) -> bool,
        start_character: Option<char>,
        end_character: Option<char>,
    ) -> Result<BuiltGrid<TValue>, BuildGridError> {
        Self::build_obstacle_grid_from_lines_impl(
            lines,
            obstacle,
            process_ctx,
            Some(process_tile),
            start_character,
            end_character,
        )
    }

    fn build_obstacle_grid_from_lines_impl<'a>(
        lines: &'_ mut Lines<'a>,
        obstacle: char,
        process_ctx: &mut TProcessCtx,
        mut process_tile: Option<fn(&mut TProcessCtx, char, UVec2) -> bool>,
        start_character: Option<char>,
        end_character: Option<char>,
    ) -> Result<BuiltGrid<TValue>, BuildGridError> {
        let mut obstacles = HashSet::new();
        let mut size = UVec2::ZERO;
        let mut start_tile = None;
        let mut end_tile = None;
        for (y, line) in lines.take_while(|l| !l.is_empty()).enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = UVec2::new(x as _, y as _);
                size = tile;
                if process_tile
                    .as_mut()
                    .is_some_and(|process_tile| process_tile(process_ctx, c, tile))
                {
                    continue;
                }
                if c == obstacle {
                    obstacles.insert(tile);
                } else if start_character.is_some_and(|s| c == s) {
                    if let Some(start_tile) = start_tile {
                        return Err(BuildGridError::MultipleStartTiles(start_tile, tile));
                    }
                    start_tile = Some(tile);
                } else if end_character.is_some_and(|e| c == e) {
                    if let Some(end_tile) = end_tile {
                        return Err(BuildGridError::MultipleEndTiles(end_tile, tile));
                    }
                    end_tile = Some(tile);
                }
            }
        }
        if size == UVec2::ZERO {
            return Err(BuildGridError::EmptyGrid);
        }
        if start_character.is_some() && start_tile.is_none() {
            return Err(BuildGridError::MissingStartTile);
        }
        if end_character.is_some() && end_tile.is_none() {
            return Err(BuildGridError::MissingEndTile);
        }
        Ok(BuiltGrid {
            grid: Grid::from_obstacles(obstacles, size),
            start_tile,
            end_tile,
        })
    }
}
