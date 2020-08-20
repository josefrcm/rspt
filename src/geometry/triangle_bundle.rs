use super::*;
use nalgebra::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub const BUNDLE_SIZE: usize = 8;
pub const EPSILON: f32 = 0.0000001;
#[allow(non_camel_case_types)]
type u32xN = [u32; BUNDLE_SIZE];
#[allow(non_camel_case_types)]
type f32xN = VectorN<f32, U8>;

///
/// Triangle bundle
#[derive(Clone, Copy)]
pub struct TriangleBundle {
    plane_eq_x: f32xN,
    plane_eq_y: f32xN,
    plane_eq_z: f32xN,
    plane_eq_w: f32xN,

    beta_eq_x: f32xN,
    beta_eq_y: f32xN,
    beta_eq_z: f32xN,
    beta_eq_w: f32xN,

    gamma_eq_x: f32xN,
    gamma_eq_y: f32xN,
    gamma_eq_z: f32xN,
    gamma_eq_w: f32xN,

    faces: [Triangle; BUNDLE_SIZE],
}

///
/// Result of a bundle-ray intersection
pub struct BundleIntersection {
    pub distance: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub face: Triangle,
}

impl BundleIntersection {
    pub fn empty() -> Self {
        Self {
            distance: f32::INFINITY,
            alpha: f32::NAN,
            beta: f32::NAN,
            gamma: f32::NAN,
            face: Triangle::invalid(),
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Build a triangle bundle
impl TriangleBundle {
    pub fn new(vertices: &[Vertex], faces: &[Triangle]) -> Self {
        // Preconditions
        if faces.len() > BUNDLE_SIZE {
            panic!(
                "Triangle bundles should be at most {} elements big!",
                BUNDLE_SIZE
            );
        }

        // Initialize the bundle to all zeros
        let mut bundle = TriangleBundle {
            plane_eq_x: zero(),
            plane_eq_y: zero(),
            plane_eq_z: zero(),
            plane_eq_w: zero(),
            beta_eq_x: zero(),
            beta_eq_y: zero(),
            beta_eq_z: zero(),
            beta_eq_w: zero(),
            gamma_eq_x: zero(),
            gamma_eq_y: zero(),
            gamma_eq_z: zero(),
            gamma_eq_w: zero(),
            faces: [Triangle::invalid(); BUNDLE_SIZE],
        };

        // Compute the triangle equations
        for (i, triangle) in faces.iter().enumerate() {
            // The triangle itself
            bundle.faces[i] = *triangle;
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
            bundle.plane_eq_x[i] = normal.x;
            bundle.plane_eq_y[i] = normal.y;
            bundle.plane_eq_z[i] = normal.z;
            bundle.plane_eq_w[i] = -normal.dot(&vertex1.coords);

            // World-to-barycentric coordinate conversion
            let barycentric = Matrix3::from_columns(&[edge_a, edge_b, normal])
                .try_inverse()
                .unwrap();

            bundle.beta_eq_x[i] = barycentric[(0, 0)];
            bundle.beta_eq_y[i] = barycentric[(0, 1)];
            bundle.beta_eq_z[i] = barycentric[(0, 2)];
            bundle.beta_eq_w[i] = -Vector3::new(
                barycentric[(0, 0)],
                barycentric[(0, 1)],
                barycentric[(0, 2)],
            )
            .dot(&vertex1.coords);

            bundle.gamma_eq_x[i] = barycentric[(1, 0)];
            bundle.gamma_eq_y[i] = barycentric[(1, 1)];
            bundle.gamma_eq_z[i] = barycentric[(1, 2)];
            bundle.gamma_eq_w[i] = -Vector3::new(
                barycentric[(1, 0)],
                barycentric[(1, 1)],
                barycentric[(1, 2)],
            )
            .dot(&vertex1.coords);
        }

        bundle
    }
}

///
/// Ray-bundle intersection
/// [Baldwin-Weber]
/// http://jcgt.org/published/0005/03/03/
impl TriangleBundle {
    ///
    /// Ray-Bundle intersection
    pub fn intersect(&self, ray: Ray) -> BundleIntersection {
        // Compute the intersection of the ray against all triangles in the bundle
        let t1: f32xN = (self.plane_eq_x * ray.origin.x)
            + (self.plane_eq_y * ray.origin.y)
            + (self.plane_eq_z * ray.origin.z)
            + self.plane_eq_w;
        let t2: f32xN = (self.plane_eq_x * ray.direction.x)
            + (self.plane_eq_y * ray.direction.y)
            + (self.plane_eq_z * ray.direction.z);
        let distances: f32xN = -t1.component_div(&t2);

        let points_x: f32xN = f32xN::repeat(ray.origin.x) + distances * ray.direction.x;
        let points_y: f32xN = f32xN::repeat(ray.origin.y) + distances * ray.direction.y;
        let points_z: f32xN = f32xN::repeat(ray.origin.z) + distances * ray.direction.z;

        let betas: f32xN = self.beta_eq_x.component_mul(&points_x)
            + self.beta_eq_y.component_mul(&points_y)
            + self.beta_eq_z.component_mul(&points_z)
            + self.beta_eq_w;

        let gammas: f32xN = self.gamma_eq_x.component_mul(&points_x)
            + self.gamma_eq_y.component_mul(&points_y)
            + self.gamma_eq_z.component_mul(&points_z)
            + self.gamma_eq_w;

        let alphas: f32xN = f32xN::repeat(1.0) - betas - gammas;

        // Find the intersection of the ray against the bundle
        let mut nearest_distance = f32::INFINITY;
        let mut nearest_index = 0;
        for i in 0..BUNDLE_SIZE {
            if (self.faces[i].material != u32::MAX)
                && (distances[i] > EPSILON)
                && (alphas[i] > 0.0)
                && (betas[i] > 0.0)
                && (gammas[i] > 0.0)
                && (distances[i] < nearest_distance)
            {
                nearest_distance = distances[i];
                nearest_index = i;
            }
        }

        // If a triangle was hit, compute the intersection parameters: coordinates, normal, material, etc.
        if nearest_distance.is_infinite() {
            BundleIntersection::empty()
        } else {
            BundleIntersection {
                distance: nearest_distance,
                alpha: alphas[nearest_index],
                beta: betas[nearest_index],
                gamma: gammas[nearest_index],
                face: self.faces[nearest_index],
            }
        }
    }
}
