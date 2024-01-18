--[[
    LOGGER
    Library for creating log entries.
]]

local Logger = {
    log_file = io.open("./dcs_jtac_tools.log", "w")
}

function Logger:log(message)
    self.log_file.write("JTAC: " .. message .. "\n")
end

return Logger