-- FCEUX full-execution coverage tracer for Legacy of the Wizard (MMC3 / mapper 4).
--
-- Unlike fceux_trace.lua (which only watches pre-known labels), this records EVERY
-- executed instruction, bank-resolved to a PRG-ROM file offset, to build a
-- code/data coverage map (CDL-style) for the matching disassembly.
--
-- Inputs (env vars):
--   LOTW_COV_OUT_DIR   required output directory
--   LOTW_COV_REPLAY    optional replay fixture path (button stream)
--   LOTW_COV_FRAMES    frames to run, default 600
--   LOTW_COV_DONE      optional completion-marker path
--
-- Outputs (TSV, in OUT_DIR):
--   coverage.tsv        prg_offset (hex) -> exec_count, first_frame, cpu_addr, instr_len
--   mapper_writes.tsv   MMC3 register write log
--   apu_writes.tsv / ppu_writes.tsv / ppu_vram_writes.tsv / oam_dma.tsv
--   coverage_summary.txt

local out_dir = os.getenv("LOTW_COV_OUT_DIR")
local replay_path = os.getenv("LOTW_COV_REPLAY")
local done_path = os.getenv("LOTW_COV_DONE")
local max_frames = tonumber(os.getenv("LOTW_COV_FRAMES") or "600")

if not out_dir or out_dir == "" then error("LOTW_COV_OUT_DIR is required") end
if not max_frames or max_frames < 1 then error("LOTW_COV_FRAMES must be positive") end

local register_execute = memory.registerexecute or memory.registerexec
if not register_execute then error("FCEUX registerexecute/registerexec unavailable") end

-- ---- 6502 instruction lengths (1/2/3 bytes), indexed by opcode 0x00..0xFF ----
-- Official + common illegal opcodes; illegals get best-effort lengths so operand
-- spans stay aligned. Length only affects operand marking, not exec detection.
local OPLEN = {}
do
  for i = 0, 255 do OPLEN[i] = 1 end
  -- 2-byte (immediate / zp / zp,x / zp,y / (zp,x) / (zp),y / relative)
  local two = {
    0x09,0x05,0x15,0x01,0x11,0x25,0x29,0x21,0x31,0x35,0x24,
    0x05,0x06,0x16,0x26,0x36,0x46,0x56,0x66,0x76,0x45,0x55,0x41,0x51,
    0x49,0x69,0x65,0x75,0x61,0x71,0xc9,0xc5,0xd5,0xc1,0xd1,0xe9,0xe5,0xf5,0xe1,0xf1,
    0xc6,0xd6,0xe6,0xf6,0xe0,0xe4,0xc0,0xc4,0xa9,0xa5,0xb5,0xa1,0xb1,0xa2,0xa6,0xb6,
    0xa0,0xa4,0xb4,0x84,0x94,0x85,0x95,0x81,0x91,0x86,0x96,
    0x10,0x30,0x50,0x70,0x90,0xb0,0xd0,0xf0,
  }
  for _, op in ipairs(two) do OPLEN[op] = 2 end
  -- 3-byte (absolute / absolute,x / absolute,y / indirect)
  local three = {
    0x0d,0x1d,0x19,0x2d,0x3d,0x39,0x4d,0x5d,0x59,0x6d,0x7d,0x79,
    0xcd,0xdd,0xd9,0xed,0xfd,0xf9,0x0e,0x1e,0x2e,0x3e,0x4e,0x5e,0x6e,0x7e,
    0xce,0xde,0xee,0xfe,0x2c,0xad,0xbd,0xb9,0xae,0xbe,0xac,0xbc,0x8d,0x9d,0x99,0x8e,0x8c,
    0x4c,0x6c,0x20,
  }
  for _, op in ipairs(three) do OPLEN[op] = 3 end
end

-- ---------------------------- MMC3 bank state -------------------------------
local PRG_PAGES = 16              -- 16 x 8 KiB PRG banks
local SECOND_LAST = PRG_PAGES - 2
local LAST = PRG_PAGES - 1
local bank_select, prg_mode = 0, 0
local bank_regs = {}
for i = 0, 7 do bank_regs[i] = nil end

local function modp(page) if page == nil then return nil end return math.mod(page, PRG_PAGES) end

local function map_prg_offset(addr)
  local page
  if addr >= 0x8000 and addr < 0xa000 then
    page = (prg_mode == 0) and modp(bank_regs[6]) or SECOND_LAST
  elseif addr >= 0xa000 and addr < 0xc000 then
    page = modp(bank_regs[7])
  elseif addr >= 0xc000 and addr < 0xe000 then
    page = (prg_mode == 0) and SECOND_LAST or modp(bank_regs[6])
  elseif addr >= 0xe000 then
    page = LAST
  end
  if page == nil then return nil end
  return page * 0x2000 + math.mod(addr, 0x2000)
end

local frame = 0
local function write_file(path, data)
  local f = assert(io.open(path, "wb")); assert(f:write(data)); assert(f:close())
end

-- ----------------------------- write logging --------------------------------
local mapper_writes, apu_writes, ppu_writes, ppu_vram_writes, oam_dma_writes = {}, {}, {}, {}, {}
local function get_cycle()
  if debugger and debugger.getcyclescount then
    local ok, v = pcall(debugger.getcyclescount); if ok and type(v) == "number" then return v end
  end
  return nil
end

local function record_mapper_write(addr, _, value)
  if addr == 0x8000 then
    bank_select = AND(value, 0x07)
    prg_mode = (AND(value, 0x40) ~= 0) and 1 or 0
  elseif addr == 0x8001 then
    bank_regs[bank_select] = value
  end
  mapper_writes[#mapper_writes + 1] = string.format("%d\t%04X\t%02X\tselect=%d\tprg_mode=%d\tr6=%s\tr7=%s\n",
    frame, addr, value, bank_select, prg_mode,
    bank_regs[6] and string.format("%02X", bank_regs[6]) or "?",
    bank_regs[7] and string.format("%02X", bank_regs[7]) or "?")
end
memory.registerwrite(0x8000, 2, record_mapper_write)

local function record_apu_write(addr, _, value)
  apu_writes[#apu_writes + 1] = string.format("%d\t%s\t%04X\t%02X\n", frame, tostring(get_cycle() or "?"), addr, value)
end
memory.registerwrite(0x4000, 0x14, record_apu_write)
memory.registerwrite(0x4015, 1, record_apu_write)
memory.registerwrite(0x4017, 1, record_apu_write)

local ppu_ctrl, ppu_addr_latch, ppu_addr_high, ppu_vram_addr = 0, 0, 0, 0
local function ppu_region(a)
  if a < 0x2000 then return "pattern" elseif a < 0x3f00 then return "nametable"
  elseif a < 0x4000 then return "palette" else return "?" end
end
local function record_ppu_write(addr, _, value)
  local cyc = get_cycle()
  local reg = (addr >= 0x2000 and addr < 0x4000) and (0x2000 + math.mod(addr, 8)) or addr
  ppu_writes[#ppu_writes + 1] = string.format("%d\t%s\t%04X\t%02X\n", frame, tostring(cyc or "?"), addr, value)
  if reg == 0x2000 then ppu_ctrl = value
  elseif reg == 0x2005 then ppu_addr_latch = 1 - ppu_addr_latch
  elseif reg == 0x2006 then
    if ppu_addr_latch == 0 then ppu_addr_high = value; ppu_addr_latch = 1
    else ppu_vram_addr = AND(ppu_addr_high * 0x100 + value, 0x3fff); ppu_addr_latch = 0 end
  elseif reg == 0x2007 then
    ppu_vram_writes[#ppu_vram_writes + 1] = string.format("%d\t%s\t%04X\t%s\t%02X\n",
      frame, tostring(cyc or "?"), ppu_vram_addr, ppu_region(ppu_vram_addr), value)
    ppu_vram_addr = AND(ppu_vram_addr + (AND(ppu_ctrl, 0x04) ~= 0 and 32 or 1), 0x3fff)
  elseif reg == 0x4014 then
    local base = value * 0x100
    local chunks = {}
    for o = 0, 0xff do chunks[#chunks + 1] = string.format("%02X", memory.readbyte(base + o)) end
    oam_dma_writes[#oam_dma_writes + 1] = string.format("%d\t%s\t%02X\t%s\n", frame, tostring(cyc or "?"), value, table.concat(chunks))
  end
end
memory.registerwrite(0x2000, 0x2000, record_ppu_write)
memory.registerwrite(0x4014, 1, record_ppu_write)
if memory.registerread then memory.registerread(0x2002, 1, function() ppu_addr_latch = 0 end) end

-- ----------------------------- exec coverage --------------------------------
local cov = {}            -- prg_offset -> {count, first_frame, addr, len}
local cov_count = 0
local ram_exec_seen = 0

local function on_exec(addr)
  local off = map_prg_offset(addr)
  if off == nil then return end
  local rec = cov[off]
  if rec == nil then
    local op = memory.readbyte(addr)
    rec = { count = 0, first_frame = frame, addr = addr, len = OPLEN[op] or 1 }
    cov[off] = rec
    cov_count = cov_count + 1
  end
  rec.count = rec.count + 1
end
register_execute(0x8000, 0x8000, on_exec)

-- Detect (rare) execution out of RAM so we know if the ROM map is incomplete.
register_execute(0x0000, 0x8000, function() ram_exec_seen = ram_exec_seen + 1 end)

-- ------------------------------- replay -------------------------------------
local valid = { up=1,down=1,left=1,right=1,A=1,B=1,start=1,select=1 }
local function parse_replay(path)
  local frames = {}
  if not path or path == "" then return frames end
  local f = assert(io.open(path, "r"))
  local total = 0
  for line in f:lines() do
    line = string.gsub(line, "#.*$", "")
    local words = {}
    for w in string.gmatch(line, "%S+") do words[#words + 1] = w end
    if #words > 0 then
      if words[1] ~= "frame" then error("unknown replay directive: " .. words[1]) end
      local count = tonumber(words[2])
      local buttons = {}
      for i = 3, #words do
        if not valid[words[i]] then error("unknown button: " .. words[i]) end
        buttons[words[i]] = true
      end
      for _ = 1, count do total = total + 1; frames[total] = buttons end
    end
  end
  f:close()
  return frames
end

local replay_frames = parse_replay(replay_path)

-- Optional exploration mode: after the replay prefix is exhausted, drive
-- deterministic pseudo-random inputs (LCG seeded by LOTW_COV_EXPLORE) to wander
-- rooms, fight, use items, and die — widening code coverage past the fixtures.
local explore_seed = tonumber(os.getenv("LOTW_COV_EXPLORE") or "")
local replay_len = 0
for k in pairs(replay_frames) do if k > replay_len then replay_len = k end end
local rng = explore_seed or 0
local function rnd(m)
  rng = (rng * 1103515245 + 12345) % 2147483648
  return math.mod(math.floor(rng / 65536), m)
end
-- Persona varies behaviour per session to widen coverage into different code:
--   0 forward (right-heavy, jumps), 1 suicidal (no jump, walk into hazards -> die
--   -> GAME OVER in bank 12), 2 menu-masher (start/select/A through menus/shop),
--   3 climber (up/down for ladders/doors/room transitions).
local persona = math.mod(explore_seed or 0, 4)
local held = {}
local function explore_input(i)
  if math.mod(i, 12) == 1 then          -- re-roll inputs periodically
    held = {}
    local mv = rnd(100)
    if mv < 40 then held.right = true elseif mv < 70 then held.left = true end
    local vv = rnd(100)
    if persona == 0 then
      if rnd(100) < 55 then held.A = true end
      if rnd(100) < 30 then held.B = true end
      if vv < 12 then held.up = true elseif vv < 22 then held.down = true end
    elseif persona == 1 then            -- suicidal: never jump, push into things
      if rnd(100) < 25 then held.B = true end
      if vv < 25 then held.up = true elseif vv < 50 then held.down = true end
    elseif persona == 2 then            -- menu masher
      if rnd(100) < 50 then held.A = true end
      if rnd(100) < 30 then held.B = true end
      if rnd(100) < 35 then held.start = true end
      if rnd(100) < 20 then held.select = true end
    else                                -- climber
      if rnd(100) < 35 then held.A = true end
      if vv < 40 then held.up = true elseif vv < 75 then held.down = true end
    end
    if rnd(100) < 5 then held.start = true end
    if rnd(100) < 4 then held.select = true end
  end
  return held
end

FCEU.speedmode("maximum")
for i = 1, max_frames do
  frame = i
  local btns
  if explore_seed and i > replay_len then
    btns = explore_input(i)
  else
    btns = replay_frames[i] or {}
  end
  joypad.set(1, btns)
  FCEU.frameadvance()
end

-- ------------------------------- emit ---------------------------------------
local offsets = {}
for off in pairs(cov) do offsets[#offsets + 1] = off end
table.sort(offsets)

local rows = { "prg_offset\texec_count\tfirst_frame\tcpu_addr\tinstr_len\n" }
for _, off in ipairs(offsets) do
  local r = cov[off]
  rows[#rows + 1] = string.format("%05X\t%d\t%d\t%04X\t%d\n", off, r.count, r.first_frame, r.addr, r.len)
end
write_file(out_dir .. "/coverage.tsv", table.concat(rows))

local function dump(name, header, list)
  local out = { header }
  for _, row in ipairs(list) do out[#out + 1] = row end
  write_file(out_dir .. "/" .. name, table.concat(out))
end
dump("mapper_writes.tsv", "frame\taddr\tvalue\tstate\n", mapper_writes)
dump("apu_writes.tsv", "frame\tcycle\taddr\tvalue\n", apu_writes)
dump("ppu_writes.tsv", "frame\tcycle\taddr\tvalue\n", ppu_writes)
dump("ppu_vram_writes.tsv", "frame\tcycle\taddr\tregion\tvalue\n", ppu_vram_writes)
dump("oam_dma.tsv", "frame\tcycle\tpage\tbytes\n", oam_dma_writes)

write_file(out_dir .. "/coverage_summary.txt", table.concat({
  "emulator=fceux\nscript=tools/fceux_coverage.lua\n",
  "replay=", replay_path or "", "\n",
  "frames=", tostring(max_frames), "\n",
  "covered_prg_offsets=", tostring(cov_count), "\n",
  "ram_exec_events=", tostring(ram_exec_seen), "\n",
  "mapper_writes=", tostring(#mapper_writes), "\n",
  "apu_writes=", tostring(#apu_writes), "\n",
  "ppu_writes=", tostring(#ppu_writes), "\n",
  "ppu_vram_writes=", tostring(#ppu_vram_writes), "\n",
  "oam_dma=", tostring(#oam_dma_writes), "\n",
  "complete=1\n",
}))

if done_path and done_path ~= "" then write_file(done_path, "complete\n") end
while true do FCEU.frameadvance() end
