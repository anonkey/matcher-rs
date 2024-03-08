#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
set -eEuo pipefail
IFS=$'\n\t'

retry() {
    for i in {1..10}; do
        if "$@"; then
            return 0
        else
            sleep "${i}"
        fi
    done
    "$@"
}
bail() {
    echo "::error::$*"
    exit 1
}
warn() {
    echo "::warning::$*"
}
download_and_checksum() {
    local url="${1:?}"
    local checksum="${2:?}"
    retry curl --proto '=https' --tlsv1.2 -fsSL --retry 10 "${url}" -o tmp
    if type -P sha256sum &>/dev/null; then
        echo "${checksum} *tmp" | sha256sum -c - >/dev/null
    elif type -P shasum &>/dev/null; then
        # GitHub-hosted macOS runner does not install GNU Coreutils by default.
        # https://github.com/actions/runner-images/issues/90
        echo "${checksum} *tmp" | shasum -a 256 -c - >/dev/null
    else
        warn "checksum requires 'sha256sum' or 'shasum' command; consider installing one of them; skipped checksum for $(basename "${url}")"
    fi
}

if [[ $# -gt 0 ]]; then
    bail "invalid argument '$1'"
fi



case "${draft}" in
    true) release_options+=("--draft") ;;
    false) ;;
    *) bail "'draft' input option must be 'true' or 'false': '${draft}'" ;;
esac
case "${allow_missing_changelog}" in
    true | false) ;;
    *) bail "'allow_missing_changelog' input option must be 'true' or 'false': '${allow_missing_changelog}'" ;;
esac

# https://cli.github.com/manual/gh_release_view
if GITHUB_TOKEN="${token}" gh release view "${tag}" &>/dev/null; then
    # https://cli.github.com/manual/gh_release_delete
    GITHUB_TOKEN="${token}" gh release delete "${tag}" -y || true
fi

# https://cli.github.com/manual/gh_release_create
GITHUB_TOKEN="${token}" retry gh release create "${release_options[@]}" --title "${title}" --notes "${notes:-}"

# Set (computed) prefix and version outputs for future step use.
computed_prefix=${tag%"${version}"}
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
    echo "computed-prefix=${computed_prefix}" >>"${GITHUB_OUTPUT}"
    echo "version=${version}" >>"${GITHUB_OUTPUT}"
else
    # Self-hosted runner may not set GITHUB_OUTPUT.
    warn "GITHUB_OUTPUT is not set; skip setting 'computed-prefix' and 'version' outputs"
    echo "computed-prefix: ${computed_prefix}"
    echo "version: ${version}"
fi
