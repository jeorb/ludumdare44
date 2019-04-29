extern crate quicksilver;
extern crate usvg;
mod path_convert;
mod glyph;

use glyph::{Glyph, GlyphSet};
use quicksilver::{
    Result,
    geom::{Transform, Vector},
    graphics::{Background::Col, Color, GpuTriangle, Mesh, ShapeRenderer, Vertex},
    input::Key,
    lifecycle::{Settings, State, Window, run},
    lyon::{
        tessellation::{FillOptions, FillTessellator},
    },
};
use path_convert::convert_path;



const BG_COLOR: Color = Color{ r: 0.2, g: 0.1, b: 0.2, a: 1.0};
const FG_COLOR: Color = Color{ r: 0.6, g: 0.2, b: 0.6, a: 1.0};

struct GameWindow {
    glyphs: GlyphSet,
    coin: Glyph,
    hero: Glyph,
    pos: Vector,
    speed: Vector,
    quit: bool,
}

impl State for GameWindow {
    fn new() -> Result<GameWindow> {

        //let mut tessellator = FillTessellator::new();
        //let mut blob = Glyph{ mesh: Mesh::new() };
        //let mut coin = Glyph{ mesh: Mesh::new() };

        let mut glyphs = GlyphSet::new().unwrap();
        glyphs.load_from_svg_bytes(include_bytes!("../static/glyphs.svg"));

        /*let svg_bytes = include_bytes!("../static/glyphs.svg");
        let mut svg_opt = usvg::Options::default();
        svg_opt.keep_named_groups = true;
        let rtree = usvg::Tree::from_data(svg_bytes, &svg_opt).unwrap();
        for node in rtree.root().descendants() {
            if let usvg::NodeKind::Group(ref g) = *node.borrow() {
                let group = g.id.clone();
                if "hero" == group || "0" == group {
                    let mut mesh = Mesh::new();
                    let mut shape_renderer = ShapeRenderer::new(&mut mesh, FG_COLOR);
                    let mut first = true;
                    for group_node in node.descendants() {
                        if let usvg::NodeKind::Path(ref p) = *group_node.borrow() {
                            if first {
                                // Skip the first element of the group (bounding rect)
                                first = false;
                                continue;
                            }
                            tessellator.tessellate_path(
                                convert_path(p),
                                &FillOptions::tolerance(0.1), &mut shape_renderer)
                            .unwrap();
                        }
                    }
                    if "hero" == group {
                        blob = Glyph{ mesh: mesh };
                    } else if "0" == group {
                        coin = Glyph{ mesh: mesh };
                    }
                }
            }
        }*/

        println!("hero {}", glyphs.get("hero").mesh.vertices.len());


        Ok(GameWindow{
            pos: Vector{x: 350.0, y: 250.0},
            speed: Vector{x: 0.0, y: 0.0},
            coin: glyphs.get("coin").clone(),
            hero: glyphs.get("hero").clone(),
            glyphs: glyphs,
            quit: false,
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

        if window.keyboard()[Key::Right].is_down() {
            self.speed.x += 2.5;
        }
        if window.keyboard()[Key::Left].is_down() {
            self.speed.x -= 2.5;
        }
        if window.keyboard()[Key::Up].is_down() {
            self.speed.y -= 2.5;
        }
        if window.keyboard()[Key::Down].is_down() {
            self.speed.y += 2.5;
        }
        if window.keyboard()[Key::Q].is_down() || window.keyboard()[Key::Escape].is_down() {
            self.quit = true;
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

        if self.quit {
            window.close();
        }

        window.clear(BG_COLOR)?;

        //println!("hero {}", self.hero.mesh.vertices.len());
        window.draw_ex(&self.hero, Col(FG_COLOR), Transform::translate(self.pos), 0);
        window.draw_ex(&self.coin, Col(FG_COLOR), Transform::translate(self.pos*0.5), 0);
        window.draw_ex(&self.coin, Col(FG_COLOR), Transform::translate(self.pos*1.5), 0);

        Ok(())
    }
}

fn main() {
    println!("Starting game...");
    let mut settings = Settings::default();
    //settings.resize = ResizeStrategy::Fill;
    //settings.fullscreen = true;
    //settings.vsync = true;
    settings.multisampling = Some(4);

    run::<GameWindow>("Your life is currency", Vector::new(800, 600), settings);
}
