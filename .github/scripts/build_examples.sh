#!/bin/sh

for path in examples/*; do
    if [ ! -d "$path" ]; then
        continue
    fi
    example=$(basename "$path")
    echo "building: $example"
    (
        cd "$path" &&
        trunk build --public-url "/$example/" &&
        # workaround until trunk can have relative "dist" paths with multiple levels
        distpath="../../dist/$example" &&
        mkdir -pv "$distpath" &&
        mv -v dist/* "$distpath/"
    )
    # exit early if any of the examples fails
    built=$?
    if [ "$built" != "0" ]; then
        exit "$built"
    fi
done
