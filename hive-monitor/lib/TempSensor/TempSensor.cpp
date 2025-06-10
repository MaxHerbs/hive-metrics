// Copyright 2025 Max Herbert

#include "TempSensor.h"

TempSensor::TempSensor(uint8_t pin) : oneWire(pin), sensor(&oneWire) {}

void TempSensor::begin() {
    sensor.begin();
}

float TempSensor::readTemperature() {
    sensor.requestTemperatures();
    return sensor.getTempCByIndex(0);
}
