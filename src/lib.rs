#![allow(clippy::type_complexity)]
#![doc = include_str!("../README.MD")]

pub mod anim;
pub mod loader;

use anim::AsepriteAnimation;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use bevy_aseprite_reader as reader;

pub use bevy::sprite::TextureAtlasBuilder;
pub use bevy_aseprite_derive::aseprite;
use reader::AsepriteInfo;

pub struct AsepritePlugin;

#[derive(Debug, SystemSet, Clone, Hash, PartialEq, Eq)]
pub enum AsepriteSystems {
    InsertSpriteSheet,
}

impl Plugin for AsepritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Aseprite>()
            .add_asset_loader(loader::AsepriteLoader)
            .add_system(loader::process_load)
            .add_system(loader::insert_sprite_sheet.in_set(AsepriteSystems::InsertSpriteSheet));
            // .add_system(anim::update_animations.after(AsepriteSystems::InsertSpriteSheet));
    }
}

// pub fn update_animations(
    // time: Res<Time>,
    // aseprites: Res<Assets<Aseprite>>,
    // mut aseprites_query: Query<(
        // &Handle<Aseprite>,
        // &mut AsepriteAnimation,
        // &mut TextureAtlasSprite,
    // )>,
// ) {
    // for (handle, mut animation, mut sprite) in aseprites_query.iter_mut() {
        // let aseprite = match aseprites.get(handle) {
            // Some(aseprite) => aseprite,
            // None => {
                // error!("Aseprite handle is invalid");
                // continue;
            // }
        // };
        // let info = match &aseprite.info {
            // Some(info) => info,
            // None => {
                // error!("Aseprite info is None");
                // continue;
            // }
        // };
        // if animation.update(info, time.delta()) {
            // sprite.index = aseprite.frame_to_idx[animation.current_frame];
        // }
    // }
// }

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "b29abc81-6179-42e4-b696-3a5a52f44f73"]
pub struct Aseprite {
    // Data is dropped after the atlas is built
    pub data: Option<reader::Aseprite>,
    // Info stores data such as tags and slices
    pub info: Option<AsepriteInfo>,
    // TextureAtlasBuilder might shift the index order when building so
    // we keep a mapping of frame# -> atlas index here
    pub frame_to_idx: Vec<usize>,
    // Atlas that gets built from the frame info of the aseprite file
    atlas: Option<Handle<TextureAtlas>>,
}

/// A bundle defining a drawn aseprite
#[derive(Debug, Bundle, Default)]
pub struct AsepriteBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub animation: AsepriteAnimation,
    pub aseprite: Handle<Aseprite>,
}
