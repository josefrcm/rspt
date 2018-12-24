use scene;


// --------------------------------------------------------------------------------------------------------------------------------------------------
// Data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub struct Vertex {
    pub coords: nalgebra::Vector4<f32>,
    pub normal: nalgebra::Vector4<f32>
}

pub struct Triangle {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub material: u32,
    pub plane_eq: nalgebra::Vector4<f32>,
    pub beta_eq: nalgebra::Vector4<f32>,
    pub gamma_eq: nalgebra::Vector4<f32>,
}

pub struct Geometry {    
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Triangle>,
    pub materials: Vec<scene::Material>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// World building
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn build_geometry(scene : &scene::Scene) -> Geometry {
    // Load each model
    let mut materials = Vec::new();
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    
    // Unroll each instance
    for m in scene.instances.iter() {
        let mesh = &scene.meshes[&m.mesh];
        let material = &scene.materials[&m.material];

        // Add the material
        let num_mat = materials.len() as u32;
        materials.push(material.clone());

        // Add the new vertices
        let vertex_offset = vertices.len() as u32;
        for v in &mesh.vertices {
            vertices.push(Vertex {
                coords: nalgebra::Vector4::new(v.x, v.y, v.z, 1.0),
                normal: nalgebra::Vector4::new(v.nx, v.ny, v.nz, 0.0)
            });
        }

        // Add the new faces
        for f in &mesh.faces {
            // The triangle itself
            let index1 = f.v1 + vertex_offset;
            let index2 = f.v2 + vertex_offset;
            let index3 = f.v3 + vertex_offset;
            faces.push(Triangle {
                v1: index1,
                v2: index2,
                v3: index3,
                material: num_mat,
                plane_eq: nalgebra::zero(),
                beta_eq: nalgebra::zero(),
                gamma_eq: nalgebra::zero()
            });
        }
    }

    // Done
    let mut geom = Geometry {
        materials: materials,
        vertices: vertices,
        faces: faces
    };
    geom.build_accel();
    geom
}

impl Geometry {
    pub fn build_accel(&mut self) {
        for f in self.faces.iter_mut() {
            // The triangle itself
            let index1 = f.v1;
            let index2 = f.v2;
            let index3 = f.v3;

            // Vertex coordinates
            let vertex1 = self.vertices[index1 as usize].coords.xyz();
            let vertex2 = self.vertices[index2 as usize].coords.xyz();
            let vertex3 = self.vertices[index3 as usize].coords.xyz();

            // Triangle normal
            let edge_a = vertex2 - vertex1;
            let edge_b = vertex3 - vertex1;
            let normal = edge_a.cross(&edge_b).normalize();

            // Plane equation
            f.plane_eq = nalgebra::Vector4::new(normal.x, normal.y, normal.z, -normal.dot(&vertex1));

            // World-to-barycentric coordinate conversion
            let barycentric = nalgebra::Matrix3::from_columns(&[edge_a, edge_b, normal]).try_inverse().unwrap();
            f.beta_eq = nalgebra::Vector4::new(
                barycentric[(0,0)],
                barycentric[(0,1)],
                barycentric[(0,2)],
                -nalgebra::Vector3::new(barycentric[(0,0)], barycentric[(0,1)], barycentric[(0,2)]).dot(&vertex1)
                );
            f.gamma_eq = nalgebra::Vector4::new(
                barycentric[(1,0)],
                barycentric[(1,1)],
                barycentric[(1,2)],
                -nalgebra::Vector3::new(barycentric[(1,0)], barycentric[(1,1)], barycentric[(1,2)]).dot(&vertex1)
                );
        }
    }
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Intersections
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub struct TriangleIntersection {
    pub distance: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32
}

pub struct MeshIntersection<'a> {
    pub distance: f32,
    pub point: nalgebra::Vector4<f32>,
    pub normal: nalgebra::Vector4<f32>,
    pub material: &'a scene::Material
}

// Ray-triangle intersection
// https://en.wikipedia.org/wiki/Moller-Trumbore_intersection_algorithm
impl Triangle {
    pub fn intersect(&self, ray: super::Ray) -> Option<TriangleIntersection> {
        const EPSILON : f32 = 0.0000001;

        let t = -self.plane_eq.dot(&ray.origin) / self.plane_eq.dot(&ray.direction);
        if t > EPSILON {
            let point = ray.origin + t * ray.direction;
            let beta = self.beta_eq.dot(&point);
            let gamma = self.gamma_eq.dot(&point);
            let alpha = 1.0 - beta - gamma;
            if (alpha > 0.0) && (beta > 0.0) && (gamma > 0.0) {
                Some(super::TriangleIntersection{
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
}



// Mesh-triangle intersection
impl Geometry {
    pub fn intersect(&self, ray: super::Ray) -> Option<MeshIntersection> {
        // Find the intersection of the ray against the mesh
        let mut nearest_face: usize = 0;
        let mut nearest_hit = super::TriangleIntersection {
            distance: 1.0 / 0.0, //std::f32::INFINITY,
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
        };

        for (index, face) in self.faces.iter().enumerate() {
            match face.intersect(ray) {
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
            let face = &self.faces[nearest_face];
            let v1 = &self.vertices[face.v1 as usize];
            let v2 = &self.vertices[face.v2 as usize];
            let v3 = &self.vertices[face.v3 as usize];
            let point = nearest_hit.alpha * v1.coords + nearest_hit.beta * v2.coords + nearest_hit.gamma * v3.coords;
            let normal = (nearest_hit.alpha * v1.normal + nearest_hit.beta * v2.normal + nearest_hit.gamma * v3.normal).normalize();
            Some(super::MeshIntersection {
                distance: nearest_hit.distance,
                point: point,
                normal: normal,
                material: &self.materials[face.material as usize]
            })
        }
    }
}
