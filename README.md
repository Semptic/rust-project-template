[![GitHub](https://img.shields.io/github/license/Semptic/rust-project-template)](https://github.com/Semptic/rust-project-template/blob/main/LICENSE)
[![test](https://github.com/Semptic/rust-project-template/actions/workflows/test.yml/badge.svg)](https://github.com/Semptic/rust-project-template/actions/workflows/test.yml)

# Rust Project template

This is a template for [cargo-generate](https://github.com/cargo-generate/cargo-generate) to create CLI applications. To use it run:

```bash
cargo generate --git https://github.com/Semptic/rust-project-template.git
```

It also creates github workflows and dependabot config. If you don't want this delete the `.github/` folder.

This also adds a ready to go [gitpod](https://gitpod.io/) configuration, if you don't need this just delete the `.gitpod.yaml` and `.gitpod.Dockerfile` files.

You must add a license file in `./LICENSE` after generating from the template.


## Test the template

Run the following command to test the template:

```bash
./test-template.sh
```
