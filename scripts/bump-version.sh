#!/bin/bash
set -evx

script_dir=$(dirname "${BASH_SOURCE[0]}")

project="$1"

source "$script_dir/utils.sh"

bump_version $1
