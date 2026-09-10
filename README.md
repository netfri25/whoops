# Whoops
my reimplementation of a unique game that I like a lot, [0h n0](0hn0.com).

### Crates in this project
 - `whoops-core` - provides all core types and traits: grids, tile, position, offset, etc.
 - `whoops-solver` - provides generic solver and generic rule-based solver for generic grids.
 - `whoops-generator` - provides generic grid generator as well as a generator implementation based on an RNG which also uses a solver to generate only valid grids. also provides traits for grid generators.
 - `whoops-frontend` - provides a playable program

### Installation
```shell
cargo install --git https://github.com/netfri25/whoops
```

### Usage
simply run
```
whoops
```

| key | action |
| :---: | :---: |
| s | lazy step / next hint |
| c | complete grid |
| u | undo |
| shift+u | redo |
| 3..9 | generate a new `NxN` grid |
| 0 | generate a `10x10` grid |
| t | toggle lazy step |
| left click | cycle to next tile |
| right click | cycle to previous tile |
