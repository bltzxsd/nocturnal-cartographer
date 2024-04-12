mod models;

use std::{env, ops::DerefMut, path};

use cartography_core::{colors, seed};
use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics, Context, GameResult,
};
use models::logger::{Log, TextParams};

#[allow(unused)]
#[derive(Debug)]
struct Cartographer {
    palette: colors::Palette,
    seed: seed::Seed,
    log: Log,
    counter: u32,
}

#[allow(unused)]
impl Cartographer {
    const HEAVY_LINE: f32 = 10.0;
    const STANDARD_LINE: f32 = 3.0;
    const THIN_LINE: f32 = 1.0;
    const BORDER: f32 = 50.0;
    const TEXT_HEIGHT: f32 = 16.0;
    const FONT: &'static str = "JetBrains Mono";

    pub fn new(ctx: &mut Context) -> GameResult<Cartographer> {
        let mut seed = seed::Seed::new();

        let palette = colors::Palette::random(seed.deref_mut(), 1.0, 1.0);

        ctx.gfx.add_font(
            Self::FONT,
            graphics::FontData::from_path(ctx, "/JetBrainsMono.ttf")?,
        );

        let txt_params = TextParams::new(
            *palette.fg(),
            Self::TEXT_HEIGHT,
            Self::FONT,
            Self::THIN_LINE,
        );

        let log = Log::new(txt_params, ctx)?;

        Ok(Cartographer {
            palette,
            seed,
            counter: 0,
            log,
        })
    }
}

impl EventHandler for Cartographer {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from(*self.palette.bg()));

        // draw the log box
        let box_offset = self.log.set_box_position(ctx, (Self::BORDER, Self::BORDER));
        let text_offset = box_offset + ggez::glam::Vec2::new(5.0, 2.0);
        let (log_text, log_box) = (self.log.text(), &self.log.mesh());

        canvas.draw(&log_text, graphics::DrawParam::default().dest(text_offset));
        canvas.draw(log_box, graphics::DrawParam::default().dest(box_offset));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        ctx.gfx
            .set_window_title(format!("Cartographer - FPS {}", ctx.time.fps().round()).as_str());
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        use ggez::input::keyboard::{KeyCode, KeyMods};
        match input.keycode {
            Some(KeyCode::S) => self.log.decr_offset(),
            Some(KeyCode::W) => self.log.incr_offset(),
            Some(KeyCode::A) => {
                self.log
                    .push(format!("Pushed String No: {}", self.counter + 1));
                self.counter += 1;
            }
            Some(KeyCode::N) => {
                self.palette = colors::Palette::random(self.seed.deref_mut(), 1.0, 1.0);
                self.log.color_mut(ctx, *self.palette.fg())?;
            }
            Some(KeyCode::C) => {
                if input.mods.contains(KeyMods::CTRL) {
                    println!("terminating!");
                    ctx.request_quit();
                }
            }
            _ => (),
        }
        Ok(())
    }
}
fn main() -> GameResult {
    let resource_dir = env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| path::PathBuf::from("./resources"),
        |path| path::PathBuf::from(path + "/resources"),
    );

    let window_setup = WindowSetup::default().title("Cartographer");

    let window_mode = WindowMode::default().dimensions(1280.0, 720.0);

    let cb = ggez::ContextBuilder::new("meshbatch", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(window_setup)
        .window_mode(window_mode);

    let (mut ctx, event_loop) = cb.build()?;

    let state = Cartographer::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
