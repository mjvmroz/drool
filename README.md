# drool 🤤

A work-in-progress [lox 🐟](https://craftinginterpreters.com/) bytecode interpreter, written in Rust 🦀.

"Drool" because crab stuffed salmon is an insanely decadent, guilty pleasure which produces this response.

This project likely won't do that, as it's my first foray into the language.

## Approach

I'm trying to squeeze performance out of this thing for the interpreter, but am sacrificing a little for ergonomics in the assembler and disassembler.

It's possible that I'm losing some juice in my instruction decoding. I think Rust should be able to understand through my inlining that the VM's use of the `Op` struct is transient in release builds, but I'm not 100% sure.
