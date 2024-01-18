--[[
    TRANSMITTER
    Sends user data over the network via UDP.
]]

local __logger = require("Logger")
local __socket = require("socket")
__socket = __socket.try(__socket.udp())

local Transmitter = {}

function Transmitter:transmit_units(units)
    -- TODO
end

function Transmitter:transmit_user_unit(user_unit)
    -- TODO
end

function Transmitter:dispose()
    self.socket:close()
end

return Transmitter