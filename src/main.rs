extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate par_map;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;
use graphics::rectangle::{square, centered_square};
use rand::prelude::*;
use std::thread::sleep;
use par_map::ParMap;
use std::time::{Duration, SystemTime};

const BACKGROUND: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const PI: f64 = 3.141;
const WIDTH:f64 = 100.0;
const HEIGHT:f64 = 100.0;

struct Circle {
    x: f64,
    y: f64,
    r: f64,
    speed_x: f64,
    speed_y: f64,
    gl: GlGraphics,
}

impl Circle {
    fn random_circles(n: i64) -> Vec<Circle> {
        let mut rng = rand::thread_rng();

        let mut circles: Vec<Circle> = Vec::new();
        for i in 1..=n {
            let (x1, y1, r1): (f64, f64, f64) = (rng.gen(), rng.gen(), rng.gen_range(1.0,30.0));
            circles.push(Circle::new(x1, y1, r1 , 0.001, 0.001, GlGraphics::new(OpenGL::V3_2)));
        }
        circles
    }

    fn is_out_of_screen(self, width: f64, height: f64) -> bool {
        return self.x > width || self.y > height;
    }

    fn new(x: f64, y: f64, r: f64, speed_x: f64, speed_y: f64, gl: GlGraphics) -> Self {
        return Self { x, y, r, speed_x, speed_y, gl };
    }
    fn render(&mut self, args: &RenderArgs) {
        let (x, y) = (self.x * args.window_size[1], self.y * args.window_size[0]);
        let square = centered_square(x, y, self.r);

        let r = self.r;

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform;
            circle_arc(RED, r, 0.0, 2.0 * PI, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.x += self.speed_x * args.dt;
        self.y += self.speed_y * args.dt;
        let mut rng = rand::thread_rng();
        self.speed_x += rng.gen_range(-1.0, 1.0) * 1e-2;
        self.speed_y += rng.gen_range(-1.0, 1.0) * 1e-2;
        if self.x < 0.0 {
            self.speed_x *= rng.gen_range(0 as f64, 1.0);
        }
        if self.y < 0.0 {
            self.speed_y *= rng.gen_range(0 as f64, 1.0);
        }
        if self.x > WIDTH {
            self.speed_x *= rng.gen_range(-1.0, 0 as f64);
        }
        if self.y > HEIGHT {
            self.speed_y *= rng.gen_range(-1.0, 0 as f64);
        }
    }
}

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let square = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);
        });
    }

}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [1920, 10180])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };
    let mut circles = Circle::random_circles(300);

    let mut events = Events::new(EventSettings::new());
    events.swap_buffers(false);
    while let Some(e) = events.next(&mut window) {
        let curr = SystemTime::now();
        if let Some(args) = e.render_args() {
            app.render(&args);
            {
                for (i, x) in circles.iter_mut().enumerate() {
                    x.render(&args);
                }
            }
        }

        if let Some(args) = e.update_args() {
            // app.update(&args);
            for x in circles.iter_mut() {
                x.update(&args);
            }
        }
        println!("took - {} \r",curr.elapsed().unwrap().as_millis());
    }
}
