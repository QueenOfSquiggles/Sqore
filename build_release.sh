#! /usr/bin/bash

echo "Building artifacts for Squiggles Core"

spinner()
{
	# function taken from https://stackoverflow.com/a/20369590
    local pid=$!
    local delay=0.25
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

echo "Verifying target installation"

echo "Ensuring docker is currently running"
dockerd &> /dev/null


echo "Building targets (this are containerized and may take a while UwU)"

echo " - Linux x86 64 Debug"
cross build -q --target=x86_64-unknown-linux-gnu \
    > /dev/null \
    & spinner
echo "   Done"

echo " - Linux x86 64 Release"
cross build -q  -r --target=x86_64-unknown-linux-gnu \
    > /dev/null \
    & spinner
echo "   Done"

echo " - Windows x64 Debug"
cross build -q --target=x86_64-pc-windows-gnu \
    > /dev/null \
    & spinner
echo "   Done"

echo " - Windows x64 Release"
cross build -q -r --target=x86_64-pc-windows-gnu \
    > /dev/null \
    & spinner
echo "   Done"

echo " - Mac Debug"
cross build -q --target=x86_64-apple-darwin \
    > /dev/null \
    & spinner
echo "   Done"

echo " - Mac Release"
cross build -q -r --target=x86_64-apple-darwin \
    > /dev/null \
    & spinner
echo "   Done"

echo "All builds complete"
echo "---"

##
##
##

# make folder
echo "Making folders"
mkdir addons
mkdir addons/squiggles_core
mkdir addons/squiggles_core/target
mkdir addons/squiggles_core/target/debug
mkdir addons/squiggles_core/target/release

echo "Copying files over"

# metadata
cp README.md addons/squiggles_core/README.md
cp LICENSE addons/squiggles_core/LICENSE
cp squiggles_core.gdextension addons/squiggles_core/squiggles_core.gdextension

# Windows libraries

cp target/x86_64-pc-windows-gnu/debug/squiggles_core.dll \
    addons/squiggles_core/target/debug/squiggles_core.dll

cp target/x86_64-pc-windows-gnu/release/squiggles_core.dll \
    addons/squiggles_core/target/release/squiggles_core.dll

# Linux libraries
cp target/x86_64-unknown-linux-gnu/debug/libsquiggles_core.so \
    addons/squiggles_core/target/debug/libsquiggles_core.so

cp target/x86_64-unknown-linux-gnu/release/libsquiggles_core.so \
    addons/squiggles_core/target/release/libsquiggles_core.so

# Mac libraries
cp target/x86_64-apple-darwin/debug/libsquiggles_core.dylib \
    addons/squiggles_core/target/debug/libsquiggles_core.dylib

cp target/x86_64-apple-darwin/release/libsquiggles_core.dylib \
    addons/squiggles_core/target/release/libsquiggles_core.dylib

echo "Copying folders over"

# static files folders
cp -r scenes addons/squiggles_core/scenes/
cp -r assets addons/squiggles_core/assets/

echo "Creating zip archive"
zip -r -q squiggles_core_release addons && rm -r addons/

echo "Your archive should be in this directory as 'squiggles_core_release.zip'"







