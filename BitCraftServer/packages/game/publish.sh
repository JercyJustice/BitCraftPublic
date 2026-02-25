#!/bin/bash

read -p "Enter the spacetime server name (e.g. bitcraft-staging): " host

for i in {2..25}; do
  spacetime publish -s "$host" "bitcraft-live-$i" -y
done
