#!/usr/bin/fish

echo "Creating new..."
set target (./scripts/new.rb)
cd $target
code .
../watch.sh
