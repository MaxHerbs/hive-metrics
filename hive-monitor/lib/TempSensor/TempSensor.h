// Copyright 2025 Max Herbert

#ifndef TEMPSENSOR_H
#define TEMPSENSOR_H

#include <OneWire.h>
#include <DallasTemperature.h>

class TempSensor {
    public:
        explicit TempSensor(uint8_t pin);
        void begin();
        float readTemperature();

    private:
        OneWire oneWire;
        DallasTemperature sensor;
};

#endif
