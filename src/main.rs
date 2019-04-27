extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color, ResizeStrategy},
    lifecycle::{Settings, State, Window, run},
};

const BG_COLOR: Color = Color{ r: 0.2, g: 0.1, b: 0.2, a: 1.0};
const FG_COLOR: Color = Color{ r: 0.6, g: 0.2, b: 0.6, a: 1.0};

struct GameWindow{
    x: i32,
    y: i32,
}

impl State for GameWindow {
    fn new() -> Result<GameWindow> {
        Ok(GameWindow{ x: 350, y: 250 })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.x += 1;
        self.y += 1;
        if self.x > 800 {
            self.x = 0;
        }
        if self.y > 600 {
            self.y = 0;
        }

        window.clear(BG_COLOR)?;
        window.draw(&Rectangle::new((self.x, self.y), (64, 64)), Col(FG_COLOR));
        window.draw(&Rectangle::new((self.x+100, self.y), (64, 64)), Col(FG_COLOR));
        window.draw(&Rectangle::new((self.x+200, self.y), (64, 64)), Col(FG_COLOR));
        window.draw(&Rectangle::new((self.x+300, self.y), (64, 64)), Col(FG_COLOR));
        window.draw(&Rectangle::new((self.x+400, self.y), (64, 64)), Col(FG_COLOR));
        Ok(())
    }
}

fn main() {
    println!("Starting game...");
    let mut settings = Settings::default();
    //settings.resize = ResizeStrategy::Fill;
    //settings.fullscreen = true;

    run::<GameWindow>("Your life is currency", Vector::new(800, 600), settings);
}
