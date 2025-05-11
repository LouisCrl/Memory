# Rust Memory

![main_screen](https://github.com/user-attachments/assets/59bfd6e3-652d-4987-b0c1-19ea16aac2d9)

## Installation

The game is written in [Rust](https://www.rust-lang.org). If you do not have a rust toolchain on your system 
yet, [install](https://www.rust-lang.org/tools/install) it, rustup is the preferred way to change that. When the toolchain is ready, go in your the file, build and run the
game with

    cargo run --release
    

## How to play

When you start the game, you spawn on the welcome page: <br>

![welcome_page](https://github.com/user-attachments/assets/2e84b61b-117d-4280-809a-3793e0254a54)

- Click on the boards to choose the dimensions of the board you will play on. <br>
- Click on the Quit button to quit the game. <br>

If you clicked on a board, the interface will appear to you like this (depending on the dimensions): <br>

![game_page](https://github.com/user-attachments/assets/e167abd0-d119-4346-a9a3-352ad60d6a75)

You have to click on a tile to choose it, you choose 2 tiles and see if this is a pair. <br>
- If this is a pair, well you got one and have to search the other. <br>
- If this is not a pair, no problems, the tiles will just disappear and you can choose others. <br>

![ongoing_game_page](https://github.com/user-attachments/assets/dbb6e86a-d757-45cb-b8de-3ef568f5e045)

When you found every pairs, the game is end and you will the endgame screen will appear to you: <br>

![game_over_page](https://github.com/user-attachments/assets/72af0db1-23e8-444c-bd47-61b6132b9524)

Click on Again to go back on the Welcome Page and choose another board to play. <br>
Click on Quit to quit the game. <br>
