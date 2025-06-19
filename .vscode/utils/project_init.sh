#!/bin/bash

if ! [ -f Cargo.toml ] ; then
    cargo init
    cargo new lib --lib

    echo "lib = { path = 'lib' }" >> "Cargo.toml"

    echo -e "use lib;\n\n$(cat src/main.rs)" > "src/main.rs"
    sed -i 's/"Hello, world!"/"Hello, world from lib {}!", lib::add(1, 2)/g' "src/main.rs"

    cargo add trpl
    echo 'openssl = { version = "0.10", features = ["vendored"] }' >> "Cargo.toml"
fi
