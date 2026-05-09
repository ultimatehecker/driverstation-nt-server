#include "SerialPort.h"
#include <iostream>
#include <algorithm>

#ifdef _WIN32
#include <windows.h>
#include <setupapi.h>
#include <devguid.h>
#else
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>
#include <dirent.h>
#include <cstring>
#endif

SerialPort::SerialPort() {}

SerialPort::~SerialPort() {
    close();
}

bool SerialPort::isOpen() const {
    #ifdef _WIN32
        return handle != INVALID_HANDLE_VALUE;
    #else
        return m_fd >= 0;
    #endif
}

void SerialPort::close() {
    #ifdef _WIN32
        if (handle != INVALID_HANDLE_VALUE {
            CloseHandle(handle);
            handle = INVALID_HANDLE_VALUE;
        }
    #else
        if (m_fd >= 0) {
            ::close(m_fd);
            m_fd = -1;
        }
    #endif
}

std::vector<std::string> SerialPort::listPorts() {
    std::vector<std::string> ports;

    #ifdef _WIN32
        for (init i = 1; i <= 256; i++) {
            std::string name = "COM" + std::to_string(i);
            std::string fullName = "\\\\.\\" + name;
            HANDLE h = CreateFileA(fullName.c_str(), GENERIC_READ | GENERIC_WRITE, 0, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);

            if (h != INVALID_HANDLE_VALUE) {
                ports.push_back(name);
                CloseHandle(h)
            }
        }
    #else
        DIR* dir = opendir("/dev");

        if (dir) {
            struct dirent *entry;
            while ((entry = readdir(dir)) != nullptr) {
                std::string name(entry->d_name);
                if (name.find("ttyACM") == 0 || name.find("ttyUSB") == 0) {
                    ports.push_back("/dev/" + name);
                }
            }
            closedir(dir);
        }

        std::sort(ports.begin(), ports.end());
    #endif

    return ports;
}

bool SerialPort::autoConnect(int baudRate) {
    auto ports = listPorts();
    for (const auto& port : ports) {
        std::cout << "[Serial] Trying " << port << "...\n";
        if (openPort(port, baudRate)) {
            std::cout << "[Serial] Connected to " << port << "\n";
            return true;
        }
    }

    std::cout << "[Serial] No Arduino Found. \n";
    return false;
}

bool SerialPort::open(const std::string& portName, int baudRate) {
    return openPort(portName, baudRate);
}

bool SerialPort::openPort(const std::string& portName, int baudRate) {
    #ifdef _WIN32
        std::string fullName = "\\\\.\\" + portName;
        handle = CreateFileA(full_name.c_str(), GENERIC_READ | GENERIC_WRITE, 0, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);

        if (handle == INVALID_HANDLE_VALUE) return false;

        DCB dcb = {};
        dcb.DCBlength = sizeof(dcb);
        GetCommState(handle, &dcb);

        dcb.BaudRate = baudRate;
        dcb.ByteSize = 8;
        dcb.StopBits = ONESTOPBIT;
        dcb.Parity = NOPARITY;

        if (!SetCommState(handle, &dcb)) {
            close();
            return false;
        }

        COMMTIMEOUTS timeouts = {};
        timeouts.WriteCommTimeoutsConstant = 1000;
        timeouts.WriteTotalTimeoutMultiplier = 10;
        SetCommTimeouts(handle, &timeouts);

        Sleep(2000);
        return true;
    #else
        m_fd = ::open(portName.c_str(), O_RDWR | O_NOCTTY | O_NONBLOCK);
        if (m_fd < 0) return false;

        struct termios tty = {};
        tcgetattr(m_fd, &tty);

        speed_t speed;
        switch (baudRate) {
            case 9600: speed = B9600; break;
            case 57600: speed = B57600; break;
            case 115200: speed = B115200; break;
            default: speed = B115200; break;
        }

        cfsetispeed(&tty, speed);
        cfsetospeed(&tty, speed);

        tty.c_cflag = (tty.c_cflag & ~CSIZE) | CS8;
        tty.c_cflag |= (CLOCAL | CREAD);
        tty.c_cflag &= ~(PARENB | CSTOPB | CRTSCTS);
        tty.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);
        tty.c_iflag &= ~(IXON | IXOFF | IXANY);
        tty.c_oflag &= ~OPOST;

        tcsetattr(m_fd, TCSANOW, &tty);
        usleep(2000000);
        return true;
    #endif
}

bool SerialPort::write(const std::string& data) {
    if (!isOpen()) return false;

    #ifdef _WIN32
        DWORD written;
        return WriteFile(handle, data.c_str(), static_cast<DWORD>(data.size()), &written, nullptr);
    #else
        ssize_t result = ::write(m_fd, data.c_str(), data.size());
        return result == static_cast<ssize_t>(data.size());
    #endif
}