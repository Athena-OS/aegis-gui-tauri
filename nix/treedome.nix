{ lib
, cargo-tauri
, cmake
, dbus
, fetchYarnDeps
, freetype
, gsettings-desktop-schemas
, gtk3
, libsoup
, mkYarnPackage
, openssl
, pkg-config
, rustPlatform
, webkitgtk
, wrapGAppsHook
, sqlite
}:

let

  pname = "treedom";
  version = "0.4.3";

  src = ../.;

  frontend-build = mkYarnPackage {
    inherit version src;
    pname = "treedome-ui";

    offlineCache = fetchYarnDeps {
      yarnLock = src + "/yarn.lock";
      hash = "sha256-CrD/n8z5fJKkBKEcvpRHJaqXBt1gbON7VsuLb2JGu1A=";
    };

    packageJSON = ../package.json;

    configurePhase = ''
      runHook preConfigure
      ln -s $node_modules node_modules
      runHook postConfigure
    '';

    buildPhase = ''
      export HOME=$(mktemp -d)
      yarn --offline run build

      mkdir -p $out/dist
      cp -r dist/** $out/dist
    '';

    distPhase = "true";
    dontInstall = true;
  };
in

rustPlatform.buildRustPackage {
  inherit version pname;

  src = ../src-tauri;

  cargoLock = {
    lockFile = ../src-tauri/Cargo.lock;
    outputHashes = {
      "fix-path-env-0.0.0" = "sha256-ewE3CwqLC8dvi94UrQsWbp0mjmrzEJIGPDYtdmQ/sGs=";
    };
  };

  preConfigure = ''
    mkdir -p dist
  '';

  # copy the frontend static resources to final build directory
  # Also modify tauri.conf.json so that it expects the resources at the new location
  postPatch = ''
    mkdir -p dist
    cp -R ${frontend-build}/dist/** dist

    ls -al
    substituteInPlace ./tauri.conf.json --replace '"distDir": "../dist",' '"distDir": "dist",'
    substituteInPlace ./tauri.conf.json --replace '"beforeBuildCommand": "yarn run build",' '"beforeBuildCommand": "",'
  '';

  nativeBuildInputs = [ cmake pkg-config cargo-tauri wrapGAppsHook ];
  buildInputs = [ dbus openssl freetype libsoup gtk3 webkitgtk gsettings-desktop-schemas sqlite ];

  buildPhase = ''
    runHook preBuild

    export VERGEN_GIT_DESCRIBE=${version}
    cargo tauri build

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin/
    mkdir -p $out/share/

    cp target/release/bundle/deb/treedome_0.0.0_amd64/data/usr/bin/treedome $out/bin/treedome
    cp -R target/release/bundle/deb/treedome_0.0.0_amd64/data/usr/share/** $out/share/

    runHook postInstall
  '';

  postInstall = ''
    wrapProgram "$out/bin/treedome" \
      --set WEBKIT_DISABLE_COMPOSITING_MODE 1
  '';

  meta = with lib; {
    description = "A local-first, encrypted, note taking application with tree-like structures, all written and saved in markdown";
    homepage = "https://codeberg.org/solver-orgz/treedom";
    license = licenses.agpl3Plus;
    mainProgram = "treedome";
    platforms = [ "x86_64-linux" ];
    maintainers = [
      {
        name = "Tengku Izdihar";
        email = "tengkuizdihar@gmail.com";
        matrix = "@tengkuizdihar:matrix.org";
        github = "tengkuizdihar";
        githubId = 22078730;
      }
    ];
  };
}
