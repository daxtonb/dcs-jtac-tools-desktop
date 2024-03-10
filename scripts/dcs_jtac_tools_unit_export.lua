local socket = require("socket")
local logger = require("logger")
local udp = socket.udp()
udp:settimeout(0)
udp:setpeername("127.0.0.1", 34254)

local frameCounter = 0
local exportFrequency = 10

function LuaExportStart()
    -- Initialization of your export logic if needed
end

function LuaExportActivityNextEvent(t)
    local _, err = pcall(function()
        frameCounter = frameCounter + 1
        if frameCounter % exportFrequency == 0 then
            exportWorldObjects()
        end
    end)

    if err then
        logger:log(err)
    end

    return frameCounter
end

-- The goal is to quickly extract all units to the DCS JTAC Hub for processing
function exportWorldObjects()
    local worldObjects = LoGetWorldObjects()
    local currentTime = LoGetModelTime()
    local missionStartTime = LoGetMissionStartTime()

    for _, obj in pairs(worldObjects) do
        -- We only care about active and non-static units
        if obj.Flags.Born and not obj.Flags.Static then
            local jsonData = string.format([[{"unit_name":"%s","group_name":"%s","coalition":%d,"position":{"latitude":%f,"longitude":%f,"altitude":%f},"unit_type":{"level_1":"%s","level_2":"%s"},"mission_date":"%s","mission_start_time":%d,"mission_time_elapsed":%d}\n]],
            obj.Name,
            obj.GroupName,
            obj.Coalition,
            obj.LatLongAlt.Lat,
            obj.LatLongAlt.Long,
            obj.LatLongAlt.Alt,
            obj.Type.level1,
            obj.Type.level2,
            string.format("%04d-%02d-%02dT%02d", MissionDate.Year, MissionDate.Month, MissionDate.Day),
            missionStartTime,
            currentTime)

        udp:send(jsonData)
        end
    end
end

function LuaExportStop()
    udp:close()
    logger:dispose()
end
