use rlua::{Context, Result};

pub trait LuaInstance: Sized {
    fn init(self, lua_ctx: Context) -> Result<()>;

    fn finalize(lua_ctx: Context) -> Result<Self>;
}

pub trait LuaEvent<K> {
    fn run_handlers(&self, lua_ctx: Context, event: K) -> Result<()>;
}