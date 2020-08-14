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
There is an API that will probably change soon that let you implement your own interface.
The interface can be a bot or a user interface.
The trait for it is `PlayerTrait`.
If you want some documentation, you can generate it with:
```
cargo doc
```
And open it in your browser with
```
cargo doc --open
```

### Direction of the project
At the moment there is three main parts to it.
* The main part
	Well, its just the main.rs file
* The core part
	Everything in `src/scrabbleutils`. It gives some tools and the API for the game.
* The interface
	It is in `src/scrab_tui`, it is a first implementation of the PlayerTrait. That offers a first feel of what works and what doesn't

And it is pretty good for now, since not everyting is working as intended.
But when the project will be stable enough, it will hopefully split into three separate repositories.
