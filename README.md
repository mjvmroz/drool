# drool 🤤

A work-in-progress [lox 🐟](https://craftinginterpreters.com/) bytecode interpreter, written in Rust 🦀.

"Drool" because crab stuffed salmon is an insanely decadent, guilty pleasure which produces this response.

This project likely won't do that, as it's my first foray into the language.

## Approach

I'm trying to squeeze performance out of this thing for the interpreter, but am sacrificing a little for ergonomics in the assembler and disassembler.

The result is that I have some stuff (eg. `Operation` / `OpCode`) encoded twice. If I knew more about Rust, perhaps I could avoid this 🤷‍♂️. I'm figuring it out as I go, but I'm already sold. This language is awesome.
