use raylib::prelude::*;
use physics_symulation::Screen;
use physics_symulation::Screen::{ExitScreen, GameScreen, MainScreen, PauseScreen};
mod button;
use crate::button::Button;

fn main() {

    let fps:u32 = 60;
    let title = "Mini-Shooter";
    let width = 1100;
    let height = 700;

    let size_x = 250.0;
    let size_y = 70.0;
    let spacing:f32 = 20.0;
    let mut text;
    let font_size = 40;
    let centre_x = (width / 2) as f32;
    let centre_y = (height / 2) as f32;
    let num_cols = 2;
    let mut is_black = true;
    let mut mode = dark_light::detect().unwrap();
    let mut bg_color = Color::BLACK;

    let button_label = ["PLAY", "OPTIONS", "EXIT"];
    let menu_label = ["THEME", "SOON", "SOON", "SOON", "SOON", "SOON"];

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title(title)
        .build();

    rl.set_target_fps(fps);
    rl.set_exit_key(None);

    let mut current_screen = Screen::MainScreen;

    while !rl.window_should_close() {

        match current_screen{
            Screen::MainScreen => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE){
                    current_screen = ExitScreen;
                }

                for (count, label) in button_label.iter().enumerate() {
                    let buttons = Button::new(
                        centre_x - (size_x / 2.0),
                        centre_y + ((size_y + spacing) * count as f32),
                        size_x,
                        size_y,
                        label,
                        font_size);

                    if buttons.is_clicked(&mut rl) {
                        match *label {
                            "PLAY" => current_screen = Screen::GameScreen,
                            "OPTIONS" => current_screen = Screen::MenuScreen,
                            "EXIT" => current_screen = ExitScreen,
                            _ => {}
                        }
                    }
                }


            }
            Screen::MenuScreen => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE){
                    current_screen = MainScreen;
                }

                for (index, label) in menu_label.iter().enumerate() {
                    let col = (index % num_cols) as f32;
                    let row = (index / num_cols) as f32;
                    let x = centre_x + (col * (size_x + spacing)) - ((num_cols as f32 * size_x + (num_cols as f32 - 1.0) * spacing) / 2.0);
                    let y = centre_y + (row * (size_y + spacing)) - 50.0;

                    let buttons = Button::new(
                        x,
                        y,
                        size_x,
                        size_y,
                        label,
                        font_size,
                    );

                    if buttons.is_clicked(&mut rl) {
                        match *label {
                            "THEME" => {
                                if mode == dark_light::Mode::Dark{
                                    mode = dark_light::Mode::Light;
                                    bg_color = Color::WHITE;

                                }else if mode == dark_light::Mode::Light {
                                    mode = dark_light::Mode::Dark;
                                    bg_color = Color::BLACK;
                                }else {
                                    mode = dark_light::Mode::Dark;
                                    bg_color = Color::BLACK;
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            Screen::GameScreen => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE){
                    current_screen = PauseScreen;
                }
            }
            Screen::PauseScreen => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE){
                    current_screen = GameScreen;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER){
                    current_screen = MainScreen;
                }
            }
            Screen::ExitScreen => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER){
                    break;
                }
            }
            Screen::DeathScreen => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE){
                    current_screen = MainScreen;
                }
            }

            _ => {
                panic!("Cannot open the screen");
            }
        }


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        match current_screen {
            Screen::MainScreen => {
                d.draw_text("MAIN SCREEN", 50, 50, 50, Color::RED);

                text = "Welcome in Mini-Shooter!";

                let text_width = d.measure_text(text, font_size);

                for (count, label) in button_label.iter().enumerate() {
                    let buttons = Button::new(
                        centre_x - (size_x / 2.0),
                        centre_y + ((size_y + spacing) * count as f32),
                        size_x,
                        size_y,
                        label,
                        font_size);
                    buttons.draw(&mut d);
                }

                d.draw_text(
                    text,
                    (centre_x as i32) - (text_width / 2),
                    (centre_y as i32) - ((spacing as i32) * 4),
                    font_size,
                    Color::BLUE);

            }
            Screen::MenuScreen => {
                d.draw_text("MENU SCREEN", 50, 50, 50, Color::RED);

                for (index, label) in menu_label.iter().enumerate() {
                    let col = (index % num_cols) as f32;
                    let row = (index / num_cols) as f32;
                    let x = centre_x + (col * (size_x + spacing)) - ((num_cols as f32 * size_x + (num_cols as f32 - 1.0) * spacing) / 2.0);
                    let y = centre_y + (row * (size_y + spacing)) - 50.0;

                    let buttons = Button::new(
                        x,
                        y,
                        size_x,
                        size_y,
                        label,
                        font_size,
                    );
                    buttons.draw(&mut d);
                }
            }
            Screen::GameScreen => {
                d.draw_text("GAME SCREEN", 50, 50, 50, Color::RED);
            }
            Screen::PauseScreen => {
                d.draw_text("PAUSE SCREEN", 50, 50, 50, Color::RED);
            }
            Screen::ExitScreen => {
                d.draw_text("EXIT SCREEN", 50, 50, 50, Color::RED);
                let exit_button = Button::new(
                    centre_x - (size_x / 2.0),
                    centre_y + (size_y + spacing),
                    size_x,
                    size_y,
                    "EXIT",
                    font_size);

                text = "Click to Exit!";

                let textWidth = d.measure_text(text, font_size);

                d.draw_text(
                    text,
                    (width - textWidth) / 2,
                    (centre_y as i32) - ((spacing as i32) * 4),
                    font_size,
                    Color::RED);

                exit_button.draw(&mut d);

                if exit_button.is_clicked(&mut d){
                    break;
                }
            }
            Screen::DeathScreen => {
                unimplemented!();
            }
            _ => {
                panic!("Cannot draw the screen");
            }
        }

    }
}