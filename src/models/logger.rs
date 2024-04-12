use ggez::{
    context::Has,
    glam::Vec2,
    graphics::{self, Drawable, GraphicsContext, Mesh, PxScale, Text, TextFragment},
};

use super::Result;

/// Logging system for displaying text on screen.
#[derive(Debug)]
pub struct Log {
    text: Vec<String>,
    text_params: TextParams,
    mesh: Mesh,
    offset: usize,
}

/// Parameters for configuring the appearance of text in the log.
#[derive(Debug)]
pub struct TextParams {
    color: [f32; 4],
    line_height: PxScale,
    font: String,
    stroke_width: f32,
}

impl TextParams {
    /// Creates a new [`TextParams`] instance.
    /// Parameters:
    /// - `color`: [r, g, b, a]
    /// - `line_height`: scale of font
    /// - `font`: name of font as stored in ggez setup
    /// - `stroke_width`: standard line weight used to construct bounding box
    pub fn new(color: [f32; 4], line_height: f32, font: &str, stroke_width: f32) -> TextParams {
        let font = font.into();
        let line_height = line_height.into();
        Self {
            color,
            line_height,
            font,
            stroke_width,
        }
    }

    /// Returns the font used.
    pub fn font(&self) -> &str {
        &self.font
    }

    /// Returns the line height in [`PxScale`]
    pub fn height(&self) -> PxScale {
        self.line_height
    }

    /// Returns the color: [r, g, b, a]
    pub fn color(&self) -> &[f32; 4] {
        &self.color
    }
}

impl Log {
    const WIDTH: f32 = 400.0;
    const HEIGHT: f32 = 88.0;
    /// Creates a new [`Log`] instance.
    pub fn new(params: TextParams, ctx: &impl Has<GraphicsContext>) -> Result<Log> {
        let mut builder = graphics::MeshBuilder::new();
        builder.rectangle(
            graphics::DrawMode::stroke(params.stroke_width),
            graphics::Rect {
                w: Self::WIDTH,
                h: Self::HEIGHT,
                ..Default::default()
            },
            graphics::Color::from(*params.color()),
        )?;
        let mesh = Mesh::from_data(ctx, builder.build());

        Ok(Self {
            text: vec![],
            text_params: params,
            offset: 0,
            mesh,
        })
    }

    /// Sets the position of the log box on the screen, based on the provided offset: (width, height).
    pub fn set_box_position(&self, ctx: &impl Has<GraphicsContext>, offset: (f32, f32)) -> Vec2 {
        let (_, screen_height) = ctx.retrieve().drawable_size();
        let width = offset.0;
        let height = screen_height - self.mesh.dimensions(ctx).unwrap().h - offset.1;
        (width, height).into()
    }

    /// Returns a [`Text`] instance with the log messages, based on the current offset.
    pub fn text(&mut self) -> Text {
        let mut text = Text::default();
        let params = &self.text_params;
        let vec_len = self.text.len();
        let end = self.offset.min(vec_len).min(vec_len);
        let start = match vec_len > 5 {
            true => end.saturating_sub(5),
            false => 0,
        };

        for i in (start..end).rev() {
            text.add(
                TextFragment::from(&self.text[i])
                    .font(params.font())
                    .scale(params.height())
                    .color(*params.color()),
            );
            text.add('\n').set_scale(params.height());
        }
        text
    }

    /// Adds a new message to the log
    ///
    /// Adding a new message will reset the log `offset` to be max,
    /// causing a jump to the top of the log.
    pub fn push(&mut self, s: String) {
        self.text.push(s);
        self.offset = self.text.len();
    }

    /// Returns a clone of the log's [`Mesh`].
    pub fn mesh(&self) -> Mesh {
        self.mesh.clone()
    }

    /// Increments the log's offset by one, if
    /// - the offset is less than the number of log messages.
    pub fn incr_offset(&mut self) {
        if self.offset < self.text.len() {
            self.offset += 1;
        }
    }

    /// Decrements the log's `offset` by one, if:
    /// - the `offset` is greater than 0
    /// - **AND** the number of log messages is greater than 5
    /// - **OR**  the `offset` is less than or equal to 5.
    ///
    /// This allows the `offset` to be
    pub fn decr_offset(&mut self) {
        let len = self.text.len();
        if (len > 5 && self.offset == 5) || (self.text.len() <= 5 && self.offset <= 5) {
            return;
        } else if self.offset > 0 {
            self.offset -= 1;
        }
    }
}
