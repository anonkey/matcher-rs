#!/bin/sh

set -evx

script_dir=$(dirname "${BASH_SOURCE[0]}")

source "$script_dir/utils.sh"

get_version "$1"

