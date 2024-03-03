#!/bin/sh

set -evx

project="$1"
dependency="$2"
script_dir=$(dirname "$0")

. "$script_dir/utils.sh"

version=`get_dependency_version "$project" "$dependency"`

echo "$version"
