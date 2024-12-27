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
