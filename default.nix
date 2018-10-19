with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "dpar-env";
  env = buildEnv { name = name; paths = buildInputs; };

  nativeBuildInputs = [
    cargo
    pkgconfig
    ragelDev
  ];

  buildInputs = [
    hdf5
    libtensorflow
    openssl
  ];
}
