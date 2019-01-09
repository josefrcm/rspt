extern crate clap;
extern crate image;
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
/// Pretty print a duration
pub fn pretty_time(t: std::time::Duration) -> String {
     let nanos = t.subsec_nanos();
     let mut seconds = t.as_secs();
     let mut minutes = seconds / 60;
     seconds = seconds % 60;
     let hours = minutes / 60;
     minutes = minutes % 60;
     format!("{}:{:02}:{:02}.{}", hours, minutes, seconds, nanos)
}



///
/// 
fn main() {
    // Load the input data
    let load_start = std::time::Instant::now();
    let options = parse_options();
    let scene = tracer::Scene::from_json(&options.scene_file).unwrap();
    let mut camera = tracer::Camera::from_json(&options.camera_file, options.resolution).unwrap();
    let load_time = load_start.elapsed();

    // Render the scene
    let render_start = std::time::Instant::now();
    let mut fb = tracer::Image::new(options.resolution, options.resolution);
    for i in 0..options.num_samples {
        println!("Rendering sample {}/{}", i+1, options.num_samples);
        let sampling = tracer::sample(&scene, &mut camera, options.max_bounces);
        fb.accum(&sampling);
    }
    fb.scale(options.num_samples);
    let render_time = render_start.elapsed();
    
    // Write the resulting image
    let save_start = std::time::Instant::now();
    println!("Writing result");
    fb.save_png(&options.image_file);
    let save_time = save_start.elapsed();

    // Print the timing results
    println!("Timing results:");
    println!("\tLoading ->   {}", pretty_time(load_time));
    println!("\tRendering -> {}", pretty_time(render_time));
    println!("\tSaving ->    {}", pretty_time(save_time));
}
