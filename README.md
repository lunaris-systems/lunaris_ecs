# lunaris_ecs

Minimal ECS abstraction layer for the Lunaris video editor.

## Overview

`lunaris_ecs` provides a thin wrapper around [bevy_ecs](https://github.com/bevyengine/bevy), re-exporting only the essential types needed for plugin development. This abstraction allows Lunaris to potentially swap ECS backends in the future without breaking plugin compatibility.

## Current Backend

Currently backed by `bevy_ecs` 0.15.0 with `multi_threaded` and `trace` features enabled.

## Exported Types

- **Core**: `Entity`, `World`
- **Component System**: `Component`, `Resource`
- **Systems**: `Commands`, `Query`, `Res`, `ResMut`, `System`, `BoxedSystem`
- **Events**: `Event`
- **Query Filters**: `With`, `Without`
- **Scheduling**: `Schedule`

## Usage

```rust
use lunaris_ecs::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn my_system(query: Query<&Position>) {
    for pos in query.iter() {
        println!("Position: ({}, {})", pos.x, pos.y);
    }
}
```

## Design Philosophy

This crate intentionally exposes a **minimal API surface** to:

1. Enable future backend swaps without breaking plugins
2. Reduce the learning curve for plugin developers
3. Maintain forward compatibility as Lunaris evolves

## Part of Lunaris

This crate is part of the [Lunaris](https://github.com/lunaris-systems/lunaris) video editor ecosystem - a microkernel-based multimedia platform designed for extensibility via Rust plugins.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
