### clix

Configurable extendable CLI tool

#### What does it do?

- Point to a directory structure (ideally a git repo)
- Parses directory structure and builds an executable CLI tool that:
  - creates a subcommand for every "regular" directory
  - creates a subcommand with functionality for every script file - the functionality is the script
  - config folders are `.config` - this could include dependency information for the script or metadata (e.g. renaming commands)
  - resource folders are `.resources` - these will include data files and other resources / assets required for scripts in the parent directory
- That's it!

#### TODO

- ~~Command configuration - allow defining of more complex commands with options and arguments~~
- Different command file types - currently only supports bash
- ~~AppConfig - allow the setting of script repository - currently hard-coded to my local test repository~~
- Remote repository - allow for the setting of a remote git repository
- Install script
- Dependency management (e.g. is python installed?)
