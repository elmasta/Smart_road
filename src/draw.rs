use crate::logic::{Direction, Car, CarType};

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::Texture;
use sdl2::ttf::Sdl2TtfContext;

use std::path::Path;

pub fn road_drawing(canvas: &mut Canvas<Window>) {
    // Clear the canvas with a black color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    //Road drawing
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    // x
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((i*50+25, 350), (i*50+75, 350));
        } else if i == 4 {
            let _ = canvas.draw_line((i*50+25, 350), (i*50+50, 350));
        } else if i == 11 {
            let _ = canvas.draw_line((i*50, 350), (i*50+25, 350));
        }
    }
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((i*50+25, 300), (i*50+75, 300));
        } else if i == 4 {
            let _ = canvas.draw_line((i*50+25, 300), (i*50+50, 300));
        } else if i == 11 {
            let _ = canvas.draw_line((i*50, 300), (i*50+25, 300));
        }
    }
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((i*50+25, 450), (i*50+75, 450));
        } else if i == 4 {
            let _ = canvas.draw_line((i*50+25, 450), (i*50+50, 450));
        } else if i == 11 {
            let _ = canvas.draw_line((i*50, 450), (i*50+25, 450));
        }
    }
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((i*50+25, 500), (i*50+75, 500));
        } else if i == 4 {
            let _ = canvas.draw_line((i*50+25, 500), (i*50+50, 500));
        } else if i == 11 {
            let _ = canvas.draw_line((i*50, 500), (i*50+25, 500));
        }
    }
    let _ = canvas.draw_line((0, 250), (800, 250));
    let _ = canvas.draw_line((0, 400), (800, 400));
    let _ = canvas.draw_line((0, 550), (800, 550));
    // y
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((350, i*50+25), (350, i*50+75));
        } else if i == 4 {
            let _ = canvas.draw_line((350, i*50+25), (350, i*50+50));
        } else if i == 11 {
            let _ = canvas.draw_line((350, i*50), (350, i*50+25));
        }
    }
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((300, i*50+25), (300, i*50+75));
        } else if i == 4 {
            let _ = canvas.draw_line((300, i*50+25), (300, i*50+50));
        } else if i == 11 {
            let _ = canvas.draw_line((300, i*50), (300, i*50+25));
        }
    }
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((450, i*50+25), (450, i*50+75));
        } else if i == 4 {
            let _ = canvas.draw_line((450, i*50+25), (450, i*50+50));
        } else if i == 11 {
            let _ = canvas.draw_line((450, i*50), (450, i*50+25));
        }
    }
    for i in 0..16 {
        if i % 2 == 0 && !(i >=4 && i <= 11) {
            let _ = canvas.draw_line((500, i*50+25), (500, i*50+75));
        } else if i == 4 {
            let _ = canvas.draw_line((500, i*50+25), (500, i*50+50));
        } else if i == 11 {
            let _ = canvas.draw_line((500, i*50), (500, i*50+25));
        }
    }
    let _ = canvas.draw_line((250, 800), (250, 0));
    let _ = canvas.draw_line((400, 800), (400, 0));
    let _ = canvas.draw_line((550, 800), (550, 0));
}

pub fn car_drawing(canvas: &mut Canvas<Window>,
                   car_list: &Vec<Car>,
                   car_a_left: &Texture,
                   car_a_right: &Texture,
                   car_a_up: &Texture,
                   car_a_down: &Texture,
                   car_b_left: &Texture,
                   car_b_right: &Texture,
                   car_b_up: &Texture,
                   car_b_down: &Texture,
                   car_c_left: &Texture,
                   car_c_right: &Texture,
                   car_c_up: &Texture,
                   car_c_down: &Texture) {

    for car in car_list {
        //test car, change width and height deending of the direction
        let mut width = 30;
        let mut height = 30;
        if car.direction == Direction::Left || car.direction == Direction::Right {
            width = 50;
        } else {
            height = 50;
        }
        let destination_rect = Rect::new(car.position[0], car.position[1], width, height); // x: 100, y: 100, width: 30, height: 50
        match car.direction {
            Direction::Left => {
                match car.car_type {
                    CarType::A => {
                        let _ = canvas.copy(car_a_left, None, Some(destination_rect));
                    },
                    CarType::B => {
                        let _ = canvas.copy(&car_b_left, None, Some(destination_rect));
                    },
                    CarType::C => {
                        let _ = canvas.copy(&car_c_left, None, Some(destination_rect));
                    },
                    _ => {},
                }
            },
            Direction::Right => {
                match car.car_type {
                    CarType::A => {
                        let _ = canvas.copy(&car_a_right, None, Some(destination_rect));
                    },
                    CarType::B => {
                        let _ = canvas.copy(&car_b_right, None, Some(destination_rect));
                    },
                    CarType::C => {
                        let _ = canvas.copy(&car_c_right, None, Some(destination_rect));
                    },
                    _ => {},
                }
            },
            Direction::Up => {
                match car.car_type {
                    CarType::A => {
                        let _ = canvas.copy(&car_a_up, None, Some(destination_rect));
                    },
                    CarType::B => {
                        let _ = canvas.copy(&car_b_up, None, Some(destination_rect));
                    },
                    CarType::C => {
                        let _ = canvas.copy(&car_c_up, None, Some(destination_rect));
                    },
                    _ => {},
                }
            },
            Direction::Down => {
                match car.car_type {
                    CarType::A => {
                        let _ = canvas.copy(&car_a_down, None, Some(destination_rect));
                    },
                    CarType::B => {
                        let _ = canvas.copy(&car_b_down, None, Some(destination_rect));
                    },
                    CarType::C => {
                        let _ = canvas.copy(&car_c_down, None, Some(destination_rect));
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }
}

pub fn draw_stats(canvas: &mut Canvas<Window>, ttf_context: &Sdl2TtfContext) {
    let texture_creator = canvas.texture_creator();
        let font = ttf_context.load_font(Path::new("src/assets/UbuntuMono[wght].ttf"), 128).unwrap();
        let surface = font
            .render("Hello, SDL2_ttf!")
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        let target = Rect::new(0, 0, 300, 50);
        let _ = canvas.copy(&texture, None, Some(target));
}