use spacetimedb::{log, ReducerContext, Table};

use crate::{
    agents,
    game::{game_state, reducer_helpers::timer_helpers::now_plus_secs},
    messages::{
        authentication::ServerIdentity,
        components::{signed_in_player_state, TravelerTaskState},
    },
    parameters_desc,
};

const SECONDS_IN_A_DAY: i32 = 24 * 60 * 60;

#[spacetimedb::table(name = traveler_task_loop_timer, public, scheduled(traveler_task_agent_loop, at = scheduled_at))]
pub struct TravelerTaskLoopTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,
}

pub fn next_tick(ctx: &ReducerContext) -> i32 {
    let seconds_elapsed = game_state::unix(ctx.timestamp);
    let start_of_current_day_timestamp = (seconds_elapsed / SECONDS_IN_A_DAY) * SECONDS_IN_A_DAY;

    let mut next_tick = None;

    for tick_time_of_day in &ctx.db.parameters_desc().version().find(&0).unwrap().traveler_tasks_times_of_day {
        let daily_timestamp_tick = *tick_time_of_day * 60 * 60;
        let next_tick_attempt = start_of_current_day_timestamp + daily_timestamp_tick;
        if next_tick_attempt >= seconds_elapsed {
            next_tick = Some(next_tick_attempt);
            break;
        }
        if next_tick.is_none() {
            next_tick = Some(next_tick_attempt + SECONDS_IN_A_DAY);
        }
    }
    next_tick.unwrap()
}

pub fn schedule_next_tick(ctx: &ReducerContext) {
    let seconds_elapsed = game_state::unix(ctx.timestamp);
    let next_tick = next_tick(ctx);
    let time_until_next_day = next_tick - seconds_elapsed;
    ctx.db
        .traveler_task_loop_timer()
        .try_insert(TravelerTaskLoopTimer {
            scheduled_id: 0,
            scheduled_at: now_plus_secs(time_until_next_day as u64, ctx.timestamp),
        })
        .ok()
        .unwrap();
}

pub fn init(ctx: &ReducerContext) {
    // Schedule instantly an agent tick - we need the ctx to come from a server call rather than the spacetimedb::init() call
    ctx.db
        .traveler_task_loop_timer()
        .try_insert(TravelerTaskLoopTimer {
            scheduled_id: 0,
            scheduled_at: now_plus_secs(0, ctx.timestamp),
        })
        .ok()
        .unwrap();
}

#[spacetimedb::reducer]
fn traveler_task_agent_loop(ctx: &ReducerContext, _timer: TravelerTaskLoopTimer) {
    if ServerIdentity::validate_server_or_admin(&ctx).is_err() {
        log::error!("Unauthorized access to traveler task agent");
        return;
    }

    schedule_next_tick(ctx);

    if !agents::should_run(ctx) {
        return;
    }

    let tasks_per_npc = ctx.db.parameters_desc().version().find(0).unwrap().traveler_tasks_per_npc;
    let next_task_refresh = next_tick(ctx);

    let requests = TravelerTaskState::generate_npc_requests_hashmap(ctx);

    // Update every signed in player tasks
    for signed_in_player in ctx.db.signed_in_player_state().iter() {
        let player_entity_id = signed_in_player.entity_id;
        TravelerTaskState::delete_all_for_player(ctx, player_entity_id);
        TravelerTaskState::generate_all_for_player(ctx, player_entity_id, &requests, tasks_per_npc, next_task_refresh);
    }
}
