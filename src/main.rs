mod ball;
mod client;
mod draw_ball;

use std::time::Duration;

use ball::Ball;
use client::Client;
use draw_ball::draw_ball;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

const GAME_WIDTH: usize = 800;
const GAME_HEIGHT: usize = 600;
const BLOCK_SIZE: usize = 10;
const WHITE: Color = Color::RGBA(255, 255, 255, 255);
const BLACK: Color = Color::RGBA(0, 0, 0, 0);

fn main() -> Result<(), String> {
  let mut client: Client = Client::new()?;
  let mut ball: Ball = Ball::new();

  draw_ball(&mut client, ball)?;
  client.canvas.present();

  'mainloop: loop {
    client.canvas.set_draw_color(WHITE);
    client.canvas.clear();

    for event in client.event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'mainloop,
        _ => {}
      }
    }

    ball = ball.change_dir();
    ball = ball.next();

    draw_ball(&mut client, ball)?;

    client.canvas.present();
    std::thread::sleep(Duration::from_millis(1));
  }
  Ok(())
}
