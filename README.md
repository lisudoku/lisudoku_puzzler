# lisudoku_puzzler

Used by https://lisudoku.xyz to generate training puzzles.

Currently it only works with classic 9x9 sudokus and 2 techniques (hidden singles, naked singles).

Usage
```
cargo run -- initial_grid technique user_solution_steps
```

Returns a list of grids along with the list of cells where you can apply the provided technique.

Example
```
cargo run -- 3000010002100000 HiddenSingle '[{"type":"digit","cells":[{"row":0,"col":3}],"value":1,"time":16},{"type":"digit","cells":[{"row":3,"col":0}],"value":1,"time":18},{"type":"digit","cells":[{"row":3,"col":1}],"value":3,"time":18},{"type":"digit","cells":[{"row":2,"col":0}],"value":4,"time":20},{"type":"digit","cells":[{"row":2,"col":3}],"value":3,"time":21},{"type":"digit","cells":[{"row":1,"col":2}],"value":3,"time":23},{"type":"digit","cells":[{"row":1,"col":0}],"value":2,"time":32},{"type":"digit","cells":[{"row":1,"col":3}],"value":4,"time":33},{"type":"digit","cells":[{"row":0,"col":2}],"value":2,"time":34},{"type":"digit","cells":[{"row":0,"col":1}],"value":4,"time":35},{"type":"digit","cells":[{"row":3,"col":2}],"value":4,"time":36},{"type":"digit","cells":[{"row":3,"col":3}],"value":2,"time":38}]'
[["3000010002100000",[{"position":{"row":0,"col":3},"value":1},{"position":{"row":2,"col":3},"value":3},{"position":{"row":3,"col":0},"value":1},{"position":{"row":1,"col":0},"value":2},{"position":{"row":3,"col":1},"value":3}]],["3001010002100000",[{"position":{"row":0,"col":2},"value":2},{"position":{"row":2,"col":3},"value":3},{"position":{"row":3,"col":0},"value":1},{"position":{"row":1,"col":0},"value":2},{"position":{"row":3,"col":1},"value":3}]],...]
```

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).
