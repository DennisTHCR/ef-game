# `structs` Module
The `structs` module contains all struct definitions.
This helps keeping import statements cleaner, since
all required struct imports can be moved into one
large import statement.
On top of that, it cleans up the modules containing logic,
since they no longer need to worry about the struct definition.
Having all structs in one location makes the whole project easier to manage.
# `camera` Module
The `camera` module initializes the camera and manages its movements.
# `input` Module
The `input` module reads input from the bevy engine and preprocesses it
for other parts of the game.
# `player` Module
The `player` module handles everything related to the player like
initializing it or moving it.
# `world` Module
The `world` module handles everything related to managing the game world.