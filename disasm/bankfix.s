; PRG banks 14+15 (FIXED, contiguous $C000-$FFFF) — file 0x1C010..0x20010
; 6979 instructions, 14015/16384 code bytes, 863 labels
; PRG bank FIX — CPU origin $C000
.segment "CODEFIX"
L_C000:
    SEI
    LDX #$FF
    TXS
    LDA #$00
    STA $2000
    STA $2001
    STA $4010
    LDA #$1F
    STA a:$0027
    STA $4015
    LDA #$C0
    STA $4017
L_C01C:
    LDA $2002
    BPL L_C01C
L_C021:
    LDA $2002
    BPL L_C021
L_C026:
    LDX #$FF
    TXS
    LDA #$00
    STA $A000
    JSR L_CD08
    JMP L_C041
    .byte $80,$A9,$07,$85,$25,$8D,$00,$80,$A9,$0D,$8D,$01,$80
L_C041:
    JSR L_D1C8
    LDA #$64
    STA $0E
    LDA #$AE
    STA $0F
    JSR L_CC9C
L_C04F:
    LDA #$00
    STA $46
    STA $7B
    STA $43
    LDA #$30
    STA $7C
    LDA #$3C
    STA $44
    LDA #$A0
    STA $45
    JSR L_C8F2
    LDA #$08
    STA $20
    JSR L_D42B
L_C06D:
    LDA $58
    BNE L_C093
    LDA #$00
    STA $85
    JSR L_C1D8
    LDA #$07
    STA $0E
    LDA #$B3
    STA $0F
    JSR L_CC9C
    CPX #$00
    BNE L_C08A
    JMP L_C06D
L_C08A:
    DEX
    BNE L_C090
    JMP L_C04F
L_C090:
    JMP L_C026
L_C093:
    LDA #$01
    STA $36
    LDA $7C
    STA $7E
    JSR L_CC43
    JSR L_D42B
    LDA $EC
    BNE L_C0C9
    JSR L_F628
    JSR L_E87C
    JSR L_F782
    JSR L_C15D
    PHP
    JSR L_C1D8
    JSR L_C2B1
    PLP
    BCS L_C0C3
    LDA $7E
    CMP $7C
    BEQ L_C0C3
    INC $3D
L_C0C3:
    JSR L_C135
    JMP L_C06D
L_C0C9:
    LDA #$EB
    STA $0E
    LDA #$A2
    STA $0F
    JSR L_CC9C
L_C0D4:
    JSR L_CC43
    LDA #$BC
    STA $0E
    LDA #$AB
    STA $0F
    JSR L_CC9C
    LDA #$E6
    STA $0E
    LDA #$A5
    STA $0F
    JSR L_CC9C
    LDA #$5D
    STA $0E
    LDA #$A7
    STA $0F
    JSR L_CC9C
    LDA #$E3
    STA $0E
    LDA #$A3
    STA $0F
    JSR L_CC9C
    LDA $58
    BNE L_C0D4
    LDA $43
    LSR A
    LSR A
    LSR A
    LSR A
    STA $44
    LDA $43
    AND #$0F
    STA $43
    LDA #$EF
    STA $0200
    LDA #$00
    STA $85
    JSR L_C1D8
    LDA #$07
    STA $0E
    LDA #$B3
    STA $0F
    JSR L_CC9C
    DEX
    BNE L_C132
    JMP L_C04F
L_C132:
    JMP L_C026
L_C135:
    LDA $3D
    BEQ L_C143
    LDA #$00
    STA $3D
    JSR L_C7FE
    JMP L_C158
L_C143:
    LDA $3C
    BEQ L_C151
    LDA #$00
    STA $3C
    JSR L_CAA5
    JMP L_C158
L_C151:
    LDA $36
    BEQ L_C158
    JSR L_C569
L_C158:
    LDA $36
    BNE L_C158
    RTS
L_C15D:
    LDA $7C
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $7B
    STA $08
    LDA $44
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $43
    SEC
    SBC $08
    CMP #$60
    BCC L_C19D
    CMP #$91
    BCC L_C1C2
    LDA $7C
    CMP #$30
    BCS L_C192
    LDA $44
    SEC
    SBC #$09
    STA $7C
    LDA $43
    STA $7B
    LDA #$01
    STA $7F
    JMP L_C1BD
L_C192:
    LDA #$30
    STA $7C
    LDA #$00
    STA $7B
    JMP L_C1C2
L_C19D:
    LDA $7C
    ORA $7B
    BEQ L_C1C2
    LDA $44
    SEC
    SBC #$06
    BCC L_C1B7
    STA $7C
    LDA $43
    STA $7B
    LDA #$FF
    STA $7F
    JMP L_C1BD
L_C1B7:
    LDA #$00
    STA $7B
    STA $7C
L_C1BD:
    JSR L_C1C7
    CLC
    RTS
L_C1C2:
    JSR L_C1C7
    SEC
    RTS
L_C1C7:
    LDA $7C
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $7B
    TAX
    LDA #$00
    ROL A
    STX $1C
    STA $1D
    RTS
L_C1D8:
    LDA $85
    BEQ L_C1EB
    LDA $84
    AND #$01
    BNE L_C1EB
    LDA #$EF
    STA $0210
    STA $0214
    RTS
L_C1EB:
    LDA $45
    CLC
    ADC #$2B
    STA $0210
    STA $0214
    LDA $7C
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $7B
    STA $08
    LDA $44
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $43
    SEC
    SBC $08
    STA $0213
    CLC
    ADC #$08
    STA $0217
    LDA $57
    STA $0212
    STA $0216
    LDX $56
    BIT $57
    BVS L_C22B
    STX $0211
    INX
    INX
    STX $0215
    RTS
L_C22B:
    STX $0215
    INX
    INX
    STX $0211
    RTS
L_C234:
    LDA $55
    CMP #$03
    LDX #$13
    BCC L_C247
    LDX #$EF
    STX $0238
    STX $023C
    JMP L_C26F
L_C247:
    STX $0238
    STX $023C
    ASL A
    ASL A
    ASL A
    ASL A
    CLC
    ADC #$C8
    STA $023B
    CLC
    ADC #$08
    STA $023F
    LDA #$FF
    STA $0239
    STA $023D
    LDA #$01
    STA $023A
    LDA #$41
    STA $023E
L_C26F:
    LDX #$02
    LDY #$10
L_C273:
    LDA $51,X
    BMI L_C2A0
    ASL A
    ASL A
    CLC
    ADC #$A1
    STA $0221,Y
    CLC
    ADC #$02
    STA $0225,Y
    TYA
    ASL A
    CLC
    ADC #$C8
    STA $0223,Y
    CLC
    ADC #$08
    STA $0227,Y
    LDA #$01
    STA $0222,Y
    STA $0226,Y
    LDA #$13
    JMP L_C2A2
L_C2A0:
    LDA #$EF
L_C2A2:
    STA $0220,Y
    STA $0224,Y
    TYA
    SEC
    SBC #$08
    TAY
    DEX
    BPL L_C273
    RTS
L_C2B1:
    LDA #$10
    STA $0A
    LDX $3F
    LDY $3E
L_C2B9:
    JSR L_C2DB
    TXA
    CLC
    ADC #$08
    ORA #$80
    TAX
    TYA
    CLC
    ADC #$30
    TAY
    DEC $0A
    BNE L_C2B9
    TXA
    CLC
    ADC #$38
    ORA #$80
    STA $3F
    TYA
    CLC
    ADC #$10
    STA $3E
    RTS
L_C2DB:
    LDA $0401,Y
    BEQ L_C35A
    LDA $040E,Y
    CMP #$BF
    BCS L_C35A
    LDA $0402,Y
    STA $0202,X
    STA $0206,X
    AND #$40
    BNE L_C302
    LDA $0400,Y
    STA $0201,X
    ADC #$02
    STA $0205,X
    JMP L_C30D
L_C302:
    LDA $0400,Y
    STA $0205,X
    ADC #$02
    STA $0201,X
L_C30D:
    LDA $040C,Y
    SEC
    SBC $7B
    AND #$0F
    STA $08
    LDA $040D,Y
    SBC $7C
    CMP #$10
    BCS L_C35A
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $08
    STA $08
    LDA $0401,Y
    CMP #$01
    BNE L_C33E
    LDA $040F,Y
    BEQ L_C33E
    CLC
    ADC $08
    STA $08
    LDA #$00
    STA $040F,Y
L_C33E:
    LDA $08
    CMP #$EF
    BCS L_C363
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
L_C35A:
    LDA #$EF
    STA $0200,X
    STA $0204,X
    RTS
L_C363:
    STA $0203,X
    LDA $040E,Y
    CLC
    ADC #$2B
    STA $0200,X
    LDA #$EF
    STA $0204,X
    RTS
L_C375:
    LDX #$03
L_C377:
    LDA $FF6B,X
    STA $0200,X
    DEX
    BPL L_C377
    LDX #$04
L_C382:
    LDA #$F8
    STA $0200,X
    INX
    BNE L_C382
    RTS
    LDA $23
    PHA
    AND #$7B
    STA $2000
    LDA #$00
    STA $29
    LDA $24
    PHA
    AND #$E7
    STA $2001
    LDA #$20
    STA $2006
    LDA #$00
    STA $2006
    LDA #$C0
    LDY #$05
L_C3AD:
    LDX #$C0
L_C3AF:
    STA $2007
    DEX
    BNE L_C3AF
    DEY
    BNE L_C3AD
    LDA #$00
    LDX #$40
L_C3BC:
    STA $2007
    DEX
    BNE L_C3BC
    LDA #$C0
    LDY #$05
L_C3C6:
    LDX #$C0
L_C3C8:
    STA $2007
    DEX
    BNE L_C3C8
    DEY
    BNE L_C3C6
    LDA #$00
    LDX #$40
L_C3D5:
    STA $2007
    DEX
    BNE L_C3D5
    PLA
    STA $24
    PLA
    STA $23
    STA $2000
    RTS
L_C3E5:
    INC $92
    LDY #$04
L_C3E9:
    TYA
    PHA
    LDA #$05
    STA $36
    LDX #$1C
L_C3F1:
    LDA $0184,X
    AND #$0F
    STA $08
    LDA $0184,X
    AND #$F0
    SEC
    SBC #$10
    BCS L_C407
    LDA #$0F
    JMP L_C409
L_C407:
    ORA $08
L_C409:
    STA $0184,X
    DEX
    BPL L_C3F1
    LSR $A0
    LSR $B0
    LSR $D0
    LDA #$00
    STA $B4
    JSR L_C135
    PLA
    TAY
    DEY
    BNE L_C3E9
    LDA #$FF
    STA $8E
    LDA #$00
    STA $94
    STA $A4
    STA $C4
    STA $92
    RTS
L_C430:
    LDY #$04
L_C432:
    TYA
    PHA
    LDA #$05
    STA $36
    LDX #$1C
L_C43A:
    LDA $0184,X
    AND #$0F
    STA $08
    LDA $0184,X
    AND #$F0
    SEC
    SBC #$10
    BCS L_C450
    LDA #$0F
    JMP L_C452
L_C450:
    ORA $08
L_C452:
    STA $0184,X
    DEX
    BPL L_C43A
    JSR L_C135
    PLA
    TAY
    DEY
    BNE L_C432
    RTS
    LDY #$04
L_C463:
    TYA
    PHA
    LDA #$05
    STA $36
    LDX #$20
L_C46B:
    LDA $0180,X
    AND #$0F
    STA $08
    LDA $0180,X
    AND #$F0
    SEC
    SBC #$10
    BCS L_C481
    LDA #$0F
    JMP L_C483
L_C481:
    ORA $08
L_C483:
    STA $0180,X
    DEX
    BPL L_C46B
    JSR L_C135
    PLA
    TAY
    DEY
    BNE L_C463
    RTS
L_C492:
    LDA #$40
    STA $09
L_C496:
    LDA #$05
    STA $36
    JSR L_C9FB
    LDX #$04
    LDY #$1C
    JSR L_C520
    JSR L_C135
    LDA $09
    SEC
    SBC #$10
    STA $09
    BPL L_C496
    JSR L_C569
    RTS
    .byte $A9,$40,$85,$09,$A9,$05,$85,$36,$A2,$04,$A0,$E0,$B1,$77,$99,$A0
    .byte $00,$C8,$CA,$D0,$F7,$A2,$00,$A0,$04,$20,$20,$C5,$20,$35,$C1,$A5
    .byte $09,$38,$E9,$10,$85,$09,$10,$DC,$20,$69,$C5,$60,$A9,$40,$85,$09
    .byte $A9,$05,$85,$36,$A2,$04,$A0,$E0,$B1,$77,$99,$A0,$00,$C8,$CA,$D0
    .byte $F7,$A2,$04,$A0,$F0,$B1,$77,$99,$A0,$00,$C8,$CA,$D0,$F7,$A2,$00
    .byte $A0,$04,$20,$20,$C5,$A2,$10,$A0,$04,$20,$20,$C5,$20,$35,$C1,$A5
    .byte $09,$38,$E9,$10,$85,$09,$10,$C8,$20,$69,$C5,$60
L_C520:
    LDA $0180,X
    AND #$0F
    STA $08
    LDA $0180,X
    AND #$F0
    SEC
    SBC $09
    BCS L_C536
    LDA #$0F
    JMP L_C538
L_C536:
    ORA $08
L_C538:
    STA $0180,X
    INX
    DEY
    BNE L_C520
    RTS
L_C540:
    TXA
    PHA
    LDA #$30
    LDX #$1F
L_C546:
    STA $0180,X
    DEX
    BPL L_C546
    JSR L_C569
    LDA #$01
    STA $36
    JSR L_C135
    JSR L_C9FB
    JSR L_C569
    LDA #$02
    STA $36
    JSR L_C135
    PLA
    TAX
    DEX
    BNE L_C540
    RTS
L_C569:
    JSR L_CC97
    LDA #$00
    STA $16
    LDA #$3F
    STA $17
    LDA #$02
    JSR L_CC8F
    RTS
    JSR L_CC97
    LDA $23
    PHA
    AND #$7B
    STA $2000
    LDA #$00
    STA $29
    LDA $24
    PHA
    AND #$E7
    STA $2001
    LDA #$23
    STA $2006
    LDA #$20
    STA $2006
    LDY #$A0
    LDX #$00
L_C59F:
    LDA $FECB,X
    STA $2007
    INX
    DEY
    BNE L_C59F
    LDA #$23
    STA $2006
    LDA #$F0
    STA $2006
    LDY #$10
    LDA #$00
L_C5B7:
    STA $2007
    DEY
    BNE L_C5B7
    LDA #$01
    INC $29
    PLA
    STA $24
    PLA
    STA $23
    STA $2000
    RTS
L_C5CB:
    LDA $7C
    AND #$FE
    STA $0C
    LDA #$00
    STA $0D
    JSR L_CA54
    JSR L_C5F7
    RTS
L_C5DC:
    LDA $7C
    AND #$FE
    STA $0C
    LDA #$00
    STA $0D
    JSR L_CA54
    LDA $0D
    SEC
    SBC #$05
    CLC
    ADC $76
    STA $0D
    JSR L_C5F7
    RTS
L_C5F7:
    LDA $23
    PHA
    AND #$7F
    ORA #$04
    STA $2000
    LDA $29
    PHA
    LDA #$00
    STA $29
    LDA $24
    PHA
    AND #$E7
    STA $2001
    LDA $0C
    PHA
    LDA $0D
    PHA
    LDA $7C
    ASL A
    AND #$1C
    STA $16
    LDA $7C
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    LDA #$12
    STA $0A
L_C636:
    LDA #$0C
    STA $0B
    LDA $17
    STA $2006
    LDA $16
    STA $2006
    LDY #$00
    STY $08
L_C648:
    LDY $08
    LDA ($0C),Y
    ASL A
    ASL A
    TAY
    LDA ($79),Y
    STA $2007
    INY
    LDA ($79),Y
    STA $2007
    INC $08
    DEC $0B
    BNE L_C648
    LDA #$0C
    STA $0B
    LDA $17
    STA $2006
    LDY $16
    INY
    STY $2006
    LDY #$00
    STY $08
L_C673:
    LDY $08
    LDA ($0C),Y
    ASL A
    ASL A
    TAY
    INY
    INY
    LDA ($79),Y
    STA $2007
    INY
    LDA ($79),Y
    STA $2007
    INC $08
    DEC $0B
    BNE L_C673
    INC $16
    INC $16
    LDA $16
    AND #$20
    BEQ L_C6A1
    LDA #$00
    STA $16
    LDA $17
    EOR #$04
    STA $17
L_C6A1:
    CLC
    LDA #$0C
    ADC $0C
    STA $0C
    LDA #$00
    ADC $0D
    STA $0D
    DEC $0A
    BNE L_C636
    PLA
    STA $0D
    PLA
    STA $0C
    LDA $7C
    LSR A
    AND #$07
    STA $16
    LDA $7C
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$C0
    ADC $16
    STA $16
    LDA #$23
    ADC $17
    STA $17
    LDA #$09
    STA $0A
L_C6D8:
    LDX #$06
L_C6DA:
    LDY #$0D
    LDA ($0C),Y
    ROL A
    ROL $08
    ROL A
    ROL $08
    LDY #$01
    LDA ($0C),Y
    ROL A
    ROL $08
    ROL A
    ROL $08
    LDY #$0C
    LDA ($0C),Y
    ROL A
    ROL $08
    ROL A
    ROL $08
    LDY #$00
    LDA ($0C),Y
    ROL A
    ROL $08
    ROL A
    ROL $08
    LDA $17
    STA $2006
    LDA $16
    STA $2006
    LDA $08
    STA $2007
    CLC
    LDA #$02
    ADC $0C
    STA $0C
    LDA #$00
    ADC $0D
    STA $0D
    CLC
    LDA #$08
    ADC $16
    STA $16
    LDA #$00
    ADC $17
    STA $17
    DEX
    BNE L_C6DA
    CLC
    LDA #$0C
    ADC $0C
    STA $0C
    LDA #$00
    ADC $0D
    STA $0D
    CLC
    LDA #$D1
    ADC $16
    STA $16
    LDA #$FF
    ADC $17
    STA $17
    LDA $16
    AND #$08
    BEQ L_C758
    LDA #$C0
    STA $16
    LDA $17
    EOR #$04
    STA $17
L_C758:
    DEC $0A
    BEQ L_C75F
    JMP L_C6D8
L_C75F:
    PLA
    STA $24
    PLA
    STA $29
    PLA
    STA $23
    STA $2000
    RTS
L_C76C:
    JSR L_CC97
    LDA $7C
    ASL A
    AND #$1F
    STA $16
    LDA $7C
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    LDA $7C
    STA $08
    LDA #$10
    STA $09
L_C793:
    LDA $08
    STA $0C
    JSR L_C833
    INC $16
    INC $16
    LDA $16
    AND #$20
    BEQ L_C7AE
    LDA #$00
    STA $16
    LDA $17
    EOR #$04
    STA $17
L_C7AE:
    INC $08
    DEC $09
    BNE L_C793
    RTS
L_C7B5:
    JSR L_CC97
    LDA $7C
    ASL A
    AND #$1F
    STA $16
    LDA $7C
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    LDA $7C
    STA $08
    LDA #$10
    STA $09
L_C7DC:
    LDA $08
    STA $0C
    JSR L_C85C
    INC $16
    INC $16
    LDA $16
    AND #$20
    BEQ L_C7F7
    LDA #$00
    STA $16
    LDA $17
    EOR #$04
    STA $17
L_C7F7:
    INC $08
    DEC $09
    BNE L_C7DC
    RTS
L_C7FE:
    JSR L_CC97
    LDA $7F
    BMI L_C80F
    LDA $7C
    CLC
    ADC #$10
    STA $0C
    JMP L_C813
L_C80F:
    LDA $7C
    STA $0C
L_C813:
    LDA $0C
    ASL A
    AND #$1F
    STA $16
    LDA $0C
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    JSR L_C833
    RTS
L_C833:
    LDA $31
    PHA
    LDA #$07
    STA $25
    STA $8000
    LDA #$09
    STA $31
    STA $8001
    LDA #$00
    STA $0D
    JSR L_CA54
    JSR L_C871
    LDA #$07
    STA $25
    STA $8000
    PLA
    STA $31
    STA $8001
    RTS
L_C85C:
    LDA #$00
    STA $0D
    JSR L_CA54
    LDA $0D
    SEC
    SBC #$05
    CLC
    ADC $76
    STA $0D
    JSR L_C871
    RTS
L_C871:
    LDA #$00
    STA $0B
    LDX #$16
L_C877:
    LDY $0B
    LDA ($0C),Y
    ASL A
    ASL A
    TAY
    LDA ($79),Y
    STA $0141,X
    INY
    LDA ($79),Y
    STA $0140,X
    INY
    LDA ($79),Y
    STA $0159,X
    INY
    LDA ($79),Y
    STA $0158,X
    INC $0B
    DEX
    DEX
    BPL L_C877
    LDA $17
    CLC
    ADC #$03
    STA $19
    LDA $16
    LSR A
    LSR A
    CLC
    ADC #$C0
    STA $0B
    LDX #$33
    LDA $16
    AND #$02
    BNE L_C8B5
    LDX #$CC
L_C8B5:
    STX $18
    LDY #$00
    LDX #$0A
L_C8BB:
    LDA $0B
    STA $0170,X
    CLC
    ADC #$08
    STA $0B
    LDA ($0C),Y
    INY
    AND #$C0
    LSR A
    LSR A
    LSR A
    LSR A
    STA $0171,X
    LDA ($0C),Y
    INY
    AND #$C0
    ORA $0171,X
    STA $0171,X
    LDA $16
    AND #$02
    BNE L_C8E8
    LSR $0171,X
    LSR $0171,X
L_C8E8:
    DEX
    DEX
    BPL L_C8BB
    LDA #$03
    JSR L_CC8F
    RTS
L_C8F2:
    JSR L_C9D2
    JSR L_C9A9
    JSR L_C909
    JSR L_C9FB
    RTS
L_C8FF:
    JSR L_C9D2
    JSR L_C909
    JSR L_C9FB
    RTS
L_C909:
    LDY #$00
    LDA ($77),Y
    ADC #$A0
    STA $7A
    LDA #$00
    STA $79
    INY
    LDA ($77),Y
    STA $2D
    INY
    LDA ($77),Y
    STA $70
    INY
    LDA ($77),Y
    STA $71
    INY
    LDA ($77),Y
    STA $74
    INY
    LDA ($77),Y
    ORA #$00
    STA $2A
    INY
    LDA ($77),Y
    ORA #$00
    STA $2B
    LDY #$07
    JSR L_CA1E
    LDA #$00
    BCC L_C942
    LDA ($77),Y
L_C942:
    STA $04A1
    BEQ L_C973
    LDA #$01
    STA $04A2
    INY
    LDA ($77),Y
    STA $04AD
    LDA #$00
    STA $04AC
    INY
    LDA ($77),Y
    STA $04AE
    INY
    LDA ($77),Y
    CMP #$17
    BNE L_C96E
    LDA #$19
    STA $04A1
    LDA #$DD
    JMP L_C970
L_C96E:
    LDA #$E9
L_C970:
    STA $04A0
L_C973:
    LDX $8E
    CPX #$05
    BCS L_C986
    LDA #$00
    SEC
L_C97C:
    ROL A
    DEX
    BPL L_C97C
    LDY #$15
    AND ($77),Y
    BNE L_C98D
L_C986:
    LDY #$0B
    LDA ($77),Y
    JSR L_D02E
L_C98D:
    LDY #$10
    LDA ($77),Y
    STA $80
    INY
    LDA ($77),Y
    STA $81
    INY
    LDA ($77),Y
    STA $82
    INY
    LDA ($77),Y
    STA $83
    LDY #$14
    LDA ($77),Y
    STA $41
    RTS
L_C9A9:
    LDA $75
    STA $77
    LDA $76
    STA $78
    LDY #$00
L_C9B3:
    LDA ($77),Y
    STA $0500,Y
    INY
    BNE L_C9B3
    INC $78
L_C9BD:
    LDA ($77),Y
    STA $0600,Y
    INY
    BNE L_C9BD
    INC $78
L_C9C7:
    LDA ($77),Y
    STA $0700,Y
    INY
    BNE L_C9C7
    INC $78
    RTS
L_C9D2:
    LDA $48
    LSR A
    CMP $30
    BEQ L_C9E0
    STA $30
    LDA #$FF
    JSR L_CC8F
L_C9E0:
    LDA $48
    AND #$01
    ASL A
    ASL A
    ORA $47
    ASL A
    ASL A
    CLC
    ADC #$80
    STA $76
    CLC
    ADC #$03
    STA $78
    LDA #$00
    STA $77
    STA $75
    RTS
L_C9FB:
    LDY #$E0
L_C9FD:
    LDA ($77),Y
    STA a:$00A0,Y
    INY
    BMI L_C9FD
    LDA $40
    CMP #$06
    BCS L_CA1D
    ASL A
    ASL A
    CLC
    ADC #$03
    TAX
    LDY #$03
L_CA13:
    LDA $FFC5,X
    STA $0190,Y
    DEX
    DEY
    BPL L_CA13
L_CA1D:
    RTS
L_CA1E:
    LDA $48
    ASL A
    ASL A
    AND #$04
    ORA $47
    TAX
    LDA $0300,X
    PHA
    LDA $48
    LSR A
    TAX
    INX
    PLA
L_CA31:
    ASL A
    DEX
    BNE L_CA31
    RTS
L_CA36:
    LDA $48
    LSR A
    TAX
    INX
    LDA #$FF
    CLC
L_CA3E:
    ROR A
    DEX
    BNE L_CA3E
    PHA
    LDA $48
    ASL A
    ASL A
    AND #$04
    ORA $47
    TAX
    PLA
    AND $0300,X
    STA $0300,X
    RTS
L_CA54:
    LDA $0D
    PHA
    JSR L_CA85
    LDA $0D
    STA $11
    PLA
    LSR A
    LSR A
    LSR A
    LSR A
    CLC
    ADC $0C
    STA $0C
    STA $10
    BCC L_CA70
    INC $0D
    INC $11
L_CA70:
    CLC
    LDA $0D
    ADC #$05
    STA $0D
    CLC
    LDA $10
    ADC $75
    STA $10
    LDA $11
    ADC $76
    STA $11
    RTS
L_CA85:
    LDA #$00
    STA $0D
    ASL $0C
    ROL $0D
    ASL $0C
    ROL $0D
    LDX $0D
    LDY $0C
    ASL $0C
    ROL $0D
    TYA
    CLC
    ADC $0C
    STA $0C
    TXA
    ADC $0D
    STA $0D
    RTS
L_CAA5:
    JSR L_CC97
    LDA #$60
    STA $16
    LDA #$23
    STA $17
    LDA #$04
    JSR L_CC8F
    RTS
L_CAB6:
    LDA $58
    CMP #$6D
    BCC L_CABE
    LDA #$6D
L_CABE:
    STA $58
    STA $08
    LDX #$00
    JSR L_CB0E
    LDA #$01
    STA $3C
    RTS
L_CACC:
    LDA $59
    CMP #$6D
    BCC L_CAD4
    LDA #$6D
L_CAD4:
    STA $59
    STA $08
    LDX #$06
    JSR L_CB0E
    LDA #$01
    STA $3C
    RTS
L_CAE2:
    LDA $5B
    CMP #$6D
    BCC L_CAEA
    LDA #$6D
L_CAEA:
    STA $5B
    STA $08
    LDX #$0C
    JSR L_CB0E
    LDA #$01
    STA $3C
    RTS
L_CAF8:
    LDA $5A
    CMP #$6D
    BCC L_CB00
    LDA #$6D
L_CB00:
    STA $5A
    STA $08
    LDX #$12
    JSR L_CB0E
    LDA #$01
    STA $3C
    RTS
L_CB0E:
    TXA
    PHA
    LDY #$05
    LDA #$DC
L_CB14:
    STA $0101,X
    INX
    DEY
    BNE L_CB14
    PLA
    PHA
    TAX
    LDY #$05
    LDA #$DF
L_CB22:
    STA $0121,X
    INX
    DEY
    BNE L_CB22
    PLA
    TAX
    JSR L_CBFA
    TXA
L_CB2F:
    DEY
    BEQ L_CB3F
    DEC $0101,X
    DEY
    BEQ L_CB3F
    DEC $0101,X
    INX
    JMP L_CB2F
L_CB3F:
    TAX
    LDY $08
L_CB42:
    DEY
    BEQ L_CB52
    DEC $0121,X
    DEY
    BEQ L_CB52
    DEC $0121,X
    INX
    JMP L_CB42
L_CB52:
    RTS
    .byte $AD,$05,$04,$C9,$6D,$90,$02,$A9,$6D,$85,$08,$A9,$00,$85,$09,$A2
    .byte $A5,$A0,$AB,$4C,$94,$CB,$AD,$05,$04,$C9,$6D,$90,$02,$A9,$6D,$85
    .byte $08,$A9,$00,$85,$09,$A2,$65,$A0,$6B,$4C,$94,$CB,$A5,$58,$C9,$6D
    .byte $90,$02,$A9,$6D,$85,$08,$A9,$80,$85,$09,$A2,$65,$A0,$6B,$4C,$94
    .byte $CB,$8A,$A6,$09,$9D,$59,$02,$9D,$5D,$02,$9D,$61,$02,$9D,$65,$02
    .byte $9D,$69,$02,$98,$9D,$6D,$02,$9D,$71,$02,$9D,$75,$02,$9D,$79,$02
    .byte $9D,$7D,$02,$20,$FA,$CB,$A5,$09,$18,$69,$18,$AA,$88,$F0,$16,$DE
    .byte $41,$02,$DE,$41,$02,$88,$F0,$0D,$DE,$41,$02,$DE,$41,$02,$E8,$E8
    .byte $E8,$E8,$4C,$BF,$CB,$A5,$09,$18,$69,$2C,$AA,$A4,$08,$88,$F0,$16
    .byte $DE,$41,$02,$DE,$41,$02,$88,$F0,$0D,$DE,$41,$02,$DE,$41,$02,$E8
    .byte $E8,$E8,$E8,$4C,$E0,$CB,$60
L_CBFA:
    LDA $08
    LDY #$00
    SEC
L_CBFF:
    INY
    SBC #$0A
    BCS L_CBFF
    ADC #$0B
    STA $08
    RTS
L_CC09:
    JSR L_CC17
    JSR L_CC2D
    PHA
    JSR L_CC17
    PLA
    STA $20
    RTS
L_CC17:
    LDA #$01
    STA $36
    JSR L_C1D8
    JSR L_C2B1
    JSR L_C234
    JSR L_C135
    JSR L_CC43
    BNE L_CC17
    RTS
L_CC2D:
    LDA #$01
    STA $36
    JSR L_C1D8
    JSR L_C2B1
    JSR L_C234
    JSR L_C135
    JSR L_CC43
    BEQ L_CC2D
    RTS
L_CC43:
    LDX #$01
    STX $4016
    DEX
    STX $4016
    LDX #$08
L_CC4E:
    LDA $4016
    ORA $4017
    LSR A
    ROL $20
    LSR A
    ROL $21
    DEX
    BNE L_CC4E
    LDA $20
    ORA $21
    STA $20
    RTS
L_CC64:
    STA $38
    BEQ L_CC8E
    LDX $3B
    LDY $3A
L_CC6C:
    STY $39
    TYA
    ASL A
    TAY
    TXA
    ROL A
    TAX
    INY
    BNE L_CC78
    INX
L_CC78:
    CLC
    TYA
    ADC $3A
    TAY
    TXA
    ADC $3B
    CLC
    ADC $39
    AND #$7F
    TAX
    STX $3B
    STY $3A
    CMP $38
    BCS L_CC6C
L_CC8E:
    RTS
L_CC8F:
    PHA
L_CC90:
    LDA $28
    BNE L_CC90
    PLA
    STA $28
L_CC97:
    LDA $28
    BNE L_CC97
    RTS
L_CC9C:
    LDA $30
    STA $32
    LDA $31
    STA $33
    LDA #$CC
    PHA
    LDA #$C7
    PHA
    LDY #$06
    STY $25
    STY $8000
    LDA #$0C
    STA $30
    STA $8001
    INY
    STY $25
    STY $8000
    LDA #$0D
    STA $31
    STA $8001
    JMP ($000E)
    LDY #$07
    STY $25
    STY $8000
    LDA $33
    STA $31
    STA $8001
    DEY
    STY $25
    STY $8000
    LDA $32
    STA $30
    STA $8001
    RTS
L_CCE4:
    LDA #$CD
    PHA
    LDA #$07
    PHA
    LDY #$07
    STY $25
    STY $8000
    LDA $33
    STA $31
    STA $8001
    DEY
    STY $25
    STY $8000
    LDA $32
    STA $30
    STA $8001
    JMP ($000E)
L_CD08:
    LDA $30
    STA $32
    LDA $31
    STA $33
    LDY #$06
    STY $25
    STY $8000
    LDA #$0C
    STA $30
    STA $8001
    INY
    STY $25
    STY $8000
    LDA #$0D
    STA $31
    STA $8001
    RTS
L_CD2C:
    STY $09
    LDY $09
    BEQ L_CD67
    LDA $20
    AND #$0F
    ASL A
    TAX
    LDA #$00
L_CD3A:
    CLC
    ADC $FE8B,X
    DEY
    BNE L_CD3A
    PHA
    AND #$0F
    STA $49
    LDY #$00
    PLA
    BPL L_CD4D
    LDY #$F0
L_CD4D:
    STY $08
    AND #$F0
    LSR A
    LSR A
    LSR A
    LSR A
    ORA $08
    STA $4A
    LDY $09
    LDA #$00
L_CD5D:
    CLC
    ADC $FE8C,X
    DEY
    BNE L_CD5D
    STA $4B
    RTS
L_CD67:
    LDA #$00
    STA $49
    STA $4A
    STA $4B
    RTS
L_CD70:
    STY $09
    LDY $09
    BEQ L_CDA9
    AND #$0F
    ASL A
    TAX
    LDA #$00
L_CD7C:
    CLC
    ADC $FE8B,X
    DEY
    BNE L_CD7C
    PHA
    AND #$0F
    STA $F5
    LDY #$00
    PLA
    BPL L_CD8F
    LDY #$F0
L_CD8F:
    STY $08
    AND #$F0
    LSR A
    LSR A
    LSR A
    LSR A
    ORA $08
    STA $F6
    LDY $09
    LDA #$00
L_CD9F:
    CLC
    ADC $FE8C,X
    DEY
    BNE L_CD9F
    STA $F7
    RTS
L_CDA9:
    LDA #$00
    STA $F5
    STA $F6
    STA $F7
    RTS
L_CDB2:
    LDY #$09
    LDX #$90
L_CDB6:
    CPY $E3
    BEQ L_CE0A
    LDA $0401,X
    BMI L_CE0A
    CMP #$01
    BEQ L_CDC7
    CMP #$1A
    BCC L_CE0A
L_CDC7:
    LDA $0400,X
    AND #$F9
    CMP #$E1
    BEQ L_CE0A
    LDA $0402,X
    AND #$20
    BNE L_CE0A
    LDA $0A
    SEC
    SBC $040E,X
    CMP #$10
    BCC L_CDE5
    CMP #$F1
    BCC L_CE0A
L_CDE5:
    LDA $0F
    SEC
    SBC $040D,X
    BEQ L_CE14
    CMP #$02
    BCC L_CE02
    CMP #$FF
    BCC L_CE0A
    LDA $0E
    SEC
    SBC $040C,X
    BEQ L_CE0A
    BMI L_CE0A
    JMP L_CE14
L_CE02:
    LDA $0E
    SEC
    SBC $040C,X
    BMI L_CE14
L_CE0A:
    TXA
    SEC
    SBC #$10
    TAX
    DEY
    BPL L_CDB6
    CLC
    RTS
L_CE14:
    STY $08
    STX $09
    SEC
    RTS
L_CE1A:
    LDY #$0A
    LDX #$A0
L_CE1E:
    CPY $E3
    BEQ L_CE6C
    LDA $0401,X
    BEQ L_CE6C
    BMI L_CE6C
    LDA $0400,X
    AND #$F9
    CMP #$E1
    BEQ L_CE6C
    LDA $0402,X
    AND #$20
    BNE L_CE6C
    LDA $0A
    SEC
    SBC $040E,X
    CMP #$10
    BCC L_CE47
    CMP #$F1
    BCC L_CE6C
L_CE47:
    LDA $0F
    SEC
    SBC $040D,X
    BEQ L_CE76
    CMP #$02
    BCC L_CE64
    CMP #$FF
    BCC L_CE6C
    LDA $0E
    SEC
    SBC $040C,X
    BEQ L_CE6C
    BMI L_CE6C
    JMP L_CE76
L_CE64:
    LDA $0E
    SEC
    SBC $040C,X
    BMI L_CE76
L_CE6C:
    TXA
    SEC
    SBC #$10
    TAX
    DEY
    BPL L_CE1E
    CLC
    RTS
L_CE76:
    STY $08
    STX $09
    SEC
    RTS
L_CE7C:
    LDA #$00
    STA $EA
    JSR L_CEB6
    BCC L_CE8F
    JSR L_CE90
    BCC L_CE8F
    LDA #$01
    STA $EA
    SEC
L_CE8F:
    RTS
L_CE90:
    SEC
    LDA $0F
    SBC $44
    BEQ L_CEB4
    CMP #$02
    BCC L_CEAB
    CMP #$FF
    BCC L_CEB2
    SEC
    LDA $0E
    SBC $43
    BEQ L_CEB2
    BMI L_CEB2
    JMP L_CEB4
L_CEAB:
    LDA $0E
    SEC
    SBC $43
    BMI L_CEB4
L_CEB2:
    CLC
    RTS
L_CEB4:
    SEC
    RTS
L_CEB6:
    LDA $0A
    SEC
    SBC $45
    CMP #$10
    BCC L_CEC3
    CMP #$F1
    BCC L_CEC5
L_CEC3:
    SEC
    RTS
L_CEC5:
    CLC
    RTS
L_CEC7:
    LDA #$00
    STA $EA
    LDA $0A
    SEC
    SBC $45
    CMP #$10
    BCC L_CED8
    CMP #$E1
    BCC L_CF00
L_CED8:
    SEC
    LDA $0F
    SBC $44
    BEQ L_CF02
    CMP #$FF
    BEQ L_CF02
    CMP #$02
    BCC L_CEF8
    CMP #$FE
    BCC L_CF00
    SEC
    LDA $0E
    SBC a:$0043
    BEQ L_CF00
    BMI L_CF00
    JMP L_CF02
L_CEF8:
    LDA $0E
    SEC
    SBC a:$0043
    BMI L_CF02
L_CF00:
    CLC
    RTS
L_CF02:
    LDA #$01
    STA $EA
    SEC
    RTS
L_CF08:
    LDA $0A
    CMP #$C0
    BCS L_CF18
    LDA $0F
    CMP #$3F
    BCC L_CF1A
    LDA $0E
    BEQ L_CF1A
L_CF18:
    SEC
    RTS
L_CF1A:
    CLC
    RTS
L_CF1C:
    LDA $0A
    CMP #$B0
    BCS L_CF2C
    LDA $0F
    CMP #$3F
    BCC L_CF2E
    LDA $0E
    BEQ L_CF2E
L_CF2C:
    SEC
    RTS
L_CF2E:
    CLC
    RTS
L_CF30:
    LDX #$0F
L_CF32:
    TXA
    PHA
    LDY $60,X
    JSR L_CF3F
    PLA
    TAX
    DEX
    BPL L_CF32
    RTS
L_CF3F:
    TXA
    PHA
    TXA
    AND #$07
    ASL A
    ASL A
    STA $16
    TXA
    AND #$08
    ASL A
    ASL A
    ASL A
    ASL A
    ORA $16
    STA $16
    LDA #$00
    STA $17
    CLC
    LDA #$C2
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    TYA
    JSR L_CFF9
    PLA
    JSR L_D017
    BCS L_CF7C
    LDA $18
    SEC
    SBC #$40
    STA $18
    LDA $19
    SEC
    SBC #$40
    STA $19
L_CF7C:
    LDA #$06
    JSR L_CC8F
    RTS
L_CF82:
    LDA #$DE
    STA $16
    LDA #$21
    STA $17
    JSR L_D051
    JSR L_CFF9
    LDA #$06
    JSR L_CC8F
    LDA #$1E
    STA $16
    LDA #$22
    STA $17
    JSR L_D038
    JSR L_CFF9
    LDA #$06
    JSR L_CC8F
    LDA #$5E
    STA $16
    LDA #$22
    STA $17
    JSR L_D067
    JSR L_CFF9
    LDA #$06
    JSR L_CC8F
    RTS
L_CFBC:
    LDA #$47
    STA $16
    LDA #$22
    STA $17
    LDA $7C
    AND #$10
    BEQ L_CFD7
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$04
    ADC $17
    STA $17
L_CFD7:
    LDA $81
    JSR L_CFF9
    LDA #$06
    JSR L_CC8F
    CLC
    LDA #$0E
    ADC $16
    STA $16
    LDA #$00
    ADC $17
    STA $17
    LDA $83
    JSR L_CFF9
    LDA #$06
    JSR L_CC8F
    RTS
L_CFF9:
    LDX #$D0
    STX $19
L_CFFD:
    CMP #$0A
    BCC L_D008
    SBC #$0A
    INC $19
    JMP L_CFFD
L_D008:
    ADC #$D0
    STA $18
    LDA $19
    CMP #$D0
    BNE L_D016
    LDA #$C0
    STA $19
L_D016:
    RTS
L_D017:
    PHA
    LDA $40
    ASL A
    TAX
    PLA
    CMP #$08
    BCC L_D022
    INX
L_D022:
    AND #$07
    TAY
    INY
    LDA $FFBB,X
L_D029:
    ASL A
    DEY
    BNE L_D029
    RTS
L_D02E:
    CMP $8E
    BEQ L_D037
    STA $8E
    JSR L_FC08
L_D037:
    RTS
L_D038:
    LDX $55
    LDA $51,X
    CMP #$06
    BNE L_D04D
    LDA $59
    BEQ L_D04D
    LDA $5C
    LSR A
    LSR A
    CLC
    ADC $5C
    CLC
    RTS
L_D04D:
    LDA $5C
    SEC
    RTS
L_D051:
    LDX $55
    LDA $51,X
    CMP #$08
    BNE L_D063
    LDA $59
    BEQ L_D063
    LDA $5D
    ASL A
    ASL A
    CLC
    RTS
L_D063:
    LDA $5D
    SEC
    RTS
L_D067:
    LDX $55
    LDA $51,X
    CMP #$09
    BNE L_D078
    LDA $59
    BEQ L_D078
    LDA $5F
    ASL A
    CLC
    RTS
L_D078:
    LDA $5F
    SEC
    RTS
L_D07C:
    LDA #$EF
    LDX #$80
L_D080:
    STA $0200,X
    INX
    INX
    INX
    INX
    BNE L_D080
    RTS
L_D08A:
    LDY #$10
    LDX #$00
L_D08E:
    LDA #$00
    STA $0401,X
    LDA #$02
    STA $0406,X
    TXA
    CLC
    ADC #$10
    TAX
    DEY
    BNE L_D08E
    LDA #$00
    STA $E9
    RTS
L_D0A5:
    LDX #$07
L_D0A7:
    LDA $0300,X
    STA $0308,X
    DEX
    BPL L_D0A7
    LDX #$0F
L_D0B2:
    LDA $60,X
    STA $0310,X
    DEX
    BPL L_D0B2
    LDA $5A
    STA $0321
    LDA $5B
    STA $0320
    RTS
L_D0C5:
    LDX #$07
L_D0C7:
    LDA $0308,X
    STA $0300,X
    DEX
    BPL L_D0C7
    LDX #$0F
L_D0D2:
    LDA $0310,X
    STA $60,X
    DEX
    BPL L_D0D2
    LDA $0321
    STA $5A
    LDA $0320
    STA $5B
    RTS
L_D0E5:
    LDY #$1F
    LDX #$26
L_D0E9:
    LDA $0322,Y
    ORA #$80
    CMP #$A0
    BCC L_D0F4
    LDA #$7F
L_D0F4:
    STA $0362,X
    DEX
    DEY
    LDA $0322,Y
    ORA #$80
    CMP #$A0
    BCC L_D104
    LDA #$7F
L_D104:
    STA $0362,X
    DEX
    DEY
    LDA $0322,Y
    ORA #$80
    CMP #$A0
    BCC L_D114
    LDA #$7F
L_D114:
    STA $0362,X
    DEX
    DEY
    LDA $0322,Y
    ORA #$80
    CMP #$A0
    BCC L_D124
    LDA #$7F
L_D124:
    STA $0362,X
    DEX
    DEY
    DEX
    BPL L_D0E9
    LDA #$13
    STA $1A
    LDA #$00
    STA $1B
    LDA #$E6
    STA $16
    LDA #$24
    STA $17
    LDA #$62
    STA $18
    LDA #$03
    STA $19
    LDA #$05
    JSR L_CC8F
    LDA #$06
    STA $16
    LDA #$25
    STA $17
    LDA #$76
    STA $18
    LDA #$03
    STA $19
    LDA #$05
    JSR L_CC8F
    RTS
L_D15F:
    LDX #$1F
    LDA #$7F
L_D163:
    STA $0322,X
    DEX
    BPL L_D163
    RTS
L_D16A:
    LDA $85
    PHA
    LDA #$00
    STA $85
    JSR L_C1D8
L_D174:
    INC $58
    JSR L_CAB6
    LDA #$16
    STA $8F
    LDA #$02
    STA $36
    JSR L_C135
    LDX $58
    CPX #$63
    BCC L_D174
    LDA #$17
    STA $8F
    LDA #$10
    STA $36
    JSR L_C135
    PLA
    STA $85
    RTS
L_D199:
    LDA $85
    PHA
    LDA #$00
    STA $85
    JSR L_C1D8
L_D1A3:
    INC $59
    JSR L_CACC
    LDA #$16
    STA $8F
    LDA #$02
    STA $36
    JSR L_C135
    LDX $59
    CPX #$63
    BCC L_D1A3
    LDA #$17
    STA $8F
    LDA #$10
    STA $36
    JSR L_C135
    PLA
    STA $85
    RTS
L_D1C8:
    LDX #$00
L_D1CA:
    LDA $9B9F,X
    STA $00,X
    INX
    BNE L_D1CA
    LDX #$3F
L_D1D4:
    LDA $9C9E,X
    STA $0100,X
    DEX
    BPL L_D1D4
    LDA #$0F
    LDX #$1F
L_D1E1:
    STA $0180,X
    DEX
    BPL L_D1E1
    LDX #$00
L_D1E9:
    LDA $9D3E,X
    STA $0300,X
    INX
    BNE L_D1E9
    LDX #$00
L_D1F4:
    LDA $9DC9,X
    STA $0400,X
    INX
    BNE L_D1F4
    RTS
L_D1FE:
    PHA
    TXA
    PHA
    TYA
    PHA
    LDA $2002
    STA $26
    LDA #$00
    STA $2003
    LDA #$02
    STA $4014
    LDA $28
    BEQ L_D21E
    LDX #$00
    STX $28
    CMP #$07
    BCC L_D221
L_D21E:
    JMP L_D351
L_D221:
    ASL A
    TAX
    LDA $D244,X
    STA $06
    LDA $D245,X
    STA $07
    LDA $2002
    LDX $17
    LDY $16
    STX $2006
    STY $2006
    LDA $23
    AND #$04
    STA $2000
    JMP ($0006)
    .byte $51,$D3,$52,$D2,$5F,$D2,$90,$D2,$E5,$D2,$34,$D3,$44,$D3,$A6,$1A
    .byte $A5,$18,$8D,$07,$20,$CA,$D0,$FA,$4C,$51,$D3
    LDA $2002
    LDA #$3F
    STA $2006
    LDA #$00
    STA $2006
    LDX #$20
    LDY #$00
L_D270:
    LDA $0180,Y
    STA $2007
    INY
    DEX
    BNE L_D270
    LDA $2002
    LDA #$3F
    STA $2006
    LDA #$00
    STA $2006
    STA $2006
    STA $2006
    JMP L_D351
    LDA $23
    ORA #$04
    STA $2000
    LDX #$17
L_D299:
    LDA $0140,X
    STA $2007
    DEX
    BPL L_D299
    LDX $17
    STX $2006
    LDX $16
    INX
    STX $2006
    LDX #$17
L_D2AF:
    LDA $0158,X
    STA $2007
    DEX
    BPL L_D2AF
    LDX #$0A
L_D2BA:
    LDY $19
    STY $2006
    LDY $0170,X
    STY $2006
    LDA $2007
    LDA $2007
    AND $18
    ORA $0171,X
    LDY $19
    STY $2006
    LDY $0170,X
    STY $2006
    STA $2007
    DEX
    DEX
    BPL L_D2BA
    JMP L_D351
    TSX
    TXA
    LDX #$FF
    TXS
    TAX
    LDY #$04
L_D2ED:
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    PLA
    STA $2007
    DEY
    BNE L_D2ED
    TXS
    JMP L_D351
    LDX $1A
    LDY #$00
L_D338:
    LDA ($18),Y
    STA $2007
    INY
    DEX
    BNE L_D338
    JMP L_D351
    LDA $19
    STA $2007
    LDA $18
    STA $2007
    JMP L_D351
L_D351:
    JSR L_D41D
    LDA $2002
    JSR L_D36E
    LDA $36
    BEQ L_D360
    DEC $36
L_D360:
    JSR L_D408
    LDA $25
    STA $8000
    PLA
    TAY
    PLA
    TAX
    PLA
    RTI
L_D36E:
    LDA $24
    STA $2001
    LDA $23
    AND #$FE
    ORA $1D
    STA $23
    STA $2000
    LDX $1C
    LDY $1E
    STX $2005
    STY $2005
    LDA $29
    BEQ L_D3BE
    LDA $2002
    LDA $23
    AND #$FE
    LDX #$00
    LDY #$C4
    STA $2000
    STX $2005
    STY $2005
    LDA #$01
    STA $8000
    LDA #$16
    STA $8001
    LDA #$04
    STA $8000
    LDA #$3E
    STA $8001
    LDA #$05
    STA $8000
    LDA #$3F
    STA $8001
L_D3BE:
    JSR L_F89A
    LDA $29
    BNE L_D3C6
    RTS
L_D3C6:
    BIT $2002
    BVS L_D3C6
L_D3CB:
    BIT $2002
    BVS L_D3D5
    BIT $2002
    BVC L_D3CB
L_D3D5:
    LDX #$12
L_D3D7:
    DEX
    BNE L_D3D7
    LDA #$01
    STA $8000
    LDA $23
    LDX $1C
    LDY $1E
    STA $2000
    STX $2005
    STY $2005
    LDA $2B
    STA $8001
    LDA #$04
    STA $8000
    LDA $2E
    STA $8001
    LDA #$05
    STA $8000
    LDA $2F
    STA $8001
    RTS
L_D408:
    DEC $84
    BEQ L_D40D
    RTS
L_D40D:
    LDX #$07
L_D40F:
    LDA $85,X
    BEQ L_D415
    DEC $85,X
L_D415:
    DEX
    BPL L_D40F
    LDA #$3C
    STA $84
    RTS
L_D41D:
    LDX #$07
L_D41F:
    LDA $2A,X
    STX $8000
    STA $8001
    DEX
    BPL L_D41F
    RTS
L_D42B:
    LDA #$FF
    STA $E3
    LDA $EB
    BEQ L_D436
    JMP L_D641
L_D436:
    JSR L_D64F
    LDA $20
    AND #$10
    BEQ L_D442
    JMP L_E00F
L_D442:
    JSR L_D596
    LDA $46
    BEQ L_D44F
    DEC $46
    LDA #$00
    STA $20
L_D44F:
    LDA $40
    CMP #$04
    BNE L_D45B
    LDA $84
    AND #$07
    BEQ L_D45F
L_D45B:
    BIT $20
    BVS L_D465
L_D45F:
    LDA $FD
    AND #$0F
    STA $FD
L_D465:
    LDA $20
    AND #$0F
    BEQ L_D475
    STA $08
    LDA $FD
    AND #$F0
    ORA $08
    STA $FD
L_D475:
    LDA $20
    AND #$20
    BEQ L_D47E
    JMP L_D55A
L_D47E:
    LDA $20
    AND #$08
    BEQ L_D487
    JSR L_DCE2
L_D487:
    LDY #$01
L_D489:
    LDA a:$0087,Y
    BEQ L_D495
    INY
    CPY #$05
    BCC L_D489
    LDY #$06
L_D495:
    JSR L_CD2C
    LDA $4E
    BNE L_D4C2
    LDA $4F
    BNE L_D4A4
    LDA $20
    BPL L_D4AC
L_D4A4:
    JSR L_D4DF
    LDA #$00
    JMP L_D4B0
L_D4AC:
    LDA #$00
    STA $22
L_D4B0:
    STA $4F
    JSR L_D991
    BCC L_D4BF
    JSR L_DF90
    BCC L_D4BF
    JMP L_D54E
L_D4BF:
    JMP L_D536
L_D4C2:
    LSR A
    LSR A
    CLC
    ADC #$01
    STA $4B
    JSR L_D991
    BCS L_D4D1
    JMP L_D536
L_D4D1:
    LDA #$00
    STA $49
    STA $4A
    JSR L_D991
    BCC L_D536
    JMP L_D54E
L_D4DF:
    LDX $4F
    BNE L_D506
    LDA $22
    BEQ L_D4E8
    RTS
L_D4E8:
    LDA #$1B
    STA $8F
    LDA $5C
    STA $4F
    LDX $55
    LDA $51,X
    CMP #$06
    BNE L_D506
    JSR L_E7F0
    BCS L_D506
    LDA $4F
    LSR A
    LSR A
    CLC
    ADC $4F
    STA $4F
L_D506:
    PLA
    PLA
    LDA #$01
    STA $22
    LDA $4F
    DEC $4F
    LSR A
    LSR A
    EOR #$FF
    CLC
    ADC #$01
    STA $4B
    JSR L_D991
    BCS L_D521
    JMP L_D536
L_D521:
    LDA #$00
    STA $49
    STA $4A
    JSR L_D991
    BCC L_D536
    INC $4F
    JSR L_DF90
    BCC L_D536
    JMP L_D54E
L_D536:
    LDA $0E
    STA $43
    LDA $0F
    STA $44
    LDA $0A
    CMP #$EF
    BCC L_D546
    LDA #$00
L_D546:
    STA $45
    JSR L_DBDD
    JMP L_D8AF
L_D54E:
    LDA #$00
    STA $4F
    STA $4E
    JSR L_DBDD
    JMP L_D8AF
L_D55A:
    LDA #$10
    STA $8F
L_D55E:
    JSR L_CC09
    AND #$F0
    BNE L_D58F
    LDA $20
    AND #$03
    BEQ L_D55E
    ASL $20
    ASL $20
    LDY #$01
    JSR L_CD2C
    LDA $4B
    CLC
    ADC $55
    BMI L_D584
    CMP #$04
    BCC L_D586
    LDA #$00
    JMP L_D586
L_D584:
    LDA #$03
L_D586:
    STA $55
    LDA #$0C
    STA $8F
    JMP L_D55E
L_D58F:
    LDA #$10
    STA $8F
    JMP L_D8AF
L_D596:
    LDY $55
    LDX $51,Y
    CPX #$02
    BCS L_D5BC
    LDA $86,X
    BEQ L_D5A3
    RTS
L_D5A3:
    JSR L_E7F0
    BCC L_D5B7
    LDA $37
    BEQ L_D5B6
    BMI L_D5B6
    LDA #$FD
    STA $37
    LDA #$1A
    STA $8F
L_D5B6:
    RTS
L_D5B7:
    LDA #$02
    STA $86,X
    RTS
L_D5BC:
    CPX #$0B
    BNE L_D5D2
    LDA $59
    BEQ L_D5C5
    RTS
L_D5C5:
    LDX $55
    LDA #$FF
    STA $51,X
    JSR L_C234
    JSR L_D199
    RTS
L_D5D2:
    CPX #$0D
    BEQ L_D5D7
    RTS
L_D5D7:
    LDA $48
    CMP #$11
    BCC L_D5E2
    LDA #$03
    STA $55
    RTS
L_D5E2:
    LDX $55
    LDA #$FF
    STA $51,X
    JSR L_C234
    LDA #$12
    STA $8F
    JMP L_D866
    .byte $60
L_D5F3:
    LDY #$0C
    LDA ($77),Y
    STA $47
    INY
    LDA ($77),Y
    STA $48
    INY
    LDA ($77),Y
    STA $44
    SEC
    SBC #$08
    BCS L_D60A
    LDA #$00
L_D60A:
    CMP #$31
    BCC L_D610
    LDA #$30
L_D610:
    STA $7C
    LDA #$00
    STA $43
    STA $7B
    INY
    LDA ($77),Y
    STA $45
    JMP L_D895
L_D620:
    JSR L_D67D
    LDA #$11
    STA $48
    LDX $6E
    DEX
    STX $47
    LDA #$12
    STA $7C
    LDA #$10
    STA $45
    LDA #$1A
    STA $44
    LDA #$00
    STA $43
    STA $7B
    JMP L_D895
L_D641:
    LDA #$00
    STA $EB
    JSR L_D67D
    LDA #$3E
    STA $2E
    JMP L_D866
L_D64F:
    LDX $55
    LDA $51,X
    CMP #$0F
    BNE L_D675
    LDA $47
    CMP #$01
    BNE L_D675
    LDA $48
    CMP #$05
    BNE L_D675
    LDA $7C
    CMP #$10
    BNE L_D675
    LDA $7B
    CMP #$00
    BNE L_D675
    LDA $45
    CMP #$A0
    BEQ L_D676
L_D675:
    RTS
L_D676:
    LDA #$01
    STA $EC
    PLA
    PLA
    RTS
L_D67D:
    JSR L_C375
    LDA #$00
    STA $85
    JSR L_C1D8
    JSR L_C234
    LDA $7C
    CMP #$21
    BCC L_D692
    LDA #$20
L_D692:
    STA $7C
    JSR L_C76C
    LDA $7C
    CLC
    ADC #$10
    STA $7C
    JSR L_C76C
    LDA #$01
    STA $08
L_D6A5:
    LDX #$0C
L_D6A7:
    LDA $1C
    CLC
    ADC $08
    STA $1C
    BCC L_D6B6
    LDA $1D
    EOR #$01
    STA $1D
L_D6B6:
    LDA #$FF
    JSR L_CC8F
    DEX
    BNE L_D6A7
    INC $08
    LDX $08
    CPX #$20
    BCC L_D6A5
    LDA #$18
    STA $8F
    LDA #$FF
    STA $90
    LDX #$08
    JSR L_C540
    RTS
L_D6D4:
    LDA $45
    CMP #$10
    BCC L_D739
    CMP #$A1
    BCS L_D750
    LDX $48
    CPX #$10
    BEQ L_D731
    JSR L_DBDD
    LDA #$00
    STA $85
    LDA $56
    AND #$07
    STA $56
    LDA $44
    BEQ L_D714
    CMP #$3E
    BCC L_D731
    LDX $47
    INX
    CPX #$04
    BCS L_D731
    STX $47
    LDA #$40
    STA $57
    JSR L_C1D8
    LDA #$00
    STA $7C
    STA $43
    STA $44
    JMP L_D772
L_D714:
    LDX $47
    DEX
    BMI L_D731
    STX $47
    LDA #$00
    STA $57
    JSR L_C1D8
    LDA #$30
    STA $7C
    LDA #$3F
    STA $44
    LDA #$00
    STA $43
    JMP L_D772
L_D731:
    CLC
    RTS
L_D733:
    JMP L_D866
L_D736:
    JMP L_D883
L_D739:
    JSR L_DC87
    BCC L_D731
    LDX $48
    BEQ L_D733
    CPX #$10
    BEQ L_D731
    DEX
    STX $48
    LDA #$B0
    STA $45
    JMP L_D761
L_D750:
    LDX $48
    CPX #$10
    BEQ L_D736
    INX
    CPX #$10
    BCS L_D731
    STX $48
    LDA #$00
    STA $45
L_D761:
    JSR L_D08A
    JSR L_D07C
    JSR L_C8F2
    JSR L_C5CB
    JSR L_C569
    SEC
    RTS
L_D772:
    JSR L_D08A
    JSR L_D07C
    LDA #$00
    STA $7B
    JSR L_C8F2
    JSR L_C76C
    JSR L_C569
    LDA $44
    BNE L_D7F8
    LDA #$FC
    STA a:$001C
    LDA #$01
    STA a:$001D
    LDA #$F0
    STA $0213
    LDA #$F8
    STA $0217
    LDA #$0F
    STA $0A
L_D7A1:
    LDA #$03
    STA $0B
L_D7A5:
    BNE L_D7C3
    INC $0213
    INC $0217
    LDA $4E
    ORA $4F
    BNE L_D7C3
    LDA $0211
    EOR #$04
    STA $0211
    LDA $0215
    EOR #$04
    STA $0215
L_D7C3:
    LDA $0213
    SEC
    SBC #$04
    STA $0213
    CLC
    ADC #$08
    STA $0217
    LDA $1C
    CLC
    ADC #$04
    STA $1C
    LDA #$FF
    JSR L_CC8F
    DEC $0B
    BPL L_D7A5
    DEC $0A
    BPL L_D7A1
    LDA #$00
    STA $16
    LDA #$24
    STA $17
    LDA #$10
    STA $0C
    JSR L_C833
    JMP L_D864
L_D7F8:
    LDA #$01
    STA a:$001D
    LDA #$00
    STA a:$001C
    LDA #$00
    STA $0213
    LDA #$08
    STA $0217
    LDA #$0F
    STA $0A
L_D810:
    LDA #$03
    STA $0B
L_D814:
    BNE L_D832
    DEC $0213
    DEC $0217
    LDA $4E
    ORA $4F
    BNE L_D832
    LDA $0211
    EOR #$04
    STA $0211
    LDA $0215
    EOR #$04
    STA $0215
L_D832:
    LDA $0213
    CLC
    ADC #$04
    STA $0213
    CLC
    ADC #$08
    STA $0217
    LDA $1C
    SEC
    SBC #$04
    STA $1C
    LDA #$FF
    JSR L_CC8F
    DEC $0B
    BPL L_D814
    DEC $0A
    BPL L_D810
    LDA #$1E
    STA $16
    LDA #$20
    STA $17
    LDA #$2F
    STA $0C
    JSR L_C833
L_D864:
    SEC
    RTS
L_D866:
    LDA #$10
    STA $48
    LDA #$03
    STA $47
    LDA #$12
    STA $7C
    LDA #$B0
    STA $45
    LDA #$1A
    STA $44
    LDA #$00
    STA $43
    STA $7B
    JMP L_D895
L_D883:
    LDA #$00
    STA $48
    STA $47
    STA $7C
    STA $45
    STA $43
    STA $7B
    LDA #$01
    STA $44
L_D895:
    JSR L_C3E5
    JSR L_D08A
    JSR L_C8F2
    JSR L_C5CB
    JSR L_D07C
    JSR L_C1C7
    JSR L_C1D8
    JSR L_C492
    SEC
    RTS
L_D8AF:
    JSR L_D8E3
    JSR L_D94E
    RTS
L_D8B6:
    LDA $43
    STA $0E
    LDA $44
    STA $0F
    LDA $45
    STA $0A
    LDA $4B
    BEQ L_D8CB
    CLC
    ADC $0A
    STA $0A
L_D8CB:
    LDA $49
    BEQ L_D8E2
    CLC
    ADC $0E
    PHA
    AND #$0F
    STA $0E
    PLA
    ASL A
    ASL A
    ASL A
    ASL A
    LDA $0F
    ADC $4A
    STA $0F
L_D8E2:
    RTS
L_D8E3:
    LDX #$3D
    LDA $46
    BNE L_D92E
    LDX #$09
    LDA $50
    BNE L_D92E
    LDA $20
    AND #$BF
    CMP #$80
    BEQ L_D92E
    LDA $4B
    BEQ L_D913
    BMI L_D90C
    LDA $4E
    BNE L_D931
    LDA $20
    AND #$04
    BEQ L_D913
    LDX #$0D
    JMP L_D92E
L_D90C:
    LDA $4F
    BEQ L_D92E
    JMP L_D931
L_D913:
    LDX #$01
    LDY #$00
    LDA $4A
    BMI L_D921
    LDA $49
    BEQ L_D930
    LDY #$40
L_D921:
    STX $08
    LDA $56
    AND #$07
    ORA $08
    STA $56
    STY $57
    RTS
L_D92E:
    STX $56
L_D930:
    RTS
L_D931:
    LDX #$39
    LDY #$00
    LDA $4A
    ORA $49
    BMI L_D941
    BNE L_D93F
    LDX #$09
L_D93F:
    LDY #$40
L_D941:
    STX $08
    LDA $56
    AND #$03
    ORA $08
    STA $56
    STY $57
    RTS
L_D94E:
    LDA $46
    BNE L_D967
    LDA $56
    CMP #$20
    BCS L_D967
    LDA $56
    BIT $20
    BVS L_D963
    AND #$EF
    JMP L_D965
L_D963:
    ORA #$10
L_D965:
    STA $56
L_D967:
    LDA $20
    AND #$0F
    BEQ L_D990
    LDA $4F
    ORA $4E
    BNE L_D990
    INC $4D
    LDA $4D
    AND #$07
    BNE L_D990
    LDA $56
    AND #$08
    BNE L_D98A
    LDA $56
    EOR #$04
    STA $56
    JMP L_D990
L_D98A:
    LDA $57
    EOR #$40
    STA $57
L_D990:
    RTS
L_D991:
    LDA $4B
    PHA
    LDA $49
    PHA
L_D997:
    JSR L_D8B6
    JSR L_CF08
    BCC L_D9A7
    JSR L_D6D4
    BCC L_D9EB
    JMP L_DA13
L_D9A7:
    JSR L_DD42
    BCS L_D9EB
    JSR L_CE1A
    BCC L_DA14
    LDA $08
    CMP #$09
    BEQ L_D9EB
    BCC L_D9D1
    LDX $09
    LDA $0401,X
    CMP #$01
    BNE L_D9C8
    JSR L_DA31
    JMP L_DA14
L_D9C8:
    JSR L_DA86
    JSR L_CA36
    JMP L_DA13
L_D9D1:
    LDX $09
    LDA $0401,X
    CMP #$01
    BEQ L_D9E4
    CMP #$1A
    BCS L_D9E7
    JSR L_DAAA
    JMP L_DA13
L_D9E4:
    JSR L_DA1B
L_D9E7:
    CLC
    JMP L_DA14
L_D9EB:
    LDA $88
    BEQ L_DA02
    LDA $49
    BEQ L_DA02
    TAX
    AND #$08
    BNE L_D9FA
    DEX
    DEX
L_D9FA:
    INX
    TXA
    AND #$0F
    STA $49
    BNE L_D997
L_DA02:
    PLA
    PHA
    STA $49
    LDX $4B
    BEQ L_DA13
    BMI L_DA0E
    DEX
    DEX
L_DA0E:
    INX
    STX $4B
    BNE L_D997
L_DA13:
    SEC
L_DA14:
    PLA
    STA $49
    PLA
    STA $4B
    RTS
L_DA1B:
    LDA $2D
    CMP #$30
    BCS L_DA30
    LDA $87
    BEQ L_DA30
    LDA $59
    BEQ L_DA30
    LDX $09
    LDA #$80
    STA $0401,X
L_DA30:
    RTS
L_DA31:
    JSR L_E86F
    BCC L_DA3C
    LDA #$06
    STA $8F
    CLC
    RTS
L_DA3C:
    LDY #$0A
    LDA ($77),Y
    CMP #$08
    BCS L_DA49
    LDY #$00
    STY $04A2
L_DA49:
    PHA
    CLC
    ADC #$02
    STA $04A1
    PLA
    ASL A
    ASL A
    CLC
    ADC #$81
    STA $04A0
    LDA #$1F
    STA $8F
    JSR L_C2B1
    LDA $85
    PHA
    LDA #$00
    STA $85
    JSR L_C1D8
    LDA $8E
    PHA
    LDA #$0E
    STA $8E
    JSR L_FC08
    LDA #$78
    STA $36
    JSR L_C135
    PLA
    STA $8E
    JSR L_FC08
    PLA
    STA $85
    SEC
    RTS
L_DA86:
    SEC
    SBC #$02
    PHA
    LDA #$00
    STA $04A1
    PLA
    CMP #$18
    BCC L_DA97
    JMP L_DB01
L_DA97:
    CMP #$08
    BCS L_DAD2
    ASL A
    TAX
    LDA $DB16,X
    STA $0C
    LDA $DB17,X
    STA $0D
    JMP ($000C)
L_DAAA:
    SEC
    SBC #$02
    CMP #$18
    BCC L_DAB2
    RTS
L_DAB2:
    PHA
    LDA #$00
    STA $0401,X
    LDA #$F0
    STA $0406,X
    LDA $08
    ASL A
    ASL A
    ASL A
    ORA #$80
    TAX
    LDA #$EF
    STA $0200,X
    STA $0204,X
    PLA
    CMP #$08
    BCC L_DAF2
L_DAD2:
    SBC #$08
    TAX
    LDA $60,X
    CMP #$0B
    BCS L_DAEC
    INC $60,X
    LDA #$13
    STA $8F
    CPX #$0E
    BEQ L_DAE6
    RTS
L_DAE6:
    JSR L_CA36
    JMP L_D620
L_DAEC:
    LDA #$1D
    STA a:$008F
    RTS
L_DAF2:
    ASL A
    TAX
    LDA $DB06,X
    STA $0C
    LDA $DB07,X
    STA $0D
    JMP ($000C)
L_DB01:
    LDA #$06
    STA $8F
    RTS
    .byte $26,$DB,$31,$DB,$3C,$DB,$52,$DB,$5D,$DB,$71,$DB,$B7,$DB,$85,$DB
    .byte $6A,$D1,$99,$D1,$47,$DB,$52,$DB,$66,$DB,$7B,$DB,$B7,$DB,$9B,$DB
    .byte $A9,$1E,$8D,$8F,$00,$A9,$05,$20,$00,$E8,$60,$A9,$11,$8D,$8F,$00
    .byte $A9,$05,$20,$16,$E8,$60,$A9,$11,$8D,$8F,$00,$A9,$02,$20,$2C,$E8
    .byte $60,$A9,$11,$8D,$8F,$00,$A9,$32,$20,$2C,$E8,$60,$A9,$1D,$8D,$8F
    .byte $00,$A9,$05,$20,$DB,$E7,$60,$A9,$15,$8D,$8F,$00,$20,$52,$E8,$60
    .byte $A9,$15,$8D,$8F,$00,$A9,$14,$20,$59,$E8,$60,$A9,$13,$8D,$8F,$00
    .byte $A9,$0A,$85,$85,$60,$A9,$13,$8D,$8F,$00,$A9,$1E,$85,$85,$60,$A9
    .byte $13,$8D,$8F,$00,$A2,$1E,$A5,$88,$F0,$08,$A5,$89,$F0,$02,$86,$8A
    .byte $86,$89,$86,$88,$60,$A9,$13,$8D,$8F,$00,$A2,$3C,$A5,$88,$F0,$0E
    .byte $A5,$89,$F0,$08,$A5,$8A,$F0,$02,$86,$8B,$86,$8A,$86,$89,$86,$88
    .byte $60,$A2,$09,$A0,$00,$B9,$01,$04,$C9,$01,$D0,$05,$A9,$80,$99,$01
    .byte $04,$98,$18,$69,$10,$A8,$CA,$D0,$EC,$A9,$18,$85,$8F,$A9,$FF,$85
    .byte $90,$A2,$02,$20,$40,$C5,$60
L_DBDD:
    LDA $86
    BNE L_DBE5
    LDA $4F
    BEQ L_DBEC
L_DBE5:
    LDA #$00
    STA $50
    JMP L_DC82
L_DBEC:
    LDA $44
    STA $0C
    STA $0F
    LDA $43
    STA $0E
    LDX $45
    STX $0D
    INX
    STX $0A
    JSR L_CA54
    LDA $43
    BNE L_DC10
    LDA #$01
    STA $50
    LDY #$00
    LDA ($0C),Y
    AND #$3F
    BEQ L_DC4D
L_DC10:
    LDA #$00
    STA $50
    LDA $45
    CMP #$B0
    BCS L_DC4A
    JSR L_CDB2
    BCC L_DC38
    LDA $2D
    CMP #$30
    BCS L_DC4D
    LDY $55
    LDX $51,Y
    CPX #$05
    BNE L_DC4D
    LDA $4E
    BEQ L_DC4D
    LDX $09
    LDA #$80
    STA $0401,X
L_DC38:
    LDY #$01
    JSR L_DCCC
    BCS L_DC4D
    LDA $43
    BEQ L_DC4A
    LDY #$0D
    JSR L_DCCC
    BCS L_DC4D
L_DC4A:
    INC $4E
    RTS
L_DC4D:
    LDA $4E
    CMP $5C
    BCC L_DC6E
    SEC
    SBC #$07
    CMP $5C
    BCC L_DC5C
    LDA $5C
L_DC5C:
    SEC
    SBC #$01
    STA $4F
    CLC
    ADC #$0A
    STA $46
    LDA #$0A
    STA a:$008F
    JSR L_E7CE
L_DC6E:
    LDA $4E
    BNE L_DC82
    LDY #$01
    JSR L_DCA8
    BCS L_DC82
    LDA $43
    BEQ L_DC82
    LDY #$0D
    JSR L_DCA8
L_DC82:
    LDA #$00
    STA $4E
    RTS
L_DC87:
    LDA $86
    ORA $4F
    BNE L_DCA6
    LDA $0E
    BNE L_DCA4
    LDA $0F
    STA $0C
    LDA #$00
    STA $0D
    JSR L_CA54
    LDY #$00
    LDA ($0C),Y
    AND #$3F
    BEQ L_DCA6
L_DCA4:
    CLC
    RTS
L_DCA6:
    SEC
    RTS
L_DCA8:
    LDA ($0C),Y
    AND #$3F
    CMP #$30
    BNE L_DCCA
    LDA $4F
    BNE L_DCB8
    LDA #$0A
    STA $4F
L_DCB8:
    LDA $85
    BNE L_DCC8
    JSR L_E7CE
    LDA #$0A
    STA a:$008F
    LDA #$01
    STA $85
L_DCC8:
    SEC
    RTS
L_DCCA:
    CLC
    RTS
L_DCCC:
    LDA ($0C),Y
    AND #$3F
    TAX
    BEQ L_DCDA
    CPX #$02
    BEQ L_DCE0
    CPX #$30
    RTS
L_DCDA:
    LDA $43
    BEQ L_DCE0
    CLC
    RTS
L_DCE0:
    SEC
    RTS
L_DCE2:
    LDX $45
    BEQ L_DD18
    DEX
    STX $0D
    LDX $44
    STX $0C
    JSR L_CA54
    LDY #$00
    LDA ($0C),Y
    AND #$3F
    CMP #$05
    BEQ L_DD19
    CMP #$04
    BEQ L_DD1E
    CMP #$03
    BEQ L_DD23
    LDA $43
    BEQ L_DD18
    LDY #$0C
    LDA ($0C),Y
    AND #$3F
    CMP #$05
    BEQ L_DD19
    CMP #$04
    BEQ L_DD1E
    CMP #$03
    BEQ L_DD23
L_DD18:
    RTS
L_DD19:
    PLA
    PLA
    JMP L_E077
L_DD1E:
    PLA
    PLA
    JMP L_E424
L_DD23:
    LDX $55
    LDA $51,X
    CMP #$0E
    BNE L_DD18
    LDX #$02
    LDY $6E
    LDA #$0E
L_DD31:
    CMP $51,X
    BNE L_DD36
    INY
L_DD36:
    DEX
    BPL L_DD31
    CPY #$04
    BNE L_DD18
    PLA
    PLA
    JMP L_D5F3
L_DD42:
    LDA #$90
    STA $E5
    LDA #$04
    STA $E6
    LDA $0E
    PHA
    LDA $0F
    PHA
    LDA $0A
    PHA
    LDA $0F
    STA $0C
    LDA $0A
    STA $0D
    JSR L_CA54
    LDY #$00
    JSR L_DD97
    BCS L_DD8D
    LDA $0E
    BEQ L_DD70
    LDY #$0C
    JSR L_DD97
    BCS L_DD8D
L_DD70:
    LDA $0A
    CMP #$B0
    BCS L_DD8C
    AND #$0F
    BEQ L_DD8C
    LDY #$01
    JSR L_DD97
    BCS L_DD8D
    LDA $0E
    BEQ L_DD8C
    LDY #$0D
    JSR L_DD97
    BCS L_DD8D
L_DD8C:
    CLC
L_DD8D:
    PLA
    STA $0A
    PLA
    STA $0F
    PLA
    STA $0E
    RTS
L_DD97:
    LDA ($0C),Y
    AND #$3F
    CMP $70
    BNE L_DDA2
    JMP L_DDB3
L_DDA2:
    CMP #$02
    BNE L_DDA9
    JMP L_DDE0
L_DDA9:
    CMP #$3E
    BNE L_DDB0
    JMP L_DE1A
L_DDB0:
    CMP #$30
    RTS
L_DDB3:
    LDA $0491
    BNE L_DDD9
    STY $0B
    LDA #$E1
    STA $ED
    LDA #$01
    STA $EE
    LDA #$01
    STA $EF
    LDA $71
    STA $F0
    LDA #$0A
    STA $F3
    JSR L_DF37
    JSR L_E99A
    LDA #$06
    STA a:$008F
L_DDD9:
    LDA $71
    AND #$3F
    CMP #$30
    RTS
L_DDE0:
    LDA $0491
    BNE L_DE18
    STY $0B
    LDX $55
    LDA $51,X
    CMP #$07
    BNE L_DDF4
    JSR L_E7F0
    BCC L_DDF9
L_DDF4:
    JSR L_E86F
    BCS L_DE18
L_DDF9:
    LDA #$E1
    STA $ED
    LDA #$01
    STA $EE
    LDA #$01
    STA $EF
    LDA $74
    STA $F0
    LDA #$0F
    STA $F3
    JSR L_DF37
    JSR L_E99A
    LDA #$06
    STA a:$008F
L_DE18:
    SEC
    RTS
L_DE1A:
    BIT $20
    BPL L_DE37
    LDA $0491
    BNE L_DE37
    STY $0B
    LDA #$01
    STA $F4
    LDY $55
    LDX $51,Y
    DEX
    DEX
    BEQ L_DE3F
    DEX
    BEQ L_DE39
    DEX
    BEQ L_DE3C
L_DE37:
    SEC
    RTS
L_DE39:
    JMP L_DE9F
L_DE3C:
    JMP L_DEE8
L_DE3F:
    LDA $59
    BEQ L_DE9D
    LDA $45
    AND #$0F
    ORA $43
    BNE L_DE9D
    LDA $FD
    AND #$0F
    ASL A
    TAX
    CLC
    LDA $44
    ADC $FEAB,X
    STA $049D
    STA $0C
    LDA #$00
    STA $049C
    CLC
    LDA $45
    ADC $FEAC,X
    STA $049E
    STA $0D
    JSR L_CA54
    LDY #$00
    STY $0B
    LDA ($0C),Y
    AND #$3F
    CMP #$3E
    BNE L_DE9D
    LDA #$E1
    STA $0490
    LDA #$01
    STA $0491
    LDA #$01
    STA $0492
    LDA #$0F
    STA $0496
    JSR L_DF80
    STA $0493
    JSR L_E7F0
    LDA #$14
    STA a:$008F
L_DE9D:
    SEC
    RTS
L_DE9F:
    LDA $FD
    AND #$0F
    BEQ L_DEE0
    LDY #$01
    JSR L_CD70
    LDY #$F8
    LDA ($79),Y
    AND #$FE
    STA $ED
    LDA #$01
    STA $EE
    LDA #$03
    STA $EF
    LDY $0B
    LDA ($0C),Y
    STA $F0
    LDA #$10
    STA $F3
    JSR L_DF80
    STA ($0C),Y
    JSR L_DF37
    JSR L_DF5E
    JSR L_F7F7
    LDA #$FF
    STA $E3
    LDA $0491
    BEQ L_DEE0
    LDA #$06
    STA a:$008F
L_DEE0:
    LDA #$00
    STA $4B
    STA $4E
    SEC
    RTS
L_DEE8:
    LDA $59
    BEQ L_DE9D
    LDA $FD
    AND #$0F
    BEQ L_DF2F
    LDY #$08
    JSR L_CD70
    LDY #$F8
    LDA ($79),Y
    AND #$FE
    STA $ED
    LDA #$01
    STA $EE
    LDA #$03
    STA $EF
    LDY $0B
    LDA ($0C),Y
    STA $F0
    LDA #$00
    STA $F3
    JSR L_DF80
    STA ($0C),Y
    JSR L_DF37
    JSR L_DF5E
    JSR L_F7F7
    LDA #$FF
    STA $E3
    LDA $EE
    BEQ L_DF2F
    LDA #$14
    STA a:$008F
    JSR L_E7F0
L_DF2F:
    LDA #$00
    STA $4B
    STA $4E
    SEC
    RTS
L_DF37:
    LDA $0B
    CMP #$0C
    BCC L_DF41
    SBC #$0C
    INC $0F
L_DF41:
    TAY
    BEQ L_DF4B
    LDA $0A
    CLC
    ADC #$10
    STA $0A
L_DF4B:
    LDA $0A
    AND #$F0
    STA $FB
    LDA #$00
    STA $FC
    LDA $0F
    STA $FA
    LDA #$00
    STA $F9
    RTS
L_DF5E:
    LDA $FA
    STA $0C
    ASL A
    AND #$1F
    STA $16
    LDA $FA
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    JSR L_C833
    RTS
L_DF80:
    LDY $0B
    LDA ($10),Y
    AND #$3F
    TAX
    LDA $74
    CPX #$3E
    BEQ L_DF8F
    LDA ($10),Y
L_DF8F:
    RTS
L_DF90:
    LDA $49
    PHP
    LDA #$00
    STA $49
    STA $4A
    PLP
    BEQ L_DFCF
    LDA $45
    AND #$0F
    BEQ L_E00D
    CMP #$06
    BCC L_DFBE
    CMP #$0B
    BCS L_DFAD
    JMP L_E00D
L_DFAD:
    LDA $20
    AND #$08
    BNE L_E00D
    LDA #$01
    STA $4B
    LDA #$00
    STA $4C
    JMP L_E009
L_DFBE:
    LDA $20
    AND #$04
    BNE L_E00D
    LDA #$FF
    STA $4B
    LDA #$FF
    STA $4C
    JMP L_E009
L_DFCF:
    LDA $4B
    PHP
    LDA #$00
    STA $4B
    STA $4C
    PLP
    BEQ L_E00D
    LDA $43
    BEQ L_E00D
    CMP #$06
    BCC L_DFFB
    CMP #$0B
    BCS L_DFEA
    JMP L_E00D
L_DFEA:
    LDA $20
    AND #$02
    BNE L_E00D
    LDA #$01
    STA $49
    LDA #$00
    STA $4A
    JMP L_E009
L_DFFB:
    LDA $20
    AND #$01
    BNE L_E00D
    LDA #$0F
    STA $49
    LDA #$FF
    STA $4A
L_E009:
    JSR L_D991
    RTS
L_E00D:
    SEC
    RTS
L_E00F:
    LDA #$03
    STA $8F
    INC $8D
    LDA $2D
    CMP #$30
    BCS L_E039
    JSR L_E620
    LDA #$08
    JSR L_E660
    JSR L_E6B7
    JSR L_CF30
    JSR L_CF82
    LDA #$08
    STA $7B
    JSR L_C1C7
    JSR L_C1D8
    JSR L_C492
L_E039:
    JSR L_CC43
    BNE L_E039
L_E03E:
    JSR L_CC43
    AND #$10
    BEQ L_E03E
L_E045:
    JSR L_CC43
    BNE L_E045
    LDA #$04
    STA $8F
    LDA $2D
    CMP #$30
    BCS L_E074
    JSR L_E642
    JSR L_C3E5
    JSR L_E79D
    LDA $FE
    JSR L_D02E
    JSR L_C8FF
    JSR L_C5CB
    JSR L_C1D8
    JSR L_C2B1
    JSR L_C1C7
    JSR L_C492
L_E074:
    DEC $8D
    RTS
L_E077:
    LDA $48
    CMP #$10
    BNE L_E080
    JMP L_E0F4
L_E080:
    JSR L_E620
    LDA #$04
    JSR L_E660
    JSR L_E778
    JSR L_C492
L_E08E:
    JSR L_E514
    BCC L_E096
    JMP L_E5FD
L_E096:
    LDA $5A
    CMP #$0A
    BCS L_E0A3
    LDA #$06
    STA $8F
    JMP L_E08E
L_E0A3:
    LDX #$0A
L_E0A5:
    TXA
    PHA
    DEC $5A
    JSR L_CAF8
    LDA #$0C
    STA $8F
    LDA #$0A
    STA $36
    JSR L_C135
    PLA
    TAX
    DEX
    BNE L_E0A5
    JSR L_C430
    JSR L_D16A
    JSR L_D199
    LDA #$08
    JSR L_E667
    JSR L_E6B7
    JSR L_CF30
    JSR L_CF82
    LDA #$08
    STA $7B
    JSR L_C1C7
    JSR L_C1D8
    JSR L_C492
    JSR L_E4AA
    LDA #$04
    JSR L_E667
    JSR L_E79D
    JSR L_E778
    JSR L_C492
    JMP L_E08E
L_E0F4:
    LDA #$00
    STA $58
    STA $59
    LDA $40
    CMP #$06
    BCS L_E112
    LDY #$02
L_E102:
    LDX $51,Y
    BMI L_E108
    INC $60,X
L_E108:
    LDX #$FF
    STX $51,Y
    DEY
    BPL L_E102
    JSR L_D0A5
L_E112:
    JSR L_E620
    LDA #$06
    STA $40
    LDA #$06
    JSR L_E660
    JSR L_CAB6
    JSR L_CACC
    LDA #$03
    STA $55
    JSR L_C234
    LDA #$F1
    STA $56
    LDA #$00
    STA $57
    JSR L_C1D8
    JSR L_E7B2
    JSR L_D08A
    JSR L_C492
L_E13F:
    JSR L_E5B4
    LDA $0A
    AND #$F0
    CMP #$50
    BNE L_E186
    LDA $0F
    AND #$0F
    CMP #$05
    BNE L_E13F
    LDA $37
    BEQ L_E13F
    LDX $8E
    INX
    CPX #$10
    BCC L_E15F
    LDX #$00
L_E15F:
    STX $8E
    JSR L_FC08
    LDA $37
    BPL L_E13F
    LDA $20
    CMP #$C3
    BNE L_E13F
    LDX #$0D
    LDA #$10
L_E172:
    STA $60,X
    DEX
    BPL L_E172
    LDA #$80
    STA $37
    STA $5A
    STA $5B
    LDA #$1A
    STA $8F
    JMP L_E13F
L_E186:
    LDX #$00
    CMP #$70
    BEQ L_E1A8
    LDX #$02
    CMP #$80
    BEQ L_E1B8
    CMP #$90
    BNE L_E13F
    LDX #$03
    LDA $0F
    AND #$0F
    CMP #$06
    BEQ L_E1DC
    INX
    CMP #$0A
    BEQ L_E1DC
    JMP L_E13F
L_E1A8:
    LDA $0F
    AND #$0F
    CMP #$06
    BEQ L_E1DC
    INX
    CMP #$08
    BEQ L_E1DC
    JMP L_E13F
L_E1B8:
    LDA $0F
    AND #$0F
    CMP #$04
    BEQ L_E1DC
    CMP #$0A
    BNE L_E1CE
    LDA #$03
    STA $8F
    JSR L_E27D
    JMP L_E13F
L_E1CE:
    CMP #$0C
    BNE L_E1D9
    LDA #$03
    STA $8F
    JSR L_E2AA
L_E1D9:
    JMP L_E13F
L_E1DC:
    STX $40
    TXA
    ASL A
    ASL A
    CLC
    ADC #$03
    TAY
    LDX #$03
L_E1E7:
    LDA $FFA7,Y
    STA $5C,X
    DEY
    DEX
    BPL L_E1E7
    LDA #$18
    STA $8F
    LDA #$FF
    STA $90
    LDA #$04
    STA $36
    JSR L_C135
    LDX #$05
    JSR L_C540
    LDA $40
    CLC
    ADC #$38
    STA $2C
    LDA #$3D
    STA $2D
    LDA #$3E
    STA $2E
    LDA #$3F
    STA $2F
    LDA #$0D
    STA $56
    LDA #$00
    STA $57
    LDA $45
    AND #$F0
    STA $45
    LDA #$04
    STA $43
    JSR L_D07C
    JSR L_C1D8
    JSR L_C135
    LDX #$05
    JSR L_C540
    LDA #$78
    STA $36
    JSR L_C135
    JSR L_C3E5
    LDA #$08
    STA $56
    LDA #$00
    STA $57
    LDA #$63
    STA $58
    STA $59
    JSR L_CAB6
    JSR L_CACC
    LDA #$02
    STA $55
    JSR L_C234
    LDA #$08
    JSR L_E660
    JSR L_E6B7
    JSR L_CF30
    JSR L_CF82
    LDA #$08
    STA $7B
    JSR L_C1C7
    JSR L_C1D8
    JSR L_C492
    JSR L_E4AA
    JMP L_E5FD
L_E27D:
    LDA #$10
    STA $7C
    JSR L_C7B5
    JSR L_C1C7
    LDA #$D4
    STA $0E
    LDA #$B4
    STA $0F
    JSR L_CC9C
    JSR L_D0E5
L_E295:
    JSR L_CC43
    BNE L_E295
L_E29A:
    JSR L_CC43
    BEQ L_E29A
    LDA #$20
    STA $7C
    JSR L_C7B5
    JSR L_C1C7
    RTS
L_E2AA:
    LDA #$30
    STA $7C
    JSR L_C7B5
    JSR L_D15F
    JSR L_D0E5
    JSR L_C1C7
L_E2BA:
    JSR L_CC43
    BNE L_E2BA
    LDA #$00
    STA $F9
    STA $F5
    STA $F7
    LDA #$F5
    STA $0281
    STA $0291
    LDA #$F7
    STA $0285
    STA $0295
    LDA #$00
    STA $0282
    STA $0286
    STA $0292
    STA $0296
    JSR L_E3D6
    JSR L_E400
L_E2EB:
    LDA #$01
    STA $36
    JSR L_CC43
    BIT $20
    BMI L_E32D
    BVS L_E333
    LDA $20
    LSR A
    BCS L_E31B
    LSR A
    BCS L_E321
    LSR A
    BCS L_E315
    LSR A
    BCS L_E327
    LSR A
    BCS L_E30F
    LSR A
    BCS L_E364
    JMP L_E333
L_E30F:
    JSR L_E347
    JMP L_E333
L_E315:
    JSR L_E3C7
    JMP L_E333
L_E31B:
    JSR L_E39E
    JMP L_E333
L_E321:
    JSR L_E3AD
    JMP L_E333
L_E327:
    JSR L_E3BA
    JMP L_E330
L_E32D:
    JSR L_E372
L_E330:
    JSR L_D0E5
L_E333:
    LDA $20
    AND #$CF
    BEQ L_E341
    LDA #$0C
    STA $8F
    LDA #$0A
    STA $36
L_E341:
    JSR L_C135
    JMP L_E2EB
L_E347:
    LDA #$77
    STA $0E
    LDA #$B5
    STA $0F
    JSR L_CC9C
    BCC L_E355
    RTS
L_E355:
    LDA #$10
    STA $8F
    JSR L_D0C5
    JSR L_CAE2
    JSR L_CAF8
    PLA
    PLA
L_E364:
    LDA #$20
    STA $7C
    JSR L_C7B5
    JSR L_C1C7
    JSR L_E7B2
    RTS
L_E372:
    LDA $F5
    ASL A
    ASL A
    ADC $F5
    ADC $F7
    CMP #$20
    BEQ L_E392
    CMP #$21
    BEQ L_E398
    CMP #$22
    BEQ L_E347
    PHA
    JSR L_E41E
    PLA
    STA $0322,X
    CPX #$1F
    BEQ L_E347
L_E392:
    INC $F9
    JSR L_E3D6
    RTS
L_E398:
    DEC $F9
    JSR L_E3D6
    RTS
L_E39E:
    LDX $F5
    INX
    CPX #$07
    BCC L_E3A7
    LDX #$00
L_E3A7:
    STX $F5
    JSR L_E400
    RTS
L_E3AD:
    LDX $F5
    DEX
    BPL L_E3B4
    LDX #$06
L_E3B4:
    STX $F5
    JSR L_E400
    RTS
L_E3BA:
    LDX $F7
    DEX
    BPL L_E3C1
    LDX #$04
L_E3C1:
    STX $F7
    JSR L_E400
    RTS
L_E3C7:
    LDX $F7
    INX
    CPX #$05
    BCC L_E3D0
    LDX #$00
L_E3D0:
    STX $F7
    JSR L_E400
    RTS
L_E3D6:
    LDX #$61
    LDA $F9
    AND #$1F
    CMP #$10
    BCC L_E3E4
    SBC #$10
    LDX #$69
L_E3E4:
    STX $0280
    STX $0284
    STA $08
    LSR A
    LSR A
    CLC
    ADC $08
    ASL A
    ASL A
    ASL A
    ADC #$36
    STA $0287
    SEC
    SBC #$08
    STA $0283
    RTS
L_E400:
    LDA $F5
    ASL A
    ASL A
    ASL A
    ADC #$36
    STA $0297
    SEC
    SBC #$08
    STA $0293
    LDA $F7
    ASL A
    ASL A
    ASL A
    ADC #$81
    STA $0290
    STA $0294
    RTS
L_E41E:
    LDA $F9
    AND #$1F
    TAX
    RTS
L_E424:
    JSR L_E620
    LDA $80
    PHA
    LDA $81
    PHA
    LDA $82
    PHA
    LDA $83
    PHA
    LDA $47
    JSR L_E660
    PLA
    STA $83
    PLA
    STA $82
    PLA
    STA $81
    PLA
    STA $80
    JSR L_E6FF
    JSR L_CFBC
    JSR L_E778
    JSR L_C492
L_E450:
    JSR L_E514
    BCS L_E4A7
    LDX #$00
    LDA $44
    AND #$0F
    CMP #$03
    BCC L_E450
    CMP #$05
    BCC L_E46D
    LDX #$02
    CMP #$0A
    BCC L_E450
    CMP #$0C
    BCS L_E450
L_E46D:
    LDA $80,X
    BMI L_E489
    PHA
    TXA
    PHA
    LDA $81,X
    JSR L_E842
    BCS L_E48E
    PLA
    PLA
    CMP #$0D
    BNE L_E489
    LDA $37
    BEQ L_E489
    LDA #$01
    STA $61
L_E489:
    LDA #$06
    JMP L_E49D
L_E48E:
    PLA
    TAX
    LDA #$FF
    STA $80,X
    JSR L_E6FF
    PLA
    TAX
    INC $60,X
    LDA #$10
L_E49D:
    STA $8F
L_E49F:
    JSR L_CC43
    BNE L_E49F
    JMP L_E450
L_E4A7:
    JMP L_E5FD
L_E4AA:
    JSR L_E562
    BCS L_E504
    LDX #$FF
    LDA $45
    CMP #$58
    BCS L_E4DD
    LDX #$00
    CMP #$38
    BCC L_E4BF
    LDX #$08
L_E4BF:
    STX $08
    LDA $44
    LSR A
    ORA $08
    TAX
    LDA $60,X
    BEQ L_E4D4
    TXA
    PHA
    JSR L_D017
    PLA
    TAX
    BCS L_E4DB
L_E4D4:
    LDA #$06
    STA $8F
    JMP L_E4AA
L_E4DB:
    DEC $60,X
L_E4DD:
    STX $08
    LDX $51
    BMI L_E4E5
    INC $60,X
L_E4E5:
    LDA $52
    STA $51
    LDA $53
    STA $52
    LDA $08
    STA $53
    LDA #$12
    STA $8F
    JSR L_E6B7
    JSR L_C234
    JSR L_CF30
    JSR L_CF82
    JMP L_E4AA
L_E504:
    LDX $55
    LDA $51,X
    CMP #$0D
    BNE L_E513
    LDA #$03
    STA $55
    JSR L_C234
L_E513:
    RTS
L_E514:
    LDA #$01
    STA $36
    JSR L_CC43
    LDA $20
    AND #$80
    BNE L_E55E
    LDA $20
    AND #$0F
    LDY #$01
    JSR L_CD2C
    JSR L_D8B6
    LDA $0A
    CMP #$8C
    BCC L_E54F
    CMP #$A1
    BCS L_E560
    LDA $0F
    AND #$0F
    CMP #$02
    BCC L_E54F
    CMP #$0D
    BCS L_E54F
    LDA $0E
    STA $43
    LDA $0F
    STA $44
    LDA $0A
    STA $45
L_E54F:
    JSR L_D8E3
    JSR L_D94E
    JSR L_C1D8
    JSR L_C135
    JMP L_E514
L_E55E:
    CLC
    RTS
L_E560:
    SEC
    RTS
L_E562:
    LDA #$01
    STA $36
    JSR L_CC43
    LDA $20
    AND #$80
    BNE L_E5B0
    LDA $20
    AND #$0F
    LDY #$01
    JSR L_CD2C
    JSR L_D8B6
    LDA $0A
    CMP #$20
    BCC L_E5A1
    CMP #$A1
    BCS L_E5B2
    LDA $0F
    AND #$0F
    CMP #$01
    BCC L_E5A1
    CMP #$0F
    BCC L_E595
    LDA $0E
    BNE L_E5A1
L_E595:
    LDA $0E
    STA $43
    LDA $0F
    STA $44
    LDA $0A
    STA $45
L_E5A1:
    JSR L_D8E3
    JSR L_D94E
    JSR L_C1D8
    JSR L_C135
    JMP L_E562
L_E5B0:
    CLC
    RTS
L_E5B2:
    SEC
    RTS
L_E5B4:
    LDA #$01
    STA $36
    JSR L_CC43
    LDA $20
    AND #$80
    BNE L_E5FC
    LDA $20
    AND #$0F
    LDY #$01
    JSR L_CD2C
    JSR L_D8B6
    LDA $0A
    CMP #$30
    BCC L_E5F3
    CMP #$A1
    BCS L_E5F3
    LDA $0F
    AND #$0F
    CMP #$02
    BCC L_E5F3
    CMP #$0D
    BCC L_E5E7
    LDA $0E
    BNE L_E5F3
L_E5E7:
    LDA $0E
    STA $43
    LDA $0F
    STA $44
    LDA $0A
    STA $45
L_E5F3:
    JSR L_C1D8
    JSR L_C135
    JMP L_E5B4
L_E5FC:
    RTS
L_E5FD:
    JSR L_E642
    JSR L_C3E5
    JSR L_E79D
    LDA $FE
    JSR L_D02E
    JSR L_C8FF
    JSR L_C5CB
    JSR L_C1D8
    JSR L_C2B1
    JSR L_C1C7
    JSR L_C492
    JMP L_D8AF
L_E620:
    PLA
    TAX
    PLA
    TAY
    LDA $8E
    STA $FE
    LDA $43
    PHA
    LDA $44
    PHA
    LDA $45
    PHA
    LDA $7B
    PHA
    LDA $7C
    PHA
    LDA $47
    PHA
    LDA $48
    PHA
    TYA
    PHA
    TXA
    PHA
    RTS
L_E642:
    PLA
    TAX
    PLA
    TAY
    PLA
    STA $48
    PLA
    STA $47
    PLA
    STA $7C
    PLA
    STA $7B
    PLA
    STA $45
    PLA
    STA $44
    PLA
    STA $43
    TYA
    PHA
    TXA
    PHA
    RTS
L_E660:
    PHA
    JSR L_C3E5
    JMP L_E66B
L_E667:
    PHA
    JSR L_C430
L_E66B:
    PLA
    PHA
    STA $08
    AND #$0C
    LSR A
    LSR A
    STA $47
    LDA $08
    AND #$03
    ASL A
    ASL A
    ASL A
    ASL A
    STA $7C
    CLC
    ADC #$07
    STA $44
    LDA #$10
    STA $48
    LDA #$08
    STA $43
    LDA #$A0
    STA $45
    LDA #$00
    STA $4F
    STA $4E
    STA $7B
    JSR L_D07C
    JSR L_C8FF
    PLA
    CMP #$04
    BNE L_E6AA
    LDA #$1F
    CLC
    ADC #$A0
    STA $7A
L_E6AA:
    JSR L_C5DC
    JSR L_D8E3
    JSR L_C1D8
    JSR L_C1C7
    RTS
L_E6B7:
    LDA #$58
    STA $08
    LDX #$02
    LDY #$10
L_E6BF:
    LDA $51,X
    BMI L_E6D6
    ASL A
    ASL A
    CLC
    ADC #$A1
    STA $0241,Y
    CLC
    ADC #$02
    STA $0245,Y
    LDA #$BB
    JMP L_E6D8
L_E6D6:
    LDA #$EF
L_E6D8:
    STA $0240,Y
    STA $0244,Y
    LDA $08
    STA $0243,Y
    CLC
    ADC #$08
    STA $0247,Y
    SEC
    SBC #$28
    STA $08
    LDA #$01
    STA $0242,Y
    STA $0246,Y
    TYA
    SEC
    SBC #$08
    TAY
    DEX
    BPL L_E6BF
    RTS
L_E6FF:
    LDA #$EF
    LDX $80
    BMI L_E72D
    LDA $60,X
    CMP #$0B
    BCC L_E712
    LDA #$EF
    STA $80
    JMP L_E72D
L_E712:
    TXA
    ASL A
    ASL A
    CLC
    ADC #$A1
    STA $0241
    CLC
    ADC #$02
    STA $0245
    LDA #$40
    STA $0243
    LDA #$48
    STA $0247
    LDA #$A4
L_E72D:
    STA $0240
    STA $0244
    LDA #$01
    STA $0242
    STA $0246
    LDA #$EF
    LDX $82
    BMI L_E769
    LDA $60,X
    CMP #$0B
    BCC L_E74E
    LDA #$EF
    STA $82
    JMP L_E769
L_E74E:
    TXA
    ASL A
    ASL A
    CLC
    ADC #$A1
    STA $0249
    CLC
    ADC #$02
    STA $024D
    LDA #$B0
    STA $024B
    LDA #$B8
    STA $024F
    LDA #$A0
L_E769:
    STA $0248
    STA $024C
    LDA #$01
    STA $024A
    STA $024E
    RTS
L_E778:
    LDA #$98
    STA $0250
    STA $0254
    LDA #$F1
    STA $0251
    LDA #$F3
    STA $0255
    LDA #$02
    STA $0252
    STA $0256
    LDA #$78
    STA $0253
    LDA #$80
    STA $0257
    RTS
L_E79D:
    LDA #$EF
    STA $0240
    STA $0244
    STA $0248
    STA $024C
    STA $0250
    STA $0254
    RTS
L_E7B2:
    LDX #$37
L_E7B4:
    LDA $FF6F,X
    STA $0280,X
    DEX
    BPL L_E7B4
    LDA #$34
    STA $2C
    LDA #$35
    STA $2D
    LDA #$36
    STA $2E
    LDA #$37
    STA $2F
    RTS
L_E7CE:
    LDA $58
    BEQ L_E7D9
    DEC $58
    JSR L_CAB6
    CLC
    RTS
L_E7D9:
    SEC
    RTS
L_E7DB:
    STA $08
    LDA $58
    SEC
    SBC $08
    STA $58
    PHP
    BCS L_E7EB
    LDA #$00
    STA $58
L_E7EB:
    JSR L_CAB6
    PLP
    RTS
L_E7F0:
    TXA
    PHA
    LDA $59
    SEC
    BEQ L_E7FD
    DEC $59
    JSR L_CACC
    CLC
L_E7FD:
    PLA
    TAX
    RTS
    .byte $18,$65,$58,$90,$05,$A9,$6D,$4C,$10,$E8,$C9,$6E,$90,$02,$A9,$6D
    .byte $85,$58,$20,$B6,$CA,$60,$18,$65,$59,$90,$05,$A9,$6D,$4C,$26,$E8
    .byte $C9,$6E,$90,$02,$A9,$6D,$85,$59,$20,$CC,$CA,$60,$18,$65,$5A,$90
    .byte $05,$A9,$6D,$4C,$3C,$E8,$C9,$6E,$90,$02,$A9,$6D,$85,$5A,$20,$F8
    .byte $CA,$60
L_E842:
    STA $08
    LDA $5A
    SEC
    SBC $08
    BCC L_E851
    STA $5A
    JSR L_CAF8
    SEC
L_E851:
    RTS
    .byte $E6,$5B,$20,$E2,$CA,$18,$60,$18,$65,$5B,$90,$05,$A9,$6D,$4C,$69
    .byte $E8,$C9,$6E,$90,$02,$A9,$6D,$85,$5B,$20,$E2,$CA,$60
L_E86F:
    LDA $5B
    BEQ L_E87A
    DEC $5B
    JSR L_CAE2
    CLC
    RTS
L_E87A:
    SEC
    RTS
L_E87C:
    LDA $48
    CMP #$10
    BNE L_E883
    RTS
L_E883:
    LDA $2D
    CMP #$30
    BCC L_E88C
    JMP L_E901
L_E88C:
    LDA $E9
    ASL A
    CLC
    ADC $E9
    STA $E3
    CLC
    ADC #$03
    STA $E4
    LDA $E3
    ASL A
    ASL A
    ASL A
    ASL A
    STA $E5
    CLC
    ADC #$20
    STA $E7
    LDA #$04
    STA $E6
    LDA $78
    STA $E8
L_E8AE:
    JSR L_E98F
    LDA $EE
    BEQ L_E8CB
    BMI L_E8D7
    CMP #$01
    BEQ L_E8C5
    CMP #$18
    BCS L_E8D1
    JSR L_EABF
    JMP L_E8DA
L_E8C5:
    JSR L_EA94
    JMP L_E8DA
L_E8CB:
    JSR L_E9A5
    JMP L_E8DA
L_E8D1:
    JSR L_EA2E
    JMP L_E8DA
L_E8D7:
    JSR L_EF1C
L_E8DA:
    JSR L_E99A
    INC $E3
    LDA $E5
    CLC
    ADC #$10
    STA $E5
    LDA $E7
    CLC
    ADC #$10
    STA $E7
    LDA $E3
    CMP $E4
    BCC L_E8AE
    LDA $E9
    CLC
    ADC #$01
    CMP #$03
    BCC L_E8FE
    LDA #$00
L_E8FE:
    STA $E9
    RTS
L_E901:
    LDA $E9
    AND #$01
    BEQ L_E90A
    JMP L_E945
L_E90A:
    LDA #$00
    STA $E5
    LDA #$04
    STA $E6
    LDA #$00
    STA $E3
    LDA #$20
    STA $E7
    LDA $78
    STA $E8
    JSR L_E98F
    LDA $EE
    BEQ L_E939
    BPL L_E933
    JSR L_F430
    JSR L_F53B
    JSR L_F552
    JMP L_E93C
L_E933:
    JSR L_F3B0
    JMP L_E93C
L_E939:
    JSR L_F349
L_E93C:
    JSR L_E99A
    JSR L_F55E
    JMP L_E988
L_E945:
    LDA #$04
    STA $E3
    LDA #$40
    STA $E5
    LDA #$04
    STA $E6
    LDA #$60
    STA $E7
    LDA $78
    STA $E8
L_E959:
    JSR L_E98F
    LDA $EE
    BEQ L_E968
    BMI L_E968
    JSR L_EA94
    JMP L_E96F
L_E968:
    LDA #$00
    STA $EE
    JSR L_EA4F
L_E96F:
    JSR L_E99A
    INC $E3
    LDA $E5
    CLC
    ADC #$10
    STA $E5
    LDA $E7
    CLC
    ADC #$10
    STA $E7
    LDA $E3
    CMP #$09
    BCC L_E959
L_E988:
    LDA $E9
    EOR #$01
    STA $E9
    RTS
L_E98F:
    LDY #$0F
L_E991:
    LDA ($E5),Y
    STA a:$00ED,Y
    DEY
    BPL L_E991
    RTS
L_E99A:
    LDY #$0F
L_E99C:
    LDA a:$00ED,Y
    STA ($E5),Y
    DEY
    BPL L_E99C
    RTS
L_E9A5:
    DEC $F3
    LDX $F3
    CPX #$3C
    BCS L_E9E6
    LDY #$02
    LDA ($E7),Y
    INY
    ORA ($E7),Y
    BNE L_E9CB
    LDA #$0C
    JSR L_CC64
    ASL A
    ASL A
    ASL A
    ASL A
    STA $0A
    LDA #$40
    JSR L_CC64
    STA $0F
    JMP L_E9D6
L_E9CB:
    LDY #$03
    LDA ($E7),Y
    STA $0A
    DEY
    LDA ($E7),Y
    STA $0F
L_E9D6:
    LDA #$00
    STA $0E
    STA $0B
    JSR L_CE7C
    BCS L_E9E6
    JSR L_F23A
    BCC L_E9E7
L_E9E6:
    RTS
L_E9E7:
    LDA $0E
    STA $F9
    LDA $0F
    STA $FA
    LDA $0A
    STA $FB
    LDA #$00
    STA $F1
    STA $F0
    STA $F4
    STA $FC
    LDY #$04
    LDA ($E7),Y
    STA $F2
    INY
    LDA ($E7),Y
    STA $F8
    LDX $40
    LDA #$00
    SEC
L_EA0D:
    ROL A
    DEX
    BPL L_EA0D
    AND $41
    BNE L_EA1D
    ASL $F8
    BCC L_EA1D
    LDA #$FF
    STA $F8
L_EA1D:
    LDA #$7F
    STA $EE
    LDA #$F9
    STA $ED
    LDA #$01
    STA $EF
    LDA $F3
    JMP L_EA30
L_EA2E:
    DEC $F3
L_EA30:
    BNE L_EA42
    LDA #$01
    STA $EE
    LDY #$00
    LDA ($E7),Y
    STA $ED
    INY
    LDA ($E7),Y
    STA $EF
    RTS
L_EA42:
    LDA $F3
    AND #$03
    BNE L_EA4E
    LDA $EF
    EOR #$40
    STA $EF
L_EA4E:
    RTS
L_EA4F:
    LDA #$1E
    JSR L_CC64
    TAX
    BNE L_EA93
    LDX #$03
    LDY #$03
    LDA $0402
    AND #$40
    BEQ L_EA64
    LDY #$13
L_EA64:
    LDA $040C,Y
    STA $F9,X
    DEY
    DEX
    BPL L_EA64
    LDA #$00
    STA $F1
    STA $F0
    STA $F4
    LDY #$04
    LDA ($E7),Y
    STA $F2
    INY
    LDA ($E7),Y
    STA $F8
    LDA #$01
    STA $EE
    LDA #$81
    STA $ED
    LDA #$04
    JSR L_CC64
    STA $EF
    LDA #$80
    STA $F1
L_EA93:
    RTS
L_EA94:
    LDY #$08
    LDA ($E7),Y
    CMP #$09
    BCC L_EA9E
    LDA #$00
L_EA9E:
    ASL A
    TAX
    LDA $EAAD,X
    STA $0E
    LDA $EAAE,X
    STA $0F
    JMP ($000E)
    .byte $FD,$EA,$69,$EB,$90,$EB,$D8,$EB,$76,$EC,$A8,$EC,$2A,$ED,$6F,$ED
    .byte $9F,$ED
L_EABF:
    LDA $F0
    BNE L_EACF
    LDA $F1
    BEQ L_EAD7
    JSR L_EEDA
    BCS L_EAD7
    JSR L_EF04
L_EACF:
    JSR L_EEBB
    BCS L_EAD7
    JSR L_EF04
L_EAD7:
    LDX $F3
    DEX
    BNE L_EAE5
    LDA #$00
    STA $EE
    LDA #$F0
    STA $F3
    RTS
L_EAE5:
    STX $F3
    CPX #$3C
    BCS L_EAF9
    LDX #$EF
    LDA $FB
    CMP #$EF
    BNE L_EAF5
    LDX $FC
L_EAF5:
    STX $FB
    STA $FC
L_EAF9:
    JSR L_F179
    RTS
    .byte $A5,$F3,$C9,$20,$B0,$0A,$A5,$F1,$D0,$25,$A5,$F5,$05,$F7,$D0,$1F
    .byte $A9,$00,$85,$F3,$20,$A6,$EE,$A9,$06,$20,$64,$CC,$18,$69,$01,$85
    .byte $F6,$A9,$04,$20,$64,$CC,$AA,$D0,$06,$A9,$80,$05,$F4,$85,$F4,$A5
    .byte $F6,$48,$A8,$A5,$F4,$20,$70,$CD,$A5,$F0,$D0,$1C,$A5,$F1,$D0,$04
    .byte $A5,$F4,$10,$05,$20,$DA,$EE,$90,$14,$A9,$00,$85,$F1,$20,$E1,$F0
    .byte $90,$0B,$20,$11,$EF,$4C,$5D,$EB,$20,$BB,$EE,$B0,$03,$20,$04,$EF
    .byte $20,$79,$F1,$20,$1E,$F0,$68,$85,$F6,$4C,$F0,$EF,$A5,$F5,$05,$F7
    .byte $D0,$03,$20,$9A,$EE,$A0,$09,$B1,$E7,$A8,$A5,$F4,$20,$70,$CD,$20
    .byte $1B,$F1,$90,$06,$20,$11,$EF,$4C,$8A,$EB,$20,$04,$EF,$20,$1E,$F0
    .byte $4C,$F0,$EF,$A5,$F5,$05,$F7,$D0,$03,$20,$8D,$EE,$A5,$F0,$F0,$08
    .byte $20,$BB,$EE,$90,$24,$4C,$CF,$EB,$A0,$09,$B1,$E7,$A8,$A5,$F4,$20
    .byte $70,$CD,$20,$E1,$F0,$B0,$18,$A0,$01,$20,$33,$F2,$90,$11,$A5,$0E
    .byte $F0,$07,$A0,$0D,$20,$33,$F2,$90,$06,$20,$04,$EF,$4C,$CF,$EB,$20
    .byte $11,$EF,$20,$79,$F1,$20,$1E,$F0,$4C,$F0,$EF,$A5,$F4,$29,$0F,$85
    .byte $F4,$A5,$F5,$05,$F7,$D0,$4A,$A5,$F9,$D0,$1A,$A5,$FA,$85,$0C,$A5
    .byte $FB,$85,$0D,$20,$54,$CA,$A0,$00,$B1,$0C,$29,$3F,$F0,$39,$C8,$B1
    .byte $0C,$29,$3F,$F0,$32,$A5,$F4,$29,$03,$D0,$04,$A9,$01,$85,$F4,$A6
    .byte $F3,$A9,$00,$85,$F3,$CA,$D0,$0D,$A5,$F4,$29,$03,$F0,$19,$49,$03
    .byte $85,$F4,$4C,$3B,$EC,$20,$19,$EE,$A9,$80,$05,$F4,$85,$F4,$4C,$3B
    .byte $EC,$A5,$F3,$C9,$10,$90,$07,$A9,$00,$85,$F3,$20,$19,$EE,$A0,$09
    .byte $B1,$E7,$A8,$A5,$F4,$20,$70,$CD,$A5,$F0,$D0,$1C,$A5,$F1,$D0,$04
    .byte $A5,$F4,$10,$05,$20,$DA,$EE,$90,$14,$A9,$00,$85,$F1,$20,$E1,$F0
    .byte $90,$0B,$20,$11,$EF,$4C,$6D,$EC,$20,$BB,$EE,$B0,$03,$20,$04,$EF
    .byte $20,$79,$F1,$20,$1E,$F0,$4C,$F0,$EF,$A5,$F5,$05,$F7,$F0,$06,$A5
    .byte $F3,$C9,$20,$90,$03,$20,$53,$EE,$A0,$09,$B1,$E7,$A8,$A5,$F4,$20
    .byte $70,$CD,$20,$1B,$F1,$90,$0B,$20,$DA,$F2,$90,$06,$20,$11,$EF,$4C
    .byte $A2,$EC,$20,$04,$EF,$20,$1E,$F0,$4C,$F0,$EF,$A5,$F0,$D0,$4E,$A5
    .byte $F1,$D0,$66,$A5,$FA,$85,$0F,$A5,$F9,$85,$0E,$A5,$FB,$85,$0A,$20
    .byte $F0,$ED,$B0,$07,$E6,$F0,$E6,$F0,$4C,$FA,$EC,$A5,$F5,$05,$F7,$D0
    .byte $03,$20,$8D,$EE,$20,$90,$CE,$B0,$17,$A0,$09,$B1,$E7,$A8,$A5,$F4
    .byte $20,$70,$CD,$20,$E1,$F0,$B0,$3C,$20,$F0,$ED,$90,$37,$4C,$10,$ED
    .byte $A9,$00,$85,$F5,$85,$F6,$20,$79,$F1,$A5,$F0,$B0,$27,$20,$BB,$EE
    .byte $20,$04,$EF,$A5,$F0,$48,$20,$79,$F1,$68,$90,$07,$69,$05,$85,$F1
    .byte $4C,$24,$ED,$20,$04,$EF,$4C,$24,$ED,$20,$DA,$EE,$B0,$06,$20,$04
    .byte $EF,$4C,$24,$ED,$20,$11,$EF,$20,$1E,$F0,$4C,$F0,$EF,$A5,$F4,$F0
    .byte $03,$4C,$D8,$EB,$A9,$01,$20,$5D,$ED,$B0,$20,$A9,$02,$20,$5D,$ED
    .byte $B0,$19,$A9,$04,$20,$5D,$ED,$B0,$12,$A9,$08,$20,$5D,$ED,$B0,$0B
    .byte $A0,$04,$B1,$E7,$85,$F2,$A9,$00,$85,$FC,$60,$A9,$01,$85,$F4,$60
    .byte $A0,$01,$20,$70,$CD,$20,$F1,$EF,$20,$7C,$CE,$90,$04,$20,$36,$F1
    .byte $38,$60,$A5,$F5,$05,$F7,$D0,$03,$20,$9A,$EE,$A0,$09,$B1,$E7,$A8
    .byte $A5,$F4,$20,$70,$CD,$20,$1B,$F1,$90,$0A,$A5,$EA,$D0,$0F,$20,$11
    .byte $EF,$4C,$94,$ED,$20,$04,$EF,$20,$1E,$F0,$4C,$F0,$EF,$A9,$80,$85
    .byte $EE,$60,$C6,$F1,$F0,$48,$A5,$F4,$D0,$06,$20,$53,$EE,$4C,$D0,$ED
    .byte $A5,$F3,$C9,$08,$90,$1D,$A5,$F4,$85,$08,$20,$53,$EE,$A5,$F4,$45
    .byte $08,$A0,$00,$A2,$04,$4A,$90,$01,$C8,$CA,$D0,$F9,$88,$F0,$04,$A5
    .byte $08,$85,$F4,$A0,$09,$B1,$E7,$A8,$A5,$F4,$20,$70,$CD,$20,$1B,$F1
    .byte $90,$03,$4C,$EB,$ED,$20,$04,$EF,$20,$1E,$F0,$4C,$F0,$EF,$A9,$00
    .byte $85,$EE,$60,$A5,$0A,$29,$0F,$D0,$21,$A5,$0F,$85,$0C,$A5,$0A,$38
    .byte $E9,$10,$85,$0D,$20,$54,$CA,$A0,$00,$20,$D3,$F2,$90,$0C,$A5,$0E
    .byte $F0,$07,$A0,$0C,$20,$D3,$F2,$90,$01,$60,$18,$60
L_EE19:
    LDX #$00
    LDA $FA
    SEC
    SBC $44
    BEQ L_EE26
    INX
    BCC L_EE26
    INX
L_EE26:
    STX $F4
    LDA $FB
    SEC
    SBC $45
    BCC L_EE46
    LDY #$09
    LDA ($E7),Y
    BEQ L_EE52
    LDA #$03
    JSR L_CC64
    TAX
    BNE L_EE52
    LDA #$80
    ORA $F4
    STA $F4
    JMP L_EE52
L_EE46:
    LDA #$03
    JSR L_CC64
    TAX
    BNE L_EE52
    LDA #$04
    STA $F4
L_EE52:
    RTS
    .byte $A5,$FA,$85,$0F,$A5,$F9,$85,$0E,$A5,$FB,$85,$0A,$20,$90,$CE,$A2
    .byte $00,$B0,$09,$A5,$FA,$38,$E5,$44,$E8,$90,$01,$E8,$86,$F4,$20,$B6
    .byte $CE,$A2,$00,$B0,$0B,$A5,$FB,$38,$E5,$45,$A2,$04,$90,$02,$A2,$08
    .byte $8A,$05,$F4,$85,$F4,$A9,$00,$85,$F3,$60,$A5,$F4,$29,$03,$D0,$02
    .byte $A9,$01,$49,$03,$85,$F4,$60,$A9,$08,$20,$64,$CC,$AA,$BD,$B3,$EE
    .byte $85,$F4,$60,$A9,$03,$20,$64,$CC,$0A,$AA,$BD,$B3,$EE,$85,$F4,$60
    .byte $01,$05,$04,$06,$02,$0A,$08,$09
L_EEBB:
    LDA $F0
    LSR A
    CLC
    ADC #$02
    STA $F7
    JSR L_F0E1
    BCS L_EEC9
    RTS
L_EEC9:
    LDA #$00
    STA $F5
    STA $F6
    JSR L_F0E1
    BCS L_EED5
    RTS
L_EED5:
    LDA #$00
    STA $F7
    RTS
L_EEDA:
    LDX $F1
    BNE L_EEE0
    LDX #$0F
L_EEE0:
    DEX
    STX $F1
    TXA
    LSR A
    EOR #$FF
    CLC
    ADC #$01
    STA $F7
    JSR L_F0E1
    BCS L_EEF2
    RTS
L_EEF2:
    LDA #$00
    STA $F5
    STA $F6
    JSR L_F0E1
    BCS L_EEFE
    RTS
L_EEFE:
    INC $F1
    JSR L_F2DA
    RTS
L_EF04:
    LDA $0E
    STA $F9
    LDA $0F
    STA $FA
    LDA $0A
    STA $FB
    RTS
L_EF11:
    LDA #$00
    STA $F5
    STA $F7
    STA $F1
    STA $F0
    RTS
L_EF1C:
    LDA $EE
    AND #$7F
    BNE L_EF45
    INC a:$00EE
    LDA #$0E
    STA $8F
    LDA #$08
    STA $F1
    LDA #$00
    STA $F5
    STA $F6
    STA $F0
    LDA $FB
    STA $FC
    LDY #$06
    LDA ($E7),Y
    STA $ED
    LDA $EF
    AND #$03
    STA $EF
L_EF45:
    LDA $F0
    BNE L_EF6E
    DEC $F1
    BEQ L_EF63
    LDA $F1
    EOR #$FF
    CLC
    ADC #$01
    STA $F7
    JSR L_EFF1
    JSR L_CF08
    BCS L_EF63
    LDA $0A
    STA $FB
    RTS
L_EF63:
    LDA $EF
    ORA #$80
    STA $EF
    LDA #$01
    STA $F0
    RTS
L_EF6E:
    INC $F0
    LDA $F0
    LSR A
    CLC
    ADC #$02
    STA $F7
    JSR L_EFF1
    JSR L_CF08
    BCS L_EF85
    LDA $0A
    STA $FB
    RTS
L_EF85:
    LDX #$00
    LDA $58
    CMP #$14
    BCC L_EFC4
    INX
    LDA $59
    CMP #$1E
    BCC L_EFC4
    LDX #$04
    LDA $5B
    CMP #$02
    BCC L_EFC4
    LDA #$14
    JSR L_CC64
    CMP #$09
    BCS L_EFAC
    TAY
    LDX $EFE7,Y
    JMP L_EFC4
L_EFAC:
    LDX #$00
    LDA $58
    CMP $59
    BCC L_EFBE
    INX
    LDA $59
    CMP $5A
    BCC L_EFC4
    JMP L_EFC2
L_EFBE:
    CMP $5A
    BCC L_EFC4
L_EFC2:
    LDX #$02
L_EFC4:
    TXA
    CLC
    ADC #$02
    STA $EE
    TXA
    ASL A
    ASL A
    ORA #$81
    STA $ED
    LDA #$01
    STA $EF
    LDA $FC
    STA $FB
    LDA #$F0
    STA $F3
    LDA #$00
    STA $F0
    STA $F1
    JSR L_F179
    RTS
    .byte $03,$03,$03,$03,$04,$04,$05,$06,$07
L_EFF0:
    RTS
L_EFF1:
    LDA $F9
    STA $0E
    LDA $FA
    STA $0F
    LDA $FB
    STA $0A
    LDA $F7
    BEQ L_F006
    CLC
    ADC $0A
    STA $0A
L_F006:
    LDA $F5
    BEQ L_F01D
    CLC
    ADC $0E
    PHA
    AND #$0F
    STA $0E
    PLA
    ASL A
    ASL A
    ASL A
    ASL A
    LDA $0F
    ADC $F6
    STA $0F
L_F01D:
    RTS
    .byte $A0,$07,$B1,$E7,$29,$03,$0A,$AA,$BD,$33,$F0,$85,$0E,$BD,$34,$F0
    .byte $85,$0F,$6C,$0E,$00,$3B,$F0,$4B,$F0,$71,$F0,$B9,$F0,$E6,$F3,$A5
    .byte $F3,$29,$03,$F0,$01,$60,$A5,$EF,$49,$40,$85,$EF,$60,$A5,$F5,$F0
    .byte $12,$A0,$00,$A5,$F6,$30,$02,$A0,$40,$84,$08,$A5,$EF,$29,$3F,$05
    .byte $08,$85,$EF,$E6,$F3,$A5,$F3,$29,$03,$F0,$01,$60,$A5,$ED,$49,$04
    .byte $85,$ED,$60,$A5,$F5,$F0,$1B,$A0,$00,$A5,$F6,$30,$02,$A0,$40,$84
    .byte $08,$A5,$EF,$29,$3F,$05,$08,$85,$EF,$A5,$ED,$29,$F7,$85,$ED,$4C
    .byte $9C,$F0,$A5,$F7,$F0,$08,$A5,$ED,$29,$F3,$09,$08,$85,$ED,$E6,$F3
    .byte $A5,$F3,$29,$03,$F0,$01,$60,$A5,$ED,$29,$08,$D0,$07,$A5,$ED,$49
    .byte $04,$85,$ED,$60,$A5,$EF,$49,$40,$85,$EF,$60,$A5,$F5,$F0,$12,$A0
    .byte $00,$A5,$F6,$30,$02,$A0,$40,$84,$08,$A5,$EF,$29,$3F,$05,$08,$85
    .byte $EF,$E6,$F3,$A5,$F3,$29,$06,$0A,$85,$08,$A5,$ED,$29,$F3,$05,$08
    .byte $85,$ED,$60
L_F0E1:
    LDA $F7
    PHA
L_F0E4:
    JSR L_EFF1
    JSR L_CF08
    BCS L_F10E
    LDX $EE
    DEX
    BNE L_F0F9
    JSR L_CE7C
    BCC L_F0F9
    JSR L_F136
L_F0F9:
    JSR L_F23A
    BCC L_F117
    LDX $F7
    BEQ L_F116
    BMI L_F106
    DEX
    DEX
L_F106:
    INX
    STX $F7
    BNE L_F0E4
    JMP L_F116
L_F10E:
    LDA #$00
    STA $EE
    LDA #$F0
    STA $F3
L_F116:
    SEC
L_F117:
    PLA
    STA $F7
    RTS
    .byte $20,$F1,$EF,$20,$7C,$CE,$90,$05,$20,$36,$F1,$38,$60,$20,$08,$CF
    .byte $90,$08,$A9,$00,$85,$EE,$A9,$F0,$85,$F3,$60
L_F136:
    LDA $85
    BNE L_F178
    LDX $EE
    DEX
    BNE L_F178
    LDA $2D
    CMP #$30
    BCC L_F154
    LDA $E3
    BEQ L_F15A
    LDX $55
    LDA $51,X
    CMP #$0A
    BEQ L_F173
    JMP L_F15A
L_F154:
    LDA $40
    CMP #$04
    BEQ L_F178
L_F15A:
    LDA $F8
    JSR L_E7DB
    LDA #$21
    STA a:$008F
    LDA #$01
    STA $90
    LDA #$01
    STA $85
    LDA $EF
    AND #$DF
    STA $EF
    RTS
L_F173:
    LDA #$01
    STA a:$008F
L_F178:
    RTS
L_F179:
    LDA $F1
    BNE L_F1D3
    LDA $FA
    STA $0C
    STA $0F
    LDA $F9
    STA $0E
    LDX $FB
    LDY $EE
    DEY
    BEQ L_F199
    CPX #$EF
    BNE L_F194
    LDX $FC
L_F194:
    STX $0D
    JMP L_F1A7
L_F199:
    CPX #$B0
    BCS L_F1CF
    STX $0D
    INX
    STX $0A
    JSR L_CE7C
    BCS L_F1D3
L_F1A7:
    JSR L_CA54
    LDA $F9
    BNE L_F1BD
    LDY #$00
    LDA ($0C),Y
    AND #$3F
    BEQ L_F1D3
    INY
    LDA ($0C),Y
    AND #$3F
    BEQ L_F1D3
L_F1BD:
    LDY #$01
    JSR L_F233
    BCS L_F1D3
    LDA $F9
    BEQ L_F1CF
    LDY #$0D
    JSR L_F233
    BCS L_F1D3
L_F1CF:
    INC $F0
    CLC
    RTS
L_F1D3:
    LDA $F0
    CMP #$0C
    BCC L_F1DE
    SEC
    SBC #$04
    STA $F1
L_F1DE:
    LDA #$00
    STA $F0
    SEC
    RTS
L_F1E4:
    LDA $F1
    BNE L_F223
    LDA $FA
    STA $0C
    STA $0F
    LDA $F9
    STA $0E
    LDX $FB
    STX $0D
    INX
    STX $0A
    JSR L_CA54
    LDA $FB
    CMP #$A0
    BCS L_F220
    JSR L_CEC7
    BCS L_F223
    LDY #$02
    JSR L_F233
    BCS L_F223
    LDY #$0E
    JSR L_F233
    BCS L_F223
    LDA $F9
    BEQ L_F220
    LDY #$1A
    JSR L_F233
    BCS L_F223
L_F220:
    INC $F0
    RTS
L_F223:
    LDA $F0
    CMP #$0C
    BCC L_F22E
    SEC
    SBC #$04
    STA $F1
L_F22E:
    LDA #$00
    STA $F0
    RTS
L_F233:
    LDA ($0C),Y
    AND #$3F
    CMP #$30
    RTS
L_F23A:
    LDA $0F
    STA $0C
    LDA $0A
    STA $0D
    JSR L_CA54
    LDY #$00
    JSR L_F2D3
    BCS L_F274
    LDA $0E
    BEQ L_F257
    LDY #$0C
    JSR L_F2D3
    BCS L_F274
L_F257:
    LDA $0A
    CMP #$B0
    BCS L_F273
    AND #$0F
    BEQ L_F273
    LDY #$01
    JSR L_F2D3
    BCS L_F274
    LDA $0E
    BEQ L_F273
    LDY #$0D
    JSR L_F2D3
    BCS L_F274
L_F273:
    CLC
L_F274:
    RTS
L_F275:
    LDA $0F
    STA $0C
    LDA $0A
    STA $0D
    JSR L_CA54
    LDY #$00
    JSR L_F2D3
    BCS L_F2D2
    LDY #$01
    JSR L_F2D3
    BCS L_F2D2
    LDY #$0C
    JSR L_F2D3
    BCS L_F2D2
    LDY #$0D
    JSR L_F2D3
    BCS L_F2D2
    LDA $0E
    BEQ L_F2AE
    LDY #$18
    JSR L_F2D3
    BCS L_F2D2
    LDY #$19
    JSR L_F2D3
    BCS L_F2D2
L_F2AE:
    LDA $0A
    CMP #$B0
    BCS L_F2D1
    AND #$0F
    BEQ L_F2D1
    LDY #$02
    JSR L_F2D3
    BCS L_F2D2
    LDY #$0E
    JSR L_F2D3
    BCS L_F2D2
    LDA $0E
    BEQ L_F2D1
    LDY #$1A
    JSR L_F2D3
    BCS L_F2D2
L_F2D1:
    CLC
L_F2D2:
    RTS
L_F2D3:
    LDA ($0C),Y
    AND #$3F
    CMP #$30
    RTS
L_F2DA:
    LDA #$00
    STA $F6
    LDX $F5
    BEQ L_F30F
    STA $F5
    LDA $FB
    AND #$0F
    BEQ L_F347
    CMP #$06
    BCC L_F302
    CMP #$0B
    BCS L_F2F5
    JMP L_F347
L_F2F5:
    LDA $F4
    AND #$08
    BNE L_F347
    LDA #$01
    STA $F7
    JMP L_F343
L_F302:
    LDA $F4
    AND #$04
    BNE L_F347
    LDA #$FF
    STA $F7
    JMP L_F343
L_F30F:
    LDX $F7
    BEQ L_F347
    STA $F7
    LDA $F9
    BEQ L_F347
    CMP #$06
    BCC L_F335
    CMP #$0B
    BCS L_F324
    JMP L_F347
L_F324:
    LDA $F4
    AND #$02
    BNE L_F347
    LDA #$01
    STA $F5
    LDA #$00
    STA $F6
    JMP L_F343
L_F335:
    LDA $F4
    AND #$01
    BNE L_F347
    LDA #$0F
    STA $F5
    LDA #$FF
    STA $F6
L_F343:
    JSR L_F0E1
    RTS
L_F347:
    SEC
    RTS
L_F349:
    LDA #$3D
    STA $2E
    LDY #$03
    LDA ($E7),Y
    STA $0A
    DEY
    LDA ($E7),Y
    STA $0F
    LDA #$00
    STA $0E
    STA $0B
    JSR L_F275
    BCC L_F364
    RTS
L_F364:
    LDA $0E
    STA $F9
    LDA $0F
    STA $FA
    LDA $0A
    STA $FB
    LDA #$00
    STA $F1
    STA $F0
    STA $F4
    LDA #$01
    STA $EE
    LDA #$81
    STA $ED
    LDA #$02
    STA $EF
    LDY #$05
    LDA ($E7),Y
    STA $F8
    LDY #$04
    LDA ($E7),Y
    STA $F2
    STA $0415
    STA $0425
    STA $0435
    LDA #$E1
    STA $0E
    LDA #$A7
    STA $0F
    JSR L_CC9C
    LDA #$53
    STA $0E
    LDA #$CB
    STA $0F
    JSR L_CC9C
    RTS
L_F3B0:
    LDA $F4
    AND #$0F
    STA $F4
    LDA $F5
    ORA $F7
    BNE L_F3E8
    LDA $F4
    AND #$03
    BNE L_F3C6
    LDA #$01
    STA $F4
L_F3C6:
    LDX $F3
    LDA #$00
    STA $F3
    DEX
    BNE L_F3DC
    LDA $F4
    AND #$03
    BEQ L_F3EE
    EOR #$03
    STA $F4
    JMP L_F3F5
L_F3DC:
    JSR L_EE19
    LDA #$80
    ORA $F4
    STA $F4
    JMP L_F3F5
L_F3E8:
    LDA $F3
    CMP #$32
    BCC L_F3F5
L_F3EE:
    LDA #$00
    STA $F3
    JSR L_EE19
L_F3F5:
    LDA $F4
    LDY #$02
    JSR L_CD70
    LDA $F0
    BNE L_F41C
    LDA $F1
    BNE L_F408
    LDA $F4
    BPL L_F40D
L_F408:
    JSR L_F4E3
    BCC L_F421
L_F40D:
    LDA #$00
    STA $F1
    JSR L_F506
    BCC L_F421
    JSR L_EF11
    JMP L_F424
L_F41C:
    JSR L_F4C3
    BCS L_F424
L_F421:
    JSR L_EF04
L_F424:
    JSR L_F1E4
    JSR L_F53B
    JSR L_F552
    JMP L_EFF0
L_F430:
    LDA $EE
    AND #$7F
    BNE L_F473
    LDA #$18
    STA $8F
    LDA #$FF
    STA $90
    LDX #$03
    JSR L_C540
    LDA #$02
    STA $36
    JSR L_C135
    LDX #$03
    JSR L_C540
    LDA #$05
    STA $36
    JSR L_C135
    LDX #$03
    JSR L_C540
    INC a:$00EE
    LDA #$02
    STA a:$008F
    LDA #$0F
    STA $F1
    LDA #$00
    STA $F5
    STA $F6
    STA $F0
    LDA $FB
    STA $FC
L_F473:
    LDA $F0
    BNE L_F49E
    DEC $F1
    BEQ L_F493
    LDA $F1
    LSR A
    LSR A
    EOR #$FF
    CLC
    ADC #$01
    STA $F7
    JSR L_EFF1
    JSR L_CF08
    BCS L_F493
    LDA $0A
    STA $FB
    RTS
L_F493:
    LDA $EF
    ORA #$80
    STA $EF
    LDA #$01
    STA $F0
    RTS
L_F49E:
    INC $F0
    LDA $F0
    LSR A
    LSR A
    CLC
    ADC #$01
    STA $F7
    JSR L_EFF1
    JSR L_CF08
    BCS L_F4B6
    LDA $0A
    STA $FB
    RTS
L_F4B6:
    LDA #$00
    STA $EE
    LDA #$F0
    STA $F3
    LDA #$01
    STA $EB
    RTS
L_F4C3:
    LDA $F0
    LSR A
    LSR A
    CLC
    ADC #$01
    STA $F7
    JSR L_F506
    BCS L_F4D2
    RTS
L_F4D2:
    LDA #$00
    STA $F5
    STA $F6
    JSR L_F0E1
    BCS L_F4DE
    RTS
L_F4DE:
    LDA #$00
    STA $F7
    RTS
L_F4E3:
    LDX $F1
    BNE L_F4E9
    LDX #$19
L_F4E9:
    DEX
    STX $F1
    TXA
    LSR A
    LSR A
    EOR #$FF
    CLC
    ADC #$01
    STA $F7
    JSR L_F506
    BCS L_F4FC
    RTS
L_F4FC:
    LDA #$00
    STA $F5
    STA $F6
    JSR L_F506
    RTS
L_F506:
    LDA $F7
    PHA
L_F509:
    JSR L_EFF1
    JSR L_CF08
    BCS L_F52E
    JSR L_CEC7
    BCC L_F519
    JSR L_F136
L_F519:
    JSR L_F275
    BCC L_F537
    LDX $F7
    BEQ L_F536
    BMI L_F526
    DEX
    DEX
L_F526:
    INX
    STX $F7
    BNE L_F509
    JMP L_F536
L_F52E:
    LDA #$00
    STA $EE
    LDA #$F0
    STA $F3
L_F536:
    SEC
L_F537:
    PLA
    STA $F7
    RTS
L_F53B:
    LDY #$00
    LDA $F6
    BMI L_F547
    LDA $F5
    BEQ L_F551
    LDY #$40
L_F547:
    STY $08
    LDA $EF
    AND #$3F
    ORA $08
    STA $EF
L_F551:
    RTS
L_F552:
    INC $F3
    LDA $F3
    AND #$0C
    ASL A
    ORA #$41
    STA $ED
    RTS
L_F55E:
    LDA $FC
    STA $041F
    STA $042F
    STA $043F
    LDA $FB
    STA $041E
    CLC
    ADC #$10
    STA $042E
    STA $043E
    LDA $F9
    STA $041C
    STA $042C
    STA $043C
    LDX $FA
    STX $042D
    INX
    STX $041D
    STX $043D
    LDX $EE
    BMI L_F59F
    LDA $0411
    ORA $0421
    ORA $0431
    BPL L_F59F
    LDX #$80
L_F59F:
    STX $0401
    STX $0411
    STX $0421
    STX $0431
    LDA $F2
    CMP $0415
    BCC L_F5B5
    LDA $0415
L_F5B5:
    CMP $0425
    BCC L_F5BD
    LDA $0425
L_F5BD:
    CMP $0435
    BCC L_F5C5
    LDA $0435
L_F5C5:
    STA $0405
    LDA $ED
    ORA #$04
    STA $0410
    ORA #$20
    STA $0430
    AND #$FB
    STA $0420
    LDA $EF
    STA $0412
    STA $0422
    STA $0432
    AND #$40
    BEQ L_F600
    LDA $0400
    LDX $0410
    STA $0410
    STX $0400
    LDA $0420
    LDX $0430
    STA $0430
    STX $0420
L_F600:
    LDA $EF
    BPL L_F61C
    LDA $0400
    LDX $0420
    STA $0420
    STX $0400
    LDA $0410
    LDX $0430
    STA $0430
    STX $0410
L_F61C:
    LDA #$53
    STA $0E
    LDA #$CB
    STA $0F
    JSR L_CC9C
    RTS
L_F628:
    LDA #$0B
    STA $E3
    LDA #$B0
    STA $E5
    LDA #$04
    STA $E6
L_F634:
    LDY #$01
    LDA ($E5),Y
    BNE L_F648
    BIT $20
    BVC L_F64B
    BIT $FD
    BVS L_F64B
    JSR L_F664
    JMP L_F64B
L_F648:
    JSR L_F6BB
L_F64B:
    INC $E3
    CLC
    LDA #$10
    ADC $E5
    STA $E5
    LDA #$00
    ADC $E6
    STA $E6
    LDA $E3
    SEC
    SBC #$0B
    CMP $5E
    BCC L_F634
    RTS
L_F664:
    JSR L_E98F
    LDA $20
    AND #$40
    ORA $FD
    STA $FD
    LDY #$02
    LDA $88
    BEQ L_F677
    LDY #$04
L_F677:
    LDA $FD
    JSR L_CD70
    JSR L_F740
    JSR L_CF08
    BCS L_F6B8
    JSR L_E7F0
    BCS L_F6B8
    LDA $0E
    STA $F9
    LDA $0F
    STA $FA
    LDA $0A
    STA $FB
    JSR L_D067
    STA $EE
    BCS L_F69F
    JSR L_E7F0
L_F69F:
    JSR L_D051
    STA $F8
    BCS L_F6A9
    JSR L_E7F0
L_F6A9:
    LDA #$00
    STA $EF
    LDA #$21
    STA $ED
    LDA #$22
    CLC
    ADC $40
    STA $8F
L_F6B8:
    JMP L_F735
L_F6BB:
    JSR L_E98F
    DEC $EE
    BEQ L_F735
    JSR L_EFF1
    JSR L_CF08
    BCS L_F722
    JSR L_CDB2
    BCC L_F729
    LDA $2D
    CMP #$30
    BCC L_F6ED
    LDA $08
    CMP #$04
    BCC L_F6ED
    LDX $09
    LDA #$80
    STA $0401,X
    LDA #$01
    STA $EE
    LDA #$0C
    STA $8F
    JMP L_F71F
L_F6ED:
    LDY $0401,X
    DEY
    BNE L_F729
    LDX $09
    LDA $EE
    LDY #$FE
    AND #$01
    BEQ L_F6FF
    LDY #$02
L_F6FF:
    TYA
    STA $040F,X
    LDA $0405,X
    SEC
    SBC $F8
    STA $0405,X
    BCS L_F71B
    LDA #$80
    STA $0401,X
    LDA #$00
    STA $0405,X
    JMP L_F71F
L_F71B:
    LDA #$06
    STA $8F
L_F71F:
    JMP L_F729
L_F722:
    LDA #$00
    STA $EE
    JMP L_F735
L_F729:
    LDA $0E
    STA $F9
    LDA $0F
    STA $FA
    LDA $0A
    STA $FB
L_F735:
    LDA $EE
    BEQ L_F73C
    JSR L_F773
L_F73C:
    JSR L_E99A
    RTS
L_F740:
    LDA $43
    STA $0E
    LDA $44
    STA $0F
    LDA $45
    STA $0A
    LDA $F7
    BEQ L_F757
    ASL A
    ASL A
    CLC
    ADC $0A
    STA $0A
L_F757:
    LDA $F5
    BEQ L_F772
    ASL A
    ASL A
    AND #$0F
    CLC
    ADC $0E
    PHA
    AND #$0F
    STA $0E
    PLA
    ASL A
    ASL A
    ASL A
    ASL A
    LDA $0F
    ADC $F6
    STA $0F
L_F772:
    RTS
L_F773:
    LDA $EE
    AND #$0C
    STA $08
    LDA $ED
    AND #$F3
    ORA $08
    STA $ED
    RTS
L_F782:
    LDA $0491
    BNE L_F788
    RTS
L_F788:
    LDA #$90
    STA $E5
    LDA #$04
    STA $E6
    JSR L_E98F
    DEC $F3
    BNE L_F7F7
    LDA $ED
    AND #$01
    BNE L_F7AA
    LDA $FB
    AND #$0F
    ORA $F9
    BEQ L_F7AA
    INC $F3
    JMP L_F7F7
L_F7AA:
    LDA #$00
    STA $EE
    LDA $F0
    BNE L_F7B5
    JMP L_F896
L_F7B5:
    LDA $FA
    STA $0C
    LDA $FB
    STA $0D
    JSR L_CA54
    LDA $F0
    LDY #$00
    STA ($0C),Y
    LDA $FA
    SEC
    SBC $7C
    CMP #$11
    BCC L_F7D3
    CMP #$FE
    BCC L_F7F4
L_F7D3:
    LDA $FA
    STA $0C
    ASL A
    AND #$1F
    STA $16
    LDA $FA
    AND #$10
    LSR A
    LSR A
    STA $17
    CLC
    LDA #$00
    ADC $16
    STA $16
    LDA #$20
    ADC $17
    STA $17
    JSR L_C833
L_F7F4:
    JMP L_F896
L_F7F7:
    LDA $ED
    AND #$01
    BEQ L_F80C
    LDA $F3
    AND #$03
    BNE L_F809
    LDA $ED
    EOR #$04
    STA $ED
L_F809:
    JMP L_F896
L_F80C:
    LDA #$09
    STA $E3
    JSR L_EFF1
    JSR L_CF1C
    BCS L_F85A
    JSR L_F23A
    BCS L_F85A
    JSR L_CE7C
    BCS L_F846
    JSR L_CDB2
    BCC L_F82E
    LDX $09
    LDA #$80
    STA $0401,X
L_F82E:
    LDA $0E
    STA $F9
    LDA $0F
    STA $FA
    LDA $0A
    STA $FB
    LDA #$00
    STA $F4
    JMP L_F896
    .byte $E6,$F3,$4C,$96,$F8
L_F846:
    LDA $F4
    BNE L_F886
    LDA $85
    BNE L_F85A
    JSR L_E7CE
    LDA #$0A
    STA a:$008F
    LDA #$02
    STA $85
L_F85A:
    LDA $F4
    BNE L_F886
    INC $F4
    LDA $F5
    BEQ L_F873
    EOR #$FF
    CLC
    ADC #$01
    AND #$0F
    STA $F5
    LDA $F6
    EOR #$FF
    STA $F6
L_F873:
    LDA $F7
    EOR #$FF
    TAX
    INX
    STX $F7
    LDA $8F
    BNE L_F883
    LDA #$06
    STA $8F
L_F883:
    JMP L_F896
L_F886:
    LDA $FB
    AND #$0F
    ORA $F9
    BEQ L_F893
    INC $F3
    JMP L_F896
L_F893:
    JMP L_F7AA
L_F896:
    JSR L_E99A
    RTS
L_F89A:
    JSR L_FD74
    LDA #$40
    STA $02
    JSR L_FA60
    LDA $8D
    BEQ L_F8CD
    LDA #$00
    BIT $D4
    BMI L_F8B7
    LDA $A9
    AND #$C0
    ORA #$30
    STA $4004
L_F8B7:
    LDA $99
    AND #$C0
    ORA #$30
    STA $4000
    LDA #$00
    STA $4008
    LDA #$30
    STA $400C
    JMP L_F8EC
L_F8CD:
    JSR L_FD87
    LDA #$00
    STA $02
    JSR L_F8F0
    LDA #$10
    STA $02
    JSR L_F96E
    LDA #$20
    STA $02
    JSR L_FA09
    LDA #$30
    STA $02
    JSR L_FB1F
L_F8EC:
    JSR L_FD9C
    RTS
L_F8F0:
    BIT $94
    BMI L_F8F7
    JMP L_F95E
L_F8F7:
    DEC $93
    BEQ L_F8FE
    JMP L_F948
L_F8FE:
    LDY #$00
    LDA ($95),Y
    BEQ L_F910
    PHP
    CMP #$FF
    BNE L_F916
    PLP
    JSR L_FB8E
    JMP L_F8FE
L_F910:
    JSR L_FCF9
    JMP L_F95E
L_F916:
    JSR L_FD6B
    AND #$7F
    STA $93
    PLP
    BMI L_F942
    JSR L_FC81
    LDA $27
    ORA #$01
    STA $27
    LDA $9A
    STA $4001
    LDA $04
    STA $4002
    LDA $05
    AND #$07
    ORA #$18
    STA $4003
    JSR L_FCC4
    JMP L_F948
L_F942:
    JSR L_FCDF
    JMP L_F948
L_F948:
    LDA $27
    LSR A
    BCS L_F94E
    RTS
L_F94E:
    DEC $9D
    BNE L_F958
    JSR L_FD11
    STA $4000
L_F958:
    JSR L_FD45
    BCS L_F95E
    RTS
L_F95E:
    LDA $99
    AND #$C0
    ORA #$30
    STA $4000
    LDA $27
    AND #$FE
    STA $27
    RTS
L_F96E:
    BIT $A4
    BMI L_F978
    BVS L_F977
    JMP L_F9F9
L_F977:
    RTS
L_F978:
    DEC $A3
    BEQ L_F97F
    JMP L_F9DD
L_F97F:
    LDY #$00
    LDA ($A5),Y
    BEQ L_F991
    PHP
    CMP #$FF
    BNE L_F997
    PLP
    JSR L_FB8E
    JMP L_F97F
L_F991:
    JSR L_FCF9
    JMP L_F9F9
L_F997:
    JSR L_FD6B
    AND #$7F
    STA $A3
    PLP
    BMI L_F9D2
    BIT $A4
    BVC L_F9AB
    JSR L_FD6B
    JMP L_F9D6
L_F9AB:
    JSR L_FC81
    LDA $27
    ORA #$02
    STA $27
    LDA $A9
    STA $4004
    LDA $AA
    STA $4005
    LDA $04
    STA $4006
    LDA $05
    AND #$07
    ORA #$18
    STA $4007
    JSR L_FCC4
    JMP L_F9DD
L_F9D2:
    BIT $A4
    BVC L_F9D7
L_F9D6:
    RTS
L_F9D7:
    JSR L_FCDF
    JMP L_F9DD
L_F9DD:
    BIT $A4
    BVC L_F9E2
    RTS
L_F9E2:
    LDA $27
    AND #$02
    BNE L_F9E9
    RTS
L_F9E9:
    DEC $AD
    BNE L_F9F3
    JSR L_FD11
    STA $4004
L_F9F3:
    JSR L_FD45
    BCS L_F9F9
    RTS
L_F9F9:
    LDA $A9
    AND #$C0
    ORA #$30
    STA $4004
    LDA $27
    AND #$FD
    STA $27
    RTS
L_FA09:
    LDA $B4
    BMI L_FA10
    JMP L_FA54
L_FA10:
    DEC $B3
    BEQ L_FA15
    RTS
L_FA15:
    LDY #$00
    LDA ($B5),Y
    BEQ L_FA27
    PHP
    CMP #$FF
    BNE L_FA2D
    PLP
    JSR L_FB8E
    JMP L_FA15
L_FA27:
    JSR L_FCF9
    JMP L_FA54
L_FA2D:
    JSR L_FD6B
    AND #$7F
    STA $B3
    PLP
    BMI L_FA54
    JSR L_FC81
    LDA $27
    ORA #$04
    STA $27
    LDA $BA
    STA $4008
    LDA $04
    STA $400A
    LDA $05
    AND #$07
    ORA #$F8
    STA $400B
    RTS
L_FA54:
    LDA #$00
    STA $4008
    LDA $27
    AND #$FB
    STA $27
    RTS
L_FA60:
    LDA $8F
    BEQ L_FA74
    LDA $D4
    BPL L_FA79
    LDA $90
    CMP $91
    BCS L_FA79
    LDA #$00
    STA $90
    STA $8F
L_FA74:
    LDA $D4
    BMI L_FA9E
    RTS
L_FA79:
    LDA $90
    STA $91
    LDA $8F
    ASL A
    TAX
    LDA $8014,X
    STA $D5
    LDA $8015,X
    STA $D6
    LDA #$80
    STA $D4
    LDA $A4
    ORA #$40
    STA $A4
    LDA #$00
    STA $8F
    STA $90
    JMP L_FAA5
L_FA9E:
    DEC $D3
    BEQ L_FAA5
    JMP L_FAF8
L_FAA5:
    LDY #$00
    LDA ($D5),Y
    BEQ L_FAB7
    PHP
    CMP #$FF
    BNE L_FAC6
    PLP
    JSR L_FB8E
    JMP L_FAA5
L_FAB7:
    LDA #$00
    STA $D4
    STA $91
    LDA $A4
    AND #$BF
    STA $A4
    JMP L_FB0F
L_FAC6:
    JSR L_FD6B
    AND #$7F
    STA $D3
    PLP
    BMI L_FAF2
    JSR L_FC81
    LDA #$02
    ORA $27
    STA $27
    LDA $DA
    STA $4005
    LDA $04
    STA $4006
    LDA $05
    AND #$07
    ORA #$C0
    STA $4007
    JSR L_FCC4
    JMP L_FAF8
L_FAF2:
    JSR L_FCDF
    JMP L_FAF8
L_FAF8:
    LDA $27
    AND #$02
    BNE L_FAFF
    RTS
L_FAFF:
    DEC $DD
    BNE L_FB09
    JSR L_FD11
    STA $4004
L_FB09:
    JSR L_FD45
    BCS L_FB0F
    RTS
L_FB0F:
    LDA $D9
    AND #$C0
    ORA #$30
    STA $4004
    LDA $27
    AND #$FD
    STA $27
    RTS
L_FB1F:
    BIT $C4
    BMI L_FB26
    JMP L_FB82
L_FB26:
    DEC $C3
    BEQ L_FB2D
    JMP L_FB6B
L_FB2D:
    LDY #$00
    LDA ($C5),Y
    BEQ L_FB3F
    PHP
    CMP #$FF
    BNE L_FB45
    PLP
    JSR L_FB8E
    JMP L_FB2D
L_FB3F:
    JSR L_FCF9
    JMP L_FB82
L_FB45:
    JSR L_FD6B
    AND #$7F
    STA $C3
    PLP
    BMI L_FB65
    LDA #$08
    ORA $27
    STA $27
    LDA $CA
    STA $400E
    LDA #$80
    STA $400F
    JSR L_FCC4
    JMP L_FB6B
L_FB65:
    JSR L_FCDF
    JMP L_FB6B
L_FB6B:
    LDA $27
    AND #$08
    BNE L_FB72
    RTS
L_FB72:
    DEC $CD
    BNE L_FB7C
    JSR L_FD11
    STA $400C
L_FB7C:
    JSR L_FD45
    BCS L_FB82
    RTS
L_FB82:
    LDA #$30
    STA $400C
    LDA $27
    AND #$F7
    STA $27
    RTS
L_FB8E:
    LDX $02
    JSR L_FD6B
    LDA ($95,X)
    STA $04
    JSR L_FD6B
    LDA ($95,X)
    STA $05
    JSR L_FD6B
    LDA $04
    CMP #$05
    BCC L_FBA8
    RTS
L_FBA8:
    ASL A
    TAX
    LDA $FBBB,X
    STA $06
    LDA $FBBC,X
    STA $07
    LDA $05
    LDX $02
    JMP ($0006)
    .byte $C5,$FB,$E2,$FB,$FF,$FB,$02,$FC,$05,$FC
    PHA
    AND #$F0
    ASL A
    ASL A
    STA $00
    LDA $99,X
    AND #$3F
    ORA $00
    STA $99,X
    PLA
    ASL A
    ASL A
    ASL A
    ASL A
    STA $A2,X
    TAY
    LDA $FDD2,Y
    STA $9A,X
    RTS
    LDA $02
    CMP #$40
    BEQ L_FBEC
    LDA $92
    BNE L_FBFE
L_FBEC:
    LDA #$0F
    CLC
    ADC $05
    SEC
    SBC #$08
    BCS L_FBF8
    LDA #$00
L_FBF8:
    ASL A
    CLC
    ADC #$01
    STA $A0,X
L_FBFE:
    RTS
    .byte $95,$99,$60
    STA $A1,X
    RTS
    STA $9A,X
    RTS
L_FC08:
    LDX #$0A
    LDA $8E
    CMP #$0A
    BCC L_FC12
    LDX #$0C
L_FC12:
    STX $34
    INX
    STX $35
    JSR L_FD87
    LDA #$00
    STA $92
    LDA #$00
    STA $8F
    LDA $8E
    CMP #$0A
    BCC L_FC2B
    SEC
    SBC #$0A
L_FC2B:
    ASL A
    TAX
    LDA $8000,X
    STA $0E
    LDA $8001,X
    STA $0F
    LDA #$93
    STA $0C
    LDA #$00
    STA $0D
    LDX #$04
L_FC41:
    LDY #$07
L_FC43:
    LDA ($0E),Y
    STA ($0C),Y
    DEY
    BPL L_FC43
    CLC
    LDA #$08
    ADC $0C
    STA $0C
    LDA #$00
    ADC $0D
    STA $0D
    LDY #$07
    LDA #$00
L_FC5B:
    STA ($0C),Y
    DEY
    BPL L_FC5B
    CLC
    LDA #$08
    ADC $0C
    STA $0C
    LDA #$00
    ADC $0D
    STA $0D
    CLC
    LDA #$08
    ADC $0E
    STA $0E
    LDA #$00
    ADC $0F
    STA $0F
    DEX
    BNE L_FC41
    JSR L_D41D
    RTS
L_FC81:
    LDX $02
    LDA ($95,X)
    JSR L_FD6B
    TAY
    AND #$0F
    ASL A
    TAX
    LDA $FDB1,X
    STA $04
    LDA $FDB2,X
    STA $05
    LDX $02
    LDA $04
    SEC
    SBC $A1,X
    STA $04
    BCS L_FCA4
    DEC $05
L_FCA4:
    TYA
    LSR A
    LSR A
    LSR A
    LSR A
    BEQ L_FCB3
    TAY
L_FCAC:
    LSR $05
    ROR $04
    DEY
    BNE L_FCAC
L_FCB3:
    RTS
L_FCB4:
    LDA #$00
    INY
L_FCB7:
    CLC
    ADC $00
    DEY
    BNE L_FCB7
    LSR A
    LSR A
    LSR A
    LSR A
    STA $00
    RTS
L_FCC4:
    LDX $02
    LDY $A2,X
    STY $9B,X
    LDA $FDCB,Y
    STA $9C,X
    LDA $FDCC,Y
    STA $9D,X
    LDA $FDCD,Y
    STA $9E,X
    LDA $FDCE,Y
    STA $9F,X
    RTS
L_FCDF:
    LDX $02
    LDA $A2,X
    CLC
    ADC #$0C
    TAY
    STY $9B,X
    LDA $FDCB,Y
    STA $9C,X
    LDA $FDCC,Y
    STA $9D,X
    LDA $FDCD,Y
    STA $9E,X
    RTS
L_FCF9:
    LDX $02
    LDA $97,X
    STA $95,X
    LDA $98,X
    STA $96,X
    BEQ L_FD0A
    LDA #$01
    STA $93,X
    RTS
L_FD0A:
    LDA $94,X
    AND #$40
    STA $94,X
    RTS
L_FD11:
    LDX $02
    LDY $9B,X
    LDA $FDCC,Y
    STA $9D,X
    LDA $9C,X
    BMI L_FD2A
    CLC
    ADC $9F,X
    CMP #$10
    BCC L_FD33
    LDA #$0F
    JMP L_FD33
L_FD2A:
    CLC
    ADC $9F,X
    CMP #$10
    BCC L_FD33
    LDA #$00
L_FD33:
    STA $9F,X
    STA $00
    LDY $A0,X
    JSR L_FCB4
    LDA $99,X
    AND #$C0
    ORA $00
    ORA #$30
    RTS
L_FD45:
    LDX $02
    DEC $9E,X
    BNE L_FD69
    LDA $9B,X
    AND #$0F
    CMP #$0C
    BCS L_FD6A
    LDA $9B,X
    ADC #$04
    TAY
    STY $9B,X
    LDA $FDCB,Y
    STA $9C,X
    LDA $FDCC,Y
    STA $9D,X
    LDA $FDCD,Y
    STA $9E,X
L_FD69:
    CLC
L_FD6A:
    RTS
L_FD6B:
    LDX $02
    INC $95,X
    BNE L_FD73
    INC $96,X
L_FD73:
    RTS
L_FD74:
    LDX #$06
    LDY #$0A
    STX $8000
    STY $8001
    INX
    INY
    STX $8000
    STY $8001
    RTS
L_FD87:
    LDA #$06
    STA $8000
    LDA $34
    STA $8001
    LDA #$07
    STA $8000
    LDA $35
    STA $8001
    RTS
L_FD9C:
    LDA #$06
    STA $8000
    LDA $30
    STA $8001
    LDA #$07
    STA $8000
    LDA $31
    STA $8001
    RTS
    .byte $AE,$06,$4E,$06,$F4,$05,$9E,$05,$4D,$05,$00,$00,$01,$05,$B9,$04
    .byte $75,$04,$35,$04,$F9,$03,$C0,$03,$8A,$03,$00,$01,$01,$0F,$F7,$01
    .byte $01,$00,$FF,$0D,$82,$00,$F9,$01,$38,$00,$00,$01,$01,$0F,$F7,$01
    .byte $01,$00,$FF,$0D,$82,$00,$F9,$01,$38,$00,$00,$01,$01,$0D,$F4,$01
    .byte $01,$03,$F8,$01,$02,$00,$F8,$01,$02,$00,$00,$02,$02,$0F,$F4,$01
    .byte $01,$05,$FF,$06,$18,$00,$FF,$04,$14,$00,$00,$01,$00,$0F,$00,$01
    .byte $00,$00,$00,$01,$00,$00,$F1,$01,$01,$00,$00,$01,$01,$0F,$F7,$01
    .byte $01,$00,$FF,$0A,$96,$00,$FF,$02,$96,$00,$00,$01,$01,$0F,$FA,$01
    .byte $01,$00,$FF,$0D,$82,$00,$F9,$01,$38,$00,$03,$01,$05,$03,$00,$01
    .byte $01,$00,$FF,$0D,$96,$00,$FF,$03,$96,$00,$00,$01,$01,$0F,$F7,$01
    .byte $01,$00,$FF,$0D,$82,$00,$FF,$02,$96,$00,$04,$01,$03,$03,$FE,$01
    .byte $01,$00,$FF,$08,$68,$00,$F9,$01,$38,$00,$06,$01,$02,$06,$FE,$01
    .byte $01,$00,$FF,$08,$68,$00,$FF,$02,$96,$00,$00,$01,$01,$0F,$F7,$01
    .byte $01,$00,$FF,$10,$60,$00,$FF,$02,$96,$00,$00,$00,$01,$00,$FF,$00
    .byte $00,$00,$00,$01,$01,$01,$FF,$01,$00,$01,$00,$FF,$01,$FF,$FF,$FF
    .byte $00,$FF,$00,$00,$01,$00,$FF,$00,$00,$00,$00,$00,$01,$00,$FF,$00
    .byte $00,$00,$00,$10,$01,$10,$FF,$10,$00,$10,$00,$F0,$01,$F0,$FF,$F0
    .byte $00,$F0,$00,$00,$01,$00,$FF,$00,$00,$00,$FD,$FC,$FC,$FC,$FC,$FC
    .byte $FD,$FC,$FC,$FC,$FC,$FC,$FD,$FC,$FC,$FC,$FC,$FC,$FD,$FC,$FC,$FC
    .byte $FC,$FC,$FD,$FC,$FC,$FC,$FC,$FC,$FC,$FD,$FB,$EC,$E9,$E6,$E5,$C0
    .byte $FB,$ED,$E1,$E7,$E9,$E3,$FB,$EB,$E5,$F9,$C0,$C0,$FB,$E7,$EF,$EC
    .byte $E4,$C0,$FB,$E9,$F4,$E5,$ED,$C0,$C0,$FB,$FB,$DD,$DD,$DE,$DF,$DF
    .byte $FB,$DD,$DD,$DD,$DE,$DF,$FB,$DD,$DE,$DF,$DF,$DF,$FB,$DD,$DF,$DF
    .byte $DF,$DF,$FB,$C0,$C0,$C0,$C0,$C0,$C0,$FB,$FB,$DA,$DA,$DA,$DA,$DC
    .byte $FB,$DB,$DF,$DF,$DF,$DF,$FB,$DA,$DA,$DA,$DA,$DB,$FB,$DA,$DA,$DA
    .byte $DB,$DC,$FB,$C0,$C0,$C0,$C0,$C0,$C0,$FB,$FE,$FC,$FC,$FC,$FC,$FC
    .byte $FE,$FC,$FC,$FC,$FC,$FC,$FE,$FC,$FC,$FC,$FC,$FC,$FE,$FC,$FC,$FC
    .byte $FC,$FC,$FE,$FC,$FC,$FC,$FC,$FC,$FC,$FE,$29,$FD,$23,$D0,$9B,$0D
    .byte $00,$64,$9B,$0F,$00,$6C,$9B,$2D,$00,$84,$9B,$2F,$00,$8C,$AB,$43
    .byte $40,$44,$AB,$41,$40,$4C,$BB,$63,$40,$64,$BB,$61,$40,$6C,$BB,$81
    .byte $00,$A4,$BB,$83,$00,$AC,$AB,$AD,$00,$A4,$AB,$AF,$00,$AC,$AB,$CD
    .byte $00,$C4,$AB,$CF,$00,$CC,$12,$03,$01,$10,$14,$02,$02,$18,$14,$01
    .byte $03,$20,$1A,$01,$03,$20,$12,$03,$05,$08,$54,$FC,$8B,$FC,$46,$FF
    .byte $26,$FC,$00,$7C,$0F,$0F,$2A,$36,$0F,$0C,$25,$36,$0F,$0C,$3C,$36
    .byte $0F,$06,$15,$36,$0F,$06,$30,$25,$00,$00,$00,$00,$00,$00,$00
L_FFE0:
    SEI
    LDA #$00
    STA $8000
    STA $A001
    STA $E000
    JMP L_C000
    .byte $00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$00,$FE,$D1,$E0,$FF,$FE
    .byte $D1
