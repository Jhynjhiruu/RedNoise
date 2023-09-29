use std::{
    env::var,
    fs::{read_dir, write},
    path::{Path, PathBuf},
};

fn generate_makefile(path: &str, src_path: &PathBuf, sdw_path: &PathBuf, glm_path: &PathBuf) {
    let target = var("CARGO_PKG_NAME").unwrap();
    let build_dir = "build";
    let out_dir = ".";
    let file = format!(
        "\
TARGET := {target}

BUILD_DIR := {build_dir}

OUT_DIR := {out_dir}

TARGET_EXE := $(OUT_DIR)/$(TARGET)

ifneq ($(GCC_PREFIX),)
	CXX := $(GCC_PREFIX)/bin/g++
	RPATH := -Wl,-rpath,
	LINK_SUFFIX := $(patsubst %,$(RPATH)$(GCC_PREFIX)/%,lib64 lib)
else
	LINK_SUFFIX :=
endif

SRC_DIR := {}
SDW_DIR := {}
GLM_DIR := {}

SRC_FILES := $(wildcard $(SRC_DIR)/*.cpp)
SDW_FILES := $(wildcard $(SDW_DIR)/*.cpp)

OBJ_FILES := $(foreach f,$(SRC_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f) \
             $(foreach f,$(SDW_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f)

$(shell mkdir -p $(BUILD_DIR) $(foreach dir,$(SRC_DIR) $(SDW_DIR),$(BUILD_DIR)/$(dir)))

$(shell mkdir -p $(OUT_DIR))

INCFLAGS := -I$(SDW_DIR) -I$(GLM_DIR)
SDLFLAGS := $(shell sdl2-config --cflags)
SDL_LINK := $(shell sdl2-config --libs)

$(TARGET_EXE): $(OBJ_FILES)
    $(CXX) -o $@ $^ $(SDL_LINK) $(LINK_SUFFIX)

$(BUILD_DIR)/%.cpp.o: %.cpp
    $(CXX) -c -o $@ $^ $(INCFLAGS) $(SDLFLAGS)

default: $(TARGET_EXE)

run: $(TARGET_EXE)
    $(TARGET_EXE)

clean:
    rm -rf $(BUILD_DIR)

.PHONY: default, run, clean
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
    let sdl2_path = PathBuf::from(&sdl2_path_env);
    let src_path = PathBuf::from("src");
    let lib_path = PathBuf::from("libs");
    let sdw_path = lib_path.join("sdw");
    let glm_path = lib_path.join("glm-0.9.7.2");
    let includes = [&sdl2_path, &sdw_path, &glm_path];

    let sdw_files = get_cxx_files(&sdw_path);

    cc::Build::new()
        .files(&sdw_files)
        .includes(includes)
        .link_lib_modifier("+whole-archive")
        .cpp(true)
        .std("c++20")
        .compile("sdw");

    let src_files = get_cxx_files(&src_path);

    cc::Build::new()
        .files(&src_files)
        .includes(includes)
        .define("__CARGO__", None)
        .cpp(true)
        .std("c++20")
        .compile("_cxx");

    for file in &src_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    generate_makefile("Makefile", &src_path, &sdw_path, &glm_path);
}
