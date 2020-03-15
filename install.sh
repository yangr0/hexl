#!/bin/bash

sudo apt-get install cargo

cargo build

mv target/debug/hexl hexl