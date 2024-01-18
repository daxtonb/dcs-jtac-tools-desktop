-- LuaUnit Setup
local luarocks_path = assert(os.getenv("APPDATA")) .. "\\luarocks"
local script_directory = arg[0]:match("(.*[\\/])") or ""
package.path = package.path .. ";" .. luarocks_path .. "\\share\\lua\\5.4\\?.lua"
package.path = package.path .. ";" .. script_directory .. "?.lua"

local lu = require('luaunit')
local Logger = require("Logger")
Test_Logger = {}

function Test_Logger:test_message_logging()
    Logger:log("foo")
    Logger:dispose()

    local log_file = io.open("./dcs_jtac_tools.log", "r")
    lu.assertNotNil(log_file)

---@diagnostic disable-next-line: need-check-nil
    local line = log_file:read("*line")
    lu.assertEquals(line, "JTAC: foo")

---@diagnostic disable-next-line: need-check-nil
    log_file:close();
end


os.exit(lu.LuaUnit.run())
