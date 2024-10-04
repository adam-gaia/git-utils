{
  projectRootFile = "flake.nix";
  programs = {
    alejandra.enable = true; # nix
    rustfmt.enable = true; # rust format
    clippy.enable = true; # rust lint
    just.enable = true; # justfile
    mdformat.enable = true; # markdown
    jsonfmt.enable = true; # json
    yamlfmt.enable = true; # yaml
    taplo.enable = true; # toml
    typos.enable = true; # spellcheck
  };
}
