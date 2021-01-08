# TODO: investigate relative path handling
# `nix-shell` (or `flake-compat`) searches for a git directory relative to the
# cwd of the `nix-shell` command instead of relative to the `shell.nix` file
#
# ```
# `nix develop` doesn't have the same issue:
# > nix-shell dev/locations/server/shell.nix --run env
# opening directory '../.git/refs/heads': No such file or directory
# > nix develop dev/locations/server
# Works as expected
# ```
#
# https://github.com/edolstra/flake-compat/blob/master/default.nix
(
  import (fetchTarball https://github.com/edolstra/flake-compat/archive/master.tar.gz) {
    src = builtins.fetchGit ./.;
  }
).shellNix
