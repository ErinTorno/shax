local name = "test_skelly@" .. local_entity:id()

local_entity:register({
    on_init = function()
        global:log(name .. " on_init")
    end,
    on_update = function()
        global:log(name .. " on_update on turn " .. global:turn_count())
    end
})