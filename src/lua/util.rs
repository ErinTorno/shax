use rlua::prelude::*;

pub fn compute_if_absent<'lua, K, V, F>(table: &LuaTable<'lua>, key: K, f: F) -> LuaResult<V>
    where K: ToLua<'lua> + Clone,
          V: ToLua<'lua> + FromLua<'lua>,
          F: FnOnce() -> LuaResult<V> {
    if !table.contains_key(key.clone())? {
        let val = f()?;
        table.set(key.clone(), val)?;
    }
    table.get(key)
}

pub fn get_if_present<'lua, K, V>(table: &LuaTable<'lua>, key: K) -> LuaResult<Option<V>>
    where K: ToLua<'lua> + Clone,
          V: ToLua<'lua> + FromLua<'lua> {
    if table.contains_key(key.clone())? {
        let val = table.get(key)?;
        Ok(Some(val))
    } else {
        Ok(None)
    }
}