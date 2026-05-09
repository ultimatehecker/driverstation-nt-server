#pragma once

#include <string>
#include <vector>

#ifdef _WIN32
#include <windows.h>
#endif

class SerialPort {
public:
    SerialPort();
    ~SerialPort();

    bool autoConnect(int baudRate);
    bool open(const std::string& portName, int baudRate);
    bool write(const std::string& data);
    bool isOpen() const;
    void close();

    static std::vector<std::string> listPorts();

private:
    bool openPort(const std::string& portName, int baudRate);

#ifdef _WIN32
    HANDLE handle = INVALID_HANDLE_VALUE;
#else
    int m_fd = -1;
#endif
};