use piston::input::*;
use opengl_graphics::{ GlGraphics, Filter, GlyphCache, TextureSettings };
use palette::_TEXT as TEXT;
use palette::_TEXT_ALT as TEXT_ALT;

pub struct TextRenderer<'a> {
    glyphs: GlyphCache<'a>,
}

impl<'a> TextRenderer<'a> {
    pub fn new() -> TextRenderer<'a> {
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let glyphs = GlyphCache::new(
                "assets/getvoip_grotesque.ttf",
                (),
                texture_settings
            )
            .expect("Could not load font");

        TextRenderer {
            glyphs,
        }
    }

    pub fn render(
        &mut self,
        gl: &mut GlGraphics,
        args: &RenderArgs,
        x: u32,
        y: u32,
        txt: &String,
        font_size: u32,
        color: [f32; 4]
    ) {
        use graphics::text;
        use graphics::Transformed;

        gl.draw(args.viewport(), |c, gl| {
            let text_transform = c.transform.trans(x.into(), y.into());

            text::Text::new_color(color, font_size).draw(
                &txt,
                &mut self.glyphs,
                &c.draw_state,
                text_transform,
                gl,
            ).expect("Could not render text");
        });
    }

    pub fn large(&mut self, gl: &mut GlGraphics, args: &RenderArgs, x: u32, y: u32, txt: &String) {
        self.render(gl, args, x, y, txt, 32, TEXT_ALT);
    }
    pub fn medium(&mut self, gl: &mut GlGraphics, args: &RenderArgs, x: u32, y: u32, txt: &String) {
        self.render(gl, args, x, y, txt, 16, TEXT);
    }
    pub fn small(&mut self, gl: &mut GlGraphics, args: &RenderArgs, x: u32, y: u32, txt: &String) {
        self.render(gl, args, x, y, txt, 12, TEXT_ALT);
    }
}
