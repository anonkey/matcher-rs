#!/bin/sh

set -evx

script_dir=$(dirname "$0")

. "$script_dir/utils.sh"

bump_dependency_version $1 $2 $3
