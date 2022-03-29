# Using anim_stitcher

`anim_stitcher` is a Python utility designed to allow artists to automatically convert their frames into sprite sheets with metadata.

## Usage

## Technical information

`anim_stitcher` exports spritesheets to `game/dist/assets/anm/...`. Each spritesheet also has a metadata JSON file beside it. The filepaths are automatically chosen based on input in the GUI.

An example output would be for an asset named `testFox` with the `Character` type.

```text
...
game/dist/assets/anm/chr/chr_testFox:
     - chr_testFox.png
     - chr_testFox.anim_meta.json
...
```
