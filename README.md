# Changesets Versionfile Plugin

This is a plugin for my [changesets](https://github.com/alex-way/changesets-go) tool which allows for automated versioning of your project. It's still a WIP.

## Installation

Create a `.changeset/config.json` file in your project root with the following contents:

```json
{
  "plugin": {
    "name": "versionfile",
    "sha256": "917952c1bbe393fb6bb5038c827354c3c4a6dfe208459adfc673c07bca5d915c",
    "url": "https://github.com/alex-way/changesets-go-versionfile-plugin/releases/download/v0.0.1/versionfile-0.0.1-x86_64-unknown-linux-gnu.wasm",
    "versionedFile": ".changeset/version"
  }
}
```

Running `changeset get-version` will automatically download the plugin, cache it and run it to get the current version of your project!
