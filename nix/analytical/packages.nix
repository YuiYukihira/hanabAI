{ inputs
, cell
}:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;

  args = import ./args.nix { inherit inputs cell; };

  cargoArtifacts = cell.packages.analytical-deps;
in
with args; rec {
  analytical-deps = crane.buildDepsOnly commonArgs;

  default = analytical-bin;

  analytical-bin = crane.buildPackage (commonArgs // {
    cargoExtraArgs = "--bin analytical-bin --features='tungstenite,color-eyre,pretty_env_logger,serde_json,serde,reqwest'";

    cargoArtifacts = analytical-deps;
  });

  analytical-lib = crane.buildPackage (commonArgs // {
    cargoExtraArgs = "--lib";

    cargoArtifacts = analytical-deps;
  });
}
