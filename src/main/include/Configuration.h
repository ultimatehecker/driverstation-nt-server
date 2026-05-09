#pragma once
#include <string>

namespace Configuration {
    constexpr int teamNumber = 1369;
    constexpr int networkTablePollHz = 20;

    namespace NT {
        constexpr const char* robotTable = "RobotState";
        constexpr const char* fmsTable = "FMSInfo";

        constexpr const char* isRobotEnabled = "RobotEnabled";
        constexpr const char* isRedAlliance = "IsRedAlliance";
    }

    constexpr const char* serialPort = "";
    constexpr int baudRate = 115200;

    constexpr int ledCount = 110;
}