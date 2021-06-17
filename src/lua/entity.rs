use bevy::prelude::*;
use enumset::*;
use rlua::prelude::*;

use crate::lua::types::*;
use crate::lua::util::*;

#[derive(Debug, EnumSetType)]
pub enum EntityEvent {
    OnInit,
    OnUpdate,
}

impl EntityEvent {
    pub fn from_string(str: &str) -> Result<EntityEvent, &str> {
        match str {
            "on_init"   => Ok(EntityEvent::OnInit),
            "on_update" => Ok(EntityEvent::OnUpdate),
            s           => Err(s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LuaEntity {
    pub entity: Entity,
    pub events_registered: EnumSet<EntityEvent>,
}

impl LuaEntity {
    pub const LUA_ENTITY_NAME: &'static str        = "local_entity";
    pub const ENTITY_EVENTS_VAR_NAME: &'static str = "_E_EVT";
    pub const ENTITY_EVENT_COUNTER: &'static str   = "_E_CTR";

    pub fn new(entity: Entity) -> LuaEntity {
        LuaEntity {
            entity,
            events_registered: EnumSet::default(),
        }
    }

    pub fn update_entity(&self, commands: &mut Commands) {
        commands.entity(self.entity)
            .insert(self.events_registered);
    }
    
    fn next_id(lua_ctx: LuaContext) -> LuaResult<i32> {
        if lua_ctx.globals().contains_key(LuaEntity::ENTITY_EVENT_COUNTER)? {
            let next_id = lua_ctx.globals().get(LuaEntity::ENTITY_EVENT_COUNTER)?;
            lua_ctx.globals().set(LuaEntity::ENTITY_EVENT_COUNTER, next_id + 1)?;
            Ok(next_id)
        } else {
            lua_ctx.globals().set(LuaEntity::ENTITY_EVENT_COUNTER, 1)?;
            Ok(0)
        }
    }
}

impl LuaInstance for LuaEntity {
    fn init(self, lua_ctx: LuaContext) -> LuaResult<()> {
        lua_ctx.globals().set(LuaEntity::LUA_ENTITY_NAME, self)
    }

    fn finalize(lua_ctx: LuaContext) -> LuaResult<Self> {
        let ety = lua_ctx.globals().get(LuaEntity::LUA_ENTITY_NAME)?;
        lua_ctx.globals().set(LuaEntity::LUA_ENTITY_NAME, None::<LuaEntity>)?;
        Ok(ety)
    }
}

impl LuaEvent<EntityEvent> for LuaEntity {
    fn run_handlers(&self, lua_ctx: LuaContext, event: EntityEvent) -> LuaResult<()> {
        if let Some(handlers) = get_if_present(&lua_ctx.globals(), LuaEntity::ENTITY_EVENTS_VAR_NAME)?
            .and_then(|t: LuaTable| get_if_present(&t, self.entity.id()).unwrap())
            .and_then::<LuaTable, _>(|t: LuaTable| get_if_present(&t, event as u8).unwrap()) {
            for pair in handlers.pairs::<i32, LuaFunction>() {
                if let Ok((_, f)) = pair {
                    f.call::<_, ()>(())?;
                } else {
                    println!("Error in LuaEntity event handler for {:?}: {:?}", event, pair);
                }
            }
        }
        Ok(())
    }
}

impl LuaUserData for LuaEntity {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("id", |_, this, ()| {
            Ok(this.entity.id())
        });
        methods.add_method_mut("register", |lua_ctx, this, table: LuaTable| {
            let new_id = LuaEntity::next_id(lua_ctx)?;
            for pair in table.pairs::<String, LuaFunction>() {
                if let Ok((event_key, f)) = pair {
                    let event = EntityEvent::from_string(&event_key).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
                    this.events_registered = this.events_registered | event;

                    let entities = compute_if_absent(&lua_ctx.globals(), LuaEntity::ENTITY_EVENTS_VAR_NAME, || lua_ctx.create_table())?;
                    let events   = compute_if_absent(&entities, this.entity.id(), || lua_ctx.create_table())?;
                    let handlers = compute_if_absent(&events, event as u8, || lua_ctx.create_table())?;
                    handlers.set(new_id, f)?;
                } else {
                    println!("failed to register event {:?}", pair);
                }
            }
            Ok(new_id)
        });
    }
}