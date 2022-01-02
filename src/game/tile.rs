use gate::{renderer::{SpriteRenderer, Affine}};

use crate::asset_id::SpriteId;
use crate::asset_id::AssetId;

pub struct Tile {
  pub id: char,
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

impl Tile {
  pub fn new(id: char, x: f64, y: f64, width: f64, height: f64) -> Tile {
    Tile {
      id: id,
      x: x,
      y: y,
      width: width,
      height: height
    }
  }

  pub fn draw(&self, renderer: &mut SpriteRenderer<AssetId>, tile_size: f64, height: f64) {
    if self.id == '0' {
      return;
    }

    let affine = &Affine::translate((self.x * tile_size) + 10., height - (self.y * tile_size) - 10.);
    match self.id {
      '1' => renderer.draw(affine, SpriteId::Tile1),
      '2' => renderer.draw(affine, SpriteId::Tile2),
      _ => renderer.draw(affine, SpriteId::Tile1)
    }
  }
}