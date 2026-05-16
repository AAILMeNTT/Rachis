---
aliases: [Feature List]
date created: Friday, March 6th 2026, 6:51:07 pm
date modified: Wednesday, May 13th 2026, 3:14:34 pm
linter-yaml-title-alias: Feature List
permalink: rachisapp/feature-list
tags: []
title: feature_list
---

# Feature List

Below is a comprehensive list of features that will be available in Rachis.

## Table of Contents

- [Feature List](#feature-list)
    - [Table of Contents](#table-of-contents)
    - [Terms](#terms)
    - [Widgets](#widgets)
        - [Editor Widget](#editor-widget)
        - [Tags Widget](#tags-widget)
        - [Notes Widget](#notes-widget)
        - [Story Widget](#story-widget)
            - [Acts](#acts)
            - [Arcs](#arcs)
            - [Scenes](#scenes)
    - [Workspaces](#workspaces)
    - [Tags](#tags)
        - [Custom Tags](#custom-tags)
    - [Notes](#notes)

## Terms

- **Flight**: Your project!
- **Rachis**: A document written in Markdown, HTML, or another supported format.
- **Widget**: A window that displays specific information.

---

## Widgets

Widgets are toggleable and resizable windows that you can utilise to streamline your workflow.

> [!note]+ Widget Tips
>
> 1. Widgets are draggable and resizable! You can drag them around and resize them to your liking.
> 2. Hold `CTRL` and click on its header to close a Widget.
> 3. Hold `ALT` and click on its header to lock it. This will prevent it from being resized or dragged.

### Editor Widget

The Editor Widget is where you write your story. At the top of the Editor, you can choose and switch between text formats — Markdown, HTML, etc. — to write in a format you're most comfortable with.[^1]

While in the Editor, you can create [tags](<#tags>) to organise your story.

### Tags Widget

The Tags Widget will display all the [tags](<#tags>) you've used. By default, it contains four folders: Characters, Events, Locations, and Items. If you've made any [custom tags](<#custom-tags>), the folders for them will be displayed here as well.

### Notes Widget

The Notes Widget will display all the [notes](<#notes>) you've created.

### Story Widget

The Story Widget displays all the files and folders you've created in this Flight. This houses all the Rachises you've written, and can be used to view the structure of your Flight.

Rachises created here can be defined as a multitude of types:

#### Acts

Acts serve as the highest story level, encompassing the overarching events that drive the story forward. They are primarily story-oriented and set up and define the resolution of the events that happen in your world.

#### Arcs

Arcs are the descriptors of a character's journey through your world. They are character-oriented and focus on the characters' growth and development.

#### Scenes

Scenes are instances of actions that happen in your story. They are action-focused, highlighting character actions or interactions with the external world.

> You may have different definitions of "arc" and "act", or have heard different descriptions for them — that's okay! Within Rachis, "Acts" and "Arcs" are not strictly more encompassing than the other, as one character's arc can span multiple acts, or vice versa.

<!---->

### Timeline Widget

The Timeline Widget organises all the [tags](#tags) you've created into a timeline, based on their chronological metadata.[^2]

---

## Workspaces

Workspaces are simply the layout of your [Widgets](<#widgets>). You can save, load, and delete workspaces as you please for a more personalised experience.

---

## Tags

Tags are used to link selections of text to a specific entity in your world. By default, tags come in five types: Character, Event, Location, Item, and Note. You can also add custom tags in "Settings > Manage Tags" — [more on that later](<#custom-tags>).

All tags are written as `<prefix>![<preceding-modifiers>]<name>[<succeding-modifiers>]!`, where:

- `<prefix>` is the prefix of the tag
- `<name>` is the name of the Rachis to link
- `[<preceding-modifiers>]` are the modifiers that must go before the name of the Rachis. All preceding modifiers are optional.
- `[<succeding-modifiers>]` are the modifiers that must go after the name of the Rachis. All succeeding modifiers are optional.

> [!note]+ Tag Tips
>
> If you find that using `!` as the delimiter for tags is a bit strange, you can always change the delimiter in "Settings > Manage Tags". Changing this value will automatically update all existing tags to use the new delimiter.
>
> Due to the other characters used to define tags, the delimiter can't be a whitespace, `#`, or `|`.

Writing out a tag will automatically create a new Rachis in the corresponding folder, if it doesn't already exist. All Rachises created by (or otherwise linked to) any tag except for Note tags can be viewed in the [Tags Widget](<#tags-widget>). Rachises tagged as a Note will be displayed in the [Notes Widget](<#notes-widget>).

> Here are some examples of tag usage:
>
> - `c!Twilight Sparkle!`: Creates a Rachis named "Twilight Sparkle" in the "Characters" folder, which shows up in the Tags Widget.
> - `e!Twilight Sparkle's Coronation!`: Creates a Rachis named "Twilight Sparkle's Coronation" in the "Events" folder. This, too, appears in the Tags Widget.
> - `l!Golden Oaks Library!`: Creates a Rachis named "Golden Oaks Library" in the "Locations" folder.
> - `i!Royal Crown!`: Creates a Rachis named "Royal Crown" in the "Items" folder.
> - `n!Friendship is Magic!`: Creates a Rachis named "Friendship is Magic" in the "Notes" folder. Unlike the previous examples, this Rachis and its folder will instead be located in the Notes Widget.

### Modifiers

Modifiers are used to affect the creation and extension of a Rachis. Below is a complete list of modifiers:

|           Key            |    Type    | Max Allowed | Description                                                                                                                                                                                                                                                                                    |
| :----------------------: | :--------: | :---------: | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|           `\`            |    ---     |  1 or more  | <ul><li>Escapes the next modifier character<li>Can be used any number of times, anywhere in the tag</ul>                                                                                                                                                                                       |
|  `<folder_name: text>/`  | Preceeding |  1 or more  | <ul><li>Defines a folder for the Rachis to be created in</ul>                                                                                                                                                                                                                                  |
| `\|<display_name: text>` | Succeeding |      1      | <ul><li>Displays a different name for the tag in the document.<li>Must be used at the very end of the tag, but before the `.` or `:` modifiers</ul>                                                                                                                                            |
|    `~<index: number>`    | Succeeding |      1      | <ul><li>Represents the index of the tag<li>Used in the event that there is more than one tag of the same type with the same name (for example, two characters named "Twilight Sparkle").<li>Incompatible with the `.` and `:` modifiers</ul>                                                   |
|           `.`            |   Final    |      1      | <ul><li>Prevents multiple Rachises of the same type from sharing the same name as this one.<li>As such, this cannot be used if there is already a Rachis of the same type with the same name.<li>Incompatible with the `~` and `:` modifiers.<li>Must be used at the very end of the tag.</ul> |
|           `:`            |   Final    |      1      | <ul><li>Prevents any other Rachis from sharing the same name as this one.<li>As such, this cannot be used if there is already a Rachis with the same name.<li>Incompatible with the `~` and `.` modifiers.                                                                                     |

> [!note]+ Examples, Cont.
>
> Here are some examples of tag usage (with modifiers this time!):
>
> - `c!Mane Six/Twilight Sparkle:!`: Creates a Rachis named "Twilight Sparkle" in the "Characters/Mane Six" folder.
>     - Because of the `:` modifier, no other Rachis can be named "Twilight Sparkle".
>     - This will fail if there is already a Rachis named "Twilight Sparkle".
> - `e!Twilight Sparkle's Coronation.!`: Creates a Rachis named "Twilight Sparkle's Coronation" in the "Events" folder.
>     - Because of the `.` modifier, other Rachises are allowed to share the same name, but cannot also be Events — they must be some other type.
>     - This won't work if there's already an Event Rachis with this name.
> - `l!Ponyville/Golden Oaks Library|Twilight's house!`: Creates a Rachis named "Golden Oaks Library" in the "Locations/Ponyville" folder
>     - Because of the `|` modifier, this will display in the document as "Twilight's house".
> - `i!Royal Crown~4|Twilight Sparkle's Crown!`: Creates a Rachis named "Royal Crown~4" in the "Items" folder
>     - Because of the `|` modifier, this will display as "Twilight Sparkle's Crown". The other crowns might belong to Princess Celestia (`i!Royal Crown~1!`), Princess Luna (`i!Royal Crown~2!`), and Princess Cadence (`i!Royal Crown~3!`).
> - `n!Friendship is Magic\!\!\!!`: Creates a Rachis named "Friendship is Magic!!!" in the Notes Widget.
>     - Because of the `\`s, each exclamation point that has one before it is recognised as a regular exclamation point, and not the ending token of the tag.

### Custom Tags

If you find that the five default tags aren't enough, you can add custom tags in "Settings > Manage Tags". In this window, you can define a tag based on the prefix (the "`c`", "`e`", "`l`", "`i`", or "`n`"), the name of the tag, the placeholder text, and the corresponding linking folder.

Fields marked with an asterisk are required.

- **Tag Prefix**\*: The prefix of the tag
- **Tag Name**\*: The name of the tag
- **Placeholder Text**: The text that will be displayed in the document
    - Defaults to `{tag_name}`
- **Linking Folder**: The folder that the tag will link to in the [Tags Widget](<#tags-widget>) or the [Notes Widget](<#notes-widget>)
    - Defaults to `Tags/{tag_name}s`

For example, the default Character, Event, Location, Item, and Note tags are defined as follows:

| Tag Prefix | Tag Name  | Placeholder Text | Linking Folder  |
| :--------: | :-------: | :--------------: | :-------------: |
|    `c`     | Character |    Character     | Tags/Characters |
|    `e`     |   Event   |      Event       |   Tags/Events   |
|    `l`     | Location  |     Location     | Tags/Locations  |
|    `i`     |   Item    |       Item       |   Tags/Items    |
|    `n`     |   Note    |       Note       |      Notes      |

> Let's put this into practice: say you had a very developed magic system complete with dozens upon dozens of spells and powers. If you wanted to create a custom tag for these features, you might define the following tags:
>
> | Tag Prefix | Tag Name | Placeholder Text | Linking Folder |
> |:----------:|----------|------------------|----------------|
> |    `s`     | Spell    | Spell            | Tags/Spells    |
> |    `p`     | Power    | Power            | Tags/Powers    |
>
> Now, whenever you want to link to a spell or power, you can write `s!My Spell!` or `p!My Power!` and it will automatically create a new Rachis in the appropriate folder.

---

## Notes

Notes are a special kind of tag, and serve to record any important information for yourself while you develop your story. You can view all the notes you've created within your flight in the [Notes Widget](<#notes-widget>).

> If you have a lot of notes, don't worry! You can search, filter, and sort the Notes you see in the Widget.

Notes can be left across any span of text, can overlap one another, and can even include [tags](<#tags>).

[^1]: For now, only Markdown is supported, but HTML is soon in the works. In the future, you'll be able to write in BBCode, RichText, and even ASCII.
[^2]: MEGA TODO NOT EVEN THINKING ABOUT IT YET
