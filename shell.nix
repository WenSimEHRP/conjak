with import <nixpkgs> { };
mkShell {
  nativeBuildInputs = [
    rustup
    coreutils
    typst
    poop
    git
    fontconfig
    sarasa-gothic
  ];
}
