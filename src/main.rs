extern crate quicksilver;
extern crate usvg;
mod path_convert;
mod glyph;
mod input;

use glyph::{Glyph, GlyphSet};
use quicksilver::{
    Result,
    geom::{Transform, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

const BG_COLOR: Color = Color{ r: 0.2, g: 0.1, b: 0.2, a: 1.0};
const FG_COLOR: Color = Color{ r: 0.6, g: 0.2, b: 0.6, a: 1.0};

struct GameWindow {
    glyphs: GlyphSet,
    sprites: Vec<Sprite>,
    coin: Glyph,
    hero: Glyph,
    pos: Vector,
    speed: Vector,
}

struct Sprite {
    pos: Vector,
    glyph: String
}

impl State for GameWindow {
    fn new() -> Result<GameWindow> {
        let mut glyphs = GlyphSet::new().unwrap();
        glyphs.load_from_svg_bytes(include_bytes!("../assets/glyphs.svg"));

        let mut sprites = Vec::new();
        sprites.push(Sprite{ pos: Vector{x: 505.0, y: 400.0}, glyph: glyph::COIN.to_owned() });

        Ok(GameWindow{
            pos: Vector{x: 350.0, y: 250.0},
            speed: Vector{x: 0.0, y: 0.0},
            coin: glyphs.get(glyph::COIN).clone(),
            hero: glyphs.get(glyph::COIN).clone(),
            glyphs: glyphs,
            sprites: sprites,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.speed.x.abs() > 0.0 || self.speed.y.abs() > 0.0 {
            self.speed *= 0.9;
            if self.speed.x.abs() < 0.1 {
                self.speed.x = 0.0;
            }
            if self.speed.y.abs() < 0.1 {
                self.speed.y = 0.0;
            }
        }

        let input = input::get_input(&window);

        self.speed.x += 2.5 * input.x;
        self.speed.y += 2.5 * input.y;

        if input.quit {
            window.close();
        }

        self.pos += self.speed;
        if self.pos.x > 800.0 {
            self.pos.x = 0.0;
        } else if self.pos.x < 0.0 {
            self.pos.x = 800.0;
        }
        if self.pos.y > 600.0 {
            self.pos.y = 0.0;
        } else if self.pos.y < 0.0 {
            self.pos.y = 600.0;
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(BG_COLOR)?;

        window.draw_ex(&self.hero, Col(FG_COLOR), Transform::translate(self.pos), 0);
        window.draw_ex(&self.coin, Col(FG_COLOR), Transform::translate(self.pos*0.5), 0);
        window.draw_ex(&self.coin, Col(FG_COLOR), Transform::translate(self.pos*1.5), 0);

        for sprite in &self.sprites {
            window.draw_ex(self.glyphs.get(&sprite.glyph), Col(FG_COLOR), Transform::translate(sprite.pos), 0);
        }

        Ok(())
    }
}

fn main() {
    //println!("Starting game...");
    let mut settings = Settings::default();
    //settings.resize = ResizeStrategy::Fill;
    //settings.fullscreen = true;
    //settings.vsync = true;
    settings.multisampling = Some(4);

    run::<GameWindow>("Your life is currency", Vector::new(800, 600), settings);
}
