local_entity:register({
    on_update = function()
        if (global:turn_count() % 2 == 0) then
            -- local_entity:set_anim("up")
        else
            -- local_entity:set_anim("down")
        end
    end
})