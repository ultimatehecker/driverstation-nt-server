#pragma once

#include <networktables/NetworkTableInstance.h>
#include <networktables/BooleanTopic.h>

struct RobotState {
    bool robotEnabled = false;
    bool isRedAlliance = false;
    bool ntConnected = false;
};

class NTClient {
public:
    explicit NTClient(int teamNumber);
    ~NTClient();

    void conenect();
    RobotState getRobotState() const;
    bool isRobotConnected() const;

private:
    int teamNumber;
    nt::NetworkTableInstance ntInstance;

    std::optional<nt::BooleanSubscriber> robotEnabledSubscriber;
    std::optional<nt::BooleanSubscriber> isRedAlliance;
};