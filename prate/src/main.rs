#[macro_use]
extern crate gate;
extern crate collider;

mod game;

gate_header!();

mod asset_loader;
mod asset_id {
    include!(concat!(env!("OUT_DIR"), "/asset_id.rs"));
}

use game::map;
use gate::renderer::{Affine, Renderer};
use gate::{App, AppContext, AppInfo, KeyCode};

use crate::game::bg;
use crate::game::player;

use crate::asset_id::AssetId;

fn main() {
    // TODO allow some flexibility in the app height
    let info = AppInfo::with_max_dims(192., 192.)
        .min_dims(256., 192.)
        .tile_width(8)
        .title("Prate");

    gate::run(info, Prate::new());
}

struct Prate {
    map: map::Map,
    player: player::Player,
}

impl Prate {
    pub fn new() -> Prate {
        Prate {
            map: map::load_map("lvl_1_map.txt"),
            player: player::Player::new(0., 0., 0.),
        }
    }
}

impl App<AssetId> for Prate {
    fn start(&mut self, ctx: &mut AppContext<AssetId>) {
      self.player.set_offset(ctx)
    }

    fn advance(&mut self, _seconds: f64, _ctx: &mut AppContext<AssetId>) {
        self.player.gravity(&self.map.tiles);
    }

    fn key_down(&mut self, _key: KeyCode, _: &mut AppContext<AssetId>) {}

    fn key_up(&mut self, _key: KeyCode, _: &mut AppContext<AssetId>) {}

    fn render(&mut self, renderer: &mut Renderer<AssetId>, ctx: &AppContext<AssetId>) {
        let mut renderer = renderer.sprite_mode();
        let (app_width, app_height) = ctx.dims();

        bg::bg(&mut renderer, ctx);

        bg::grid(&self.map, app_width, app_height, &mut renderer);

        self.player.draw(&mut renderer);
    }
}
