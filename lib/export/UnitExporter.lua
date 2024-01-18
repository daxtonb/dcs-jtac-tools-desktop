--[[
    UNIT EXPORTER
    Uses the DCS API to fetch all units and returns units that have moved and are 
    relevant to the user's configurations.
]]

local Constants = require("Constants");
local UnitExporter = {}
local __units_by_name = {}

local function unit_coalition_is_flagged(unit, coalition_flag)
    if unit.CoalitionID == Constants.unit_coalition.BLUFOR and coalition_flag & Constants.coalition_flags.BLUFOR then
        return true
    elseif unit.CoalitionID == Constants.unit_coalition.REDFOR and coalition_flag & Constants.coalition_flags.REDFOR then
        return true
    elseif unit.CoalitionID == Constants.unit_coalition.NETRUAL and coalition_flag & Constants.coalition_flags.NEUTRAL then
        return true
    else
        return false
    end
end

local function unit_type_is_flagged(unit, unit_type_flag)
    if unit.type == Constants.unit_type.GROUND and unit_type_flag & Constants.unit_type_flags.GROUND then
        return true
    elseif unit.type == Constants.unit_type.AIR and unit_type_flag & Constants.unit_type_flags.AIR then
        return true
    elseif unit.type == Constants.unit_type.SEA and unit_type_flag & Constants.unit_type_flags.SEA then
        return true
    else
        return false
    end
end

local function unit_has_moved(unit)
    local matching_unit = __units_by_name[unit.UnitName]
    return not matching_unit
        or matching_unit.LatLongAlt.Lat ~= unit.LatLongAlt.Lat 
        or matching_unit.LatLongAlt.Long ~= unit.LatLongAlt.Long
end


function UnitExporter:get_relevant_moved_units(coalition_flag, unit_type_flag)
    local moved_units = {}

    for _, unit in pairs(LoGetWorldObjects()) do
        if unit.Flags.Born 
            and unit_coalition_is_flagged(unit, coalition_flag) 
            and unit_type_is_flagged(unit, unit_type_flag)
            and unit_has_moved(unit) then
                moved_units[#moved_units + 1] = unit
        end
    end

    return moved_units
end

function UnitExporter:find_by_name(unit_name)
    if not unit_name then
        return nil
    end
    
    return __units_by_name[unit_name]
end

return UnitExporter