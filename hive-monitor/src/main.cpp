// Copyright 2025 Max Herbert

#include <Arduino.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include <ConfigManager.h>
#include <Settings.h>
#include <TempSensor.h>
#include <TaskScheduler.h>

TempSensor temp_sensor(ONE_WIRE_BUS_PIN);

void wait_for_wifi();

Config config;
int64_t previousSeconds = 0;


String auth_token = "";

void sendMetricsCallback();
void authenticationCallback();
Task metrics(METRICS_PERIOD, TASK_FOREVER, &sendMetricsCallback);
Task authentication(AUTHENTICATION_PERIOD, TASK_FOREVER, &sendMetricsCallback);
Scheduler runner;


void setup() {
  Serial.begin(115200);
  Serial.println("Some text");
  ConfigManager manager("/config.json", SD_PIN);
  config = manager.init();
  config.print();


  temp_sensor.begin();

  WiFi.begin(config.ssid, config.password);
  wait_for_wifi();
  authenticationCallback();

  runner.addTask(metrics);
  runner.addTask(authentication);
  metrics.enable();
  authentication.enable();
}

void loop() {
  if (WiFi.status() != WL_CONNECTED) {
    Serial.println("WiFi disconnected. Reconnecting...");
    WiFi.disconnect();
    WiFi.begin(config.ssid, config.password);
    wait_for_wifi();
  }

  runner.execute();
}


void wait_for_wifi() {
  Serial.println("Connecting");
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("Connected");
}


void sendMetricsCallback() {
  if (auth_token == "") {
    Serial.println("Auth token was null. Skipping submisson");
    return;
  }

  WiFiClientSecure secureClient;
  HTTPClient http;
  secureClient.setCACert(root_cert);

  if (!http.begin(secureClient, config.hostname)) {
    Serial.println("[TELEMETRY] Unable to connect");
    return;
  }
  http.addHeader("Content-Type", "application/json");
  String auth = "Bearer :" + auth_token;
  http.addHeader("Authorization", auth);

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
}

void authenticationCallback() {
  WiFiClientSecure secureClient;
  HTTPClient http;
  secureClient.setCACert(root_cert);

  if (!http.begin(secureClient, config.auth_endpoint)) {
    Serial.println("[AUTH] Unable to connect");
    return;
  }

  http.addHeader("Content-Type", "application/x-www-form-urlencoded");

  String postData =
      "grant_type=client_credentials"
      "&client_id=" + String(config.client) +
      "&client_secret=" + String(config.client_secret);

  Serial.println("Requesting new auth token");
  int response_code = http.POST(postData);

  if (response_code != 200) {
    Serial.printf("[AUTH] Request failed, error: %s\n", http.errorToString(response_code).c_str());
    auth_token = "";
    return;
  }

  Serial.printf("[AUTH] Response code: %d\n", response_code);
  String response_body = http.getString();
  Serial.println("[AUTH] Token response:");
  JsonDocument json;
  DeserializationError error = deserializeJson(json, response_body);
  if (error) {
    auth_token = "";
    Serial.println(error.f_str());
    return;
  }

  auth_token = json["access_token"].as<String>();
  http.end();
}
