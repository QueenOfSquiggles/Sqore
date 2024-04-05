#! /usr/bin/bash

read -p 'Bundle zip archive as well? y/n :]' do_bundle

echo "Building artifacts for Sqore"

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

echo "Building documentation"
cargo doc --no-deps --document-private-items --target-dir . \
	 > /dev/null \
	 & spinner
echo "   Done"

##
##
##


# Stage for local (using as a git submodule)

# Windows libraries

cp target/x86_64-pc-windows-gnu/debug/sqore.dll \
    target/debug/sqore.dll

cp target/x86_64-pc-windows-gnu/release/sqore.dll \
    target/release/sqore.dll

# Linux libraries
cp target/x86_64-unknown-linux-gnu/debug/libsqore.so \
    target/debug/libsqore.so

cp target/x86_64-unknown-linux-gnu/release/libsqore.so \
    target/release/libsqore.so

# Mac libraries
cp target/x86_64-apple-darwin/debug/libsqore.dylib \
    target/debug/libsqore.dylib

cp target/x86_64-apple-darwin/release/libsqore.dylib \
    target/release/libsqore.dylib

if [ "$do_bundle" == 'y' ]; then


	# make folder
	echo "Making folders"
	mkdir addons
	mkdir addons/sqore
	mkdir addons/sqore/target
	mkdir addons/sqore/target/debug
	mkdir addons/sqore/target/release

	echo "Copying files over"

	# metadata
	cp README.md addons/sqore/README.md
	cp LICENSE addons/sqore/LICENSE
	cp sqore.gdextension addons/sqore/sqore.gdextension

	# Stage for zip archive

	# Windows libraries

	cp target/x86_64-pc-windows-gnu/debug/sqore.dll \
		addons/sqore/target/debug/sqore.dll

	cp target/x86_64-pc-windows-gnu/release/sqore.dll \
		addons/sqore/target/release/sqore.dll

	# Linux libraries
	cp target/x86_64-unknown-linux-gnu/debug/libsqore.so \
		addons/sqore/target/debug/libsqore.so

	cp target/x86_64-unknown-linux-gnu/release/libsqore.so \
		addons/sqore/target/release/libsqore.so

	# Mac libraries
	cp target/x86_64-apple-darwin/debug/libsqore.dylib \
		addons/sqore/target/debug/libsqore.dylib

	cp target/x86_64-apple-darwin/release/libsqore.dylib \
		addons/sqore/target/release/libsqore.dylib

	echo "Copying folders over"
	# static files folders
	cp -r scenes addons/sqore/scenes/
	cp -r assets addons/sqore/assets/
	cp -r doc addons/sqore/doc

	echo "Creating zip archive"
	zip -r -q sqore_release addons && rm -r addons/
	echo "Your archive should be in this directory as 'sqore_release.zip'"
fi

echo "Done building"








