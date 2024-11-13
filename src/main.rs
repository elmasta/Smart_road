mod logic;
mod draw;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use std::time::Duration;
use std::path::Path;

const CAR_SPEED: u32 = 5;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("SDL2 Intersection", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Initialisation de SDL2_image
    image::init(InitFlag::PNG | InitFlag::JPG)?;
    let texture_creator = canvas.texture_creator();
    let car_a_left = texture_creator.load_texture(Path::new("src/assets/car_a_1.png"))?;
    let car_a_up = texture_creator.load_texture(Path::new("src/assets/car_a_2.png"))?;
    let car_a_right = texture_creator.load_texture(Path::new("src/assets/car_a_3.png"))?;
    let car_a_down = texture_creator.load_texture(Path::new("src/assets/car_a_4.png"))?;
    let car_b_left = texture_creator.load_texture(Path::new("src/assets/car_b_1.png"))?;
    let car_b_up = texture_creator.load_texture(Path::new("src/assets/car_b_2.png"))?;
    let car_b_right = texture_creator.load_texture(Path::new("src/assets/car_b_3.png"))?;
    let car_b_down = texture_creator.load_texture(Path::new("src/assets/car_b_4.png"))?;
    let car_c_left = texture_creator.load_texture(Path::new("src/assets/car_c_1.png"))?;
    let car_c_up = texture_creator.load_texture(Path::new("src/assets/car_c_2.png"))?;
    let car_c_right = texture_creator.load_texture(Path::new("src/assets/car_c_3.png"))?;
    let car_c_down = texture_creator.load_texture(Path::new("src/assets/car_c_4.png"))?;

    let mut event_pump = sdl_context.event_pump()?;

    //car list
    let mut car_list: Vec<logic::Car> = vec![];
    let mut spawnrate = 0;

    'running: loop {

        //retrieve the occupied squares
        //println!("{:?}", car_list);
        let mut occupied_squares: Vec<Vec<i32>> = logic::check_collision(&car_list);
        // let mut seen_prefixes = HashSet::new();
        // let mut result = Vec::new();
        // println!("{:?}", occupied_squares);
        // for vec in occupied_squares {
        //     let prefix = (vec[0], vec[1]);
            
        //     if !seen_prefixes.contains(&prefix) {
        //         seen_prefixes.insert(prefix);
        //         result.push(vec);
        //     }
        // }
        //occupied_squares = result;

        for (index, car) in car_list.iter().enumerate() {
            println!("{:?}", occupied_squares);
            let extra_occupied_squares = logic::extra_reserved_squares(&car, &occupied_squares, index);
            println!("{:?}", extra_occupied_squares);
            for item in extra_occupied_squares {
                if !occupied_squares.contains(&item) {
                    occupied_squares.push(item);
                }
            }
            println!("------------------------");
        }
        println!("result = {:?}", occupied_squares);

        let mut car: logic::Car = logic::Car::default();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if spawnrate == 0 {
                        car = logic::create_car(logic::Direction::Left, &occupied_squares);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if spawnrate == 0 {
                        car = logic::create_car(logic::Direction::Right, &occupied_squares);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if spawnrate == 0 {
                        car = logic::create_car(logic::Direction::Up, &occupied_squares);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if spawnrate == 0 {
                        car = logic::create_car(logic::Direction::Down, &occupied_squares);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if spawnrate == 0 {
                        car = logic::create_car(logic::Direction::NoDirection, &occupied_squares);
                    }
                },
                _ => {}
            }
        }
        if !(car == logic::Car::default()) {
            car_list.push(car);
            spawnrate = 10;
        }
        
        println!("{:?}", car_list);
        println!("----------------------------");

        // Draw the road
        draw::road_drawing(&mut canvas);

        let mut prob_limit = 0;
        for (index, car) in car_list.iter_mut().enumerate() {
            prob_limit += car.move_car(CAR_SPEED, &mut occupied_squares, index, prob_limit);
        }

        draw::car_drawing(&mut canvas, 
                          &car_list, 
                          &car_a_left,
                          &car_a_right,
                          &car_a_up,
                          &car_a_down,
                          &car_b_left,
                          &car_b_right,
                          &car_b_up,
                          &car_b_down,
                          &car_c_left,
                          &car_c_right,
                          &car_c_up,
                          &car_c_down);
                          
        draw::draw_stats(&mut canvas, &ttf_context);

        // Present the updated canvas
        canvas.present();

        // Remove cars that are out
        logic::remove_car(&mut car_list);

        if spawnrate > 0 {
            spawnrate -= 1;
        }

        // Delay to control the frame rate
        ::std::thread::sleep(Duration::from_millis(16));  // ~60 FPS
    }

    Ok(())
}