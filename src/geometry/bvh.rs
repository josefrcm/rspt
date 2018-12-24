use nalgebra;
use std::f32;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

const BUNDLE_SIZE : usize = 10;
const NODE_SIZE : usize = 4;

pub struct BVH_Node {
    bounds: super::BoundingBox,
    child: BVH
}


///
/// Recursive bounding volume hierarchy
pub enum BVH {
    Node(Vec<BVH_Node>),
    Leaf(super::TriangleBundle)
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Build the whole world geometry
impl BVH {
    ///
    /// Constructor
    pub fn build(vertices: &Vec<super::Vertex>, faces: &Vec<super::Triangle>) -> Self {
        // Bottom-up building
        //let bounds = faces.chunks(BUNDLE_SIZE).map(|f| super::BoundingBox::build2(&vertices, &f.to_vec())).collect();
        //let leaves = faces.chunks(BUNDLE_SIZE).map(|f| super::TriangleBundle::build(&vertices, &f.to_vec())).map(|x| BVH::Leaf(x)).collect();
        //BVH::Node(bounds, leaves)
        let mut leaves = Vec::new();
        for c in faces.chunks(BUNDLE_SIZE) {
            let foo = super::BoundingBox::build2(&vertices, &c.to_vec());
            let bar = super::TriangleBundle::build(&vertices, &c.to_vec());
            leaves.push(BVH_Node {
                bounds: foo,
                child: BVH::Leaf(bar)
            });
        }
        BVH::Node(leaves)
    }


    ///
    /// Ray intersection
    pub fn intersect(&self, ray: super::Ray) -> Option<super::Intersection> {
        match self {
            BVH::Node(children) => {
                // Check each intersection in order
                let mut nearest_hit = super::Intersection {
                    distance: std::f32::INFINITY,
                    alpha: 0.0,
                    beta: 0.0,
                    gamma: 0.0,
                    v1: 0,
                    v2: 0,
                    v3: 0,
                    material: 0
                };

                for (index, child) in children.iter().enumerate() {
                    let intersection = child.bounds.intersect(ray);
                    if intersection.start < nearest_hit.distance {
                        match child.child.intersect(ray) {
                            Some(hit) => {
                                if hit.distance < nearest_hit.distance {
                                    nearest_hit = hit;
                                }
                            },
                            None => {}
                        }
                    }
                }

                // If a triangle was hit, compute the intersection parameters: coordinates, normal, material, etc.
                if nearest_hit.distance.is_infinite() {
                    None
                } else {
                    Some(nearest_hit)
                }
            },
            BVH::Leaf(bundle) => {
                bundle.intersect(ray)
            }
        }
    }
}
