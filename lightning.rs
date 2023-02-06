struct Lightning {
    x: f32,
    y: f32,
    z: f32,
    damage: i32,
}

impl Lightning {
    fn new(x: f32, y: f32, z: f32) -> Lightning {
        Lightning {
            x,
            y,
            z,
            damage: 50,
        }
    }

    fn cast(&self) {
        println!("Casting lightning at ({}, {}, {}) with damage {}", self.x, self.y, self.z, self.damage);
    }
}

fn main() {
    let lightning = Lightning::new(0.0, 0.0, 0.0);
    lightning.cast();
}
