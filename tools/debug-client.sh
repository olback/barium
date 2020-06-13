#!/bin/sh

export GOBJECT_DEBUG=instance-count
export GTK_DEBUG=interactive
cargo r --release --bin barium-client -- $1
