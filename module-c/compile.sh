#!/usr/bin/env bash

gcc -c -Wall -Werror -fpic module.c
gcc -shared -o libmodule.so module.o
