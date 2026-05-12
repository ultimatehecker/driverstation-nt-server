#pragma once
#include <vector>
#include <chrono>
#include "Color.h"

using Frame = std::vector<Color>;

class Animation {
public:
    virtual ~Animation() = default;

    // Called each loop — returns true if the frame changed and if packet should be sent to the Arduino
    virtual bool update() = 0;

    const Frame& getFrame() const {
        return m_frame;
    }

    int getStartIndex() const {
        return m_startIndex;
    }

    int getEndIndex() const {
        return m_endIndex;
    }

protected:
    Animation(int startIndex, int endIndex) : m_startIndex(startIndex), m_endIndex(endIndex), m_frame(endIndex - startIndex + 1, Color::Off()) {}

    // checks if enough time has passed for the next frame based on the configured frame rate
    bool shouldAdvanceFrame() {
        if (m_frameRateHz <= 0) return false;
        auto now = std::chrono::steady_clock::now();
        auto elapsed = std::chrono::duration<double, std::milli>(now - m_lastFrame).count();
        double intervalMs = 1000.0 / m_frameRateHz;

        if (elapsed >= intervalMs) {
            m_lastFrame = now;
            return true;
        }

        return false;
    }

    // brightness and apply to a color
    Color applyBrightness(const Color& color) const {
        return color.withBrightness(m_brightness);
    }

    int m_startIndex;
    int m_endIndex;
    Frame m_frame;
    Color m_color = Color::White();
    float m_brightness = 1.0f;
    int m_size = 1;
    int m_frameRateHz = 24;

private:
    std::chrono::steady_clock::time_point m_lastFrame = std::chrono::steady_clock::now();
};