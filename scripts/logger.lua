--[[
    LOGGER
    Library for creating log entries.
]]

local __log_file = io.open("./dcs_jtac_tools.log", "w")

local Logger = {}

function Logger:log(message)
    if __log_file then
        __log_file:write("JTAC: " .. message .. "\n")
    end
end

function Logger:dispose()
    if __log_file then
        __log_file:close()
    end
end

return Logger