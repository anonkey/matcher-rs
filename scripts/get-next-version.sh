#!/bin/sh

set -evx

script_dir=$(dirname "$0")

. "$script_dir/utils.sh"

get_next_version "$1"
