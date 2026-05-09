#include "NTClient.h"
#include "Configuration.h"
#include <format>
#include <iostream>

NTClient::NTClient(int teamNumber) : teamNumber(teamNumber) , ntInstance(nt::NetworkTableInstance::GetDefault()) {}

NTClient::~NTClient() {
    ntInstance.StopClient();
}

void NTClient::conenect() {
    ntInstance.StartClient4("driverstation-nt-server");
    std::string robotAddress = std::format("10.{}.{}.2", teamNumber / 100, teamNumber % 100); // TODO: Check if this return 13.69

    auto m_robotTable = ntInstance.GetTable(Configuration::NT::robotTable);
    auto m_fmsTable = ntInstance.GetTable(Configuration::NT::fmsTable);

    robotEnabledSubscriber = m_robotTable->GetBooleanTopic(Configuration::NT::isRobotEnabled).Subscribe(false);
    isRedAlliance = m_fmsTable->GetBooleanTopic(Configuration::NT::isRedAlliance).Subscribe(false);

    std::cout << "[NT] Connecting to " << robotAddress << "...\n";
}

bool NTClient::isRobotConnected() const {
    return ntInstance.IsConnected();
}

RobotState NTClient::getRobotState() const {
    return RobotState{
        .robotEnabled = robotEnabledSubscriber->Get(),
        .isRedAlliance = isRedAlliance->Get(),
        .ntConnected = ntInstance.IsConnected()
    };
}