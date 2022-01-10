{
  description = "swayhide - A window swallower for sway";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs;
  };

  outputs = inputs:
    with inputs;
    let
      systems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      config = system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        swayhide = pkgs.rustPlatform.buildRustPackage {
          name = "swayhide";
          version = "0.2.0";
          src = ./.;
          cargoSha256 = "sha256-jL+o/WkKIyJufyTVJDtyB/pCGqrk2dZ01QzBEbglzrY=";
        };
      in {
        defaultPackage.${system} = swayhide;

        devShell.${system} = pkgs.mkShell {
          buildInputs = with pkgs; [ rustc cargo rustfmt ];
        };
      };
    in builtins.foldl' (acc: system: acc // (config system)) { } systems;
}
