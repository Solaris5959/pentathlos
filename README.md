# Obsidian MCP Server TODO

## P0 — Core / Essential

- [x] Expose vault directories and files as MCP Resources
- [ ] Tool to read notes
- [ ] Tool to create new notes
- [ ] Tool to append text to existing notes
- [ ] Tool to search & replace text within a note (replaces granular list/task editing)
- [ ] Tool to delete notes
- [ ] Tool to move/rename files (without auto-updating references)
- [ ] Tool to search vault by filename
- [ ] Tool to search vault by text content
- [ ] Basic Markdown parsing / AST in Rust
- [ ] YAML/frontmatter parsing in Rust
- [ ] Prevent writes outside the configured vault root
- [ ] Atomic file writes
- [ ] Configurable excluded paths
- [ ] Structured MCP tool outputs
- [ ] Consistent MCP error handling

## P1 — Obsidian Semantics & Properties

- [ ] Parse and resolve Obsidian internal links (`[[]]`)
- [ ] Support link aliases (`[[note|alias]]`)
- [ ] Support heading links (`[[note#heading]]`)
- [ ] Support block references (`[[note#^block]]`)
- [ ] Parse Obsidian embeds (`![[...]]`)
- [ ] Find backlinks to a note
- [ ] Find outgoing links from a note
- [ ] Find unresolved/broken internal links
- [ ] Read note properties / frontmatter
- [ ] Update note properties
- [ ] Query notes by property
- [ ] Validate frontmatter against a schema
- [ ] List all tags and usage counts
- [ ] Find notes by tag
- [ ] Correctly distinguish tags in Markdown, YAML, and code blocks

## P1 — Safe Mutations

- [ ] Add dry-run mode for mutations
- [ ] Return before/after diffs for edits
- [ ] Add atomic multi-file operations
- [ ] Maximum file-size limits
- [ ] Require explicit confirmation for destructive operations

## P2 — Search & Context (LLM Power Tools)

- [ ] Regex search
- [ ] Search within a directory/subtree
- [ ] Search excluding configured directories
- [ ] Tag & Property search
- [ ] Link/backlink search
- [ ] Search surrounding context around matches (return relevant sections)
- [ ] `get_note_context` tool (Mega-tool returning note contents + frontmatter + backlinks + outgoing links + tags in one payload)
- [ ] Tool to summarize a note's connections
- [ ] Vault health/diagnostics tool (Find broken links, orphans, empty notes, malformed YAML)

## P2 — MCP Resources

- [ ] `obsidian://vault`
- [ ] `obsidian://note/<path>`
- [ ] `obsidian://backlinks/<note>`
- [ ] `obsidian://tags/<tag>`
- [ ] `obsidian://recent`
- [ ] `obsidian://orphans`
- [ ] `obsidian://broken-links`
- [ ] `obsidian://daily/today`

## P3 — Architecture & Infrastructure

- [ ] Build reusable Rust vault engine independent of MCP
- [ ] Keep MCP layer thin over vault primitives
- [ ] Create reusable `resolve_link()` primitive
- [ ] Create reusable `backlinks()` primitive
- [ ] Create reusable `search()` abstraction
- [ ] Create reusable mutation/diff abstraction
- [ ] Add comprehensive unit tests
- [ ] Add integration tests against real Obsidian vaults
- [ ] Add fixture vault containing common Obsidian syntax
- [ ] Benchmark indexing/search performance

## P4 — Reach Goals & Advanced Features

- [ ] Detect newly added attachments
- [ ] Recent Screenshot Sorting to appropriate note directories
- [ ] Find notes referencing an attachment
- [ ] Find unused/missing attachments
- [ ] Parse callouts
- [ ] Parse aliases
- [ ] Parse footnotes
- [ ] Parse comments
- [ ] Detect Dataview queries
- [ ] Detect Tasks plugin syntax
- [ ] Detect Templater templates
- [ ] Support daily notes & note templates natively
- [ ] Vault graph mapping (Query incoming/outgoing graph edges)
- [ ] Find connected/isolated notes
- [ ] Find clusters of related notes
- [ ] Export vault graph (JSON, Graphviz/DOT, Mermaid)
- [ ] Watch vault filesystem for changes & incrementally update index
- [ ] MCP prompt for daily note creation
- [ ] MCP prompt for weekly review
- [ ] Allow user-defined prompt templates

## Running/Developing

To test against the MCP server, use `npx @modelcontextprotocol/inspector cargo run --quiet` to send messages to the server and receive responses. 

To build and run the MCP server, use `cargo build` and `cargo run --quiet`.

To connect the MCP server to you local LLM, point the MCP to the local working directory, with command `cargo` and arguments `run` and `--quiet`.
    Note that `--quiet` is required, otherwise the LLM will be inundated with logs and it will be unable to properly parse the incoming informatio