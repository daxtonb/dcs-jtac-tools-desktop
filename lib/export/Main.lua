--[[
    MAIN
    Main entry point of the program. Provides the Export.lua overrides.
]]

dofile("./Config/World/World.lua")
package.path = package.path .. ";.\\LuaSocket\\?.lua"
package.cpath = package.cpath .. ";.\\LuaSocket\\?.dll"

local current_frame = 0
local config_file_path = ""
local socket = require("socket")
socket = socket.try(socket.udp())

local Constants = require("Constants")
local Logger = require("Logger")
local UnitExporter = require("UnitExporter")
local UserConfig = require("UserConfig"):new(config_file_path)
local Transmitter = require("Transmitter")

local function handle_next_frame()
    current_frame = current_frame + 1
    local frame_frequency = UserConfig:get_value("frame_frequency") or 10

    if current_frame % frame_frequency ~= 0 then
        return
    end

    if UserConfig:config_changed() then
        UserConfig:reload()
    end

    local caolition_flag = UserConfig:get_value('coalition_flag') or Constants.coalition_flags.BLUFOR
    local unit_type_flag = UserConfig:get_value('unit_type_flag') or Constants.unit_type_flags.GROUND
    local user_unit_name = UserConfig:get_value('user_unit_name')

    local moved_units = UnitExporter:get_relevant_moved_units(caolition_flag, unit_type_flag)
    local user_unit = UnitExporter:find_by_name(user_unit_name)

    Transmitter:transmit_units(moved_units)
    Transmitter:transmit_user_unit(user_unit)
end

local function dispose()
    Transmitter:dispose()
end

LuaExportBeforeNextFrame = function()
    local sucess, result = pcall(function() handle_next_frame() end)

    if not sucess then
        Logger:log('ERROR LuaExportBeforeNextFrame: ' .. result)
    end
end

LuaExportStop = function()
    local sucess, result = pcall(function() dispose() end)

    if not sucess then
        Logger:log('ERROR LuaExportBeforeNextFrame: ' .. result)
    end
end