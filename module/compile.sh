#!/usr/bin/env bash

gcc -c -Wall -Werror -fpic src/module.c -o target/module-c.o
gcc -shared -o target/libmodule-c.so target/module-c.o
