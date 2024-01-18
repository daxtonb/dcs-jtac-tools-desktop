# dcs-jtac-tools-desktop
DCS JTAC Tools for desktop

## Development Environment Setup
### Lua
1. Download an install GCC using [these instructions](https://dev.to/gamegods3/how-to-install-gcc-in-windows-10-the-easier-way-422j)
2. Use the MinGW Installation Manager to install mingw32-base.
3. Download the latest version of [Lua](https://www.lua.org/download.html). Extract the zipped file and run the following command in the extracted directory: 
```bash
mingw32-make PLAT=mingw MYCFLAGS=-DLUA_BUILD_AS_DLL MYLIBS=lua54.dll
```
4. Copy all of the files generated in the `src/` directory into a new directory (i.e. `C:\Program Files\Lua`) and add it to the Environment Variables path.
5. Download the latest build of [LuaRocks](https://luarocks.github.io/luarocks/releases/). Copy the extracted files to a new directory (i.e. `C:\Program Files\LuaRocks`) and add it to the Environment Variables path.
6. Update LuaRocks
```bash
luarocks install luarocks
```
7. Install the LuaUnit library
```bash
luarocks install luaunit
```