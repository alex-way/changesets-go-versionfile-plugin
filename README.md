# Changesets Versionfile Plugin

This is a plugin for my [changesets](https://github.com/alex-way/changesets-go) tool which allows for automated versioning of your project. It's still a WIP.

## Installation

Create a `.changeset/config.json` file in your project root with the following contents:

```json
{
  "plugin": {
    "name": "versionfile",
    "sha256": "67245629ec3b9447c4eeb6d7bbdb66ce36937e1c1cea6410d388f43f8bb68e17",
    "url": "https://github.com/alex-way/changesets-go-versionfile-plugin/releases/download/0.0.2/versionfile.wasm",
    "versionedFile": ".changeset/version"
  }
}
```

Running `changeset get-version` will automatically download the plugin, cache it and run it to get the current version of your project!
