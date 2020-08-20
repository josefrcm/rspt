use std;
use std::f32;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use super::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Vertex
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub coords: nalgebra::Point3<f32>,
    pub normal: nalgebra::Vector3<f32>,
}

///
/// Triangle
#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub material: u32,
}

///
/// Polygon mesh
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: BVH,
}

///
/// Result of a mesh-ray intersection
pub struct MeshIntersection {
    pub distance: f32,
    pub material: u32,
    pub point: nalgebra::Point3<f32>,
    pub normal: nalgebra::Vector3<f32>,
}

impl MeshIntersection {
    pub fn empty() -> Self {
        Self {
            distance: f32::INFINITY,
            material: u32::MAX,
            point: nalgebra::geometry::Point::origin(),
            normal: nalgebra::zero(),
        }
    }
}


// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Triangle {
    pub fn invalid() -> Self {
        Self {
            v1: u32::MAX,
            v2: u32::MAX,
            v3: u32::MAX,
            material: u32::MAX,
        }
    }
}

impl Mesh {
    ///
    /// Create a new mesh from an array of vertices and an array of triangles
    pub fn new(vertices: Vec<Vertex>, faces: Vec<Triangle>) -> Self {
        // Build the acceleration structure
        let mut bundles = Vec::new();
        for c in faces.chunks(BUNDLE_SIZE) {
            let foo = TriangleBundle::new(&vertices, &c.to_vec());
            let bar = AABB::from_faces(&vertices, &c.to_vec());
            bundles.push((foo, bar));
        }
        let tree = BVH::build_mesh(&bundles);

        // Done
        Mesh {
            vertices: vertices,
            faces: tree,
        }
    }

    ///
    /// Load a mesh from a PLY file
    pub fn load_ply(filename: &Path, material: u32) -> Result<Self, std::io::Error> {
        println!("Loading mesh {}", filename.display());
        let f = File::open(filename)?;
        let file = BufReader::new(&f);
        let mut lines = file.lines();

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Triangle> = Vec::new();
        let mut num_vertices: usize = 0;
        let mut num_faces: usize = 0;

        // Read the header
        loop {
            match lines.next() {
                None => break,
                Some(line) => {
                    let foo = line.unwrap();
                    let fields: Vec<&str> = foo.split(" ").collect();
                    if fields[0] == "end_header" {
                        break;
                    } else if (fields[0] == "element") && (fields[1] == "vertex") {
                        num_vertices = fields[2].parse::<usize>().unwrap();
                    } else if (fields[0] == "element") && (fields[1] == "face") {
                        num_faces = fields[2].parse::<usize>().unwrap();
                    }
                }
            }
        }

        // Read the vertices
        for _i in 0..num_vertices {
            match lines.next() {
                None => break,
                Some(line) => {
                    let foo = line.unwrap();
                    let fields: Vec<&str> = foo.split(" ").collect();
                    let vx = fields[0].parse::<f32>().unwrap();
                    let vy = fields[1].parse::<f32>().unwrap();
                    let vz = fields[2].parse::<f32>().unwrap();
                    let nx = fields[3].parse::<f32>().unwrap();
                    let ny = fields[4].parse::<f32>().unwrap();
                    let nz = fields[5].parse::<f32>().unwrap();
                    vertices.push(Vertex {
                        coords: nalgebra::Point3::new(vx, vy, vz),
                        normal: nalgebra::Vector3::new(nx, ny, nz),
                    });
                }
            }
        }

        // Read the faces
        for _i in 0..num_faces {
            match lines.next() {
                None => break,
                Some(line) => {
                    let foo = line.unwrap();
                    let fields: Vec<&str> = foo.split(" ").collect();
                    let index1 = fields[1].parse::<u32>().unwrap();
                    let index2 = fields[2].parse::<u32>().unwrap();
                    let index3 = fields[3].parse::<u32>().unwrap();

                    faces.push(Triangle {
                        v1: index1,
                        v2: index2,
                        v3: index3,
                        material: material,
                    });
                }
            }
        }

        // Build the acceleration structure
        Ok(Self::new(vertices, faces))
    }

    ///
    /// Compute the bounding box
    pub fn bounds(&self) -> AABB {
        AABB::from_vertices(&self.vertices)
    }
}

///
/// Mesh-ray intersection
impl Mesh {
    ///
    /// Compute the mesh-ray intersection
    pub fn intersect(&self, ray: Ray) -> MeshIntersection {
        let hit = self.faces.intersect(ray);
        if hit.distance.is_finite() {
            let v1 = &self.vertices[hit.face.v1 as usize];
            let v2 = &self.vertices[hit.face.v2 as usize];
            let v3 = &self.vertices[hit.face.v3 as usize];
            let point = hit.alpha * v1.coords.coords
                + hit.beta * v2.coords.coords
                + hit.gamma * v3.coords.coords;
            let normal =
                (hit.alpha * v1.normal + hit.beta * v2.normal + hit.gamma * v3.normal).normalize();
                MeshIntersection {
                point: nalgebra::Point3::new(point.x, point.y, point.z),
                normal: normal,
                distance: hit.distance,
                material: hit.face.material,
            }
        } else {
            MeshIntersection::empty()
        }
    }
}
