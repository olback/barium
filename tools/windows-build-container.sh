#!/bin/sh

docker run \
    -it \
    --volume=$(pwd):/home/circleci/project \
    --volume=$(pwd)/.cargo-cache:/home/circleci/.cargo/registry \
    olback/rust-gtk-windows /bin/bash \
