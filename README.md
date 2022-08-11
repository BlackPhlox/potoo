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

The Cursed Editor for [Bevy](https://bevyengine.org)

Plan is to migrate [bevy_cursed_editor](https://github.com/BlackPhlox/bevy_cursed_editor) and split `bevy_codegen` out to a seperate repository once `potoo` as at MVP.

- Uses [hot-lib-reloader](https://github.com/rksm/hot-lib-reloader-rs) for hot-reloading allowing for faster iteration times of your bevy applications.

- Some things like data structures cannot be changed during hot-reloading runtime, `potoo` knows this as it know about the code and reloads your entire application, to reduce friction.

- Project files will be saved as `.po2`, which will include version check for compatibility and auto-conversion.

# Usage

Notice: This will be handle automatically by the editor in the future.

## Linux and macOS
Use 
```
$ cargo watch -w systems -w components -x "build -p systems --features dynamic"
$ cargo run --features reload
```

## Windows
```
$ cargo watch -w systems -w components -x "build -p systems --features dynamic"
$ cargo run --features reload
```

