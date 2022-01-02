use crate::game::tile::Tile;
use gate::{renderer::{Affine, SpriteRenderer}, AppContext};

use crate::asset_id::{SpriteId, AssetId};


pub struct Player {
  pub x: f64,
  pub y: f64,
  pub speed: f64,
  pub action: i8, // 0 = Jump, 1 = Run, 2 = Run, 3 = idle (Gun)
  pub index: i8,
  pub index_timeout: i16,
  offx: f64,
  offy: f64
}

impl Player {
  pub fn new(x: f64, y: f64, speed: f64) -> Player {
    Player {
      x: x,
      y: y,
      offx: 0.0,
      offy: 0.0,
      speed: speed,
      action: 3,
      index: 1,
      index_timeout: 500
    }
  }

  fn chicken_jump(index: i8) -> SpriteId {
    match index {
      1 => SpriteId::Chicken1,
      2 => SpriteId::Chicken2,
      3 => SpriteId::Chicken3,
      4 => SpriteId::Chicken4,
      5 => SpriteId::Chicken5,
      _ => SpriteId::Chicken1
    }
  }

  fn chicken_run(index: i8) -> SpriteId {
    match index {
      1 => SpriteId::Chicken6,
      2 => SpriteId::Chicken7,
      3 => SpriteId::Chicken8,
      4 => SpriteId::Chicken9,
      5 => SpriteId::Chicken10,
      6 => SpriteId::Chicken11,
      _ => SpriteId::Chicken6
    }
  }

  fn chicken_run_gun(index: i8) -> SpriteId {
    match index {
      1 => SpriteId::Chicken12,
      2 => SpriteId::Chicken13,
      3 => SpriteId::Chicken14,
      4 => SpriteId::Chicken15,
      5 => SpriteId::Chicken16,
      6 => SpriteId::Chicken17,
      _ => SpriteId::Chicken12
    }
  }

  fn chicken_idle(index: i8) -> SpriteId {
    match index {
      1 => SpriteId::Chicken1,
      2 => SpriteId::Chicken2,
      _ => SpriteId::Chicken1
    }
  }

  fn chicken(action: i8, index: i8) -> SpriteId {
    match action {
      0 => Player::chicken_jump(index),
      1 => Player::chicken_run(index),
      2 => Player::chicken_run_gun(index),
      3 => Player::chicken_idle(index),
      _ => SpriteId::Chicken1
    }
  }

  pub fn draw(&self, renderer: &mut SpriteRenderer<AssetId>) {
    let affine = &Affine::translate(self.x + self.offx, self.y + self.offy).pre_scale(2.);
    renderer.draw(affine, Player::chicken(self.action, self.index));
  }

  pub fn collides_with_map(&self, map: &Vec<Vec<Tile>>) -> bool {
    for i in map.iter().flatten() {
      if (i.x < self.x + self.speed) && (i.x + i.width > self.x - self.speed) && (i.y < self.y + self.speed) && (i.y + i.height > self.y - self.speed) {
        return true
      }
    }
    false
  }

  pub fn gravity(&mut self, map: &Vec<Vec<Tile>>) {
    if !self.collides_with_map(map) {
      self.y += self.speed;
    }
  }

  pub fn set_offset(&mut self, ctx: &AppContext<AssetId>) {
    let (_w, h) = ctx.dims();
    self.offx = 10.;
    self.offy = h - 10.;
  }
}
