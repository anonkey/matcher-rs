#!/bin/sh

get_version() {
  cargo_path="$1"
  current_version=`toml get $cargo_path package.version`
  current_version=${current_version//\"/}

  echo "$current_version"
}
