-- FCEUX reference capture driver for the Legacy of the Wizard port.
--
-- Inputs are supplied via environment variables:
--   LOTW_REFERENCE_OUT_DIR  required output directory
--   LOTW_REFERENCE_REPLAY   optional replay fixture path
--   LOTW_REFERENCE_FRAMES   comma-separated frame numbers to capture
--
-- The script writes only local generated artifacts: screenshots as PPM, CPU RAM
-- dumps, and a manifest. Do not commit the output directory.

local out_dir = os.getenv("LOTW_REFERENCE_OUT_DIR")
local replay_path = os.getenv("LOTW_REFERENCE_REPLAY")
local capture_frames_env = os.getenv("LOTW_REFERENCE_FRAMES") or "1,60,120,180"
local done_path = os.getenv("LOTW_REFERENCE_DONE")
local apu_trace_path = os.getenv("LOTW_REFERENCE_APU_TRACE")

if not out_dir or out_dir == "" then
  error("LOTW_REFERENCE_OUT_DIR is required")
end

local function split_words(line)
  local words = {}
  for word in string.gmatch(line, "%S+") do
    table.insert(words, word)
  end
  return words
end

local valid_buttons = {
  up = true,
  down = true,
  left = true,
  right = true,
  A = true,
  B = true,
  start = true,
  select = true,
}

local function parse_capture_frames(text)
  local frames = {}
  for token in string.gmatch(text, "[^,]+") do
    local frame = tonumber(token)
    if not frame or frame < 1 then
      error("invalid capture frame: " .. token)
    end
    frames[frame] = true
  end
  return frames
end

local function parse_replay(path)
  local frames = {}
  local total = 0

  if not path or path == "" then
    return frames, total
  end

  local f = assert(io.open(path, "r"))
  for line in f:lines() do
    line = string.gsub(line, "#.*$", "")
    local words = split_words(line)

    if #words > 0 then
      if words[1] ~= "frame" then
        error("unknown replay directive: " .. words[1])
      end

      local count = tonumber(words[2])
      if not count or count < 1 then
        error("invalid replay frame count")
      end

      local buttons = {}
      for i = 3, #words do
        if not valid_buttons[words[i]] then
          error("unknown replay button: " .. words[i])
        end
        buttons[words[i]] = true
      end

      for _ = 1, count do
        total = total + 1
        frames[total] = buttons
      end
    end
  end
  f:close()

  return frames, total
end

local function write_file(path, data)
  local f = assert(io.open(path, "wb"))
  assert(f:write(data))
  assert(f:close())
end

local current_frame = 0
local apu_writes = {}
local function record_apu_write(addr, _, value)
  if apu_trace_path and apu_trace_path ~= "" then
    apu_writes[#apu_writes + 1] = string.format("%d\t%04X\t%02X\n", current_frame, addr, value)
  end
end

if apu_trace_path and apu_trace_path ~= "" then
  memory.registerwrite(0x4000, 0x14, record_apu_write)
  memory.registerwrite(0x4015, 1, record_apu_write)
  memory.registerwrite(0x4017, 1, record_apu_write)
end

local function write_ram_dump(frame)
  local chunks = {}
  for addr = 0, 0x7ff do
    chunks[#chunks + 1] = string.char(memory.readbyte(addr))
  end
  write_file(string.format("%s/ram_%06d.bin", out_dir, frame), table.concat(chunks))
end

local function write_ppu_dump(frame)
  local chunks = {}
  chunks[#chunks + 1] = ppu.readbyterange(0x2000, 0x1000)
  chunks[#chunks + 1] = ppu.readbyterange(0x3f00, 0x20)
  write_file(string.format("%s/ppu_%06d.bin", out_dir, frame), table.concat(chunks))
end

local function write_ppm_from_gd(frame)
  local gd = gui.gdscreenshot()
  local header_len = 11
  local width = 256
  local height = 240
  local expected = header_len + width * height * 4

  if string.len(gd) < expected then
    error(string.format("unexpected screenshot size: got %d, need at least %d", string.len(gd), expected))
  end

  local out = { string.format("P6\n%d %d\n255\n", width, height) }
  local pos = header_len + 1
  for _ = 1, width * height do
    -- FCEUX GD true-color pixels are alpha, red, green, blue.
    local _, r, g, b = string.byte(gd, pos, pos + 3)
    out[#out + 1] = string.char(r, g, b)
    pos = pos + 4
  end

  write_file(string.format("%s/frame_%06d.ppm", out_dir, frame), table.concat(out))
end

local capture_frames = parse_capture_frames(capture_frames_env)
local replay_frames, replay_length = parse_replay(replay_path)
local max_frame = replay_length

for frame in pairs(capture_frames) do
  if frame > max_frame then
    max_frame = frame
  end
end

local manifest = assert(io.open(out_dir .. "/capture_manifest.txt", "wb"))
manifest:write("emulator=fceux\n")
manifest:write("script=tools/fceux_capture.lua\n")
manifest:write("replay=", replay_path or "", "\n")
manifest:write("frames=", capture_frames_env, "\n")
manifest:write("max_frame=", tostring(max_frame), "\n")

FCEU.speedmode("maximum")

for frame = 1, max_frame do
  current_frame = frame
  local buttons = replay_frames[frame] or {}
  joypad.set(1, buttons)
  FCEU.frameadvance()

  if capture_frames[frame] then
    write_ppm_from_gd(frame)
    write_ram_dump(frame)
    write_ppu_dump(frame)
    manifest:write(string.format("captured_frame=%d\n", frame))
    manifest:flush()
  end
end

manifest:write("complete=1\n")
manifest:close()

if apu_trace_path and apu_trace_path ~= "" then
  write_file(apu_trace_path, "frame\taddr\tvalue\n" .. table.concat(apu_writes))
end

if done_path and done_path ~= "" then
  write_file(done_path, "complete\n")
end

-- Keep FCEUX alive briefly so the launcher can observe the done marker and
-- terminate the process cleanly from outside the Lua environment.
while true do
  FCEU.frameadvance()
end
