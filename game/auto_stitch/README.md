# The Auto-Stitch directory

This is a bit of a *magic* directory. Anything put in here will automatically be turned into a spritesheet at compile time.

## File organization

In this directory, framesets are expected to be stored in subdirectories. The names of these are important as they will translate into the names of the spritesheets.

For example, if you put a set of frames in `auto_stitch/chr_testFox`, you will get a spritesheet generated to `dist/gen/anm/chr/chr_testFox.png`.
