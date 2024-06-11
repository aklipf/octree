use glam::{DVec3, Vec3};
use num_traits::{Float, PrimInt, Unsigned};

pub trait Vector3D
where
    Self::Scalar: Float,
{
    type Scalar;
    const ZERO: Self;
}

impl Vector3D for Vec3 {
    type Scalar = f32;
    const ZERO: Self = Vec3::ZERO;
}
impl Vector3D for DVec3 {
    type Scalar = f64;
    const ZERO: Self = DVec3::ZERO;
}

pub trait Index: PrimInt + Unsigned + Into<usize> {
    fn mask() -> Self;
    fn empty() -> Self;
    fn root() -> Self;
    fn node_idx(block: usize, child: usize) -> Self;
    fn points_idx(block: usize) -> Self;
    fn child(&self) -> usize;
    fn node_block(&self) -> usize;
    fn points_block(&self) -> usize;
    fn to_tuple(&self) -> (usize, usize);
    fn is_leaf(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_root(&self) -> bool;
}

impl Index for usize {
    #[inline]
    fn mask() -> Self {
        !(Self::max_value() << 3)
    }

    #[inline]
    fn empty() -> Self {
        Self::max_value()
    }

    #[inline]
    fn root() -> Self {
        Self::max_value() >> 1
    }

    #[inline]
    fn node_idx(block: usize, child: usize) -> Self {
        (block << 3) | child
    }

    #[inline]
    fn points_idx(block: usize) -> Self {
        block | (!(Self::max_value() >> 1))
    }

    #[inline]
    fn child(&self) -> usize {
        *self & Self::mask()
    }

    #[inline]
    fn node_block(&self) -> usize {
        *self >> 3
    }

    #[inline]
    fn points_block(&self) -> usize {
        *self & (Self::max_value() >> 1)
    }

    #[inline]
    fn to_tuple(&self) -> (usize, usize) {
        (self.node_block(), self.child())
    }

    #[inline]
    fn is_leaf(&self) -> bool {
        (*self & (!(Self::max_value() >> 1))) != 0usize
    }

    #[inline]
    fn is_empty(&self) -> bool {
        *self == Self::empty()
    }

    #[inline]
    fn is_root(&self) -> bool {
        *self == Self::root()
    }
}

#[cfg(test)]
mod tests {
    use super::Index;

    #[test]
    fn test_child() {
        assert_eq!(0usize.child(), 0usize);
        assert_eq!(4usize.child(), 4usize);
        assert_eq!(7usize.child(), 7usize);
        assert_eq!(16usize.child(), 0usize);
        assert_eq!(2980usize.child(), 4usize);
        assert_eq!(6031usize.child(), 7usize);
    }

    #[test]
    fn test_mask() {
        assert_eq!(usize::mask(), 7);
    }

    #[test]
    fn test_block() {
        assert_eq!(0.node_block(), 0);
        assert_eq!(4.node_block(), 0);
        assert_eq!(7.node_block(), 0);
        assert_eq!(16.node_block(), 2);
        assert_eq!(2980.node_block(), 372);
        assert_eq!(6031.node_block(), 753);
    }

    #[test]
    fn test_is_empty() {
        assert_eq!(0.is_empty(), false);
        assert_eq!((usize::max_value() >> 1).is_empty(), false);
        assert_eq!(((usize::max_value() >> 1) + 1).is_empty(), false);
        assert_eq!(usize::max_value().is_empty(), true);
    }

    #[test]
    fn test_is_leaf() {
        assert_eq!(0.is_leaf(), false);
        assert_eq!((usize::max_value() >> 1).is_leaf(), false);
        assert_eq!(((usize::max_value() >> 1) + 1).is_leaf(), true);
        assert_eq!(usize::max_value().is_leaf(), true);
    }
}
