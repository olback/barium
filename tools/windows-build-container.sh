#!/bin/sh

docker run \
    -it \
    --volume=$(pwd):/home/circleci/project \
    --volume=$(pwd)/../tray-indicator:/home/circleci/tray-indicator \
    olback/rust-gtk-windows /bin/bash \
