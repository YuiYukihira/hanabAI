{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    std = {
      url = "github:divnix/std";
      inputs.devshell.url = "github:numtide/devshell";
      inputs.nixago.url = "github:nix-community/nixago";
    };
    semver = {
      url = "sourcehut:~yuiyukihira/semver";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    helpers.url = "sourcehut:~yuiyukihira/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { std, ... }@inputs:
    std.growOn
      {
        inherit inputs;
        cellsFrom = ./nix;
        cellBlocks = [
          (std.blockTypes.runnables "apps")
          (std.blockTypes.installables "packages")
          (std.blockTypes.devshells "devshells")
          (std.blockTypes.nixago "configs")
          (std.blockTypes.functions "toolchain")
          (std.blockTypes.functions "args")
        ];
      }
      {
        packages = std.harvest inputs.self [ [ "analytical" "packages" ] ];
        apps = std.harvest inputs.self [ [ "analytical" "apps" ] ];
        devShells = std.harvest inputs.self [ [ "_automation" "devshells" ] ];
        checks = std.harvest inputs.self [ [ "analytical" "checks" ] ];
      };

  nixConfig = {
    extra-substituters = [ "https://yuiyukihira.cachix.org" ];
    extra-trusted-public-keys = [ "yuiyukihira.cachix.org-1:TuN52rUDSZIRJLC1zbD7a53Z/sv4pZIDt/b55LuzEJ4=" ];
  };
}
