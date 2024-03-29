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
      commit-msg = {
        commands = {
          conform = {
            run = "${nixpkgs.conform}/bin/conform enforce --commit-msg-file {1}";
          };
        };
      };
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

  conform = {
    data = {
      commit = {
        header = {
          length = 89;
          imperative = true;
          case = "upper";
          invalidLastCharacters = ".,!?";
        };
        body = {
          required = true;
        };
        dco = true;
        spellcheck = {
          locale = "US";
        };
        conventional = {
          types = [
            "build"
            "chore"
            "ci"
            "docs"
            "feat"
            "fix"
            "perf"
            "refactor"
            "style"
            "test"
            "wip"
          ];
          descriptionLength = 72;
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

  fly = {
    data = {
      app = "hanabAI";
      primary_region = "lhr";

      build = {
        image = "lucyekatarina/analytical-bin:${inputs.cells.analytical.args.crateName.version}";
      };
    };
    output = "fly.toml";
  };
}
