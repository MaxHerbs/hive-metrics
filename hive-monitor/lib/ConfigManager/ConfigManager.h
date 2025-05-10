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
