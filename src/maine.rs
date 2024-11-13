use rand::Rng;
use rand::seq::SliceRandom;

extern crate sdl2;

use sdl2::libc::close;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use std::time::Duration;
use std::path::Path;

const CAR_SPEED: i32 = 5;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    NoDirection,
}

#[derive(Debug, Clone, PartialEq)]
enum CarType {
    A,
    B,
    C,
}

impl CarType {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let car_type = [CarType::A, CarType::B, CarType::C];
        car_type.choose(&mut rng).unwrap().clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Car {
    direction: Direction,
    arrival_direction: Direction,
    car_type: CarType,
    position: [i32; 2],
    changed: bool,
}

impl Direction {
    fn random(to_exclude: Direction) -> Self {
        let mut rng = rand::thread_rng();
        match to_exclude {
            Direction::Left => {
                let direction = [Direction::Up, Direction::Down, Direction::Left];
                return direction.choose(&mut rng).unwrap().clone();
            },
            Direction::Up => {
                let direction = [Direction::Up, Direction::Left, Direction::Right];
                return direction.choose(&mut rng).unwrap().clone();
            },
            Direction::Down => {
                let direction = [Direction::Down, Direction::Left, Direction::Right];
                return direction.choose(&mut rng).unwrap().clone();
            },
            Direction::Right => {
                let direction = [Direction::Up, Direction::Down, Direction::Right];
                return direction.choose(&mut rng).unwrap().clone();
            },
            Direction::NoDirection => {
                let direction = [Direction::Up, Direction::Down, Direction::Right, Direction::Left];
                return direction.choose(&mut rng).unwrap().clone();
            },
        }
    }
}

impl Car {
    fn move_car(&mut self, speed: u32/*, cross: Vec<Vec<bool>>*/) -> bool {
        let mut move_distance: i32 = 0;
        match self.direction {
            Direction::Up => {
                //give moving distance
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[1] - 1 * speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[1] - 2 * speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[1] - 3 * speed as i32;
                    },
                }
                //check if should turn
                if self.arrival_direction == Direction::Left {
                    if move_distance < 360 {
                        self.position[1] = 360;
                        self.direction = Direction::Left;
                        self.changed = true
                    } else {
                        self.position[1] = move_distance
                    }
                } else if self.arrival_direction == Direction::Right {
                    if move_distance < 510 {
                        self.position[1] = 510;
                        self.direction = Direction::Right;
                        self.changed = true
                    } else {
                        self.position[1] = move_distance
                    }
                } else {
                    self.position[1] = move_distance
                }
            },
            Direction::Down => {
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[1] + 1 * speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[1] + 2 * speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[1] + 3 * speed as i32;
                    },
                }
                //check if should turn
                if self.arrival_direction == Direction::Left {
                    if move_distance > 260 {
                        self.position[1] = 260;
                        self.direction = Direction::Left;
                        self.changed = true
                    } else {
                        self.position[1] = move_distance
                    }
                } else if self.arrival_direction == Direction::Right {
                    if move_distance > 410 {
                        self.position[1] = 410;
                        self.direction = Direction::Right;
                        self.changed = true
                    } else {
                        self.position[1] = move_distance
                    }
                } else {
                    self.position[1] = move_distance
                }
            },
            Direction::Left => {
                match self.car_type {
                    //check if reserved square and change direction
                    CarType::A => {
                        move_distance = self.position[0] - 1 * speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[0] - 2 * speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[0] - 3 * speed as i32;
                    },
                }
                self.position[0] = move_distance
            },
            Direction::Right => {
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[0] + 1 * speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[0] + 2 * speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[0] + 3 * speed as i32;
                    },
                }
                self.position[0] = move_distance
            },
            _ => {}
        }
        //check if on intersection
        self.position[1] >= 201 && self.position[1] <= 549 && self.position[0] >= 201 && self.position[0] <= 549
    }

    fn colision_detection(&self, car_list: Vec<Car>) -> u32 {
        let mut closest: i32 = 1000;
        for car in car_list {
            if *self != car {
                match car.direction {
                    Direction::Up => {
                        if self.position[0] == car.position[0] && self.position[1] > car.position[1] {
                           
                        }
                    },
                    Direction::Down => {
                        if self.position[0] == car.position[0] && self.position[1] < car.position[1] {
                            if closest > car.position[1] - self.position[1] {
                                closest = car.position[1] - self.position[1];
                            }
                        }
                    },
                    Direction::Left => {
                        if self.position[1] == car.position[1] && self.position[0] > car.position[0] {
                            return 1;
                        }
                    },
                    Direction::Right => {
                        if self.position[1] == car.position[1] && self.position[0] < car.position[0] {
                            if closest > car.position[0] - self.position[0] {
                                closest = car.position[0] - self.position[0];
                            }
                        }
                    },
                    _ => {return 0;},
                }
            }
        }
        if closest <= 50 {
            return 0;
        } else if closest <= 75 {
            return 1;
        } else if closest <= 100 {
            return 2;
        } else {
            return 3;
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL2 Intersection", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    //init cross road
    //car list
    let mut car_list: Vec<Car> = vec![];
    //trafic light
    //let mut direction_light: Direction = Direction::Left;
    //if car on intersection
    //let mut intersection: bool = false;

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
    // let mut reserved_squares: Vec<Vec<bool>> = vec![
    //     vec![false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false]
    // ];
    let mut direction_light: Direction = Direction::NoDirection;
    //if car on intersection
    let mut intersection: bool = false;

    'running: loop {
        println!("new loop");
        for event in event_pump.poll_iter() {
            let mut too_close: bool = false;
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let arrival_direction = Direction::random(Direction::Left);
                    match arrival_direction {
                        Direction::Left => {
                            for car in &car_list {
                                if car.position[0] >= 700 && car.position[1] == 310 {
                                    too_close = true;
                                    break;
                                }
                            }
                            if !too_close {
                                car_list.push(Car{direction: Direction::Left, arrival_direction, car_type: CarType::random(), position: [800, 310], changed: false});
                            }
                        },
                        Direction::Up => {
                            for car in &car_list {
                                if car.position[0] >= 700 && car.position[1] == 260 {
                                    too_close = true;
                                    break;
                                }
                            }
                            if !too_close {
                                car_list.push(Car{direction: Direction::Left, arrival_direction, car_type: CarType::random(), position: [800, 260], changed: false});
                            }
                        },
                        Direction::Down => {
                            for car in &car_list {
                                if car.position[0] >= 700 && car.position[1] == 360 {
                                    too_close = true;
                                    break;
                                }
                            }
                            if !too_close {
                                car_list.push(Car{direction: Direction::Left, arrival_direction, car_type: CarType::random(), position: [800, 360], changed: false});
                            }
                        }
                        _ => println!("Error")
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    //if !too_close {
                        let arrival_direction = Direction::random(Direction::Right);
                        match arrival_direction {
                            Direction::Right => {
                                for car in &car_list {
                                    if car.position[0] <= 50 && car.position[1] == 460 {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if !too_close {
                                    car_list.push(Car{direction: Direction::Right, arrival_direction, car_type: CarType::random(), position: [-50, 460], changed: false});
                                }
                            },
                            Direction::Up => {
                                for car in &car_list {
                                    if car.position[0] <= 50 && car.position[1] == 410 {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if !too_close {
                                    car_list.push(Car{direction: Direction::Right, arrival_direction, car_type: CarType::random(), position: [-50, 410], changed: false});
                                }
                            },
                            Direction::Down => {
                                for car in &car_list {
                                    if car.position[0] <= 50 && car.position[1] == 510 {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if !too_close {
                                    car_list.push(Car{direction: Direction::Right, arrival_direction, car_type: CarType::random(), position: [-50, 510], changed: false});
                                }
                            }
                            _ => println!("Error")
                        }
                    //}
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if !too_close {
                        let arrival_direction = Direction::random(Direction::Up);
                        match arrival_direction {
                            Direction::Right => {
                                car_list.push(Car{direction: Direction::Up, arrival_direction, car_type: CarType::random(), position: [510, 800], changed: false});
                            },
                            Direction::Up => {
                                car_list.push(Car{direction: Direction::Up, arrival_direction, car_type: CarType::random(), position: [460, 800], changed: false});
                            },
                            Direction::Left => {
                                car_list.push(Car{direction: Direction::Up, arrival_direction, car_type: CarType::random(), position: [410, 800], changed: false});
                            }
                            _ => println!("Error")
                        }
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if !too_close {
                        let arrival_direction = Direction::random(Direction::Down);
                        match arrival_direction {
                            Direction::Right => {
                                for car in &car_list {
                                    if car.position[1] <= 50 && car.position[0] == 360 {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if !too_close {
                                    car_list.push(Car{direction: Direction::Down, arrival_direction, car_type: CarType::random(), position: [360, -50], changed: false});
                                }
                            },
                            Direction::Down => {
                                for car in &car_list {
                                    if car.position[1] <= 50 && car.position[0] == 310 {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if !too_close {
                                    car_list.push(Car{direction: Direction::Down, arrival_direction, car_type: CarType::random(), position: [310, -50], changed: false});
                                }
                            },
                            Direction::Left => {
                                for car in &car_list {
                                    if car.position[1] <= 50 && car.position[0] == 260 {
                                        too_close = true;
                                        break;
                                    }
                                }
                                if !too_close {
                                    car_list.push(Car{direction: Direction::Down, arrival_direction, car_type: CarType::random(), position: [260, -50], changed: false});
                                }
                            }
                            _ => println!("Error")
                        }
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    let starting_position = Direction::random(Direction::NoDirection);
                    let mut random_car = Car{direction: starting_position, arrival_direction: Direction::random(starting_position.clone()), car_type: CarType::random(), position: [0, 0], changed: false};
                    match random_car.direction {
                        Direction::Up => {
                            match random_car.arrival_direction {
                                Direction::Up => {
                                    random_car.position = [460, 800];
                                },
                                Direction::Left => {
                                    random_car.position = [410, 800];
                                },
                                Direction::Right => {
                                    random_car.position = [510, 800];
                                },
                                _ => println!("Error")
                            }
                        },
                        Direction::Down => {
                            match random_car.arrival_direction {
                                Direction::Down => {
                                    random_car.position = [310, 800];
                                },
                                Direction::Left => {
                                    random_car.position = [260, 800];
                                },
                                Direction::Right => {
                                    random_car.position = [360, 800];
                                },
                                _ => println!("Error")
                            }
                        },
                        Direction::Left => {
                            match random_car.arrival_direction {
                                Direction::Down => {
                                    random_car.position = [800, 360];
                                },
                                Direction::Left => {
                                    random_car.position = [800, 310];
                                },
                                Direction::Up => {
                                    random_car.position = [800, 260];
                                },
                                _ => println!("Error")
                            }
                        },
                        Direction::Right => {
                            match random_car.arrival_direction {
                                Direction::Down => {
                                    random_car.position = [-50, 510];
                                },
                                Direction::Right => {
                                    random_car.position = [-50, 460];
                                },
                                Direction::Up => {
                                    random_car.position = [-50, 410];
                                },
                                _ => println!("Error")
                            }
                        },
                        _ => println!("Error")
                    }
                    // for car in &car_list {
                    //     if random_car.direction == Direction::Left && car.direction == Direction::Left && car.position[0] >= 650 {
                    //         //println!("1");
                    //         too_close = true;
                    //         break;
                    //     } else if random_car.direction == Direction::Right && car.direction == Direction::Right && car.position[0] <= 100 {
                    //         //println!("2");
                    //         too_close = true;
                    //         break;
                    //     } else if random_car.direction == Direction::Up && car.direction == Direction::Up && car.position[1] >= 650 {
                    //         //println!("3");
                    //         too_close = true;
                    //         break;
                    //     } else if random_car.direction == Direction::Down && car.direction == Direction::Down && car.position[1] <= 100 {
                    //         //println!("4");
                    //         too_close = true;
                    //         break;
                    //     } else {
                    //         //println!("5");
                    //     }
                    // }
                    if !too_close {
                        car_list.push(random_car);
                    }
                },
                _ => {}
            }
        }

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

        let mut indices_to_remove = Vec::new();
        for (index, car) in car_list.iter_mut().enumerate() {
            //println!("{}, {}", car.position[0], car.position[1]);
            if car.position[0] >= 900 || car.position[0] < -100 || car.position[1] >= 900 || car.position[1] < -100 {
                indices_to_remove.push(index);
            }
        }
        let mut index_rectifier = 0;
        for &index in indices_to_remove.iter() {
            car_list.remove(index - index_rectifier);
            index_rectifier += 1;
        }

        let mut closest_car: i32 = 0;
        'checklight: for (index, car) in car_list.iter_mut().enumerate() {
            if car.position[0] >= 900 || car.position[0] < -100 || car.position[1] >= 900 || car.position[1] < -100 {
                indices_to_remove.push(index);
            } else if !intersection {
                match car.direction {
                    Direction::Up => {
                        if (car.position[1] - 750).abs() >= closest_car && car.position[1] >= 550 {
                            closest_car = (car.position[1] - 800).abs();
                            direction_light = Direction::Up;
                        }
                    },
                    Direction::Right => {
                        if car.position[0] >= closest_car && car.position[0] <= 200 {
                            closest_car = car.position[0];
                            direction_light = Direction::Right;
                        }
                    },
                    Direction::Down => {
                        if car.position[1] >= closest_car && car.position[1] <= 200 {
                            closest_car = car.position[1];
                            direction_light = Direction::Down;
                        }
                    },
                    Direction::Left => {
                        if (car.position[0] - 750).abs() >= closest_car && car.position[0] >= 550 {
                            closest_car = (car.position[0] - 800).abs();
                            direction_light = Direction::Left;
                        }
                    },
                    _ => {println!("Empty");},
                }
            }
            if closest_car == 350 {
                break 'checklight;
            }
        }

        intersection = false;
        let clone_car_list = car_list.clone();
        for (_index, car) in car_list.iter_mut().enumerate() {
            if car.move_car(car.colision_detection(clone_car_list.clone())/*, reserved_squares*/) {
                intersection = true;
            }
            // if car.direction == direction_light || direction_light == Direction::NoDirection {
            //     intersection = car.move_car(car.colision_detection(clone_car_list.clone())/*, reserved_squares*/);
            // }
            
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
                            canvas.copy(&car_a_left, None, Some(destination_rect))?;
                        },
                        CarType::B => {
                            canvas.copy(&car_b_left, None, Some(destination_rect))?;
                        },
                        CarType::C => {
                            canvas.copy(&car_c_left, None, Some(destination_rect))?;
                        },
                    }
                },
                Direction::Right => {
                    match car.car_type {
                        CarType::A => {
                            canvas.copy(&car_a_right, None, Some(destination_rect))?;
                        },
                        CarType::B => {
                            canvas.copy(&car_b_right, None, Some(destination_rect))?;
                        },
                        CarType::C => {
                            canvas.copy(&car_c_right, None, Some(destination_rect))?;
                        },
                    }
                },
                Direction::Up => {
                    match car.car_type {
                        CarType::A => {
                            canvas.copy(&car_a_up, None, Some(destination_rect))?;
                        },
                        CarType::B => {
                            canvas.copy(&car_b_up, None, Some(destination_rect))?;
                        },
                        CarType::C => {
                            canvas.copy(&car_c_up, None, Some(destination_rect))?;
                        },
                    }
                },
                Direction::Down => {
                    match car.car_type {
                        CarType::A => {
                            canvas.copy(&car_a_down, None, Some(destination_rect))?;
                        },
                        CarType::B => {
                            canvas.copy(&car_b_down, None, Some(destination_rect))?;
                        },
                        CarType::C => {
                            canvas.copy(&car_c_down, None, Some(destination_rect))?;
                        },
                    }
                },
                _ => {}
            }
        }

        // Present the updated canvas
        canvas.present();

        // Delay to control the frame rate
        ::std::thread::sleep(Duration::from_millis(16));  // ~60 FPS
    }

    Ok(())
}
