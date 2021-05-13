use amethyst::{
    input::{InputHandler, StringBindings},
    derive::SystemDesc,
    ecs::{Read, System, SystemData},
};

#[derive(SystemDesc)]
pub struct InputSystem;

impl <'s> System<'s> for InputSystem {

    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self:: SystemData) {
        if let Some((x, y)) = input.mouse_position() {
            //println!("{} {}", x, y);
        }

        if input.action_is_down("Quit").unwrap_or(false) {
            std::process::exit(0);
        }
    }
}
