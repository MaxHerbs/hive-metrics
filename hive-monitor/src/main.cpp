// Copyright 2025 Max Herbert

#include <Arduino.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include <ConfigManager.h>
#include <Settings.h>
#include <TempSensor.h>

TempSensor temp_sensor(ONE_WIRE_BUS_PIN);

void wait_for_wifi();

Config config;
int64_t previousSeconds = 0;


void setup() {
  Serial.begin(115200);
  Serial.println("Some text");
  ConfigManager manager("/config.json", SD_PIN);
  config = manager.init();
  config.print();


  temp_sensor.begin();

  WiFi.begin(config.ssid, config.password);
  wait_for_wifi();
}

void loop() {
  int64_t currentSeconds = millis()/1000;

  if ((currentSeconds - previousSeconds) < UPDATE_FREQ) {
    delay(10);
    return;
  }

  if (WiFi.status() != WL_CONNECTED) {
    Serial.println("WiFi disconnected. Reconnecting...");
    WiFi.disconnect();
    WiFi.begin(config.ssid, config.password);
    wait_for_wifi();
  }

  WiFiClient client;
  HTTPClient http;

  http.begin(client, config.hostname);
  http.addHeader("Content-Type", "application/json");


  JsonDocument doc;
  doc["id"] = config.id;
  doc["location"] = config.location;



  JsonObject metrics = doc.createNestedObject("metrics");
  float temperature = temp_sensor.readTemperature();
  metrics["temperature"] = temperature;

  String post_body;
  serializeJson(doc, post_body);
  Serial.println("JSON to POST:");
  Serial.println(post_body);

  int response = http.POST(post_body);
  Serial.print("Response Code:");
  Serial.println(response);

  http.end();
  previousSeconds = millis()/1000;

}


void wait_for_wifi() {
  Serial.println("Connecting");
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("Connected");
}
