use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod slot_machine;
use crate::calculate::add_item;
use crate::calculate::roll;
use slot_machine::*;

mod renderer;
use renderer::Renderer;

fn main() -> Result<(), String> {
    // Gamestate init
    let (mut slot_machine, mut items) = SlotMachine::new();
    let mut game_state = slot_machine::State::Selecting;

    // Sdl init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("lbal", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut sdl_renderer = Renderer::new(window)?;




    for _ in 0..7 {
        items = add_item(Item::Monkey, items.clone());
        items = add_item(Item::CoconutHalf, items.clone());
        items = add_item(Item::Wildcard, items.clone());
    }

    items = roll(items);
    let (mut money, mut items, slot_machine) = slot_machine.calculate(items);
    
    'mainloop: loop {
        sdl_renderer.render(items.clone(), money)?;
            for event in sdl_context.event_pump()?.poll_iter() {
                match event {
                    Event::KeyDown {keycode: Option::Some(Keycode::Space),..} => {
                        items = roll(items);
                        sdl_renderer.render(items.clone(), money)?;
                    },
                    Event::Quit {..} => break 'mainloop,
                    _ => {}
                }
            }
        }
/*        match game_state {
            State::Normal => {
                renderer.render(items.clone(), money)?;
                for event in sdl_context.event_pump()?.poll_iter() {
                    match event {
                        Event::KeyDown {keycode: Option::Some(Keycode::Space),..} => (),
                        Event::Quit {..} => break 'mainloop,
                        _ => {}
                    }
                }
            }
            State::Selecting => {
                todo!();
            }
            State::Paused => {
                todo!();
            }
        }
    }
*/
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    Ok(())
}
