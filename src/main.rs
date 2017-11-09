extern crate piston_window;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use piston_window::*;


struct World {
    ants: Vec<Ant>,
    grid: Grid
}

impl World {
    fn new(width: usize, height: usize) -> World {
        return World {
            ants: Vec::new(),
            grid: Grid::new(width, height)
        }
    }

    fn update(&mut self) {
        for ant in self.ants.iter_mut() {
            ant.update(&mut self.grid)
        }
    }

    fn add_ant(&mut self, ant: Ant) {
        self.ants.push(ant)
    }
}

struct Grid {
    size: Vector,
    tiles: Vec<[f32; 4]>
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        return Grid {
            tiles: vec![[1.0, 1.0, 1.0, 1.0]; width * height],
            size: Vector {
                x: width,
                y: height
            }
        }
    }

    fn set(&mut self, position: &mut iVector, value: [f32; 4]) {
        self.bound_position(position);

        self.tiles[self.size.x * position.x as usize + position.y as usize] = value;
    }

    fn get(&self, position: &mut iVector) -> [f32; 4] {
        self.bound_position(position);

        return self.tiles[self.size.x * position.x as usize + position.y as usize];
    }

    fn bound_position(&self, position: &mut iVector) {
        if position.x < 0 {
            position.x += self.size.x as isize;
        }   

        if position.y < 0 {
            position.y += self.size.y as isize;
        }

        if position.x >= self.size.x as isize {
            position.x -= self.size.x as isize;
        }

        if position.y >= self.size.y as isize {
            position.y -= self.size.y as isize;
        }
    }
}

struct iVector {
    x: isize,
    y: isize,
}

impl iVector {
    fn add(&mut self, array: [isize; 2]) {
        self.x += array[0];
        self.y += array[1];
    }
}

struct Vector {
    x: usize,
    y: usize
}

struct Ant {
    color: [f32; 4],
    position: iVector,
    direction: usize
}

impl Ant {
    fn new() -> Ant {
        let mut rng = rand::thread_rng();
        let range = Range::new(0.0, 1.0);

        return Ant {
            color: [range.ind_sample(&mut rng) as f32, range.ind_sample(&mut rng) as f32, range.ind_sample(&mut rng) as f32, 1.0],
            position: iVector {
                x: 50,
                y: 50
            },
            direction: 3
        }
    }

    fn update(&mut self, world: &mut Grid) {
        static DIRECTIONS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
        let color = world.get(&mut self.position);

        if color[0] == 1.0 && color[1] == 1.0 && color[2] == 1.0 {
            self.direction += 1;
            self.direction %= 4;
            world.set(&mut self.position, self.color)
        } else {
            self.direction += 3;
            self.direction %= 4;
            world.set(&mut self.position, [1.0; 4])
        }

        self.position.add(DIRECTIONS[self.direction]);
    }
}

fn main() {
    let mut world = World::new(100, 100);
    world.add_ant(Ant::new());
    world.add_ant(Ant::new());
    let size = 5.0;
    let mut window: PistonWindow = WindowSettings::new("Piston Ant", [640, 480]).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            world.update();
            world.update();
            world.update();
            world.update();

            clear([1.0; 4], graphics);
            
            for (index, tile) in world.grid.tiles.iter().enumerate() {
                let x = index / world.grid.size.x;
                let y = index % world.grid.size.x;
                rectangle(
                    *tile,
                    [size * (x as f64), size * (y as f64), size, size],
                    context.transform,
                    graphics
                );
            }
        });
    }
}