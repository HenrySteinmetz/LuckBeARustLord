use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod slot_machine;
use slot_machine::SlotMachine;
use slot_machine::State;
use slot_machine::Item;

mod renderer;
use renderer::Renderer;
use renderer::Point;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("lbal", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(window)?;
    let (mut _slot_machine, mut items) = SlotMachine::new();
    
    'mainloop: loop {
        renderer.render(items.clone())?;
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {keycode: Option::Some(Keycode::Space),..} => (),
                Event::Quit {..}|Event::KeyDown {keycode: Option::Some(Keycode::Escape),..} => break 'mainloop,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
