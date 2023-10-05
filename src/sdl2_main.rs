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
    // Has to be selecting in the future
    let mut game_state = slot_machine::State::Normal;

    // Sdl init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("lbal", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut sdl_renderer = Renderer::new(window, 1920, 1080)?;

    for _ in 0..7 {
        items = add_item(Item::Clubs, items);
        items = add_item(Item::Diamonds, items);
    }

    items = roll(items);
    let (money, mut items, mut slot_machine) = slot_machine.calculate(items);
    
    'mainloop: loop {
        sdl_renderer.render(items.clone(), money, game_state)?;
            for event in sdl_context.event_pump()?.poll_iter() {
                match game_state {
                    State::Normal => {
                        match event {
                            Event::KeyDown {keycode: Some(Keycode::Space),..} => {
                                items = roll(items.clone());
                                let (money, items, _slot_machine) = slot_machine.calculate(items.clone());
                                println!("{:?}", money);
                                sdl_renderer.render(items.clone(), money, game_state)?;
                            },
                            Event::KeyDown {keycode: Some(Keycode::Escape),..} => game_state = State::Paused,
                            Event::Quit {..} => break 'mainloop,
                            _ => {}
                        }
                    }

                    State::Paused => {
                        match event {
                            Event::KeyDown {keycode: Some(Keycode::Escape),..} => game_state = State::Normal,
                            Event::Quit {..} => break 'mainloop,
                            _ => {}
                        }
                    }

                    State::Selecting => {
                        match event {
                            Event::Quit {..} => break 'mainloop,
                            _ => {}
                        }
                    }
                }

            }
        }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    Ok(())
}
