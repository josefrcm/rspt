use geometry::*;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

const NODE_SIZE : usize = 4;

#[derive(Clone)]
pub enum Node<T> {
    Empty,
    Leaf(T),
    Branch(Box<BVH<T>>)
}


///
/// Recursive bounding volume hierarchy
#[derive(Clone)]
pub struct BVH<T> {
    bounds: [BoundingBox; NODE_SIZE],
    children: [Node<T>; NODE_SIZE]
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Build the whole world geometry
impl<T: Clone> BVH<T> {
    ///
    /// An empty tree
    pub fn empty() -> Self {
        BVH {
            bounds: [BoundingBox::empty(); NODE_SIZE],
            children: [Node::Empty, Node::Empty, Node::Empty, Node::Empty]
        }
    }


    ///
    /// Constructor
    pub fn build(elements: &[(T, BoundingBox)]) -> Self {
        let leaves = Self::build_leaves(elements);
        Self::build_branches(&leaves)
    }


    ///
    /// First stage: turn the leaves into nodes
    fn build_leaves(elements: &[(T, BoundingBox)]) -> Vec<Box<BVH<T>>> {
        elements.chunks(NODE_SIZE).map(|c| {
            let mut leaf = Self::empty();
            for i in 0..c.len() {
                leaf.children[i] = Node::Leaf(c[i].0.clone());
                leaf.bounds[i] = c[i].1;
            }
            Box::new(leaf)
        }).collect()
    }


    ///
    /// Second stage: recursively merge the nodes into bigger nodes
    fn build_branches(elements: &[Box<BVH<T>>]) -> Self {
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



///
/// Ray intersection
impl<T: Intersectable> Intersectable for BVH<T> {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // Check each child in order
        let mut nearest_hit = Intersection::empty();
        for i in 0..self.bounds.len() {
            let intersection = self.bounds[i].intersect(ray);
            if intersection.start < nearest_hit.distance {
                match &self.children[i] {
                    Node::Empty => {},
                    Node::Leaf(x) => {
                        match x.intersect(ray) {
                            Some(hit) => {
                                if hit.distance < nearest_hit.distance {
                                    nearest_hit = hit;
                                }
                            },
                            None => {}
                        }
                    },
                    Node::Branch(xs) => {
                        match xs.intersect(ray) {
                            Some(hit) => {
                                if hit.distance < nearest_hit.distance {
                                    nearest_hit = hit;
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }

        // If a triangle was hit, compute the intersection parameters: coordinates, normal, material, etc.
        if nearest_hit.distance.is_infinite() {
            None
        } else {
            Some(nearest_hit)
        }
    }
}
