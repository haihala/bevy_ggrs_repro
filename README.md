# Bevy_ggrs parent-children handling bug reproduction

Minimal reproduction for https://github.com/gschup/bevy_ggrs/issues/118

There are three versions:

- `default` which is pretty much 1:1 from the Bevy "Update glTF Scene" -example
- `rollback`, which is the same with added `bevy_ggrs` stuff
- `rollback2`, which is `rollback`, but with the rollback components added to a new root entity

Run with `cargo run --bin <version>`and feel free to play with the code.

Especially see the comments in the `move_scene_entities` function at the end of
each file.

The problem is how the rollback version doesn't move.
