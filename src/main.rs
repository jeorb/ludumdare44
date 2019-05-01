extern crate quicksilver;
extern crate usvg;
mod path_convert;
mod glyph;
mod input;

use glyph::{Glyph, GlyphSet};
use quicksilver::{
    Result,
    geom::{Transform, Vector},
    graphics::{Background::Col, Color, ResizeStrategy},
    lifecycle::{Settings, State, Window, run},
};

const BG_COLOR: Color = Color{ r: 0.2, g: 0.1, b: 0.2, a: 1.0};
const FG_COLOR: Color = Color{ r: 0.6, g: 0.2, b: 0.6, a: 1.0};

struct GameWindow {
    glyphs: GlyphSet,
    sprites: Vec<Sprite>,
    hero: Glyph,
    pos: Vector,
    speed: Vector,
    cooldown: usize,
}

struct Sprite {
    pos: Vector,
    speed: Vector,
    ttl: usize,
    glyph: String
}

impl State for GameWindow {
    fn new() -> Result<GameWindow> {
        let mut glyphs = GlyphSet::new().unwrap();
        glyphs.load_from_svg_bytes(include_bytes!("../assets/glyphs.svg"));

        let mut sprites = Vec::new();

        /*sprites.push(Sprite{
                pos: Vector{ x: 0.0, y: 0.0 },
                speed: Vector{ x: 0.1, y: 0.15 },
                ttl: 1000,
                glyph: "fake".to_owned()
            });*/

        Ok(GameWindow{
            pos: Vector{x: 350.0, y: 250.0},
            speed: Vector{x: 0.0, y: 0.0},
            hero: glyphs.get(glyph::HERO).clone(),
            glyphs: glyphs,
            sprites: sprites,
            cooldown: 0,
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

        self.sprites.retain(|s| s.ttl > 0);

        for sprite in &mut self.sprites {
            sprite.ttl -= 1;
            sprite.pos += sprite.speed;
        }

        if self.cooldown > 0 {
            self.cooldown -= 1;
        } else if input.shoot {
            self.cooldown = 6;
            self.sprites.push(Sprite{
                pos: Vector{ x: self.pos.x, y: self.pos.y - 50.0 },
                speed: Vector{ x: self.speed.x, y: self.speed.y - 20.0 },
                ttl: 60,
                glyph: glyph::COIN.to_owned()
            });
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(BG_COLOR)?;

        for sprite in &self.sprites {
            window.draw_ex(self.glyphs.get(&sprite.glyph), Col(FG_COLOR), Transform::translate(sprite.pos), 0);
        }

        window.draw_ex(&self.hero, Col(FG_COLOR), Transform::translate(self.pos), 0);

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn is_wasm() -> bool {
    return false;
}

#[cfg(target_arch = "wasm32")]
fn is_wasm() -> bool {
    return true;
}

fn main() {
    //println!("Starting game...");
    let mut settings = Settings::default();
    settings.resize = ResizeStrategy::Fill;
    
    if is_wasm(){
        settings.fullscreen = true;
    }

    //settings.vsync = true;
    settings.multisampling = Some(4);

    run::<GameWindow>("Your life is currency", Vector::new(800, 600), settings);
}
