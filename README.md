## xrpg: Extensible RPG

The project that inspired this was `rpg-cli`, another game written in Rust that used your directory structure as the map for a dungeon by replacing the `cd` command. Cool stuff!

Unfortunately for me, the game was quite simple and would get old after a while. Another glaring problem was that it scaled the difficulty based off of how far away from `~` you were; if you're someone like me who keeps directories organized, that is absolutely horrendous.

So I decided to take the basic concept, evolve it by using a directed graph as a game map (dynamically rendered with `mermaid` in your browser) and make it super extensible with Lua mods and JSON data!

Therefore, `xrpg` is more of an engine than a game itself. Even the base game itself will be written in Lua and modifiable, just like any other mod meant for `xrpg`.

### Roadmap

- [ ] Implement basic UI with Ratatui.
    - [x] Write the UI scaffolding.
    - [ ] Refactor `main.rs` into multiple files. Tabs will have different files in a directory.
    - [ ] State enum (Menu, Main, Quit. Switch `should_quit` to this state).
    - [ ] Implement main menu.
- [ ] Implement the basic game loop.
    - [ ] Define a basic game state struct.
    - [ ] Define an entity struct (all characters, player, enemies, animals).
    * Player marker will be a UUID pointing to a HashMap<UUID, Entity>.
    * Will need to figure out how many of those entities to actually fully simulate (maybe keep scanning the graph from player's spot outwards and only simulate those). Non-entity related events will still be handled globally. Cooldowns will store a "game time timestamp" of when they expire, simple check.
    - [ ] Add an mpsc event queue for the game, include a catch-all variant for Lua registered events.
    - [ ] Add event handlers, the rust end will handle the most basic events before anything else, and then it will pass it onto the Lua registered event handlers if conditions are met.
    - [ ] Step function (each step is a minute of game time, 1440 steps in a day. Might benefit from a threadpool or rayon parallelization), will poll for events each step and consume.
    * Recurring events will be useful for potion effects and such, might need event variants.
    - [ ] Integrate game loop into UI.
- [ ] Implement Lua integration.
    - [ ] Add Lua support first and foremost: interface to rust-facing game functions, a way to register new events and event handlers, and a way to access the entire game state, the player, and any other entity. We should be able to associate custom data to graph nodes and edges through Lua. We should also be able to define custom data for ALL entities.
    * Unsure right now, but I think we should only use Lua without JSON. You can still initialize data like this.
    - [ ] Further down the line, we need to allow extending the UI through Lua. Maybe allow some basic widget interaction with Lua code? Predefined layouts from JSON? We'll see.
- [ ] Build script to bundle mermaid.js and use it offline while compiling. Another thread will be spun up to output the bundled mermaid.js and mermaid map graph to an html file. We could even write a Lua module to recompile the map on node removed, node added, edge changed and other such events.
