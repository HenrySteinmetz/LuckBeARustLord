use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod slot_machine;
use crate::slot_machine::item_functions::add_item;
use crate::slot_machine::*;

mod renderer;
use renderer::Renderer;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("lbal", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(window)?;
    let (mut slot_machine, mut items) = SlotMachine::new();
    
    add_item(Item::Wildcard, &mut items);
    add_item(Item::Wildcard, &mut items);
    add_item(Item::Coin, &mut items);
    add_item(Item::Diamond, &mut items);

    let temp = slot_machine.calculate(items);
    let money = temp.0;
    items = temp.1;
    slot_machine = temp.2;

    println!("{}", money);

    'mainloop: loop {
        renderer.render(items.clone())?;
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {keycode: Option::Some(Keycode::Space),..} => (),
                Event::Quit {..} => break 'mainloop,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
