TARGET := RedNoise

BUILD_DIR := build

SRC_DIR := src
SDW_DIR := libs/sdw
GLM_DIR := libs/glm-0.9.7.2

SRC_FILES := $(wildcard $(SRC_DIR)/*.cpp)
SDW_FILES := $(wildcard $(SDW_DIR)/*.cpp)

OBJ_FILES := $(foreach f,$(SRC_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f) $(foreach f,$(SDW_FILES:.cpp=.cpp.o),$(BUILD_DIR)/$f)

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