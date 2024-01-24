{ inputs
, cell
}:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;

  args = import ./args.nix { inherit inputs cell; };

  cargoArtifacts = cell.packages.analytical-deps;
in
with args; {
  inherit (cell.packages) analytical;

  analytical-clippy = crane.cargoClippy (commonArgs // {
    inherit cargoArtifacts;
    cargoClippyExtraArgs = "--all-targets -- --deny warnings";
  });

  analytical-docs = crane.cargoDoc (commonArgs // {
    inherit cargoArtifacts;
  });
}
