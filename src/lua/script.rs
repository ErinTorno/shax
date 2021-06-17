use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use rlua::{Lua, prelude::*, StdLib};
use std::{borrow::BorrowMut, sync::Mutex};

use crate::lua::global::*;
use crate::lua::types::*;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "f63d791c-ed06-4a84-91ef-f01b640799fe"]
pub struct LuaScript(pub Vec<u8>);

impl AsRef<[u8]> for LuaScript {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    } 
}

#[derive(Default)]
pub struct LuaScriptLoader;

impl AssetLoader for LuaScriptLoader {
    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            load_context.set_default_asset(LoadedAsset::new(LuaScript(bytes.iter().cloned().collect())));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] { &["lua"] }
}

pub struct LuaResource {
    lua:        Mutex<Lua>,
    pub global: Global,
}

impl Default for LuaResource {
    fn default() -> LuaResource {
        let lua = Lua::new_with(StdLib::BASE | StdLib::COROUTINE | StdLib::TABLE | StdLib::STRING | StdLib::UTF8 | StdLib::MATH | StdLib::PACKAGE);
        let global = lua.context(|lua_ctx| {
            Global::init(lua_ctx)
        }).expect("Failed to init Lua global");
        LuaResource {
            lua: Mutex::new(lua),
            global,
        }
    }
}

impl LuaResource {
    pub fn exec_script<S: ?Sized>(&mut self, source: &S) where S: AsRef<[u8]> {
        let mut lua_guard = self.lua.lock().unwrap();
        let _ = lua_guard.borrow_mut().context(|lua_ctx| {
            lua_ctx.load(source).exec()
        });
    }

    pub fn exec_script_with_instance<S: ?Sized, I: LuaInstance>(&mut self, source: &S, instance: I) -> LuaResult<I> where S: AsRef<[u8]> {
        let mut lua_guard = self.lua.lock().unwrap();
        lua_guard.borrow_mut().context(|lua_ctx| {
            instance.init(lua_ctx)?;
            lua_ctx.load(source).exec()?;
            I::finalize(lua_ctx)
        })
    }

    pub fn run_event<K, L: LuaInstance + LuaEvent<K> + Clone>(&mut self, key: K, instance: L) -> LuaResult<L> {
        let mut lua_guard = self.lua.lock().unwrap();
        lua_guard.borrow_mut().context(|lua_ctx| {
            instance.clone().init(lua_ctx)?;
            instance.run_handlers(lua_ctx, key)?;
            L::finalize(lua_ctx)
        })
    }

    pub fn sync(&mut self) {
        let mut lua_guard = self.lua.lock().unwrap();
        lua_guard.borrow_mut().context(|lua_ctx| {
            lua_ctx.globals().set("global", self.global.clone())
        }).expect("failed to sync LuaResource");
    }
}