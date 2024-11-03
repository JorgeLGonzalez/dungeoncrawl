# Dungeon Crawl

From [Hands-on Rust: Effective Learning through 2D Game Development and Play](https://pragprog.com/titles/hwrust/hands-on-rust/).

See https://opengameart.org/ for more game art.

## Issue: periphery walls

In the cell automata and drunkard walk builders, the floor often hits the screen boundary without walls. Is that expected? Regardless, it looks weird.
Somewhat related, monsters sometimes spawn in isolated tile floors.

## Issue (Resolved): Monsters able to move on top of each other

This seems to be a bug in the book's [chasing](./src/systems/chasing.rs) (though it is possible I somehow introduced it in my refactoring efforts). The original logic is for each monster to move closer to the player and we ignore any desired moves to a position already occupied by another monster. The bug is that the check is on the position of monsters **at the beginning** of the turn, so you can have more than one monster decide to move to the same (currently empty) position. There was no guard against that.

The fix is simple (see `will_be_occupied`). The 'chasing' system keeps track of the planned moves for each monster and ignores any new moves to a position that will be occupied based on the accumulated planned moves.

Note that a more elegant solution would probably be to check for future position occupation rather than current occupation instead. As things stand now, a monster is unable to move to a position currently occupied by another monster even though the second monster may end up anyway moving out of it in the turn.

## Issue (Resolved): items hide monsters

Annoying that monsters may disappear behind items. Think this is because monsters and items are all rendered in entity_render which simply loops through all entities (including the player). We could ensure items are rendered first, that should fix it.

Resolved by adding a render order field to the Render component and then using it to order in which the entities are drawn in entity_render.
