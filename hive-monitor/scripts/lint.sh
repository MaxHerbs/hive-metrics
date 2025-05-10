cpplint --repository=. --recursive --linelength=120 --verbose=4 --filter=-build/include,-build/header_guard ./src
cpplint --repository=. --recursive --exclude ./lib/ArduinoJson --linelength=120 --verbose=4 --filter=-build/include,-build/header_guard ./lib
