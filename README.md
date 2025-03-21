# Gizmox

**This is a personal project I'm building as I go. It's very much a WIP, so use at your own risk.**

Gizmox is a tiny cross-platform CLI with small utilities I find useful day to day. I’m adding commands as I need them, so expect things to evolve (or break).

---

## Commands

### `copy-rename`

Copies a single input file and lets you generate as many renamed copies as you want.

#### Parameters
- ```<input-file>``` The path to the file you want to copy. Example: hello_world.png
- ```--ext <full-extension>``` **(optional)** If your file has a compound extension or includes a '.' in its name, this lets you manually define what should be considered the extension. If not provided, everything after the last '.' will be considered the extension. Example: png.meta

#### Usage

```bash
gm copy-rename <input-file> [--ext <full-extension>]
