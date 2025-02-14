#!/bin/bash

INCLUDE_PATH=$(pkg-config --cflags-only-I filerix | sed 's/-I//g')

bindgen wrapper.h -o bindings.rs -- -I"$INCLUDE_PATH"
