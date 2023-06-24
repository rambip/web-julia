{
    description = "webapp to generate julia fractal";
    inputs = {
        wasm-tooling.url = github:rambip/wasm-tooling;
        flake-utils.url = github:numtide/flake-utils;
    };

    outputs = {self, nixpkgs, flake-utils, wasm-tooling}: with flake-utils.lib;
        eachSystem [system.x86_64-linux system.x86_64-darwin] (system:
            let rust-tooling = wasm-tooling.lib."${system}".rust;
            in
            {
                packages.default =
                    rust-tooling.buildWithTrunk {
                        src = ./.;
                        extraFiles = ["index.html" "index.js" "style.css"];
                    };
                devShells.default = rust-tooling.makeDevShell {src=./.;};
            }
        );
}
