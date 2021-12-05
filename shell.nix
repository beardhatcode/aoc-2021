{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    entr
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    (pkgs.writeShellScriptBin
      "compile-run"
      ''
        set -e
        echo -e "\n\n\n\n\n\n\n\n"
        export   RUST_BACKTRACE=full;
        cargo build
        ./target/debug/day$1-$2 $3
      '')
    (pkgs.writeShellScriptBin
      "compile-run-watch"
      ''
        echo "src/bin/day$1-$2.rs" | entr  compile-run $1 $2 $3
      '')
  ];

  RUST_BACKTRACE = 1;
}
