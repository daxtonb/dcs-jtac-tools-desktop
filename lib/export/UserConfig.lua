--[[
    USER CONFIG
    User config utility that retrieves configurations from a file, and is capable
    of detecting config changes.
]]

local __lfs = require('lfs')
local __logger = require("Logger")

local Module = {}
local UserConfig = {}

local __config_file_path
local __config
local __last_updated

local function get_config_file_modification_timestamp()
    return __lfs.attributes(__config_file_path, "modification")
end

local function load_user_config()
    __logger:log("Loading user configs")
    local config = {}

    local file = io.open(__config_file_path, "r")
    if not file then
        __logger.log("Failed to open file at " .. __config_file_path)
        return
    end

    for line in file:lines() do
        -- Split the line on '=' into key-value tuple
        local key, value = line:match("(%w+)%s*=%s*(%w+)")

        if key and value then
            config[key] = value
        end
    end

    return config
end

function Module:new(config_file_path)
    __config_file_path = config_file_path
    UserConfig:reload()

    return UserConfig
end

function UserConfig:reload()
    __config = load_user_config()
    self.last_update = get_config_file_modification_timestamp()
end

function UserConfig:config_changed()
    return get_config_file_modification_timestamp() > __last_updated
end

function UserConfig:get_value(key)
    return __config[key]
end

return Module