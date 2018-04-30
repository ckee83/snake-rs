use piston::input::*;
use opengl_graphics::{ GlGraphics, Filter, GlyphCache, TextureSettings };
use palette::_TEXT as TEXT;

pub struct TextRenderer<'a> {
    glyphs: GlyphCache<'a>,
}

impl<'a> TextRenderer<'a> {
    pub fn new() -> TextRenderer<'a> {
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let glyphs = GlyphCache::new(
                "assets/Unique.ttf",
                (),
                texture_settings
            )
            .expect("Could not load font");

        TextRenderer {
            glyphs,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::text;
        use graphics::Transformed;

        let txt = String::from("Paused");

        let x: u32 = 300;
        let y: u32 = 300;

        gl.draw(args.viewport(), |c, gl| {
            let text_transform = c.transform.trans(x.into(), y.into());

            text::Text::new_color(TEXT, 32).draw(
                &txt,
                &mut self.glyphs,
                &c.draw_state,
                text_transform,
                gl,
            ).expect("Could not render text");
        });
    }
}
