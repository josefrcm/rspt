// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Triangle bundle
pub struct TriangleBundle {
    faces: Vec<Triangle2>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Build a triangle bundle
impl TriangleBundle {
    pub fn build(vertices: &Vec<super::Vertex>, faces: &Vec<super::Triangle>) -> Self {
        let mut bundle = super::TriangleBundle {
            faces: Vec::new()
        };

        // Compute the triangle equations
        for triangle in faces {
            // The triangle itself
            let index1 = triangle.v1;
            let index2 = triangle.v2;
            let index3 = triangle.v3;

            // Vertex coordinates
            let vertex1 = vertices[index1 as usize].coords.xyz();
            let vertex2 = vertices[index2 as usize].coords.xyz();
            let vertex3 = vertices[index3 as usize].coords.xyz();

            // Triangle normal
            let edge_a = vertex2 - vertex1;
            let edge_b = vertex3 - vertex1;
            let normal = edge_a.cross(&edge_b).normalize();

            // Plane equation
            let plane_eq = nalgebra::Vector4::new(normal.x, normal.y, normal.z, -normal.dot(&vertex1));

            // World-to-barycentric coordinate conversion
            let barycentric = nalgebra::Matrix3::from_columns(&[edge_a, edge_b, normal]).try_inverse().unwrap();
            let beta_eq = nalgebra::Vector4::new(
                barycentric[(0,0)],
                barycentric[(0,1)],
                barycentric[(0,2)],
                -nalgebra::Vector3::new(barycentric[(0,0)], barycentric[(0,1)], barycentric[(0,2)]).dot(&vertex1)
                );
            let gamma_eq = nalgebra::Vector4::new(
                barycentric[(1,0)],
                barycentric[(1,1)],
                barycentric[(1,2)],
                -nalgebra::Vector3::new(barycentric[(1,0)], barycentric[(1,1)], barycentric[(1,2)]).dot(&vertex1)
                );

            // Add the triangle to the bundle
            bundle.faces.push(Triangle2 {
                material: triangle.material,
                v1: triangle.v1,
                v2: triangle.v2,
                v3: triangle.v3,
                plane_eq: plane_eq,
                beta_eq: beta_eq,
                gamma_eq: gamma_eq
            });
        }

        bundle
    }
}



impl super::Intersectable for TriangleBundle {
    ///
    /// Ray-Bundle intersection
    fn intersect(&self, ray: super::Ray) -> Option<super::Intersection> {
        // Find the intersection of the ray against the bundle
        let mut nearest_face: usize = 0;
        let mut nearest_hit = TriangleIntersection {
            distance: std::f32::INFINITY,
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
        };

        for (index, face) in self.faces.iter().enumerate() {
            match intersect_triangle(face, ray) {
                Some(intersection) => {
                    if intersection.distance < nearest_hit.distance {
                        nearest_hit = intersection;
                        nearest_face = index;
                    }
                },
                None => {}
            }
        }

        // If a triangle was hit, compute the intersection parameters: coordinates, normal, material, etc.
        if nearest_hit.distance.is_infinite() {
            None
        } else {
            Some(super::Intersection {
                distance: nearest_hit.distance,
                alpha: nearest_hit.alpha,
                beta: nearest_hit.beta,
                gamma: nearest_hit.gamma,
                v1: self.faces[nearest_face].v1,
                v2: self.faces[nearest_face].v2,
                v3: self.faces[nearest_face].v3,
                material: self.faces[nearest_face].material,
                point: nalgebra::zero(),
                normal: nalgebra::zero(),
            })
        }
    }
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Private stuff
// --------------------------------------------------------------------------------------------------------------------------------------------------


///
/// Triangle (with structures for accelerated ray-triangle intersection)
struct Triangle2 {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub material: u32,
    pub plane_eq: nalgebra::Vector4<f32>,
    pub beta_eq: nalgebra::Vector4<f32>,
    pub gamma_eq: nalgebra::Vector4<f32>,
}



///
/// Intersection against a single triangle
struct TriangleIntersection {
    pub distance: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32
}



///
/// Ray-triangle intersection
/// https://en.wikipedia.org/wiki/Moller-Trumbore_intersection_algorithm
fn intersect_triangle(triangle: &Triangle2, ray: super::Ray) -> Option<TriangleIntersection> {
    const EPSILON : f32 = 0.0000001;

    let t = -triangle.plane_eq.dot(&ray.origin) / triangle.plane_eq.dot(&ray.direction);
    if t > EPSILON /*& (t >= ray.start) && (t < ray.finish)*/ {
        let point = ray.origin + t * ray.direction;
        let beta = triangle.beta_eq.dot(&point);
        let gamma = triangle.gamma_eq.dot(&point);
        let alpha = 1.0 - beta - gamma;
        if (alpha > 0.0) && (beta > 0.0) && (gamma > 0.0) {
            Some(TriangleIntersection{
                distance: t,
                alpha: alpha,
                beta: beta,
                gamma: gamma
            })
        } else {
            None
        }
    }
    else {
        // This means that there is a line intersection but not a ray intersection.
        None
    }
}
