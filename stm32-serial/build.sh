#!/bin/bash

PROJ="${PWD##*/}"
BUILD="debug"

if [[ "$@" = *--release* ]]; then
	BUILD="release"
fi

xargo build $@

if [ $? -eq "0" ]; then
	# clear
	rm "$PROJ.bin"
	arm-none-eabi-objcopy -O binary "target/thumbv7m-none-eabi/$BUILD/$PROJ" "$PROJ.bin"
fi
