#!/bin/bash

result="$(cargo run)"

if [ "$result" == "Hello, world!" ]; then
    echo "Test Passed"
else
    echo "Test Failed: Expected \"Hello, world!\", got $result"
    exit 1
fi