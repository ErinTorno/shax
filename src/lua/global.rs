use rlua::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::lua::util::*;

#[derive(Clone, Default)]
pub struct Global {
    pub counter: usize,
    pub turn_count: usize,
    pub is_debug: bool,
    pub var_watchers: HashMap<String, HashSet<usize>>,
}

impl Global {
    pub const GLOBAL_VAR_NAME: &'static str   = "global";
    pub const VAR_VALUES_VAR_NAME: &'static str   = "_G_VAL";
    pub const VAR_HANDLERS_VAR_NAME: &'static str = "_G_HDL";
    pub const EVENTS_VAR_NAME: &'static str       = "_G_EVT";

    pub fn init(lua_ctx: LuaContext) -> LuaResult<Global> {
        let vars     = lua_ctx.create_table()?;
        let handlers = lua_ctx.create_table()?;
        let events   = lua_ctx.create_table()?;
        let global = Global {is_debug: true, ..Global::default()};
        lua_ctx.globals().set(Global::VAR_VALUES_VAR_NAME, vars)?;
        lua_ctx.globals().set(Global::VAR_HANDLERS_VAR_NAME, handlers)?;
        lua_ctx.globals().set(Global::EVENTS_VAR_NAME, events)?;
        lua_ctx.globals().set(Global::GLOBAL_VAR_NAME, global.clone())?;
        Ok(global)
    }

    pub fn next_id(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}

impl LuaUserData for Global {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("next_id", |_, this, ()| {
            Ok(this.next_id())
        });
        methods.add_method_mut("register", |lua_ctx, this, table: LuaTable| {
            let new_id = this.next_id();
            for pair in table.pairs::<String, LuaFunction>() {
                if let Ok((event_key, f)) = pair {
                    let events   = compute_if_absent(&lua_ctx.globals(), Global::EVENTS_VAR_NAME, || lua_ctx.create_table())?;
                    let handlers = compute_if_absent(&events, event_key.as_str(), || lua_ctx.create_table())?;
                    handlers.set(new_id, f)?;
                } else {
                    println!("Failed to register event {:?}", pair);
                }
            }
            Ok(new_id)
        });
        // Debug
        methods.add_method("is_debug", |_, this, ()| {
            Ok(this.is_debug)
        });
        methods.add_method("log", |_, this, s: String| {
            if this.is_debug {
                println!("lua => {}", s); // TODO use something else, log to file maybe?
            }
            Ok(())
        });
        // Getters and Setters
        methods.add_method("player", |_, _, _idx: i32| {
            Err::<(), LuaError>(LuaError::RuntimeError("global:player(idx) not implemented".to_string()))
        });
        methods.add_method("turn_count", |_, this, ()| {
            Ok(this.turn_count)
        });
        // Variables
        methods.add_method("get", |lua_ctx, _, key: String| {
            let vars: LuaTable = lua_ctx.globals().get(Global::VAR_VALUES_VAR_NAME).expect("Missing global var values table");
            vars.get::<_, LuaValue>(key)
        });
        methods.add_method("set", |lua_ctx, this, (key, val): (String, LuaValue)| {
            let vars: LuaTable = lua_ctx.globals().get(Global::VAR_VALUES_VAR_NAME).expect("Missing global var values table");
            if let Some(handler_ids) = this.var_watchers.get(&key) {
                let old_value: LuaValue = vars.get(key.clone())?;
                let handlers: LuaTable = lua_ctx.globals().get(Global::VAR_HANDLERS_VAR_NAME)?;
                let table: LuaTable = handlers.get(key.clone())?;
                for id in handler_ids {
                    table.get::<_, LuaFunction>(id.clone()).unwrap().call::<_, ()>((old_value.clone(), val.clone()))?;
                }
            }
            vars.set(key, val).unwrap();
            Ok(())
        });
        methods.add_method_mut("unwatch", |lua_ctx, this, (key, handler_id): (String, usize)| {
            if let Some(set) = this.var_watchers.get_mut(&key) {
                set.remove(&handler_id);
                let handlers: LuaTable = lua_ctx.globals().get(Global::VAR_HANDLERS_VAR_NAME)?;
                let table: LuaTable = handlers.get(key)?;
                table.set(handler_id, None::<i32>)?;
            }
            Ok(())
        });
        methods.add_method_mut("watch", |lua_ctx, this, (key, watcher): (String, LuaFunction)| {
            let new_id = this.next_id();
            this.var_watchers.entry(key.clone()).or_insert_with(|| HashSet::new()).insert(new_id);
            let handlers: LuaTable = lua_ctx.globals().get(Global::VAR_HANDLERS_VAR_NAME)?;
            if !handlers.contains_key(key.clone())? {
                let table = lua_ctx.create_table()?;
                handlers.set(key.clone(), table)?;
            }
            let table: LuaTable = handlers.get(key)?;
            table.set(new_id, watcher)?;
            Ok(new_id)
        });
    }
}