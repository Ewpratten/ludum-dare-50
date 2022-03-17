# Third Party Dependencies

Since we evolve so fast during game jams, it is sometimes impractical to deal with the pull-request/review process of our dependencies.

We also have a habit of breaking well-known Rust crates during these weekends, so it has just become more practical to build our game in a bit of a monorepo using custom forks of dependencies.

## Should I add my dependency as a submodule

Probably not.

But, if you have found a critical bug in something (like raylib), and cannot get it merged upstream, just fork the repo, add it here with `git submodule add <repo>` and use it.