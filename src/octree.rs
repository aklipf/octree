use std::io::empty;

use glam::Vec3;
use num_traits::One;

use crate::types::Index;

#[derive(Debug)]
pub(crate) struct Node<I: Index> {
    pub(crate) data: I,
    pub(crate) size: I,
}

#[derive(Debug)]
pub(crate) struct PointsBlock<const B: usize> {
    pub(crate) points: [Vec3; B],
}

#[derive(Debug)]
pub(crate) struct NodeBlock<I: Index> {
    pub(crate) parent: I,
    pub(crate) nodes: [Node<I>; 8],
}

#[derive(Debug)]
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
                data: I::node_idx(0, 0),
                size: I::zero(),
            },
            nodes: vec![NodeBlock {
                parent: I::root(),
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
        self.add_to(I::root().to_tuple(), point, self.center, self.size);
    }

    pub fn len(&self) -> usize {
        self.root.size.into()
    }

    #[inline]
    fn add_to(
        &mut self,
        node_idx: (usize, usize),
        point: Vec3,
        center: Vec3,
        size: f32,
    ) -> (usize, usize) {
        let node: &mut Node<I> = self.mut_node(node_idx);
        node.size = node.size + I::one();

        let mut center: Vec3 = center;
        let mut size: f32 = size;

        let node_idx = self.locate_increment(node_idx, point, &mut center, &mut size);
        self.insert(node_idx, point, center, size)
    }

    fn locate_increment(
        &mut self,
        node_idx: (usize, usize),
        point: Vec3,
        center: &mut Vec3,
        size: &mut f32,
    ) -> (usize, usize) {
        let mut current: &mut Node<I> = self.mut_node(node_idx);
        let mut child_idx: usize = 0;
        let mut block_idx: usize = 0;

        while !current.data.is_leaf() {
            child_idx = Self::branch(point, center, size);
            block_idx = current.data.node_block();
            current = self.mut_node((block_idx, child_idx));
            current.size = current.size + I::one();
        }

        (block_idx, child_idx)
    }

    fn insert(
        &mut self,
        node_idx: (usize, usize),
        point: Vec3,
        center: Vec3,
        size: f32,
    ) -> (usize, usize) {
        let points_len: usize = self.points.len();
        let node: &mut Node<I> = self.mut_node(node_idx);
        let idx: usize = node.size.into();

        if node.data.is_empty() {
            node.data = I::points_idx(points_len);
            let mut leaf_points: PointsBlock<B> = Default::default();
            leaf_points.points[0] = point;
            self.points.push(leaf_points);

            return node_idx;
        }

        if idx <= B {
            let points_idx = node.data.points_block();
            self.points[points_idx].points[idx - 1] = point;

            return node_idx;
        }

        self.subdivide(node_idx, center, size);
        self.add_to(node_idx, point, center, size)
    }

    fn subdivide(&mut self, node_idx: (usize, usize), center: Vec3, size: f32) {
        let new_block = self.nodes.len();
        let node: &mut Node<I> = self.mut_node(node_idx);

        // backup point informations
        let points_block: usize = node.data.points_block();

        // setup the node with childs
        node.data = I::node_idx(new_block, 0);
        node.size = I::zero();
        self.nodes.push(NodeBlock {
            parent: I::node_idx(node_idx.0, node_idx.1),
            nodes: Default::default(),
        });

        // insert the previous points into the new nodes
        let mut last_node_idx: (usize, usize) = (0, 0);
        for point in self.points[points_block].points {
            let current_idx: (usize, usize) = self.add_to(node_idx, point, center, size);
            let current_data = self.node(current_idx).data;
            if (!current_data.is_empty()) && current_data.points_block() == (self.points.len() - 1)
            {
                last_node_idx = current_idx;
            }
        }

        // free useless PointsBlock points_block
        self.points[points_block] = self.points.pop().unwrap();

        let node: &mut Node<I> = self.mut_node(last_node_idx);
        node.data = I::points_idx(points_block);
    }

    #[inline]
    fn node(&self, idx: (usize, usize)) -> &Node<I> {
        if idx.0 == usize::root().node_block() {
            return &self.root;
        }

        &self.nodes[idx.0].nodes[idx.1]
    }

    #[inline]
    fn mut_node(&mut self, idx: (usize, usize)) -> &mut Node<I> {
        if idx.0 == usize::root().node_block() {
            return &mut self.root;
        }

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
    use glam::{vec3, Vec3};

    use crate::types::Index;

    use super::Octree;

    #[test]
    fn octree_default() {
        let tree: Octree = Octree::default();

        assert_eq!(tree.size, 2.0);
        assert_eq!(tree.center, Vec3::ZERO);
        assert_eq!(tree.points.len(), 0);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.nodes[0].parent, usize::root());
        for i in 0..8 {
            assert_eq!(tree.nodes[0].nodes[i].size, 0);
            assert_eq!(tree.nodes[0].nodes[i].data, usize::empty());
        }
        assert_eq!(tree.root.size, 0);
        assert_eq!(tree.root.data, usize::node_idx(0, 0));
    }

    #[test]
    fn octree_add() {
        let mut tree: Octree = Octree::default();

        tree.add(vec3(0.0, 0.0, 0.0));
    }
}
