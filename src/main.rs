extern crate quicksilver;
extern crate usvg;
extern crate rand;
mod path_convert;
mod glyph;
mod input;
mod sprite;

use rand::Rng;
use glyph::{Glyph, GlyphSet};
use sprite::Sprite;
use quicksilver::{
    Result,
    geom::{Scalar, Shape, Transform, Vector},
    graphics::{Background::Col, Color, ResizeStrategy},
    input::{Key, ButtonState,},
    lifecycle::{Event, Settings, State, Window, run},
};

const BG_COLOR: Color = Color{ r: 0.2, g: 0.1, b: 0.2, a: 1.0};
const FG_COLOR: Color = Color{ r: 0.6, g: 0.2, b: 0.6, a: 1.0};
const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 1000.0;

struct GameWindow {
    glyphs: GlyphSet,
    sprites: Vec<Sprite>,
    hero: Glyph,
    pos: Vector,
    speed: Vector,
    cooldown: usize,
    mouse_pos: Vector,
    mouse_cooldown: usize,
    mouse_pressed: bool,
    scale: Vector,
    frame: usize,
    show_fps: bool,
}

impl State for GameWindow {
    fn new() -> Result<GameWindow> {
        let mut glyphs = GlyphSet::new().unwrap();
        glyphs.load_from_svg_bytes(include_bytes!("../assets/glyphs.svg"));

        let mut sprites = Vec::new();

        {
            let mut cursor = Sprite::new(
                "cursor",
                Vector{ x: -1000.0, y: -1000.0 }
            );
            cursor.visible = false;
            cursor.z = 10.0;
            sprites.push(cursor);
        }

        /* 2mark
        for x in 0..100 {
            for y in 0..100 {
                let mut sprite = Sprite::new(
                    "2".to_owned(),
                    Vector{ x: (x*5) as f32, y: (y*5) as f32 }
                );
                sprite.z = -10.0;
                sprite.scale = Vector { x: 0.1, y: 0.1 };
                sprites.push(sprite);
            }
        }*/

        Ok(GameWindow{
            pos: Vector{x: WIDTH/2.0 - 350.0, y: HEIGHT/2.0 - 50.0},
            speed: Vector{x: 0.0, y: 0.0},
            hero: glyphs.get(glyph::HERO).clone(),
            glyphs: glyphs,
            sprites: sprites,
            cooldown: 0,
            mouse_pos: Vector{ x: 0.0, y: 0.0 },
            mouse_cooldown: 0,
            mouse_pressed: false,
            scale: Vector{x: 1.0, y: 1.0},
            frame: 0,
            show_fps: true,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseButton(_button, state) => {
                match state {
                    ButtonState::Pressed => self.mouse_pressed = true,
                    ButtonState::Held => self.mouse_pressed = true,
                    ButtonState::Released => self.mouse_pressed = false,
                    _ => {},
                }
            }
            _ => {}
        };

        Ok(())
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
        match window.keyboard()[Key::Tab] {
            ButtonState::Pressed => {
                //println!("View:          {:?}", window.view());
                //println!("Screen Size:   {:?}", window.screen_size());
            },
            _ => ()
        }

        match window.keyboard()[Key::F] {
            ButtonState::Pressed => {
                self.show_fps = if self.show_fps { false } else { true };
            },
            _ => ()
        }

        let mut input = input::get_input(&window, self.mouse_pressed);

        self.speed.x += 2.5 * input.x;
        self.speed.y += 2.5 * input.y;

        if input.quit {
            window.close();
        }

        self.pos += self.speed;
        if self.pos.x > WIDTH {
            self.pos.x = 0.0;
        } else if self.pos.x < 0.0 {
            self.pos.x = WIDTH;
        }
        if self.pos.y > HEIGHT {
            self.pos.y = 0.0;
        } else if self.pos.y < 0.0 {
            self.pos.y = HEIGHT;
        }

        self.sprites.retain(|s| (!s.use_ttl) || s.ttl > 0);

        for sprite in &mut self.sprites {
            sprite.ttl -= 1;
            sprite.move_by(sprite.speed);
        }

        if self.cooldown > 0 {
            self.cooldown -= 1;
        } else if input.shoot {
            self.scale *= 0.99;
            self.cooldown = 6;
            self.sprites.push({
                let mut sprite = Sprite::new(glyph::COIN, Vector{ x: self.pos.x, y: self.pos.y - 50.0 });
                sprite.speed = Vector{ x: self.speed.x, y: self.speed.y - 20.0 };
                sprite.use_ttl = true;
                sprite.ttl = 60;
                sprite.scale = self.scale;
                sprite.z = 1.0;
                sprite
            });
        } else {
            if self.scale.y < 1.0 {
                self.scale *= 1.001;
            }
        }

        let mouse = window.mouse().pos();
        if mouse != self.mouse_pos {
            self.mouse_pos = mouse;
            self.mouse_cooldown = 60;
            self.sprites[0].move_to(Vector{ x: mouse.x, y: mouse.y });
            self.sprites[0].visible = true;
        } else {
            if self.mouse_cooldown > 0 {
                self.mouse_cooldown -= 1;
            } else {
                self.sprites[0].visible = false;
            }
        }
        //if self.mouse_pressed {
        //    input.x = 1.0 + (mouse.x*2.0 - WIDTH*2.0)/WIDTH;
        //    input.y = 1.0 + (mouse.y*2.0 - HEIGHT*2.0)/HEIGHT;
        //}


        if self.show_fps {
            if self.frame % 60 == 0 {
                self.glyphs.insert("fps", Glyph::from_text(format!("fps: {:.0}/{:.0}", window.current_fps(), window.average_fps()), 12.0, Col(FG_COLOR), &self.glyphs));
                self.sprites.push({
                    let mut sprite = Sprite::new("fps", Vector{ x: 10.0, y: 10.0 });
                    sprite.use_ttl = true;
                    sprite.ttl = 60;
                    sprite.z = 10.0;
                    sprite
                });
            }
            self.frame += 1;
        }


        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(BG_COLOR)?;

        for sprite in &self.sprites {
            if sprite.visible {
                window.draw_ex(
                    self.glyphs.get(&sprite.glyph),
                    Col(FG_COLOR),
                    Transform::translate(sprite.bounds.center()) * Transform::scale(sprite.scale),
                    sprite.z
                );
            }
        }

        window.draw_ex(&self.hero, Col(FG_COLOR), Transform::translate(self.pos) * Transform::scale(self.scale), 10);

        window.draw_ex(
            &Glyph::from_text("Hello, World!\n0.0123456789.,'\";:&!/\\|[]{}\nABCDEFGHIJKLMNOPQRSTUVWXYZ\nabcdefghijklmnopqrstuvwxyz".to_owned(), 10.0, Col(FG_COLOR),
            &self.glyphs), Col(FG_COLOR), Transform::translate(Vector{x:self.pos.x, y:self.pos.y+100.0}) * Transform::scale(self.scale), 0);

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

    run::<GameWindow>("Happy Mother's Day!", Vector::new(WIDTH, HEIGHT), settings);
}
