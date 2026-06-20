#!/usr/bin/env bash
# Shared PATH for Cougr Circom scripts (Bun-managed node_modules).

cougr_circuits_root() {
  local lib_dir
  lib_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  cd "$lib_dir/.." && pwd
}

cougr_circuits_export_path() {
  local root
  root="$(cougr_circuits_root)"
  export PATH="$root/node_modules/.bin:$PATH"
}