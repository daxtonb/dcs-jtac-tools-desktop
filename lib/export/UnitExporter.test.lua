-- LuaUnit Setup
local luarocks_path = assert(os.getenv("APPDATA")) .. "\\luarocks"
local script_directory = arg[0]:match("(.*[\\/])") or ""
package.path = package.path .. ";" .. luarocks_path .. "\\share\\lua\\5.4\\?.lua"
package.path = package.path .. ";" .. script_directory .. "?.lua"

local lu = require('luaunit')
local Constants = require('Constants')
local UnitExporter = require('UnitExporter')
Test_Unit_Exporter = {}

local function set_up_unit(coalition, unit_type, latitude, longitude, unit_name, is_born)
    function LoGetWorldObjects()
        return {
            unit_id = {
                UnitName = unit_name or math.random(),
                CoalitionID = coalition,
                Type = {
                    level1 = unit_type
                },
                LatLongAlt = {
                    Lat = latitude or 0.0,
                    Long = longitude or 0.0
                },
                Flags = {
                    Born = is_born or 1
                }
            }
         }
    end
end

function Test_Unit_Exporter:test_matching_unit_returned_by_coalition()
    local unit_type = Constants.unit_type.GROUND
    local unit_type_flag = Constants.unit_type_flags.GROUND
    local coalition_id_flag_pairs = { 
        { Constants.unit_coalition.BLUFOR, Constants.coalition_flags.BLUFOR },  
        { Constants.unit_coalition.REDFOR, Constants.coalition_flags.REDFOR },  
        { Constants.unit_coalition.NETRUAL, Constants.coalition_flags.NEUTRAL }  
    }
    
    for _, id_flag in pairs(coalition_id_flag_pairs) do
        set_up_unit(id_flag[1], unit_type, 0, 0)
        local results = UnitExporter:get_relevant_moved_units(id_flag[2], unit_type_flag)
        lu.assertNotNil(results)
        lu.assertEquals(#results, 1)
    end
end

function Test_Unit_Exporter:test_no_unit_returned_for_non_matching_coalition()
    local unit_type = Constants.unit_type.GROUND
    local unit_type_flag = Constants.unit_type_flags.GROUND
    local coalition_id_flag_pairs = { 
        { Constants.unit_coalition.BLUFOR, Constants.coalition_flags.BLUFOR },  
        { Constants.unit_coalition.REDFOR, Constants.coalition_flags.REDFOR },  
        { Constants.unit_coalition.NETRUAL, Constants.coalition_flags.NEUTRAL }  
    }
    
    for _, id_flag in pairs(coalition_id_flag_pairs) do
        set_up_unit(id_flag[1], unit_type, 0, 0)
        local results = UnitExporter:get_relevant_moved_units(128, unit_type_flag)
        lu.assertNotNil(results)
        lu.assertEquals(#results, 0)
    end
end

function Test_Unit_Exporter:test_matching_unit_returned_by_unit_type()
    local unit_coalition = Constants.unit_coalition.BLUFOR
    local unit_coalition_flag = Constants.unit_coalition.BLUFOR
    local unit_type_id_flag_pairs = {
        { Constants.unit_type.AIR, Constants.unit_type_flags.AIR },
        { Constants.unit_type.GROUND, Constants.unit_type_flags.GROUND },
        { Constants.unit_type.SEA, Constants.unit_type_flags.SEA }
    }
    
    for _, id_flag in pairs(unit_type_id_flag_pairs) do
        set_up_unit(unit_coalition, id_flag[1], 0, 0)
        local results = UnitExporter:get_relevant_moved_units(unit_coalition_flag, id_flag[2])
        lu.assertNotNil(results)
        lu.assertEquals(#results, 1)
    end
end

function Test_Unit_Exporter:test_no_unit_returned_for_non_matching_unit_type()
    local unit_coalition = Constants.unit_coalition.BLUFOR
    local unit_coalition_flag = Constants.unit_coalition.BLUFOR
    local unit_type_id_flag_pairs = {
        { Constants.unit_type.AIR, Constants.unit_type_flags.AIR },
        { Constants.unit_type.GROUND, Constants.unit_type_flags.GROUND },
        { Constants.unit_type.SEA, Constants.unit_type_flags.SEA }
    }
    
    for _, id_flag in pairs(unit_type_id_flag_pairs) do
        set_up_unit(unit_coalition, id_flag[1], 0, 0)
        local results = UnitExporter:get_relevant_moved_units(unit_coalition_flag, 128)
        lu.assertNotNil(results)
        lu.assertEquals(#results, 0)
    end
end

function Test_Unit_Exporter:test_no_unit_returned_when_not_moving()
    local unit_coalition = Constants.unit_coalition.NETRUAL
    local unit_coalition_flag = Constants.coalition_flags.NEUTRAL
    local unit_type = Constants.unit_type.SEA
    local unit_type_flag = Constants.unit_type_flags.SEA

    set_up_unit(unit_coalition, unit_type, 1, 1, "foo")
    local results = UnitExporter:get_relevant_moved_units(unit_coalition_flag, unit_type_flag)
    lu.assertNotNil(results)
    lu.assertEquals(#results, 1)

    results = UnitExporter:get_relevant_moved_units(unit_coalition_flag, unit_type_flag)
    lu.assertNotNil(results)
    lu.assertEquals(#results, 0)

end

function Test_Unit_Exporter:test_unit_no_unit_returned_when_not_spawned()
    local unit_coalition = Constants.unit_coalition.NETRUAL
    local unit_coalition_flag = Constants.coalition_flags.NEUTRAL
    local unit_type = Constants.unit_type.SEA
    local unit_type_flag = Constants.unit_type_flags.SEA

    set_up_unit(unit_coalition, unit_type, 1, 1, "foo", 0)
    local results = UnitExporter:get_relevant_moved_units(unit_coalition_flag, unit_type_flag)
    lu.assertNotNil(results)
    lu.assertEquals(#results, 0)
end
os.exit(lu.LuaUnit.run())