#!/bin/bash

get_version() {
  typeset cargo_path=`get_cargo $1`

  typeset current_version=`toml get $cargo_path package.version`
  typeset current_version=${current_version//\"/}

  echo "$current_version"
}

get_path() {
  echo "packages/$1"
}

get_cargo() {
  echo "`get_path $1`/Cargo.toml"
}

get_current_tag() {
  echo "$1-`get_version $1`"
}

get_current_commit() {
  typeset current_tag=`get_current_tag $1`

  echo `git rev-list -n 1 $current_tag`
}

get_dependency_version_data() {
  typeset cargo_path=`get_cargo $1`

  typeset version=`toml get "$cargo_path" dependencies."$2" || echo '""'`

  echo "$version"
}

get_dependency_version() {
  typeset version=`get_dependency_version_data "$1" "$2"`
  typeset type=`echo $version | jq -r 'type' || echo ""`

  if [ "$type" != "string" ]; then
      typeset version=`echo $version | jq '.version'`
  fi

  typeset version=${version//\"/}

  echo "$version"
}

get_dependency_path() {
  typeset cargo_path=`get_cargo $1`

  typeset version=`get_dependency_version_data "$1" "$2"`

  typeset type=`echo $version | jq -r 'type'`

  typeset suffix=""

  if [ "$type" != "string" ]; then
      typeset suffix=".version"
  fi

  echo dependencies."$dependency""$suffix"
}

get_next_version() {
  conventional_commits_next_version --calculation-mode "Batch" --from-reference `get_current_tag $1` --from-version `get_version $1` --monorepo `get_path $1`
}


bump_dependency_version() {
  typeset project="$1"
  typeset cargo_path=`get_cargo $1`
  typeset dependency="$2"
  typeset new_version="$3"

  typeset dependency_path=`get_dependency_path "$project" "$dependency"`

  typeset new_cargo=`toml set "$cargo_path" "$dependency_path" "$new_version"`

  echo "$new_cargo" > "$cargo_path"
}
