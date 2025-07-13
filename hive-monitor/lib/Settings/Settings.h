// Copyright 2025 Max Herbert

#ifndef SETTINGS_H
#define SETTINGS_H

#include <Arduino.h>


#define METRICS_PERIOD 10000
#define SD_PIN 5
#define ONE_WIRE_BUS_PIN 4

inline void system_exit(char* message ) {
    Serial.println("Critical error:");
    Serial.println(message);

    for (;;) {
        delay(1000);
    }
}

#endif
