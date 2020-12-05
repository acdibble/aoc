#! /bin/sh

printf -v day "%02d" $1

deno run --allow-read "./day$day/part$2.ts"
