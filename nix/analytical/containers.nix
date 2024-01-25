{ inputs
, cell
}:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;
in
{
  analytical = std.lib.ops.mkOCI {
    name = "analytical";
    tag = cell.args.crateName.version;
    entrypoint = cell.packages.analytical-bin;
  };
}
