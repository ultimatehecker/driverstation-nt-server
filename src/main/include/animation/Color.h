#pragma once
#include <cstdint>
#include <algorithm>

struct Color {
    uint8_t r = 0;
    uint8_t g = 0;
    uint8_t b = 0;

    constexpr Color() = default;
    constexpr Color(uint8_t r, uint8_t g, uint8_t b) : r(r), g(g), b(b) {}

    [[nodiscard]] Color withBrightness(float scalar) const {
        scalar = std::clamp(scalar, 0.0f, 1.0f);
        return {static_cast<uint8_t>(r * scalar), static_cast<uint8_t>(g * scalar), static_cast<uint8_t>(b * scalar)};
    }

    static constexpr Color Red() {
        return { 255, 0, 0 };
    }

    static constexpr Color Orange() {
        return { 255, 165, 0 };
    }

    static constexpr Color Yellow() {
        return { 255, 255, 0 };
    }

    static constexpr Color Green() {
        return { 0, 255, 0 };
    }

    static constexpr Color Cyan() {
        return { 0, 255, 255 };
    }

    static constexpr Color Blue() {
        return { 0, 0, 255 };
    }

    static constexpr Color Magenta() {
        return { 255, 0, 255 };
    }

    static constexpr Color Purple() {
        return { 128, 0, 128 };
    }

    static constexpr Color White() {
        return { 255, 255, 255 };
    }

    static constexpr Color Off() {
        return { 0, 0, 0 };
    }
};