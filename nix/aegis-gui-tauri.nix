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
, rustup
}:

let

  pname = "aegis-gui-tauri";
  version = "0.0.0";

  src = ../.;

  frontend-build = mkYarnPackage {
    inherit version src;
    pname = "aegis-gui-tauri-ui";

    offlineCache = fetchYarnDeps {
      yarnLock = src + "/yarn.lock";
      hash = "sha256-NLgGH0hJqg2FhEbDq8TKTuhSZSTDacoUYITddiPHSVM=";
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
      #"fix-path-env-0.0.0" = "sha256-NLgGH0hJqg2FhEbDq8TKTuhSZSTDacoUYITddiPHSVM=";
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
    #cargo update -p clap_derive@4.5.0 --precise clap_derive@4.0.0

    export VERGEN_GIT_DESCRIBE=${version}
    cargo tauri build -- --locked

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin/
    mkdir -p $out/share/

    cp target/release/bundle/deb/aegis-gui-tauri_0.0.0_amd64/data/usr/bin/aegis-gui-tauri $out/bin/aegis-gui-tauri
    cp -R target/release/bundle/deb/aegis-gui-tauri_0.0.0_amd64/data/usr/share/** $out/share/

    runHook postInstall
  '';

  postInstall = ''
    wrapProgram "$out/bin/aegis-gui-tauri" \
      --set WEBKIT_DISABLE_COMPOSITING_MODE 1
  '';

  meta = with lib; {
    description = "A local-first, encrypted, note taking application with tree-like structures, all written and saved in markdown";
    homepage = "https://github.com/Athena-OS/aegis-gui-tauri";
    license = licenses.agpl3Only;
    mainProgram = "aegis-gui-tauri";
    platforms = [ "x86_64-linux" ];
    maintainers = [
      {
        name = "Athena Team";
        email = "keeper@athenaos.org";
        matrix = "@AthenaOS:matrix.org";
        github = "Athena-OS";
        githubId = 22078730;
      }
    ];
  };
}
