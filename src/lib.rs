use robotics_lib::{event::events::Event, world::World};

pub trait RunnableUi {
    fn process_tick(&mut self, world: &mut World);
    fn handle_event(&mut self, event: Event);
}
