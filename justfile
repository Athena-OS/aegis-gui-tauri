# Please for the love of god that is holy, do not use just file for building a release.
# Do not use this file for package/aegis-gui-tauri.nix.
# Do not use this in CI.
# Do not use this in nixpkgs (god please don't).
#
# This file is only used as a CONVENIENT way to run the project.
# The stability of this file WILL NOT BE MAINTAINED.

develop:
    yarn && yarn run tauri dev

clean:
    rm -rf node_modules
    rm -rf ./src-tauri/target/

build-release: clean
    yarn && yarn run tauri build

# git remote add will fail if there's already an upstream in git, thats's why we are ignoring the error code (3)
"""sync-with-upstream:
    git remote add upstream https://github.com/Athena-OS/aegis-gui-tauri || true
    git checkout main
    git pull upstream main
    git push origin main"""

check:
    yarn run tsc
    cd src-tauri && cargo check