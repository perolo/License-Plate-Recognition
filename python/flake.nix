# 
# thanks to:
# https://github.com/kemalmao19/devShell/tree/main
# for inspiration
#

{
  description = "This is my python dev: Data Science Libraries Edition";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, }:
    let
      # Systems supported
      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forAllSystems = f:
        nixpkgs.lib.genAttrs allSystems
        (system: f { pkgs = import nixpkgs { inherit system; }; });
    in {
      # Development environment output
      devShells = forAllSystems ({ pkgs }: {
        default = let
          # 
        in pkgs.mkShell {
          buildInputs = (with pkgs; [
            uv
            rustup
            ruff
            python311Packages.pip
            python311Packages.numpy
            python311Packages.jupyter
            python311Packages.notebook
            python311Packages.toml
            python311Packages.easyocr
            python311Packages.imutils
            python311Packages.opencv-python
            python311Packages.matplotlib
          ]);
          shellHook = ''
            export PS1='\n\[\033[1;34m\](Python + Rust):\w]\$ \[\033[0m\]'
            echo "Welcome to the $name development shell!"
            echo "All necessary libraries and tools are installed."
          '';          
        };
      });
    };
}