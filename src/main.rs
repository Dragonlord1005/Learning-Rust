use std::io

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World,
}

#[allow(unused)]
fn main() {
    println!("Guess the number!")

    println!("Please input your guess")
}
