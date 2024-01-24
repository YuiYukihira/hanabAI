{ inputs
, cell
}:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;
in
{
  default = inputs.flake-utils.lib.mkApp {
    drv = cell.packages.default;
  };
  analytical = inputs.flake-utils.lib.mkApp {
    drv = cell.packages.analytical;
  };
}
