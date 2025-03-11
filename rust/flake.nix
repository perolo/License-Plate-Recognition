{
  description = "Rust development environment with OpenCV, Clang, Tesseract, and other dependencies";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          clang_14
          llvmPackages_14.libclang
          #clang
          #libclang
          leptonica
          tesseract
          #tesseract.eng
          #tesseract.dev
          autoconf
          automake
          libtool
          pkg-config
          libpng
          libjpeg
          libtiff
          zlib
          libwebp
          openjpeg
          giflib
          libarchive
          curl
          opencv
        ];

        # Set the LIBCLANG_PATH environment variable
        LIBCLANG_PATH = "${pkgs.llvmPackages_14.libclang.lib}/lib";

        # Optional: Set other environment variables if needed
        
        shellHook = ''
          echo "Rust development environment ready!";
          export PS1='\n\[\033[1;34m\](Rust):\w]\$\[\033[0m\]';
        '';
      };
    });
}
