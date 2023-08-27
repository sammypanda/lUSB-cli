with (import <nixpkgs> {});

mkShell {
  buildInputs = [
    pkg-config
    libusb
  ];
}

