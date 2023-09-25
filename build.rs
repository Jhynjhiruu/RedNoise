use std::{env::var, fs::read_dir, path::Path};

fn main() {
    let sdl2_path_env = var("DEP_SDL2_INCLUDE").unwrap();
    println!("{}", sdl2_path_env);
    let sdl2_path = Path::new(&sdl2_path_env);
    let sdw_path = Path::new("libs/sdw/");
    let glm_path = Path::new("libs/glm-0.9.7.2/");
    let includes = [sdl2_path, sdw_path, glm_path];

    let cxx_files = read_dir(sdw_path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|p| p.path())
        .filter(|f| f.extension().is_some_and(|e| e == "cpp"));

    println!("{:?}", cxx_files);

    cc::Build::new()
        .files(cxx_files)
        .includes(includes)
        .link_lib_modifier("+whole-archive")
        .cpp(true)
        .compile("sdw");

    cc::Build::new()
        .file("src/RedNoise.cpp")
        .includes(includes)
        .cpp(true)
        .compile("_cxx");

    println!("cargo:rerun-if-changed=src/RedNoise.cpp");
}
