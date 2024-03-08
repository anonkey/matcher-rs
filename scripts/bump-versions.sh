#!/bin/bash
set -vx

script_dir=$(dirname "${BASH_SOURCE[0]}")

source "$script_dir/utils.sh"

bump_version matcher-derive-impl
bump_version matcher-derive

[[ `git status --porcelain` ]]
