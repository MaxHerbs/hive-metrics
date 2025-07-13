// Copyright 2025 Max Herbert

#ifndef CONFIG_MANAGER_H
#define CONFIG_MANAGER_H

#include <Arduino.h>
#include <Settings.h>
#include <ArduinoJson.h>
#include "FS.h"
#include "SD.h"
#include "SPI.h"

struct Config {
    char ssid[64];
    char password[64];
    char hostname[128];

    char auth_endpoint[128];
    char client[64];
    char client_secret[64];

    char id[32];
    char location[32];

    void print() const {
        Serial.println("Config:");
        Serial.print("  SSID: ");
        Serial.println(ssid);
        Serial.print("  Password: ");
        Serial.println(password);
        Serial.print("  Hostname: ");
        Serial.println(hostname);
        Serial.print("  ID: ");
        Serial.println(id);
        Serial.print("  Location: ");
        Serial.println(location);
        Serial.print("  Auth Endpoint: ");
        Serial.println(auth_endpoint);
        Serial.print("  Client: ");
        Serial.println(client);
    }
};


class ConfigManager {
public:
    ConfigManager(const char* _fileName, int _sdCs);

    Config init();
    char* readFile(fs::FS &fs, const char *path);

    JsonDocument json;

private:
    const char* fileName;
    int sdCs;

    Config loadConfig(char* configText);

};

#endif
