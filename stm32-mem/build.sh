#!/bin/bash

PROJ="${PWD##*/}"
BUILD="debug"
OUT="$PROJ.bin"

if [[ "$@" = *--release* ]]; then
	BUILD="release"
fi

xargo build $@

if [ $? -eq "0" ]; then
	# clear
	arm-none-eabi-objcopy -O binary "target/thumbv7m-none-eabi/$BUILD/$PROJ" "$OUT"
fi
