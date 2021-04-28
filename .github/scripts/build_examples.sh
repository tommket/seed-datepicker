#!/bin/sh

for path in examples/*; do
    if [[ ! -d $path ]]; then
        continue
    fi
    example=$(basename "$path")
    echo "building: $example"
    (
        cd "$path" &&
        trunk build --public-url "/dist/$example/" &&
        # workaround until trunk can have relative "dist" paths with multiple levels
        distpath="../../dist/$example" &&
        mkdir -pv "$distpath" &&
        mv -v dist/* "$distpath/"
    )
done
