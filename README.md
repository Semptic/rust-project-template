# Rust CLI template

This is a template for [cargo-generate](https://github.com/cargo-generate/cargo-generate) to create CLI applications. To use it run:

```bash
cargo generate --git semptic/rust-cli-template
```

It also creates github workflows for coverage and tests. If you don't want this delete the `workflows` folder otherwise move the `workflows` folder into `.github/`.

You must add a license file in `./LICENSE` after generating from the template.