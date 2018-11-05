with import <nixpkgs> {};

let
  python-tensorflow = with super; python3Packages.tensorflow.overrideAttrs (oldAttrs: rec {
    pname = "tensorflow";
    name = "${pname}-${version}";
    version = "1.9.0";

    src = fetchurl {
      url = "https://storage.googleapis.com/tensorflow/linux/cpu/tensorflow-1.9.0-cp36-cp36m-linux_x86_64.whl";
      sha256 = "0x2l64ab7i8nr0dzvsryblhn869qyjb85xkhy69nwahqswb68hxl";
    };
  });
in stdenv.mkDerivation rec {
  name = "dpar-writegraph-env";
  env = buildEnv { name = name; paths = buildInputs; };

  propagatedBuildInputs = [
    python-tensorflow
    python3Packages.toml
  ];

  nativeBuildInputs = [
  ];

  buildInputs = [
    curl
    libtensorflow
    openssl
  ] ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;
}
