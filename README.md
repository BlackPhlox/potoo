# Potoo

The Cursed Editor for [Bevy](https://bevyengine.org)

Plan is to migrate [bevy_cursed_editor](https://github.com/BlackPhlox/bevy_cursed_editor) and split `bevy_codegen` out to a seperate repository once `potoo` as at MVP.

Uses [hot-lib-reloader](https://github.com/rksm/hot-lib-reloader-rs) for hot-reloading allowing for faster iteration times of your bevy applications.

Project files is saved as `.po2`

Use `cargo run --features reload --target-dir "target-bin" `

