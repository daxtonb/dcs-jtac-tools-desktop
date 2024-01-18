--[[
    CONSTANTS
    Library for constant values
]]

local Constants = {
    coalition_flags = {
        NEUTRAL = 1,    -- 0001
        BLUFOR  = 2,    -- 0010
        REDFOR  = 4     -- 0100
    },
    unit_type_flags = {
        AIR     = 1,    -- 0001
        GROUND  = 2,    -- 0010
        SEA     = 4     -- 0100      
    },
    unit_type = {
        AIR     = 'A',
        GROUND  = 'G',
        SEA     = 'S'
    },
    unit_coalition = {
        NETRUAL = 0,
        REDFOR  = 1,
        BLUFOR  = 2
    }
}

return Constants