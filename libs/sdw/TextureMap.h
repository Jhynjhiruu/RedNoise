#pragma once

#include "Utils.h"
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>

class TextureMap {
    public:
    size_t width;
    size_t height;
    std::vector<uint32_t> pixels;

    TextureMap();
    TextureMap(const std::string &filename);
    friend std::ostream &operator<<(std::ostream &os, const TextureMap &point);
};
