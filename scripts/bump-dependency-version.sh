#!/bin/bash

set -evx

script_dir=$(dirname "${BASH_SOURCE[0]}")

source "$script_dir/utils.sh"

bump_dependency_version $1 $2 $3
