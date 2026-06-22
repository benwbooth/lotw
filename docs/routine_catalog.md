# Routine Catalog

This catalog is the working map for the numbered `routine_####` functions that
remain in `src/game.rs` and `src/native.rs`.

The numbers are stable port labels, not good semantic names. A routine should
only be renamed or given a public semantic alias when its dataflow and call
sites are understood well enough that the name is more useful than the numeric
label. Until then, this file is the source of truth for the current hypothesis.

## Status Legend

- `named`: the Rust code already has a semantic name.
- `inferred`: the role follows from call sites, memory reads/writes, and tests.
- `cluster`: the routine belongs to a known subsystem, but its exact local role
  still needs trace evidence or a narrow regression test.
- `unknown`: do not rename yet.

## Runtime Skeleton

The native executable runs these major phases:

1. `main_init` initializes RAM, banks, PPU/APU state, builds the starting room,
   and enters `native::main_loop_dispatch`.
2. `native::main_loop_dispatch` is the foreground game loop.
3. `frame::FrameRunner` parks the foreground loop at explicit waits.
4. `vblank_commit` is the NMI-style interrupt boundary. It saves the current
   `RoutineContext`, commits OAM/VRAM/audio/frame timers, then restores the
   interrupted context.
5. `game_update` processes live input, movement, item actions, character swaps,
   room collisions, and follow-up state changes for one foreground tick.
6. `routine_0266`, `routine_0212`, and `routine_0271` update player shots,
   room actors, and the special tile-removal projectile before the render pass.

## Memory Map

The port still uses the original RAM layout. These names are inferred from the
Rust dataflow and should be preferred in comments and future renames.

| Address | Meaning | Evidence |
|---|---|---|
| `0x20` | current buttons | `read_controllers`, `game_update`, tests |
| `0x23` | PPU control shadow | `statusbar_split`, VRAM upload routines |
| `0x24` | PPU mask shadow | renderer and sprite-zero handling |
| `0x25` | selected PRG-bank register | bank switching helpers |
| `0x26` | PPU status latch | `vblank_commit` |
| `0x28` | queued VRAM job id | `queue_ppu_job_and_wait`, `vblank_commit` |
| `0x2A..0x2F` | PPU bank shadows | `text_attr_build`, `ppu_commit_banks` |
| `0x30..0x33` | PRG bank shadows and saved banks | farcall helpers |
| `0x36` | frame wait countdown | `frame` module, `vblank_commit_tail` |
| `0x40` | current family member index | character select and actor logic |
| `0x41` | room actor availability mask | scene assembly and actor spawn |
| `0x43..0x45` | player sub-tile, tile-x, pixel-y | movement/collision/projectiles |
| `0x46` | action lockout timer | `game_update`, item pickup feedback |
| `0x47..0x48` | map-space room coordinates | room assembly |
| `0x49..0x4F` | movement/action scratch | movement and collision checks |
| `0x51..0x55` | carried items and selected slot | inventory and action routines |
| `0x58` | health-like resource counter | `routine_0205`, native helpers |
| `0x59` | magic-like resource counter | `routine_0204`, projectile/action use |
| `0x5A..0x5B` | secondary counters | inventory and item accounting |
| `0x70..0x74` | current room tile/action ids | scene assembly and item actions |
| `0x75..0x78` | room map pointer and saved pointer | tile addressing and scene assembly |
| `0x79..0x7A` | current metasprite/table pointer | sprite build and room assets |
| `0x7B..0x7C` | scroll/world position | sprite projection and room render |
| `0x84..0x8C` | frame timers | `frame_counters` |
| `0x8D..0x8F` | audio/sfx mode and pending sfx id | sound tick and sfx overlay |
| `0x90..0x92` | sfx/music scratch | sound init and overlay voice |
| `0x93..0xC6` | music channel state | `routine_0273..routine_0289` |
| `0xD3..0xDD` | sfx overlay channel state | `sfx_overlay_voice` |
| `0xE3..0xE9` | object-loop slot index and pointers | actor update loops |
| `0xED..0xFC` | current object scratch slot | copied by `routine_0213/0214` |
| `0xFD` | held/directional action latch | input edge/action gating |
| `0x0200..0x02FF` | OAM staging | render and vblank commit |
| `0x0300..0x03FF` | persistent room/event bits | map progress and room flags |
| `0x0400..0x04AF` | 16-byte object slots | actors, items, projectiles, door slot |
| `0x04B0..` | pooled player projectile slots | `routine_0266..0268` |

## Complete Numbered Routine Coverage

The following ranges cover every numbered routine currently present. A range is
used when the routines form one tightly-coupled subsystem and individual names
would currently be weaker than the cluster name.

| Module | Routines | Status | Current role |
|---|---:|---|---|
| `game` | `0003`, `0005..0019` | cluster | opening/title scripted helpers, timed waits, cutscene sprite setup, and startup scene setup |
| `game` | `0021..0028`, `0030..0032` | cluster | password/title/menu support and first-room transition helpers |
| `native` | `0001`, `0002`, `0004`, `0020`, `0029` | inferred | high-level start flow, intro/menu flow, and blocking input gates rewritten around frame tasks |
| `native` | `0033`, `0034`, `0039`, `0045`, `0049`, `0050`, `0055` | inferred | title screen, family select, password entry, and start-game screen orchestration |
| `game` | `0035..0038`, `0040..0044`, `0046..0048` | cluster | family/password/menu visual and state helpers |
| `game` | `0051..0054`, `0056`, `0057` | cluster | transition, palette, and display setup helpers used by menu/start flows |
| `native` | `0058` | inferred | post-render PPU/status handling in the foreground loop |
| `game` | `0059..0066` | inferred | frame render pass, OAM clearing, background/object sprite projection, and palette/display setup |
| `native` | `0067..0072`, `0074` | inferred | room transition and item/inventory screen orchestration |
| `game` | `0073..0089` | inferred | VRAM/PPU setup, room render upload, palette updates, and room assembly helpers |
| `game` | `0090..0092` | inferred | tile address math and deferred frame work commit |
| `game` | `0093..0096` | inferred | HUD resource display update helpers |
| `game` | `0097..0103`, `0106..0108` | cluster | movement vector, tile lookup, direction, and frame/input helper routines |
| `native` | `0104`, `0105` | inferred | wait-for-release and wait-for-press input gates |
| `native` | `0109`, `0110` | inferred | object/player overlap search across live object slots |
| `game` | `0111..0116` | inferred | player/object hitbox and screen-bound checks |
| `game` | `0117..0123` | cluster | persistent room flag and room tile mutation helpers |
| `game` | `0124..0128` | inferred | item effect helpers and resource display refresh |
| `game` | `0129..0132` | cluster | inventory/status UI update helpers |
| `native` | `0133`, `0134` | inferred | inventory/status screen flow and return path |
| `game` | `0135..0147` | inferred | item action dispatch, item pickup/collection, actor contact, and room-interaction checks |
| `native` | `0148` | inferred | consume secondary counter/resource for an action |
| `game` | `0149..0150` | inferred | collectible dispatch and object slot clear on pickup |
| `game` | `0151..0162` | cluster | item-specific pickup/effect handlers |
| `native` | `0163` | inferred | player movement collision commit and contact feedback |
| `game` | `0164..0168` | cluster | tile collision probes, item-use gating, and family/item interaction helpers |
| `native` | `0169` | inferred | tile action dispatch, including item use and special projectile spawn |
| `game` | `0170..0173` | inferred | object spawn coordinate setup, room tile readback, and movement intent resolution |
| `native` | `0174..0177` | inferred | character swap/inventory selection flow |
| `native` | `0175` | inferred | inventory item compaction and carried-item reordering |
| `game` | `0178..0186` | cluster | inventory/menu cursor, item list, and status draw helpers |
| `native` | `0187..0191`, `0193`, `0194` | inferred | room transition/death/return-home state handling |
| `game` | `0192`, `0195..0203` | cluster | room transition, damage/effect, and score/resource effect helpers |
| `game` | `0204..0211` | inferred | resource/counter add, subtract, cap, decrement, and HUD sync |
| `game` | `0212` | inferred | main room actor scheduler |
| `game` | `0213`, `0214` | inferred | copy 16-byte object slot to/from scratch RAM |
| `game` | `0215..0219` | inferred | inactive actor spawn, respawn delay, boss dispatch, and normal actor tick |
| `game` | `0220..0229` | cluster | per-actor behavior handlers selected by room actor data |
| `game` | `0230..0239` | inferred | actor movement helper routines, velocity reset, and collision response |
| `native` | `0240` | inferred | high-bit/special actor update path |
| `game` | `0241..0249` | inferred | object motion prediction, animation, collision scan, and hit/damage application |
| `game` | `0250..0256` | inferred | terrain collision probes and actor movement validation |
| `game` | `0257`, `0258`, `0260..0265` | inferred | actor spawn/setup and multi-sprite boss/body slot composition |
| `native` | `0259` | inferred | inventory/equipment screen helper path |
| `game` | `0266` | inferred | pooled player projectile slot scheduler |
| `game` | `0267` | inferred | allocate/spawn a player projectile from current input and facing |
| `game` | `0268` | inferred | update one player projectile slot and clear it on expiry/collision |
| `game` | `0269` | inferred | project projectile position from player pose and velocity |
| `game` | `0270` | inferred | copy projectile direction bits into sprite attributes |
| `game` | `0271` | inferred | special tile-removal projectile scheduler |
| `game` | `0272` | inferred | update special projectile movement, collision, bounce, and tile replacement |
| `game` | `0273..0276` | inferred | four music/audio channel tickers |
| `game` | `0277..0282` | inferred | music bytecode/control command dispatcher |
| `game` | `0283..0289` | inferred | note period, envelope, duration, and silence helpers |

## Named Non-Numbered Routines

These already have semantic names and should be kept as the preferred call
surface when touching nearby code:

| Routine | Role |
|---|---|
| `farcall_bank_09_r7` | temporarily map bank 9 into PRG slot 7 and build a metasprite |
| `farcall_bank_0C0D_seed` | seed PRG banks 0x0C/0x0D into the bank shadows |
| `farcall_return_home` | restore saved PRG bank shadows after a farcall-style section |
| `frame_counters` | tick coarse frame timers once per second |
| `game_update` | foreground input/player/item update |
| `inc16_95` | increment the music stream pointer for the selected channel |
| `main_init` | hardware/RAM/bootstrap sequence and handoff to main loop |
| `metasprite_build` | build HUD/metasprite staging data for a queued VRAM upload |
| `ppu_commit_banks` | write all PPU bank shadows to the mapper |
| `ram_state_init` | initialize zero-page, palette, and RAM defaults from ROM tables |
| `read_controllers` | read replay/live input into the current button byte |
| `reset` | top-level reset entry |
| `rng_update` | update random source bounded by `r.value` |
| `scene_assemble` | rebuild room state from current map coordinates |
| `sfx_overlay_voice` | play pending sound effects over music channel state |
| `song_init` | initialize all music channels for the selected song id |
| `sound_tick` | per-frame music and sfx tick |
| `statusbar_split` | status-bar scroll/bank update plus audio tick |
| `text_attr_build` | derive room actor/tile/CHR metadata from the current room record |
| `vblank_commit` | NMI-style interrupt body for OAM, VRAM jobs, and tail work |
| `vblank_commit_tail` | common NMI tail: banks, status bar, sound, frame timers |
| `vram_*` | deferred VRAM job implementations |

## Next Renaming Targets

The safest concrete rename/alias batches are:

1. Projectile system: `routine_0266..0272`.
2. Object-slot scratch copy: `routine_0213`, `routine_0214`.
3. Resource/HUD counters: `routine_0204..0211`.
4. Tile address and collision probes: `routine_0090..0116`, `0250..0256`.
5. Audio command engine: `routine_0273..0289`.

Each batch should come with a narrow regression test or an existing replay smoke
before replacing numeric call sites.
