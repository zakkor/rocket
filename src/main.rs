extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use graphics::*;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const BLACK:  [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

struct ColoredRect {
    color: [f32; 4],
    rectangle: graphics::types::Rectangle,
}

impl ColoredRect {
    fn new() -> Self {
        ColoredRect {
            color: GREEN,
            rectangle: [0.0, 0.0, 0.0, 0.0]
        }
    }

    fn width_height(self, w: f64, h: f64) -> Self {
        let mut cr = self;
        cr.rectangle[2] = w;
        cr.rectangle[3] = h;
        cr
    }

    fn color(self, color: [f32; 4]) -> Self {
        let mut cr = self;
        cr.color = color;
        cr
    }

    fn position(self, x: f64, y: f64) -> Self {
        let mut cr = self;
        cr.rectangle[0] = x;
        cr.rectangle[1] = y;
        cr
    }

    fn next_color(&mut self) {
        self.color = match self.color {
            RED   => GREEN,
            GREEN => BLUE,
            BLUE  => RED,
            _     => [0.0, 0.0, 0.0, 0.0]
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    squares: Vec<ColoredRect>,
    player: ColoredRect,
    game_over: bool,
    bg_image: [Texture; 2],
    bg_position: [f64; 2],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // Clear the screen.
        clear(BLACK, &mut self.gl);

        let bg_image = &self.bg_image;
        let bg_position = self.bg_position;

        self.gl.draw(args.viewport(), |c, gl| {
            image(&bg_image[0], c.transform.trans(0.0, bg_position[0]), gl);
            image(&bg_image[1], c.transform.trans(0.0, bg_position[1]), gl);
        });

        let player = &self.player;
        self.gl.draw(args.viewport(), |c, gl| {
            rectangle(player.color, player.rectangle, c.transform, gl);
        });

        for s in &mut self.squares {
            self.gl.draw(args.viewport(), |c, gl| {
                rectangle(s.color, s.rectangle, c.transform, gl);
            });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Move down
        for i in 0..2 {
            self.bg_position[i] += 100.0 * args.dt;
            if self.bg_position[i] > 720.0 {
                self.bg_position[i] = -720.0;
            }

        }

        let mut number_of_pops = 0;

        for s in &mut self.squares {
            let (sr, pr) = (s.rectangle, self.player.rectangle);

            // Clean up
            if sr[1] > 720.0 {
                number_of_pops += 1;
            }

            if (pr[1] > sr[1]) && (pr[1] < (sr[1] + sr[3]))
                && (pr[0] > sr[0]) && (pr[0] < (sr[0] + sr[2]))
                && s.color != self.player.color {
                    self.game_over = true;
            }
            s.rectangle[1] += 100.0 * args.dt;
        }

        // Clean up
        while number_of_pops > 0 {
            self.squares.pop();
            number_of_pops -= 1;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "rocket",
            [1280, 720]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Load background image
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("res").unwrap();
    let image_path = assets.join("nebula.jpg");
    let my_image1 = Texture::from_path(&image_path).unwrap();
    let my_image2 = Texture::from_path(&image_path).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),

        squares: vec![ ColoredRect::new().width_height(640.0, 50.0).color(GREY).position(0.0, -200.0),
                       ColoredRect::new().width_height(640.0, 50.0).color(GREY).position(700.0, -200.0),
                       ColoredRect::new().width_height(1280.0, 25.0).color(RED).position(0.0, 0.0),
                       ColoredRect::new().width_height(1280.0, 25.0).color(GREEN).position(0.0, 100.0),
                       ColoredRect::new().width_height(1280.0, 25.0).color(BLUE).position(0.0, 200.0),
                       ColoredRect::new().width_height(1280.0, 25.0).color(RED).position(0.0, 300.0),],

        player: ColoredRect::new()
            .position(0.0, 720.0 - 75.0)
            .width_height(25.0, 25.0)
            .color(GREEN),

        game_over: false,

        bg_image: [my_image1, my_image2],

        bg_position: [-720.0, 0.0],
    };

    let mut cursor = [0.0, 0.0];

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if app.game_over {
            break;
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(_)  => app.player.next_color(),
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
            }
        };

        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            app.player.rectangle[0] = x;
        });

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
