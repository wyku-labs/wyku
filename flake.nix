{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

    in {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          strictDeps = true;
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

          shellHook = ''
            messages=(
              "🦀 The Crab Has Been Summoned"
              "🦀 The Crab Has Been Summoned"
              "🦀 The Crab Has Been Summoned"
              "The Crab Is Loose. Good Luck"
              "🦀 The Crab Has Been Summoned"
              "🦀 The Crab Has Been Summoned"
              "🦀 The Crab Has Been Summoned"
            )
            echo "''${messages[$RANDOM % ''${#messages[@]}]}"
          '';
        };
      });
    };
}
