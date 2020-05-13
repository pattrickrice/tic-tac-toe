# Tic Tac Toe in Rust

After reading through "the book" as I've started learning rust, I thought a good starting place for applying what I've learned is building a tic-tac-toe game. The program also uses [box drawing characters](https://en.wikipedia.org/wiki/Box-drawing_character).

## Launching the program

```bash
$git clone git@github.com:pattrickrice/tic-tac-toe.git
$cargo run
➜  tic-tac-toe git:(master) cargo run             
   Compiling tic-tac-toe v0.1.0 (/home/patrick/tic-tac-toe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/tic-tac-toe`
┌───┬───┬───┐
│   │   │   │
├───┼───┼───┤
│   │   │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘

Turn X: Enter [row, col]: 
```

## Room for improvement

Here's a list of known places where this program could be improved in case I decided I want to put more free time into this.
1. I'm sure the code could use a refactor.
2. Unit tests
3. Integration tests
3. GitHub ci
4. n-curses-like terminal rendering (n-curses only has bindings on linux?)
5. Minimal AI; maybe something that just chooses a random available square.
