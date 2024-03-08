#!/bin/bash
set -evx

project="$1"
source "$script_dir/utils.sh"

bump_version $1
