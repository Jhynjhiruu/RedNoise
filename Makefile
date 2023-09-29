TARGET := RedNoise

BUILD_DIR := build

OUT_DIR := .

TARGET_EXE := $(OUT_DIR)/$(TARGET)

ifneq ($(GCC_PREFIX),)
	CXX := $(GCC_PREFIX)/bin/g++
	RPATH := -Wl,-rpath,
	LINK_SUFFIX := $(patsubst %,$(RPATH)$(GCC_PREFIX)/%,lib64 lib)
else
	LINK_SUFFIX :=
endif

SRC_DIR := src
SDW_DIR := libs/sdw
GLM_DIR := libs/glm-0.9.7.2

SRC_FILES := $(wildcard $(SRC_DIR)/*.cpp)
SDW_FILES := $(wildcard $(SDW_DIR)/*.cpp)

OBJ_FILES := $(foreach f,$(SRC_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f) $(foreach f,$(SDW_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f)

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
