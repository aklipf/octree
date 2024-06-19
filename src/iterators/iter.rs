use glam::Vec3;

use crate::octree::{AsPoint, Octree};

pub struct TreeIterator<'a, P: AsPoint + Clone, N: Default> {
    pub(crate) tree: &'a Octree<P, N>,
    pub(crate) stack: Vec<(u32, Vec3, f32)>,
}

pub struct IterStemNode {
    pub(crate) node_idx: u32,
    pub center: Vec3,
    pub size: f32,
}

pub struct IterLeafNode {
    pub(crate) node_idx: u32,
    pub center: Vec3,
    pub size: f32,
}

pub enum IterNode {
    Stem(IterStemNode),
    Leaf(IterLeafNode),
}
