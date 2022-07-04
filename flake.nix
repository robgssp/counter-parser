{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
        {
          packages = {
            counter-parser = naersk-lib.buildPackage ./.;
            default = self.packages.${system}.counter-parser;
          };

          apps = {
            default = utils.lib.mkApp {
              drv = self.defaultPackage."${system}";
            };
          };

          devShell = with pkgs; mkShell {
            buildInputs = [ cargo rustc rustfmt rls rustup pre-commit rustPackages.clippy ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        }) //
    {
      nixosModule = { config, lib, pkgs, ... }:
        with lib;
        let cfg = config.services.counter-parser;
            pkg = self.packages.x86_64-linux.counter-parser;
        in {
          options.services.counter-parser = {
            enable = mkEnableOption "enables the counter-parser service";
          };

          config = mkIf cfg.enable {
            systemd.services.counter-parser = {
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];
              description = "Counter Parser service";
              serviceConfig = {
                Type = "simple";
                Restart = "always";
                RestartSec = "1";
                ExecStart = "${pkg}/bin/web";
                TimeoutStopSec = "5s";
              };
            };
          };
        };
      nixosConfigurations.container = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          self.nixosModule
          ({ pkgs, ... }: {
            boot.isContainer = true;
            networking.firewall.allowedTCPPorts = [ 2369 ];
            services.counter-parser.enable = true;
            system.stateVersion = "22.05";
          })
        ];
      };
    };
}
