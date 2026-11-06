# Tetris Logic

A small and portable rust crate that implements an interface to all the logic behind a tetris game.

## Motivation

The reason I made this is because I love tetris, and found myself implementing much of the logic behind a simple tetris game again and again.

I wanted to run tetris in different places like a game in your terminal, on a micro:bit with an external screen, etc...

## Goals

### Portability

So to make tetris run (almost) everywhere, I need two technical things:

1. No dependencies (no-std), maybe some optional dependencies that would make it easier to use, if you can afford to enable them on your platform.
2. FFI (Foreign Function Interface) so I'm not bound to write the frontend in rust, and could expose my rust library to other languages.

### Rules

Another question with this project that I gotta answer is:

*Which tetris rules am I gonna support?*

Especially since there's so many tetris games out there.

When starting this project I referenced the 2009 Tetris Design Guidelines, which described how e.g. SRS works. Later I figured it could be nice if I had more options for the rules, like in any tetris game, there's different game modes and settings you can tweak, like if you wanna see a ghost piece, or change the rotation system etc...

So the idea is to make the project have as many rules from different games implemented as possible, and then let the user of the library include what rules they want or potentially also implement their own custom rules.

## API

The core idea of this library is to abstract away the implementation of many of the rules in tetris, so the user of the library only have to focus on platform-specific stuff.

The library roughly works like this:

1. The user first supply what rules a game uses using a Settings struct.
2. Then the user does event handling on their specific platform, and calls a function every frame with the actions performed.
3. The user gets back a new frame, and stats to display, or if the game ended it will represent that in some way.

## TODO List

[ ] Add ARR (Automatic Repeat Rate) and DAS (Delayed Auto Shift)
[ ] Add a buffer zone and implement the ways you can lose
[ ] Split the engine into multiple modular parts so I can implement multiple rules
[ ] Lock phase / lockdown rules
[ ] Ghost piece
[ ] Hard drop
[ ] Soft drop
[ ] Completion phase
[ ] Make the board size customizable at compile time
[ ] Make a std feature
[ ] Make the board size customizable at runtime if an allocator is provided
[ ] FFI
[ ] Make more examples
[ ] Make the nix flake nicer to use, and add a nix run command with a ratatui example
