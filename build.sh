#!/bin/bash

cargo build --release

sudo mv ./target/release/idk /usr/bin
