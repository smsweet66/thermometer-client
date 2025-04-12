#!/bin/bash

echo "70" > /dev/thermometer

cargo build
cargo run

sleep 1

expected_result='{"temperature":70,"id":"sensor01","name":"office","connected":true}'
result=$(curl -X GET "http://127.0.0.1:8000/temperature")

exit_code=0

if [ "$result" == "$expected_result" ]; then
    echo "Test Passed"
else
    echo "Test Failed: Expected $expected_result, got $result"
    exit_code=1
fi

start-stop-daemon -K -n thermometer
exit $exit_code
