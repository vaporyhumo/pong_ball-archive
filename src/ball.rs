use crate::{BLOCK_SIZE, GAME_HEIGHT, GAME_WIDTH};

#[derive(Copy, Clone)]
pub struct Ball {
  pub x: usize,
  pub y: usize,
  pub x_vel: isize,
  pub y_vel: isize,
}

impl Ball {
  pub fn new() -> Self {
    Ball {
      x: 0,
      y: 0,
      x_vel: 1,
      y_vel: 1,
    }
  }
  pub fn x(&self) -> usize {
    self.x
  }
  pub fn y(&self) -> usize {
    self.y
  }

  fn bottom(&self) -> usize {
    self.y + BLOCK_SIZE
  }

  fn right(&self) -> usize {
    self.x + BLOCK_SIZE
  }

  fn top(&self) -> usize {
    self.y
  }

  fn left(&self) -> usize {
    self.x
  }

  pub fn change_dir(&self) -> Self {
    let mut ball: Ball = self.to_owned();
    if self.bottom() == GAME_HEIGHT {
      ball = Ball { y_vel: -1, ..ball };
    }

    if self.right() == GAME_WIDTH {
      ball = Ball { x_vel: -1, ..ball };
    }

    if self.top() == 0 {
      ball = Ball { y_vel: 1, ..ball };
    }

    if self.left() == 0 {
      ball = Ball { x_vel: 1, ..ball };
    }

    ball
  }

  pub fn next(&self) -> Self {
    Ball {
      x: ((self.x as isize) + self.x_vel) as usize,
      y: ((self.y as isize) + self.y_vel) as usize,
      ..self.to_owned()
    }
  }
}
