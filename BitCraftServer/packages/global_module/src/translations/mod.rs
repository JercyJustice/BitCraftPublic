use crate::game::{game_state, handlers::authentication::has_role};
use crate::messages::authentication::Role;
use crate::messages::components::*;
use spacetimedb::{self, ReducerContext, Table};

#[spacetimedb::table(name = official_translators, public)]
pub struct OfficialTranslators {
    pub lang: String,
    #[index(btree)]
    pub player_entity_id: u64,
}

#[spacetimedb::table(name = translation_corrections, public)]
pub struct TranslationCorrections {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub string_index: u32,
    pub player_entity_id: u64,
    #[index(btree)]
    pub lang: String,
    pub translation: String,
}

#[spacetimedb::reducer]
pub fn correct_translation(ctx: &ReducerContext, string_index: u32, lang: String, translation: String) -> Result<(), String> {
    // insert or update translation if the caller is an official translator for the lang
    let actor_id = game_state::actor_id(&ctx, true)?;
    if !ctx
        .db
        .official_translators()
        .player_entity_id()
        .filter(&actor_id)
        .any(|ot| ot.lang == lang)
    {
        return Err("Unauthorized".into());
    }

    let table = ctx.db.translation_corrections();
    if let Some(mut entry) = table.string_index().filter(&string_index).find(|t| t.lang == lang) {
        entry.translation = translation;
        table.id().update(entry);
    } else {
        let new_entry = TranslationCorrections {
            id: 0,
            player_entity_id: actor_id,
            string_index,
            lang,
            translation,
        };
        table.insert(new_entry);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn add_official_translator(ctx: &ReducerContext, username: String, lang: String) -> Result<(), String> {
    if !has_role(ctx, &ctx.sender, Role::Admin) {
        return Err("Unauthorized".into());
    }

    if let Some(player) = ctx
        .db
        .player_lowercase_username_state()
        .username_lowercase()
        .find(&username.to_lowercase())
    {
        let new_entry = OfficialTranslators {
            lang,
            player_entity_id: player.entity_id,
        };
        ctx.db.official_translators().insert(new_entry);
    } else {
        return Err("Player not found".into());
    }

    Ok(())
}
