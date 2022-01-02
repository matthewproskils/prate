use crate::game::map::Map;
use crate::{
    asset_id::{AssetId, SpriteId},
    Affine,
};
use gate::{renderer::SpriteRenderer, AppContext};


pub fn grid(map: &Map, app_width: f64, app_height: f64, renderer: &mut SpriteRenderer<AssetId>) {
    let tileSize: f64 = app_width / map.width as f64;
    for (x, row) in map.tiles.iter().enumerate() {
      for (y, tile) in row.iter().enumerate() {
        tile.draw(renderer, tileSize, app_height);
      }
    }
}

pub fn bg (renderer: &mut SpriteRenderer<AssetId>, ctx: &AppContext<AssetId>) {
  let (app_width, app_height) = ctx.dims();

  let x_off = 0.5 * app_width;
  let y_off = 0.5 * app_height;

  let affine = &Affine::translate(x_off, y_off).pre_scale(2.);
  renderer.draw(affine, SpriteId::Ground);
}