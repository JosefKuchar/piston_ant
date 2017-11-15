extern crate piston_window;
extern crate rand;
extern crate opengl_graphics;
extern crate piston;
extern crate image;

use rand::distributions::{IndependentSample, Range};
use piston_window::*;
use opengl_graphics::{ OpenGL };

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
    tiles: Vec<[u8; 4]>
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        return Grid {
            tiles: vec![[255; 4]; width * height],
            size: Vector {
                x: width,
                y: height
            }
        }
    }

    fn set(&mut self, position: &mut IVector, value: [u8; 4]) {
        self.bound_position(position);

        self.tiles[self.size.x * position.y as usize + position.x as usize] = value;
    }

    fn get(&self, position: &mut IVector) -> [u8; 4] {
        self.bound_position(position);

        return self.tiles[self.size.x * position.y as usize + position.x as usize];
    }

    fn bound_position(&self, position: &mut IVector) {
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

struct IVector {
    x: isize,
    y: isize,
}

impl IVector {
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
    color: [u8; 4],
    position: IVector,
    direction: usize
}

impl Ant {
    fn new(grid_size: &Vector) -> Ant {
        let mut rng = rand::thread_rng();
        let range = Range::new(0, 256);
        let x_range = Range::new(0, grid_size.x);
        let y_range = Range::new(0, grid_size.y);

        return Ant {
            color: [range.ind_sample(&mut rng) as u8, range.ind_sample(&mut rng) as u8, range.ind_sample(&mut rng) as u8, 255],
            position: IVector {
                x: x_range.ind_sample(&mut rng) as isize,
                y: y_range.ind_sample(&mut rng) as isize
            },
            direction: 3
        }
    }

    fn update(&mut self, world: &mut Grid) {
        static DIRECTIONS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
        let color = world.get(&mut self.position);

        if color[0] == 255 && color[1] == 255 && color[2] == 255 {
            self.direction += 1;
            self.direction %= 4;
            world.set(&mut self.position, self.color)
        } else {
            self.direction += 3;
            self.direction %= 4;
            world.set(&mut self.position, [255; 4])
        }

        self.position.add(DIRECTIONS[self.direction]);
    }
}

fn main() {
    let width = 640;
    let height = 360;
    
    let mut world = World::new(width, height);

    let opengl = OpenGL::V3_2;

    for _ in 0..7
     {
        let ant = Ant::new(&world.grid.size);
        world.add_ant(ant);
    }

    let mut zoom = 1.0;

    let mut window: PistonWindow = WindowSettings::new("Piston Ant", [640, 480]).opengl(opengl).build().unwrap();

    let mut canvas = image::ImageBuffer::new(width as u32, height as u32);
    let mut texture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new().filter(texture::Filter::Nearest)
    ).unwrap();

    while let Some(e) = window.next() {
        
        if let Some(_) = e.update_args() {
            for _ in 0..300 {
                world.update();
            }
        }
        
        if let Some(_) = e.render_args() {
            
            for (index, tile) in world.grid.tiles.iter().enumerate() {

                let x = index % world.grid.size.x;
                let y = index / world.grid.size.x;
                
                canvas.put_pixel(x as u32, y as u32, image::Rgba(*tile));
            }

            texture.update(&mut window.encoder, &canvas).unwrap();

            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);
                image(&texture, c.transform.scale(zoom, zoom), g);
            });
        }

        e.mouse_scroll(|_, y| {
            zoom += y;

            if zoom < 1.0 {
                zoom = 1.0;
            }
        });
    }
}