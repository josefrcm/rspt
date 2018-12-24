extern crate clap;
extern crate rand;
extern crate rayon;
extern crate nalgebra;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod geometry;
mod tracer;


///
/// Program options
struct ProgramOptions {
    resolution: usize,
    max_bounces: usize,
    num_samples: usize,
    scene_file: std::path::PathBuf,
    camera_file: std::path::PathBuf,
    image_file: std::path::PathBuf
}



///
/// Parse the command line arguments
fn parse_options() -> ProgramOptions {
    // Argument definition
    let matches = clap::App::new("Rusty Ray")
                          .version("0.1")
                          .author("José Franco Campos <josefrancocampos@gmail.com>")
                          .about("A toy path-tracer in Rust")
                          .arg(clap::Arg::with_name("num_samples")
                               .short("s")
                               .long("num_samples")
                               .value_name("SAMPLES")
                               .help("Number of samples to trace - More samples, more quality")
                               .takes_value(true))
                          .arg(clap::Arg::with_name("resolution")
                               .short("r")
                               .long("resolution")
                               .value_name("PIXELS")
                               .help("Image resolution - The output image will be squared")
                               .takes_value(true))
                          .arg(clap::Arg::with_name("max_bounces")
                               .short("b")
                               .long("max_bounces")
                               .value_name("BOUNCES")
                               .help("Maximum number of bounces per ray")
                               .takes_value(true))
                          .arg(clap::Arg::with_name("input")
                               .short("i")
                               .long("input")
                               .value_name("FILE")
                               .help("Path to the scene definition file")
                               .takes_value(true)
                               .required(true))
                          .arg(clap::Arg::with_name("camera")
                               .short("c")
                               .long("input")
                               .value_name("FILE")
                               .help("Path to the camera definition file")
                               .takes_value(true)
                               .required(true))
                          .arg(clap::Arg::with_name("output")
                               .short("o")
                               .long("output")
                               .value_name("FILE")
                               .help("Name of the resulting image")
                               .takes_value(true)
                               .required(true))
                          .get_matches();

    // Read the values
    ProgramOptions {
        resolution: matches.value_of("resolution").unwrap_or("1024").parse::<usize>().unwrap(),
        num_samples: matches.value_of("num_samples").unwrap_or("10").parse::<usize>().unwrap(),
        max_bounces: matches.value_of("max_bounces").unwrap_or("4").parse::<usize>().unwrap(),
        scene_file: std::path::PathBuf::from(matches.value_of("input").unwrap_or("").to_string()),
        camera_file: std::path::PathBuf::from(matches.value_of("camera").unwrap_or("").to_string()),
        image_file: std::path::PathBuf::from(matches.value_of("output").unwrap_or("").to_string())
    }
}



///
/// 
fn main() {
    // Load the input data
    let options = parse_options();
    let scene = tracer::Scene::from_json(&options.scene_file).unwrap();
    let camera = tracer::Camera::from_json(&options.camera_file, options.resolution).unwrap();

    // Render the scene
    let mut fb = tracer::Image::new(options.resolution, options.resolution);
    for i in 0..options.num_samples {
        println!("Rendering sample {}/{}", i, options.num_samples);
        //let rays = camera.make_rays();
        //let sampling = rays.par_iter().map(|&r| tracer::sample(&scene, r, options.max_bounces)).collect();
        let sampling = tracer::sample(&scene, &camera, options.max_bounces);
        fb.accum(&sampling);
    }
    
    // Write the resulting image
    println!("Writing result");
    fb.normalize();
    fb.save_tga(&options.image_file);
    println!("Done");
}
