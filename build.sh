#!/bin/bash

if [ -z "$1" ]
    then
        echo "Please supply version"
    else
        rm releases/*

        echo "Building for version $1"

        declare -A cmds=(
            ["win32"]="zip -j releases/convert_mame_extras_romvault-win32-$1.zip target/i686-pc-windows-gnu/release/convert-mame-extras-romvault.exe"
            ["win64"]="zip -j releases/convert_mame_extras_romvault-win64-$1.zip target/x86_64-pc-windows-gnu/release/convert-mame-extras-romvault.exe"
            ["linux64"]="tar -cvzf releases/convert_mame_extras_romvault-linux64-$1.tgz --directory=target/release convert-mame-extras-romvault"
        )

        for env in "${!cmds[@]}"; do
            echo "build $env"
            cargo "build_$env"
            ${cmds[$env]}
        done
fi
