#!/usr/bin/env python3
"""Curated symbol map for the matching disassembly.

Names resolve to fixed addresses, so substituting them in the .s changes no bytes
— the ld65 round-trip stays identical. Curated from the analysis workflow
(docs/analysis/*, byte-verified) and recon. Conventions: hardware REGISTERS
UPPER_SNAKE; RAM variables, routines, and data tables lower_snake.

Maps:
- REGS_RAM: addr -> name for data operands (registers, RAM vars, data-table bases)
- ROUTINES: addr -> name for code labels (renames L_xxxx)
"""

REGS_RAM: dict[int, str] = {
    # ---- NES hardware registers ----
    0x2000: "PPUCTRL", 0x2001: "PPUMASK", 0x2002: "PPUSTATUS", 0x2003: "OAMADDR",
    0x2004: "OAMDATA", 0x2005: "PPUSCROLL", 0x2006: "PPUADDR", 0x2007: "PPUDATA",
    0x4000: "SQ1_VOL", 0x4001: "SQ1_SWEEP", 0x4002: "SQ1_LO", 0x4003: "SQ1_HI",
    0x4004: "SQ2_VOL", 0x4005: "SQ2_SWEEP", 0x4006: "SQ2_LO", 0x4007: "SQ2_HI",
    0x4008: "TRI_LINEAR", 0x400A: "TRI_LO", 0x400B: "TRI_HI",
    0x400C: "NOISE_VOL", 0x400E: "NOISE_LO", 0x400F: "NOISE_HI",
    0x4010: "DMC_FREQ", 0x4011: "DMC_RAW", 0x4012: "DMC_START", 0x4013: "DMC_LEN",
    0x4014: "OAMDMA", 0x4015: "APU_STATUS", 0x4016: "JOY1", 0x4017: "APU_FRAME",
    # MMC3 (mapper 4)
    0x8000: "MMC3_BANK_SELECT", 0x8001: "MMC3_BANK_DATA",
    0xA000: "MMC3_MIRROR", 0xA001: "MMC3_PRGRAM",
    0xC000: "MMC3_IRQ_LATCH", 0xC001: "MMC3_IRQ_RELOAD",
    0xE000: "MMC3_IRQ_DISABLE", 0xE001: "MMC3_IRQ_ENABLE",

    # ---- NMI / VRAM job queue (analysis: nmi-dispatch, byte-verified) ----
    0x0016: "vram_dst_lo", 0x0017: "vram_dst_hi",
    0x0018: "vram_src_lo", 0x0019: "vram_src_hi", 0x001A: "vram_len",
    0x0023: "ppuctrl_shadow", 0x0025: "mmc3_select_shadow",
    0x0026: "nmi_scratch", 0x0028: "nmi_vram_req",

    # ---- MMC3 register shadow R0..R7 (committed each frame by ppu_commit_banks) ----
    0x002A: "mmc3_r0_shadow", 0x002B: "mmc3_r1_shadow", 0x002C: "mmc3_r2_shadow",
    0x002D: "mmc3_r3_shadow", 0x002E: "mmc3_r4_shadow", 0x002F: "mmc3_r5_shadow",
    0x0030: "mmc3_r6_shadow", 0x0031: "mmc3_r7_shadow",
    0x0034: "snd_music_bank0", 0x0035: "snd_music_bank1",

    # ---- RNG (byte-verified at rng_update $CC64) ----
    0x0038: "rng_count", 0x0039: "rng_s0", 0x003A: "rng_s1", 0x003B: "rng_s2",

    # ---- player / game state (Data Crystal RAM map; load-bearing ones verified) ----
    0x0040: "cur_character",
    0x0043: "player_x_fine", 0x0044: "player_x_tile", 0x0045: "player_y",
    0x0047: "map_screen_x", 0x0048: "map_screen_y",
    0x0051: "carried_item0", 0x0052: "carried_item1", 0x0053: "carried_item2",
    0x0055: "equipped_item",
    0x0058: "health", 0x0059: "magic", 0x005A: "gold", 0x005B: "keys",
    0x005C: "stat_jump", 0x005D: "stat_strength", 0x005E: "shots_allowed", 0x005F: "shot_range",
    0x0060: "inventory_counts",
    0x007B: "scroll_x_fine", 0x007C: "scroll_x_tile",
    0x00F2: "boss_life",

    # ---- save block ($0300-$0321; password payload) ----
    0x0300: "save_inventory", 0x0310: "save_inventory_counts",
    0x0320: "save_keys", 0x0321: "save_gold",
    0x0400: "sprite_tables",

    # ---- data-table bases (named constants; referenced from code) ----
    0xD244: "nmi_vram_dispatch_table",   # NMI VRAM-op jump table
    0xFDB1: "note_period_table",         # sound: equal-tempered periods
    0xEFE7: "drop_item_table",           # enemy drop roll -> item
}

# Code labels (CPU addr -> name). From analysis: routine-names / nmi-dispatch /
# rng-drops / table-0x14000. Fixed banks ($C000-$FFFF) unless noted.
ROUTINES: dict[int, str] = {
    # boot / main
    0xFFE0: "reset", 0xC000: "main_init", 0xC06D: "main_loop_dispatch",
    0xD1C8: "ram_state_init", 0xD42B: "game_update",
    # NMI
    0xD1FE: "nmi_handler", 0xD351: "nmi_tail", 0xD408: "frame_counters",
    0xD41D: "ppu_commit_banks", 0xD36E: "statusbar_split",
    # NMI VRAM upload helpers
    0xD252: "vram_fill_run", 0xD25F: "vram_upload_palette", 0xD290: "vram_upload_hud",
    0xD2E5: "vram_blit_stack", 0xD334: "vram_copy_indirect", 0xD344: "vram_poke2",
    # far-call dispatchers
    0xCC9C: "farcall_bank_0C0D", 0xCCE4: "farcall_return_home",
    0xCD08: "farcall_bank_0C0D_seed", 0xC833: "farcall_bank_09_r7",
    # input / rng / ppu jobs / scene
    0xCC43: "read_controllers", 0xCC64: "rng_update", 0xCC8F: "queue_ppu_job_and_wait",
    0xC909: "text_attr_build", 0xC8F2: "scene_assemble", 0xC871: "metasprite_build",
    # enemy drops
    0xEF85: "enemy_drop_choose", 0xEFAC: "drop_money_chooser", 0xEFC4: "item_spawn_setup",
    # sound driver
    0xF89A: "sound_tick", 0xFC08: "song_init", 0xFA60: "sfx_overlay_voice",
    0xFD74: "sound_set_default_banks", 0xFD87: "sound_set_song_banks",
    0xFD9C: "sound_restore_game_banks",
    # bank 13 ($A000 window)
    0xA400: "oam_sprite_engine",
}


# Data regions inside the code banks (from the completeness-audit workflow).
# Emitted as comment delimiters before the .byte block at each address so code
# and data are visually separated. addr -> "name (kind)".
DATA_REGIONS: dict[int, str] = {
    # bank 13 ($A000-$BFFF)
    0xA000: "title_credits_nametable (data)",
    0xAAFC: "sprite_data (sprite)",
    0xB0AC: "per_character_carried_item_table (data)",
    0xB4AF: "gameover_menu_text (text, ASCII+$A0)",
    0xB6FC: "oam_tables_and_credits_nametable (data)",
    0xB79C: "ending_credits_text (text)",
    0xBD89: "bank13_zero_pad (data)",
    0xBFA4: "bank13_tail_records (data)",
    # fixed banks 14+15 ($C000-$FFFF)
    0xC034: "dead_mmc3_fragment (unused)",
    0xD244: "nmi_vram_dispatch_table (jump_table)",
    0xDB06: "item_action_dispatch_tables (jump_table)",
    0xEAAD: "boss_state_dispatch_table (jump_table)",
    0xEEB3: "sound_lookup_eeb3 (table)",
    0xEFE7: "drop_item_table (table)",
    0xF033: "phase_dispatch_table (jump_table)",
    0xFBBB: "sound_command_dispatch_table (jump_table)",
    0xFC00: "hud_menu_text (text, ASCII+$A0)",
    0xFDB1: "note_period_table + sound assets (sound)",
    0xFFEF: "reset_padding",
    0xFFFA: "cpu_vectors (nmi/reset/irq)",
}


def load_extra(path) -> None:
    """Optionally merge a JSON list of {addr_hex,name,kind}; curated names win."""
    import json
    from pathlib import Path
    p = Path(path)
    if not p.exists():
        return
    for s in json.loads(p.read_text()):
        addr = int(str(s["addr_hex"]).lower().lstrip("$").replace("0x", ""), 16)
        (ROUTINES if s["kind"] == "routine" else REGS_RAM).setdefault(addr, s["name"])
