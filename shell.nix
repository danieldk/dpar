with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "dpar-env";
  env = buildEnv { name = name; paths = buildInputs; };

  nativeBuildInputs = [
    pkgconfig
    latest.rustChannels.stable.rust
  ];

  buildInputs = [
    curl
    libtensorflow
    openssl
  ] ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;
}
