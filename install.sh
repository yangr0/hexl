#!/bin/bash

sudo apt-get install cargo

cargo build

input() {
    read -n1 -p "Do you want this to be accessible anywhere(add to /bin)? [yn]" input

case $input in  

    y|Y) sudo mv target/debug/hexl /bin ;; 
    n|N) mv target/debug/hexl hexl ;; 
    *) echo "Invalid Options" && input ;;

esac
}
input