--[[
    TRANSMITTER
    Sends user data over the network via UDP.
]]

local Module = {}
local Transmitter = {}

local __socket
local __logger

function Module:new(socket, logger)
    __socket = socket
    __logger = logger

    return Transmitter
end

function Transmitter:transmit_units(units)
    -- TODO
end

function Transmitter:transmit_user_unit(user_unit)
    -- TODO
end

function Transmitter:dispose()
    self.socket:close()
end

return Module