{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-20.03";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      defaultPackage.${system} = pkgs.mkShell {
        name = "nixshell";
        buildInputs = with pkgs; [
          # cargo build will through minifb -> x11_dl -> pkg-config run:
          #     pkg-config --variable=libdir x11
          #     pkg-config --variable=libdir xcursor
          # This requires the following packages
          xorg.libX11
          xorg.libXcursor
          pkg-config
        ];
      };
    };
}
