use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    NoDirection,
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

impl Default for Direction {
    fn default() -> Self { Direction::NoDirection }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CarType {
    A,
    B,
    C,
    NoCarType,
}

impl Default for CarType {
    fn default() -> Self { CarType::NoCarType }
}

impl CarType {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let car_type = [CarType::A, CarType::B, CarType::C];
        car_type.choose(&mut rng).unwrap().clone()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Car {
    pub direction: Direction,
    pub arrival_direction: Direction,
    pub car_type: CarType,
    pub position: [i32; 2],
    pub changed: bool,
    pub reserved: bool,
}

impl Car {
    pub fn move_car(&mut self, speed: u32, occupied_squares: &mut Vec<Vec<i32>>, index: usize, prob_limit: u8) -> u8 {
        let mut move_distance: i32 = 0;
        let mut prob = false;
        match self.direction {
            Direction::Up => {
                //give moving distance
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[1] - speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[1] - 2 - speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[1] - 4 - speed as i32;
                    },
                    _ => {},
                }
                //check if should turn
                if self.arrival_direction == Direction::Left {
                    if move_distance < 360 {
                        self.position[1] = 360;
                        self.position[0] = 400;
                        self.direction = Direction::Left;
                        self.changed = true;
                        return 0
                    } else if move_distance < 550 {
                        prob = true
                    }
                } else if self.arrival_direction == Direction::Right {
                    if move_distance < 510 {
                        self.position[1] = 510;
                        self.position[0] = 500;
                        self.direction = Direction::Right;
                        self.changed = true;
                        return 0
                    }
                }
                if !(self.arrival_direction == Direction::Left && prob_limit >= 3 && move_distance < 500) {
                    let mut risk = 3;
                    for square in &mut *occupied_squares {
                        if square[1] == move_distance / 50 && square[0] == self.position[0] / 50 && square[2] != index as i32 {
                            risk = 0;
                        } else if self.arrival_direction != Direction::Right && square[1] == (move_distance / 50) - 1 && square[0] == self.position[0] / 50 && risk >= 1 && square[2] != index as i32 {
                            risk = 1;
                        } else if self.arrival_direction != Direction::Right && square[1] == (move_distance / 50) - 2 && square[0] == self.position[0] / 50 && risk >= 2 && square[2] != index as i32 {
                            risk = 2;
                        }
                    }
                    match risk {
                        1 => self.position[1] -= 1,
                        2 => self.position[1] -= 2,
                        3 => self.position[1] = move_distance,
                        _ => {
                            if self.position[1] % 50 != 0 {
                                self.position[1] -= 1
                            }
                        },
                    }
                }
            },
            Direction::Down => {
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[1] + speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[1] + 2 + speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[1] + 4 + speed as i32;
                    },
                    _ => {},
                }
                //check if should turn
                if self.arrival_direction == Direction::Left {
                    if move_distance > 260 {
                        self.position[1] = 260;
                        self.position[0] = 250;
                        self.direction = Direction::Left;
                        self.changed = true;
                        return 0
                    }
                } else if self.arrival_direction == Direction::Right {
                    if move_distance > 410 {
                        self.position[1] = 410;
                        self.position[0] = 350;
                        self.direction = Direction::Right;
                        self.changed = true;
                        return 0
                    } else if move_distance > 200 {
                        prob = true
                    }
                }
                if !(self.arrival_direction == Direction::Right && prob_limit >= 3 && move_distance > 200) {
                    let mut risk = 3;
                    for square in &mut *occupied_squares {
                        if square[1] == (move_distance / 50) + 1 && square[0] == self.position[0] / 50 && square[2] != index as i32 {
                            risk = 0;
                        } else if self.arrival_direction != Direction::Left && square[1] == (move_distance / 50) + 2 && square[0] == self.position[0] / 50 && risk >= 1 && square[2] != index as i32 {
                            risk = 1;
                        } else if self.arrival_direction != Direction::Left && square[1] == (move_distance / 50) + 3 && square[0] == self.position[0] / 50 && risk >= 2 && square[2] != index as i32 {
                            risk = 2;
                        }
                    }
                    match risk {
                        1 => self.position[1] += 1,
                        2 => self.position[1] += 2,
                        3 => self.position[1] = move_distance,
                        _ => {
                            if self.position[1] % 50 != 0 {
                                self.position[1] += 1
                            }
                        },
                    }
                }
            },
            Direction::Left => {
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[0] - speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[0] - 2 - speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[0] - 4 - speed as i32;
                    },
                    _ => {},
                }
                if self.arrival_direction == Direction::Up {
                    if move_distance < 510 {
                        self.position[0] = 510;
                        self.position[1] = 250;
                        self.direction = Direction::Up;
                        self.changed = true;
                        return 0
                    }
                } else if self.arrival_direction == Direction::Down {
                    if move_distance < 360 {
                        self.position[0] = 360;
                        self.position[1] = 350;
                        self.direction = Direction::Down;
                        self.changed = true;
                        return 0
                    } else if move_distance < 550 {
                        prob = true
                    }
                }
                if !(self.arrival_direction == Direction::Down && prob_limit >= 3 && move_distance < 550) {
                    let mut risk = 3;
                    for square in &mut *occupied_squares {
                        if square[0] == (move_distance / 50) && square[1] == self.position[1] / 50 && square[2] != index as i32 {
                            risk = 0;
                        } else if self.arrival_direction != Direction::Up && square[0] == (move_distance / 50) - 1 && square[1] == self.position[1] / 50 && risk >= 1 && square[2] != index as i32 {
                            risk = 1;
                        } else if self.arrival_direction != Direction::Up && square[0] == (move_distance / 50) - 2 && square[1] == self.position[1] / 50 && risk >= 2 && square[2] != index as i32 {
                            risk = 2;
                        }
                    }
                    match risk {
                        1 => self.position[0] -= 1,
                        2 => self.position[0] -= 2,
                        3 => self.position[0] = move_distance,
                        _ => {
                            if self.position[0] % 50 != 0 {
                                self.position[0] -= 1
                            }
                        },
                    }
                }
            },
            Direction::Right => {
                match self.car_type {
                    CarType::A => {
                        move_distance = self.position[0] + speed as i32;
                    },
                    CarType::B => {
                        move_distance = self.position[0] + 2 + speed as i32;
                    },
                    CarType::C => {
                        move_distance = self.position[0] + 4 + speed as i32;
                    },
                    _ => {},
                }
                if self.arrival_direction == Direction::Up {
                    if move_distance > 410 {
                        self.position[0] = 410;
                        self.position[1] = 400;
                        self.direction = Direction::Up;
                        self.changed = true;
                        return 0
                    } else if move_distance > 200 {
                        prob = true
                    }
                } else if self.arrival_direction == Direction::Down {
                    if move_distance > 260 {
                        self.position[0] = 260;
                        self.position[1] = 500;
                        self.direction = Direction::Down;
                        self.changed = true;
                        return 0
                    }
                }
                if !(self.arrival_direction == Direction::Up && prob_limit >= 3 && move_distance > 200) {
                    let mut risk = 3;
                    for square in &mut *occupied_squares {
                        if square[0] == (move_distance / 50) + 1 && square[1] == self.position[1] / 50 && square[2] != index as i32 {
                            risk = 0;
                        } else if self.arrival_direction != Direction::Down && square[0] == (move_distance / 50) + 2 && square[1] == self.position[1] / 50 && risk >= 1 && square[2] != index as i32 {
                            risk = 1;
                        } else if self.arrival_direction != Direction::Down && square[0] == (move_distance / 50) + 3 && square[1] == self.position[1] / 50 && risk >= 2 && square[2] != index as i32 {
                            risk = 2;
                        }
                    }
                    match risk {
                        1 => self.position[0] += 1,
                        2 => self.position[0] += 2,
                        3 => self.position[0] = move_distance,
                        _ => {
                            if self.position[0] % 50 != 0 {
                                self.position[0] += 1
                            }
                        },
                    }
                }
            },
            _ => {}
        }
        match self.direction {
            Direction::Down | Direction::Up => {
                let mut new_positions = vec![vec![(self.position[0]-10)/50, self.position[1]/50, index as i32]];
                if self.position[1]%50 > 0 {
                    new_positions.push(vec![(self.position[0]-10)/50, self.position[1]/50, index as i32])
                }
                for item in new_positions {
                    if !occupied_squares.contains(&item) {
                        occupied_squares.push(item);
                    }
                }
            },
            Direction::Left | Direction::Right => {
                let mut new_positions = vec![vec![self.position[0], (self.position[1]-10)/50, index as i32]];
                if self.position[0]%50 > 0 {
                    new_positions.push(vec![self.position[0]/50, (self.position[1]-10)/50, index as i32])
                }
                for item in new_positions {
                    if !occupied_squares.contains(&item) {
                        occupied_squares.push(item);
                    }
                }
            }
            _ => {},
        }
        if prob {
            return 1
        }
        return 0
    }
}

pub fn create_car(mut direction: Direction, occupied_squares: &Vec<Vec<i32>>) -> Car {
    if direction == Direction::NoDirection {
        direction = Direction::random(Direction::NoDirection);
    }
    match direction {
        Direction::Left => {
            let arrival_direction = Direction::random(Direction::Left);
            let mut y = 0;
            match arrival_direction {
                Direction::Down => y = 8,
                Direction::Up => y = 6,
                Direction::Left => y = 7,
                _ => {},
            }
            for squares in occupied_squares {
                if (squares[0] == 15 || squares[0] == 14) && squares[1] == y-1 {
                    return Car::default()
                }
            }
            return Car{direction: Direction::Left, arrival_direction, car_type: CarType::random(), position: [15*50, (y-1)*50 + 10], changed: false, reserved: false}
        },
        Direction::Right => {
            let arrival_direction = Direction::random(Direction::Right);
            let mut y = 0;
            match arrival_direction {
                Direction::Down => y = 11,
                Direction::Up => y = 9,
                Direction::Right => y = 10,
                _ => {},
            }
            for squares in occupied_squares {
                if (squares[0] == 0 || squares[0] == 1) && squares[1] == y-1 {
                    return Car::default()
                }
            }
            return Car{direction: Direction::Right, arrival_direction, car_type: CarType::random(), position: [0, (y-1)*50 + 10], changed: false, reserved: false}
        },
        Direction::Up => {
            let arrival_direction = Direction::random(Direction::Up);
            let mut x = 0;
            match arrival_direction {
                Direction::Left => x = 9,
                Direction::Up => x = 10,
                Direction::Right => x = 11,
                _ => {},
            }
            for squares in occupied_squares {
                if squares[0] == x-1 && (squares[1] == 15 || squares[1] == 14) {
                    return Car::default()
                }
            }
            return Car{direction: Direction::Up, arrival_direction, car_type: CarType::random(), position: [(x-1)*50 + 10, 15*50], changed: false, reserved: false}
        },
        Direction::Down => {
            let arrival_direction = Direction::random(Direction::Down);
            let mut x = 0;
            match arrival_direction {
                Direction::Left => x = 6,
                Direction::Down => x = 7,
                Direction::Right => x = 8,
                _ => {},
            }
            for squares in occupied_squares {
                if squares[0] == x-1 && (squares[1] == 0 || squares[1] == 1) {
                    return Car::default()
                }
            }
            return Car{direction: Direction::Down, arrival_direction, car_type: CarType::random(), position: [(x-1)*50 + 10, 0], changed: false, reserved: false}
        },
        _ => {},
    }
    return Car::default()
}

pub fn check_collision(carlist: &Vec<Car>) -> Vec<Vec<i32>> {
    let mut occupied_squares: Vec<Vec<i32>> = Vec::new(); 
    for (index, car) in carlist.iter().enumerate() {
        occupied_squares.push(vec![car.position[0]/50, car.position[1]/50, index as i32]);
        match car.direction {
            Direction::Down | Direction::Up => {
                if car.position[1] % 50 > 0 {
                    occupied_squares.push(vec![car.position[0]/50, (car.position[1]/50)+1, index as i32]);
                }
            },
            Direction::Left | Direction::Right => {
                if car.position[0] % 50 > 0 {
                    occupied_squares.push(vec![(car.position[0]/50)+1, car.position[1]/50, index as i32]);
                }
            },
            _ => {},
        }
    }
    return occupied_squares
}

pub fn extra_reserved_squares(car: &Car, occupied_squares: &Vec<Vec<i32>>, index: usize) -> Vec<Vec<i32>> {
    let mut extra_reserved: Vec<Vec<i32>> = Vec::new();
    if !car.changed {
        match car.direction {
            Direction::Down => {
                if car.arrival_direction == car.direction && (car.position[1]/50)+1 >= 7 && (car.position[1]/50)+1 <= 10 {
                    //reserve the squares in front of the car
                    let mut position_correction = 0;
                    if car.position[1]%50 > 0 {
                        position_correction = 1
                    }
                    for i in (car.position[1]/50)+position_correction..10 {
                        let mut already_reserved = false;
                        for square in occupied_squares {
                            if square[1] == i && square[0] == (car.position[0]-10)/50 && square[2] != index as i32 {
                                already_reserved = true;
                                break
                            }
                        }
                        if already_reserved {
                            break
                        } else {
                            extra_reserved.push(vec![car.position[0]/50, i, index as i32]);
                        }
                    }
                }
            },
            Direction::Up => {
                if car.arrival_direction == car.direction && (car.position[1]/50)+1 >= 7 && (car.position[1]/50) <= 10 {
                    let mut position_correction = 0;
                    if car.position[1]%50 > 0 {
                        position_correction = 1
                    }
                    for i in (6..(car.position[1]/50)-position_correction).rev() {
                        let mut already_reserved = false;
                        for square in occupied_squares {
                            //println!("{:?}, {}, {}, {}", square, i, (car.position[0]-10)/50, index);
                            if square[1] == i && square[0] == (car.position[0]-10)/50 && square[2] != index as i32 {
                                already_reserved = true;
                                break
                            }
                        }
                        if already_reserved {
                            break
                        } else {
                            extra_reserved.push(vec![car.position[0]/50, i, index as i32]);
                        }
                    }
                }
            },
            Direction::Left => {
                if car.arrival_direction == car.direction && (car.position[0]/50)+1 >= 7 && (car.position[0]/50)+1 <= 10 {
                    let mut position_correction = 0;
                    if car.position[1]%50 > 0 {
                        position_correction = 1
                    }
                    for i in (6..(car.position[0]/50)-position_correction+1).rev() {
                        let mut already_reserved = false;
                        for square in occupied_squares {
                            if square[0] == i && square[1] == (car.position[1]-10)/50 && square[2] != index as i32 {
                                already_reserved = true;
                                break
                            }
                            if already_reserved {
                                break
                            }
                        }
                        if already_reserved {
                            break
                        } else {
                            extra_reserved.push(vec![i, car.position[0]/50, index as i32]);
                        }
                    }
                }
            },
            Direction::Right => {
                if car.arrival_direction == car.direction && (car.position[0]/50)+1 >= 7 && (car.position[0]/50)+1 <= 10 {
                    let mut position_correction = 0;
                    if car.position[1]%50 > 0 {
                        position_correction = 1
                    }
                    for i in (car.position[0]/50)-position_correction..11 {
                        let mut already_reserved = false;
                        for square in occupied_squares {
                            if square[0] == i && square[1] == (car.position[1]-10)/50 && square[2] != index as i32 {
                                already_reserved = true;
                                break
                            }
                            if already_reserved {
                                break
                            }
                        }
                        if already_reserved {
                            break
                        } else {
                            extra_reserved.push(vec![i, car.position[0]/50, index as i32]);
                        }
                    }
                }
            },
            _ => {},
        }
    }
    return extra_reserved
}

pub fn remove_car(car_list: &mut Vec<Car>) {
    car_list.retain(|car| {
        !(car.position[0] >= 900 || car.position[0] < -100 || car.position[1] >= 900 || car.position[1] < -100)
    });
}