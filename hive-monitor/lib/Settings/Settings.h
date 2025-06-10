// Copyright 2025 Max Herbert

#ifndef SETTINGS_H
#define SETTINGS_H

#include <Arduino.h>


#define UPDATE_FREQ 30
#define sdCsPin 5

inline void system_exit(char* message ) {
    Serial.println("Critical error:");
    Serial.println(message);

    for (;;) {
        delay(1000);
    }
}

#endif
