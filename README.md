# derive_display_from_debug
A trivial Rust macro to derive the Display trait for any type with the Debug trait

## It's as simple as it sounds
All it does is generate a Display implementation that uses the Debug representation of the object.

## Why would someone use this?
I created it to reduce useless code for types that must implement std::error::Error (which requires Display) but have no need to define user-facing text, such as:
* personal test programs
* early versions of software which will only be used by developers

and possibly

* internationalized software, where all user-facing strings are determined by localization resources.

You can use it anywhere you think that the Debug text output is good enough for Display purposes, or where you don't want to implement Display but must do so to satisfy interface requirements.

## How to use it

```rust
use derive_display_from_debug::Display;

#[derive(Debug, Display)]
struct NewStruct {}

```
That's it. Now
```rust
println!("{:?}", NewStruct{});
```
and
```rust
println!("{}", NewStruct{});
```
will have the same output.
