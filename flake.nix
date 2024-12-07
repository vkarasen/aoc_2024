{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};
      in rec {
        defaultPackage = naersk-lib.buildPackage ./.;
        setupDayPkg = pkgs.writeShellApplication rec {
          name = "setup_day";
          runtimeInputs = with pkgs; [aoc-cli git];
          text =
            #bash
            ''
              if [ -z ''${1+x} ]; then
                echo "Usage: ${name} <Day>" && exit 1
              fi
              day=$1
              if ((day >= 1 && day <= 24)); then
                echo "setting up day $day"
              else
                echo "day $day is not a valid aoc day, aborting" && exit 1
              fi

			  projdir=$(git rev-parse --show-toplevel)
			  daydir=$projdir/src/day$day
			  mkdir -p "$daydir"
			  dayfile=$daydir/mod.rs
			  if [ -f "$dayfile" ]; then
			  	echo "Day file $dayfile exists, not changing"
			  else
			  	echo "Creating file $dayfile"
			  	cp templates/mod.rs "$dayfile"
			  	git add "$dayfile"
			  fi
			  mkdir -p input
			  inputfile=$projdir/input/day''${day}.txt
              aoc download --year 2024 --day "$day" --input-only --input-file "$inputfile"
            '';
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [cargo rustc rustfmt pre-commit rustPackages.clippy aoc-cli setupDayPkg];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
