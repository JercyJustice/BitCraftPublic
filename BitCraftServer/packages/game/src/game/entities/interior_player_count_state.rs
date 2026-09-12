use spacetimedb::{ReducerContext, Table};

use crate::messages::components::{interior_player_count_state, DimensionNetworkState, InteriorPlayerCountState};

impl InteriorPlayerCountState {
    pub fn create(ctx: &ReducerContext, dimension_network_state: &DimensionNetworkState) {
        ctx.db.interior_player_count_state().insert(InteriorPlayerCountState {
            entity_id: dimension_network_state.building_id,
            dimension_network_entity_id: dimension_network_state.entity_id,
            player_count: 0,
        });
    }

    pub fn inc(ctx: &ReducerContext, dimension_network_entity_id: u64) {
        if let Some(mut c) = ctx
            .db
            .interior_player_count_state()
            .dimension_network_entity_id()
            .find(dimension_network_entity_id)
        {
            c.player_count += 1;
            ctx.db.interior_player_count_state().entity_id().update(c);
        }
    }

    pub fn dec(ctx: &ReducerContext, dimension_network_entity_id: u64) {
        if let Some(mut c) = ctx
            .db
            .interior_player_count_state()
            .dimension_network_entity_id()
            .find(dimension_network_entity_id)
        {
            if c.player_count > 0 {
                c.player_count -= 1;
                ctx.db.interior_player_count_state().entity_id().update(c);
            }
        }
    }

    pub fn reset(ctx: &ReducerContext, dimension_network_entity_id: u64) {
        if let Some(mut c) = ctx
            .db
            .interior_player_count_state()
            .dimension_network_entity_id()
            .find(dimension_network_entity_id)
        {
            if c.player_count != 0 {
                c.player_count = 0;
                ctx.db.interior_player_count_state().entity_id().update(c);
            }
        }
    }
}
