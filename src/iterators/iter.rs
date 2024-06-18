use glam::Vec3;

use crate::octree::{AsPoint, Octree};

use super::subdivide::Subdivide;

pub struct TreeIterator<'a, P: AsPoint + Clone, N: Default> {
    pub(crate) tree: &'a Octree<P, N>,
    pub(crate) stack: Vec<(u32, Vec3, f32)>,
}

pub struct StemNode<'a, P: AsPoint + Clone, N: Default> {
    pub(crate) tree: &'a Octree<P, N>,
    pub(crate) node_idx: u32,
    pub(crate) center: Vec3,
    pub(crate) size: f32,
}

pub struct LeafNode<'a, P: AsPoint + Clone, N: Default> {
    pub(crate) tree: &'a Octree<P, N>,
    pub(crate) node_idx: u32,
    pub(crate) center: Vec3,
    pub(crate) size: f32,
}

pub enum Node<'a, P: AsPoint + Clone, N: Default> {
    Stem(StemNode<'a, P, N>),
    Leaf(LeafNode<'a, P, N>),
}
