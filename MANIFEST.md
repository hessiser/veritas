# Overview
This file is a lightweight map of the current source tree and the major responsibilities of each module.

# Manifest
- `src`
    - `entry` - Process startup. Waits for the target game modules, installs subscribers, starts the server, and initializes the overlay.
    - `battle` - Battle-state aggregation and event dispatch. `BattleContext` owns the live battle summary and turns subscriber events into packets for clients.
    - `export` - Export and CSV generation for battle results and related summary data.
    - `kreide` - Game integration layer and IL2CPP helpers.
        - `helpers` - Helper functions used by subscribers and battle logic.
        - `types` - IL2CPP-facing type definitions and bindings used by the game integration layer.
    - `logging` - Logging setup and routing for console, file, and UI output.
    - `models` - Shared data types used by subscribers, the battle context, and the server payloads.
        - `events` - Internal event types. Includes battle flow events and damage events.
        - `types` - Core game data types such as `Avatar`, `Enemy`, `Entity`, `Property`, `BattleStats`, `Skill`, and `TurnInfo`.
        - `packets` - Serialized server packets broadcast to clients, including connection, error, and battle event payloads.
    - `overlay` - DirectX 11 overlay bootstrap and presentation.
    - `prelude` - Common imports, aliases, and re-exports shared across modules.
    - `server` - Socket server and broadcast layer. Serves the client connection and emits packets to connected clients.
    - `subscribers` - Game hooks that observe runtime activity and forward it into the battle layer.
        - `battle` - Hooks for battle flow, damage, stat changes, wave/cycle updates, and lineup initialization.
    - `ui` - Overlay UI built with [egui](https://github.com/emilk/egui).
        - `app` - UI application state and top-level app logic.
        - `config` - User-facing UI and runtime configuration.
        - `helpers` - UI helper utilities, including image and property-icon loading.
        - `themes` - Theme definitions for the UI.
        - `views` - View composition and screen-level layout.
        - `widgets` - Reusable UI widgets.
    - `updater` - Update checks and update-flow helpers.