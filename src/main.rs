extern crate piston_window;
extern crate rand;
use piston_window::*;


struct World {
    size: Vector,
    tiles: Vec<[f32; 4]>,
    ants: Vec<Ant>
}

impl World {
    fn new(width: usize, height: usize) -> World {
        return World {
            tiles: vec![[0.0, 0.0, 0.0, 1.0]; width * height],
            size: Vector {
                x: width,
                y: height
            },
            ants: Vec::new()
        }
    }

    fn set(&mut self, x: usize, y: usize, value: [f32; 4]) {
        self.tiles[self.size.x * x + y] = value;
    }

    fn get(&self, x: usize, y: usize) -> [f32; 4] {
        return self.tiles[self.size.x * x + y];
    }

    fn update(&mut self) {
        for ant in self.ants.iter_mut() {
            ant.update(&mut self.tiles)
        }
    }

    fn add_ant(&mut self, ant: Ant) {
        self.ants.push(ant)
    }
}

struct Vector {
    x: usize,
    y: usize,
}

struct Ant {
    color: [f32; 4],
    position: Vector
}

impl Ant {
    fn new() -> Ant {
        let mut rng = rand::thread_rng();

        return Ant {
            color: [1.0, 0.0, 0.0, 1.0],
            position: Vector {
                x: 25,
                y: 25
            }
        }
    }

    fn update(&self, world: &Vec<[f32; 4]>) {
         
    }
}

fn main() {
    let mut world = World::new(50, 50);
    world.add_ant(Ant::new());
    world.update();
    let size = 5.0;
    let mut window: PistonWindow = WindowSettings::new("Piston Ant", [640, 480]).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            for (index, tile) in world.tiles.iter().enumerate() {
                let x = index / world.size.x;
                let y = index % world.size.x;
                rectangle(
                    *tile,
                    [size * (x as f64), size * (y as f64), size, size],
                    context.transform,
                    graphics
                );
            }
        });
    }

    println!("Hello world!");
}