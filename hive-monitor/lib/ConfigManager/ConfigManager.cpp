// Copyright 2025 Max Herbert

#include "ConfigManager.h"
#include <Arduino.h>
#include <Settings.h>
#include <ArduinoJson.h>
#include "FS.h"
#include "SD.h"
#include "SPI.h"

#define max_str_len 4096

ConfigManager::ConfigManager(const char *_fileName, int _sdCs) {
    fileName = _fileName;
    sdCs = _sdCs;
    Serial.println(fileName);

    if (!SD.begin(_sdCs)) {
        system_exit("Card Mount Failed");
    }

    uint8_t cardType = SD.cardType();
    if (cardType == CARD_NONE) {
        system_exit("No SD card found");
    }
    Serial.print("SD card mounted with size: ");
    Serial.println(SD.cardSize() / (1024 * 1024));
}

Config ConfigManager::init() {
    char *configString = readFile(SD, fileName);

    if (configString == nullptr) {
        system_exit("Failed to read config file");
    }
    Config config = loadConfig(configString);
    free(configString);

    return config;
}

Config ConfigManager::loadConfig(char *configText) {
    DeserializationError error = deserializeJson(json, configText);
    if (error) {
        Serial.println(error.f_str());
        system_exit("Failed to deserialise json");
    }

    const char* requiredKeys[] = { "ssid", "password", "hostname", "id", "location", "client", "client_secret", "auth_endpoint" };
    const size_t numKeys = sizeof(requiredKeys) / sizeof(requiredKeys[0]);

    bool allKeysPresent = true;

    for (size_t i = 0; i < numKeys; i++) {
        if (json[requiredKeys[i]].isNull()) {
            Serial.print("Missing key: ");
            Serial.println(requiredKeys[i]);
            allKeysPresent = false;
        }
    }

    if (!allKeysPresent) {
        system_exit("Missing config keys");
    }
    Config config;

    strlcpy(config.ssid, json["ssid"], sizeof(config.ssid));
    strlcpy(config.password, json["password"], sizeof(config.password));
    strlcpy(config.hostname, json["hostname"], sizeof(config.hostname));
    strlcpy(config.id, json["id"], sizeof(config.id));
    strlcpy(config.location, json["location"], sizeof(config.location));
    strlcpy(config.auth_endpoint, json["auth_endpoint"], sizeof(config.auth_endpoint));
    strlcpy(config.client, json["client"], sizeof(config.client));
    strlcpy(config.client_secret, json["client_secret"], sizeof(config.client_secret));
    return config;
}

char *ConfigManager::readFile(fs::FS &fs, const char *path) {
    Serial.printf("Reading file: %s\n", path);

    File file = fs.open(path);
    if (!file) {
        Serial.println("Failed to open file for reading");
        return nullptr;
    }

    char *fileStream = reinterpret_cast<char *>(malloc(max_str_len));
    if (!fileStream) {
        Serial.println("Failed to allocate memory");
        file.close();
        return nullptr;
    }

    size_t bytesRead = file.readBytes(fileStream, max_str_len);
    fileStream[bytesRead] = '\0';

    file.close();

    return fileStream;
}
