struct Fireball {
    x: f32,
    y: f32,
    z: f32,
    velocity: f32,
}

impl Fireball {
    fn new(x: f32, y: f32, z: f32) -> Fireball {
        Fireball {
            x,
            y,
            z,
            velocity: 10.0,
        }
    }

    fn update(&mut self) {
        self.z += self.velocity;
    }

    fn render(&self) {
        println!("Rendering fireball at ({}, {}, {})", self.x, self.y, self.z);
    }
}

fn main() {
    let mut fireball = Fireball::new(0.0, 0.0, 0.0);

    for _ in 0..10 {
        fireball.update();
        fireball.render();
    }
}
