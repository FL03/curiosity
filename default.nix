{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.66.0";
  wasmTarget = "wasm32-wasi";


  rustWithWasmTarget = rustPkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasmTarget ];
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.1.0";
    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  wasm = rustPlatformWasm.buildRustPackage (common // {
    pname = "curiosity";

    buildPhase = ''
      cargo xtask build --release
    '';  
    installPhase = ''
      mkdir -p $out/lib
      cp target/wasm32-wasi/release/*.wasm $out/lib/
    '';  
  });
}