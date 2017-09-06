#!/bin/bash

PROJ=${PWD##*/}

BUILD="debug"
if [[ $* == *--release* ]]; then
	BUILD="release"
fi


arm-none-eabi-objdump -d "target/thumbv7m-none-eabi/$BUILD/$PROJ"
