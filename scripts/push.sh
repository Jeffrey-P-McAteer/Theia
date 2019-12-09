#!/bin/sh

set -e

git push $1 $2 $3 sourcehut master
git push $1 $2 $3 github master

