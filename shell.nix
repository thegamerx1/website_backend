with import <nixpkgs> { };

stdenv.mkDerivation {

  name = "rusty";
  buildInputs = [ clang gcc openssl pkgconfig dbus.dev ];

  # Point bindgen to where the clang library would be
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  # Make clang aware of a few headers (stdbool.h, wchar.h)
  BINDGEN_EXTRA_CLANG_ARGS = with pkgs; ''
    -isystem ${llvmPackages.libclang.lib}/lib/clang/${
      lib.getVersion clang
    }/include
    -isystem ${llvmPackages.libclang.out}/lib/clang/${
      lib.getVersion clang
    }/include
    -isystem ${glibc.dev}/include
  '';

  # For Rust language server and rust-analyzer
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
