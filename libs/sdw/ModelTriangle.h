#pragma once

#include "Colour.h"
#include "TexturePoint.h"
#include <array>
#include <glm/glm.hpp>
#include <string>

struct ModelTriangle {
    std::array<glm::vec3, 3> vertices{};
    std::array<TexturePoint, 3> texturePoints{};
    Colour colour{};
    glm::vec3 normal{};

    ModelTriangle();
    ModelTriangle(const glm::vec3 &v0, const glm::vec3 &v1, const glm::vec3 &v2, Colour trigColour);
    friend std::ostream &operator<<(std::ostream &os, const ModelTriangle &triangle);
};
