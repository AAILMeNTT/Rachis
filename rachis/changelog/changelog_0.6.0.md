---
title: changelog_0.6.0
type: note
permalink: rachis/changelog/changelog-0.6.0
---

refactor: it's here!!!!! it arrived!!!!!!!!

sqlite is dead long live native file system

## Global

### Version & Meta

- bump from 0.6.0_pre2 to 0.6.0
- upgrade to rust 2024 instead of 2021
- and also you cant have any of my hidden stuff

### Rust Dependencies

- just put carrots on versions. i mean karats. also some upgrades
- new dependencies, [regex](https://docs.rs/regex) and [walkdir](https://docs.rs/walkdir). directory walking pmo so i made it not pmo
- new dev dependency, [proptest](https://docs.rs/proptest). i dont really get it but i guess it works cool. gonna try using it more maybe

### Frontend Dependencies

- just put on karats on versions. i mean carets. also some upgrades

## Frontend

### Stores

- `landing.svelte.ts` now records current Flight
    - it also has a biiiiig function where it reconciles Flights (what does that mean ailment???) it means it verifies that all the `.flight`s are in the same place they were before and if they aren't then you get lasered
    - and `add()` takes a string id param now because so does the ipc command it calls

### Components

- "Open Flight" button in sidebar reconciles Flights before doing nothing
- "New Flight" button in sidebar now actually creates the `.flight` for the new Flight, quite handy
- added new `radio` Form Dialog type it's is really good

### Style

- tailwind-ified `FormDialog.svelte`

## Backend

### Rust

- tidily consolidated imports into a single `use {}` block
- i recently learned about `impl AsRef<T>` so i kindaaaaa spammed it everywhere 😁
- i also recently learned about `{:#?}` so that's like everywhere too

#### Module Structure

- new modules `entities`, `io`, `tree`:
    - `entities`: data structures wrapping `.flight` SQLite table rows (4 structs, serde-only, no ORM)
    - `io`: I/O layer: `ContentService` (file read/write) + `Finder` (dir scan/walker) + `FlightContext` (orchestrator)
    - `tree`: workspace tree types (Branch, Leaf, WidgetType, Direction from the widget system that i started a month ago)
- got rid of `storage` module cause it was kind of a lot. and also just. dead wrong
- THIS JUST IN: `domain` has shattered into `domain::flight` and `domain::rachis`
    - new type `Flight` for top-level flight container (conceptually the project data)
        - it has so many things just look at them honestly
        - also its exported to TS with ts_rs

#### Functionality

- in order to better match the mental model `db` has been replaced with `flight` in `lib.rs`
    - i'll elaborate more in the release tag. maybe
- all the shitty sqlite crud IPC commands were KILLED!!!!! and replaced with ✨beautiful✨ crud IPC commands:
    - `create_flight(flight_path, flight_name)` - creates a new Flight directory on disk, initialises `.flight` metadata, + stores the FlightContext
    - `get_flight(flight_path, flight_name, flight_id)` - opens a Flight connection by path, reads its metadata, and caches the FlightContext
    - `create_file(title, type, content)` - replaces `create_rachis`. delegates to `FlightContext::create_file`, which handles filename derivation, subdirectory mapping, file creation, metadata indexing, and identity theft
    - `save_file(id, content)` - replaces `update_rachis`. delegates to `FlightContext::save_file` for writing content to disk

- also updated some other IPC commands:
    - `add_registry_flight()` now takes explicit UUID forrrrr uhhhhh ... fun
    - `remove_registry_flight()` - returns an error if it fails
    - `toggle_registry_flight_favorite()` evolved into `update_registry_flight()` at level 60, meaning you can put whatever things you want to update and it'll onl- its just a rust implementation of ts Partial trust me
    - `get_most_recent_flight()` renamed to `get_recent_registry_flight()`
- and killed `get_registry_stats()` because. let's be real. 😶‍🌫️
- woops i also forgot that `reconcile_registry_flights()` exists
    - it walks all scan paths in the registry and reports whether anything funky happened (`.flight` moved/replaced/renamed/etc.)
- `registry.toggle_favorite()` → `registry.update()` (made generic instead of just for favourite toggling)
- other registry commands renamed too that's it

- renamed `ProjectFiles` to `ProjectFile` cause its only one
- renamed `Model` to `WorkspaceLayout` because frankly that's a moronic name

---

im gonna be so honest. i don't really remember everything i did. if you h8 me for it thats cool, just know that, by virtue of being me, i am cooler than you

- AAILMeNTT
