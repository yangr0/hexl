#!/bin/bash

sudo apt-get install cargo

cargo build

input() {
    read -n1 -p "Do you want this to be accessible anywhere(add to /bin)? [y/n]" input

case $input in  

    y|Y) sudo mv target/debug/hexl /bin && echo "Use the command \"hexl\" to run" ;; 
    n|N) mv target/debug/hexl hexl ;; 
    *) echo "Invalid Options" && input ;;

esac
}
input
