#!/bin/bash

kitty --title client sh -c "make client; exec sh" &
kitty --title server sh -c "make server; exec sh" &
kitty --title proxy sh -c "make proxy; exec sh" &
