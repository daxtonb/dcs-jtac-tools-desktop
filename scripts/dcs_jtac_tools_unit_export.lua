-----------------------------------------------------------
--                     Dependencies
-----------------------------------------------------------
dofile("./Config/World/World.lua")
package.path = package.path .. ";.\\LuaSocket\\?.lua"
package.cpath = package.cpath .. ";.\\LuaSocket\\?.dll"

-----------------------------------------------------------
--                  DCS JTAC Tools
-----------------------------------------------------------
local DcsJtacTools = {}

function DcsJtacTools:Initialize()
    self.frameFrequency = 100
    self.address = "127.0.0.1"
    self.port = "34254"

    local userProfile = os.getenv("userprofile"):gsub("\\","/")
    self.log_file = io.open(userProfile .. "/Saved Games/DCS.openbeta/Logs/DcsJtacTools.log", 'w')
    self:log('Initializing DCS JTAC Tools')
    self:log(string.format("emitting to %s:%s every %d frames", self.address, self.port, self.frameFrequency))
    
    self.socket = require("socket")
    self.udp = self.socket.try(self.socket.udp())
    self.currentFrame = 0
    self.missionStartTime = LoGetMissionStartTime()
end



--[[
    {
    ["Pitch"] = 0.020816618576646,
    ["Type"] = {
        ["level3"] = 26,
        ["level1"] = 2,
        ["level4"] = 14,
        ["level2"] = 17,
    },
    ["Country"] = 0,
    ["Flags"] = {
        ["Jamming"] = false,
        ["IRJamming"] = false,
        ["Born"] = true,
        ["Static"] = false,
        ["Invisible"] = false,
        ["Human"] = false,
        ["AI_ON"] = true,
        ["RadarActive"] = false,
    },
    ["GroupName"] = "Ground-3",
    ["PositionAsMatrix"] = {
        ["y"] = {
            ["y"] = 0.99966436624527,
            ["x"] = 0.021639443933964,
            ["z"] = -0.021639443933964,
        },
        ["x"] = {
            ["y"] = 0.020815117284656,
            ["x"] = -0.9982323050499,
            ["z"] = -0.020815117284656,
        },
        ["p"] = {
            ["y"] = 86.433990471004,
            ["x"] = 15808.849751935,
            ["z"] = 32835.261060565,
        },
        ["z"] = {
            ["y"] = 0.01542529091239,
            ["x"] = -0.055354587733746,
            ["z"] = 0.9982323050499,
        },
    },
    ["Coalition"] = "Allies",
    ["Heading"] = 3.0858819484711,
    ["Name"] = "mrap_mk19",
    ["Position"] = {
        ["y"] = 86.433990471004,
        ["x"] = 15808.849751935,
        ["z"] = 32835.261060565,
    },
    ["UnitName"] = "Ground-3-1",
    ["LatLongAlt"] = {
        ["Long"] = 31.583208351465,
        ["Lat"] = 30.193920851419,
        ["Alt"] = 86.433990471004,
    },
    ["CoalitionID"] = 1,
    ["Bank"] = -0.015429245308042,
}

The goal is to quickly extract all units to the DCS JTAC Hub for processing
]]
function DcsJtacTools:ExportUnits()
    self.currentFrame = self.currentFrame + 1

    if self.currentFrame % self.frameFrequency ~= 0 then
        return
    end

    local worldObjects = LoGetWorldObjects()
    local currentTime = LoGetModelTime()

    for _, obj in pairs(worldObjects) do
        if obj.UnitName and obj.Flags.Born and not obj.Flags.Static then
            local jsonData = string.format([[{"unit_name":"%s","group_name":"%s","coalition":%s,"position":{"latitude":%.5f,"longitude":%.5f,"altitude":%s,"heading":%.5f},"unit_type":{"level_1":%d,"level_2":%d},"mission_date":"%s","mission_start_time":%d,"mission_time_elapsed":%d}]] .. "\n",
            obj.UnitName,
            obj.GroupName,
            obj.CoalitionID,
            obj.LatLongAlt.Lat,
            obj.LatLongAlt.Long,
            obj.LatLongAlt.Alt,
            obj.Heading,
            obj.Type.level1,
            obj.Type.level2,
            string.format("%04d-%02d-%02d", MissionDate.Year, MissionDate.Month, MissionDate.Day),
            self.missionStartTime,
            currentTime)

        self.socket.try(self.udp:sendto(jsonData, self.address, self.port))
        end
    end
end

function DcsJtacTools:Dispose()
    self:log('Shutting down DCS JTAC Tools')
    self.socket.try(self.udp:close())
    self.log_file:close();
end

function DcsJtacTools:log(message)
    self.log_file:write(message .. '\n')
end

-----------------------------------------------------------
--                  Export.lua Overrides
-----------------------------------------------------------
LuaExportStart = function()
    local success, result = pcall(function() DcsJtacTools:Initialize() end)

    if not success then
        DcsJtacTools:log('ERROR LuaExportStart: ' .. result)
    end
end

LuaExportBeforeNextFrame = function()
    local success, result = pcall(function() DcsJtacTools:ExportUnits() end)

    if not success then
        DcsJtacTools:log('ERROR LuaExportBeforeNextFrame: ' .. result)
    end
end

LuaExportStop = function()
    local success, result = pcall(function() DcsJtacTools:Dispose() end)

    if not success then
        DcsJtacTools:log('ERROR LuaExportBeforeNextFrame: ' .. result)
    end
end