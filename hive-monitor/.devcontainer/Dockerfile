FROM python:3.12 as developer

RUN curl -fsSL -o get-platformio.py https://raw.githubusercontent.com/platformio/platformio-core-installer/master/get-platformio.py
RUN python3 get-platformio.py
ENV PATH="$PATH:/root/.platformio/penv/bin"
RUN pip install cpplint