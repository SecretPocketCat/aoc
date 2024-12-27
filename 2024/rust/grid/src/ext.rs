use glam::UVec2;

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
}
