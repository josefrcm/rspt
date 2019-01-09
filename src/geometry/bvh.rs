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
    bounds: [super::BoundingBox; NODE_SIZE],
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
            bounds: [super::BoundingBox::empty(); NODE_SIZE],
            children: [Node::Empty, Node::Empty, Node::Empty, Node::Empty]
        }
    }


    ///
    /// Constructor
    pub fn build(elements: &[(T, super::BoundingBox)]) -> Self {
        let leaves = Self::build_leaves(elements);
        Self::build_branches(&leaves)
    }


    ///
    /// First stage: turn the leaves into nodes
    fn build_leaves(elements: &[(T, super::BoundingBox)]) -> Vec<Box<BVH<T>>> {
        let mut leaves = Vec::new();
        for c in elements.chunks(NODE_SIZE) {
            let mut new_node = Self::empty();
            let mut i = 0;
            for e in c {
                new_node.children[i] = Node::Leaf(e.0.clone());
                new_node.bounds[i] = e.1;
                i = i+1;
            }
            leaves.push(Box::new(new_node));
        }

        /*for c in elements.chunks(NODE_SIZE) {
            let mut new_node = Self::empty();
            let mut i = 0;
            for e in c {
                new_node.children[i] = Node::Leaf(e.0);
                new_node.bounds[i] = e.1;
                i = i+1;
            }
            leaves.push(Box::new(new_node));
        }*/

        leaves
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
                    node.bounds[i] = c[i].bounds[0].merge(&c[i].bounds[1]).merge(&c[i].bounds[2]).merge(&c[i].bounds[3]);
                }
                leaves.push(Box::new(node));
            }
            Self::build_branches(&leaves)
        }        
    }
}



///
/// Ray intersection
impl<T: super::Intersectable> super::Intersectable for BVH<T> {
    fn intersect(&self, ray: super::Ray) -> Option<super::Intersection> {
        // Check each child in order
        let mut nearest_hit = super::Intersection::empty();
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
