{ inputs
, cell
}:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;

  craneLib = inputs.crane.lib;
in
rec {
  crane = craneLib.overrideToolchain inputs.cells.rust.toolchain.rust;

  crateName = crane.crateNameFromCargoToml { cargoToml = "${src}/analytical/Cargo.toml"; };

  commonArgs = {
    inherit src;
    inherit (crateName) pname version;
    cargoToml = "${src}/analytical/Cargo.toml";
    cargoLock = "${src}/analytical/Cargo.lock";
    postUnpack = ''
      cd $sourceRoot/analytical
      sourceRoot="."
    '';
  };

  src = std.incl (inputs.self) [
    (inputs.self + /analytical)
  ];
}
