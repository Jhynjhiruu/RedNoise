use std::{
    env::var,
    fs::{read_dir, write},
    path::{Path, PathBuf},
};

fn generate_makefile(path: &str, src_path: &Path, sdw_path: &Path, glm_path: &Path) {
    let file = format!(
        "\
TARGET := RedNoise

BUILD_DIR := build

SRC_DIR := {}
SDW_DIR := {}
GLM_DIR := {}

SRC_FILES := $(wildcard $(SRC_DIR)/*.cpp)
SDW_FILES := $(wildcard $(SDW_DIR)/*.cpp)

OBJ_FILES := $(foreach f,$(SRC_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f) \
             $(foreach f,$(SDW_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f)

$(shell mkdir -p $(BUILD_DIR) $(foreach dir,$(SRC_DIR) $(SDW_DIR),$(BUILD_DIR)/$(dir)))

INCFLAGS := -I$(SDW_DIR) -I$(GLM_DIR)
SDLFLAGS := $(shell sdl2-config --cflags)
SDL_LINK := $(shell sdl2-config --libs)

$(TARGET): $(OBJ_FILES)
    $(CXX) -o $@ $^ $(SDL_LINK)

$(BUILD_DIR)/%.cpp.o: %.cpp
    $(CXX) -c -o $@ $^ $(INCFLAGS) $(SDLFLAGS)

all: $(TARGET)

clean:
    rm -rf $(BUILD_DIR)

.PHONY: all, clean
",
        src_path.display(),
        sdw_path.display(),
        glm_path.display()
    )
    .replace("    ", "\t");

    write(path, file).unwrap();
}

fn get_cxx_files<T: AsRef<Path>>(path: T) -> Vec<PathBuf> {
    read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|p| p.path())
        .filter(|f| f.extension().is_some_and(|e| e == "cpp"))
        .collect()
}

fn main() {
    let sdl2_path_env = var("DEP_SDL2_INCLUDE").unwrap();
    println!("{}", sdl2_path_env);
    let sdl2_path = Path::new(&sdl2_path_env);
    let src_path = Path::new("src");
    let sdw_path = Path::new("libs/sdw");
    let glm_path = Path::new("libs/glm-0.9.7.2");
    let includes = [sdl2_path, sdw_path, glm_path];

    let sdw_files = get_cxx_files(sdw_path);

    cc::Build::new()
        .files(&sdw_files)
        .includes(includes)
        .link_lib_modifier("+whole-archive")
        .cpp(true)
        .compile("sdw");

    let src_files = get_cxx_files(src_path);

    cc::Build::new()
        .files(&src_files)
        .includes(includes)
        .define("__CARGO__", None)
        .cpp(true)
        .compile("_cxx");

    for file in &src_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    generate_makefile("Makefile", src_path, sdw_path, glm_path);
}
