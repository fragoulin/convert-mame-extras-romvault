#!/bin/bash

if [ -z "$1" ]
    then
        echo "Please supply version"
    else
        rm releases/*

        echo "Building for version $1"

        declare -A envs=(
            ["win32"]="i686-pc-windows-gnu/release/convert-mame-extras-romvault.exe"
            ["win64"]="/x86_64-pc-windows-gnu/release/convert-mame-extras-romvault.exe"
            ["linux64"]="/release/convert-mame-extras-romvault"
        )

        for env in "${!envs[@]}"; do
            echo "build $env"
            cargo "build_$env"
            zip -j releases/convert_mame_extras_romvault-$env-$1.zip target/${envs[$env]}
        done
fi
