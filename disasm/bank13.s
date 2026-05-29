.include "lotw.inc"
; PRG bank 13 (swappable, runs at $A000) — file 0x1A010..0x1C010
; 2337 instructions, 4914/8192 code bytes, 274 labels
; PRG bank 13 — CPU origin $A000
.segment "CODE13"
    .byte $7A,$7B,$7C,$7D,$7E,$00,$00,$1F,$1F,$1F,$1F,$1F,$00,$00,$00,$7F
    .byte $80,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$81,$82,$83,$00,$00,$1F,$1F,$1F,$1F,$1F,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$1F,$1F,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$19
    .byte $04,$05,$07,$07,$00,$08,$09,$1A,$0A,$0B,$09,$08,$0C,$0D,$0A,$00
    .byte $00,$00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$0E,$0F,$10,$11,$12,$13,$09,$0B,$1B,$14
    .byte $0D,$15,$1C,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00,$00,$00
    .byte $00,$00,$19,$04,$05,$07,$06,$00,$10,$13,$16,$15,$0F,$17,$00,$00
    .byte $00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00,$00,$00
    .byte $00,$00,$00,$16,$14,$15,$0B,$0D,$0E,$0B,$0A,$00,$08,$18,$00,$00
    .byte $00,$00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$0D,$14,$0D
    .byte $11,$0B,$0D,$0A,$0F,$00,$0F,$10,$00,$13,$17,$0B,$09,$14,$15,$13
    .byte $00,$14,$0D,$15,$1C,$00,$00,$1F,$1F,$1F,$1F,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$00,$00
    .byte $00,$00,$1F,$1F,$1F,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F
    .byte $1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$1F,$55,$55,$55,$55,$55,$55,$55
    .byte $55,$55,$05,$05,$05,$05,$05,$05,$55,$55,$10,$50,$50,$50,$50,$00
    .byte $55,$55,$55,$55,$55,$55,$55,$55,$55,$55,$99,$AA,$AA,$AA,$AA,$65
    .byte $55,$55,$5A,$5A,$5A,$5A,$5A,$5A,$55,$55,$55,$55,$55,$55,$55,$55
    .byte $55,$05,$05,$05,$05,$05,$05,$05,$05,$0F,$29,$34,$11,$0F,$27,$16
    .byte $06,$0F,$20,$18,$08,$0F,$21,$11,$01,$0F,$00,$10,$30,$0F,$14,$14
    .byte $14,$0F,$22,$1A,$28,$0F,$16,$0F,$0F,$1C,$1E
    LDA #$18
    STA $8F
    LDA #$00
    STA $85
    JSR $C1D8
    LDX #$02
    LDA #$40
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    JSR $D08A
    JSR $C2B1
    LDX #$03
    LDA #$40
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    JSR L_A5B5
    LDA #$20
    STA $8F
    LDA #$3C
    STA $36
    JSR $C135
    LDA #$13
    STA map_screen_y
    LDA #$02
    STA map_screen_x
    LDA #$F2
    STA $0E
    LDA #$C8
    STA $0F
    JSR $CCE4
    JSR $C38B
    LDA #$EF
    STA $0200
    LDA #$22
    STA $1E
    LDA #$00
    STA scroll_x_fine
    STA player_x_fine
    LDA #$10
    STA scroll_x_tile
    LDA #$CB
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    LDX #$04
    LDA #$40
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    LDA #$00
    STA scroll_x_tile
    LDA #$6C
    STA $0E
    LDA #$C7
    STA $0F
    JSR $CCE4
    LDA #$3D
    STA mmc3_r3_shadow
L_A378:
    LDX $1E
    BNE L_A37E
    LDX #$F0
L_A37E:
    CPX #$C2
    BEQ L_A395
    DEX
    STX $1E
    TXA
    AND #$08
    LSR A
    LSR A
    LSR A
    STA $1D
    LDA #$FF
    JSR $CC8F
    JMP L_A378
L_A395:
    LDX #$02
    LDA #$40
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    LDA #$C7
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    LDA #$00
    STA $040C
    STA $040D
    STA $0406
    STA $E9
    STA scroll_x_fine
    STA scroll_x_tile
    LDA #$64
    STA $0405
    LDA #$08
    STA $3E
    LDA player_x_tile
    ASL A
    ASL A
    ASL A
    ASL A
    ORA player_x_fine
    STA player_x_fine
    JSR L_AD7A
    LDA #$EF
    STA $0210
    STA $0214
    JSR L_A7D2
    JSR L_A7F0
    RTS
    LDA #$00
    STA $E5
    LDA #$04
    STA $E6
    JSR $E98F
    LDA boss_life
    BNE L_A3F5
    JMP L_A7FF
L_A3F5:
    BIT nmi_scratch
    BVC L_A43C
    LDX $3E
    INX
    INX
    TXA
    AND #$06
oam_sprite_engine:
    BEQ L_A43C
    ASL A
    ASL A
    ASL A
    TAX
    LDA $0401,X
    BEQ L_A43C
    LDA #$00
    STA $0401,X
    LDA $1C
    CLC
    ADC $040C,X
    CMP #$B0
    BCC L_A437
    CMP #$D0
    BCS L_A437
    LDA boss_life
    SEC
    SBC #$02
    BCS L_A427
    LDA #$00
L_A427:
    STA boss_life
    JSR $CB69
    LDA #$20
    STA $8F
    LDA #$01
    STA $90
    JMP L_A43C
L_A437:
    LDA #$01
    STA a:$008F
L_A43C:
    LDA $FA
    BEQ L_A443
    JMP L_A56D
L_A443:
    LDX $F3
    BEQ L_A45F
    DEX
    BEQ L_A45C
    DEX
    BEQ L_A459
    DEX
    BEQ L_A456
    DEX
    BNE L_A45F
    JMP L_A549
L_A456:
    JMP L_A4F0
L_A459:
    JMP L_A4CF
L_A45C:
    JMP L_A4A6
L_A45F:
    JMP L_A462
L_A462:
    LDA $1C
    CLC
    ADC player_x_fine
    BCS L_A48C
    CMP #$C0
    BCS L_A48C
    LDX $1C
    CPX #$40
    BCS L_A48C
    CMP #$A0
    BCS L_A47B
    CMP #$80
    BCS L_A497
L_A47B:
    LDA $1E
    CMP #$C3
    BCS L_A48C
    LDA #$01
    STA $F3
    LDA #$04
    STA $E9
    JMP L_A4A6
L_A48C:
    LDA #$03
    STA $F3
    LDA #$02
    STA $E9
    JMP L_A4F0
L_A497:
    LDA #$02
    STA $F3
    LDA #$08
    STA $E9
    LDA #$B3
    STA $7A
    JMP L_A56D
L_A4A6:
    DEC $E9
    BEQ L_A4C8
    LDA $E9
    ASL A
    AND #$01
    CLC
    ADC #$A0
    ADC #$10
    STA $7A
    LDA $1C
    CLC
    ADC #$04
    STA $1C
    CMP #$40
    BCS L_A4C8
    LDA #$C2
    STA $1E
    JMP L_A56D
L_A4C8:
    LDA #$00
    STA $F3
    JMP L_A56D
L_A4CF:
    DEC $E9
    BEQ L_A4E5
    LDA #$B4
    STA $7A
    LDA $1E
    CMP #$C3
    BCC L_A4E2
    SEC
    SBC #$04
    STA $1E
L_A4E2:
    JMP L_A56D
L_A4E5:
    LDA #$B3
    STA $7A
    LDA #$00
    STA $F3
    JMP L_A56D
L_A4F0:
    DEC $E9
    BEQ L_A531
    LDA #$B2
    STA $7A
    LDA $1C
    BEQ L_A509
    SEC
    SBC #$04
    BCS L_A503
    LDA #$00
L_A503:
    STA $1C
    CMP #$11
    BCS L_A517
L_A509:
    LDA $1E
    CMP #$C3
    BCC L_A52E
    SEC
    SBC #$04
    STA $1E
    JMP L_A52E
L_A517:
    LDA $1E
    CMP #$D2
    BCC L_A529
    LDA $1C
    BEQ L_A52E
    SEC
    SBC #$04
    STA $1C
    JMP L_A52E
L_A529:
    CLC
    ADC #$04
    STA $1E
L_A52E:
    JMP L_A56D
L_A531:
    LDA $1C
    BEQ L_A53C
    LDA #$00
    STA $F3
    JMP L_A56D
L_A53C:
    LDA #$B0
    STA $7A
    INC $F3
    LDA #$04
    STA $E9
    JMP L_A56D
L_A549:
    DEC $E9
    BEQ L_A562
    LDA $E9
    CMP #$04
    BNE L_A557
    LDA #$20
    STA $8F
L_A557:
    LDA #$B5
    STA $7A
    LDA #$C2
    STA $1E
    JMP L_A56D
L_A562:
    LDA #$B3
    STA $7A
    LDA #$00
    STA $F3
    JMP L_A56D
L_A56D:
    JSR L_A574
    JSR $E99A
    RTS
L_A574:
    LDA $FA
    BNE L_A59D
    LDA #$0E
    STA vram_dst_lo
    LDA #$20
    STA vram_dst_hi
    LDA $1D
    EOR #$01
    ASL A
    ASL A
    ORA vram_dst_hi
    STA vram_dst_hi
    LDA $1D
    EOR #$01
    ASL A
    ASL A
    ASL A
    ASL A
    CLC
    ADC #$07
    ORA scroll_x_tile
    STA $F9
    LDA #$09
    STA $FA
L_A59D:
    LDA $F9
    STA $0C
    JSR $C833
    INC vram_dst_lo
    INC vram_dst_lo
    INC $F9
    DEC $FA
    BNE L_A5B4
    LDA $1D
    EOR #$01
    STA $1D
L_A5B4:
    RTS
L_A5B5:
    LDY #$04
L_A5B7:
    TYA
    PHA
    LDA #$05
    STA $36
    LDX #$0C
L_A5BF:
    LDA $0180,X
    AND #$0F
    STA $08
    LDA $0180,X
    AND #$F0
    SEC
    SBC #$10
    BCS L_A5D5
    LDA #$0F
    JMP L_A5D7
L_A5D5:
    ORA $08
L_A5D7:
    STA $0180,X
    DEX
    BPL L_A5BF
    JSR $C135
    PLA
    TAY
    DEY
    BNE L_A5B7
    RTS
    LDA #$01
    STA $E3
    LDA #$10
    STA $E5
    LDA #$04
    STA $E6
L_A5F2:
    LDY #$01
    LDA ($E5),Y
    BNE L_A606
    BIT $20
    BVC L_A609
    BIT $FD
    BVS L_A609
    JSR L_A622
    JMP L_A609
L_A606:
    JSR L_A657
L_A609:
    INC $E3
    CLC
    LDA #$10
    ADC $E5
    STA $E5
    LDA #$00
    ADC $E6
    STA $E6
    LDA $E3
    CMP #$04
    BCC L_A5F2
    JSR L_A6E0
    RTS
L_A622:
    JSR $E98F
    LDA $20
    AND #$40
    ORA $FD
    STA $FD
    LDA $FD
    LDY #$02
    JSR L_A7B1
    JSR L_A683
    JSR L_A6B1
    BCS L_A678
    LDA $0E
    STA $F9
    LDA $0A
    STA $FB
    LDA #$18
    STA $EE
    LDA #$00
    STA $EF
    LDA #$21
    STA $ED
    LDA #$19
    STA $8F
    JMP L_A678
L_A657:
    JSR $E98F
    DEC $EE
    BEQ L_A678
    JSR L_A6C5
    JSR L_A6B1
    BCS L_A669
    JMP L_A670
L_A669:
    LDA #$00
    STA $EE
    JMP L_A678
L_A670:
    LDA $0E
    STA $F9
    LDA $0A
    STA $FB
L_A678:
    LDA $EE
    BEQ L_A67F
    JSR L_A6A2
L_A67F:
    JSR $E99A
    RTS
L_A683:
    LDA player_x_fine
    STA $0E
    LDA player_y
    STA $0A
    LDA $F7
    BEQ L_A696
    ASL A
    ASL A
    CLC
    ADC $0A
    STA $0A
L_A696:
    LDA $F5
    BEQ L_A6A1
    ASL A
    ASL A
    CLC
    ADC $0E
    STA $0E
L_A6A1:
    RTS
L_A6A2:
    LDA $EE
    AND #$0C
    STA $08
    LDA $ED
    AND #$F3
    ORA $08
    STA $ED
    RTS
L_A6B1:
    LDA $0A
    CMP #$A1
    BCS L_A6C1
    LDA $0E
    CMP #$F1
    BCC L_A6C3
    LDA $0E
    BEQ L_A6C3
L_A6C1:
    SEC
    RTS
L_A6C3:
    CLC
    RTS
L_A6C5:
    LDA $F9
    STA $0E
    LDA $FB
    STA $0A
    LDA $F7
    BEQ L_A6D6
    CLC
    ADC $0A
    STA $0A
L_A6D6:
    LDA $F5
    BEQ L_A6DF
    CLC
    ADC $0E
    STA $0E
L_A6DF:
    RTS
L_A6E0:
    LDA #$88
    STA $0F
    LDA #$10
    STA $0E
    LDA #$03
L_A6EA:
    PHA
    JSR L_A703
    LDA $0F
    CLC
    ADC #$08
    STA $0F
    LDA $0E
    CLC
    ADC #$10
    STA $0E
    PLA
    SEC
    SBC #$01
    BNE L_A6EA
    RTS
L_A703:
    LDX $0F
    LDY $0E
    LDA $0401,Y
    BEQ L_A754
    LDA $040E,Y
    CMP #$BF
    BCS L_A754
    LDA $0402,Y
    STA $0202,X
    STA $0206,X
    AND #$40
    BNE L_A72F
    LDA sprite_tables,Y
    STA $0201,X
    CLC
    ADC #$02
    STA $0205,X
    JMP L_A73B
L_A72F:
    LDA sprite_tables,Y
    STA $0205,X
    CLC
    ADC #$02
    STA $0201,X
L_A73B:
    LDA $040C,Y
    STA $0203,X
    CLC
    ADC #$08
    STA $0207,X
    LDA $040E,Y
    CLC
    ADC #$2B
    STA $0200,X
    STA $0204,X
    RTS
L_A754:
    LDA #$EF
    STA $0200,X
    STA $0204,X
    RTS
L_A75D:
    DEC $3E
    BPL L_A765
    LDA #$07
    STA $3E
L_A765:
    LDA $3E
    AND #$06
    BEQ L_A78E
    LDA $3E
    ASL A
    ASL A
    TAX
    LDA $0280,X
    STA $0200
    LDA $0281,X
    STA $0201
    LDA $0282,X
    STA $0202
    LDA $0283,X
    STA $0203
    LDA #$EF
    STA $0280,X
    RTS
L_A78E:
    LDA $3E
    ASL A
    ASL A
    TAX
    LDA $0210,X
    STA $0200
    LDA $0211,X
    STA $0201
    LDA $0212,X
    STA $0202
    LDA $0213,X
    STA $0203
    LDA #$EF
    STA $0210,X
    RTS
L_A7B1:
    STY $09
    AND #$0F
    ASL A
    TAX
    LDA #$00
L_A7B9:
    CLC
    ADC $FE8B,X
    DEY
    BNE L_A7B9
    STA a:$00F5
    LDY $09
    LDA #$00
L_A7C7:
    CLC
    ADC $FE8C,X
    DEY
    BNE L_A7C7
    STA a:$00F7
    RTS
L_A7D2:
    LDX #$3F
L_A7D4:
    LDA $AAFC,X
    STA $0240,X
    DEX
    BPL L_A7D4
    JSR $CB69
    RTS
    LDX #$3F
L_A7E3:
    LDA $AB3C,X
    STA $0240,X
    DEX
    BPL L_A7E3
    JSR $CB53
    RTS
L_A7F0:
    LDX #$3F
L_A7F2:
    LDA $AB7C,X
    STA $02C0,X
    DEX
    BPL L_A7F2
    JSR $CB7F
    RTS
L_A7FF:
    JSR $CB69
    LDA #$00
    STA $0411
    STA $0421
    STA $0431
    STA $FA
    STA $85
    STA $88
    JSR L_AD7A
    JSR L_A6E0
    LDA #$EF
    STA $0200
L_A81E:
    LDA player_y
    CMP #$A0
    BCS L_A834
    INC player_y
    JSR L_AD7A
    LDA #$01
    STA $36
L_A82D:
    LDA $36
    BNE L_A82D
    JMP L_A81E
L_A834:
    LDA #$00
    STA $4E
    STA $4F
    JSR L_ACE0
    JSR L_AD7A
    LDA #$20
    STA scroll_x_tile
    LDA #$01
    STA $1D
    LDA #$20
    STA $8F
    LDA #$80
    STA $90
    LDA #$B6
    STA $7A
L_A854:
    JSR L_A574
    LDA $FA
    BNE L_A854
L_A85B:
    JSR L_A574
    LDA $FA
    BNE L_A85B
    LDA #$20
    STA $8F
    LDA #$80
    STA $90
    LDA #$B7
    STA $7A
L_A86E:
    JSR L_A574
    LDA $FA
    BNE L_A86E
L_A875:
    JSR L_A574
    LDA $FA
    BNE L_A875
    LDA #$00
    STA $10
L_A880:
    LDA $84
    AND #$07
    BNE L_A894
    LDA $1D
    EOR #$01
    STA $1D
    LDA #$20
    STA $8F
    LDA #$80
    STA $90
L_A894:
    LDA #$FF
    JSR $CC8F
    BIT nmi_scratch
    BVC L_A8A5
    LDA #$05
    JSR L_AE2F
    JSR $CB7F
L_A8A5:
    LDA $3E
    BNE L_A8AD
    LDA #$02
    STA $3E
L_A8AD:
    JSR L_AD7A
    JSR L_A75D
    DEC $10
    BNE L_A880
    LDA #$01
    STA $1D
    LDA #$FF
    JSR $CC8F
    LDA health
    BNE L_A8C5
    RTS
L_A8C5:
    LDA #$EF
    STA $0200
    LDA #$18
    STA $8F
    LDA #$FF
    STA $90
    LDA #$01
    STA $08
L_A8D6:
    LDA player_y
    SEC
    SBC $08
    STA player_y
    ADC #$2B
    CMP #$EF
    BCS L_A8F0
    JSR L_AD7A
    INC $08
    LDA #$FF
    JSR $CC8F
    JMP L_A8D6
L_A8F0:
    LDA #$EF
    STA $0210
    STA $0214
    LDA #$00
    STA $3E
    LDA #$80
    STA $3F
    JSR $D08A
    JSR L_B29B
    JSR $C461
    JSR $C38B
    JSR $C375
    LDA #$10
    STA map_screen_y
    LDA #$03
    STA map_screen_x
    LDA #$F2
    STA $0E
    LDA #$C8
    STA $0F
    JSR $CCE4
    LDA #$12
    STA scroll_x_tile
    LDA #$C0
    STA player_y
    LDA #$1A
    STA player_x_tile
    LDA #$01
    STA player_x_fine
    STA scroll_x_fine
    LDA #$09
    STA $56
    LDA #$35
    STA mmc3_r2_shadow
    LDA #$34
    STA mmc3_r3_shadow
    LDA #$36
    STA mmc3_r4_shadow
    LDA #$37
    STA mmc3_r5_shadow
    LDA #$01
    STA $0411
    STA $0421
    STA $0431
    STA $0441
    LDA #$A0
    STA $041E
    STA $042E
    STA $043E
    LDA #$70
    STA $044E
    LDA #$33
    STA $044D
    JSR L_AAAE
    CLC
    LDA #$2D
    STA $0410
    ADC #$20
    STA $0420
    ADC #$20
    STA $0430
    LDA #$81
    STA $0440
    LDA #$40
    STA $0412
    STA $0422
    STA $0432
    STA $0442
    JSR $C57A
    LDA #$CB
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    JSR $CAB6
    JSR $CACC
    JSR $CAF8
    JSR $CAE2
    JSR $C1C7
    JSR $D07C
    JSR $C1D8
    JSR $C234
    JSR $C2B1
    LDA #$07
    STA cur_character
    LDA #$92
    STA $0E
    LDA #$C4
    STA $0F
    JSR $CCE4
    LDA #$05
    STA $8C
L_A9CD:
    JSR L_AAEE
    LDA $8C
    BNE L_A9CD
L_A9D4:
    LDA player_y
    CMP #$A0
    BEQ L_A9FF
    DEC player_y
    JSR L_AAEE
    JSR L_AAEE
    LDA player_y
    CMP #$A0
    BEQ L_A9FF
    DEC player_y
    LDA $57
    EOR #$40
    STA $57
    JSR $C1D8
    JSR L_AAEE
    JSR L_AAEE
    JSR $C135
    JMP L_A9D4
L_A9FF:
    LDA #$0D
    STA $56
    JSR $C1D8
    LDA #$03
    STA $8C
L_AA0A:
    JSR L_AAEE
    LDA $8C
    BNE L_AA0A
L_AA11:
    LDA #$01
    STA $36
    LDA scroll_x_tile
    STA $7E
    LDA #$01
    STA $20
    LDA #$2B
    STA $0E
    LDA #$D4
    STA $0F
    JSR $CCE4
    LDA #$5D
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    JSR L_AAAE
    JSR $C1D8
    JSR $C2B1
    LDA $7E
    CMP scroll_x_tile
    BEQ L_AA44
    INC $3D
L_AA44:
    JSR $C135
    LDA player_x_tile
    CMP #$37
    BNE L_AA11
    LDA #$19
    STA $56
    LDA #$39
    STA $0410
    LDA #$59
    STA $0420
    LDA #$79
    STA $0430
    LDA #$91
    STA $0440
    LDA #$14
    STA $8C
L_AA69:
    LDA $56
    EOR #$04
    STA $56
    LDA $0410
    EOR #$04
    STA $0410
    LDA $0420
    EOR #$04
    STA $0420
    LDA $0430
    EOR #$04
    STA $0430
    LDA $0440
    EOR #$04
    STA $0440
    JSR L_AAEE
    JSR L_AAEE
    JSR L_AAEE
    JSR L_AAEE
    JSR L_AAEE
    JSR L_AAEE
    JSR L_AAEE
    JSR L_AAEE
    LDA $8C
    BNE L_AA69
    JMP L_B13D
L_AAAE:
    LDA $56
    AND #$1F
    STA $08
    LDA $0410
    AND #$E0
    ORA $08
    STA $0410
    LDA $0420
    AND #$E0
    ORA $08
    STA $0420
    LDA $0430
    AND #$E0
    ORA $08
    STA $0430
    LDA player_x_fine
    STA $041C
    STA $042C
    STA $043C
    LDX player_x_tile
    INX
    STX $042D
    DEX
    DEX
    DEX
    STX $043D
    DEX
    STX $041D
    RTS
L_AAEE:
    JSR $C1D8
    JSR $C2B1
    LDA #$01
    STA $36
    JSR $C135
    RTS
    .byte $58,$51,$03,$A0,$58,$53,$03,$A8,$58,$55,$03,$B0,$58,$57,$03,$B8
    .byte $58,$59,$03,$C0,$58,$5B,$03,$C8,$64,$61,$03,$A8,$64,$61,$03,$B2
    .byte $64,$61,$03,$BC,$64,$61,$03,$C6,$64,$61,$03,$D0,$74,$67,$03,$A8
    .byte $74,$67,$03,$B2,$74,$67,$03,$BC,$74,$67,$03,$C6,$74,$67,$03,$D0
    .byte $38,$9D,$03,$C0,$38,$9F,$03,$C8,$38,$B9,$03,$D0,$38,$BB,$03,$D8
    .byte $38,$BD,$03,$E0,$38,$BF,$03,$E8,$44,$A1,$03,$C8,$44,$A1,$03,$D2
    .byte $44,$A1,$03,$DC,$44,$A1,$03,$E6,$44,$A1,$03,$F0,$54,$A7,$03,$C8
    .byte $54,$A7,$03,$D2,$54,$A7,$03,$DC,$54,$A7,$03,$E6,$54,$A7,$03,$F0
    .byte $30,$6D,$03,$28,$30,$6F,$03,$30,$30,$71,$03,$38,$30,$73,$03,$40
    .byte $30,$75,$03,$48,$30,$77,$03,$50,$3C,$61,$03,$20,$3C,$61,$03,$2A
    .byte $3C,$61,$03,$34,$3C,$61,$03,$3E,$3C,$61,$03,$48,$4C,$67,$03,$20
    .byte $4C,$67,$03,$2A,$4C,$67,$03,$34,$4C,$67,$03,$3E,$4C,$67,$03,$48
    LDA $20
    AND #$10
    BEQ L_ABC5
    JMP L_AE11
L_ABC5:
    BIT $20
    BVS L_ABCF
    LDA $FD
    AND #$0F
    STA $FD
L_ABCF:
    LDA $20
    AND #$0F
    BEQ L_ABDF
    STA $08
    LDA $FD
    AND #$F0
    ORA $08
    STA $FD
L_ABDF:
    LDA $85
    BNE L_AC13
    BIT nmi_scratch
    BVC L_AC2A
    LDX $3E
    INX
    TXA
    AND #$06
    BNE L_AC2A
    LDA $1C
    CLC
    ADC $040C,X
    CMP #$B0
    LDA #$0A
    BCC L_ABFD
    LDA #$05
L_ABFD:
    JSR L_AE2F
    LDA #$0A
    STA $4F
    LDA #$21
    STA $8F
    LDA #$02
    STA $90
    LDA #$01
    STA $85
    JSR $CB7F
L_AC13:
    LDA $4F
    BNE L_AC22
    LDA $4E
    BNE L_AC22
    LDA #$00
    STA $85
    JMP L_AC2A
L_AC22:
    LDA $20
    AND #$F0
    ORA #$02
    STA $20
L_AC2A:
    JSR L_AE51
    LDA $4E
    BNE L_AC52
    LDA $4F
    BNE L_AC39
    LDA $20
    BPL L_AC41
L_AC39:
    JSR L_AC6D
    LDA #$00
    JMP L_AC45
L_AC41:
    LDA #$00
    STA $22
L_AC45:
    STA $4F
    JSR L_ADC7
    BCC L_AC4F
    JMP L_ACAF
L_AC4F:
    JMP L_ACA1
L_AC52:
    LSR A
    LSR A
    CLC
    ADC #$01
    STA $4B
    JSR L_ADC7
    BCS L_AC61
    JMP L_ACA1
L_AC61:
    LDA #$00
    STA $49
    JSR L_ADC7
    BCC L_ACA1
    JMP L_ACAF
L_AC6D:
    LDX $4F
    BNE L_AC7E
    LDA $22
    BEQ L_AC76
    RTS
L_AC76:
    LDA #$1B
    STA $8F
    LDA stat_jump
    STA $4F
L_AC7E:
    PLA
    PLA
    LDA #$01
    STA $22
    DEC $4F
    TXA
    LSR A
    LSR A
    EOR #$FF
    CLC
    ADC #$01
    STA $4B
    JSR L_ADC7
    BCC L_ACA1
    LDA #$00
    STA $49
    JSR L_ADC7
    BCC L_ACA1
    JMP L_ACAF
L_ACA1:
    LDA $0E
    STA player_x_fine
    LDA $0A
    STA player_y
    JSR L_ADE4
    JMP L_ACBB
L_ACAF:
    LDA #$00
    STA $4F
    STA $4E
    JSR L_ADE4
    JMP L_ACBB
L_ACBB:
    JSR L_ACE0
    JSR L_AD3B
    JSR L_AD7A
    RTS
L_ACC5:
    LDA player_x_fine
    STA $0E
    LDA player_y
    STA $0A
    LDA $4B
    BEQ L_ACD6
    CLC
    ADC $0A
    STA $0A
L_ACD6:
    LDA $49
    BEQ L_ACDF
    CLC
    ADC $0E
    STA $0E
L_ACDF:
    RTS
L_ACE0:
    LDX #$09
    LDA $20
    AND #$BF
    CMP #$80
    BEQ L_AD1F
    LDA $4B
    BEQ L_AD06
    BMI L_ACFF
    LDA $4E
    BNE L_AD22
    LDA $20
    AND #$04
    BEQ L_AD06
    LDX #$0D
    JMP L_AD1F
L_ACFF:
    LDA $4F
    BEQ L_AD1F
    JMP L_AD22
L_AD06:
    LDX #$01
    LDY #$00
    LDA $49
    BMI L_AD12
    BEQ L_AD21
    LDY #$40
L_AD12:
    STX $08
    LDA $56
    AND #$07
    ORA $08
    STA $56
    STY $57
    RTS
L_AD1F:
    STX $56
L_AD21:
    RTS
L_AD22:
    LDX #$39
    LDY #$00
    LDA $49
    BMI L_AD2E
    BEQ L_AD21
    LDY #$40
L_AD2E:
    STX $08
    LDA $56
    AND #$03
    ORA $08
    STA $56
    STY $57
    RTS
L_AD3B:
    LDA $56
    CMP #$20
    BCS L_AD50
    LDA $56
    BIT $20
    BVS L_AD4C
    AND #$EF
    JMP L_AD4E
L_AD4C:
    ORA #$10
L_AD4E:
    STA $56
L_AD50:
    LDA $20
    AND #$0F
    BEQ L_AD79
    LDA $4F
    ORA $4E
    BNE L_AD79
    INC $4D
    LDA $4D
    AND #$07
    BNE L_AD79
    LDA $56
    AND #$08
    BNE L_AD73
    LDA $56
    EOR #$04
    STA $56
    JMP L_AD79
L_AD73:
    LDA $57
    EOR #$40
    STA $57
L_AD79:
    RTS
L_AD7A:
    LDA $85
    BEQ L_AD8D
    LDA $84
    AND #$01
    BNE L_AD8D
    LDA #$EF
    STA $0210
    STA $0214
    RTS
L_AD8D:
    LDA player_y
    CLC
    ADC #$2B
    STA $0210
    STA $0214
    LDA player_x_fine
    STA $0213
    CLC
    ADC #$08
    STA $0217
    LDA $57
    ORA #$20
    STA $0212
    STA $0216
    BIT $57
    BVS L_ADBC
    LDX $56
    STX $0211
    INX
    INX
    STX $0215
    RTS
L_ADBC:
    LDX $56
    STX $0215
    INX
    INX
    STX $0211
    RTS
L_ADC7:
    LDA $4B
    PHA
L_ADCA:
    JSR L_ACC5
    JSR L_AE41
    BCC L_ADE0
    LDX $4B
    BEQ L_ADDF
    BMI L_ADDA
    DEX
    DEX
L_ADDA:
    INX
    STX $4B
    BNE L_ADCA
L_ADDF:
    SEC
L_ADE0:
    PLA
    STA $4B
    RTS
L_ADE4:
    LDA $4F
    BEQ L_ADEA
    CLC
    RTS
L_ADEA:
    LDA player_y
    CMP #$A0
    BCS L_ADF3
    INC $4E
    RTS
L_ADF3:
    LDA $4E
    CMP stat_jump
    BCC L_AE0C
    SEC
    SBC #$07
    CMP stat_jump
    BCC L_AE02
    LDA stat_jump
L_AE02:
    SEC
    SBC #$01
    STA $4F
    LDA #$0A
    STA a:$008F
L_AE0C:
    LDA #$00
    STA $4E
    RTS
L_AE11:
    LDA #$03
    STA $8F
    INC $8D
L_AE17:
    JSR $CC43
    BNE L_AE17
L_AE1C:
    JSR $CC43
    AND #$10
    BEQ L_AE1C
L_AE23:
    JSR $CC43
    BNE L_AE23
    LDA #$04
    STA $8F
    DEC $8D
    RTS
L_AE2F:
    STA $08
    LDA health
    SEC
    SBC $08
    STA health
    PHP
    BCS L_AE3F
    LDA #$00
    STA health
L_AE3F:
    PLP
    RTS
L_AE41:
    LDA $0A
    CMP #$A1
    BCS L_AE4D
    LDA $0E
    CMP #$F1
    BCC L_AE4F
L_AE4D:
    SEC
    RTS
L_AE4F:
    CLC
    RTS
L_AE51:
    LDA $20
    AND #$0F
    ASL A
    TAX
    LDA $FE8B,X
    STA a:$0049
    LDA $FE8C,X
    STA a:$004B
    RTS
L_AE64:
    JSR L_B631
    LDA #$37
    STA mmc3_r2_shadow
    LDA #$00
    STA $29
    LDA #$A0
    STA ppuctrl_shadow
    STA PPUCTRL
    LDA #$00
    STA $24
    STA PPUMASK
    LDA #$00
    STA $1C
    STA $1D
    LDA #$E8
    STA $1E
    LDA #$0F
    LDX #$1F
L_AE8B:
    STA $0180,X
    DEX
    BPL L_AE8B
    LDA #$69
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    JSR $D08A
    JSR $C375
    JSR L_B102
    LDA #$15
    STA mmc3_r2_shadow
    LDA #$09
    STA $8E
    JSR $FC08
    JSR L_B648
    LDA #$1E
    STA $24
    STA PPUMASK
    LDA #$78
    STA $36
L_AEBE:
    LDA $36
    BNE L_AEBE
    JSR L_B6A6
    LDA #$14
    STA $8C
L_AEC9:
    LDA #$01
    STA $36
    JSR $CC43
    CMP #$FF
    BNE L_AEDA
    LDA #$1A
    STA $8F
    STA $37
L_AEDA:
    AND #$10
    BNE L_AF17
    LDA $21
    CMP #$83
    BEQ L_AF1A
    LDA $84
    AND #$07
    BNE L_AF05
    LDA $0182
    AND #$0F
    STA $08
    LDA $0182
    AND #$F0
    SEC
    SBC #$10
    BCS L_AEFD
    LDA #$30
L_AEFD:
    STA $0193
    ORA $08
    STA $0182
L_AF05:
    LDA #$35
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    LDA $8C
    BNE L_AEC9
    JMP L_AF1D
L_AF17:
    JMP L_B0B1
L_AF1A:
    JMP L_B13D
L_AF1D:
    JSR $C461
    JSR $C375
    JSR $D08A
    JSR L_B10E
    LDA #$04
    JSR $CC64
    STA map_screen_x
    LDA #$10
    JSR $CC64
    STA map_screen_y
    LDA #$F2
    STA $0E
    LDA #$C8
    STA $0F
    JSR $CCE4
L_AF42:
    LDA #$40
    JSR $CC64
    STA player_x_tile
    STA $0C
    LDA #$00
    STA player_x_fine
    LDA #$0B
    JSR $CC64
    ASL A
    ASL A
    ASL A
    ASL A
    STA player_y
    STA $0D
    JSR $CA54
    LDY #$00
    LDA ($0C),Y
    AND #$3F
    CMP #$30
    BCS L_AF42
    CMP #$02
    BEQ L_AF42
    CMP $70
    BEQ L_AF42
    INY
    LDA ($0C),Y
    AND #$3F
    CMP #$30
    BCC L_AF42
    BEQ L_AF42
    LDA player_x_tile
    SEC
    SBC #$08
    BCS L_AF85
    LDA #$00
L_AF85:
    CMP #$30
    BCC L_AF8B
    LDA #$30
L_AF8B:
    STA scroll_x_tile
    LDA #$00
    STA scroll_x_fine
L_AF91:
    LDA #$05
    JSR $CC64
    TAX
    TAY
    SEC
    LDA #$00
L_AF9B:
    ROL A
    DEY
    BPL L_AF9B
    AND $41
    BEQ L_AF91
    LDA $B0AC,X
    STA carried_item0
    LDA #$00
    STA equipped_item
    STX cur_character
    TXA
    ASL A
    ASL A
    CLC
    ADC #$03
    TAY
    LDX #$03
L_AFB7:
    LDA $FFA7,Y
    STA stat_jump,X
    DEY
    DEX
    BPL L_AFB7
    LDA cur_character
    CLC
    ADC #$38
    STA mmc3_r2_shadow
    LDA #$3E
    STA mmc3_r4_shadow
    LDA #$20
    STA mmc3_r5_shadow
    LDA #$0D
    STA $56
    LDA #$00
    STA $57
    LDA #$01
    STA $42
    LDA #$64
    STA health
    STA magic
    LDA #$8B
    STA $0E
    LDA #$C3
    STA $0F
    JSR $CCE4
    JSR $C57A
    LDA #$CB
    STA $0E
    LDA #$C5
    STA $0F
    JSR $CCE4
    JSR $CAB6
    JSR $CACC
    JSR $CAF8
    JSR $CAE2
    JSR $C1C7
    JSR $D07C
    JSR $C1D8
    JSR $C234
    LDA #$92
    STA $0E
    LDA #$C4
    STA $0F
    JSR $CCE4
    LDA #$0A
    STA $8C
L_B021:
    LDA #$01
    STA $36
    LDA scroll_x_tile
    STA $7E
    JSR L_B11A
    JSR $CC43
    AND #$10
    BEQ L_B036
    JMP L_B0B1
L_B036:
    LDA $FE
    STA $20
    LDA $49
    ORA $4B
    BEQ L_B044
    DEC $42
    BNE L_B04F
L_B044:
    LDA #$80
    STA $42
    JSR L_B0E4
    LDA $20
    STA $FE
L_B04F:
    LDA #$2B
    STA $0E
    LDA #$D4
    STA $0F
    JSR $CCE4
    LDA #$28
    STA $0E
    LDA #$F6
    STA $0F
    JSR $CCE4
    LDA #$7C
    STA $0E
    LDA #$E8
    STA $0F
    JSR $CCE4
    LDA #$82
    STA $0E
    LDA #$F7
    STA $0F
    JSR $CCE4
    LDA #$5D
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    JSR $C1D8
    JSR $C2B1
    LDA $7E
    CMP scroll_x_tile
    BEQ L_B094
    INC $3D
L_B094:
    LDA #$35
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    LDA $8C
    BEQ L_B0A6
    JMP L_B021
L_B0A6:
    JSR $C461
    JMP L_AE64
    .byte $03,$04,$05,$02,$08
L_B0B1:
    JSR $C461
    LDA #$8B
    STA $0E
    LDA #$C3
    STA $0F
    JSR $CCE4
    JSR $C57A
    JSR $C375
    JSR L_B631
    JSR $CAB6
    JSR $CAF8
    JSR $CAE2
    JSR $CAF8
    LDA #$01
    STA $36
    LDA #$35
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    RTS
L_B0E4:
    LDA #$04
    JSR $CC64
    TAX
    LDA $B0FE,X
    STA $20
    LDA #$0A
    JSR $CC64
    TAX
    BNE L_B0FD
    LDA $20
    ORA #$40
    STA $20
L_B0FD:
    RTS
    .byte $81,$84,$82,$00
L_B102:
    LDX #$7F
L_B104:
    LDA $B71C,X
    STA $0240,X
    DEX
    BPL L_B104
    RTS
L_B10E:
    LDX #$1F
L_B110:
    LDA $B6FC,X
    STA $0240,X
    DEX
    BPL L_B110
    RTS
L_B11A:
    LDX #$EF
    LDA $84
    AND #$30
    BEQ L_B124
    LDX #$80
L_B124:
    STX $0240
    STX $0244
    STX $0248
    STX $024C
    STX $0250
    STX $0254
    STX $0258
    STX $025C
    RTS
L_B13D:
    INC $92
    JSR L_B29B
    JSR $C461
    JSR $C38B
    JSR L_B2EE
    LDA #$20
    STA mmc3_r0_shadow
    LDA #$22
    STA mmc3_r1_shadow
    LDA $24
    ORA #$18
    STA $24
    LDA #$FF
    JSR $CC8F
    LDA #$0A
    STA $8E
    JSR $FC08
    LDA #$00
    STA a:$001C
    STA a:$001D
    STA a:$000A
    STA scroll_x_fine
    STA scroll_x_tile
    JSR L_B2CC
    LDA #$40
    STA vram_src_lo
    LDA #$01
    STA vram_src_hi
    LDA #$20
    STA vram_len
    LDA #$9C
    STA $0C
    LDA #$B7
    STA $0D
L_B18B:
    JSR L_B25D
    JSR L_B1EA
    BCS L_B19B
    JSR L_B25D
    JSR L_B215
    BCC L_B18B
L_B19B:
    LDA #$20
    STA $8F
L_B19F:
    LDA $D4
    BEQ L_B19F
L_B1A3:
    LDA $D4
    BNE L_B1A3
    LDA #$3C
    STA $36
L_B1AB:
    LDA $36
    BNE L_B1AB
    LDA #$00
    STA $94
    STA $A4
    STA $B4
    STA $C4
    LDA #$18
    STA $8F
    LDX #$0A
L_B1BF:
    TXA
    PHA
    LDA #$30
    LDX #$1F
L_B1C5:
    STA $0180,X
    DEX
    BPL L_B1C5
    JSR $C569
    LDA #$01
    STA $36
    JSR $C135
    JSR L_B2CC
    JSR $C569
    LDA #$02
    STA $36
    JSR $C135
    PLA
    TAX
    DEX
    BNE L_B1BF
L_B1E7:
    JMP L_B1E7
L_B1EA:
    JSR L_B2FC
    LDY #$00
L_B1EF:
    LDA ($0C),Y
    BEQ L_B213
    CMP #$0D
    BEQ L_B209
    AND #$0F
    STA $08
    LDA ($0C),Y
    AND #$F0
    ASL A
    ORA $08
    STA $0140,Y
    INY
    JMP L_B1EF
L_B209:
    JSR L_B24E
    LDA #$05
    JSR L_B278
    CLC
    RTS
L_B213:
    SEC
    RTS
L_B215:
    JSR L_B2FC
    LDY #$00
L_B21A:
    LDA ($0C),Y
    BEQ L_B24C
    CMP #$0D
    BEQ L_B237
    AND #$0F
    STA $08
    LDA ($0C),Y
    AND #$F0
    ASL A
    ORA $08
    CLC
    ADC #$10
    STA $0140,Y
    INY
    JMP L_B21A
L_B237:
    INY
    TYA
    CLC
    ADC $0C
    STA $0C
    BCC L_B242
    INC $0D
L_B242:
    JSR L_B24E
    LDA #$05
    JSR L_B278
    CLC
    RTS
L_B24C:
    SEC
    RTS
L_B24E:
    LDA #$08
    STA vram_dst_hi
    LDA $0A
    ASL A
    ROL vram_dst_hi
    ASL A
    ROL vram_dst_hi
    STA vram_dst_lo
    RTS
L_B25D:
    INC $0A
    LDA $0A
    AND #$07
    BEQ L_B26D
    LDA #$FF
    JSR L_B278
    JMP L_B25D
L_B26D:
    LDA $0A
    CMP #$F0
    BNE L_B277
    LDA #$00
    STA $0A
L_B277:
    RTS
L_B278:
    PHA
    LDA $0A
    CLC
    ADC #$06
    CMP #$F0
    BCC L_B285
    CLC
    ADC #$10
L_B285:
    STA $1E
    PLA
    JSR $CC8F
    LDA #$FF
    JSR $CC8F
    LDA #$FF
    JSR $CC8F
    LDA #$FF
    JSR $CC8F
    RTS
L_B29B:
    LDA #$00
    STA $B4
    LDA #$10
    STA $0D
L_B2A3:
    LDA $A0
    BEQ L_B2A9
    DEC $A0
L_B2A9:
    LDA $B0
    BEQ L_B2AF
    DEC $B0
L_B2AF:
    LDA $D0
    BEQ L_B2B5
    DEC $D0
L_B2B5:
    LDA #$14
    STA $0C
L_B2B9:
    JSR $C2B1
    LDA #$01
    STA $36
    JSR $C135
    DEC $0C
    BNE L_B2B9
    DEC $0D
    BNE L_B2A3
    RTS
L_B2CC:
    LDA #$0F
    STA $0180
    LDA #$0C
    STA $0181
    LDA #$10
    STA $0182
    LDA #$30
    STA $0183
    LDA #$0F
    LDX #$1B
L_B2E4:
    STA $0184,X
    DEX
    BPL L_B2E4
    JSR $C569
    RTS
L_B2EE:
    LDX #$00
    LDA #$EF
L_B2F2:
    STA $0200,X
    INX
    INX
    INX
    INX
    BNE L_B2F2
    RTS
L_B2FC:
    LDY #$1F
    LDA #$C0
L_B300:
    STA $0140,Y
    DEY
    BPL L_B300
    RTS
    LDA $8E
    PHA
    INC $8D
    JSR $D07C
    LDX #$35
    LDY #$00
    JSR L_B4C5
    LDA #$3C
    STA $36
    JSR $C135
    LDA #$08
    JSR $D02E
    DEC $8D
    LDA #$05
    STA $0A
L_B328:
    LDX #$0D
    LDY #$00
    JSR L_B4C5
    LDX #$01
    LDY #$00
    JSR L_B4C5
    LDX #$09
    LDY #$00
    JSR L_B4C5
    LDX #$01
    LDY #$40
    JSR L_B4C5
    DEC $0A
    BNE L_B328
    LDA #$01
    STA $36
    LDA #$31
    STA $56
    JSR $C1D8
    JSR $C135
    LDA $EC
    BNE L_B383
    LDA $37
    BPL L_B363
    INC $37
    JMP L_B372
L_B363:
    LDX equipped_item
    LDA carried_item0,X
    CMP #$0C
    BNE L_B383
    LDA #$FF
    STA carried_item0,X
    JSR $C234
L_B372:
    JSR $D16A
    LDA #$19
    STA $56
    JSR $CC09
    PLA
    JSR $D02E
    LDX #$00
    RTS
L_B383:
    PLA
    JSR $C461
    LDA #$00
    STA $EC
    STA $3E
    LDA #$80
    STA $3F
    JSR $C38B
    JSR $D08A
    JSR $C2B1
    LDA #$16
    STA mmc3_r1_shadow
    LDA #$36
    STA mmc3_r2_shadow
    LDA #$00
    STA a:$001C
    STA a:$001D
    STA a:$001E
    STA scroll_x_fine
    STA scroll_x_tile
    LDA #$6B
    STA vram_dst_lo
    LDA #$21
    STA vram_dst_hi
    LDA #$AF
    STA vram_src_lo
    LDA #$B4
    STA vram_src_hi
    LDA #$09
    STA vram_len
    LDA #$05
    JSR $CC8F
    LDA #$4C
    STA vram_dst_lo
    LDA #$22
    STA vram_dst_hi
    LDA #$B8
    STA vram_src_lo
    LDA #$B4
    STA vram_src_hi
    LDA #$05
    STA vram_len
    LDA #$05
    JSR $CC8F
    LDA #$8C
    STA vram_dst_lo
    LDA #$22
    STA vram_dst_hi
    LDA #$BD
    STA vram_src_lo
    LDA #$B4
    STA vram_src_hi
    LDA #$08
    STA vram_len
    LDA #$05
    JSR $CC8F
    LDA #$05
    STA player_x_tile
    LDA #$00
    STA player_x_fine
    LDA #$70
    STA player_y
    LDA #$39
    STA $56
    JSR $C375
    JSR $C1D8
    LDA #$E0
    STA $0E
    LDA #$C4
    STA $0F
    JSR $CCE4
L_B41D:
    JSR $CC09
    AND #$10
    BNE L_B431
    LDA player_y
    EOR #$10
    STA player_y
    LDA #$0C
    STA $8F
    JMP L_B41D
L_B431:
    LDA #$18
    STA $8F
    LDA player_y
    CMP #$70
    BEQ L_B450
    JSR $C461
    LDA #$78
    STA $36
    LDA #$35
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    LDX #$02
    RTS
L_B450:
    JSR $D0C5
    LDA #$FF
    STA carried_item0
    STA carried_item1
    STA carried_item2
    LDA #$03
    STA equipped_item
    LDA #$06
    STA cur_character
    LDA #$03
    STA map_screen_x
    LDA #$10
    STA map_screen_y
    JSR $C461
    LDA #$02
    STA $8E
    JSR $C38B
    JSR $C57A
    JSR $CAB6
    JSR $CACC
    JSR $CAE2
    JSR $CAF8
    LDA #$F2
    STA $0E
    LDA #$C8
    STA $0F
    JSR $CCE4
    LDA #$0F
    LDX #$1F
L_B493:
    STA $0180,X
    DEX
    BPL L_B493
    LDA #$EF
    STA $0210
    STA $0214
    LDA #$B4
    STA $0E
    LDA #$C4
    STA $0F
    JSR $CCE4
    LDX #$01
    RTS
    .byte $E7,$E1,$ED,$E5,$C0,$EF,$F6,$E5,$F2,$F2,$E5,$F4,$F2,$F9,$E3,$EF
    .byte $EE,$F4,$E9,$EE,$F5,$E5
L_B4C5:
    STX $56
    STY $57
    LDA #$08
    STA $36
    JSR $C1D8
    JSR $C135
    RTS
    LDX #$0F
    LDY #$07
L_B4D8:
    LDA $0308,Y
    LSR A
    LSR A
    LSR A
    LSR A
    STA $0322,X
    DEX
    LDA $0308,Y
    AND #$0F
    STA $0322,X
    DEX
    DEY
    BPL L_B4D8
    LDX #$0F
L_B4F1:
    LDA save_inventory_counts,X
    AND #$0F
    STA $0332,X
    DEX
    BPL L_B4F1
    LDA save_keys
    LDX #$0F
L_B501:
    LSR A
    ROL $0322,X
    DEX
    DEX
    BPL L_B501
    LDA save_gold
    LDX #$0F
L_B50E:
    LSR A
    ROL $0332,X
    DEX
    DEX
    BPL L_B50E
    LDA #$00
    LDX #$1F
L_B51A:
    CLC
    ADC $0322,X
    DEX
    BPL L_B51A
    STA $0389
    LDA #$0A
    LDX #$1F
L_B528:
    EOR $0322,X
    DEX
    BPL L_B528
    STA $038A
    LDA $0389
    LDX #$0E
L_B536:
    LSR A
    ROL $0322,X
    DEX
    DEX
    BPL L_B536
    LDA $038A
    LDX #$0E
L_B543:
    LSR A
    ROL $0332,X
    DEX
    DEX
    BPL L_B543
    LDA $0331
    STA rng_s1
    LDA $0341
    STA rng_s2
    LDX #$0E
L_B557:
    STX $08
    LDA #$20
    JSR $CC64
    LDX $08
    EOR $0322,X
    STA $0322,X
    LDA #$20
    JSR $CC64
    LDX $08
    EOR $0332,X
    STA $0332,X
    DEX
    BPL L_B557
    RTS
    LDX #$1F
L_B579:
    LDA $0322,X
    STA $0342,X
    DEX
    BPL L_B579
    LDA $0351
    STA rng_s1
    LDA $0361
    STA rng_s2
    LDX #$0E
L_B58E:
    STX $08
    LDA #$20
    JSR $CC64
    LDX $08
    EOR $0342,X
    STA $0342,X
    LDA #$20
    JSR $CC64
    LDX $08
    EOR $0352,X
    STA $0352,X
    DEX
    BPL L_B58E
    LDX #$0E
L_B5AF:
    LSR $0352,X
    ROR A
    DEX
    DEX
    BPL L_B5AF
    STA $038A
    LDX #$0E
L_B5BC:
    LSR $0342,X
    ROR A
    DEX
    DEX
    BPL L_B5BC
    STA $0389
    LDA #$00
    LDX #$1F
L_B5CB:
    CLC
    ADC $0342,X
    DEX
    BPL L_B5CB
    CMP $0389
    BEQ L_B5DA
    JMP L_B629
L_B5DA:
    LDA #$0A
    LDX #$1F
L_B5DE:
    EOR $0342,X
    DEX
    BPL L_B5DE
    CMP $038A
    BEQ L_B5EC
    JMP L_B629
L_B5EC:
    LDX #$0F
L_B5EE:
    LSR $0342,X
    ROR A
    DEX
    DEX
    BPL L_B5EE
    STA save_keys
    LDX #$0F
L_B5FB:
    LSR $0352,X
    ROR A
    DEX
    DEX
    BPL L_B5FB
    STA save_gold
    LDX #$0F
    LDY #$07
L_B60A:
    LDA $0342,X
    ASL A
    ASL A
    ASL A
    ASL A
    DEX
    ORA $0342,X
    DEX
    STA $0308,Y
    DEY
    BPL L_B60A
    LDX #$0F
L_B61E:
    LDA $0352,X
    STA save_inventory_counts,X
    DEX
    BPL L_B61E
    CLC
    RTS
L_B629:
    LDA #$1C
    STA $8F
    STA $90
    SEC
    RTS
L_B631:
    LDX #$40
L_B633:
    LDA $9B9F,X
    STA $00,X
    INX
    CPX #$8C
    BNE L_B633
    LDA #$0F
    LDX #$1F
L_B641:
    STA $0180,X
    DEX
    BPL L_B641
    RTS
L_B648:
    LDA ppuctrl_shadow
    PHA
    AND #$7B
    STA PPUCTRL
    LDA #$00
    STA $29
    LDA $24
    PHA
    AND #$E7
    STA PPUMASK
    LDA #$20
    STA PPUADDR
    LDA #$00
    STA PPUADDR
    LDX #$00
L_B668:
    LDA $9EC9,X
    STA PPUDATA
    INX
    BNE L_B668
    LDX #$00
L_B673:
    LDA $9FC9,X
    STA PPUDATA
    INX
    BNE L_B673
    LDX #$00
L_B67E:
    LDA $A0C9,X
    STA PPUDATA
    INX
    BNE L_B67E
    LDX #$00
L_B689:
    LDA $A1C9,X
    STA PPUDATA
    INX
    BNE L_B689
    LDA $A2E9
    STA mmc3_r0_shadow
    LDA $A2EA
    STA mmc3_r1_shadow
    PLA
    STA $24
    PLA
    STA ppuctrl_shadow
    STA PPUCTRL
    RTS
L_B6A6:
    LDA #$40
    STA $09
L_B6AA:
    LDA #$05
    STA $36
    JSR L_B6F0
    LDX #$00
    LDY #$20
    JSR L_B6D0
    LDA #$35
    STA $0E
    LDA #$C1
    STA $0F
    JSR $CCE4
    LDA $09
    SEC
    SBC #$10
    STA $09
    BPL L_B6AA
    JSR $C569
    RTS
L_B6D0:
    LDA $0180,X
    AND #$0F
    STA $08
    LDA $0180,X
    AND #$F0
    SEC
    SBC $09
    BCS L_B6E6
    LDA #$0F
    JMP L_B6E8
L_B6E6:
    ORA $08
L_B6E8:
    STA $0180,X
    INX
    DEY
    BNE L_B6D0
    RTS
L_B6F0:
    LDX #$1F
L_B6F2:
    LDA $A2C9,X
    STA $0180,X
    DEX
    BPL L_B6F2
    RTS
    .byte $80,$C1,$00,$60,$80,$C3,$00,$68,$80,$C5,$00,$70,$80,$C7,$00,$78
    .byte $80,$C9,$00,$80,$80,$CB,$00,$88,$80,$CD,$00,$90,$80,$CF,$00,$98
    .byte $6E,$01,$00,$60,$6E,$03,$00,$68,$6E,$05,$00,$70,$6E,$07,$00,$78
    .byte $6E,$09,$00,$80,$6E,$0B,$00,$88,$6E,$0D,$00,$90,$6E,$0F,$00,$98
    .byte $F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8
    .byte $F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8
    .byte $F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8
    .byte $F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8
    .byte $F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8
    .byte $F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8,$F8
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$43,$52,$45,$44
    .byte $49,$54,$53,$0D,$0D,$0D,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$43,$41,$53,$54,$0D,$0D,$0D,$20,$20,$20,$20
    .byte $20,$20,$57,$61,$72,$72,$69,$6F,$72,$20,$20,$58,$65,$6D,$6E,$20
    .byte $57,$6F,$72,$7A,$65,$6E,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$57
    .byte $69,$7A,$61,$72,$64,$20,$20,$4D,$61,$79,$6E,$61,$20,$57,$6F,$72
    .byte $7A,$65,$6E,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$52,$61,$6E,$67
    .byte $65,$72,$20,$20,$52,$6F,$61,$73,$20,$57,$6F,$72,$7A,$65,$6E,$0D
    .byte $0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$45,$6C,$66,$20,$20
    .byte $4C,$79,$6C,$6C,$20,$57,$6F,$72,$7A,$65,$6E,$0D,$0D,$20,$20,$20
    .byte $20,$20,$20,$4D,$6F,$6E,$73,$74,$65,$72,$20,$20,$50,$6F,$63,$68
    .byte $69,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$4D,$6F,$6E,$73,$74,$65,$72,$73,$0D,$0D
    .byte $0D,$20,$20,$20,$20,$20,$20,$4B,$69,$6E,$67,$20,$44,$72,$61,$67
    .byte $6F,$6E,$20,$20,$4B,$65,$65,$6C,$61,$20,$20,$0D,$0D,$0D,$0D,$0D
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$54,$61,$72,$61,$74
    .byte $75,$6E,$65,$73,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $41,$72,$63,$68,$77,$69,$6E,$67,$65,$72,$0D,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$45,$72,$65,$62,$6F,$6E,$65,$0D,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$52,$6F,$63,$6B,$67,$61,$65
    .byte $61,$0D,$0D,$0D,$0D,$0D,$20,$20,$20,$20,$52,$6F,$63,$6B,$20,$56
    .byte $65,$65,$73,$74,$20,$20,$20,$20,$20,$20,$20,$20,$20,$4D,$75,$0D
    .byte $20,$20,$20,$20,$20,$4D,$6F,$72,$69,$63,$64,$6F,$20,$20,$20,$20
    .byte $20,$20,$20,$52,$6F,$69,$64,$20,$4D,$6F,$6F,$6E,$0D,$20,$20,$20
    .byte $20,$20,$20,$47,$61,$72,$62,$61,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $4B,$69,$6C,$6C,$65,$72,$20,$42,$61,$74,$0D,$20,$20,$20,$20,$20
    .byte $4B,$72,$61,$75,$67,$65,$6E,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$4B,$69,$6D,$75,$0D,$20,$20,$20,$20,$20,$20,$47,$72,$69,$64
    .byte $65,$6C,$20,$20,$20,$20,$20,$20,$20,$20,$43,$72,$61,$77,$6C,$65
    .byte $72,$0D,$20,$20,$20,$20,$53,$6E,$61,$6B,$65,$20,$4B,$69,$64,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$41,$72,$79,$75,$0D,$20,$20,$20
    .byte $59,$61,$73,$68,$69,$6E,$6F,$74,$6B,$69,$6E,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$47,$65,$72,$73,$0D,$20,$44,$65,$72,$75,$64,$65,$61
    .byte $74,$68,$67,$61,$64,$65,$64,$6F,$20,$20,$20,$20,$53,$6B,$65,$6C
    .byte $65,$74,$6F,$6E,$0D,$20,$20,$20,$20,$20,$20,$20,$53,$6C,$75,$67
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$54,$69,$67,$65,$72,$0D
    .byte $20,$20,$20,$20,$20,$43,$79,$63,$6C,$6F,$70,$73,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$4D,$75,$6D,$6D,$79,$0D,$20,$20,$20,$20,$4C
    .byte $69,$7A,$61,$72,$64,$20,$4D,$61,$6E,$20,$20,$20,$20,$20,$20,$20
    .byte $44,$77,$61,$72,$66,$0D,$20,$20,$20,$20,$20,$20,$47,$69,$61,$6E
    .byte $74,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$4F,$72,$63,$0D
    .byte $20,$20,$20,$20,$45,$6C,$65,$6D,$65,$6E,$74,$61,$6C,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$57,$72,$69,$74,$68,$0D,$20,$20,$20,$20,$20
    .byte $45,$67,$67,$2D,$6D,$61,$6E,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $4D,$69,$6D,$69,$63,$0D,$20,$20,$20,$20,$20,$20,$20,$52,$6F,$63
    .byte $6B,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$53,$6C,$69,$6D,$65
    .byte $0D,$20,$20,$20,$20,$6C,$69,$67,$68,$74,$62,$61,$6C,$6C,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$50,$72,$61,$6E,$64,$69,$0D,$20,$20,$20
    .byte $20,$20,$20,$4D,$65,$6D,$65,$73,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$47,$6F,$6C,$65,$6D,$0D,$20,$20,$20,$20,$20,$20,$4D,$6F
    .byte $6E,$63,$68,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$57,$69,$7A
    .byte $61,$72,$64,$0D,$20,$20,$20,$20,$20,$46,$72,$6F,$67,$2D,$6D,$61
    .byte $6E,$20,$20,$20,$20,$20,$20,$20,$20,$20,$4D,$61,$79,$75,$0D,$20
    .byte $20,$20,$20,$20,$44,$61,$72,$75,$2D,$64,$6F,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$4B,$69,$72,$72,$75,$0D,$20,$20,$20,$20,$20,$42
    .byte $75,$70,$75,$72,$63,$68,$20,$20,$20,$20,$20,$20,$20,$20,$20,$44
    .byte $6F,$72,$61,$6B,$0D,$20,$20,$20,$20,$20,$20,$20,$4C,$69,$6F,$6E
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$46,$6C,$61,$69,$6C,$20,$53,$6E
    .byte $61,$69,$6C,$0D,$20,$20,$20,$20,$20,$20,$52,$6F,$6D,$61,$6E,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$4D,$65,$74,$61,$20,$42,$6C,$61,$63
    .byte $6B,$0D,$20,$20,$20,$20,$20,$20,$20,$45,$64,$6F,$0D,$0D,$0D,$0D
    .byte $0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$53,$54,$41,$46,$46,$0D,$0D,$0D
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$53,$63,$65,$6E,$61,$72,$69,$6F
    .byte $20,$53,$74,$61,$66,$66,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$48,$61,$74,$61,$62,$6F,$77,$0D,$20,$20,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$4F,$6E,$79,$61,$6E,$6B,$6F,$0D
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$47,$61,$6E,$63
    .byte $68,$61,$6E,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $44,$72,$2E,$20,$4B,$65,$79,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$50,$72,$6F,$67,$72,$61,$6D
    .byte $6D,$69,$6E,$67,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$44,$72,$2E,$20,$4B,$65,$79,$0D,$0D,$0D,$0D,$0D,$0D,$0D
    .byte $0D,$20,$20,$20,$20,$20,$50,$72,$6F,$67,$72,$61,$6D,$6D,$69,$6E
    .byte $67,$20,$61,$73,$73,$69,$73,$74,$61,$6E,$63,$65,$0D,$0D,$20,$20
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$48,$61,$74,$61,$62,$6F
    .byte $77,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$4F,$6E
    .byte $79,$61,$6E,$6B,$6F,$0D,$0D,$0D,$0D,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$41,$72,$74,$20,$26,$20,$47,$72,$61,$70,$68,$69,$63,$0D
    .byte $0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$47,$61,$6E
    .byte $63,$68,$61,$6E,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$4B,$61,$69,$6A,$69,$6E,$0D,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$4E,$6F,$77,$74,$65,$6E,$20,$4D,$75,$73,$75,$6D,$65,$0D,$0D
    .byte $0D,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $4D,$75,$73,$69,$63,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$20,$20,$4B,$6F,$73,$68,$69,$72,$6F,$6E,$0D,$0D,$0D,$0D,$0D
    .byte $20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$50,$72,$6F,$64
    .byte $75,$63,$65,$0D,$0D,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20,$20
    .byte $20,$53,$68,$61,$63,$68,$6F,$77,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$0D
    .byte $0D,$0D,$0D,$0D,$0D,$0D,$0D,$20,$20,$20,$20,$4C,$65,$67,$61,$63
    .byte $79,$20,$6F,$66,$20,$74,$68,$65,$20,$57,$69,$7A,$61,$72,$64,$0D
    .byte $0D,$20,$20,$20,$20,$20,$20,$20,$20,$40,$31,$39,$38,$37,$20,$20
    .byte $46,$61,$6C,$63,$6F,$6D,$0D,$40,$31,$39,$38,$38,$20,$42,$72,$6F
    .byte $64,$65,$72,$62,$75,$6E,$64,$20,$53,$6F,$66,$74,$77,$61,$72,$65
    .byte $2C,$20,$49,$6E,$63,$2E,$0D,$0D,$0D,$0D,$0D,$0D,$0D,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$44,$51,$45,$50,$44,$60,$47,$4C
    .byte $4C,$46,$48,$49,$45,$C0,$C1,$C0,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$61,$61,$1A,$46,$45,$45,$47,$4B
    .byte $45,$74,$74,$45,$C0,$C1,$C0,$C0,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00
    .byte $00,$00,$00,$00
