#!/bin/bash

cargo run &

sleep 10

expected_result='{"tempurature":70,"id":"sensor01","name":"office","connected":true}'
result=$(curl -X GET "http://127.0.0.1:8000/tempurature")

if [ "$result" == "$expected_result" ]; then
    echo "Test Passed"
else
    echo "Test Failed: Expected $expected_result, got $result"
    exit 1
fi