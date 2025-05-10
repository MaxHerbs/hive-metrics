// Copyright 2025 Max Herbert

#include <Arduino.h>
#include <WiFi.h>
#include <ConfigManager.h>
#include <Settings.h>



void setup() {
  Serial.begin(115200);
  Serial.println("Some text");
  ConfigManager manager("/config.json", sdCsPin);
  Config config = manager.init();
  Serial.println(config.ssid);
  Serial.println(config.password);

  WiFi.begin(config.ssid, config.password);
}

void loop() {

}
