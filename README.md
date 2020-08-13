# scrabble-rs

A simple scrabble game.

## Download
Using git :
```
git clone https://github.com/Koalab99/scrabble-rs.git
```

## Build
Using cargo :
```
cd scrabble-rs
cargo build --release
```

## Playing
Warning : This game is still work in progress, it might not follow exactly the rules.
Start the application :
```
./target/release/scrabble-rs
```

## Contributing
I'm sorry for the lack of documentation, I'm working on it.

You can tweak the `dico.txt` and `english_tileset.txt` file.

### dico.txt
The dictionnary itself. It should have one uppercased word per line

### english_tileset.txt
The set of tile to use.
The syntax is the following
```
<letter> <occurencies> <points>
```
with:
- `<letter>` being one character
- `<occurencies>` the number of tiles with this letter in the bag
- `<points>` the number of points it gives

And a small exception for the wildcard, it is represented by the character '\*'.

### If you feel adventurous
There is a really simple API that will probably change soon that let you implement your own interface.
The interface can be a bot or a user interface.
You can see the trait for it in `src/scrabbleutil/player.rs`
and my basic implementation of it is in `src/scrab_tui/mod.rs`.
It will go away some day, but since the project is just starting it's easier to get everything together.
