#!/usr/bin/env bash
#
# A helper script for pulling upstream versions of sqlparser[1] in to
# packages/sqltk-parser, a soft fork of sqlparser, with symbols renamed,
# and some additional features (in an aspirationally temporary manner, until
# they're added or merged upstream).
#
#   1. https://github.com/apache/datafusion-sqlparser-rs
#
# Usage: pull-upstream-sqlparser.sh <VERSION>

set -eEu

### Preconditions

if [ -z "${1:-}" ]; then
  echo "Usage: $(basename "$0") revision-of-sqlparser"
  echo "   eg. $(basename "$0") v0.53.0"
  exit 1
else
  REV=$1
fi

PKG_ROOT=$(unset CDPATH; cd "$(dirname "$0")" && pwd)
if [ ! -d "${PKG_ROOT}/.git" ]; then
  echo "Please run from the repo root."
  exit 2
fi

if command -v gsed >/dev/null; then
  # Probably on MacOS w/ gnu-sed
  SED=$(command -v gsed)
elif [ "$(sed --version | grep -c GNU)" -gt 0 ]; then
  # Probably on Linux
  SED=$(command -v sed)
else
  echo "Please install GNU's sed ('brew install gnu-sed' on MacOS). No action has been taken."
  exit 3
fi

SQLTK_PARSER_RELATIVE_PATH="packages/sqltk-parser"
SQLTK_PARSER_PATH="${PKG_ROOT}/${SQLTK_PARSER_RELATIVE_PATH}"
UPSTREAM_REPO_URL="https://github.com/apache/datafusion-sqlparser-rs"

### Helpers

info() {
  >&2 echo -e "\e[1;34m***\e[0m $*"
}
infops() { 
  # as in, Info PS/postscript. Indents output to show under info() and error() calls
  >&2 echo -e "    $*"
}
error() {
  >&2 echo -e "\e[1;31m!!!\e[0m $*"
}
prompt() {
  # $1 is the text prompt
  # $2 is the default value
  >&2 echo -ne "[\e[1;32m?\e[0m] $1 "
  read -r PROMPT
  if [ -z "$PROMPT" ]; then
    PROMPT=${2:-}
  fi
  [ "$(echo "$PROMPT" | grep -ic "y")" -gt 0 ] # returns 0/true or 1/false to caller
}

### Script

cd "$PKG_ROOT"

#
# 0. New branch?
#
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" == "main" ]; then
  if prompt "Current branch seems to be $CURRENT_BRANCH; I recommend creating a separate branch, then re-running this command. Continue anyway?" "n"; then
    info "Continuing..."
  else
    info "Cancelled."
    exit
  fi
fi

#
# 1. Get the latest from upstream
#
if ! git subtree pull --prefix "$SQLTK_PARSER_RELATIVE_PATH" "$UPSTREAM_REPO_URL" "$REV"; then
  if [ "$(git ls-files --unmerged | wc -l)" -gt 0 ]; then
    info "Conflicts detected; auto-accepting upstream changes..."
    infops "NOTE! This means that sqltk-parser changes that conflict with the sqlparser"
    infops "upstream are automatically being cleared; this will need manual investigation."
  else
    error "git subtree pull failed for unknown reason! exiting."
  fi
fi

#
# 2. For all conflicting Rust source files (*.rs) resolve with the upstream version.
#
# TODO: This bit is when we'd find out if our added features (eg. LOCK TABLE)
#       are conflicting with upstream code. It's difficult to make upgrades easy
#       **and** provide an easy facility for working through these more
#       selective conflicts. Suggestions very welcome.
while IFS='' read -r UNMERGED_RS_FILE; do
  (set -x; git checkout --theirs -- "$UNMERGED_RS_FILE" && git add "$UNMERGED_RS_FILE")
done < <(git status --porcelain=v1 | grep '^UU .*\.rs$' | "$SED" -e 's/^UU //' )
echo

#
# 3. for Cargo.toml & Cargo.lock & README differences - fix manually for now, in the future - automate
#
# (Do this now to reduce the clutter the user has to deal with, and to allow a
#  clean-up with `git merge --abort` *before* it gets difficult to do so.)
prompt_git_merge_abort() {
  trap - SIGINT
  echo
  prompt "Ctrl-C detected; run 'git merge --abort' to clean up? [Yn]" "y" && (
    (set -x; git merge --abort)
    info "Done."
  )
  exit
}
while true; do
  UNMERGED_OTHER_FILES=$(git status --porcelain=v1 | grep '^UU ' | "$SED" -e 's/^UU //' )
  if [ -n "$UNMERGED_OTHER_FILES" ]; then
    info "There are non-.rs conflicting files that need manual resolution:"
    while IFS="" read -r UNMERGED_OTHER_FILE; do
      infops "  $UNMERGED_OTHER_FILE"
    done < <(echo -e "$UNMERGED_OTHER_FILES")

    trap prompt_git_merge_abort SIGINT
    info   "This script will pause. After resolving these in a separate terminal"
    infops "(by editing them, and then 'git add'-ing them), please hit enter to "
    infops "continue the rest of the script."
    read -r PROMPT
  else
    break
  fi
done

#
# 4. delete sqlparser's .github directory (for build hygiene reasons)
#
SQLPARSER_GITHUB_DIR="${SQLTK_PARSER_PATH}/.github"
info "Removing the .github folder from the sqlparser / sqltk-parser folder, for hygiene. This is recommended."
prompt "Remove $SQLPARSER_GITHUB_DIR ? [yN]" "n" && (
  git rm -rf "${SQLPARSER_GITHUB_DIR}"
)

# 5. for every *.rs file in sqltk-parser (both conflicting and non-conflicting):
#   a. replace `sqlparser` with `sqltk_parser`
#   b. replace `sqlparser_derive` with `sqltk_parser_derive`
#   c. ... something something github.com links, literal "`sqlparser'" mentions in docs.
#
info "Replacing sqlparser with sqltk_parser (et al) for..."
while IFS='' read -r RS_FILE; do
  infops "  ${RS_FILE}"
  "$SED" -i'' \
    -e 's/`sqlparser_derive'\''/sqltk-parser-derive/g' \
    -e 's/sqlparser_derive/sqltk_parser_derive/g' \
    -e 's/`sqlparser'\''/sqltk-parser/g' \
    -e 's/sqlparser-rs/SQLPARSER-RS/g' \
    -e 's/sqlparser/sqltk_parser/g' \
    -e 's/SQLPARSER-RS/sqlparser-rs/g' \
    "$RS_FILE"
  git add "${RS_FILE}"
done < <(find "$SQLTK_PARSER_PATH" -name "*.rs")

# 6. Pre-flight testing
#   a. Regenerate the generated files
info "Regenerating and 'git add'-ing the 'sqltk-parser'-derived impls..."
cargo run -p sqltk-codegen
(set -x; git add packages/sqltk/src/generated/*_impls.rs)

info "'git add'-ing Cargo.lock"
(set -x; git add Cargo.lock)

#   b. `cargo test`, to make sure nothing we have done has been clobbered.
info "Running cargo test..."
cargo test

# 7. PROFIT! - um - push a new PR to `sqltk` after testing a local build with Proxy.
info "Everything seems to be in order. After reviewing the 'git diff', please run:"
infops "  git merge --continue"
