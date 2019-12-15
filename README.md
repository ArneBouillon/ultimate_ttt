# ultimate_ttt

## Introduction

Basic implementation of the game of [Ultimate Tic-Tac-Toe](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe). Features a very bare-bones CLI, as well as a [Monte Carlo Tree Search (MCTS)](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search)-based AI. I wrote this to experiment with both Rust and MCTS, so nothing too advanced. The AI is capable, but takes relatively long to come up with good moves.

## UI

The UI is completely command line based. The board will be printed, using `O` for player 1's moves, and `X` for player 2's. If there is a sub-board in which the current player has to make a move, the left and right bounds of that sub-board will be broader than usual.

To make a move, the user types in X and Y coordinates. Both are 0-indexed. X goes from left to right, Y from top to bottom. In case the entire board is available, the player should first give the (newline-separated) coordinates of the sub-board they want to play in, followed by the (newline-separated) coordinates of the desired square inside the sub-board. If a specific sub-board is required, only the latter two coordinates should be typed.

## AI

The AI uses a basic implementation of MCTS. The time the AI has to make a move is configurable; 2-3 seconds are enough to beat most human players, 10 seconds will beat other reasonably-skilled AIs. I have made some efforts to optimise this (on move 1, it runs about 55,000 simulations per second on my hardware), but it still seems rather slow, which is probably related to my inexperience with using Rust.

After each move, the AI will print the amount of simulations it has managed to do, as well as the results it expects to gain (defeat = 0, draw = 0.5, win = 1).
