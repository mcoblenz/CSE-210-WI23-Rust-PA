#[derive(Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn new_red() -> Color {
        Color::new(255, 0, 0)
    }

    pub fn new_green() -> Color {
        Color::new(0, 255, 0)
    }

    pub fn new_blue() -> Color {
        Color::new(0, 0, 255)
    }

    /**
     * Returns a new `Color` whose components are the sum of `c1` and `c2`'s components, modulo 256.
     *
     * First, try writing this function the "obvious" way with arithmetic operations. The test for
     * this method (which you can run with `cargo test part1_color` will fail) with a panic.
     *
     * Note which line of the test is causing the panic: why not the other?
     *
     * Then, look through the documentation for `u8` and see if there is a method that will help you.
     * https://doc.rust-lang.org/std/primitive.u8.html
     */
    pub fn cross(c1: &Color, c2: &Color) -> Color {
        // unimplemented!();
        //let red = (c1.r + c2.r).rem_euclid(u8::MAX);
        //let green = (c1.g + c2.g).rem_euclid(u8::MAX);
        //let blue = (c1.b + c2.b).rem_euclid(u8::MAX);
        let red = c1.r.overflowing_add(c2.r).0;
        let green = c1.g.overflowing_add(c2.g).0;
        let blue = c1.b.overflowing_add(c2.b).0;
        Color::new(red,green,blue)
    }
}
