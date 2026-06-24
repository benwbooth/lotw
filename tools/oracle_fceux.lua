-- Differential-oracle FCEUX side. Reads a `frame <count> <buttons>` replay from
-- $LOTW_REPLAY, feeds it to controller 1, and dumps $0000-$07FF each frame to
-- $LOTW_ORACLE_OUT. Lua 5.1-safe (no native bitops).
local replay = os.getenv("LOTW_REPLAY")
local out_path = os.getenv("LOTW_ORACLE_OUT") or "/tmp/oracle_fceux.bin"
local bits = {A=1,B=2,select=4,start=8,up=16,down=32,left=64,right=128}
local frames = {}
for line in io.open(replay):lines() do
  line = line:gsub("#.*",""):gsub("^%s+",""):gsub("%s+$","")
  if line ~= "" then
    local toks = {}
    for t in line:gmatch("%S+") do toks[#toks+1] = t end
    if toks[1] == "frame" then
      local cnt = tonumber(toks[2]) or 0
      local b = 0
      for i = 3, #toks do b = b + (bits[toks[i]] or 0) end  -- distinct bits: + == |
      for _ = 1, cnt do frames[#frames+1] = b end
    end
  end
end
local function bit(b, k) return (math.floor(b / (2 ^ k)) % 2) == 1 end
local o = io.open(out_path, "wb")
for i = 1, #frames do
  local b = frames[i]
  joypad.set(1, {A=bit(b,0),B=bit(b,1),select=bit(b,2),start=bit(b,3),
                 up=bit(b,4),down=bit(b,5),left=bit(b,6),right=bit(b,7)})
  emu.frameadvance()
  local t = {}
  for a = 0, 0x7FF do t[#t+1] = string.char(memory.readbyte(a)) end
  o:write(table.concat(t))
end
o:close()
os.exit()
