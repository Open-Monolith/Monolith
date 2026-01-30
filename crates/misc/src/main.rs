use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;
#[cfg(test)]
mod tests {
  use super::*;
  use bevy::time::{ TimeUpdateStrategy};

  fn hello_world() {
    println!("Hello World!");
  }

  #[test]
  fn text_fixed_game_loop_right() {
    let mut app = App::new();
    app.insert_resource(TimeUpdateStrategy::ManualDuration(
      Duration::from_secs(1),
    ));

    app.add_systems(FixedUpdate, hello_world);

    // Now an update will advance the time by 1 second,
    // which is enough to trigger the fixed timestep system.
    app.update();
  }
}
