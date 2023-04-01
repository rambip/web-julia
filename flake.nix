{
    description = "webapp to generate julia fractal";
    inputs.wasm-tooling.url = github:rambip/wasm-tooling;

    outputs = {self, nixpkgs, wasm-tooling}: {
        defaultPackage.x86_64-linux = 
            let pkgs = import nixpkgs {system = "x86_64-linux";};
                tooling = pkgs.callPackage wasm-tooling {};
            in
            tooling.rust.buildWithTrunk {
                src = ./.;
                extraFiles = ["index.html" "index.js" "style.css"];
            };
        devShell.x86_64-linux = 
            let pkgs = import nixpkgs {system = "x86_64-linux";};
                tooling = pkgs.callPackage wasm-tooling {};
            in
            tooling.rust.devShell {
                src = ./.;
            } ;
    };
}
