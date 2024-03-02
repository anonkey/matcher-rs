#!/bin/sh

set -evx

project="$1"
dependency="$2"
script_dir=$(dirname "${BASH_SOURCE[0]}")

source "$script_dir/utils.sh"

version=`get_dependency_version "$project" "$dependency"`

echo "$version"
