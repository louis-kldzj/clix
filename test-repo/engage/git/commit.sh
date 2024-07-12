#!/bin/bash

# git add, commit, push helper script

MESSAGE=$1

echo "executing: git add *"
git add *

echo "executing: git commit -m $MESSAGE"
git commit -m "$MESSAGE"

echo "executing: git push"
git push

echo "done"
