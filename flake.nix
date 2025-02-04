# 1BRC - my take on the 1 Billion Row Challenge
# Copyright (C) 2024  Charles German <5donuts@pm.me>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
{
  description = "My take on the 1 Billion Row Challenge";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Nix library for building Cargo projects
    # See: https://github.com/ipetkov/crane
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, crane, ... }:
  let
    # For details on this approach to supporting multiple architectures, see:
    # https://xeiaso.net/blog/nix-flakes-1-2022-02-21/
    systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
    forAllSystems = nixpkgs.lib.genAttrs systems;
    nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; } );

    rustOverrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
  in {
    # Create a dev-shell with Rust utils installed
    # For details, see: https://nixos.wiki/wiki/Rust#Installation_via_rustup
    devShells = forAllSystems (system:
      let
        pkgs = nixpkgsFor.${system};
      in {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            clang
            llvmPackages_latest.bintools
            rustup
          ];

          RUSTC_VERSION = rustOverrides.toolchain.channel;
        };
      });

    packages = forAllSystems(system:
      let
        pkgs = nixpkgsFor.${system};
      in {
        # Make the binary available with `nix run . -- <args>`
        default = (crane.mkLib pkgs).buildPackage {
          src = ./.;
        };

        # Run the create_measurements.py script with `nix run .#create-measurements -- <args>`
        create-measurements = pkgs.stdenvNoCC.mkDerivation {
          name = "create-measurements";

          # See: https://serverfault.com/a/1049157
          src = pkgs.fetchFromGitHub {
            owner = "gunnarmorling";
            repo = "onebrc";
            rev = "db064194be375edc02d6dbcd21268ad40f7e2869";
            sha256 = "0s6izfdc39jgjjynqsihaj0h3dgrdcnls9l8x1086685w3l8dxsa";
          };

          nativeBuildInputs = with pkgs; [
            gnused
          ];

          propagatedBuildInputs = with pkgs; [
            python3
          ];

          # TODO: this is a bit hacky, I bet I could apply a patch here instead
          buildPhase = ''
            # Make the output directories
            mkdir -p $out/{bin,data}

            # Update the path to weather_stations.csv to the location in the Nix store we copy that file
            sed -i "s#../../../data/weather_stations.csv#$out/data/weather_stations.csv#g" src/main/python/create_measurements.py

            # Update the path to measurements.txt to be placed in the CWD instead of a hardcoded path
            sed -i 's#"../../../data/measurements.txt"#"{}/measurements.txt".format(os.getcwd())#g' src/main/python/create_measurements.py

            # Finally, update the output about "test data written to ..." to be the new location
            sed -i 's#1brc/data/measurements.txt"#{}/measurements.txt".format(os.getcwd())#g' src/main/python/create_measurements.py
          '';

          installPhase = ''
            cp data/weather_stations.csv $out/data/
            cp src/main/python/create_measurements.py $out/bin/create-measurements
          '';
        };
      });
  };
}
