-- ********************************************************
-- AUTHOR   Dax
-- VERSION  0.0.0.1a 
-- DATE     2024-01-16
-- 
-- This file facilitates exportation of ground unit
-- location data to be consumed by the DCS JTAC Tools
-- app for Android.
-- ********************************************************

-----------------------------------------------------------
--                     Dependencies
-----------------------------------------------------------
dofile("./Config/World/World.lua")
package.path = package.path .. ";.\\LuaSocket\\?.lua"
package.cpath = package.cpath .. ";.\\LuaSocket\\?.dll"
local lfs = require("lfs")

-----------------------------------------------------------
--                      Utilities
-----------------------------------------------------------

local Utils = {
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

-----------------------------------------------------------
--                 Unit Export Utility
-----------------------------------------------------------
local Exporter = {
    units_by_id = {}
}

function Exporter:get_relevant_moved_units(coalition_flag, unit_type_flag)
    local __moved_units = {}

    for id, unit in pairs(LoGetWorldObjects()) do
        if unit.Flags.Born 
            and self:__unit_coalition_is_flagged(unit, coalition_flag) 
            and self:__unit_type_is_flagged(unit, unit_type_flag)
            and self:__unit_has_moved(id, unit) then
                __moved_units[#__moved_units + 1] = unit
        end
    end

    return __moved_units
end

function Exporter:__unit_coalition_is_flagged(unit, coalition_flag)
    if unit.CoalitionID == Utils.unit_coalition.BLUFOR and coalition_flag | Utils.coalition_flags.BLUFOR then
        return true
    elseif unit.CoalitionID == Utils.unit_coalition.REDFOR and coalition_flag | Utils.coalition_flags.REDFOR then
        return true
    elseif unit.CoalitionID == Utils.unit_coalition.NETRUAL and coalition_flag | Utils.coalition_flags.NEUTRAL then
        return true
    else
        return false
    end
end

function Exporter:__unit_type_is_flagged(unit, unit_type_flag)
    if unit.type == Utils.unit_type.GROUND and unit_type_flag | Utils.unit_type_flags.GROUND then
        return true
    elseif unit.type == Utils.unit_type.AIR and unit_type_flag | Utils.unit_type_flags.AIR then
        return true
    elseif unit.type == Utils.unit_type.SEA and unit_type_flag | Utils.unit_type_flags.SEA then
        return true
    else
        return false
    end
end

function Exporter:__unit_has_moved(id, unit)
    local matching_unit = self.units_by_id[id]
    return not matching_unit
        or matching_unit.LatLongAlt.Lat ~= unit.LatLongAlt.Lat 
        or matching_unit.LatLongAlt.Long ~= unit.LatLongAlt.Long
end

-----------------------------------------------------------
--                  DCS JTAC Tools
-----------------------------------------------------------
local JTAC = {
    config_file_path = ""
}

function JTAC:initialize()
    self.log_file = io.open("./dcs_jtac_tools.log", "w")
    self:log("Initializeing DCS JTAC Tools.")

    self:load_user_config()

    local socket = require("socket")
    self.socket = socket.try(socket.udp())
    self.current_frame = 0
end

function JTAC:load_user_config()
    self.config = {}

    local file = io.open(self.config_file_path, "r")
    if not file then
        self:log("Failed to load user config file.")
        return
    end

    self:log("Loading user configurations:")
    for line in file:lines() do
        -- Split the line on '=' into key-value tuple
        local key, value = line:match("(%w+)%s*=%s*(%w+)")

        if key and value then
            self:log(string.format("%s = %s", key, value))
            self.config[key] = value
        end
    end

    self.config.last_updated = self:__get_config_file_modification_timestamp();
end

function JTAC:__get_config_file_modification_timestamp()
    return lfs.attributes(self.config_file_path, "modification")
end

function JTAC:__config_changed()
    return self:__get_config_file_modification_timestamp() > self.config.last_updated
end

function JTAC:handle_next_frame()
    self.current_frame = self.current_frame + 1
    local moved_units = Exporter:get_relevant_moved_units(self.config.coalition_flag, self.config.unit_type_flag)
end

function JTAC:log(message)
    self.log_file:write(message .. '\n')
end

function JTAC:dispose()
    self.socket:close()
end

-----------------------------------------------------------
--                  Export.lua Overrides
-----------------------------------------------------------
LuaExportStart = function()
    local sucess, result = pcall(function() JTAC:Initialize() end)

    if not sucess then
        JTAC:log('ERROR LuaExportStart: ' .. result)
    end
end

LuaExportBeforeNextFrame = function()
    local sucess, result = pcall(function() JTAC:handle_next_frame() end)

    if not sucess then
        JTAC:log('ERROR LuaExportBeforeNextFrame: ' .. result)
    end
end

LuaExportStop = function()
    local sucess, result = pcall(function() JTAC:Dispose() end)

    if not sucess then
        JTAC:log('ERROR LuaExportBeforeNextFrame: ' .. result)
    end
end