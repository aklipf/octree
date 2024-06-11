use glam::Vec3;
use num_traits::One;

use crate::types::Index;

pub(crate) struct Node<I: Index> {
    pub(crate) data: I,
    pub(crate) size: I,
}

pub(crate) struct PointsBlock<const B: usize> {
    pub(crate) points: [Vec3; B],
}

pub(crate) struct NodeBlock<I: Index> {
    pub(crate) parent: I,
    pub(crate) nodes: [Node<I>; 8],
}

pub struct Octree<I: Index = usize, const B: usize = 5> {
    pub(crate) root: Node<I>,
    pub(crate) nodes: Vec<NodeBlock<I>>,
    pub(crate) points: Vec<PointsBlock<B>>,
    pub(crate) center: Vec3,
    pub(crate) size: f32,
}

impl<I: Index> Default for Node<I> {
    fn default() -> Self {
        Node::<I> {
            data: I::empty(),
            size: I::zero(),
        }
    }
}

impl<const B: usize> Default for PointsBlock<B> {
    fn default() -> Self {
        PointsBlock::<B> {
            points: [Vec3::ZERO; B],
        }
    }
}

impl<I: Index, const B: usize> Default for Octree<I, B> {
    fn default() -> Self {
        Self {
            root: Node {
                data: I::idx(0, 0),
                size: I::zero(),
            },
            nodes: vec![NodeBlock {
                parent: I::empty(),
                nodes: Default::default(),
            }],
            points: Default::default(),
            center: Vec3::ZERO,
            size: 2.0,
        }
    }
}

impl<I: Index, const B: usize> Octree<I, B> {
    pub fn add(&mut self, point: Vec3) {
        let (node_idx, center, size) = self.locate_node(point);
        self.insert(node_idx, point, center, size);
    }

    fn locate_node(&self, point: Vec3) -> ((usize, usize), Vec3, f32) {
        let mut current: &Node<I> = &self.root;
        let mut child_idx: usize = 0;
        let mut block_idx: usize = 0;
        let mut center: Vec3 = self.center;
        let mut size: f32 = self.size;

        while !current.data.is_leaf() {
            child_idx = Self::branch(point, &mut center, &mut size);
            block_idx = current.data.node_block();
            current = self.node((block_idx, child_idx));
        }

        ((block_idx, child_idx), center, size)
    }

    fn insert(&mut self, node_idx: (usize, usize), point: Vec3, center: Vec3, size: f32) {
        let node: &mut Node<I> = self.mut_node(node_idx);

        if node.data.is_empty() {
            node.size = I::one();

            let mut leaf_points: PointsBlock<B> = Default::default();
            leaf_points.points[0] = point;
            self.points.push(leaf_points);
        } else if node.size.into() < B {
            let new_size = node.size + I::one();
            node.size = new_size;

            let points_idx = node.data.points_block();
            self.points[points_idx].points[new_size.into()] = point;
        }
    }

    #[inline]
    fn node(&self, idx: (usize, usize)) -> &Node<I> {
        &self.nodes[idx.0].nodes[idx.1]
    }

    #[inline]
    fn mut_node(&mut self, idx: (usize, usize)) -> &mut Node<I> {
        &mut self.nodes[idx.0].nodes[idx.1]
    }

    #[inline]
    fn branch(point: Vec3, center: &mut Vec3, size: &mut f32) -> usize {
        let diff = point - *center;
        *center += (*size * 0.25) * diff.signum();
        *size *= 0.5;

        diff.is_negative_bitmask() as usize
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::types::Index;

    use super::Octree;

    #[test]
    fn octree_default() {
        let tree: Octree = Octree::default();

        assert_eq!(tree.size, 2.0);
        assert_eq!(tree.center, Vec3::ZERO);
        assert_eq!(tree.points.len(), 0);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.nodes[0].parent, usize::empty());
        for i in 0..8 {
            assert_eq!(tree.nodes[0].nodes[i].size, 0);
            assert_eq!(tree.nodes[0].nodes[i].data, usize::empty());
        }
        assert_eq!(tree.root.size, 0);
        assert_eq!(tree.root.data, usize::idx(0, 0));
    }
}
