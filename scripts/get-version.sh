#!/bin/sh

set -evx

script_dir=$(dirname "$0")

source "$script_dir/utils.sh"

get_version "$1"

