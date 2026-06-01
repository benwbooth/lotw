-- FCEUX side of the lockstep co-simulation: dump 2 KiB of CPU RAM ($0000-$07FF)
-- once per frame to a raw binary trace, matching test/lockstep_port.c, while
-- injecting a per-frame input stream so both sides see identical buttons.
--   QT_QPA_PLATFORM=offscreen LIBGL_ALWAYS_SOFTWARE=1 \
--   LOCKSTEP_INPUT=/tmp/lotw_input.bin LOCKSTEP_FRAMES=3000 \
--   fceux --sound 0 --loadlua tools/re/lockstep_fceux.lua rom/lotw.nes
-- Env: LOCKSTEP_OUT (output path), LOCKSTEP_FRAMES (count), LOCKSTEP_INPUT
-- (raw 1 byte/frame controller stream, serial bit order bit0=A,1=B,2=Select,
-- 3=Start,4=Up,5=Down,6=Left,7=Right; absent => zero input / attract demo).
local outp   = os.getenv("LOCKSTEP_OUT") or "/tmp/fceux_trace.bin"
local frames = tonumber(os.getenv("LOCKSTEP_FRAMES") or "2000")
local inp    = os.getenv("LOCKSTEP_INPUT")
local perreadp = os.getenv("LOCKSTEP_PERREAD")  -- dump per-READ input (content-aligned)
local input = nil
if inp then local fi = io.open(inp, "rb"); if fi then input = fi:read("*all"); fi:close() end end
local f = io.open(outp, "wb")
local perread = perreadp and io.open(perreadp, "wb") or nil
local n = 0

-- Record the input byte the game read at each read_controllers ($CC43) call, so the
-- port can replay input by controller-READ count (immune to frame-timing slips).
local readcount = 0
local rcfile = perreadp and io.open(perreadp .. ".count", "wb") or nil
if perread then
  memory.registerexec(0xCC43, function()
    -- joypad.set (in registerbefore) lands one frame late, so the game's read on
    -- frame n actually sees the input set for frame n-1. Record what it READ.
    if input and n >= 1 and (n - 1) < #input then perread:write(string.char(string.byte(input, n)))
    else perread:write(string.char(0)) end
    readcount = readcount + 1
  end)
end

local function buttons_for(frame)
  if not input or frame >= #input then return nil end
  local b = string.byte(input, frame + 1)   -- lua strings are 1-indexed
  return {
    A      = (b % 2) >= 1,            B      = (math.floor(b/2)  % 2) >= 1,
    select = (math.floor(b/4)  % 2) >= 1, start  = (math.floor(b/8)  % 2) >= 1,
    up     = (math.floor(b/16) % 2) >= 1, down   = (math.floor(b/32) % 2) >= 1,
    left   = (math.floor(b/64) % 2) >= 1, right  = (math.floor(b/128)% 2) >= 1,
  }
end

-- set input for the frame about to run, then dump that frame's RAM after it runs
emu.registerbefore(function()
  local btn = buttons_for(n)
  if btn then joypad.set(1, btn) end
end)
emu.registerafter(function()
  f:write(memory.readbyterange(0x0000, 0x800))   -- binary string, one byte/addr
  if rcfile then  -- per-frame cumulative read count, 4 bytes LE
    rcfile:write(string.char(readcount % 256, math.floor(readcount/256) % 256,
                             math.floor(readcount/65536) % 256, 0))
  end
  n = n + 1
  if n >= frames then f:close(); if perread then perread:close() end; io.stderr:write("lockstep_fceux: wrote "..n.." frames\n"); os.exit() end
end)
