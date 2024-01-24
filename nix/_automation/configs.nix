{ inputs
, cell
}:
let
  inherit (inputs) nixpkgs std;
  l = nixpkgs.lib // builtins;
in
{
  lefthook = {
    data = {
      pre-commit = {
        commands = {
          treefmt = {
            run = "${nixpkgs.treefmt}/bin/treefmt {staged_files}";
          };
          clippy = {
            run = "cd analytical && ${inputs.cells.rust.toolchain.rust}/bin/cargo clippy";
          };
        };
      };
    };
  };

  prettier = {
    data = {
      printWidth = 80;
      proseWrap = "always";
    };
    output = ".prettierrc";
    format = "json";
  };

  treefmt = {
    data = {
      formatter = {
        nix = {
          command = "nixpkgs-fmt";
          includes = [ "*.nix" ];
        };
        prettier = {
          command = "prettier";
          options = [ "--write" ];
          includes = [ "*.md" ];
        };
        rustfmt = {
          command = "rustfmt";
          options = [ "--edition" "2021" ];
          includes = [ "*.rs" ];
        };
      };
    };

    packages = [
      nixpkgs.nixpkgs-fmt
      nixpkgs.nodePackages.prettier
    ];
  };
}
