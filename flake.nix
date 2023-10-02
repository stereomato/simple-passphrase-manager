{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs";
		flake-utils.url = "github:numtide/flake-utils";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};

	};
	outputs = { self, fenix, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system: 
			let
				pkgs = nixpkgs.legacyPackages.${system};
				toolchain = fenix.packages.${system}.default.toolchain;
			in {
				devShell = pkgs.mkShell {
					buildInputs = with pkgs; [
						toolchain
						rust-analyzer
					];
				};
				#packages.default = pkgs.stdenv.mkDerivation {
				#	name = "glTest";
				#	src = ./test.cpp;
				#	buildInputs = with pkgs; [
				#		(rust.override (old: { extensions = ["rust-src" "rust-analysis"];}))
				#	];
				#	dontUnpack = true;
				#	buildPhase = ''
				#	'';
				#};


			}
		);
}