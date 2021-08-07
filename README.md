# Rust CLI template

This is a template for [cargo-generate](https://github.com/cargo-generate/cargo-generate) to create CLI applications. To use it run:

```bash
cargo generate --git semptic/rust-cli-template
```

It also creates github workflows for coverage and tests. If you don't want this delete the `.github/workflows` folder.

You must add a license file in `./LICENSE` after generating from the template.

Everything above the following line should be deleted once generated.
---

<!-- 
If you want to attach some badges, comment them out. You need to adapt the parts in [[ ]])

[![Crates.io](https://img.shields.io/crates/v/{{project-name}}?label={{project-name}})](https://crates.io/crates/{{project-name}})
[![Crates.io](https://img.shields.io/crates/v/{{project-name}}-lib?label={{project-name}}-lib)](https://crates.io/crates/{{project-name}}-lib)
[![GitHub](https://img.shields.io/github/license/[[user]]/{{project-name}})](https://github.com/[[user]]/{{project-name}}/blob/main/LICENSE)
![test](https://github.com/[[user]]/{{project-name}}/workflows/test/badge.svg)

-->

# {{project-name}}

{{description}}