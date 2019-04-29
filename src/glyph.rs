extern crate quicksilver;
extern crate usvg;

use std::collections::HashMap;
use quicksilver::{
    Result,
    geom::{Transform, Scalar},
    graphics::{Background, Background::Col, Color, Drawable, GpuTriangle, Mesh, ShapeRenderer, Vertex},
    lyon::{
        tessellation::{FillOptions, FillTessellator},
    },
};
use crate::path_convert::convert_path;


pub struct Glyph {
    pub mesh: Mesh,
}

impl Drawable for Glyph {

    fn draw<'a>(&self, dest: &mut Mesh, background: Background<'a>, transform: Transform, z: impl Scalar){
        let offset = dest.vertices.len() as u32;
        dest.vertices.extend(self.mesh.vertices.iter()
            .map(|v| Vertex{pos: transform * v.pos, col: v.col, tex_pos: v.tex_pos}));
        dest.triangles.extend(self.mesh.triangles.iter()
            .map(|t| GpuTriangle{
                z:t.z,
                indices:[t.indices[0]+offset, t.indices[1]+offset, t.indices[2]+offset],
                image: t.image.clone()}));
    }
}

impl Clone for Glyph {
    fn clone(&self) -> Glyph {
        let mut mesh = Mesh::new();
        mesh.vertices.extend(self.mesh.vertices.iter()
            .map(|v| Vertex{pos: v.pos, col: v.col, tex_pos: v.tex_pos}));
        mesh.triangles.extend(self.mesh.triangles.iter()
            .map(|t| GpuTriangle{
                z:t.z,
                indices:t.indices,
                image: t.image.clone()}));
        Glyph{ mesh: mesh }
    }
}

pub struct GlyphSet {
    glyphs: HashMap<String, Glyph>,
}

impl GlyphSet {
    pub fn new() -> Result<GlyphSet> {
        let mut glyphs: HashMap<String, Glyph> = HashMap::new();
        glyphs.insert("missing".to_owned(), Glyph{ mesh: Mesh::new() });
        Ok(GlyphSet{
            glyphs: glyphs,
        })
    }

    pub fn get(&self, key: &str) -> &Glyph {
        match self.glyphs.get(key) {
            Some(glyph) => glyph,
            None => self.glyphs.get("missing").unwrap(),
        }
    }

    pub fn load_from_svg_bytes(&mut self, bytes: &[u8]){
        let mut tessellator = FillTessellator::new();
        let color: Color = Color{ r: 0.6, g: 0.2, b: 0.6, a: 1.0};

        let mut svg_opt = usvg::Options::default();
        svg_opt.keep_named_groups = true;
        let rtree = usvg::Tree::from_data(bytes, &svg_opt).unwrap();
        for node in rtree.root().descendants() {
            if let usvg::NodeKind::Group(ref g) = *node.borrow() {
                let group = g.id.clone();
                let group_len = group.chars().count();
                if group.starts_with("glyph_") && group_len > 6{
                    let key: String = group.chars().skip(6).take(group_len-6).collect();
                    let mut mesh = Mesh::new();
                    let mut shape_renderer = ShapeRenderer::new(&mut mesh, color);
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
                    println!("Found {} {}", key, mesh.vertices.len());
                    self.glyphs.insert(key, Glyph{ mesh: mesh });
                }
            }
        }
    }
}
