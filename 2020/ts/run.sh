#! /bin/sh

deno run --allow-read "./day$(printf "%02d" $1)/part$2.ts"
