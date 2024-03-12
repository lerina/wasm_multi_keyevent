use anyhow::Result;

use crate::engine::{Game, KeyState, Point, Rect, Renderer};

pub struct GameObj {
    position: Point,
}

impl GameObj {
    pub fn new() -> Self {
        GameObj {
            position: Point { x: 50, y: 50 },
        }
    }
}

impl Game for GameObj {
    fn initialize(&self) -> Result<Box<dyn Game>> {
        Ok(Box::new(GameObj {
            position: self.position,
        }))
    }

    fn update(&mut self, keystate: &KeyState) {
        let mut velocity = Point { x: 0, y: 0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3;
            log!("ArrowDown");
        }

        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3;
            log!("ArrowUp");
        }

        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3;
            log!("ArrowRight");
        }

        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3;
            log!("ArrowLeft");
        }

        self.position.x += velocity.x;
        self.position.y += velocity.y;
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        renderer.set_common_style();
        renderer.set_stroke_style("yellow");
        renderer.set_fill_style("black");
        renderer.stroke_rect(self.position.x, self.position.y, 50.0, 50.0);
        renderer.fill_rect(self.position.x, self.position.y, 50.0, 50.0);
    } //^-- draw
}
