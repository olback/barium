#!/bin/sh

docker run \
    -it \
    --volume=$(pwd):/home/circleci/project \
    olback/rust-gtk-windows /bin/bash \
