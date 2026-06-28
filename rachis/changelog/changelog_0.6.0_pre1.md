---
title: changelog_0.6.0_pre1
type: note
permalink: rachis/changes/changelog-0.6.0-1
---

feat(ui? i guess?): what's a word that rhymes with widget

## Global

- bump from 0.5.0 to 0.6.0_pre1

## Frontend

### Components

- please hold applause until the end
- introducing the _WIDGET SYSTEM_ (i'll write something up that further explains these but for now just a basic rundown)
- `BaseWidget` for wrapping all other Widgets with shared functionality
- `EditorWidget` (minimal) for editing a Rachis' content
- `NotesWidget` (prospective) for displaying the user's notes
- `PickerWidget` (prospective) for picking a Widget
    - this is only generated when a user splits their workspace in any way
- and speaking of workspaces, `Workspace` for rendering Branches and Leaves
- `BranchView` for rendering Branches (splits in the workspace)
- i really need to get better at committing smaller and faster so im pushing just this one but there will be many more widgets to come
- "what the hell are you on about, aailmentt?" i hear you ask. "shut up," i hear you scream.

### Style

- i know everything's really ugly rn but i can't be bothered to learn web design on top of learning rust, typescript, svelte, _and_ good UX/UI okay i'll get to it i promise
- also i changed the purple a little bit because for some reason it was turning yellow? or it _was_ yellow? or it... _wasn't_? i don't know bro i don't get it i don't even know how to explain it
- this is a reminder to me to make more themes at some point i can't stand looking at twilight-purple.css anymore 🦄🦄🦄🦄

## Backend

### Types

- `Tree` (renamed from `Workspace`) for representing the workspace the user interacts with
- `Branch` for representing a split in the Tree
- `Leaf` for representing a Widget
- `TreeNode` (renamed from `WorkspaceNode`) is the politically correct term for referring to both a Leaf and a Branch please be kind <3
- `WidgetType` is an enum that.. well
- `Direction` is just an enum its either horizontal or vertical because you can't split a Branch in any other direction i won't let you

### Rust

- and this is where it get REAL scary
- added `tree` module, which defines the like data structures for the types mentioned before
- added gigantic ahh `ops` module which defines the.. operations.. of the types
    - these are really weird so i also made a shitton of tests for them

- you may now applaud

---

- AAILMeNTT
