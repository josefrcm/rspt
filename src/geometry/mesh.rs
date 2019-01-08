use std;
use std::f32;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

use geometry::util::Intersectable;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Vertex
pub struct Vertex {
    pub coords: nalgebra::Vector4<f32>,
    pub normal: nalgebra::Vector4<f32>
}


///
/// Triangle
#[derive(Clone)]
pub struct Triangle {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub material: u32
}


///
/// Polygon mesh
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: super::BVH<super::TriangleBundle>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

const BUNDLE_SIZE : usize = 10;



impl Mesh {
    ///
    /// Load a mesh from a PLY file
    pub fn load_ply(filename: &Path, material: u32) -> Result<Self, std::io::Error> {
        println!("Loading mesh {}", filename.display());
        let f = try!(File::open(filename));
        let file = BufReader::new(&f);
        let mut lines = file.lines();
        
        let mut vertices : Vec<super::Vertex> = Vec::new();
        let mut faces : Vec<super::Triangle> = Vec::new();
        let mut num_vertices : usize = 0;
        let mut num_faces : usize = 0;
        
        // Read the header
        loop {
            match lines.next() {
                None => break,
                Some(line) => {
                    let foo = line.unwrap();
                    let fields : Vec<&str> = foo.split(" ").collect();
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
                    let fields : Vec<&str> = foo.split(" ").collect();
                    let vx = fields[0].parse::<f32>().unwrap();
                    let vy = fields[1].parse::<f32>().unwrap();
                    let vz = fields[2].parse::<f32>().unwrap();
                    let nx = fields[3].parse::<f32>().unwrap();
                    let ny = fields[4].parse::<f32>().unwrap();
                    let nz = fields[5].parse::<f32>().unwrap();
                    vertices.push(super::Vertex {
                        coords: nalgebra::Vector4::new(vx, vy, vz, 1.0),
                        normal: nalgebra::Vector4::new(nx, ny, nz, 0.0)
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
                    let fields : Vec<&str> = foo.split(" ").collect();
                    let index1 = fields[1].parse::<u32>().unwrap();
                    let index2 = fields[2].parse::<u32>().unwrap();
                    let index3 = fields[3].parse::<u32>().unwrap();
                    
                    faces.push(Triangle {
                        v1: index1,
                        v2: index2,
                        v3: index3,
                        material: material
                    });
                }
            }
        }        

        // Build the acceleration structure
        let mut bundles = Vec::new();
        for c in faces.chunks(BUNDLE_SIZE) {
            let foo = super::TriangleBundle::build(&vertices, &c.to_vec());
            let bar = super::BoundingBox::build2(&vertices, &c.to_vec());
            bundles.push((foo, bar));
        }
        let tree = super::BVH::build(bundles);

        // Done
        Ok(Mesh {
            vertices: vertices,
            faces: tree
        })
        
    }


    ///
    /// Compute the bounding box
    pub fn bounds(&self) -> super::BoundingBox {
        super::BoundingBox::build(&self.vertices)
    }
}



///
/// Mesh-ray intersection
impl Intersectable for Mesh {
    ///
    /// Compute the mesh-ray intersection
    fn intersect(&self, ray: super::Ray) -> Option<super::Intersection> {
        match self.faces.intersect(ray) {
            None => None,
            Some(hit) => {
                let v1 = &self.vertices[hit.v1 as usize];
                let v2 = &self.vertices[hit.v2 as usize];
                let v3 = &self.vertices[hit.v3 as usize];
                let point = hit.alpha * v1.coords + hit.beta * v2.coords + hit.gamma * v3.coords;
                let normal = (hit.alpha * v1.normal + hit.beta * v2.normal + hit.gamma * v3.normal).normalize();
                Some(super::Intersection {
                    point: point,
                    normal: normal,
                    distance: hit.distance,
                    material: hit.material,
                    alpha: f32::NAN,
                    beta: f32::NAN,
                    gamma: f32::NAN,
                    v1: 0,
                    v2: 0,
                    v3: 0
                })
            }
        }
    }
}
