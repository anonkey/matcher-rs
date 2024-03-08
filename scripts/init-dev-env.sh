#!/bin/bash

set -vx

cargo install conventional_commits_next_version --version ^6
cargo install conventional_commits_linter git-cliff toml-cli

if [ -d .git/hooks -a -z "$SKIP_GIT_HOOKS" ]; then
  for file in git-hooks/*; do
    ln -s ../../$file .git/hooks/
  done
fi

