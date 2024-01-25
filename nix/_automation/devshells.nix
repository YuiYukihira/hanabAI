{ inputs, cell }:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;
in l.mapAttrs (_: std.lib.dev.mkShell) {
  default = { extraModulesDir, ... }: {
    name = "hanabAI devshell";

    imports = [
      std.std.devshellProfiles.default
      inputs.helpers.nixosModules.base
      inputs.helpers.nixosModules."language/rust"
    ];

    commands = [
      {
        package = inputs.semver.packages.semver;
        category = "releases";
        help = "A tool to make creating semantic version easier";
      }
      { package = nixpkgs.cachix; }
      {
        package = nixpkgs.writeShellScriptBin "deploy" ''
          std //analytical/containers/analytical-bin:load
          ${nixpkgs.docker}/bin/docker push lucyekatarina/analytical-bin:${inputs.cells.analytical.args.crateName.version}
          ${nixpkgs.flyctl}/bin/flyctl deploy
        '';
        category = "releases";
        help = "Build, Load & Deploy";
      }
    ];

    language.rust = { packageSet = inputs.cells.rust.toolchain.rust; };

    nixago = [
      ((std.lib.dev.mkNixago std.lib.cfg.lefthook) cell.configs.lefthook)
      (std.lib.dev.mkNixago cell.configs.prettier)
      ((std.lib.dev.mkNixago std.lib.cfg.treefmt) cell.configs.treefmt)
      ((std.lib.dev.mkNixago std.lib.cfg.conform) cell.configs.conform)
      (std.lib.dev.mkNixago cell.configs.fly)
    ];
  };
}
