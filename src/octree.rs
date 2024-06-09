use glam::Vec3;
use num_traits::One;

use crate::types::{Index, Vector3D};

struct Octree<V: Vector3D = Vec3, I: Index = u32, const B: usize = 32> {
    pub(crate) stems: Vec<StemNode<I>>,
    pub(crate) leafs: Vec<LeafNode<V, I, B>>,
    pub(crate) center: V,
    pub(crate) size: <V as Vector3D>::Scalar,
}

struct StemNode<I: Index> {
    pub(crate) parent: I,
    pub(crate) childs: [I; 8],
}

struct LeafNode<V: Vector3D, I: Index, const B: usize> {
    pub(crate) points: [V; B],
    pub(crate) parent: I,
    pub(crate) size: I,
}

impl<V: Vector3D, I: Index, const B: usize> Default for Octree<V, I, B>
where
    V::Scalar: One,
{
    fn default() -> Self {
        Self {
            stems: Default::default(),
            leafs: Default::default(),
            center: V::ZERO,
            size: V::Scalar::one(),
        }
    }
}

