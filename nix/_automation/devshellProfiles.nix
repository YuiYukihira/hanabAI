{ inputs, cell }: {
  analytical = { config, lib, pkgs, ... }:
    with lib;
    let
      cfg = config.services.analytical;
      start-command = pkgs.writeShellScriptBin "start-analytical" ''
        export HANABI_HOST=${
          if (cfg.host == "nix-service") then "localhost" else cfg.host
        }
        export HANABI_PORT=${cfg.port}
        export HANABI_USERNAME=${cfg.username}
        export HANABI_PASSWORD=${cfg.password}
        export USE_TLS="${if cfg.tls then "true" else "false"}"
        export RUST_LOG="${cfg.logging}"
        ${cfg.package}/bin/analytical-bin
      '';
    in
    {
      imports = [ inputs.hanabi-live.devshellProfiles.hanabi-live ];

      options.services.analytical = {
        enable = mkEnableOption "Enable the service";
        package = mkOption {
          type = types.package;
          default = inputs.cells.analytical.packages.analytical-bin;
          description = "Package to use";
        };
        username = mkOption {
          type = types.str;
          default = "analytical-bot";
          description = "User to log in as";
        };
        password = mkOption {
          type = types.str;
          description = "User password";
        };
        tls = mkEnableOption "Should connect with TLS";
        host = mkOption {
          type = types.str;
          default = "nix-service";
          description =
            "Host of the hanabi-live server. (Set to 'nix-service' to start server when this starts)";
        };
        port = mkOption {
          type = types.str;
          description = "Host port";
        };
        logging = mkOption {
          type = types.str;
          default = "error";
          description = "Log level";
        };
      };

      config = {
        __services.analytical = {
          command = "${start-command}/bin/start-analytical";
          enable = cfg.enable;
          depends = mkIf (cfg.host == "nix-service") [ "hanabi-live" ];
        };
      };
    };
}
