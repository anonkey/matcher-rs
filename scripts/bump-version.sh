#!/bin/sh
set -evx

project="$1"
shift;
cliff_args="$*"
script_dir=$(dirname "${BASH_SOURCE[0]}")

source "$script_dir/utils.sh"

project_path=`get_path $project`
cargo_path=`get_cargo $project`
current_version=`get_version $project`
current_tag=`get_current_tag $project`
current_tag_commit=`get_current_commit $project`

next_version=`get_next_version $project`

if [ "$current_version" = "$next_version" ];
then
  echo "Version is already $next_version"
  exit 0
fi

echo "Bumping $project from $current_version to $next_version"

git cliff $cliff_args --tag "$next_version" --include-path "$project_path/*"  -u --prepend "$project_path/CHANGELOG.md" "$current_tag_commit"..HEAD

new_cargo=`toml set $cargo_path package.version $next_version`

echo "$new_cargo" > $cargo_path

package_path=`get_path`

for project_to_update in `ls $package_path`; do
  echo "Bumping $project_to_update"
  version=`get_dependency_version "$project_to_update" "$project"`

  if [ "$version" != "" ];
  then
    bump_dependency_version "$project_to_update" "$project" "$next_version";
  fi
done

git tag $project-$next_version
