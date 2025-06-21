with import <nixpkgs> { };
mkShell {
  nativeBuildInputs = [
    rustup
    coreutils
    typst
    poop
    git
    sarasa-gothic
  ];
}
