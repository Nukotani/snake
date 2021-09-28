use std::collections::HashMap;
use std::time::Duration;

extern crate rand;
use rand::Rng;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Clone)]
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeRect {
    direction: Direction,
    rect: Rect,
}

impl SnakeRect {
    fn new(direction: Direction, x: i32, y: i32) -> SnakeRect {
        SnakeRect {
            direction, 
            rect: Rect::new(x, y, 10, 10),
        }
    }

    fn slide(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.y() <= 0 {
                    self.rect.set_y(290);
                } else {
                    self.rect.set_y(self.y() - 10);
                }
            },
            Direction::Down => self.rect.set_y((self.y() + 10) % 300),
            Direction::Left => {
                if self.x() <= 0 {
                    self.rect.set_x(590);
                } else {
                    self.rect.set_x(self.x() - 10);
                }
            },

            Direction::Right => self.rect.set_x((self.x() + 10) % 600),
        };
    }

    fn x(&self) -> i32 {
        self.rect.x()
    }
    fn y(&self) -> i32 {
        self.rect.y()
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    fn get_direction(&self) -> Direction {
        self.direction.clone()
    }
}

struct Food {
    x: i32,
    y: i32,
}

impl Food {
    fn new() -> Food {
        Food {
            x: rand::thread_rng().gen_range(0..60) * 10,
            y: rand::thread_rng().gen_range(0..30) * 10,
        }
    }
}

struct Snake {
    head: SnakeRect,
    body: Option<Vec<SnakeRect>>,
    food: Food,
    length: usize,
    is_moving: bool,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            head: SnakeRect::new(Direction::Right, 100, 100),
            body: None,
            food: Food::new(),
            length: 1,
            is_moving: false,
        }
    }
    
    fn slide(&mut self) {
        let mut direction = self.get_direction();
        match &mut self.body {
            Some(vec) => {
                for i in vec.iter_mut() {
                    i.slide();

                    if i.get_direction() != direction {
                        let temp = i.get_direction();
                        i.set_direction(direction);
                        direction = temp;
                    }
                }
            },
            _ => {},
        };

        self.head.slide();
        self.check_collision();
    }

    fn grow(&mut self) {
        self.length += 1;

        match &mut self.body {
            Some(vec) => {
                let old_tail = vec.get(vec.len() - 1).unwrap();
                let old_tail_direction = old_tail.get_direction();
                let (x, y) = match old_tail_direction { 
                    Direction::Up => (old_tail.x(), old_tail.y() + 10),
                    Direction::Down => (old_tail.x(), old_tail.y() - 10),
                    Direction::Left => (old_tail.x() + 10, old_tail.y()),
                    Direction::Right => (old_tail.x() - 10, old_tail.y()),
                };
                vec.push(SnakeRect::new(old_tail_direction, x, y));
            },
            _ => {
                let (x, y) = match self.head.direction {
                    Direction::Up => (self.head.x(), self.head.y() + 10),
                    Direction::Down => (self.head.x(), self.head.y() - 10),
                    Direction::Left => (self.head.x() + 10, self.head.y()),
                    Direction::Right => (self.head.x() - 10, self.head.y()),
                }; 
                let mut vec: Vec<SnakeRect> = Vec::new();
                vec.push(SnakeRect::new(self.get_direction(), x, y));
                self.body = Some(vec);
            },
        };
    }
    
    fn render(&self) -> Vec<Rect> {
        let mut renderer: Vec<Rect> = vec![self.head.rect.clone()];

        match &self.body {
            Some(vect) => {
                for i in vect.iter() {
                    renderer.push(i.rect.clone());
                }
            },
            _ => {},
        };
        renderer.push(Rect::new(self.food.x, self.food.y, 10, 10));

        renderer
    }

    fn check_collision(&mut self) {
        match &self.body {
            Some(vec) => {
        //TODO: use iterator's methods
                for i in vec.iter() {
                    if self.head.x() == i.x() && self.head.y() == i.y() {
                        self.clear();
                        break;
                    }
                }
            },
            _ => {},
        };
        if self.head.x() == self.food.x && self.head.y() == self.food.y {
            self.food = Food::new();
            self.grow();
        }
    }

    fn clear(&mut self) {
        *self = Snake::new();
    }

    fn get_direction(&self) -> Direction {
        self.head.direction.clone()
    }
    fn set_direction(&mut self, direction: Direction) {
        self.head.direction = direction;
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("snake", 600, 300)
        .position_centered()
        .build()
        .unwrap();

    let mut keybinds: HashMap<Keycode, Box<dyn Fn(&mut Snake)>>  = HashMap::new();

    keybinds.insert(Keycode::Up, Box::new(|snake: &mut Snake| {
            if snake.get_direction() == Direction::Down {
                return;
            }
            snake.is_moving = true;
            snake.set_direction(Direction::Up);
        })
    );
    keybinds.insert(Keycode::Down, Box::new(|snake: &mut Snake| {
            if snake.get_direction() == Direction::Up {
                return;
            }
            snake.is_moving = true;
            snake.set_direction(Direction::Down);
        })
    );
    keybinds.insert(Keycode::Left, Box::new(|snake: &mut Snake| {
            if snake.get_direction() == Direction::Right {
                return;
            }
            snake.is_moving = true;
            snake.set_direction(Direction::Left);
        })
    );
    keybinds.insert(Keycode::Right, Box::new(|snake: &mut Snake| {
            if snake.get_direction() == Direction::Left {
                return;
            }
            snake.is_moving = true;
            snake.set_direction(Direction::Right);
        })
    );
    keybinds.insert(Keycode::Space, Box::new(|snake: &mut Snake| {
            if snake.is_moving {
                snake.is_moving = false;
            } else {
                snake.is_moving = true;
            }
        })
    );

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut snake = Snake::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        if snake.is_moving {
            snake.slide();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rects(&snake.render()).unwrap();

        canvas.present();

        match event_pump.poll_event() {
            Some(event) => {
                match event {
                    Event::Quit {..} => break 'running,
                    Event::KeyDown { keycode: Some(key), ..} => {
                        match keybinds.get(&key) {
                            Some(closure) => closure(&mut snake),
                            _ => {},
                        };
                    },
                    _ => {},
                };
            },
            _ => {},
        };

        std::thread::sleep(Duration::new(0, 100_000_000u32));
    }
}
