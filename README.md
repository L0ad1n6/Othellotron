# Othellotron

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

My home baked Othello Bot. It is decent.

## Table of Contents

- [Setup](#setup)
- [How do I play?](#how-do-i-play)
- [How does it play?](#how-does-it-play)
- [Project Structure](#project-structure)
- [Future Ideas](#future-ideas)
- [License](#license)

### Setup
First step is to download [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html#:~:text=The%20easiest%20way%20to%20get,rustup%20will%20also%20install%20cargo%20.&text=It%20will%20download%20a%20script%2C%20and%20start%20the%20installation.).

Second step is to clone the GitHub repository:
```
git clone https://github.com/L0ad1n6/Othellotron
```

Last step is to compile and run
```
cd Othellotron

cargo run --release
```

NOTE: If you are on MacOS or Linux you can use the binary in the latest release. Future releases will have more supported platforms.

### How do I play?
```
    A   B   C   D   E   F   G   H
  +---+---+---+---+---+---+---+---+
8 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
7 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
6 |   |   |   |   | X |   |   |   |
  +---+---+---+---+---+---+---+---+
5 |   |   |   | W | B | X |   |   |
  +---+---+---+---+---+---+---+---+
4 |   |   | X | B | W |   |   |   |
  +---+---+---+---+---+---+---+---+
3 |   |   |   | X |   |   |   |   |
  +---+---+---+---+---+---+---+---+
2 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
1 |   |   |   |   |   |   |   |   |
  +---+---+---+---+---+---+---+---+
```

Note the board above is not colored but printed board is.

W = White Pieces,
B = Black Pieces,
X = Valid moves for current player

To play enter the row when you are prompted with:
```
Enter Row (Number):
```

And enter the letter of the column when you are prompted with:
```
Enter Column (Letter):
```

If move is invalid, you will restart the process until you enter a valid move.

Unfortunately if you accidentally miss-type something, the program will panic and the game will be lost. This will not be an issue in future versions (Hopefully).

#### Example of panicking move:
```
Enter Row (Number): 2a
```

### How does it play?

The observant player might wonder, but how does a computer even play othello.

The computer uses a variety of techniques to search and find good moves.

1. The bot generates all moves for the current board state
2. The bot iterates over the array of moves:
    * playing the move
    * generating all moves for new state
    * un-playing the move
3. Step 2 continues until a certain depth is reached
4. All states at the base of the game tree are evaluated
5. The value of the states is propagated back to the root
6. The moves are sorted by the best state that they can lead to
7. The best move is chosen and played on the board

This is a simple overview of what the bot does every round. Some of the techniques that have been used to execute and optimize this process are listed down below.

#### [Minimax](https://en.wikipedia.org/wiki/Minimax)

Defined as the smallest set of maximum possible losses, minimax is what was used to find moves that minimize the damage that the opponent can do.

#### [Alpha-Beta Pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning)

Alpha-Beta pruning is an optimization for Minimax, if a node presents itself to be better for the opponent than what we have already searched then the branch can be pruned as it is no longer an option.

#### [Iterative Deepening - Coming Soon](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search)

Iterative deepening is a good solution to the problem of finding what depth to search to. Instead of searching to a fixed depth, the process of iterative deepening incrementally increases the depth. When time runs out you can take the results from the last completed search. Typically iterative deepening is faster then searching to a depth of "N" as you can use the results of each previous search to order the moves that are searched to increase the number of moves that are pruned.

### Project Structure

| Name                 | Description                                                                |
| -------------------- | -------------------------------------------------------------------------- |
| **Cargo.toml**       | Has all crate dependencies and crate information                           |
| **Cargo.lock**       | File generated from Cargo.toml                                             |
| **LICENSE**          | Holds license for crate                                                    |
| **.gitignore**       | Standard gitignore file to prevent unwanted files form being committed     |
| **src/main.rs**      | Entry point to crate                                                       |
| **src/human.rs**     | Contains code for all human related operations                             |
| **src/game**         | Contains code to run the game of othello                                   |
| **src/bot**          | Contains code for all bot actions                                          |
| **src/bot/moves**    | Contains the code for move related operations                              |

### Future Ideas

This project is far from perfect. There is lots of room for improvement as its play style is decent at best. In the future I hope to implement these optimizations to improve the performance of the bot:

* Iterative Deepening
* Pruned position search while generating moves
* Transposition table for looking up board states that have been evaluated during search
* Cache for slow functions: generate moves, evaluate (Transposition Table)
* Improve UI/UX. Possible solutions:
  *  writing custom user input function with standard library, 
  * Making a webpage *that hosts on local host to play the game more interactively
* Trying Monte-Carlo Tree Search
* Optimize constants and factors in evaluation function
* Potentially switch board data structure to bitmaps
* Multi Threading!

### License

Licensed under the [MIT](LICENSE) License

Copyright (c) 2022 Altan Mehmet Ünver (L0ad1n6)

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.