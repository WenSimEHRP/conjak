with import <nixpkgs> { };
let
  fonts = makeFontsConf {
    fontDirectories = [
      sarasa-gothic
    ];
  };
in
mkShell {
  nativeBuildInputs = [
    rustup
    coreutils
    typst
    sarasa-gothic
  ];
  shellHook = ''
    export FONTCONFIG_FILE=${fonts}
  '';
}
