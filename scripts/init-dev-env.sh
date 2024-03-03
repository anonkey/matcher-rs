cargo install conventional_commits_next_version --version ^6
cargo install conventional_commits_linter
cargo install git-cliff

if [ -d .git/hooks ]; then
  for file in git-hooks/*; do
    ln -s ../../$file .git/hooks/
  done
fi

