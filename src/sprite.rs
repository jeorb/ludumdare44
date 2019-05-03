
use quicksilver::geom::{Circle, Line, Rectangle, Shape, Triangle, Vector, about_equal};

#[derive(Clone)]
pub struct Sprite {
    pub bounds: Rectangle,
    pub speed: Vector,
    pub visible: bool,
    pub use_ttl: bool,
    pub ttl: usize,
    pub glyph: String,
    pub scale: Vector,
    pub z: f32,
}

impl Sprite {
    pub fn new(glyph: String, pos: Vector) -> Sprite {
        // TODO: Calculate from the glyph's mesh
        let size = Vector{ x: 100.0, y: 100.0 };
        Sprite{
            bounds: Rectangle {
                pos: pos - size/2, 
                size: size,
            },
            speed: Vector{ x: 0.0, y: 0.0 },
            visible: true,
            use_ttl: false,
            ttl: 1,
            glyph: glyph,
            scale: Vector{x: 1.0, y: 1.0},
            z: 0.0,
        }
    }

    pub fn move_by(&mut self, pos: Vector) {
        self.bounds.pos += pos;
    }

    pub fn move_to(&mut self, pos: Vector) {
        self.bounds.pos = pos - self.bounds.size/2;
    }

}

impl Shape for Sprite {
    //`overlaps`, `center`, `bounding_box`, `translate`

    fn center(&self) -> Vector {
        self.bounds.center()
    }

    fn contains(&self, point: impl Into<Vector>) -> bool {
        self.bounds.contains(point)
    }

    fn overlaps(&self, shape: &impl Shape) -> bool {
        self.bounds.overlaps(shape)
    }

    fn bounding_box(&self) -> Rectangle {
        self.bounds
    }

    fn translate(&self, v: impl Into<Vector>) -> Self {
        let mut sprite = self.clone();
        sprite.bounds.pos += v.into();
        sprite
    }
}