#include "NTClient.h"
#include "Configuration.h"
#include <format>
#include <iostream>

NTClient::NTClient(int teamNumber) : teamNumber(teamNumber) , ntInstance(nt::NetworkTableInstance::GetDefault()) {}

NTClient::~NTClient() {
    ntInstance.StopClient();
}

void NTClient::connect() {
    ntInstance.StartClient4("driverstation-nt-server");

    ntInstance.SetServerTeam(teamNumber, NT_DEFAULT_PORT4);
    ntInstance.SetServer("172.22.11.2", NT_DEFAULT_PORT4);
    ntInstance.SetServer("localhost", NT_DEFAULT_PORT4);

    std::cout << "[NT] Connecting to " << teamNumber << "...\n";

    auto m_robotTable = ntInstance.GetTable(Configuration::NT::robotTable);
    auto m_fmsTable = ntInstance.GetTable(Configuration::NT::fmsTable);

    robotEnabledSubscriber = m_robotTable->GetBooleanTopic(Configuration::NT::isRobotEnabled).Subscribe(false);
    isRedAlliance = m_fmsTable->GetBooleanTopic(Configuration::NT::isRedAlliance).Subscribe(false);
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