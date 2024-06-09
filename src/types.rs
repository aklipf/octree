use glam::{DVec3, Vec3};
use num_traits::{PrimInt, Unsigned};

pub trait Vector3D {
    type Scalar;
    const ZERO: Self;
}

impl Vector3D for Vec3 {
    type Scalar = f32;
    const ZERO: Self = Self::ZERO;
}
impl Vector3D for DVec3 {
    type Scalar = f64;
    const ZERO: Self = Self::ZERO;
}

pub trait Index: PrimInt + Unsigned {
    type IDX;

    fn is_leaf(&self) -> bool;
    fn mask(&self) -> Self::IDX;
    fn index(&self) -> Self::IDX;
}

impl<T: PrimInt + Unsigned> Index for T {
    type IDX = T;

    #[inline]
    fn mask(&self) -> Self::IDX {
        T::max_value() >> 1
    }

    #[inline]
    fn is_leaf(&self) -> bool {
        *self > self.mask()
    }

    #[inline]
    fn index(&self) -> Self::IDX {
        *self & self.mask()
    }
}

#[cfg(test)]
mod tests {
    use super::Index;

    #[test]
    fn check_is_leaf() {
        assert_eq!(0u8.is_leaf(), false);
        assert_eq!(127u8.is_leaf(), false);
        assert_eq!(128u8.is_leaf(), true);
        assert_eq!(255u8.is_leaf(), true);

        assert_eq!(0u16.is_leaf(), false);
        assert_eq!(32767u16.is_leaf(), false);
        assert_eq!(32768u16.is_leaf(), true);
        assert_eq!(65535u16.is_leaf(), true);

        assert_eq!(0u32.is_leaf(), false);
        assert_eq!(2147483647u32.is_leaf(), false);
        assert_eq!(2147483648u32.is_leaf(), true);
        assert_eq!(4294967295u32.is_leaf(), true);

        assert_eq!(0u64.is_leaf(), false);
        assert_eq!(9223372036854775807u64.is_leaf(), false);
        assert_eq!(9223372036854775808u64.is_leaf(), true);
        assert_eq!(18446744073709551615u64.is_leaf(), true);
    }

    #[test]
    fn check_mask() {
        assert_eq!(0u8.mask(), (1u8 << 7) - 1);
        assert_eq!(0u16.mask(), (1u16 << 15) - 1);
        assert_eq!(0u32.mask(), (1u32 << 31) - 1);
        assert_eq!(0u64.mask(), (1u64 << 63) - 1);
    }

    #[test]
    fn check_index() {
        assert_eq!(0u8.index(), 0u8);
        assert_eq!(127u8.index(), 127u8);
        assert_eq!(128u8.index(), 0u8);
        assert_eq!(255u8.index(), 127u8);

        assert_eq!(0u16.index(), 0u16);
        assert_eq!(32767u16.index(), 32767u16);
        assert_eq!(32768u16.index(), 0u16);
        assert_eq!(65535u16.index(), 32767u16);

        assert_eq!(0u32.index(), 0u32);
        assert_eq!(2147483647u32.index(), 2147483647u32);
        assert_eq!(2147483648u32.index(), 0u32);
        assert_eq!(4294967295u32.index(), 2147483647u32);

        assert_eq!(0u64.index(), 0u64);
        assert_eq!(9223372036854775807u64.index(), 9223372036854775807u64);
        assert_eq!(9223372036854775808u64.index(), 0u64);
        assert_eq!(18446744073709551615u64.index(), 9223372036854775807u64);
    }
}
