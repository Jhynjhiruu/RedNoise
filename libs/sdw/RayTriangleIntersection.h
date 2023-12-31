#pragma once

#include "ModelTriangle.h"
#include <glm/glm.hpp>
#include <iostream>

struct RayTriangleIntersection {
    glm::vec3 intersectionPoint;
    float distanceFromCamera;
    ModelTriangle intersectedTriangle;
    size_t triangleIndex;

    RayTriangleIntersection();
    RayTriangleIntersection(const glm::vec3 &point, float distance, const ModelTriangle &triangle, size_t index);
    friend std::ostream &operator<<(std::ostream &os, const RayTriangleIntersection &intersection);
};
