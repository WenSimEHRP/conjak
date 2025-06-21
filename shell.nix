with import <nixpkgs> { };
let
  fonts = makeFontsConf {
    fontDirectories = [
      sarasa-gothic
      ubuntu-sans
    ];
  };
in
mkShell {
  nativeBuildInputs = [
    rustup
    coreutils
    typst
    poop
    sarasa-gothic
    ubuntu-sans
  ];
  shellHook = ''
    export FONTCONFIG_FILE=${fonts}
  '';
}
