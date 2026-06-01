-- In the house (new-game start: char 6, health 0), find which input activates a
-- character (changes $40 off 6 and/or sets health $58 nonzero) and the PC.
local PATH = "/tmp/health_trace.txt"
local log = io.open(PATH, "w")
local function out(s) if log then log:write(s.."\n"); log:flush() end end
local phase_name = "boot"

local function regw(addr, label)
  memory.registerwrite(addr, function(a, b, c)
    local v = c or b
    out(string.format("f%-5d [%s] %s <- $%02X  PC=$%04X px=$%02X py=$%02X",
        emu.framecount(), phase_name, label, v or 0, memory.getregister("pc"),
        memory.readbyte(0x44), memory.readbyte(0x45)))
  end)
end
regw(0x58, "health"); regw(0x40, "char")

out("== trace start ==")
local n = 0
emu.registerafter(function()
  local inp = {}
  if n >= 200 and n < 340 then inp.start = true; phase_name = "enter"
  elseif n >= 400 then
    -- tap each button in turn (3 frames on / rest off, so presses are debounced)
    local seg = math.floor((n - 400) / 120)         -- 120-frame segment per button
    local tap = ((n - 400) % 120) < 4               -- short tap at segment start
    local names = {"A","B","up","down","left","right","start","select"}
    local nm = names[(seg % 8) + 1]
    phase_name = nm
    if tap then inp[nm] = true end
  end
  joypad.set(1, inp)
  if n % 120 == 0 then
    out(string.format("  [f%d] health=$%02X char=$%02X mapX=$%02X mapY=$%02X $8E=$%02X",
        emu.framecount(), memory.readbyte(0x58), memory.readbyte(0x40),
        memory.readbyte(0x47), memory.readbyte(0x48), memory.readbyte(0x8E)))
  end
  n = n + 1
  if n >= 1600 then out("== trace end =="); if log then log:close() end; os.exit() end
end)
