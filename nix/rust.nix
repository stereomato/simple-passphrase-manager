{ sources ? import ./sources.nix }:

let
	pkgs =
		import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
	channel = "stable";
	date = "2023-02-09";
	targets = [ ];
	chan = pkgs.rustChannelOfTargets channel date targets;
in chan
