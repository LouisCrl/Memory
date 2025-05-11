//this file is the UI management
use std::thread;
use std::time::Duration;
use macroquad::prelude::*;
use crate::model::tiles::{Tiles, TileState, Tile};
use crate::controller::game::{GameState, EndGameChoice};

//colors used in the program
const RASPBERRY:Color = Color::new(199.0 / 255.0, 44.0 / 255.0, 72.0 / 255.0, 1.0);
pub const GREY_PURPLE:Color = Color::new(234.0 / 255.0, 230.0 / 255.0, 248.0 / 255.0, 1.0);
const DARKER_GREY_PURPLE: Color = Color::new(200.0 / 255.0, 196.0 / 255.0, 220.0 / 255.0, 1.0);

pub struct UI;

impl UI {
    //setup and show the welcome page
    pub async fn menu() -> usize {
        let boards = [
            "assets/menu/2x3.png",
            "assets/menu/3x4.png",
            "assets/menu/4x5.png",
            "assets/menu/5x6.png",
        ];

        let mut textures = Vec::new();
        for board in boards {
            let byte = load_file(board).await.unwrap();
            let image = Image::from_file_with_format(&byte, Some(ImageFormat::Png)).unwrap();
            let texture = Texture2D::from_image(&image);
            textures.push(texture);
        }
        let formats = ["2x3", "3x4", "4x5", "5x6"];
        let nb_pairs = [3, 6, 10, 15];

        let dim_square_x = textures[0].width() / 5.0;
        let dim_square_y = textures[0].height() / 5.0;

        let coor_square_x = (screen_width() - dim_square_x) / 5.0;
        let coor_square_y = (screen_height() - dim_square_y) / 2.0;

        let font_size_format = 42.0;

        let welcome = "Welcome";
        let font_size_welcome = 72.0;
        let text_start_dimensions = measure_text(welcome, None, font_size_welcome as u16, 1.0);
        let coor_start_x = (screen_width() - text_start_dimensions.width) / 2.0;
        let coor_start_y = (screen_height() - text_start_dimensions.height) / 6.0;
        let choose = "Choose your gamemode !";
        let font_size_choose = 56.0;
        let text_choose_dimensions = measure_text(choose, None, font_size_choose as u16, 1.0);
        let coor_choose_x = (screen_width() - text_choose_dimensions.width) / 2.0;
        let coor_choose_y = coor_start_y + 72.0;

        let text_dimensions = measure_text("Game Over", None, 72.0 as u16, 1.0);
        let box_dimensions = (text_dimensions.width + 80.0, text_dimensions.height + 30.0);
        let coor_box_x = (screen_width() - box_dimensions.0) / 2.0;
        let coor_quit_box_y = 4.0 * ((screen_height() - box_dimensions.1) / 5.0);
        let border = 5.0;

        let quit_text = "Quit";
        let font_size_button = 56.0;
        let quit_text_dimensions = measure_text(quit_text, None, font_size_button as u16, 1.0);
        let quit_coor_x = coor_box_x + (box_dimensions.0 - quit_text_dimensions.width) / 2.0;
        let quit_coor_y = coor_quit_box_y + (box_dimensions.1 + quit_text_dimensions.height) / 2.0;

        loop {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if coor_quit_box_y - border <= mouse_y 
                    && mouse_y <= coor_quit_box_y + box_dimensions.1 + border 
                    && coor_box_x - border <= mouse_x 
                    && mouse_x <= coor_box_x + box_dimensions.0 + border 
                {
                    return 0;
                }
                for i in 0..textures.len() {
                    let c_x = (i + 1) as f32;
                    let current_square_x = c_x * coor_square_x;
                    if coor_square_y - border <= mouse_y 
                        && mouse_y <= coor_square_y + dim_square_y + border 
                        && current_square_x - border <= mouse_x 
                        && mouse_x <= current_square_x + dim_square_x + border 
                    {
                        while is_mouse_button_down(MouseButton::Left) {
                            clear_background(GREY_PURPLE);
                            next_frame().await;
                        }
                        return nb_pairs[i];
                    }
                }
            }

            clear_background(GREY_PURPLE);

            draw_text(welcome, coor_start_x, coor_start_y, font_size_welcome, BLACK);
            draw_text(choose, coor_choose_x, coor_choose_y, font_size_choose, BLACK);

            for (i, texture) in textures.iter().enumerate() {
                let c_x = (i + 1) as f32;
                let current_square_x = c_x * coor_square_x;
                draw_rectangle(
                    current_square_x - border,
                    coor_square_y - border,
                    dim_square_x + 2.0 * border,
                    dim_square_y + 2.0 * border,
                    BLACK
                );
                draw_texture_ex(
                    texture,
                    current_square_x,
                    coor_square_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(dim_square_x, dim_square_y)),
                        ..Default::default()
                    },
                );
                let format_text = formats[i];
                let text_dims = measure_text(format_text, None, font_size_format as u16, 1.0);
                let coor_format_x = current_square_x + (dim_square_x - text_dims.width) / 2.0;
                let coor_format_y = coor_square_y + dim_square_y + 30.0;
                draw_text(format_text, coor_format_x, coor_format_y, font_size_format, BLACK);
            }
            //Quit
            draw_rectangle(
                coor_box_x - border,
                coor_quit_box_y - border,
                box_dimensions.0 + border * 2.0,
                box_dimensions.1 + border * 2.0,
                BLACK
            );
            draw_rectangle(
                coor_box_x,
                coor_quit_box_y,
                box_dimensions.0,
                box_dimensions.1,
                DARKER_GREY_PURPLE
            );
            draw_text(quit_text, quit_coor_x, quit_coor_y, font_size_button, BLACK);

            next_frame().await;
        }
    }

    //setup and show the game screen
    pub async fn render_game(nb_pairs: usize) {
        let mut state = GameState::Ongoing;

        let mut board = Tiles::init(nb_pairs);

        let mut current_tiles: Vec<(usize, usize)> = Vec::new();

        let mut textures = Self::charge_textures(&board, &current_tiles).await;

        let n = textures[0].len() as f32;
        let spacing = 20.0;
        let texture_width = 243.0 / (n / 1.5);
        let texture_height = 365.0 / (n / 1.5);
        let total_width = textures[0].len() as f32 * (texture_width + spacing) - spacing;
        let total_height = textures.len() as f32 * (texture_height + spacing) - spacing;
        let start_x = (screen_width() - total_width) / 2.0;
        let start_y = (screen_height() - total_height) / 2.0;

        let last_index = textures.len() - 1;
        let last_width = textures[last_index].len() as f32 * (texture_width + spacing) - spacing;
        let last_x = (screen_width() - last_width) / 2.0;

        while state == GameState::Ongoing {
            if current_tiles.len() == 2 {
                thread::sleep(Duration::from_millis(500));
                current_tiles.clear();
                textures = Self::charge_textures(&board, &current_tiles).await;
            }
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();

                for (i, textures_line) in textures.clone().iter().enumerate() {
                    let y_base_rect = start_y + i as f32 * (texture_height + spacing);
                    for (j, _texture) in textures_line.iter().enumerate() {
                        let x_base_rect = start_x + j as f32 * (texture_width + spacing);

                        let rect = Rect::new(x_base_rect, y_base_rect, texture_width, texture_height);
                        if rect.contains(vec2(mouse_x, mouse_y)) {
                            current_tiles.push((i, j));
                            textures = Self::charge_textures(&board, &current_tiles).await;
                            if current_tiles.len() == 2 {
                                if board.check_possible(&current_tiles[0]) && board.check_possible(&current_tiles[1]) {
                                    if board.check_equals_tiles(&board.tiles()[current_tiles[0].0][current_tiles[0].1], &board.tiles()[current_tiles[1].0][current_tiles[1].1]) {
                                        board.change_tile(current_tiles[0]);
                                        board.change_tile(current_tiles[1]);
                                        if board.check_all_discovered() {
                                            state = GameState::End;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            clear_background(GREY_PURPLE);

            for (i, texture_line) in textures.iter().enumerate() {
                let y = start_y + i as f32 * (texture_height + spacing);
                for (j, texture) in texture_line.iter().enumerate() {
                    let mut x = start_x + j as f32 * (texture_width + spacing);
                    if i == last_index {
                        x = last_x + j as f32 * (texture_width + spacing);
                    }
                    Self::draw_rounded_rectangle(
                        x,
                        y,
                        texture_width,
                        texture_height,
                        60.0 / (n / 1.5),
                        RASPBERRY
                    );
                    draw_texture_ex(
                        texture,
                        x,
                        y,
                        BLACK,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(texture_width, texture_height)),
                            ..Default::default()
                        },
                    );
                }
            }

            next_frame().await;
        }
    }

    //setup and show the endgame screen
    pub async fn endgame() -> EndGameChoice {
        let over_text = "Game Over";
        let font_size_title = 72.0;
        let text_dimensions = measure_text(over_text, None, font_size_title as u16, 1.0);
        let coor_text_x = (screen_width() - text_dimensions.width) / 2.0;
        let coor_text_y = (screen_height() - text_dimensions.height) / 2.0;

        let box_dimensions = (text_dimensions.width + 80.0, text_dimensions.height + 30.0);
        let coor_box_x = (screen_width() - box_dimensions.0) / 2.0;
        let coor_again_box_y = 3.0 * ((screen_height() - box_dimensions.1) / 4.0);
        let border = 5.0;

        let font_size_button = 56.0;

        let again_text = "Again";
        let again_text_dimensions = measure_text(again_text, None, font_size_button as u16, 1.0);
        let again_coor_x = coor_box_x + (box_dimensions.0 - again_text_dimensions.width) / 2.0;
        let again_coor_y = coor_again_box_y + (box_dimensions.1 + again_text_dimensions.height) / 2.0;

        let coor_quit_box_y = coor_again_box_y + (screen_height() - box_dimensions.1) / 6.0;

        let quit_text = "Quit";
        let quit_text_dimensions = measure_text(quit_text, None, font_size_button as u16, 1.0);
        let quit_coor_x = coor_box_x + (box_dimensions.0 - quit_text_dimensions.width) / 2.0;
        let quit_coor_y = coor_quit_box_y + (box_dimensions.1 + quit_text_dimensions.height) / 2.0;

        loop {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                //Again
                if coor_again_box_y - border <= mouse_y 
                    && mouse_y <= coor_again_box_y + box_dimensions.1 + border 
                    && coor_box_x - border <= mouse_x 
                    && mouse_x <= coor_box_x + box_dimensions.0 + border 
                {
                    return EndGameChoice::Again;
                }
                //Quit
                else if coor_quit_box_y - border <= mouse_y 
                    && mouse_y <= coor_quit_box_y + box_dimensions.1 + border 
                    && coor_box_x - border <= mouse_x 
                    && mouse_x <= coor_box_x + box_dimensions.0 + border 
                {
                    return EndGameChoice::Quit;
                }
            }

            clear_background(GREY_PURPLE);

            //Game Over
            draw_text(over_text, coor_text_x, coor_text_y, font_size_title, BLACK);

            //Again
            draw_rectangle(
                coor_box_x - border,
                coor_again_box_y - border,
                box_dimensions.0 + border * 2.0,
                box_dimensions.1 + border * 2.0,
                BLACK
            );
            draw_rectangle(
                coor_box_x,
                coor_again_box_y,
                box_dimensions.0,
                box_dimensions.1,
                DARKER_GREY_PURPLE
            );
            draw_text(again_text, again_coor_x, again_coor_y, font_size_button, BLACK);

            //Quit
            draw_rectangle(
                coor_box_x - border,
                coor_quit_box_y - border,
                box_dimensions.0 + border * 2.0,
                box_dimensions.1 + border * 2.0,
                BLACK
            );
            draw_rectangle(
                coor_box_x,
                coor_quit_box_y,
                box_dimensions.0,
                box_dimensions.1,
                DARKER_GREY_PURPLE
            );
            draw_text(quit_text, quit_coor_x, quit_coor_y, font_size_button, BLACK);

            next_frame().await;
        }
    }

    //draw a rectangle with rounded corners
    fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
        draw_rectangle(
            x + radius,
            y,
            w - 2.0 * radius,
            h,
            color
        );
        draw_rectangle(
            x,
            y + radius,
            w,
            h - 2.0 * radius,
            color
        );

        draw_circle(
            x + radius,
            y + radius,
            radius,
            color
        );
        draw_circle(
            x + w - radius,
            y + radius,
            radius,
            color
        );
        draw_circle(
            x + radius,
            y + h - radius,
            radius,
            color
        );
        draw_circle(
            x + w - radius,
            y + h - radius,
            radius,
            color
        );
    }

    //charge and stock the textures of every tiles
    pub async fn charge_textures(tiles: &Tiles, coords: &Vec<(usize, usize)>) -> Vec<Vec<Texture2D>> {
        let mut board = Vec::new();
        for r in 0..tiles.tiles().len() {
            let mut lines = Vec::new();
            for c in 0..tiles.tiles()[r].len() {
                match tiles.tiles_state()[r][c] {
                    TileState::Discovered => lines.push(Self::get_texture(&tiles.tiles()[r][c]).await),
                    TileState::NotDiscovered => {
                        if coords.contains(&(r, c)) {
                            lines.push(Self::get_texture(&tiles.tiles()[r][c]).await);
                        } else {
                            let byte = load_file("assets/empty.png").await.unwrap();
                            let image = Image::from_file_with_format(&byte, Some(ImageFormat::Png)).unwrap();
                            let texture = Texture2D::from_image(&image);
                            lines.push(texture);
                        }
                    },
                }
            }
            board.push(lines);
        }
        board
    }

    //give the path for every tiles pictures
    pub fn get_texture_for_tile(tile: &Tile) -> &'static str {
        match tile {
            Tile::Circle1 => "assets/circle/circle_1.png",
            Tile::Circle2 => "assets/circle/circle_2.png",
            Tile::Circle3 => "assets/circle/circle_3.png",
            Tile::Circle4 => "assets/circle/circle_4.png",
            Tile::Circle5 => "assets/circle/circle_5.png",
            Tile::Circle6 => "assets/circle/circle_6.png",
            Tile::Circle7 => "assets/circle/circle_7.png",
            Tile::Circle8 => "assets/circle/circle_8.png",
            Tile::Circle9 => "assets/circle/circle_9.png",
            Tile::Stick1 => "assets/stick/stick_1.png",
            Tile::Stick2 => "assets/stick/stick_2.png",
            Tile::Stick3 => "assets/stick/stick_3.png",
            Tile::Stick4 => "assets/stick/stick_4.png",
            Tile::Stick5 => "assets/stick/stick_5.png",
            Tile::Stick6 => "assets/stick/stick_6.png",
            Tile::Stick7 => "assets/stick/stick_7.png",
            Tile::Stick8 => "assets/stick/stick_8.png",
            Tile::Stick9 => "assets/stick/stick_9.png",
            Tile::Character1 => "assets/character/character_1.png",
            Tile::Character2 => "assets/character/character_2.png",
            Tile::Character3 => "assets/character/character_3.png",
            Tile::Character4 => "assets/character/character_4.png",
            Tile::Character5 => "assets/character/character_5.png",
            Tile::Character6 => "assets/character/character_6.png",
            Tile::Character7 => "assets/character/character_7.png",
            Tile::Character8 => "assets/character/character_8.png",
            Tile::Character9 => "assets/character/character_9.png",
        }
    }

    //take the tile and return the texture
    pub async fn get_texture(tile: &Tile) -> Texture2D {
        let byte = load_file(Self::get_texture_for_tile(tile)).await.unwrap();
        let image = Image::from_file_with_format(&byte, Some(ImageFormat::Png)).unwrap();
        let texture = Texture2D::from_image(&image);
        texture
    }
}
