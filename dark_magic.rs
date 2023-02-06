struct DarkMagic {
    x: f32,
    y: f32,
    z: f32,
    damage: i32,
    range: f32,
}

impl DarkMagic {
    fn new(x: f32, y: f32, z: f32) -> DarkMagic {
        DarkMagic {
            x,
            y,
            z,
            damage: 75,
            range: 10.0,
        }
    }

    fn cast(&self) {
        println!("Casting dark magic at ({}, {}, {}) with damage {} and range {}", self.x, self.y, self.z, self.damage, self.range);
    }
}

fn main() {
    let dark_magic = DarkMagic::new(0.0, 0.0, 0.0);
    dark_magic.cast();
}
