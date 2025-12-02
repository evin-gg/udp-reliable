#!/bin/bash

num="$1"
proxy_target="proxy${num}"

kitty --title client sh -c "make client; exec sh" &
kitty --title server sh -c "make server; exec sh" &
kitty --title proxy sh -c "make ${proxy_target}; exec sh" &

