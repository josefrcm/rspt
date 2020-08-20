use super::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

const NODE_SIZE: usize = 4;

///
/// A node of the BVH
/// TODO: use some kind of tagged box to reduce storage requirements
#[derive(Clone)]
pub enum Node {
    Empty,
    Leaf(Box<TriangleBundle>),
    Branch(Box<BVH>),
    //Instance(Box<nalgebra::Transform3<f32>>, Box<BVH>),
}

///
/// Recursive bounding volume hierarchy
#[derive(Clone)]
pub struct BVH {
    bounds: [AABB; NODE_SIZE],
    children: [Node; NODE_SIZE],
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Tree construction
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Build the whole world geometry
impl BVH {
    ///
    /// An empty tree
    pub fn empty() -> Self {
        BVH {
            bounds: [AABB::empty(); NODE_SIZE],
            children: [Node::Empty, Node::Empty, Node::Empty, Node::Empty],
        }
    }

    ///
    /// Constructor
    pub fn build_mesh(elements: &[(TriangleBundle, AABB)]) -> Self {
        let leaves = Self::build_leaves(elements);
        Self::build_branches(&leaves)
    }

    /*///
    /// Constructor
    pub fn build_world(elements: &[Mesh]) -> Self {
        let leaves = Self::build_leaves(elements);
        Self::build_branches(&leaves)
        let mut bundles = Vec::new();
        for m in meshes {
            let b = m.bounds();
            bundles.push((m, b));
        }
    }*/

    ///
    /// First stage: turn the leaves into nodes
    fn build_leaves(elements: &[(TriangleBundle, AABB)]) -> Vec<Box<BVH>> {
        elements
            .chunks(NODE_SIZE)
            .map(|c| {
                let mut leaf = Self::empty();
                for i in 0..c.len() {
                    leaf.children[i] = Node::Leaf(Box::new(c[i].0.clone()));
                    leaf.bounds[i] = c[i].1;
                }
                Box::new(leaf)
            })
            .collect()
    }

    ///
    /// Second stage: recursively merge the nodes into bigger nodes
    fn build_branches(elements: &[Box<BVH>]) -> Self {
        // First case: an empty tree
        if elements.len() == 0 {
            Self::empty()
        }
        // Second case: a tree with just one leaf
        else if elements.len() == 1 {
            (*elements[0]).clone()
        }
        // Third case: merge the leaves into groups of NODE_SIZE elements
        else {
            let mut leaves = Vec::new();
            for c in elements.chunks(NODE_SIZE) {
                let mut node = Self::empty();
                for i in 0..c.len() {
                    node.children[i] = Node::Branch(c[i].clone());
                    node.bounds[i] = union(&c[i].bounds);
                }
                leaves.push(Box::new(node));
            }
            Self::build_branches(&leaves)
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Tree traversal
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Ray intersection
impl BVH {
    pub fn intersect(&self, ray: Ray) -> BundleIntersection {
        // Check each child in order
        let mut nearest_hit = BundleIntersection::empty();
        for i in 0..self.bounds.len() {
            let intersection = self.bounds[i].intersect(ray);
            if intersection.start < nearest_hit.distance {
                match &self.children[i] {
                    Node::Empty => {}
                    Node::Leaf(bundle) => {
                        let hit = bundle.intersect(ray);
                        if hit.distance < nearest_hit.distance {
                            nearest_hit = hit;
                        }
                    }
                    Node::Branch(tree) => {
                        let hit = tree.intersect(ray);
                        if hit.distance < nearest_hit.distance {
                            nearest_hit = hit;
                        }
                    }
                    /*Node::Instance(transform, tree) => {
                        let hit = tree.intersect(ray);
                        if hit.distance < nearest_hit.distance {
                            nearest_hit = hit;
                        }
                    }*/
                }
            }
        }

        // If a triangle was hit, compute the intersection parameters: coordinates, normal, material, etc.
        nearest_hit
    }
}
