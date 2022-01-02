// Copyright 2017-2019 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gate::asset_id::*;
use std::mem;

pub struct AssetId;

impl AppAssetId for AssetId {
    type Sprite = SpriteId;
    type Music = MusicId;
    type Sound = SoundId;
}


// Copyright 2017-2019 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[repr(u8)]
pub enum SpriteId {
    BgTileR0C0,
    BgTileR0C1,
    Disc0,
    Disc1,
    Disc2,
    Disc3,
    Disc4,
    Pillars,
}

impl IdU16 for SpriteId {
    fn id_u16(self) -> u16 { self as u16 }
    fn count() -> u16 { 8 }
    fn from_u16(id: u16) -> Option<Self> {
        if id < Self::count() { Some(unsafe { mem::transmute(id as u8) }) } else { None }
    }
}


// Copyright 2017-2019 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[repr(u8)]
pub enum MusicId {
    Tick,
}

impl IdU16 for MusicId {
    fn id_u16(self) -> u16 { self as u16 }
    fn count() -> u16 { 1 }
    fn from_u16(id: u16) -> Option<Self> {
        if id < Self::count() { Some(unsafe { mem::transmute(id as u8) }) } else { None }
    }
}


// Copyright 2017-2019 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[repr(u8)]
pub enum SoundId {
    Error,
    Shuffle,
}

impl IdU16 for SoundId {
    fn id_u16(self) -> u16 { self as u16 }
    fn count() -> u16 { 2 }
    fn from_u16(id: u16) -> Option<Self> {
        if id < Self::count() { Some(unsafe { mem::transmute(id as u8) }) } else { None }
    }
}

