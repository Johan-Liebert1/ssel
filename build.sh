#!/bin/bash

cargo build --release

sudo mv ./target/release/ssel /usr/bin
