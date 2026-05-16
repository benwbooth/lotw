use lotw_port::cpu6502::{Bus, Cpu6502, StepResult};
use lotw_port::{rom::InesRom, sha256};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct BlockCandidate {
    id: usize,
    cpu_addr: u16,
    prg_offset: usize,
    byte_count: u16,
    first_opcode: u8,
}

#[derive(Debug, Clone)]
struct BlockState {
    cpu_addr: u16,
    prg_offset: usize,
    have_regs: bool,
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    have_ram: bool,
    ram: [u8; 0x800],
}

#[derive(Debug, Clone)]
struct BlockExecCase {
    replay: String,
    native_index: usize,
    block: BlockCandidate,
    state: BlockState,
    first_frame: u32,
    hit_ordinal: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecStatus {
    LeftBlock,
    Stopped,
    UnsupportedOpcode,
    StepLimit,
    InvalidBlock,
}

#[derive(Debug, Clone)]
struct ExternalWriteEvent {
    write_index: u32,
    external_index: u32,
    addr: u16,
    value: u8,
}

#[derive(Debug, Clone)]
struct ExecResult {
    status: ExecStatus,
    steps: u32,
    unsupported_opcode: u8,
    final_pc: u16,
    cycles: u64,
    writes: u32,
    ppu_writes: u32,
    apu_writes: u32,
    mapper_writes: u32,
    unmapped_reads: u32,
    state_applied: bool,
    final_a: u8,
    final_x: u8,
    final_y: u8,
    final_p: u8,
    final_s: u8,
    final_ram_sha256: [u8; 32],
    external_writes: Vec<ExternalWriteEvent>,
}

impl Default for ExecResult {
    fn default() -> Self {
        Self {
            status: ExecStatus::StepLimit,
            steps: 0,
            unsupported_opcode: 0,
            final_pc: 0,
            cycles: 0,
            writes: 0,
            ppu_writes: 0,
            apu_writes: 0,
            mapper_writes: 0,
            unmapped_reads: 0,
            state_applied: false,
            final_a: 0,
            final_x: 0,
            final_y: 0,
            final_p: 0,
            final_s: 0,
            final_ram_sha256: [0; 32],
            external_writes: Vec::new(),
        }
    }
}

struct BlockBus<'a> {
    rom: &'a InesRom,
    block: &'a BlockCandidate,
    ram: [u8; 0x800],
    ppu_regs: [u8; 8],
    apu_regs: [u8; 0x18],
    writes: u32,
    ppu_writes: u32,
    apu_writes: u32,
    mapper_writes: u32,
    unmapped_reads: u32,
    external_writes: Vec<ExternalWriteEvent>,
}

pub fn run(
    rom_path: &Path,
    blocks_path: &Path,
    out_dir: &Path,
    max_steps: u32,
    states_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rom, rom_sha256) = load_rom(rom_path)?;
    let blocks = read_blocks(blocks_path)?;
    if blocks.is_empty() {
        return Err("block_exec: no block candidates".into());
    }
    let states = match states_path {
        Some(path) => read_states(path)?,
        None => Vec::new(),
    };
    let results = blocks
        .iter()
        .map(|block| execute_block(&rom, block, find_state(&states, block), max_steps))
        .collect::<Vec<_>>();

    recreate_dir(out_dir)?;
    write_report(
        out_dir,
        &rom_sha256,
        blocks_path,
        states_path,
        &blocks,
        &results,
    )?;
    println!("block_exec: wrote {}", out_dir.display());
    println!(
        "block_exec: {} candidates checked with max_steps={}",
        blocks.len(),
        max_steps
    );
    Ok(())
}

pub fn run_case_states(
    rom_path: &Path,
    cases_path: &Path,
    out_dir: &Path,
    max_steps: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rom, rom_sha256) = load_rom(rom_path)?;
    let cases = read_case_states(cases_path)?;
    if cases.is_empty() {
        return Err("block_exec: no explicit state cases".into());
    }
    let results = cases
        .iter()
        .map(|case| execute_block(&rom, &case.block, Some(&case.state), max_steps))
        .collect::<Vec<_>>();

    recreate_dir(out_dir)?;
    write_case_state_report(out_dir, &rom_sha256, cases_path, &cases, &results)?;
    println!("block_exec: wrote {}", out_dir.display());
    println!(
        "block_exec: {} explicit state cases checked with max_steps={}",
        cases.len(),
        max_steps
    );
    Ok(())
}

pub fn run_cli(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [flag, rom, cases, out_dir] if flag == "--case-states" => {
            run_case_states(Path::new(rom), Path::new(cases), Path::new(out_dir), 64)
        }
        [flag, rom, cases, out_dir, max_steps] if flag == "--case-states" => run_case_states(
            Path::new(rom),
            Path::new(cases),
            Path::new(out_dir),
            max_steps.parse().unwrap_or(64).max(1),
        ),
        [rom, blocks, out_dir] => run(
            Path::new(rom),
            Path::new(blocks),
            Path::new(out_dir),
            64,
            None,
        ),
        [rom, blocks, out_dir, max_steps] => run(
            Path::new(rom),
            Path::new(blocks),
            Path::new(out_dir),
            max_steps.parse().unwrap_or(64).max(1),
            None,
        ),
        [rom, blocks, out_dir, max_steps, states] => run(
            Path::new(rom),
            Path::new(blocks),
            Path::new(out_dir),
            max_steps.parse().unwrap_or(64).max(1),
            Some(Path::new(states)),
        ),
        _ => Err("Usage: block-exec <rom.nes> <block_candidates.tsv> <out_dir> [max_steps] [label_states.tsv]\n       block-exec --case-states <rom.nes> <block_state_cases.tsv> <out_dir> [max_steps]".into()),
    }
}

fn load_rom(path: &Path) -> Result<(InesRom, String), Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    let sha256_hex = sha256::digest_hex(&bytes);
    Ok((InesRom::parse(&bytes)?, sha256_hex))
}

fn recreate_dir(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)
}

fn split_tsv(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn invalid_tsv<T>(path: &Path, line_no: usize, actual: usize, expected: usize) -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "{}:{line_no} has {actual} fields, expected at least {expected}",
            path.display()
        ),
    ))
}

fn parse_hex_u32(value: &str) -> Option<u32> {
    if value.is_empty() || value == "unknown" {
        return None;
    }
    u32::from_str_radix(value, 16).ok()
}

fn parse_hex_u16(value: &str) -> Option<u16> {
    parse_hex_u32(value).and_then(|v| u16::try_from(v).ok())
}

fn parse_hex_u8(value: &str) -> Option<u8> {
    parse_hex_u32(value).and_then(|v| u8::try_from(v).ok())
}

fn parse_ram_hex(value: &str) -> Option<[u8; 0x800]> {
    if value.len() != 0x1000 {
        return None;
    }
    let mut ram = [0u8; 0x800];
    for (index, byte) in ram.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(ram)
}

fn write_ram_hex(file: &mut fs::File, ram: &[u8; 0x800]) -> io::Result<()> {
    for byte in ram {
        write!(file, "{byte:02X}")?;
    }
    Ok(())
}

fn read_blocks(path: &Path) -> io::Result<Vec<BlockCandidate>> {
    let text = fs::read_to_string(path)?;
    let mut blocks = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 7 {
            return invalid_tsv(path, line_no + 1, fields.len(), 7);
        }
        let Some(cpu_addr) = parse_hex_u16(fields[1]) else {
            continue;
        };
        let Some(first_opcode) = parse_hex_u8(fields[6]) else {
            continue;
        };
        let Ok(id) = fields[0].parse::<usize>() else {
            continue;
        };
        let Some(prg_offset) = parse_hex_u32(fields[2]).map(|v| v as usize) else {
            continue;
        };
        let Ok(byte_count) = fields[3].parse::<u16>() else {
            continue;
        };
        blocks.push(BlockCandidate {
            id,
            cpu_addr,
            prg_offset,
            byte_count,
            first_opcode,
        });
    }
    Ok(blocks)
}

fn read_states(path: &Path) -> io::Result<Vec<BlockState>> {
    let text = fs::read_to_string(path)?;
    let mut states = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() < 10 {
            return invalid_tsv(path, line_no + 1, fields.len(), 10);
        }
        let Some(cpu_addr) = parse_hex_u16(fields[0]) else {
            continue;
        };
        let Some(prg_offset) = parse_hex_u32(fields[1]).map(|v| v as usize) else {
            continue;
        };
        let regs = parse_hex_u16(fields[3]).and_then(|pc| {
            Some((
                pc,
                parse_hex_u8(fields[4])?,
                parse_hex_u8(fields[5])?,
                parse_hex_u8(fields[6])?,
                parse_hex_u8(fields[7])?,
                parse_hex_u8(fields[8])?,
            ))
        });
        let ram = parse_ram_hex(fields[9]);
        states.push(BlockState {
            cpu_addr,
            prg_offset,
            have_regs: regs.is_some(),
            pc: regs.map(|regs| regs.0).unwrap_or(0),
            a: regs.map(|regs| regs.1).unwrap_or(0),
            x: regs.map(|regs| regs.2).unwrap_or(0),
            y: regs.map(|regs| regs.3).unwrap_or(0),
            p: regs.map(|regs| regs.4).unwrap_or(0),
            s: regs.map(|regs| regs.5).unwrap_or(0),
            have_ram: ram.is_some(),
            ram: ram.unwrap_or([0; 0x800]),
        });
    }
    Ok(states)
}

fn read_case_states(path: &Path) -> io::Result<Vec<BlockExecCase>> {
    let text = fs::read_to_string(path)?;
    let mut cases = Vec::new();
    for (line_no, line) in text.lines().enumerate().skip(1) {
        let fields = split_tsv(line);
        if fields.len() != 14 {
            return invalid_tsv(path, line_no + 1, fields.len(), 14);
        }
        let native_index = fields[1].parse::<usize>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} invalid native index: {err}",
                    path.display(),
                    line_no + 1
                ),
            )
        })?;
        let cpu_addr = parse_hex_u16(fields[2]).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid cpu address", path.display(), line_no + 1),
            )
        })?;
        let prg_offset = parse_hex_u32(fields[3]).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid prg offset", path.display(), line_no + 1),
            )
        })? as usize;
        let byte_count = fields[4].parse::<u16>().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}:{} invalid byte count: {err}",
                    path.display(),
                    line_no + 1
                ),
            )
        })?;
        let first_frame = fields[5].parse::<u32>().unwrap_or(0);
        let hit_ordinal = fields[6].parse::<u32>().unwrap_or(0);
        let pc = parse_hex_u16(fields[7]).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid pc", path.display(), line_no + 1),
            )
        })?;
        let ram = parse_ram_hex(fields[13]).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}:{} invalid RAM image", path.display(), line_no + 1),
            )
        })?;
        let block = BlockCandidate {
            id: native_index,
            cpu_addr,
            prg_offset,
            byte_count,
            first_opcode: 0,
        };
        let state = BlockState {
            cpu_addr,
            prg_offset,
            have_regs: true,
            pc,
            a: parse_hex_u8(fields[8]).unwrap_or(0),
            x: parse_hex_u8(fields[9]).unwrap_or(0),
            y: parse_hex_u8(fields[10]).unwrap_or(0),
            p: parse_hex_u8(fields[11]).unwrap_or(0),
            s: parse_hex_u8(fields[12]).unwrap_or(0),
            have_ram: true,
            ram,
        };
        cases.push(BlockExecCase {
            replay: fields[0].to_string(),
            native_index,
            block,
            state,
            first_frame,
            hit_ordinal,
        });
    }
    Ok(cases)
}

fn find_state<'a>(states: &'a [BlockState], block: &BlockCandidate) -> Option<&'a BlockState> {
    states
        .iter()
        .find(|state| state.cpu_addr == block.cpu_addr && state.prg_offset == block.prg_offset)
}

fn in_block_range(block: &BlockCandidate, addr: u16) -> bool {
    let start = block.cpu_addr as u32;
    let end = start + block.byte_count as u32;
    let addr = addr as u32;
    addr >= start && addr < end
}

fn is_terminal_self_jmp_block(rom: &InesRom, block: &BlockCandidate) -> bool {
    if block.byte_count != 3 || block.prg_offset + 3 > rom.prg_rom().len() {
        return false;
    }
    if rom.prg_rom()[block.prg_offset] != 0x4c {
        return false;
    }
    let target = u16::from_le_bytes([
        rom.prg_rom()[block.prg_offset + 1],
        rom.prg_rom()[block.prg_offset + 2],
    ]);
    target == block.cpu_addr
}

impl<'a> BlockBus<'a> {
    fn new(rom: &'a InesRom, block: &'a BlockCandidate) -> Self {
        Self {
            rom,
            block,
            ram: [0; 0x800],
            ppu_regs: [0; 8],
            apu_regs: [0; 0x18],
            writes: 0,
            ppu_writes: 0,
            apu_writes: 0,
            mapper_writes: 0,
            unmapped_reads: 0,
            external_writes: Vec::new(),
        }
    }

    fn map_prg_read(&self, addr: u16) -> Option<u8> {
        let prg = self.rom.prg_rom();
        let window_base_addr = self.block.cpu_addr & 0xe000;
        let window_delta = (self.block.cpu_addr - window_base_addr) as usize;
        let window_base_offset = self.block.prg_offset.checked_sub(window_delta);

        if in_block_range(self.block, addr) {
            let off = self.block.prg_offset + usize::from(addr - self.block.cpu_addr);
            if off < prg.len() {
                return Some(prg[off]);
            }
        }

        if addr >= window_base_addr && u32::from(addr) < u32::from(window_base_addr) + 0x2000 {
            if let Some(base) = window_base_offset {
                let off = base + usize::from(addr - window_base_addr);
                if off < prg.len() {
                    return Some(prg[off]);
                }
            }
        }

        if addr >= 0xc000 && prg.len() >= 0x4000 {
            let off = prg.len() - 0x4000 + usize::from(addr - 0xc000);
            if off < prg.len() {
                return Some(prg[off]);
            }
        }

        None
    }

    fn record_external_write(&mut self, addr: u16, value: u8) {
        self.external_writes.push(ExternalWriteEvent {
            write_index: self.writes,
            external_index: self.external_writes.len() as u32 + 1,
            addr,
            value,
        });
    }
}

impl Bus for BlockBus<'_> {
    fn read(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            return self.ram[usize::from(addr & 0x07ff)];
        }
        if (0x2000..0x4000).contains(&addr) {
            return self.ppu_regs[usize::from(addr & 0x0007)];
        }
        if (0x4000..=0x4017).contains(&addr) {
            return self.apu_regs[usize::from(addr - 0x4000)];
        }
        if addr >= 0x8000 {
            if let Some(value) = self.map_prg_read(addr) {
                return value;
            }
        }
        self.unmapped_reads += 1;
        0
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.writes += 1;
        if addr < 0x2000 {
            self.ram[usize::from(addr & 0x07ff)] = value;
            return;
        }
        if (0x2000..0x4000).contains(&addr) {
            self.ppu_writes += 1;
            self.ppu_regs[usize::from(addr & 0x0007)] = value;
            self.record_external_write(addr, value);
            return;
        }
        if (0x4000..=0x4017).contains(&addr) {
            self.apu_writes += 1;
            self.apu_regs[usize::from(addr - 0x4000)] = value;
            self.record_external_write(addr, value);
            return;
        }
        if addr >= 0x8000 {
            self.mapper_writes += 1;
            self.record_external_write(addr, value);
        }
    }
}

fn execute_block(
    rom: &InesRom,
    block: &BlockCandidate,
    state: Option<&BlockState>,
    max_steps: u32,
) -> ExecResult {
    let mut bus = BlockBus::new(rom, block);
    let mut result = ExecResult::default();

    if block.byte_count == 0
        || block.prg_offset >= rom.prg_rom().len()
        || block.prg_offset + usize::from(block.byte_count) > rom.prg_rom().len()
    {
        let cpu = Cpu6502::default();
        capture_final_state(&mut result, &cpu, &bus);
        result.status = ExecStatus::InvalidBlock;
        result.final_pc = block.cpu_addr;
        return result;
    }

    if let Some(state) = state {
        if state.have_ram {
            bus.ram = state.ram;
            result.state_applied = true;
        }
    }

    let mut cpu = Cpu6502 {
        pc: block.cpu_addr,
        ..Cpu6502::default()
    };
    if let Some(state) = state {
        if state.have_regs {
            cpu.a = state.a;
            cpu.x = state.x;
            cpu.y = state.y;
            cpu.p = state.p;
            cpu.s = state.s;
            cpu.pc = if state.pc == 0 {
                block.cpu_addr
            } else {
                state.pc
            };
            result.state_applied = true;
        }
    }

    for i in 0..max_steps {
        let pc_before_step = cpu.pc;
        if !in_block_range(block, cpu.pc) {
            result.status = ExecStatus::LeftBlock;
            break;
        }
        match cpu.step(&mut bus) {
            StepResult::Ok => {}
            StepResult::Stopped => {
                result.steps += 1;
                result.status = ExecStatus::Stopped;
                break;
            }
            StepResult::UnsupportedOpcode(opcode) => {
                result.steps += 1;
                result.status = ExecStatus::UnsupportedOpcode;
                result.unsupported_opcode = opcode;
                break;
            }
        }
        result.steps += 1;
        if pc_before_step == block.cpu_addr
            && cpu.pc == block.cpu_addr
            && is_terminal_self_jmp_block(rom, block)
        {
            result.status = ExecStatus::LeftBlock;
            break;
        }
        if i + 1 == max_steps {
            result.status = ExecStatus::StepLimit;
        }
    }

    capture_final_state(&mut result, &cpu, &bus);
    result.cycles = cpu.cycles;
    result.writes = bus.writes;
    result.ppu_writes = bus.ppu_writes;
    result.apu_writes = bus.apu_writes;
    result.mapper_writes = bus.mapper_writes;
    result.unmapped_reads = bus.unmapped_reads;
    result.external_writes = bus.external_writes;
    result
}

fn capture_final_state(result: &mut ExecResult, cpu: &Cpu6502, bus: &BlockBus<'_>) {
    result.final_pc = cpu.pc;
    result.final_a = cpu.a;
    result.final_x = cpu.x;
    result.final_y = cpu.y;
    result.final_p = cpu.p;
    result.final_s = cpu.s;
    result.final_ram_sha256 = sha256::digest(&bus.ram);
}

fn status_name(status: ExecStatus) -> &'static str {
    match status {
        ExecStatus::LeftBlock => "left_block",
        ExecStatus::Stopped => "stopped",
        ExecStatus::UnsupportedOpcode => "unsupported_opcode",
        ExecStatus::StepLimit => "step_limit",
        ExecStatus::InvalidBlock => "invalid_block",
    }
}

fn status_index(status: ExecStatus) -> usize {
    match status {
        ExecStatus::LeftBlock => 0,
        ExecStatus::Stopped => 1,
        ExecStatus::UnsupportedOpcode => 2,
        ExecStatus::StepLimit => 3,
        ExecStatus::InvalidBlock => 4,
    }
}

fn external_write_kind(addr: u16) -> &'static str {
    if (0x2000..0x4000).contains(&addr) {
        "ppu"
    } else if (0x4000..=0x4017).contains(&addr) {
        "apu"
    } else if addr >= 0x8000 {
        "mapper"
    } else {
        "unknown"
    }
}

fn write_report(
    out_dir: &Path,
    rom_sha256: &str,
    blocks_path: &Path,
    states_path: Option<&Path>,
    blocks: &[BlockCandidate],
    results: &[ExecResult],
) -> io::Result<()> {
    let mut manifest = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(manifest, "sha256={rom_sha256}")?;
    writeln!(manifest, "blocks={}", blocks_path.display())?;
    writeln!(
        manifest,
        "states={}",
        states_path.map_or(String::new(), |path| path.display().to_string())
    )?;
    write_manifest_tail(
        &mut manifest,
        results,
        "block_count",
        blocks.len(),
        "tooling-only semantic block execution",
    )?;

    let mut tsv = fs::File::create(out_dir.join("block_exec.tsv"))?;
    writeln!(tsv, "id\tcpu_addr\tprg_offset\tbytes\tfirst_opcode\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256")?;
    for (block, result) in blocks.iter().zip(results) {
        write!(
            tsv,
            "{}\t{:04X}\t{:05X}\t{}\t{:02X}\t{}\t{}\t",
            block.id,
            block.cpu_addr,
            block.prg_offset,
            block.byte_count,
            block.first_opcode,
            status_name(result.status),
            result.steps
        )?;
        if result.status == ExecStatus::UnsupportedOpcode {
            write!(tsv, "{:02X}", result.unsupported_opcode)?;
        }
        writeln!(
            tsv,
            "\t{:04X}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{}",
            result.final_pc,
            result.cycles,
            result.writes,
            result.ppu_writes,
            result.apu_writes,
            result.mapper_writes,
            result.unmapped_reads,
            u8::from(result.state_applied),
            result.final_a,
            result.final_x,
            result.final_y,
            result.final_p,
            result.final_s,
            sha256::hex(&result.final_ram_sha256)
        )?;
    }

    let mut external = fs::File::create(out_dir.join("block_external_writes.tsv"))?;
    writeln!(
        external,
        "id\tcpu_addr\tprg_offset\twrite_index\texternal_index\tkind\taddr\tvalue"
    )?;
    for (block, result) in blocks.iter().zip(results) {
        for event in &result.external_writes {
            writeln!(
                external,
                "{}\t{:04X}\t{:05X}\t{}\t{}\t{}\t{:04X}\t{:02X}",
                block.id,
                block.cpu_addr,
                block.prg_offset,
                event.write_index,
                event.external_index,
                external_write_kind(event.addr),
                event.addr,
                event.value
            )?;
        }
    }
    write_unsupported(out_dir, results)
}

fn write_manifest_tail(
    file: &mut fs::File,
    results: &[ExecResult],
    count_key: &str,
    count: usize,
    scope: &str,
) -> io::Result<()> {
    let mut status_counts = [0u32; 5];
    let mut external_write_rows = 0usize;
    for result in results {
        status_counts[status_index(result.status)] += 1;
        external_write_rows += result.external_writes.len();
    }
    writeln!(file, "{count_key}={count}")?;
    writeln!(file, "left_block={}", status_counts[0])?;
    writeln!(file, "stopped={}", status_counts[1])?;
    writeln!(file, "unsupported_opcode={}", status_counts[2])?;
    writeln!(file, "step_limit={}", status_counts[3])?;
    writeln!(file, "invalid_block={}", status_counts[4])?;
    writeln!(file, "external_write_rows={external_write_rows}")?;
    writeln!(file, "external_write_alloc_failed=0")?;
    writeln!(file, "scope={scope}")?;
    writeln!(file, "complete=1")?;
    Ok(())
}

fn write_case_state_report(
    out_dir: &Path,
    rom_sha256: &str,
    cases_path: &Path,
    cases: &[BlockExecCase],
    results: &[ExecResult],
) -> io::Result<()> {
    let mut manifest = fs::File::create(out_dir.join("manifest.txt"))?;
    writeln!(manifest, "sha256={rom_sha256}")?;
    writeln!(manifest, "cases={}", cases_path.display())?;
    write_manifest_tail(
        &mut manifest,
        results,
        "case_count",
        cases.len(),
        "tooling-only semantic block execution from explicit states",
    )?;

    let mut tsv = fs::File::create(out_dir.join("block_state_exec.tsv"))?;
    writeln!(tsv, "replay\tnative_index\tcpu_addr\tprg_offset\tbytes\tfirst_frame\thit_ordinal\tpc\ta\tx\ty\tp\ts\tram_0000_07ff\tstatus\tsteps\tunsupported_opcode\tfinal_pc\tcycles\twrites\tppu_writes\tapu_writes\tmapper_writes\tunmapped_reads\tstate_applied\tfinal_a\tfinal_x\tfinal_y\tfinal_p\tfinal_s\tfinal_ram_sha256")?;
    for (case, result) in cases.iter().zip(results) {
        write!(
            tsv,
            "{}\t{}\t{:04X}\t{:05X}\t{}\t{}\t{}\t{:04X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t",
            case.replay,
            case.native_index,
            case.block.cpu_addr,
            case.block.prg_offset,
            case.block.byte_count,
            case.first_frame,
            case.hit_ordinal,
            case.state.pc,
            case.state.a,
            case.state.x,
            case.state.y,
            case.state.p,
            case.state.s
        )?;
        write_ram_hex(&mut tsv, &case.state.ram)?;
        write!(tsv, "\t{}\t{}\t", status_name(result.status), result.steps)?;
        if result.status == ExecStatus::UnsupportedOpcode {
            write!(tsv, "{:02X}", result.unsupported_opcode)?;
        }
        writeln!(
            tsv,
            "\t{:04X}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{:02X}\t{}",
            result.final_pc,
            result.cycles,
            result.writes,
            result.ppu_writes,
            result.apu_writes,
            result.mapper_writes,
            result.unmapped_reads,
            u8::from(result.state_applied),
            result.final_a,
            result.final_x,
            result.final_y,
            result.final_p,
            result.final_s,
            sha256::hex(&result.final_ram_sha256)
        )?;
    }

    let mut external = fs::File::create(out_dir.join("block_state_external_writes.tsv"))?;
    writeln!(external, "replay\tnative_index\tcpu_addr\tprg_offset\tfirst_frame\thit_ordinal\twrite_index\texternal_index\tkind\taddr\tvalue")?;
    for (case, result) in cases.iter().zip(results) {
        for event in &result.external_writes {
            writeln!(
                external,
                "{}\t{}\t{:04X}\t{:05X}\t{}\t{}\t{}\t{}\t{}\t{:04X}\t{:02X}",
                case.replay,
                case.native_index,
                case.block.cpu_addr,
                case.block.prg_offset,
                case.first_frame,
                case.hit_ordinal,
                event.write_index,
                event.external_index,
                external_write_kind(event.addr),
                event.addr,
                event.value
            )?;
        }
    }
    write_unsupported(out_dir, results)
}

fn write_unsupported(out_dir: &Path, results: &[ExecResult]) -> io::Result<()> {
    let mut opcode_counts = [0u32; 256];
    for result in results {
        if result.status == ExecStatus::UnsupportedOpcode {
            opcode_counts[result.unsupported_opcode as usize] += 1;
        }
    }
    let mut file = fs::File::create(out_dir.join("unsupported_opcodes.tsv"))?;
    writeln!(file, "opcode\tcount")?;
    for (opcode, count) in opcode_counts.iter().enumerate() {
        if *count != 0 {
            writeln!(file, "{opcode:02X}\t{count}")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rom_with_prg(prg: &[u8]) -> InesRom {
        let mut bytes = vec![0u8; 16 + 0x8000 + 0x2000];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 0x40;
        bytes[16..16 + prg.len()].copy_from_slice(prg);
        InesRom::parse(&bytes).unwrap()
    }

    #[test]
    fn executes_block_until_pc_leaves_range() {
        let rom = rom_with_prg(&[0xa9, 0x42, 0x85, 0x10, 0xea]);
        let block = BlockCandidate {
            id: 0,
            cpu_addr: 0x8000,
            prg_offset: 0,
            byte_count: 4,
            first_opcode: 0xa9,
        };
        let result = execute_block(&rom, &block, None, 8);
        assert_eq!(result.status, ExecStatus::LeftBlock);
        assert_eq!(result.final_pc, 0x8004);
        assert_eq!(result.final_a, 0x42);
        assert_eq!(result.writes, 1);
        assert_eq!(
            result.final_ram_sha256,
            sha256::digest(&{
                let mut ram = [0u8; 0x800];
                ram[0x10] = 0x42;
                ram
            })
        );
    }

    #[test]
    fn records_external_writes() {
        let rom = rom_with_prg(&[0xa9, 0x80, 0x8d, 0x00, 0x20]);
        let block = BlockCandidate {
            id: 0,
            cpu_addr: 0x8000,
            prg_offset: 0,
            byte_count: 5,
            first_opcode: 0xa9,
        };
        let result = execute_block(&rom, &block, None, 8);
        assert_eq!(result.status, ExecStatus::LeftBlock);
        assert_eq!(result.ppu_writes, 1);
        assert_eq!(result.external_writes.len(), 1);
        assert_eq!(result.external_writes[0].addr, 0x2000);
        assert_eq!(result.external_writes[0].value, 0x80);
    }
}
