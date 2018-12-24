use serde_json;

use std;
use std::f32;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::ops::Deref;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Mesh loading
// --------------------------------------------------------------------------------------------------------------------------------------------------

fn load_ply(filename: &Path) -> Result<super::Mesh, std::io::Error> {
    println!("Loading {}", filename.display());
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
                    x: vx,
                    y: vy,
                    z: vz,
                    nx: nx,
                    ny: ny,
                    nz: nz
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
                
                faces.push(super::Triangle {
                    v1: index1,
                    v2: index2,
                    v3: index3
                });
            }
        }
    }
    
    // Done
    Ok(super::Mesh {
        vertices: vertices,
        faces: faces
    })
    
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Scene loading
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct SceneDescription {
    pub camera: super::Camera,
    pub materials: HashMap<String, super::Material>,
    pub meshes: HashMap<String, String>,
    pub instances: Vec<super::Instance>
}


// TODO: check keys
pub fn load(filename: &Path) -> Result<super::Scene, std::io::Error> {
    // Load the scene description from the JSON file
    let file = File::open(filename)?;
    let desc : super::SceneDescription = serde_json::from_reader(file)?;

    // Load each model
    let base_dir = filename.parent().unwrap();
    let mut meshes = HashMap::new();
    for (k,v) in &desc.meshes {
        let mesh_path = base_dir.join(v);
        let mesh = load_ply(mesh_path.deref())?;
        meshes.insert(k.clone(), mesh);
    }

    // Build the scene
    Ok(super::Scene {
        camera: desc.camera,
        materials: desc.materials,
        meshes: meshes,
        instances: desc.instances
    })
}
