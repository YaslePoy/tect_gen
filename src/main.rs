mod basic;

use crate::basic::Table;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, ImageFormat};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use rand::rngs::ThreadRng;

const WIN_SIZE_F: f32 = 1000.0;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("tectonic_generator", "YaslePoy")
        .window_setup(WindowSetup::default().title("Tectonic generator"))
        .window_mode(
            WindowMode::default()
                .resizable(false)
                .dimensions(WIN_SIZE_F, WIN_SIZE_F),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    display: Image,
    pub raw_view: Table<Color>,
    require_draw: bool,
    state: Box<dyn LandscapeState>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let initial_size = 32_u32;
        let raw = Table::new(Color::BLACK, initial_size as usize);
        let game = MyGame {
            display: Image::from_pixels(
                _ctx,
                &vec![0_u8; (initial_size * initial_size * 4) as usize],
                ImageFormat::Rgba8UnormSrgb,
                initial_size,
                initial_size,
            ),
            raw_view: raw,
            require_draw: false,
            state: Box::new(SeedLandscape::new(initial_size)),
        };
        game
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::T)
        {
            self.state.propagate(LandscapeTick::Tick);
            self.require_draw = true;
        }
        if _ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::N)
        {
            self.state.propagate(LandscapeTick::Next);
            self.require_draw = true;
        }
        if _ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::S)
        {
            self.raw_view.grow();
            self.state.propagate(LandscapeTick::Scale);
            self.require_draw = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        if self.require_draw {
            self.state.render(&mut self.raw_view);
            let pixels: Vec<u8> = self.raw_view.clone().try_into().unwrap();
            self.display = Image::from_pixels(
                ctx,
                &pixels,
                ImageFormat::Rgba8UnormSrgb,
                self.raw_view.side as u32,
                self.raw_view.side as u32,
            );
            self.require_draw = false;
        }
        
        let scale = WIN_SIZE_F / self.raw_view.side as f32;
        canvas.draw(
            &self.display,
            DrawParam::default().scale(Vec2::new(scale, scale)),
        );

        canvas.finish(ctx)
    }
}

enum LandscapeTick {
    Scale,
    Tick,
    Next,
}

trait LandscapeState {
    fn propagate(&mut self, action: LandscapeTick);
    fn render(&mut self, display: &mut Table<Color>);
}

struct SeedLandscape {
    field: Table<bool>,
    random: ThreadRng,
    color: Color,
}

impl SeedLandscape {
    pub fn new(size: u32) -> SeedLandscape {
        Self {
            field: Table::new(false, size as usize),
            random: rand::rng(),
            color: Color::WHITE,
        }
    }
}

impl LandscapeState for SeedLandscape {
    fn propagate(&mut self, action: LandscapeTick) {
        match action {
            LandscapeTick::Scale => self.field.grow(),
            LandscapeTick::Tick => loop {
                let index = self.random.random_range(0..self.field.data.len());
                if !self.field[index] {
                    self.field[index] = true;
                    break;
                }
            },
            LandscapeTick::Next => {
                
            }
        }
    }

    fn render(&mut self, display: &mut Table<Color>) {
        self.field.convert_copy(display, |x| {
            if x { self.color } else { Color::BLACK }
        });
    }
}
