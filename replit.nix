{ pkgs }: {
	deps = [
        pkgs.rustup
        pkgs.rust-analyzer
        pkgs.cargo-edit
	];
}