# Potoo 👻🐦

<div align="center">
<a href="https://crates.io/crates/potoo"><img src="https://img.shields.io/crates/v/potoo" alt="link to crates.io"></a>
<a href="https://docs.rs/potoo"><img src="https://docs.rs/potoo/badge.svg" alt="link to docs.rs"></a>
<a href="https://github.com/BlackPhlox/potoo/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/crates/l/potoo" alt="link to license"></a>
<a href="https://crates.io/crates/potoo"><img src="https://img.shields.io/crates/d/potoo" alt="downloads/link to crates.io"></a>   
<a href="https://github.com/BlackPhlox/potoo"><img src="https://img.shields.io/github/stars/BlackPhlox/potoo" alt="stars/github repo"></a>
<a href="https://github.com/BlackPhlox/potoo/actions/workflows/ci.yml"><img src="https://github.com/BlackPhlox/potoo/actions/workflows/ci.yml/badge.svg" alt="github actions"></a>
<a href="https://github.com/bevyengine/bevy/tree/latest"><img src="https://img.shields.io/badge/Bevy%20Tracking-Release-lightblue" alt="tracking bevy release branch"></a>
</div><br>

The Cursed Editor for [Bevy](https://bevyengine.org).

A code-first editor that allows you to export your plugin or game project to code.

Currently [bevy_cursed_editor](https://github.com/BlackPhlox/bevy_cursed_editor) is being migrated into `potoo` and focusing on getting the [bevy_editor_pls](https://github.com/jakobhellermann/bevy_editor_pls) dependency working for bevy `0.9` and hot-reloading with the editor.

 `bevy_codegen`, which is the export part, is then split out to a separate repository once `potoo` as at MVP.

- Uses [hot-lib-reloader](https://github.com/rksm/hot-lib-reloader-rs) for hot-reloading allowing for faster iteration times of your bevy applications.

# Future plan

- Import existing bevy code using `syn`

- Some things like data structures cannot be changed during hot-reloading runtime, `potoo` knows this and only reloads your entire application when required, to reduce friction for the user.

- Project files will be saved as `.po2`, which will include version check for compatibility and auto-conversion.

# Usage

Notice: This will be handle automatically by the editor in the future.

Run these commands in two seperate terminals to enable hot-reloading: 
## Linux and macOS

```
$ cargo watch -w systems -w components -x "build -p systems --features dynamic"
$ cargo run --features reload
```

## Windows
```
$ cargo watch -w systems -x "build -p systems --features dynamic" --ignore-nothing
$ cargo run --features reload --target-dir "target-bin"
```

# Licensing
The project is under dual license MIT and Apache-2.0

