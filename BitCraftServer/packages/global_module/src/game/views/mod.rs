use crate::messages::components::{
    previous_empire_name_state__view, previous_player_skills_state__view, previous_player_username_state__view, PreviousEmpireNameState,
    PreviousPlayerSkillsState, PreviousPlayerUsernameState,
};

use spacetimedb::{view, ViewContext};

#[view(name = previous_username, public)]
pub fn previous_username(ctx: &ViewContext) -> Vec<PreviousPlayerUsernameState> {
    let mut previous_username = Vec::new();
    if let Some(ppus) = ctx.db.previous_player_username_state().identity().find(ctx.sender) {
        previous_username.push(ppus);
    }
    previous_username
}

#[view(name = previous_empire_name, public)]
pub fn previous_empire_name(ctx: &ViewContext) -> Vec<PreviousEmpireNameState> {
    let mut previous_username = Vec::new();
    if let Some(pens) = ctx.db.previous_empire_name_state().emperor_identity().find(ctx.sender) {
        previous_username.push(pens);
    }
    previous_username
}

#[view(name = previous_player_skills, public)]
pub fn previous_player_skills(ctx: &ViewContext) -> Vec<PreviousPlayerSkillsState> {
    let mut previous_player_skills = Vec::new();
    if let Some(ppss) = ctx.db.previous_player_skills_state().identity().find(ctx.sender) {
        previous_player_skills.push(ppss);
    }
    previous_player_skills
}
