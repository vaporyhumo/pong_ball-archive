use sdl2::rect::Rect;

use crate::{ball::Ball, BLACK, BLOCK_SIZE, client::Client};

pub fn draw_ball(client: &mut Client, ball: Ball) -> Result<(), String> {
  client.canvas.set_draw_color(BLACK);
  client.canvas.fill_rect(Rect::new(
  ball.x() as i32,
  ball.y() as i32,
  BLOCK_SIZE as u32,
  BLOCK_SIZE as u32,
    ))?;
  Ok(())
}
