{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.93.0";

  wasm = "wasm32-unknown-unknown";
  wasi = "wasm32-wasip2";

  rustDefaultTarget = rustPkgs.rust-bin.stable.${rustVersion}.default;

  rustWithWasiTarget = rustPkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasi ];
  };

  rustWithWasmTarget = rustPkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasm ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustDefaultTarget;
    rustc = rustDefaultTarget;
  };

  rustPlatformWasi = makeRustPlatform {
    cargo = rustWithWasiTarget;
    rustc = rustWithWasiTarget;
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.1.0";
    src = self;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  workspace = rustPlatform.buildRustPackage (common // {
    cargoBuildFlags = "--release --workspace";
  });
}
