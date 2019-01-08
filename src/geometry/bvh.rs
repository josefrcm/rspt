// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub enum Node<T> {
    Empty,
    Leaf(T),
    Branch(BVH<T>)
}


///
/// Recursive bounding volume hierarchy
pub struct BVH<T> {
    bounds: Vec<super::BoundingBox>,
    children: Vec<Node<T>>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Build the whole world geometry
impl<T> BVH<T> {
    ///
    /// Constructor
    pub fn build(elements: Vec<(T, super::BoundingBox)>) -> Self {
        const NODE_SIZE : usize = 4;

        // Bottom-up building
        //let bounds = faces.chunks(BUNDLE_SIZE).map(|f| super::BoundingBox::build2(&vertices, &f.to_vec())).collect();
        //let leaves = faces.chunks(BUNDLE_SIZE).map(|f| super::TriangleBundle::build(&vertices, &f.to_vec())).map(|x| BVH::Leaf(x)).collect();
        //BVH::Node(bounds, leaves)

        let mut bounds = Vec::new();
        let mut children = Vec::new();
        for (elem, bbox) in elements {
            bounds.push(bbox);
            children.push(Node::Leaf(elem));
        }
        BVH {
            bounds: bounds,
            children: children
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
