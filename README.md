
# `.xp`
*A standard file format for resume / cv source data*
## why
I am tired of entering the same information, again and again, into a variety of sites and profiles and docs, all with the same goal: to apply for some position via resume / cv.

Instead, I want to have the latest version of that info in one file.

That's `.xp`. It's not really a new file format (just `json` under the nonexistant hood), but I hope it could lay the groundwork for a common format that can be imported to or exported from sites that have experience profiles, or used as the source to generate a resume/cv PDF (via [typst](https://typst.app/), for example).

Read more about my thought process here ///blog wip///

# `.xp` format
*TODO: describe/show format simply, tbd if gen from schema*
- see [examples/](./examples/)
- see [schema](./xp.schema.json)
# Using `.xp` files
## Reading & Writing
- any json viewer or text editor should work
- for proper syntax highlighting, either add `.json` to the end of your `.xp` file to have your editor treat it as a `.json` file, or see [Editor Support](#editor-support) for ways to use `.xp` directly
## Encoding & Decoding
If your project uses Rust, you can simply add this (todo insert cargo link) as a cargo dependency, and then decode the `Xp` type via serde. 
_TODO: Decoding guidance for other languages/platforms_
*TODO: cargo crate*
## Editor Support
`.xp` files conform to the [JSON Standard](https://www.json.org/json-en.html), so we just need to configure the editor to treat `.xp` like `.json` files to get all the `.json` features and syntax highlighting offered by that editor
### [vim](https://www.vim.org/)
in your `.vimrc`
```vim
autocmd BufRead,BufNewFile *.xp set filetype=json
```
### [nvim](https://neovim.io/)
in your `init.lua`
```lua
vim.filetype.add({
  extension = {
    xp = "json",
  },
})
```
# Using this repo
## as a cli
*TODO: proper cli docs*
- validate a `.xp` file to determine it matches the expected format
- generate a JSON schema based on the models as they are defined currently. This is mainly for the development of this library 
## as a Rust library
*TODO: cargo crate*
- use the `Xp` model and its sub-models in your project, which allows you to encode, decode, and manipulate experience data
## as a JSON Schema
- [xp.schema.json](./xp.schema.json) — a [JSON Schema](https://json-schema.org/) representing the `Xp` model
# Contributing / Principles
*I haven't decided on a proper contribution policy, but for now all issues and PRs are okay. I have common goals that will guide the review process, and will be codified as needed:*
1. I am using this myself, so its capabilities will likely adapt to my needs, but I only want to add things to the spec that seem like they could also be broadly applicable to others.
2. I specifically do not want to add bloat or complexity for something I think is very simple, so any complexity increases will need good justification
3. At the same time, I do not want decoding to be too rigid, so ****additional fields should be allowed****, meaning you can add your own fields without impacting validation. It's not guaranteed that other implementers of this format will be okay with these extra fields, but this library always will.
4. At first, I am not focusing on migration, since I do not believe this to be any sort of runtime / load-bearing format. It is for storing your own information, and importing/exporting. It will have a clear version history here on GitHub. 
# AI
This format is for humans. Files in this format represent the experiences, education, and skills of real human beings. As a result, I believe a human should be in the loop for any interaction with this format. That being said, I know AI will be used to interact with this, and that's unavoidable. Because of this, consumers of this format must do their own KYC / IDV to ensure they are dealing with a real person. That should always be required for acceptance anyways, so I don't believe this to be a huge issue, but it needed mentioning. If you are using AI to create fake experience files, I am disappointed in you. If you are trying to use this format to represent capabilities or experiences of an AI agent, I am even more disappointed in you.
# Releasing new versions
0. install needed crates
```sh
cargo install cargo-license
cargo install cargo-release
```

1. verify licences of deps
`cargo license`
2. check [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html) 
2. dry run release process
`cargo release`