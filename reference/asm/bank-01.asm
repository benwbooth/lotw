; LotW PRG bank 1 (8KB), disassembled at $8000
8000: LDX $B6,Y
8002: LDX $B6,Y
8004: LDX $B6,Y
8006: LDX $B6,Y
8008: LDX $B6,Y
800A: LDX $B6,Y
800C: DEC $CECE
800F: DEC $CECE
8012: DEC $CECE
8015: DEC $C0B6
8018: DEC $CECE
801B: DEC $D4DA
801E: DEC $CECE
8021: DEC $B6B6
8024: DEC $CECE
8027: DEC $CECE
802A: DEC $C5CE
802D: CMP ($B6,X)
802F: LDX $CE,Y
8031: DEC $CECE
8034: .byte $DA
8035: .byte $D4
8036: DEC $CECE
8039: DEC $B6B6
803C: DEC $CECE
803F: DEC $CECE
8042: DEC $CECE
8045: DEC $B6B6
8048: CPY #$C0
804A: CPY #$C0
804C: CPY #$C0
804E: CPY #$C0
8050: CPY #$C0
8052: CPY #$B6
8054: DEC $CECE
8057: DEC $CEB6
805A: DEC $CECE
805D: DEC $B6CE
8060: DEC $CECE
8063: DEC $D4DA
8066: DEC $CECE
8069: DEC $B6CE
806C: DEC $CECE
806F: DEC $CECE
8072: DEC $CECE
8075: DEC $B6CE
8078: DEC $CECE
807B: DEC $D4B6
807E: DEC $CECE
8081: DEC $B6CE
8084: DEC $CECE
8087: DEC $CECE
808A: DEC $CECE
808D: DEC $B6CE
8090: CMP $CDCD
8093: CMP $CDCD
8096: CMP $CDCD
8099: CMP $B6CD
809C: .byte $6F
809D: .byte $6F
809E: .byte $EF
809F: .byte $EF
80A0: LDX $D8,Y
80A2: DEC $CECE
80A5: DEC $B6CE
80A8: CLV
80A9: .byte $6F
80AA: .byte $6F
80AB: .byte $6F
80AC: .byte $6F
80AD: CLD
80AE: DEC $D4DA
80B1: DEC $B6B6
80B4: LDX $DD,Y
80B6: .byte $6F
80B7: .byte $6F
80B8: CMP $E2,X
80BA: DEC $CECE
80BD: DEC $B6B6
80C0: LDX $DE,Y
80C2: CMP $DD,X
80C4: DEC $E3,X
80C6: DEC $CECE
80C9: LDX $CE,Y
80CB: CPY #$B6
80CD: CMP $EAD6,X
80D0: CPY $CCCC
80D3: CPY $B6CC
80D6: CPY $B6B6
80D9: DEC $EBD5,X
80DC: CMP $CDCD
80DF: CMP $CDB6
80E2: CMP $B6B6
80E5: .byte $77
80E6: DEC $DE,X
80E8: .byte $AF
80E9: .byte $AF
80EA: .byte $AF
80EB: .byte $AF
80EC: LDX $AF,Y
80EE: .byte $6F
80EF: LDX $B6,Y
80F1: .byte $77
80F2: .byte $77
80F3: .byte $77
80F4: .byte $AF
80F5: .byte $AF
80F6: .byte $AF
80F7: LDX $AF,Y
80F9: .byte $AF
80FA: .byte $AF
80FB: LDX $B6,Y
80FD: .byte $77
80FE: .byte $AF
80FF: .byte $AF
8100: .byte $AF
8101: .byte $AF
8102: .byte $77
8103: LDX $FB,Y
8105: .byte $FA
8106: .byte $F3
8107: LDX $B6,Y
8109: .byte $77
810A: .byte $77
810B: .byte $AF
810C: .byte $AF
810D: .byte $77
810E: .byte $77
810F: LDX $AF,Y
8111: .byte $AF
8112: .byte $AF
8113: LDX $B6,Y
8115: LDX $77,Y
8117: .byte $77
8118: .byte $2F
8119: .byte $77
811A: LDX $B6,Y
811C: .byte $AF
811D: .byte $AF
811E: .byte $AF
811F: LDX $B6,Y
8121: LDX $77,Y
8123: .byte $03
8124: .byte $2F
8125: .byte $77
8126: LDX $B6,Y
8128: .byte $FB
8129: .byte $FA
812A: .byte $F3
812B: LDX $B6,Y
812D: LDX $77,Y
812F: .byte $77
8130: .byte $2F
8131: .byte $77
8132: LDX $B6,Y
8134: .byte $AF
8135: .byte $AF
8136: .byte $AF
8137: LDX $B6,Y
8139: .byte $77
813A: .byte $77
813B: .byte $AF
813C: .byte $AF
813D: .byte $77
813E: .byte $77
813F: LDX $AF,Y
8141: .byte $AF
8142: .byte $AF
8143: LDX $B6,Y
8145: .byte $77
8146: .byte $AF
8147: .byte $AF
8148: .byte $AF
8149: .byte $AF
814A: .byte $77
814B: LDX $FB,Y
814D: .byte $FA
814E: .byte $F3
814F: LDX $B6,Y
8151: .byte $77
8152: .byte $77
8153: .byte $77
8154: .byte $AF
8155: .byte $AF
8156: .byte $AF
8157: LDX $AF,Y
8159: .byte $AF
815A: .byte $AF
815B: LDX $B6,Y
815D: .byte $77
815E: CMP $DD,X
8160: .byte $AF
8161: .byte $AF
8162: .byte $AF
8163: .byte $AF
8164: LDX $AF,Y
8166: .byte $AF
8167: LDX $B6,Y
8169: CMP $EAD6,X
816C: CPY $CCCC
816F: CPY $CCB6
8172: CPY $B6B6
8175: DEC $EBD5,X
8178: CMP $CDCD
817B: CMP $B6CD
817E: CMP $B6B6
8181: CMP $DED6,X
8184: .byte $AF
8185: CLD
8186: DEC $CECE
8189: LDX $CE,Y
818B: LDX $B6,Y
818D: DEC $DDD5,X
8190: .byte $AF
8191: CLD
8192: DEC $CECE
8195: DEC $B6B6
8198: LDX $6F,Y
819A: DEC $DE,X
819C: .byte $AF
819D: CLD
819E: DEC $D4DA
81A1: DEC $B6B6
81A4: LDX $6F,Y
81A6: .byte $6F
81A7: .byte $6F
81A8: .byte $AF
81A9: CLD
81AA: DEC $CECE
81AD: DEC $B6CE
81B0: LDX $CC,Y
81B2: CPY $CCCC
81B5: CPY $CCCC
81B8: CPY $CCCC
81BB: LDX $B6,Y
81BD: DEC $CECE
81C0: .byte $D3
81C1: .byte $D4
81C2: .byte $D4
81C3: .byte $D4
81C4: DEC $CECE
81C7: LDX $B6,Y
81C9: DEC $CECE
81CC: .byte $D3
81CD: .byte $D4
81CE: .byte $D4
81CF: .byte $D4
81D0: DEC $CECE
81D3: LDX $B6,Y
81D5: DEC $CECE
81D8: .byte $D3
81D9: .byte $D4
81DA: .byte $D4
81DB: .byte $D4
81DC: DEC $B6CE
81DF: LDX $B6,Y
81E1: CMP $CDCD
81E4: CMP $CDCD
81E7: CMP $CDCD
81EA: LDX $B6,Y
81EC: LDX $6F,Y
81EE: .byte $6F
81EF: .byte $6F
81F0: .byte $AF
81F1: CLD
81F2: DEC $CECE
81F5: DEC $B6B6
81F8: LDX $D5,Y
81FA: CMP $AF6F,X
81FD: BNE $81CD
81FF: .byte $DA
8200: .byte $D4
8201: DEC $B6B6
8204: LDX $D6,Y
8206: DEC $DDD5,X
8209: CMP ($CE),Y
820B: DEC $CECE
820E: DEC $B6B6
8211: CMP $DD,X
8213: DEC $EA,X
8215: CPY $CCCC
8218: CPY $CCCC
821B: LDX $B6,Y
821D: DEC $DE,X
821F: CMP $CE,X
8221: DEC $D4D3
8224: .byte $D4
8225: DEC $B6CE
8228: LDX $D5,Y
822A: CMP $CED6,X
822D: DEC $D4D3
8230: .byte $D4
8231: DEC $B6CE
8234: LDX $B6,Y
8236: DEC $EBD5,X
8239: CMP $CDCD
823C: CMP $CDCD
823F: LDX $B6,Y
8241: LDX $6F,Y
8243: DEC $DE,X
8245: .byte $6F
8246: .byte $6F
8247: .byte $6F
8248: .byte $6F
8249: CMP $DD,X
824B: LDX $B6,Y
824D: LDX $6F,Y
824F: .byte $6F
8250: .byte $6F
8251: .byte $6F
8252: .byte $6F
8253: CMP $DD,X
8255: DEC $B6,X
8257: LDX $B6,Y
8259: LDX $B6,Y
825B: LDX $6F,Y
825D: .byte $6F
825E: .byte $6F
825F: DEC $DE,X
8261: .byte $6F
8262: LDX $B6,Y
8264: LDX $B6,Y
8266: .byte $6F
8267: .byte $6F
8268: .byte $6F
8269: .byte $6F
826A: .byte $6F
826B: .byte $6F
826C: .byte $6F
826D: .byte $6F
826E: LDX $B6,Y
8270: LDX $B6,Y
8272: LDX $B6,Y
8274: .byte $6F
8275: .byte $6F
8276: .byte $6F
8277: .byte $6F
8278: .byte $6F
8279: CMP $B6,X
827B: LDX $B6,Y
827D: LDX $B6,Y
827F: LDX $B6,Y
8281: LDX $B6,Y
8283: .byte $6F
8284: .byte $6F
8285: DEC $B6,X
8287: LDX $B6,Y
8289: LDX $B6,Y
828B: LDX $6F,Y
828D: .byte $6F
828E: .byte $6F
828F: .byte $6F
8290: .byte $6F
8291: .byte $6F
8292: LDX $B6,Y
8294: LDX $B6,Y
8296: LDX $B6,Y
8298: LDX $B6,Y
829A: LDX $B6,Y
829C: .byte $6F
829D: LDX $B6,Y
829F: LDX $B6,Y
82A1: LDX $B6,Y
82A3: .byte $6F
82A4: .byte $6F
82A5: .byte $6F
82A6: .byte $6F
82A7: .byte $6F
82A8: .byte $6F
82A9: LDX $B6,Y
82AB: LDX $B6,Y
82AD: LDX $B6,Y
82AF: LDX $B6,Y
82B1: .byte $6F
82B2: .byte $6F
82B3: .byte $6F
82B4: .byte $6F
82B5: LDX $B6,Y
82B7: LDX $B6,Y
82B9: LDX $B6,Y
82BB: LDX $B6,Y
82BD: .byte $6F
82BE: .byte $6F
82BF: .byte $6F
82C0: .byte $6F
82C1: LDA $B6B6,Y
82C4: LDX $B6,Y
82C6: LDX $B6,Y
82C8: LDX $B6,Y
82CA: LDX $6F,Y
82CC: .byte $6F
82CD: .byte $6F
82CE: LDX $B6,Y
82D0: LDX $B6,Y
82D2: LDX $B6,Y
82D4: LDX $B6,Y
82D6: LDX $B6,Y
82D8: .byte $6F
82D9: .byte $6F
82DA: LDX $B6,Y
82DC: LDX $B6,Y
82DE: LDX $B6,Y
82E0: LDX $B6,Y
82E2: LDX $B6,Y
82E4: .byte $6F
82E5: .byte $6F
82E6: LDA $B6B6,Y
82E9: LDX $B6,Y
82EB: LDX $B6,Y
82ED: LDX $B6,Y
82EF: .byte $6F
82F0: .byte $6F
82F1: .byte $6F
82F2: .byte $6F
82F3: LDX $B6,Y
82F5: LDX $B6,Y
82F7: LDX $B6,Y
82F9: LDX $B6,Y
82FB: .byte $6F
82FC: .byte $6F
82FD: .byte $6F
82FE: .byte $6F
82FF: LDX $01,Y
8301: PLP
8302: .byte $0B
8303: BCS $8373
8305: .byte $04
8306: ASL $01
8308: .byte $13
8309: LDY #$0D
830B: BRK
830C: .byte $03
830D: ORA ($27,X)
830F: JSR $C800
8312: BRK
8313: INY
8314: .byte $1F
8315: .byte $03
8316: BRK
8317: BRK
8318: BRK
8319: BRK
831A: BRK
831B: BRK
831C: BRK
831D: BRK
831E: BRK
831F: BRK
8320: ADC ($03,X)
8322: ORA $90
8324: .byte $23
8325: ORA ($6D,X)
8327: .byte $02
8328: .byte $02
8329: ORA ($00,X)
832B: BRK
832C: BRK
832D: BRK
832E: BRK
832F: BRK
8330: ADC ($03,X)
8332: AND $2380,Y
8335: ORA ($6D,X)
8337: .byte $02
8338: .byte $02
8339: ORA ($00,X)
833B: BRK
833C: BRK
833D: BRK
833E: BRK
833F: BRK
8340: ADC ($02),Y
8342: ASL $40,X
8344: ASL $01,X
8346: ADC $0202,X
8349: .byte $02
834A: BRK
834B: BRK
834C: BRK
834D: BRK
834E: BRK
834F: BRK
8350: ADC ($02),Y
8352: .byte $0F
8353: BCC $8361
8355: ORA ($7D,X)
8357: .byte $02
8358: BRK
8359: .byte $02
835A: BRK
835B: BRK
835C: BRK
835D: BRK
835E: BRK
835F: BRK
8360: ADC ($03),Y
8362: ORA #$A0
8364: .byte $0C
8365: ORA ($7D,X)
8367: .byte $02
8368: BRK
8369: .byte $02
836A: BRK
836B: BRK
836C: BRK
836D: BRK
836E: BRK
836F: BRK
8370: ADC ($03),Y
8372: .byte $22
8373: BCC $8381
8375: ORA ($7D,X)
8377: .byte $02
8378: BRK
8379: .byte $02
837A: BRK
837B: BRK
837C: BRK
837D: BRK
837E: BRK
837F: BRK
8380: EOR ($01),Y
8382: .byte $13
8383: LDY #$3C
8385: .byte $03
8386: EOR $0602,X
8389: BRK
838A: BRK
838B: BRK
838C: BRK
838D: BRK
838E: BRK
838F: BRK
8390: ADC ($02),Y
8392: PLP
8393: BCC $83A1
8395: ORA ($7D,X)
8397: .byte $02
8398: BRK
8399: .byte $02
839A: BRK
839B: BRK
839C: BRK
839D: BRK
839E: BRK
839F: BRK
83A0: ADC ($03),Y
83A2: .byte $32
83A3: BCC $83B1
83A5: ORA ($7D,X)
83A7: .byte $02
83A8: .byte $02
83A9: .byte $02
83AA: BRK
83AB: BRK
83AC: BRK
83AD: BRK
83AE: BRK
83AF: BRK
83B0: BRK
83B1: BRK
83B2: BRK
83B3: BRK
83B4: BRK
83B5: BRK
83B6: BRK
83B7: BRK
83B8: BRK
83B9: BRK
83BA: BRK
83BB: BRK
83BC: BRK
83BD: BRK
83BE: BRK
83BF: BRK
83C0: BRK
83C1: BRK
83C2: BRK
83C3: BRK
83C4: BRK
83C5: BRK
83C6: BRK
83C7: BRK
83C8: BRK
83C9: BRK
83CA: BRK
83CB: BRK
83CC: BRK
83CD: BRK
83CE: BRK
83CF: BRK
83D0: BRK
83D1: BRK
83D2: BRK
83D3: BRK
83D4: BRK
83D5: BRK
83D6: BRK
83D7: BRK
83D8: BRK
83D9: BRK
83DA: BRK
83DB: BRK
83DC: BRK
83DD: BRK
83DE: BRK
83DF: BRK
83E0: .byte $0F
83E1: ORA ($05),Y
83E3: BMI $83F4
83E5: ASL A
83E6: ROL A
83E7: AND $120F,Y
83EA: ASL $26,X
83EC: .byte $0F
83ED: BPL $8421
83EF: BMI $8400
83F1: ORA $30
83F3: ROL $0F,X
83F5: ORA $26
83F7: BMI $8408
83F9: ASL A
83FA: AND #$30
83FC: .byte $0F
83FD: .byte $0F
83FE: ASL $30,X
8400: LDX $B6,Y
8402: LDX $B6,Y
8404: LDX $B6,Y
8406: LDX $6F,Y
8408: .byte $6F
8409: .byte $6F
840A: .byte $6F
840B: LDX $B9,Y
840D: .byte $6F
840E: .byte $6F
840F: STA $9D,X
8411: STA $B6,X
8413: .byte $6F
8414: .byte $6F
8415: .byte $6F
8416: INC $6FB6,X
8419: .byte $6F
841A: .byte $6F
841B: STX $8A,Y
841D: .byte $A7
841E: LDX $B6,Y
8420: .byte $6F
8421: .byte $6F
8422: .byte $EF
8423: LDX $6F,Y
8425: STA $9D,X
8427: .byte $AF
8428: STX $8E8E
842B: LDX $B6,Y
842D: LDX $6F,Y
842F: LDX $6F,Y
8431: STX $9E,Y
8433: STA $8E,X
8435: STX $B68E
8438: LDX $B6,Y
843A: .byte $6F
843B: LDX $F3,Y
843D: .byte $77
843E: STA $FB77,X
8441: .byte $FA
8442: .byte $FA
8443: .byte $F3
8444: .byte $77
8445: LDX $6F,Y
8447: LDX $AC,Y
8449: .byte $A7
844A: TXA
844B: .byte $A7
844C: STX $8E8E
844F: STX $B68E
8452: .byte $6F
8453: LDX $8E,Y
8455: STX $8E8E
8458: STX $8E8E
845B: CMP $C1
845D: LDX $6F,Y
845F: LDX $8C,Y
8461: STY $8C8C
8464: STY $8C8C
8467: STY $B68C
846A: .byte $6F
846B: LDX $8D,Y
846D: STA $8D8D
8470: STA $8D8D
8473: STA $B68D
8476: .byte $6F
8477: LDX $8E,Y
8479: STX $8E8E
847C: STX $8E8E
847F: STX $B68E
8482: .byte $6F
8483: LDX $AD,Y
8485: LDX $92
8487: LDX $8E
8489: STX $B68E
848C: LDX $B6,Y
848E: .byte $6F
848F: LDX $6F,Y
8491: .byte $77
8492: .byte $9E
8493: STA $8E,X
8495: STX $B6B6
8498: LDX $6F,Y
849A: .byte $6F
849B: LDX $6F,Y
849D: STA $9D,X
849F: STX $8E,Y
84A1: STX $B6B6
84A4: LDX $6F,Y
84A6: .byte $6F
84A7: LDX $B8,Y
84A9: STX $9E,Y
84AB: STA $8E,X
84AD: STX $B6B6
84B0: .byte $6F
84B1: .byte $6F
84B2: .byte $6F
84B3: LDX $B6,Y
84B5: LDX $B6,Y
84B7: STX $8E,Y
84B9: LDX $B6,Y
84BB: LDX $6F,Y
84BD: .byte $6F
84BE: .byte $6F
84BF: LDX $B6,Y
84C1: LDX $B6,Y
84C3: LDX $B6,Y
84C5: LDX $B6,Y
84C7: LDX $6F,Y
84C9: .byte $6F
84CA: INC $B6B6,X
84CD: .byte $77
84CE: LDX $B6,Y
84D0: LDX $B6,Y
84D2: LDX $B6,Y
84D4: LDX $6F,Y
84D6: .byte $6F
84D7: LDX $B6,Y
84D9: .byte $77
84DA: LDX $B6,Y
84DC: .byte $77
84DD: LDX $B6,Y
84DF: .byte $6F
84E0: .byte $6F
84E1: .byte $6F
84E2: .byte $6F
84E3: LDX $B6,Y
84E5: .byte $77
84E6: LDX $77,Y
84E8: .byte $77
84E9: .byte $6F
84EA: .byte $6F
84EB: .byte $6F
84EC: .byte $6F
84ED: .byte $6F
84EE: .byte $6F
84EF: LDX $B6,Y
84F1: .byte $77
84F2: .byte $77
84F3: .byte $77
84F4: .byte $6F
84F5: .byte $6F
84F6: .byte $6F
84F7: .byte $6F
84F8: .byte $6F
84F9: .byte $6F
84FA: .byte $77
84FB: LDX $B6,Y
84FD: LDX $77,Y
84FF: .byte $77
8500: .byte $6F
8501: .byte $6F
8502: .byte $6F
8503: .byte $6F
8504: .byte $6F
8505: .byte $6F
8506: .byte $77
8507: LDX $B6,Y
8509: LDX $B6,Y
850B: .byte $77
850C: .byte $77
850D: .byte $77
850E: .byte $6F
850F: .byte $6F
8510: .byte $6F
8511: .byte $6F
8512: .byte $77
8513: LDX $B6,Y
8515: LDX $77,Y
8517: .byte $77
8518: .byte $77
8519: .byte $6F
851A: .byte $6F
851B: .byte $6F
851C: .byte $6F
851D: .byte $6F
851E: .byte $77
851F: LDX $B6,Y
8521: LDX $77,Y
8523: .byte $77
8524: .byte $77
8525: .byte $77
8526: .byte $77
8527: .byte $6F
8528: .byte $6F
8529: .byte $77
852A: .byte $77
852B: LDX $B6,Y
852D: .byte $77
852E: .byte $77
852F: .byte $77
8530: .byte $6F
8531: .byte $6F
8532: .byte $6F
8533: .byte $6F
8534: .byte $6F
8535: .byte $77
8536: .byte $77
8537: LDX $B6,Y
8539: .byte $77
853A: .byte $77
853B: .byte $6F
853C: .byte $6F
853D: .byte $6F
853E: .byte $6F
853F: .byte $6F
8540: .byte $6F
8541: .byte $77
8542: .byte $77
8543: LDA $AFB6,Y
8546: .byte $6F
8547: .byte $6F
8548: .byte $6F
8549: .byte $6F
854A: .byte $6F
854B: .byte $6F
854C: .byte $77
854D: .byte $77
854E: .byte $77
854F: .byte $6F
8550: LDX $AF,Y
8552: .byte $6F
8553: .byte $6F
8554: .byte $6F
8555: .byte $6F
8556: .byte $6F
8557: .byte $6F
8558: .byte $77
8559: .byte $77
855A: .byte $6F
855B: .byte $6F
855C: LDX $AF,Y
855E: .byte $77
855F: .byte $6F
8560: .byte $6F
8561: .byte $6F
8562: .byte $6F
8563: .byte $6F
8564: .byte $6F
8565: .byte $77
8566: .byte $6F
8567: .byte $6F
8568: LDX $AF,Y
856A: .byte $77
856B: .byte $77
856C: .byte $6F
856D: .byte $6F
856E: .byte $6F
856F: .byte $6F
8570: .byte $6F
8571: .byte $77
8572: .byte $6F
8573: .byte $6F
8574: LDX $AF,Y
8576: .byte $AF
8577: .byte $77
8578: .byte $77
8579: .byte $6F
857A: .byte $6F
857B: .byte $6F
857C: .byte $6F
857D: .byte $6F
857E: .byte $77
857F: .byte $6F
8580: LDX $77,Y
8582: .byte $AF
8583: .byte $AF
8584: .byte $77
8585: .byte $77
8586: .byte $6F
8587: .byte $6F
8588: .byte $6F
8589: .byte $6F
858A: .byte $6F
858B: .byte $6F
858C: LDX $77,Y
858E: .byte $77
858F: .byte $AF
8590: .byte $AF
8591: .byte $77
8592: .byte $77
8593: .byte $6F
8594: .byte $6F
8595: .byte $6F
8596: .byte $6F
8597: .byte $6F
8598: LDX $77,Y
859A: .byte $77
859B: .byte $77
859C: .byte $AF
859D: .byte $6F
859E: .byte $77
859F: .byte $77
85A0: .byte $6F
85A1: .byte $6F
85A2: .byte $6F
85A3: .byte $6F
85A4: LDX $77,Y
85A6: .byte $77
85A7: .byte $77
85A8: .byte $77
85A9: .byte $6F
85AA: .byte $EF
85AB: .byte $77
85AC: .byte $77
85AD: .byte $6F
85AE: .byte $6F
85AF: .byte $6F
85B0: LDX $AF,Y
85B2: .byte $6F
85B3: .byte $C3
85B4: .byte $6F
85B5: .byte $EF
85B6: .byte $EF
85B7: .byte $EF
85B8: .byte $6F
85B9: .byte $6F
85BA: .byte $6F
85BB: .byte $6F
85BC: LDX $AF,Y
85BE: .byte $77
85BF: .byte $77
85C0: .byte $77
85C1: INC $7777,X
85C4: .byte $77
85C5: .byte $77
85C6: .byte $77
85C7: .byte $77
85C8: LDX $AF,Y
85CA: CPY #$C0
85CC: CPY #$C0
85CE: CPY #$C0
85D0: CPY #$C0
85D2: CPY #$C0
85D4: LDX $F7,Y
85D6: .byte $F7
85D7: .byte $F7
85D8: .byte $F7
85D9: .byte $F7
85DA: .byte $F7
85DB: .byte $F7
85DC: .byte $F7
85DD: .byte $F7
85DE: .byte $F7
85DF: .byte $F7
85E0: LDX $8E,Y
85E2: STX $8E8E
85E5: STX $8E8E
85E8: STX $C08E
85EB: CPY #$B6
85ED: STX $8EF7
85F0: .byte $F7
85F1: STX $8EF7
85F4: STX $F78E
85F7: STX $8EB6
85FA: .byte $F7
85FB: .byte $F7
85FC: .byte $F7
85FD: .byte $F7
85FE: .byte $F7
85FF: .byte $F7
8600: .byte $F7
8601: .byte $F7
8602: .byte $F7
8603: STX $8EB6
8606: STX $8E8E
8609: STX $8E8E
860C: .byte $F7
860D: STX $8E8E
8610: LDX $8E,Y
8612: .byte $F7
8613: STX $8EF7
8616: .byte $F7
8617: STX $F7F7
861A: .byte $F7
861B: .byte $F7
861C: LDX $8E,Y
861E: .byte $F7
861F: .byte $F7
8620: .byte $F7
8621: .byte $F7
8622: .byte $F7
8623: .byte $F7
8624: .byte $F7
8625: STX $8E8E
8628: LDX $8E,Y
862A: STX $8E8E
862D: STX $C1C4
8630: .byte $F7
8631: STX $8E8E
8634: LDX $8E,Y
8636: .byte $F7
8637: STX $8EF7
863A: .byte $F7
863B: STX $8EF7
863E: STX $B68E
8641: STX $F7F7
8644: .byte $F7
8645: .byte $F7
8646: .byte $F7
8647: .byte $F7
8648: .byte $F7
8649: STX $8E8E
864C: LDX $8E,Y
864E: STX $8E8E
8651: STX $8E8E
8654: .byte $F7
8655: STX $F78E
8658: LDX $8E,Y
865A: .byte $F7
865B: STX $8EF7
865E: .byte $F7
865F: STX $8EF7
8662: STX $B68E
8665: STA $F7F7,Y
8668: .byte $F7
8669: .byte $F7
866A: .byte $F7
866B: .byte $F7
866C: .byte $F7
866D: STX $8E8E
8670: LDX $8E,Y
8672: STX $8E8E
8675: STX $8E8E
8678: STX $8E8E
867B: STX $98B9
867E: STY $8C8C
8681: STY $8C8C
8684: STY $8C8C
8687: STY $98AF
868A: STX $8E8E
868D: STX $8E8E
8690: STX $F88E
8693: STX $FAFA
8696: .byte $FA
8697: .byte $F2
8698: .byte $F2
8699: .byte $FA
869A: .byte $FA
869B: .byte $F2
869C: .byte $F3
869D: .byte $C2
869E: INC $FB,X
86A0: .byte $AF
86A1: TYA
86A2: STX $8E8E
86A5: STX $8E8E
86A8: STX $F98E
86AB: STX $98AF
86AE: STX $8E93
86B1: STX $8E93
86B4: STX $8E93
86B7: STX $98B8
86BA: STA $8D8D
86BD: STA $8D8D
86C0: STA $8D8D
86C3: STA $8EB6
86C6: STX $8E8E
86C9: STX $8E8E
86CC: STX $F78E
86CF: STX $F7F7
86D2: .byte $F7
86D3: .byte $F7
86D4: .byte $F7
86D5: .byte $F7
86D6: .byte $F7
86D7: .byte $F7
86D8: STX $8E8E
86DB: STX $C0C0
86DE: CPY #$C0
86E0: CPY #$C0
86E2: CPY #$F7
86E4: STX $F78E
86E7: STX $8EF7
86EA: STX $8E8E
86ED: STX $F78E
86F0: STX $8E8E
86F3: STX $8EF7
86F6: STX $8E8E
86F9: STX $F78E
86FC: STX $F78E
86FF: .byte $F7
8700: ORA ($2D,X)
8702: ASL A
8703: INC $6F,X
8705: .byte $04
8706: ASL $01
8708: ORA $20
870A: ASL $03,X
870C: ORA ($03,X)
870E: .byte $12
870F: BCC $871C
8711: .byte $3C
8712: .byte $0C
8713: .byte $3C
8714: .byte $1F
8715: ORA #$03
8717: BRK
8718: BRK
8719: BRK
871A: BRK
871B: BRK
871C: BRK
871D: BRK
871E: BRK
871F: BRK
8720: EOR ($03,X)
8722: ASL $2010,X
8725: ORA ($4D,X)
8727: .byte $02
8728: ASL $01
872A: BRK
872B: BRK
872C: BRK
872D: BRK
872E: BRK
872F: BRK
8730: EOR ($03,X)
8732: .byte $37
8733: BCC $8767
8735: ORA $4D
8737: .byte $02
8738: ASL $01
873A: BRK
873B: BRK
873C: BRK
873D: BRK
873E: BRK
873F: BRK
8740: EOR ($03,X)
8742: ORA #$80
8744: .byte $32
8745: .byte $03
8746: EOR $0602
8749: ORA ($00,X)
874B: BRK
874C: BRK
874D: BRK
874E: BRK
874F: BRK
8750: EOR ($03),Y
8752: AND #$10
8754: .byte $37
8755: .byte $03
8756: ADC $0201
8759: ORA ($00,X)
875B: BRK
875C: BRK
875D: BRK
875E: BRK
875F: BRK
8760: EOR ($02),Y
8762: BIT $4110
8765: .byte $03
8766: ADC $0201
8769: ORA ($00,X)
876B: BRK
876C: BRK
876D: BRK
876E: BRK
876F: BRK
8770: EOR ($02),Y
8772: .byte $2F
8773: BPL $87B6
8775: .byte $03
8776: ADC $0201
8779: ORA ($00,X)
877B: BRK
877C: BRK
877D: BRK
877E: BRK
877F: BRK
8780: EOR ($03),Y
8782: ASL $90,X
8784: .byte $23
8785: ORA ($6D,X)
8787: ORA ($02,X)
8789: ORA ($00,X)
878B: BRK
878C: BRK
878D: BRK
878E: BRK
878F: BRK
8790: ADC ($01),Y
8792: .byte $0C
8793: LDY #$23
8795: ORA ($7D,X)
8797: .byte $02
8798: .byte $02
8799: .byte $02
879A: BRK
879B: BRK
879C: BRK
879D: BRK
879E: BRK
879F: BRK
87A0: ADC #$01
87A2: .byte $1B
87A3: BVS $87E1
87A5: ASL A
87A6: ADC $00
87A8: ASL $00
87AA: BRK
87AB: BRK
87AC: BRK
87AD: BRK
87AE: BRK
87AF: BRK
87B0: BRK
87B1: BRK
87B2: BRK
87B3: BRK
87B4: BRK
87B5: BRK
87B6: BRK
87B7: BRK
87B8: BRK
87B9: BRK
87BA: BRK
87BB: BRK
87BC: BRK
87BD: BRK
87BE: BRK
87BF: BRK
87C0: BRK
87C1: BRK
87C2: BRK
87C3: BRK
87C4: BRK
87C5: BRK
87C6: BRK
87C7: BRK
87C8: BRK
87C9: BRK
87CA: BRK
87CB: BRK
87CC: BRK
87CD: BRK
87CE: BRK
87CF: BRK
87D0: BRK
87D1: BRK
87D2: BRK
87D3: BRK
87D4: BRK
87D5: BRK
87D6: BRK
87D7: BRK
87D8: BRK
87D9: BRK
87DA: BRK
87DB: BRK
87DC: BRK
87DD: BRK
87DE: BRK
87DF: BRK
87E0: .byte $0F
87E1: ORA ($05),Y
87E3: BMI $87F4
87E5: ASL A
87E6: ROL A
87E7: AND $120F,Y
87EA: ASL $26,X
87EC: .byte $0F
87ED: BPL $8821
87EF: BMI $8800
87F1: ORA $30
87F3: ROL $0F,X
87F5: ORA $26
87F7: BMI $8808
87F9: .byte $02
87FA: BIT $30
87FC: .byte $0F
87FD: .byte $02
87FE: .byte $2B
87FF: BMI $87F8
8801: STX $8E8E
8804: STX $8E8E
8807: .byte $F7
8808: STX $F78E
880B: .byte $F7
880C: .byte $F7
880D: .byte $F7
880E: .byte $F7
880F: .byte $F7
8810: .byte $F7
8811: .byte $F7
8812: .byte $F7
8813: .byte $F7
8814: STX $8E8E
8817: .byte $F7
8818: .byte $F7
8819: .byte $F7
881A: .byte $F7
881B: STX $8E8E
881E: STX $8E8E
8821: STX $F78E
8824: .byte $F7
8825: .byte $FB
8826: .byte $F3
8827: .byte $F7
8828: STX $8EF7
882B: STX $F78E
882E: .byte $F7
882F: .byte $F7
8830: .byte $F7
8831: STX $038E
8834: STX $FBF7
8837: .byte $FA
8838: .byte $F3
8839: .byte $F7
883A: STX $F7C0
883D: .byte $FB
883E: .byte $F3
883F: .byte $F7
8840: STX $8EF7
8843: STX $F78E
8846: STX $F7F7
8849: .byte $F7
884A: .byte $F7
884B: STX $8E8E
884E: STX $8E8E
8851: .byte $F7
8852: STX $F7F7
8855: STX $8E8E
8858: STX $F7F7
885B: STX $F78E
885E: STX $F7F7
8861: STY $8C8C
8864: STY $8C8C
8867: STY $F78C
886A: STY $F7F7
886D: STX $F78E
8870: STX $FE8E
8873: STX $F78E
8876: STX $F7F7
8879: STX $8E8E
887C: STX $8E8E
887F: STX $F7FE
8882: STX $F7F7
8885: STA $8D8D
8888: STA $8D8D
888B: STA $F78D
888E: STA $F7F7
8891: .byte $AF
8892: .byte $AF
8893: TYA
8894: STX $8E8E
8897: STX $F78E
889A: STX $F7F7
889D: .byte $AF
889E: .byte $AF
889F: TYA
88A0: STX $8E93
88A3: .byte $93
88A4: STX $8EF7
88A7: .byte $F7
88A8: .byte $F7
88A9: .byte $AF
88AA: .byte $AF
88AB: .byte $F7
88AC: STX $8E93
88AF: .byte $93
88B0: STX $8EF7
88B3: .byte $F7
88B4: .byte $F7
88B5: .byte $AF
88B6: .byte $AF
88B7: TYA
88B8: STX $8E8E
88BB: STX $F78E
88BE: STX $F7F7
88C1: .byte $AF
88C2: TYA
88C3: STY $8C8C
88C6: STY $8C8C
88C9: .byte $F7
88CA: STY $F7F7
88CD: .byte $AF
88CE: TYA
88CF: STX $8E93
88D2: INC $F7F7,X
88D5: .byte $F7
88D6: STX $F7F7
88D9: .byte $AF
88DA: TYA
88DB: STX $8E8E
88DE: STX $8EF7
88E1: STX $F78E
88E4: .byte $F7
88E5: .byte $AF
88E6: TYA
88E7: STX $8E93
88EA: STX $8EF7
88ED: CMP $81
88EF: .byte $F7
88F0: .byte $F7
88F1: .byte $AF
88F2: TYA
88F3: STX $8E8E
88F6: STX $8EF7
88F9: STX $F78E
88FC: .byte $F7
88FD: .byte $AF
88FE: TYA
88FF: STX $8E93
8902: STX $F7F7
8905: .byte $F7
8906: .byte $F7
8907: .byte $F7
8908: .byte $F7
8909: .byte $AF
890A: TYA
890B: STA $8D8D
890E: STA $8D8D
8911: .byte $F7
8912: .byte $F7
8913: .byte $F7
8914: .byte $F7
8915: .byte $AF
8916: .byte $AF
8917: .byte $AF
8918: .byte $AF
8919: TYA
891A: STX $F7F7
891D: .byte $F7
891E: .byte $F7
891F: .byte $F7
8920: CPY #$C0
8922: CPY #$F7
8924: .byte $F7
8925: .byte $F7
8926: .byte $F7
8927: .byte $F7
8928: STX $F78E
892B: .byte $F7
892C: .byte $F7
892D: .byte $F7
892E: .byte $F7
892F: .byte $F7
8930: .byte $AF
8931: TYA
8932: .byte $F7
8933: STX $FE8E
8936: STX $F7F7
8939: .byte $F7
893A: .byte $F7
893B: .byte $AF
893C: .byte $AF
893D: TYA
893E: STX $8E8E
8941: STX $8E8E
8944: CPY #$C0
8946: CPY #$C0
8948: CPY #$C0
894A: CPY #$C0
894C: CPY #$C0
894E: CPY #$C0
8950: .byte $77
8951: .byte $77
8952: .byte $77
8953: .byte $77
8954: .byte $77
8955: .byte $77
8956: .byte $77
8957: .byte $77
8958: .byte $77
8959: .byte $77
895A: .byte $77
895B: .byte $77
895C: .byte $77
895D: INC $FEFE,X
8960: INC $FEFE,X
8963: INC $FEFE,X
8966: INC $77C0,X
8969: INC $FEFE,X
896C: .byte $6F
896D: TYA
896E: STY $8C8C
8971: STY $778C
8974: .byte $6F
8975: INC $FEFE,X
8978: .byte $6F
8979: TYA
897A: STX $938E
897D: STX $778E
8980: .byte $6F
8981: INC $FEFE,X
8984: .byte $6F
8985: BCC $8915
8987: STX $8E8E
898A: STX $7777
898D: INC $95FE,X
8990: STA $8E91,X
8993: STX $8E8E
8996: INC $7777,X
8999: INC $96FE,X
899C: TAX
899D: STY $8C8C
89A0: STY $8C8C
89A3: .byte $77
89A4: .byte $77
89A5: INC $95FE,X
89A8: .byte $AB
89A9: STA $8D8D
89AC: STA $8D8D
89AF: .byte $77
89B0: .byte $77
89B1: INC $96FE,X
89B4: .byte $9E
89B5: STA $9D,X
89B7: STX $A3,Y
89B9: STX $778E
89BC: .byte $77
89BD: INC $95FE,X
89C0: STA $9E96,X
89C3: STA $A2,X
89C5: STX $778E
89C8: .byte $77
89C9: .byte $77
89CA: .byte $77
89CB: .byte $77
89CC: .byte $77
89CD: .byte $77
89CE: .byte $77
89CF: STX $77,Y
89D1: .byte $77
89D2: .byte $77
89D3: .byte $77
89D4: .byte $77
89D5: .byte $6F
89D6: .byte $6F
89D7: .byte $6F
89D8: .byte $6F
89D9: .byte $6F
89DA: .byte $6F
89DB: .byte $6F
89DC: .byte $6F
89DD: .byte $6F
89DE: .byte $6F
89DF: .byte $77
89E0: .byte $77
89E1: .byte $6F
89E2: .byte $77
89E3: .byte $77
89E4: .byte $77
89E5: .byte $77
89E6: .byte $77
89E7: .byte $77
89E8: .byte $77
89E9: .byte $77
89EA: .byte $77
89EB: .byte $77
89EC: .byte $77
89ED: .byte $6F
89EE: .byte $6F
89EF: .byte $6F
89F0: .byte $6F
89F1: CPY #$C0
89F3: CPY #$C0
89F5: CPY #$C0
89F7: .byte $77
89F8: .byte $77
89F9: .byte $77
89FA: .byte $77
89FB: .byte $77
89FC: .byte $77
89FD: .byte $77
89FE: .byte $77
89FF: .byte $77
8A00: .byte $77
8A01: .byte $77
8A02: .byte $6F
8A03: .byte $77
8A04: .byte $77
8A05: .byte $6F
8A06: .byte $6F
8A07: .byte $6F
8A08: .byte $6F
8A09: .byte $6F
8A0A: .byte $6F
8A0B: .byte $6F
8A0C: .byte $6F
8A0D: .byte $6F
8A0E: .byte $6F
8A0F: .byte $77
8A10: .byte $77
8A11: .byte $6F
8A12: .byte $77
8A13: .byte $77
8A14: .byte $77
8A15: .byte $77
8A16: .byte $77
8A17: .byte $77
8A18: .byte $77
8A19: .byte $77
8A1A: .byte $77
8A1B: .byte $77
8A1C: .byte $77
8A1D: .byte $6F
8A1E: .byte $6F
8A1F: .byte $6F
8A20: .byte $6F
8A21: CPY #$C0
8A23: CPY #$C0
8A25: CPY #$C0
8A27: .byte $77
8A28: .byte $77
8A29: .byte $77
8A2A: .byte $77
8A2B: .byte $77
8A2C: .byte $77
8A2D: .byte $77
8A2E: .byte $77
8A2F: .byte $77
8A30: .byte $77
8A31: .byte $77
8A32: .byte $6F
8A33: .byte $77
8A34: .byte $77
8A35: ADC $41,X
8A37: .byte $77
8A38: .byte $77
8A39: .byte $77
8A3A: .byte $77
8A3B: .byte $77
8A3C: .byte $77
8A3D: .byte $77
8A3E: .byte $6F
8A3F: .byte $77
8A40: .byte $77
8A41: .byte $6F
8A42: .byte $6F
8A43: .byte $6F
8A44: .byte $6F
8A45: .byte $6F
8A46: .byte $6F
8A47: .byte $6F
8A48: .byte $6F
8A49: .byte $77
8A4A: .byte $6F
8A4B: .byte $77
8A4C: .byte $77
8A4D: .byte $77
8A4E: .byte $77
8A4F: .byte $77
8A50: .byte $77
8A51: .byte $77
8A52: .byte $6F
8A53: .byte $77
8A54: .byte $77
8A55: .byte $77
8A56: .byte $6F
8A57: .byte $77
8A58: .byte $77
8A59: .byte $6F
8A5A: .byte $6F
8A5B: .byte $6F
8A5C: .byte $6F
8A5D: .byte $6F
8A5E: .byte $6F
8A5F: .byte $6F
8A60: .byte $6F
8A61: .byte $77
8A62: .byte $6F
8A63: .byte $77
8A64: .byte $77
8A65: .byte $6F
8A66: .byte $77
8A67: .byte $77
8A68: .byte $77
8A69: .byte $77
8A6A: .byte $77
8A6B: .byte $77
8A6C: .byte $6F
8A6D: .byte $77
8A6E: .byte $6F
8A6F: .byte $77
8A70: .byte $77
8A71: .byte $6F
8A72: .byte $77
8A73: .byte $6F
8A74: .byte $6F
8A75: .byte $6F
8A76: .byte $6F
8A77: .byte $6F
8A78: .byte $6F
8A79: .byte $77
8A7A: .byte $6F
8A7B: .byte $77
8A7C: .byte $77
8A7D: .byte $6F
8A7E: .byte $77
8A7F: .byte $6F
8A80: CPY #$C0
8A82: CPY #$C0
8A84: CPY #$77
8A86: .byte $6F
8A87: .byte $77
8A88: .byte $77
8A89: .byte $6F
8A8A: .byte $77
8A8B: .byte $6F
8A8C: .byte $77
8A8D: .byte $77
8A8E: .byte $77
8A8F: .byte $77
8A90: .byte $77
8A91: .byte $77
8A92: .byte $6F
8A93: .byte $77
8A94: .byte $77
8A95: .byte $6F
8A96: .byte $77
8A97: .byte $6F
8A98: .byte $6F
8A99: .byte $6F
8A9A: .byte $6F
8A9B: .byte $6F
8A9C: .byte $6F
8A9D: .byte $77
8A9E: .byte $6F
8A9F: .byte $77
8AA0: .byte $77
8AA1: .byte $6F
8AA2: .byte $77
8AA3: .byte $77
8AA4: .byte $77
8AA5: .byte $77
8AA6: .byte $77
8AA7: .byte $77
8AA8: .byte $6F
8AA9: .byte $77
8AAA: .byte $6F
8AAB: .byte $77
8AAC: .byte $77
8AAD: .byte $6F
8AAE: .byte $77
8AAF: .byte $6F
8AB0: .byte $6F
8AB1: .byte $6F
8AB2: .byte $6F
8AB3: .byte $6F
8AB4: .byte $6F
8AB5: .byte $77
8AB6: .byte $6F
8AB7: .byte $77
8AB8: .byte $77
8AB9: .byte $6F
8ABA: .byte $77
8ABB: .byte $6F
8ABC: CPY #$C0
8ABE: CPY #$C0
8AC0: CPY #$77
8AC2: .byte $6F
8AC3: .byte $77
8AC4: .byte $77
8AC5: .byte $6F
8AC6: .byte $77
8AC7: .byte $6F
8AC8: .byte $77
8AC9: .byte $77
8ACA: .byte $77
8ACB: .byte $77
8ACC: .byte $77
8ACD: .byte $77
8ACE: .byte $6F
8ACF: .byte $77
8AD0: .byte $77
8AD1: .byte $6F
8AD2: .byte $77
8AD3: .byte $6F
8AD4: .byte $6F
8AD5: .byte $6F
8AD6: .byte $6F
8AD7: .byte $6F
8AD8: .byte $6F
8AD9: .byte $6F
8ADA: .byte $6F
8ADB: .byte $77
8ADC: .byte $77
8ADD: .byte $6F
8ADE: .byte $77
8ADF: .byte $77
8AE0: .byte $77
8AE1: .byte $77
8AE2: .byte $77
8AE3: .byte $77
8AE4: .byte $77
8AE5: .byte $77
8AE6: .byte $77
8AE7: .byte $77
8AE8: .byte $77
8AE9: .byte $6F
8AEA: .byte $77
8AEB: .byte $77
8AEC: .byte $77
8AED: .byte $77
8AEE: .byte $6F
8AEF: .byte $6F
8AF0: .byte $6F
8AF1: .byte $77
8AF2: .byte $77
8AF3: .byte $77
8AF4: .byte $77
8AF5: .byte $6F
8AF6: .byte $77
8AF7: .byte $77
8AF8: .byte $77
8AF9: .byte $77
8AFA: .byte $6F
8AFB: .byte $6F
8AFC: .byte $6F
8AFD: .byte $77
8AFE: .byte $77
8AFF: .byte $77
8B00: ORA ($2C,X)
8B02: AND $C4,X
8B04: .byte $2F
8B05: .byte $04
8B06: ASL $01
8B08: AND $A0
8B0A: BRK
8B0B: .byte $03
8B0C: .byte $03
8B0D: .byte $0C
8B0E: ASL $0640,X
8B11: BVC $8B1E
8B13: .byte $14
8B14: PHP
8B15: PHP
8B16: BRK
8B17: BRK
8B18: BRK
8B19: BRK
8B1A: BRK
8B1B: BRK
8B1C: BRK
8B1D: BRK
8B1E: BRK
8B1F: BRK
8B20: EOR ($03,X)
8B22: .byte $0C
8B23: LDY #$23
8B25: .byte $02
8B26: EOR $0201
8B29: .byte $02
8B2A: BRK
8B2B: BRK
8B2C: BRK
8B2D: BRK
8B2E: BRK
8B2F: BRK
8B30: EOR ($03,X)
8B32: ORA $2380
8B35: .byte $02
8B36: EOR $0201
8B39: .byte $02
8B3A: BRK
8B3B: BRK
8B3C: BRK
8B3D: BRK
8B3E: BRK
8B3F: BRK
8B40: EOR ($03,X)
8B42: AND $A0,X
8B44: .byte $23
8B45: .byte $02
8B46: EOR $0201
8B49: .byte $02
8B4A: BRK
8B4B: BRK
8B4C: BRK
8B4D: BRK
8B4E: BRK
8B4F: BRK
8B50: EOR ($02,X)
8B52: .byte $12
8B53: LDY #$23
8B55: .byte $02
8B56: EOR $0201
8B59: .byte $02
8B5A: BRK
8B5B: BRK
8B5C: BRK
8B5D: BRK
8B5E: BRK
8B5F: BRK
8B60: EOR ($02,X)
8B62: ORA $60,X
8B64: .byte $23
8B65: ORA ($4D,X)
8B67: ORA ($02,X)
8B69: .byte $03
8B6A: BRK
8B6B: BRK
8B6C: BRK
8B6D: BRK
8B6E: BRK
8B6F: BRK
8B70: EOR ($02,X)
8B72: AND ($A0),Y
8B74: .byte $23
8B75: .byte $02
8B76: EOR $0201
8B79: .byte $02
8B7A: BRK
8B7B: BRK
8B7C: BRK
8B7D: BRK
8B7E: BRK
8B7F: BRK
8B80: ADC ($02,X)
8B82: ROL $10,X
8B84: AND $6D04
8B87: .byte $02
8B88: .byte $03
8B89: .byte $02
8B8A: BRK
8B8B: BRK
8B8C: BRK
8B8D: BRK
8B8E: BRK
8B8F: BRK
8B90: ADC ($01,X)
8B92: AND $2D80,Y
8B95: .byte $03
8B96: ADC $0302
8B99: .byte $02
8B9A: BRK
8B9B: BRK
8B9C: BRK
8B9D: BRK
8B9E: BRK
8B9F: BRK
8BA0: ADC ($03,X)
8BA2: .byte $33
8BA3: .byte $80
8BA4: AND $6D03
8BA7: .byte $02
8BA8: .byte $03
8BA9: .byte $02
8BAA: BRK
8BAB: BRK
8BAC: BRK
8BAD: BRK
8BAE: BRK
8BAF: BRK
8BB0: BRK
8BB1: BRK
8BB2: BRK
8BB3: BRK
8BB4: BRK
8BB5: BRK
8BB6: BRK
8BB7: BRK
8BB8: BRK
8BB9: BRK
8BBA: BRK
8BBB: BRK
8BBC: BRK
8BBD: BRK
8BBE: BRK
8BBF: BRK
8BC0: BRK
8BC1: BRK
8BC2: BRK
8BC3: BRK
8BC4: BRK
8BC5: BRK
8BC6: BRK
8BC7: BRK
8BC8: BRK
8BC9: BRK
8BCA: BRK
8BCB: BRK
8BCC: BRK
8BCD: BRK
8BCE: BRK
8BCF: BRK
8BD0: BRK
8BD1: BRK
8BD2: BRK
8BD3: BRK
8BD4: BRK
8BD5: BRK
8BD6: BRK
8BD7: BRK
8BD8: BRK
8BD9: BRK
8BDA: BRK
8BDB: BRK
8BDC: BRK
8BDD: BRK
8BDE: BRK
8BDF: BRK
8BE0: .byte $0F
8BE1: ORA ($05),Y
8BE3: BMI $8BF4
8BE5: ORA $24,X
8BE7: .byte $34
8BE8: .byte $0F
8BE9: .byte $12
8BEA: ASL $26,X
8BEC: .byte $0F
8BED: BPL $8C21
8BEF: BMI $8C00
8BF1: ORA $30
8BF3: ROL $0F,X
8BF5: ORA $26
8BF7: BMI $8C08
8BF9: CLC
8BFA: SEC
8BFB: BMI $8C0C
8BFD: CLC
8BFE: ROL A
8BFF: BMI $8C78
8C01: .byte $6F
8C02: .byte $77
8C03: .byte $77
8C04: .byte $77
8C05: .byte $77
8C06: .byte $6F
8C07: .byte $6F
8C08: .byte $6F
8C09: .byte $77
8C0A: .byte $77
8C0B: .byte $77
8C0C: .byte $77
8C0D: .byte $6F
8C0E: .byte $AF
8C0F: .byte $6F
8C10: .byte $6F
8C11: .byte $6F
8C12: .byte $6F
8C13: .byte $6F
8C14: .byte $6F
8C15: .byte $6F
8C16: .byte $77
8C17: .byte $77
8C18: .byte $77
8C19: INC $AFAF,X
8C1C: .byte $77
8C1D: ORA $1D,X
8C1F: .byte $6F
8C20: .byte $6F
8C21: SBC $77,X
8C23: .byte $77
8C24: .byte $77
8C25: INC $AFFE,X
8C28: .byte $77
8C29: ASL $1E,X
8C2B: .byte $6F
8C2C: .byte $6F
8C2D: .byte $6F
8C2E: SBC $77,X
8C30: .byte $77
8C31: INC $156F,X
8C34: ORA $6F6F,X
8C37: ORA $1D,X
8C39: .byte $6F
8C3A: SBC $77,X
8C3C: .byte $77
8C3D: .byte $6F
8C3E: .byte $6F
8C3F: ASL $1E,X
8C41: .byte $6F
8C42: .byte $77
8C43: ASL $1E,X
8C45: .byte $6F
8C46: SBC $77,X
8C48: .byte $77
8C49: ORA $1D,X
8C4B: ORA $1D,X
8C4D: .byte $6F
8C4E: .byte $6F
8C4F: .byte $6F
8C50: .byte $AF
8C51: .byte $AF
8C52: SBC $77,X
8C54: .byte $77
8C55: ASL $1E,X
8C57: ASL $1E,X
8C59: .byte $6F
8C5A: .byte $6F
8C5B: .byte $6F
8C5C: .byte $6F
8C5D: .byte $AF
8C5E: SBC $77,X
8C60: .byte $77
8C61: ORA $1D,X
8C63: ORA $1D,X
8C65: .byte $6F
8C66: .byte $6F
8C67: ORA $1D,X
8C69: .byte $6F
8C6A: SBC $77,X
8C6C: .byte $77
8C6D: INC $FEFE,X
8C70: INC $6F77,X
8C73: ASL $1E,X
8C75: SBC $F5,X
8C77: .byte $77
8C78: .byte $77
8C79: ORA $1D,X
8C7B: ORA $1D,X
8C7D: ORA $1D,X
8C7F: .byte $6F
8C80: SBC $F5,X
8C82: SBC $77,X
8C84: .byte $77
8C85: ASL $1E,X
8C87: ASL $1E,X
8C89: ASL $1E,X
8C8B: .byte $6F
8C8C: .byte $6F
8C8D: SBC $F5,X
8C8F: .byte $77
8C90: .byte $77
8C91: .byte $AF
8C92: .byte $AF
8C93: ORA $1D,X
8C95: .byte $6F
8C96: INC $6F77,X
8C99: .byte $6F
8C9A: SBC $77,X
8C9C: .byte $77
8C9D: .byte $AF
8C9E: .byte $EF
8C9F: ASL $1E,X
8CA1: .byte $6F
8CA2: .byte $6F
8CA3: .byte $6F
8CA4: .byte $6F
8CA5: .byte $6F
8CA6: SBC $77,X
8CA8: .byte $77
8CA9: ORA $1D,X
8CAB: ORA $1D,X
8CAD: .byte $6F
8CAE: .byte $6F
8CAF: .byte $6F
8CB0: INC $6F77,X
8CB3: .byte $77
8CB4: .byte $77
8CB5: ASL $1E,X
8CB7: ASL $1E,X
8CB9: .byte $6F
8CBA: .byte $6F
8CBB: .byte $6F
8CBC: .byte $6F
8CBD: .byte $6F
8CBE: .byte $6F
8CBF: .byte $77
8CC0: .byte $77
8CC1: .byte $AF
8CC2: DEY
8CC3: STY $8C8C
8CC6: STY $8C8C
8CC9: STY $778C
8CCC: .byte $77
8CCD: .byte $AF
8CCE: STX $8E8E
8CD1: .byte $77
8CD2: .byte $77
8CD3: .byte $77
8CD4: .byte $77
8CD5: .byte $77
8CD6: .byte $77
8CD7: CPY #$77
8CD9: .byte $AF
8CDA: .byte $89
8CDB: STA $8D8D
8CDE: STX $8E8E
8CE1: STX $8E8E
8CE4: .byte $77
8CE5: .byte $AF
8CE6: .byte $AF
8CE7: ASL $1E,X
8CE9: TYA
8CEA: STX $8E8E
8CED: STX $7777
8CF0: .byte $77
8CF1: INC $1577,X
8CF4: ORA $8E98,X
8CF7: TXS
8CF8: STY $8E,X
8CFA: SBC $77,X
8CFC: .byte $77
8CFD: .byte $AF
8CFE: .byte $AF
8CFF: ASL $1E,X
8D01: TYA
8D02: STX $949A
8D05: STX $77F5
8D08: .byte $77
8D09: INC $77FE,X
8D0C: .byte $AF
8D0D: TYA
8D0E: STX $8E8E
8D11: STX $7777
8D14: .byte $77
8D15: .byte $AF
8D16: .byte $AF
8D17: .byte $AF
8D18: .byte $AF
8D19: TYA
8D1A: STX $8E8E
8D1D: STX $77F5
8D20: .byte $77
8D21: INC $77FE,X
8D24: DEY
8D25: STY $8C8C
8D28: STY $F58C
8D2B: .byte $77
8D2C: .byte $77
8D2D: .byte $AF
8D2E: .byte $AF
8D2F: TYA
8D30: STX $8E8E
8D33: STX $8E8E
8D36: .byte $77
8D37: .byte $77
8D38: .byte $77
8D39: INC $77FE,X
8D3C: STX $8E8E
8D3F: STX $8E8E
8D42: SBC $77,X
8D44: .byte $77
8D45: .byte $AF
8D46: .byte $AF
8D47: TYA
8D48: STX $9A8E
8D4B: STY $8E,X
8D4D: STX $77F5
8D50: .byte $77
8D51: INC $77FE,X
8D54: STX $9A8E
8D57: STY $77,X
8D59: .byte $77
8D5A: .byte $77
8D5B: .byte $77
8D5C: .byte $77
8D5D: .byte $AF
8D5E: .byte $AF
8D5F: TYA
8D60: STX $8E8E
8D63: STX $8E8E
8D66: SBC $77,X
8D68: .byte $77
8D69: .byte $AF
8D6A: .byte $AF
8D6B: TYA
8D6C: STX $8E77
8D6F: STX $8E8E
8D72: STX $7777
8D75: .byte $77
8D76: .byte $77
8D77: .byte $77
8D78: .byte $77
8D79: .byte $77
8D7A: .byte $77
8D7B: .byte $77
8D7C: .byte $77
8D7D: STX $778E
8D80: STX $8E8E
8D83: STX $8185
8D86: .byte $77
8D87: STX $8E8E
8D8A: STX $7777
8D8D: STX $778E
8D90: .byte $77
8D91: .byte $77
8D92: .byte $77
8D93: STX $8E8E
8D96: .byte $77
8D97: .byte $77
8D98: .byte $77
8D99: STX $8E8E
8D9C: STX $8E8E
8D9F: STX $778E
8DA2: .byte $77
8DA3: .byte $F7
8DA4: .byte $77
8DA5: STX $8E8E
8DA8: STX $8E8E
8DAB: STX $77F5
8DAE: .byte $77
8DAF: .byte $F7
8DB0: .byte $77
8DB1: .byte $77
8DB2: .byte $77
8DB3: .byte $77
8DB4: .byte $77
8DB5: .byte $77
8DB6: .byte $77
8DB7: .byte $77
8DB8: .byte $77
8DB9: .byte $77
8DBA: .byte $F7
8DBB: .byte $6F
8DBC: .byte $F7
8DBD: .byte $F7
8DBE: .byte $F7
8DBF: .byte $F7
8DC0: .byte $83
8DC1: .byte $AF
8DC2: .byte $F7
8DC3: .byte $77
8DC4: .byte $77
8DC5: .byte $77
8DC6: .byte $F7
8DC7: .byte $6F
8DC8: CPY #$C0
8DCA: CPY #$F7
8DCC: .byte $F7
8DCD: .byte $AF
8DCE: .byte $F7
8DCF: .byte $F7
8DD0: .byte $F7
8DD1: .byte $77
8DD2: .byte $6F
8DD3: .byte $6F
8DD4: .byte $F7
8DD5: .byte $6F
8DD6: .byte $6F
8DD7: .byte $F7
8DD8: .byte $F7
8DD9: .byte $6F
8DDA: .byte $F7
8DDB: .byte $F7
8DDC: .byte $EF
8DDD: .byte $EF
8DDE: .byte $6F
8DDF: .byte $6F
8DE0: .byte $F7
8DE1: .byte $6F
8DE2: .byte $6F
8DE3: .byte $F7
8DE4: .byte $6F
8DE5: .byte $6F
8DE6: .byte $6F
8DE7: .byte $F7
8DE8: .byte $EF
8DE9: .byte $EF
8DEA: .byte $6F
8DEB: .byte $6F
8DEC: .byte $F7
8DED: .byte $6F
8DEE: .byte $6F
8DEF: .byte $F7
8DF0: .byte $6F
8DF1: .byte $6F
8DF2: .byte $6F
8DF3: .byte $F7
8DF4: .byte $EF
8DF5: .byte $EF
8DF6: .byte $6F
8DF7: .byte $6F
8DF8: .byte $F7
8DF9: .byte $6F
8DFA: .byte $6F
8DFB: .byte $F7
8DFC: .byte $6F
8DFD: .byte $6F
8DFE: .byte $6F
8DFF: .byte $F7
8E00: .byte $EF
8E01: .byte $EF
8E02: .byte $6F
8E03: .byte $6F
8E04: .byte $F7
8E05: .byte $F7
8E06: .byte $6F
8E07: .byte $F7
8E08: ROR $EFF7,X
8E0B: .byte $F7
8E0C: .byte $EF
8E0D: .byte $EF
8E0E: .byte $6F
8E0F: .byte $6F
8E10: .byte $F7
8E11: .byte $F7
8E12: .byte $6F
8E13: .byte $F7
8E14: .byte $6F
8E15: .byte $F7
8E16: .byte $EF
8E17: .byte $F7
8E18: .byte $EF
8E19: .byte $EF
8E1A: .byte $6F
8E1B: .byte $6F
8E1C: .byte $F7
8E1D: .byte $F7
8E1E: .byte $6F
8E1F: .byte $F7
8E20: .byte $6F
8E21: .byte $F7
8E22: .byte $EF
8E23: .byte $F7
8E24: .byte $EF
8E25: .byte $EF
8E26: .byte $6F
8E27: .byte $6F
8E28: .byte $F7
8E29: .byte $F7
8E2A: ROR $6FF7,X
8E2D: .byte $F7
8E2E: .byte $EF
8E2F: .byte $F7
8E30: .byte $EF
8E31: .byte $EF
8E32: .byte $6F
8E33: .byte $6F
8E34: .byte $F7
8E35: .byte $F7
8E36: .byte $6F
8E37: .byte $F7
8E38: .byte $6F
8E39: .byte $F7
8E3A: .byte $EF
8E3B: .byte $F7
8E3C: .byte $EF
8E3D: .byte $EF
8E3E: .byte $6F
8E3F: .byte $6F
8E40: .byte $F7
8E41: .byte $F7
8E42: .byte $6F
8E43: .byte $F7
8E44: .byte $6F
8E45: .byte $F7
8E46: .byte $EF
8E47: .byte $F7
8E48: .byte $EF
8E49: .byte $EF
8E4A: .byte $6F
8E4B: .byte $6F
8E4C: .byte $F7
8E4D: .byte $6F
8E4E: .byte $6F
8E4F: .byte $F7
8E50: .byte $6F
8E51: .byte $F7
8E52: .byte $EF
8E53: .byte $F7
8E54: .byte $EF
8E55: .byte $EF
8E56: .byte $6F
8E57: .byte $6F
8E58: .byte $F7
8E59: .byte $6F
8E5A: .byte $6F
8E5B: .byte $F7
8E5C: .byte $EF
8E5D: .byte $F7
8E5E: ROR $EFF7,X
8E61: .byte $EF
8E62: .byte $6F
8E63: .byte $6F
8E64: .byte $F7
8E65: .byte $6F
8E66: .byte $6F
8E67: .byte $F7
8E68: ROR $EFF7,X
8E6B: .byte $F7
8E6C: .byte $EF
8E6D: .byte $EF
8E6E: .byte $6F
8E6F: .byte $6F
8E70: .byte $F7
8E71: .byte $6F
8E72: .byte $6F
8E73: .byte $F7
8E74: .byte $EF
8E75: .byte $F7
8E76: .byte $EF
8E77: .byte $F7
8E78: .byte $EF
8E79: .byte $EF
8E7A: .byte $6F
8E7B: .byte $6F
8E7C: .byte $F7
8E7D: .byte $6F
8E7E: .byte $F7
8E7F: .byte $EF
8E80: .byte $6F
8E81: .byte $F7
8E82: .byte $EF
8E83: .byte $F7
8E84: .byte $EF
8E85: .byte $EF
8E86: .byte $6F
8E87: .byte $6F
8E88: .byte $F7
8E89: .byte $6F
8E8A: .byte $F7
8E8B: .byte $EF
8E8C: .byte $F7
8E8D: .byte $EF
8E8E: .byte $EF
8E8F: .byte $F7
8E90: .byte $EF
8E91: .byte $EF
8E92: .byte $6F
8E93: .byte $6F
8E94: .byte $F7
8E95: .byte $6F
8E96: .byte $F7
8E97: .byte $EF
8E98: .byte $F7
8E99: .byte $EF
8E9A: .byte $F7
8E9B: .byte $EF
8E9C: .byte $EF
8E9D: .byte $EF
8E9E: .byte $6F
8E9F: .byte $6F
8EA0: .byte $F7
8EA1: .byte $6F
8EA2: .byte $F7
8EA3: .byte $EF
8EA4: .byte $F7
8EA5: .byte $EF
8EA6: .byte $F7
8EA7: .byte $EF
8EA8: .byte $EF
8EA9: .byte $EF
8EAA: .byte $6F
8EAB: .byte $6F
8EAC: .byte $F7
8EAD: .byte $6F
8EAE: .byte $F7
8EAF: .byte $EF
8EB0: .byte $F7
8EB1: .byte $EF
8EB2: .byte $F7
8EB3: .byte $EF
8EB4: .byte $EF
8EB5: .byte $6F
8EB6: .byte $6F
8EB7: .byte $F7
8EB8: .byte $F7
8EB9: ROR $6FF7,X
8EBC: .byte $F7
8EBD: .byte $EF
8EBE: .byte $F7
8EBF: .byte $EF
8EC0: .byte $6F
8EC1: .byte $6F
8EC2: .byte $F7
8EC3: .byte $6F
8EC4: .byte $F7
8EC5: .byte $6F
8EC6: .byte $F7
8EC7: .byte $6F
8EC8: .byte $F7
8EC9: .byte $EF
8ECA: .byte $6F
8ECB: .byte $6F
8ECC: .byte $F7
8ECD: .byte $6F
8ECE: .byte $F7
8ECF: .byte $6F
8ED0: .byte $F7
8ED1: .byte $6F
8ED2: .byte $6F
8ED3: ROR $EFF7,X
8ED6: .byte $6F
8ED7: .byte $6F
8ED8: .byte $6F
8ED9: .byte $6F
8EDA: ADC $6F,X
8EDC: .byte $F7
8EDD: .byte $6F
8EDE: .byte $6F
8EDF: .byte $F7
8EE0: .byte $F7
8EE1: .byte $EF
8EE2: .byte $6F
8EE3: .byte $6F
8EE4: .byte $6F
8EE5: ADC $6F,X
8EE7: .byte $6F
8EE8: .byte $F7
8EE9: .byte $6F
8EEA: .byte $6F
8EEB: .byte $F7
8EEC: .byte $6F
8EED: .byte $6F
8EEE: .byte $6F
8EEF: .byte $6F
8EF0: .byte $6F
8EF1: .byte $F7
8EF2: .byte $F7
8EF3: .byte $6F
8EF4: .byte $F7
8EF5: .byte $F7
8EF6: .byte $F7
8EF7: .byte $F7
8EF8: .byte $F7
8EF9: .byte $F7
8EFA: .byte $F7
8EFB: .byte $F7
8EFC: .byte $F7
8EFD: .byte $F7
8EFE: .byte $F7
8EFF: .byte $F7
8F00: ORA ($2B,X)
8F02: AND $F0,X
8F04: .byte $6F
8F05: .byte $04
8F06: ASL $01
8F08: ORA $50
8F0A: BRK
8F0B: .byte $03
8F0C: ORA ($01,X)
8F0E: SEC
8F0F: BPL $8F11
8F11: INY
8F12: BRK
8F13: INY
8F14: PHP
8F15: PHP
8F16: BRK
8F17: BRK
8F18: BRK
8F19: BRK
8F1A: BRK
8F1B: BRK
8F1C: BRK
8F1D: BRK
8F1E: BRK
8F1F: BRK
8F20: EOR ($03,X)
8F22: ASL $90
8F24: AND $4D01
8F27: .byte $02
8F28: BRK
8F29: ORA ($00,X)
8F2B: BRK
8F2C: BRK
8F2D: BRK
8F2E: BRK
8F2F: BRK
8F30: EOR ($03,X)
8F32: ROL $20,X
8F34: AND $4D01
8F37: .byte $02
8F38: BRK
8F39: ORA ($00,X)
8F3B: BRK
8F3C: BRK
8F3D: BRK
8F3E: BRK
8F3F: BRK
8F40: EOR ($02),Y
8F42: ASL $23A0,X
8F45: ORA ($5D,X)
8F47: .byte $03
8F48: .byte $02
8F49: ORA $00
8F4B: BRK
8F4C: BRK
8F4D: BRK
8F4E: BRK
8F4F: BRK
8F50: EOR ($02,X)
8F52: ORA $2D90,Y
8F55: .byte $02
8F56: EOR $0002
8F59: .byte $02
8F5A: BRK
8F5B: BRK
8F5C: BRK
8F5D: BRK
8F5E: BRK
8F5F: BRK
8F60: EOR ($02,X)
8F62: BMI $8FC4
8F64: AND $4D02
8F67: .byte $02
8F68: BRK
8F69: .byte $02
8F6A: BRK
8F6B: BRK
8F6C: BRK
8F6D: BRK
8F6E: BRK
8F6F: BRK
8F70: EOR ($03),Y
8F72: .byte $0F
8F73: LDY #$23
8F75: .byte $02
8F76: EOR ($03),Y
8F78: .byte $02
8F79: ORA $00
8F7B: BRK
8F7C: BRK
8F7D: BRK
8F7E: BRK
8F7F: BRK
8F80: ADC ($03),Y
8F82: .byte $34
8F83: BPL $8FAD
8F85: .byte $03
8F86: ADC $0502,X
8F89: ORA $00
8F8B: BRK
8F8C: BRK
8F8D: BRK
8F8E: BRK
8F8F: BRK
8F90: ADC ($03,X)
8F92: AND #$20
8F94: PLP
8F95: .byte $02
8F96: ADC $0302
8F99: .byte $02
8F9A: BRK
8F9B: BRK
8F9C: BRK
8F9D: BRK
8F9E: BRK
8F9F: BRK
8FA0: ADC ($02,X)
8FA2: .byte $27
8FA3: BVC $8FCD
8FA5: .byte $02
8FA6: ADC $0302
8FA9: .byte $02
8FAA: BRK
8FAB: BRK
8FAC: BRK
8FAD: BRK
8FAE: BRK
8FAF: BRK
8FB0: BRK
8FB1: BRK
8FB2: BRK
8FB3: BRK
8FB4: BRK
8FB5: BRK
8FB6: BRK
8FB7: BRK
8FB8: BRK
8FB9: BRK
8FBA: BRK
8FBB: BRK
8FBC: BRK
8FBD: BRK
8FBE: BRK
8FBF: BRK
8FC0: BRK
8FC1: BRK
8FC2: BRK
8FC3: BRK
8FC4: BRK
8FC5: BRK
8FC6: BRK
8FC7: BRK
8FC8: BRK
8FC9: BRK
8FCA: BRK
8FCB: BRK
8FCC: BRK
8FCD: BRK
8FCE: BRK
8FCF: BRK
8FD0: BRK
8FD1: BRK
8FD2: BRK
8FD3: BRK
8FD4: BRK
8FD5: BRK
8FD6: BRK
8FD7: BRK
8FD8: BRK
8FD9: BRK
8FDA: BRK
8FDB: BRK
8FDC: BRK
8FDD: BRK
8FDE: BRK
8FDF: BRK
8FE0: .byte $0F
8FE1: ORA ($05),Y
8FE3: BMI $8FF4
8FE5: ORA $24,X
8FE7: .byte $34
8FE8: .byte $0F
8FE9: ROL A
8FEA: CLC
8FEB: ROL $0F,X
8FED: BPL $9021
8FEF: BMI $9000
8FF1: ORA $30
8FF3: ROL $0F,X
8FF5: ORA $26
8FF7: BMI $9008
8FF9: .byte $02
8FFA: ROL $30
8FFC: .byte $0F
8FFD: .byte $02
8FFE: ROL A
8FFF: BMI $8FBD
9001: LDY $BCBC,X
9004: LDY $BCBC,X
9007: LDY $BCBC,X
900A: LDY $C0BC,X
900D: CPY #$C0
900F: .byte $52
9010: .byte $52
9011: LDY $BCBC,X
9014: .byte $CB
9015: .byte $CB
9016: .byte $CB
9017: LDA $BC
9019: LDY $5252,X
901C: INC $5252,X
901F: LDY $C4C9,X
9022: CMP ($BC,X)
9024: LDY $5252,X
9027: .byte $7C
9028: .byte $52
9029: .byte $7C
902A: .byte $52
902B: LDY $CACA,X
902E: DEX
902F: LDY $52BC,X
9032: .byte $52
9033: .byte $7C
9034: INC $5252,X
9037: LDY $6F52,X
903A: LDY $BCBC,X
903D: .byte $52
903E: .byte $7C
903F: .byte $7C
9040: .byte $52
9041: LDY $BCBC,X
9044: .byte $52
9045: .byte $52
9046: .byte $52
9047: LDY $6FBC,X
904A: .byte $52
904B: .byte $7C
904C: INC $BCBC,X
904F: LDY $BCBC,X
9052: .byte $EF
9053: LDY $BCBC,X
9056: .byte $52
9057: .byte $7C
9058: .byte $6F
9059: .byte $52
905A: INC $5252,X
905D: .byte $52
905E: .byte $52
905F: LDY $BCBC,X
9062: .byte $6F
9063: .byte $7C
9064: .byte $52
9065: BEQ $90DD
9067: .byte $52
9068: ROR $76,X
906A: .byte $52
906B: LDY $BCBC,X
906E: .byte $52
906F: .byte $EF
9070: .byte $52
9071: .byte $77
9072: .byte $77
9073: ROR $77,X
9075: .byte $77
9076: INC $BCBC,X
9079: LDY $FEFE,X
907C: .byte $52
907D: ROR $52,X
907F: .byte $77
9080: .byte $52
9081: .byte $52
9082: .byte $52
9083: LDY $BCBC,X
9086: .byte $6F
9087: LDY $7752,X
908A: .byte $52
908B: .byte $6F
908C: .byte $52
908D: .byte $6F
908E: .byte $52
908F: LDY $6FBC,X
9092: INC $6F52,X
9095: LDY $52BC,X
9098: .byte $52
9099: .byte $52
909A: ROR $BC,X
909C: LDY $BC52,X
909F: LDY $BCBC,X
90A2: LDY $6F52,X
90A5: ROR $77,X
90A7: LDY $52BC,X
90AA: LDY $BCBC,X
90AD: LDY $52BC,X
90B0: ROR $77,X
90B2: ROR $BC,X
90B4: LDY $BC52,X
90B7: .byte $52
90B8: INC $BCEF,X
90BB: .byte $52
90BC: .byte $77
90BD: ROR $77,X
90BF: LDY $EFA5,X
90C2: LDY $5252,X
90C5: .byte $52
90C6: LDA $52
90C8: INC $5277,X
90CB: LDY $52BC,X
90CE: LDY $BC52,X
90D1: INC $52BC,X
90D4: .byte $52
90D5: .byte $6F
90D6: INC $BCBC,X
90D9: .byte $6F
90DA: LDY $BC52,X
90DD: .byte $52
90DE: LDY $BCBC,X
90E1: .byte $EF
90E2: .byte $52
90E3: LDY $52BC,X
90E6: LDY $5252,X
90E9: INC $5252,X
90EC: LDY $5252,X
90EF: LDY $6FBC,X
90F2: LDY $52BC,X
90F5: .byte $52
90F6: LDA $6F
90F8: LDY $5252,X
90FB: RTI
90FC: LDY $BC52,X
90FF: .byte $52
9100: .byte $52
9101: LDY $52BC,X
9104: LDY $BCBC,X
9107: LDY $52BC,X
910A: .byte $52
910B: INC $526F,X
910E: .byte $52
910F: INC $5252,X
9112: .byte $52
9113: LDY $52BC,X
9116: LDY $BCFE,X
9119: LDY $5252,X
911C: LDA $52
911E: .byte $52
911F: LDY $52BC,X
9122: LDY $52FE,X
9125: .byte $52
9126: .byte $52
9127: LDY $BCBC,X
912A: .byte $EF
912B: LDY $52BC,X
912E: LDY $BC52,X
9131: .byte $52
9132: LDY $BCBC,X
9135: LDY $BC6F,X
9138: LDY $BCBC,X
913B: .byte $52
913C: .byte $52
913D: .byte $52
913E: LDY $FE52,X
9141: .byte $52
9142: .byte $52
9143: LDY $52BC,X
9146: .byte $52
9147: .byte $EF
9148: .byte $52
9149: LDY $52BC,X
914C: LDY $5252,X
914F: LDY $52BC,X
9152: .byte $52
9153: LDY $BCBC,X
9156: LDY $5252,X
9159: INC $BCBC,X
915C: LDY $4105,X
915F: LDY $BCBC,X
9162: LDY $BCBC,X
9165: .byte $64
9166: LDY $BCBC,X
9169: LDY $BC62,X
916C: LDY $628D,X
916F: LSR $5E5E,X
9172: LSR $BCC0,X
9175: LDY $BC64,X
9178: LDY $628D,X
917B: LSR $5E5E,X
917E: LSR $BCBC,X
9181: .byte $62
9182: .byte $63
9183: LDY $8DBC,X
9186: .byte $64
9187: LSR $5E5E,X
918A: LSR $BCBC,X
918D: .byte $62
918E: LDY $60BC,X
9191: ADC ($61,X)
9193: ADC ($61,X)
9195: ADC ($61,X)
9197: LDY $62BC,X
919A: LDY $CCBC,X
919D: CPY $CCCC
91A0: CPY $CCCC
91A3: LDY $80BC,X
91A6: ADC ($5F,X)
91A8: LSR $B65E,X
91AB: .byte $62
91AC: LSR $5E5E,X
91AF: LDY $BCBC,X
91B2: LDY $6160,X
91B5: ADC ($B7,X)
91B7: RTS
91B8: ADC ($61,X)
91BA: ADC ($BC,X)
91BC: LDY $BCBC,X
91BF: CPY $CCCC
91C2: CPY $CCB6
91C5: CPY $BCCC
91C8: LDY $5EBC,X
91CB: .byte $63
91CC: .byte $5F
91CD: LSR $B75E,X
91D0: .byte $64
91D1: LSR $BC5E,X
91D4: LDY $6160,X
91D7: LDX $60,Y
91D9: ADC ($61,X)
91DB: ADC ($61,X)
91DD: ADC ($61,X)
91DF: LDY $CCBC,X
91E2: CPY $CCB7
91E5: CPY $CCCC
91E8: CPY $CCCC
91EB: LDY $6FBC,X
91EE: .byte $6F
91EF: .byte $6F
91F0: .byte $6F
91F1: .byte $6F
91F2: INC $67B6,X
91F5: PHA
91F6: PHA
91F7: LDY $6FBC,X
91FA: .byte $6F
91FB: .byte $6F
91FC: .byte $6F
91FD: .byte $6F
91FE: .byte $6F
91FF: .byte $B7
9200: .byte $67
9201: PHA
9202: PHA
9203: LDY $6FBC,X
9206: .byte $6F
9207: LDX $6F,Y
9209: .byte $6F
920A: .byte $6F
920B: .byte $6F
920C: .byte $67
920D: PHA
920E: PHA
920F: LDY $6FBC,X
9212: .byte $6F
9213: .byte $B7
9214: .byte $6F
9215: .byte $6F
9216: .byte $6F
9217: .byte $6F
9218: LDX $48,Y
921A: PHA
921B: LDY $6FBC,X
921E: .byte $6F
921F: .byte $6F
9220: .byte $6F
9221: .byte $6F
9222: ADC $B748
9225: PHA
9226: PHA
9227: LDY $6FBC,X
922A: ROR $6E
922C: LDX $6E,Y
922E: .byte $57
922F: PHA
9230: PHA
9231: PHA
9232: PHA
9233: LDY $6FBC,X
9236: .byte $6F
9237: ADC $48B7
923A: PHA
923B: PHA
923C: PHA
923D: LDX $48,Y
923F: LDY $6FBC,X
9242: .byte $6F
9243: ADC $4848
9246: PHA
9247: PHA
9248: PHA
9249: .byte $B7
924A: PHA
924B: LDY $6FBC,X
924E: .byte $6F
924F: ADC $B648
9252: PHA
9253: PHA
9254: PHA
9255: PHA
9256: PHA
9257: LDY $6FBC,X
925A: .byte $6F
925B: ADC $B748
925E: PHA
925F: PHA
9260: PHA
9261: PHA
9262: PHA
9263: LDY $66BC,X
9266: ROR $576E
9269: PHA
926A: PHA
926B: PHA
926C: PHA
926D: PHA
926E: PHA
926F: LDY $6FBC,X
9272: .byte $6F
9273: .byte $6F
9274: .byte $6F
9275: .byte $6F
9276: LDX $6D,Y
9278: PHA
9279: PHA
927A: INC $BCBC,X
927D: .byte $6F
927E: ROR $48
9280: PHA
9281: PHA
9282: .byte $B7
9283: PHA
9284: PHA
9285: PHA
9286: PHA
9287: LDY $CCBC,X
928A: CPY $CCCC
928D: CPY $CCCC
9290: CPY $CCCC
9293: LDY $6FBC,X
9296: ROR $6E
9298: ROR $6E6E
929B: LDX $67,Y
929D: PHA
929E: PHA
929F: LDY $BCBC,X
92A2: .byte $6F
92A3: .byte $6F
92A4: .byte $6F
92A5: .byte $6F
92A6: .byte $6F
92A7: .byte $B7
92A8: .byte $67
92A9: PHA
92AA: PHA
92AB: LDY $BCBC,X
92AE: CPY $CCCC
92B1: CPY $CCCC
92B4: CPY $CCCC
92B7: LDY $BCBC,X
92BA: LDY $6F6F,X
92BD: .byte $6F
92BE: .byte $6F
92BF: .byte $6F
92C0: LDX $48,Y
92C2: PHA
92C3: LDY $BCBC,X
92C6: LDY $6F6F,X
92C9: .byte $6F
92CA: .byte $6F
92CB: .byte $6F
92CC: .byte $B7
92CD: PHA
92CE: PHA
92CF: LDY $BCBC,X
92D2: LDY $CCBC,X
92D5: CPY $CCCC
92D8: CPY $CCCC
92DB: LDY $BCBC,X
92DE: LDY $BCBC,X
92E1: .byte $6F
92E2: .byte $6F
92E3: .byte $6F
92E4: .byte $6F
92E5: .byte $67
92E6: LDY $BCBC,X
92E9: LDY $BCBC,X
92EC: LDY $6F6F,X
92EF: .byte $6F
92F0: .byte $6F
92F1: .byte $67
92F2: LDY $BCBC,X
92F5: LDY $BCBC,X
92F8: LDY $6FBC,X
92FB: .byte $6F
92FC: .byte $6F
92FD: .byte $67
92FE: LDY $00BC,X
9301: BIT $25
9303: .byte $52
9304: .byte $52
9305: .byte $04
9306: ASL $01
9308: AND $60
930A: .byte $04
930B: ORA ($00,X)
930D: BRK
930E: BRK
930F: BRK
9310: .byte $0B
9311: EOR ($0C,X)
9313: EOR ($01,X)
9315: .byte $02
9316: BRK
9317: BRK
9318: BRK
9319: BRK
931A: BRK
931B: BRK
931C: BRK
931D: BRK
931E: BRK
931F: BRK
9320: EOR ($03,X)
9322: AND $50,X
9324: .byte $23
9325: ORA ($4D,X)
9327: .byte $02
9328: BRK
9329: .byte $02
932A: BRK
932B: BRK
932C: BRK
932D: BRK
932E: BRK
932F: BRK
9330: EOR ($03,X)
9332: ORA $23A0,Y
9335: ORA ($4D,X)
9337: .byte $02
9338: .byte $02
9339: .byte $02
933A: BRK
933B: BRK
933C: BRK
933D: BRK
933E: BRK
933F: BRK
9340: EOR ($03),Y
9342: .byte $13
9343: BPL $9365
9345: ORA ($5D,X)
9347: .byte $02
9348: .byte $02
9349: .byte $02
934A: BRK
934B: BRK
934C: BRK
934D: BRK
934E: BRK
934F: BRK
9350: ADC ($03),Y
9352: BRK
9353: BRK
9354: JSR $7D01
9357: ORA ($01,X)
9359: ORA $00
935B: BRK
935C: BRK
935D: BRK
935E: BRK
935F: BRK
9360: ADC ($03),Y
9362: .byte $32
9363: BVS $9383
9365: ORA ($7D,X)
9367: ORA ($04,X)
9369: ORA $00
936B: BRK
936C: BRK
936D: BRK
936E: BRK
936F: BRK
9370: ADC ($03),Y
9372: BRK
9373: BRK
9374: .byte $0C
9375: ORA ($7D,X)
9377: ORA ($01,X)
9379: ORA $00
937B: BRK
937C: BRK
937D: BRK
937E: BRK
937F: BRK
9380: ADC ($03,X)
9382: SEC
9383: LDY #$1E
9385: ORA ($6D,X)
9387: .byte $02
9388: BRK
9389: .byte $02
938A: BRK
938B: BRK
938C: BRK
938D: BRK
938E: BRK
938F: BRK
9390: ADC ($03,X)
9392: AND $A0
9394: ASL $6D01,X
9397: .byte $02
9398: BRK
9399: .byte $02
939A: BRK
939B: BRK
939C: BRK
939D: BRK
939E: BRK
939F: BRK
93A0: ADC ($03,X)
93A2: .byte $04
93A3: BCC $93C3
93A5: ORA ($6D,X)
93A7: .byte $02
93A8: BRK
93A9: .byte $02
93AA: BRK
93AB: BRK
93AC: BRK
93AD: BRK
93AE: BRK
93AF: BRK
93B0: BRK
93B1: BRK
93B2: BRK
93B3: BRK
93B4: BRK
93B5: BRK
93B6: BRK
93B7: BRK
93B8: BRK
93B9: BRK
93BA: BRK
93BB: BRK
93BC: BRK
93BD: BRK
93BE: BRK
93BF: BRK
93C0: BRK
93C1: BRK
93C2: BRK
93C3: BRK
93C4: BRK
93C5: BRK
93C6: BRK
93C7: BRK
93C8: BRK
93C9: BRK
93CA: BRK
93CB: BRK
93CC: BRK
93CD: BRK
93CE: BRK
93CF: BRK
93D0: BRK
93D1: BRK
93D2: BRK
93D3: BRK
93D4: BRK
93D5: BRK
93D6: BRK
93D7: BRK
93D8: BRK
93D9: BRK
93DA: BRK
93DB: BRK
93DC: BRK
93DD: BRK
93DE: BRK
93DF: BRK
93E0: .byte $0F
93E1: ORA ($05),Y
93E3: BMI $93F4
93E5: .byte $02
93E6: .byte $1C
93E7: .byte $3C
93E8: .byte $0F
93E9: .byte $07
93EA: ROL $38
93EC: .byte $0F
93ED: BPL $9421
93EF: BMI $9400
93F1: ORA $30
93F3: ROL $0F,X
93F5: ORA $26
93F7: BMI $9408
93F9: ASL $26
93FB: BMI $940C
93FD: .byte $0B
93FE: AND #$30
9400: SBC $F9F9,Y
9403: SBC $F9F9,Y
9406: .byte $6F
9407: .byte $6F
9408: .byte $6F
9409: .byte $67
940A: SBC $F9F9,Y
940D: .byte $6F
940E: .byte $6F
940F: .byte $6F
9410: .byte $6F
9411: .byte $6F
9412: .byte $6F
9413: .byte $6F
9414: .byte $6F
9415: .byte $67
9416: RTI
9417: RTI
9418: SBC $6F6F,Y
941B: .byte $6F
941C: .byte $4B
941D: .byte $4B
941E: .byte $4B
941F: .byte $4B
9420: .byte $4B
9421: .byte $4B
9422: SBC $F9F9,Y
9425: .byte $6F
9426: .byte $6F
9427: .byte $6F
9428: EOR #$49
942A: EOR #$49
942C: EOR #$F9
942E: SBC $F9F9,Y
9431: .byte $6F
9432: .byte $6F
9433: .byte $6F
9434: LSR A
9435: LSR A
9436: LSR A
9437: LSR A
9438: SBC $F9F9,Y
943B: SBC $6FF9,Y
943E: .byte $6F
943F: .byte $6F
9440: .byte $5A
9441: EOR #$5D
9443: SBC $5CF9,Y
9446: SBC $F9F9,Y
9449: .byte $6F
944A: .byte $6F
944B: .byte $6F
944C: .byte $5A
944D: EOR $F9F9,X
9450: .byte $5C
9451: EOR $F9F9,X
9454: SBC $6F6F,Y
9457: .byte $6F
9458: .byte $5A
9459: SBC $5CF9,Y
945C: EOR #$F9
945E: SBC $F9F9,Y
9461: .byte $6F
9462: .byte $6F
9463: .byte $6F
9464: SBC $5AF9,Y
9467: CLI
9468: CLI
9469: CLI
946A: CLI
946B: SBC $4BF9,Y
946E: .byte $4B
946F: SBC $4BF9,Y
9472: .byte $4B
9473: .byte $4B
9474: .byte $4B
9475: .byte $4B
9476: .byte $4B
9477: SBC $49F9,Y
947A: SBC $49F9,Y
947D: EOR #$49
947F: EOR #$49
9481: EOR #$49
9483: SBC $4AF9,Y
9486: SBC $4A4A,Y
9489: LSR A
948A: LSR A
948B: LSR A
948C: LSR A
948D: LSR A
948E: LSR A
948F: SBC $6FF9,Y
9492: SBC $6FEF,Y
9495: .byte $6F
9496: .byte $6F
9497: .byte $6F
9498: .byte $6F
9499: .byte $67
949A: PHA
949B: SBC $6FF9,Y
949E: .byte $6F
949F: SBC $666F,Y
94A2: ROR $576E
94A5: PHA
94A6: PHA
94A7: SBC $6FF9,Y
94AA: .byte $6F
94AB: SBC $6F6F,Y
94AE: .byte $6F
94AF: .byte $6F
94B0: .byte $57
94B1: PHA
94B2: PHA
94B3: SBC $6FF9,Y
94B6: .byte $6F
94B7: .byte $6F
94B8: ROR $6E
94BA: ROR $576E
94BD: PHA
94BE: PHA
94BF: SBC $6FF9,Y
94C2: .byte $6F
94C3: .byte $6F
94C4: SBC $6F6F,Y
94C7: .byte $6F
94C8: .byte $57
94C9: PHA
94CA: PHA
94CB: SBC $6FF9,Y
94CE: .byte $6F
94CF: ROR $F9
94D1: ROR $4857
94D4: PHA
94D5: PHA
94D6: PHA
94D7: SBC $6FF9,Y
94DA: .byte $6F
94DB: .byte $6F
94DC: .byte $6F
94DD: .byte $6F
94DE: .byte $57
94DF: PHA
94E0: PHA
94E1: PHA
94E2: PHA
94E3: SBC $6FF9,Y
94E6: .byte $6F
94E7: .byte $6F
94E8: .byte $6F
94E9: SBC $4857,Y
94EC: PHA
94ED: PHA
94EE: PHA
94EF: SBC $6FF9,Y
94F2: .byte $6F
94F3: .byte $6F
94F4: .byte $6F
94F5: SBC $6F6F,Y
94F8: ADC $4848
94FB: SBC $6FF9,Y
94FE: .byte $6F
94FF: ROR $6E
9501: ROR $6E6E
9504: ROR $4857
9507: SBC $4BF9,Y
950A: .byte $4B
950B: .byte $4B
950C: .byte $4B
950D: .byte $4B
950E: SBC $4B4B,Y
9511: .byte $4B
9512: .byte $4B
9513: SBC $49F9,Y
9516: EOR #$49
9518: EOR #$49
951A: SBC $4949,Y
951D: .byte $44
951E: EOR ($F9,X)
9520: SBC $4A4A,Y
9523: LSR A
9524: LSR A
9525: LSR A
9526: CPY #$C0
9528: CPY #$C0
952A: CPY #$F9
952C: SBC $6F6F,Y
952F: .byte $6F
9530: CLI
9531: CLI
9532: CLI
9533: CLI
9534: CLI
9535: CLI
9536: SBC $F9F9,Y
9539: SBC $F9F9,Y
953C: SBC $F9F9,Y
953F: SBC $F9F9,Y
9542: SBC $AFF9,Y
9545: .byte $AF
9546: .byte $AF
9547: .byte $AF
9548: .byte $AF
9549: .byte $AF
954A: .byte $AF
954B: .byte $AF
954C: .byte $AF
954D: .byte $AF
954E: .byte $AF
954F: .byte $AF
9550: .byte $AF
9551: .byte $AF
9552: .byte $AF
9553: .byte $AF
9554: .byte $AF
9555: .byte $AF
9556: .byte $AF
9557: .byte $AF
9558: .byte $AF
9559: .byte $AF
955A: .byte $AF
955B: .byte $AF
955C: .byte $AF
955D: .byte $AF
955E: .byte $AF
955F: .byte $AF
9560: .byte $AF
9561: .byte $AF
9562: .byte $AF
9563: .byte $AF
9564: .byte $AF
9565: .byte $AF
9566: .byte $AF
9567: .byte $AF
9568: .byte $AF
9569: .byte $AF
956A: .byte $AF
956B: .byte $AF
956C: .byte $AF
956D: .byte $AF
956E: .byte $AF
956F: .byte $AF
9570: .byte $AF
9571: .byte $AF
9572: .byte $AF
9573: .byte $AF
9574: .byte $AF
9575: .byte $AF
9576: .byte $AF
9577: .byte $AF
9578: .byte $AF
9579: .byte $AF
957A: .byte $AF
957B: .byte $AF
957C: .byte $AF
957D: .byte $AF
957E: .byte $AF
957F: .byte $AF
9580: .byte $AF
9581: .byte $AF
9582: .byte $AF
9583: .byte $AF
9584: .byte $AF
9585: .byte $AF
9586: .byte $AF
9587: .byte $AF
9588: .byte $AF
9589: .byte $AF
958A: .byte $AF
958B: .byte $AF
958C: .byte $AF
958D: .byte $AF
958E: .byte $AF
958F: .byte $AF
9590: .byte $AF
9591: .byte $AF
9592: .byte $AF
9593: .byte $AF
9594: .byte $AF
9595: .byte $AF
9596: .byte $AF
9597: .byte $AF
9598: .byte $AF
9599: .byte $AF
959A: .byte $AF
959B: .byte $AF
959C: .byte $AF
959D: .byte $AF
959E: .byte $AF
959F: .byte $AF
95A0: .byte $AF
95A1: .byte $AF
95A2: .byte $AF
95A3: .byte $AF
95A4: .byte $AF
95A5: .byte $AF
95A6: .byte $AF
95A7: .byte $AF
95A8: .byte $AF
95A9: .byte $AF
95AA: .byte $AF
95AB: .byte $AF
95AC: .byte $AF
95AD: .byte $AF
95AE: .byte $AF
95AF: .byte $AF
95B0: .byte $AF
95B1: .byte $AF
95B2: .byte $AF
95B3: .byte $AF
95B4: .byte $AF
95B5: .byte $AF
95B6: .byte $AF
95B7: .byte $AF
95B8: .byte $AF
95B9: .byte $AF
95BA: .byte $AF
95BB: .byte $AF
95BC: SBC $EBFD,X
95BF: SBC $FDFD,X
95C2: SBC $FDFD,X
95C5: SBC $FDFD,X
95C8: CPY #$C0
95CA: CPY #$C0
95CC: CPY #$C0
95CE: CPY #$C0
95D0: CPY #$C0
95D2: CPY #$FD
95D4: SBC $FDFD,X
95D7: SBC $FDFD,X
95DA: SBC $FDFD,X
95DD: SBC $FDAF,X
95E0: CPY #$89
95E2: .byte $89
95E3: STA $9B98,X
95E6: .byte $89
95E7: STA $FD9B,X
95EA: .byte $AF
95EB: SBC $8989,X
95EE: .byte $89
95EF: SBC $9AFD,X
95F2: TYA
95F3: SBC $FD99,X
95F6: .byte $AF
95F7: SBC $8989,X
95FA: .byte $89
95FB: .byte $89
95FC: .byte $89
95FD: SBC $FDFD,X
9600: STA $AFFD,Y
9603: SBC $9898,X
9606: TYA
9607: .byte $9B
9608: .byte $89
9609: .byte $89
960A: .byte $89
960B: SBC $FD99,X
960E: .byte $AF
960F: SBC $FDFD,X
9612: SBC $9B9A,X
9615: .byte $89
9616: .byte $89
9617: SBC $FD99,X
961A: .byte $AF
961B: SBC $9D89,X
961E: .byte $9B
961F: SBC $8999,X
9622: STA $99FD,X
9625: SBC $FDAF,X
9628: .byte $89
9629: SBC $FD99,X
962C: STA $FD9D,Y
962F: SBC $FD9C,X
9632: .byte $AF
9633: SBC $FD89,X
9636: STA $9CFD,Y
9639: SBC $899C,X
963C: .byte $89
963D: SBC $FDAF,X
9640: TYA
9641: SBC $989A,X
9644: .byte $9B
9645: .byte $89
9646: .byte $89
9647: .byte $89
9648: .byte $89
9649: SBC $FDAF,X
964C: SBC $FDFD,X
964F: SBC $9B9A,X
9652: .byte $89
9653: .byte $89
9654: .byte $89
9655: SBC $C0AF,X
9658: .byte $89
9659: SBC $FDFD,X
965C: SBC $9D9C,X
965F: .byte $9B
9660: .byte $89
9661: SBC $AFAF,X
9664: .byte $89
9665: .byte $89
9666: .byte $89
9667: STA $899B,X
966A: SBC $899C,X
966D: SBC $FDFD,X
9670: .byte $89
9671: ORA $81
9673: SBC $899C,X
9676: .byte $89
9677: .byte $89
9678: .byte $89
9679: .byte $89
967A: SBC $8BFD,X
967D: .byte $8B
967E: .byte $8B
967F: .byte $8B
9680: .byte $8B
9681: .byte $8B
9682: .byte $8B
9683: .byte $8B
9684: .byte $8B
9685: .byte $8B
9686: SBC $98FD,X
9689: TYA
968A: TYA
968B: TYA
968C: TYA
968D: .byte $9B
968E: STA $9898,X
9691: TYA
9692: SBC $F1FD,X
9695: SED
9696: SBC $FDFD,X
9699: STA $FDFD,Y
969C: SBC $FDFD,X
969F: SBC $8989,X
96A2: SBC $FDFD,X
96A5: TXS
96A6: .byte $9B
96A7: SBC $899C,X
96AA: SBC $89FD,X
96AD: .byte $89
96AE: .byte $89
96AF: SBC $FDFD,X
96B2: .byte $9C
96B3: .byte $89
96B4: .byte $89
96B5: .byte $89
96B6: SBC $8AFD,X
96B9: TXA
96BA: TXA
96BB: SBC $FDFD,X
96BE: TXA
96BF: TXA
96C0: TXA
96C1: TXA
96C2: SBC $89FD,X
96C5: .byte $89
96C6: .byte $89
96C7: SBC $FDFD,X
96CA: SBC $989A,X
96CD: .byte $9B
96CE: SBC $89FD,X
96D1: .byte $89
96D2: .byte $89
96D3: .byte $89
96D4: .byte $89
96D5: SBC $FDFD,X
96D8: SBC $FD99,X
96DB: SBC $8989,X
96DE: .byte $89
96DF: .byte $89
96E0: .byte $89
96E1: .byte $89
96E2: .byte $89
96E3: SBC $99FD,X
96E6: SBC $98FD,X
96E9: .byte $9B
96EA: .byte $89
96EB: .byte $89
96EC: .byte $89
96ED: .byte $89
96EE: .byte $89
96EF: .byte $89
96F0: SBC $FD99,X
96F3: SBC $99FD,X
96F6: .byte $89
96F7: .byte $89
96F8: .byte $89
96F9: .byte $89
96FA: .byte $89
96FB: .byte $89
96FC: SBC $FD99,X
96FF: SBC $2700,X
9702: .byte $2B
9703: .byte $6F
9704: .byte $87
9705: .byte $04
9706: ASL $01
9708: .byte $07
9709: .byte $80
970A: .byte $0C
970B: BRK
970C: BRK
970D: BRK
970E: BRK
970F: BRK
9710: ORA $37
9712: ORA $1F5F
9715: .byte $0B
9716: BRK
9717: BRK
9718: BRK
9719: BRK
971A: BRK
971B: BRK
971C: BRK
971D: BRK
971E: BRK
971F: BRK
9720: EOR ($03,X)
9722: .byte $14
9723: LDY #$23
9725: ORA ($4D,X)
9727: ORA ($02,X)
9729: .byte $02
972A: BRK
972B: BRK
972C: BRK
972D: BRK
972E: BRK
972F: BRK
9730: EOR ($03,X)
9732: .byte $04
9733: BVS $9758
9735: ORA ($4D,X)
9737: ORA ($00,X)
9739: .byte $02
973A: BRK
973B: BRK
973C: BRK
973D: BRK
973E: BRK
973F: BRK
9740: EOR ($03,X)
9742: ROL $90,X
9744: ORA $4D04,Y
9747: ORA ($00,X)
9749: .byte $02
974A: BRK
974B: BRK
974C: BRK
974D: BRK
974E: BRK
974F: BRK
9750: EOR ($03),Y
9752: .byte $0B
9753: LDY #$20
9755: ORA ($5D,X)
9757: .byte $02
9758: BRK
9759: .byte $02
975A: BRK
975B: BRK
975C: BRK
975D: BRK
975E: BRK
975F: BRK
9760: EOR ($03),Y
9762: ROL $1950,X
9765: .byte $03
9766: EOR $0302,X
9769: .byte $02
976A: BRK
976B: BRK
976C: BRK
976D: BRK
976E: BRK
976F: BRK
9770: EOR ($03),Y
9772: BMI $96F4
9774: ORA $5D04,Y
9777: .byte $02
9778: .byte $03
9779: .byte $02
977A: BRK
977B: BRK
977C: BRK
977D: BRK
977E: BRK
977F: BRK
9780: ADC ($03,X)
9782: ASL $50,X
9784: PLP
9785: ORA ($6D,X)
9787: .byte $02
9788: BRK
9789: ORA ($00,X)
978B: BRK
978C: BRK
978D: BRK
978E: BRK
978F: BRK
9790: ADC ($03,X)
9792: AND $1790,Y
9795: .byte $04
9796: ADC $0002
9799: ORA ($00,X)
979B: BRK
979C: BRK
979D: BRK
979E: BRK
979F: BRK
97A0: ADC ($03,X)
97A2: .byte $3B
97A3: BCC $97BD
97A5: .byte $04
97A6: ADC $0002
97A9: ORA ($00,X)
97AB: BRK
97AC: BRK
97AD: BRK
97AE: BRK
97AF: BRK
97B0: BRK
97B1: BRK
97B2: BRK
97B3: BRK
97B4: BRK
97B5: BRK
97B6: BRK
97B7: BRK
97B8: BRK
97B9: BRK
97BA: BRK
97BB: BRK
97BC: BRK
97BD: BRK
97BE: BRK
97BF: BRK
97C0: BRK
97C1: BRK
97C2: BRK
97C3: BRK
97C4: BRK
97C5: BRK
97C6: BRK
97C7: BRK
97C8: BRK
97C9: BRK
97CA: BRK
97CB: BRK
97CC: BRK
97CD: BRK
97CE: BRK
97CF: BRK
97D0: BRK
97D1: BRK
97D2: BRK
97D3: BRK
97D4: BRK
97D5: BRK
97D6: BRK
97D7: BRK
97D8: BRK
97D9: BRK
97DA: BRK
97DB: BRK
97DC: BRK
97DD: BRK
97DE: BRK
97DF: BRK
97E0: .byte $0F
97E1: ORA ($05),Y
97E3: BMI $97F4
97E5: .byte $02
97E6: .byte $1C
97E7: .byte $3C
97E8: .byte $0F
97E9: .byte $07
97EA: ROL $38
97EC: .byte $0F
97ED: BPL $9821
97EF: BMI $9800
97F1: ORA $30
97F3: ROL $0F,X
97F5: ORA $26
97F7: BMI $9808
97F9: ORA ($25,X)
97FB: BMI $980C
97FD: .byte $07
97FE: .byte $27
97FF: SEC
9800: SBC $989A,X
9803: TYA
9804: TYA
9805: TYA
9806: TYA
9807: TYA
9808: SBC $FD9A,X
980B: SBC $8BFD,X
980E: .byte $8B
980F: .byte $8B
9810: .byte $8B
9811: .byte $8B
9812: .byte $8B
9813: .byte $8B
9814: SBC $FD8B,X
9817: SBC $8AFD,X
981A: TXA
981B: TXA
981C: INC $8A8A,X
981F: TXA
9820: SBC $FD8A,X
9823: SBC $FDFD,X
9826: SBC $8A8A,X
9829: TXA
982A: TXA
982B: SBC $8AFD,X
982E: SBC $C0FD,X
9831: .byte $8B
9832: SBC $8B8B,X
9835: .byte $8B
9836: .byte $8B
9837: SBC $8B8B,X
983A: .byte $8B
983B: SBC $8AFD,X
983E: SBC $8A8A,X
9841: TXA
9842: TXA
9843: SBC $8A8A,X
9846: TXA
9847: SBC $6FFD,X
984A: SBC $AFAF,X
984D: ROR $6E
984F: SBC $4848,X
9852: PHA
9853: SBC $6FFD,X
9856: SBC $6E66,X
9859: ROR $FD6E
985C: PHA
985D: PHA
985E: PHA
985F: CPY #$FD
9861: .byte $6F
9862: SBC $AFAF,X
9865: ADC $FD48
9868: PHA
9869: INC $FD48,X
986C: SBC $FD6F,X
986F: .byte $AF
9870: .byte $AF
9871: ADC $FDFE
9874: PHA
9875: PHA
9876: INC $FDFD,X
9879: .byte $6F
987A: SBC $AFAF,X
987D: ADC $4848
9880: INC $4848,X
9883: SBC $6FFD,X
9886: SBC $AFAF,X
9889: ADC $4848
988C: PHA
988D: PHA
988E: PHA
988F: SBC $6FFD,X
9892: SBC $AFAF,X
9895: .byte $AF
9896: INC $486D,X
9899: PHA
989A: PHA
989B: SBC $6FFD,X
989E: SBC $6F6F,X
98A1: .byte $6F
98A2: .byte $6F
98A3: .byte $6F
98A4: ADC $4848
98A7: SBC $AFFD,X
98AA: SBC $666F,X
98AD: ROR $6E6E
98B0: ADC $FE48
98B3: SBC $AFFD,X
98B6: SBC $AFAF,X
98B9: .byte $AF
98BA: .byte $AF
98BB: INC $486D,X
98BE: INC $FDFD,X
98C1: LSR $4BFD
98C4: .byte $4B
98C5: .byte $4B
98C6: .byte $4B
98C7: .byte $4B
98C8: .byte $4B
98C9: .byte $4B
98CA: .byte $4B
98CB: SBC $4FFD,X
98CE: SBC $4A4A,X
98D1: LSR A
98D2: SBC $4AFD,X
98D5: LSR A
98D6: LSR A
98D7: SBC $AFFD,X
98DA: SBC $FEAF,X
98DD: .byte $AF
98DE: .byte $AF
98DF: SBC $6FAF,X
98E2: .byte $67
98E3: SBC $AFFD,X
98E6: SBC $FEAF,X
98E9: INC $FDFE,X
98EC: .byte $AF
98ED: .byte $6F
98EE: .byte $67
98EF: SBC $AFFD,X
98F2: SBC $AFAF,X
98F5: INC $FDAF,X
98F8: .byte $AF
98F9: .byte $6F
98FA: .byte $67
98FB: SBC $AFFD,X
98FE: SBC $FEAF,X
9901: INC $FDFE,X
9904: .byte $AF
9905: .byte $6F
9906: .byte $67
9907: SBC $6FFD,X
990A: SBC $AFAF,X
990D: .byte $AF
990E: .byte $AF
990F: SBC $6FAF,X
9912: .byte $67
9913: SBC $AFFD,X
9916: SBC $C2FD,X
9919: SBC $FDFD,X
991C: SBC $FDFD,X
991F: SBC $AFFD,X
9922: .byte $AF
9923: .byte $AF
9924: .byte $AF
9925: .byte $AF
9926: .byte $AF
9927: .byte $AF
9928: .byte $AF
9929: .byte $AF
992A: .byte $AF
992B: .byte $AF
992C: SBC $AFFD,X
992F: .byte $AF
9930: .byte $7C
9931: .byte $7C
9932: .byte $AF
9933: .byte $AF
9934: .byte $AF
9935: .byte $7C
9936: .byte $AF
9937: .byte $AF
9938: .byte $AF
9939: .byte $AF
993A: .byte $AF
993B: .byte $AF
993C: .byte $AF
993D: .byte $AF
993E: .byte $AF
993F: .byte $AF
9940: .byte $AF
9941: .byte $AF
9942: .byte $AF
9943: .byte $AF
9944: CPY #$7C
9946: .byte $AF
9947: .byte $7C
9948: .byte $7C
9949: .byte $7C
994A: .byte $AF
994B: .byte $AF
994C: .byte $AF
994D: .byte $7C
994E: .byte $AF
994F: .byte $AF
9950: .byte $7C
9951: .byte $7C
9952: .byte $7C
9953: .byte $7C
9954: .byte $AF
9955: .byte $AF
9956: .byte $AF
9957: .byte $AF
9958: .byte $AF
9959: .byte $AF
995A: .byte $AF
995B: .byte $AF
995C: CPY #$AF
995E: .byte $7C
995F: .byte $7C
9960: .byte $AF
9961: .byte $7C
9962: .byte $7C
9963: .byte $7C
9964: .byte $7C
9965: .byte $7C
9966: .byte $AF
9967: .byte $7C
9968: .byte $7C
9969: .byte $AF
996A: .byte $AF
996B: .byte $7C
996C: .byte $AF
996D: .byte $AF
996E: .byte $AF
996F: .byte $7C
9970: .byte $7C
9971: .byte $AF
9972: .byte $AF
9973: .byte $7C
9974: .byte $7C
9975: .byte $AF
9976: .byte $AF
9977: .byte $7C
9978: .byte $AF
9979: .byte $AF
997A: .byte $AF
997B: .byte $7C
997C: .byte $AF
997D: .byte $AF
997E: .byte $7C
997F: .byte $7C
9980: .byte $7C
9981: .byte $AF
9982: .byte $AF
9983: .byte $7C
9984: .byte $AF
9985: .byte $AF
9986: .byte $AF
9987: .byte $7C
9988: .byte $AF
9989: .byte $AF
998A: .byte $7C
998B: .byte $7C
998C: .byte $7C
998D: .byte $CB
998E: .byte $CB
998F: .byte $7C
9990: .byte $CB
9991: .byte $CB
9992: .byte $CB
9993: .byte $7C
9994: .byte $7C
9995: .byte $CB
9996: .byte $7C
9997: .byte $7C
9998: .byte $7C
9999: DEX
999A: DEX
999B: .byte $7C
999C: DEX
999D: DEX
999E: DEX
999F: DEX
99A0: .byte $7C
99A1: DEX
99A2: DEX
99A3: .byte $7C
99A4: .byte $7C
99A5: CMP $7CC9,Y
99A8: CMP $D6C9,Y
99AB: CMP #$7C
99AD: .byte $7C
99AE: CMP $7C7C,Y
99B1: CMP $7CC9,Y
99B4: .byte $DA
99B5: CLD
99B6: CLD
99B7: CLD
99B8: CLD
99B9: .byte $7C
99BA: CMP $7C7C,Y
99BD: CMP $7CDD,Y
99C0: CPY $CCCC
99C3: CPY $7CCC
99C6: CMP $7C7C,Y
99C9: CMP $7CBE,Y
99CC: CMP $D6C9,Y
99CF: CMP #$DD
99D1: .byte $7C
99D2: CMP $7C7C,Y
99D5: CMP $7CC9,Y
99D8: CMP $C9C9,Y
99DB: CMP #$74
99DD: .byte $7C
99DE: CMP $7C7C,Y
99E1: CMP $7CC9,Y
99E4: CMP $D6C9,Y
99E7: CMP #$C9
99E9: .byte $7C
99EA: CMP $7C7C,Y
99ED: CMP $C1
99EF: .byte $7C
99F0: .byte $DA
99F1: CLD
99F2: CLD
99F3: CLD
99F4: CLD
99F5: .byte $7C
99F6: CMP $7C7C,Y
99F9: .byte $DA
99FA: CLD
99FB: .byte $7C
99FC: CPY $CCCC
99FF: CPY $7CCC
9A02: CMP $7C7C,Y
9A05: .byte $7C
9A06: .byte $7C
9A07: .byte $7C
9A08: CMP $D6C9,Y
9A0B: CMP #$DD
9A0D: .byte $7C
9A0E: CMP $7C7C,Y
9A11: CMP $7CC9,Y
9A14: CMP $C9C9,Y
9A17: CMP #$74
9A19: .byte $7C
9A1A: CMP $7C7C,Y
9A1D: CPY $C1
9A1F: .byte $7C
9A20: CMP $D6C9,Y
9A23: CMP #$C9
9A25: .byte $7C
9A26: CMP $7C7C,Y
9A29: CMP $7CC9,Y
9A2C: .byte $DA
9A2D: CLD
9A2E: CLD
9A2F: CLD
9A30: CLD
9A31: .byte $7C
9A32: CMP $7C7C,Y
9A35: CMP $7CC9,Y
9A38: CPY $CCCC
9A3B: CPY $7CCC
9A3E: CMP $7C7C,Y
9A41: CMP $7CC9,Y
9A44: CMP $D6C9,Y
9A47: CMP #$C9
9A49: .byte $7C
9A4A: CMP $7C7C,Y
9A4D: CMP $7CC9,Y
9A50: CMP $C9C9,Y
9A53: CMP #$DD
9A55: .byte $7C
9A56: CMP $7C7C,Y
9A59: CMP $7CC9,Y
9A5C: CMP $D6C9,Y
9A5F: CMP #$74
9A61: .byte $7C
9A62: CMP $7C7C,Y
9A65: CMP $7CC9,Y
9A68: .byte $DA
9A69: CLD
9A6A: CLD
9A6B: CLD
9A6C: CLD
9A6D: .byte $7C
9A6E: CMP $7C7C,Y
9A71: CMP $7CDD,Y
9A74: CPY $CCCC
9A77: CPY $7CCC
9A7A: CMP $7C7C,Y
9A7D: CMP $7CBE,Y
9A80: CMP $D6C9,Y
9A83: CMP #$DD
9A85: .byte $7C
9A86: .byte $DA
9A87: CPY #$7C
9A89: CMP $7CC9,Y
9A8C: CMP $C9C9,Y
9A8F: CMP #$7C
9A91: .byte $7C
9A92: .byte $7C
9A93: .byte $7C
9A94: .byte $7C
9A95: .byte $E2
9A96: CMP #$7C
9A98: CMP $D6C9,Y
9A9B: CMP $D97C,X
9A9E: LDX $7C7C,Y
9AA1: .byte $E2
9AA2: DEC $E27C,X
9AA5: CMP #$C9
9AA7: .byte $7C
9AA8: .byte $7C
9AA9: .byte $DA
9AAA: LDX $7C7C,Y
9AAD: .byte $E2
9AAE: DEC $E27C,X
9AB1: CMP #$DD
9AB3: .byte $7C
9AB4: CMP $BEBE,Y
9AB7: .byte $7C
9AB8: .byte $7C
9AB9: .byte $E2
9ABA: DEC $E27C,X
9ABD: DEC $7C74,X
9AC0: CMP $BEBE,Y
9AC3: .byte $7C
9AC4: .byte $7C
9AC5: .byte $E2
9AC6: DEC $E27C,X
9AC9: DEC $7CDE,X
9ACC: CMP $BEBE,Y
9ACF: .byte $7C
9AD0: .byte $7C
9AD1: .byte $E2
9AD2: DEC $E27C,X
9AD5: DEC $7CDE,X
9AD8: CMP $BEBE,Y
9ADB: .byte $7C
9ADC: .byte $7C
9ADD: .byte $E2
9ADE: DEC $E27C,X
9AE1: DEC $7CDE,X
9AE4: CMP $BEBE,Y
9AE7: .byte $7C
9AE8: .byte $7C
9AE9: .byte $E2
9AEA: DEC $E27C,X
9AED: DEC $7CDE,X
9AF0: CMP $BEBE,Y
9AF3: .byte $7C
9AF4: .byte $7C
9AF5: .byte $E2
9AF6: DEC $E27C,X
9AF9: DEC $7CDE,X
9AFC: CMP $C9C9,Y
9AFF: .byte $7C
9B00: BRK
9B01: BIT $34
9B03: BCS $9B4D
9B05: .byte $04
9B06: ASL $01
9B08: ASL $10,X
9B0A: BRK
9B0B: .byte $03
9B0C: BRK
9B0D: BRK
9B0E: BRK
9B0F: BRK
9B10: BRK
9B11: .byte $5F
9B12: ORA ($50,X)
9B14: PHP
9B15: PHP
9B16: BRK
9B17: BRK
9B18: BRK
9B19: BRK
9B1A: BRK
9B1B: BRK
9B1C: BRK
9B1D: BRK
9B1E: BRK
9B1F: BRK
9B20: ADC ($03,X)
9B22: .byte $23
9B23: JSR $0323
9B26: ADC $0002
9B29: .byte $02
9B2A: BRK
9B2B: BRK
9B2C: BRK
9B2D: BRK
9B2E: BRK
9B2F: BRK
9B30: EOR ($02,X)
9B32: .byte $32
9B33: BVC $9B4E
9B35: ORA ($4D,X)
9B37: .byte $02
9B38: .byte $03
9B39: .byte $02
9B3A: BRK
9B3B: BRK
9B3C: BRK
9B3D: BRK
9B3E: BRK
9B3F: BRK
9B40: EOR ($03),Y
9B42: ASL $30
9B44: ASL $5D01,X
9B47: .byte $02
9B48: BRK
9B49: .byte $02
9B4A: BRK
9B4B: BRK
9B4C: BRK
9B4D: BRK
9B4E: BRK
9B4F: BRK
9B50: EOR ($03),Y
9B52: ASL $1E40
9B55: ORA ($5D,X)
9B57: .byte $02
9B58: BRK
9B59: .byte $02
9B5A: BRK
9B5B: BRK
9B5C: BRK
9B5D: BRK
9B5E: BRK
9B5F: BRK
9B60: EOR ($03),Y
9B62: ASL $1E60,X
9B65: ORA ($5D,X)
9B67: .byte $02
9B68: BRK
9B69: .byte $02
9B6A: BRK
9B6B: BRK
9B6C: BRK
9B6D: BRK
9B6E: BRK
9B6F: BRK
9B70: ADC ($02,X)
9B72: AND $80
9B74: ASL $6D01,X
9B77: .byte $02
9B78: BRK
9B79: .byte $02
9B7A: BRK
9B7B: BRK
9B7C: BRK
9B7D: BRK
9B7E: BRK
9B7F: BRK
9B80: ADC ($02),Y
9B82: AND $1950,X
9B85: ORA ($7D,X)
9B87: .byte $02
9B88: .byte $04
9B89: .byte $02
9B8A: BRK
9B8B: BRK
9B8C: BRK
9B8D: BRK
9B8E: BRK
9B8F: BRK
9B90: ADC ($02),Y
9B92: .byte $14
9B93: BCC $9BAE
9B95: ORA ($7D,X)
9B97: .byte $02
9B98: .byte $04
9B99: .byte $02
9B9A: BRK
9B9B: BRK
9B9C: BRK
9B9D: BRK
9B9E: BRK
9B9F: BRK
9BA0: ADC ($02,X)
9BA2: .byte $2F
9BA3: .byte $80
9BA4: ASL $6D01,X
9BA7: .byte $02
9BA8: BRK
9BA9: .byte $02
9BAA: BRK
9BAB: BRK
9BAC: BRK
9BAD: BRK
9BAE: BRK
9BAF: BRK
9BB0: BRK
9BB1: BRK
9BB2: BRK
9BB3: BRK
9BB4: BRK
9BB5: BRK
9BB6: BRK
9BB7: BRK
9BB8: BRK
9BB9: BRK
9BBA: BRK
9BBB: BRK
9BBC: BRK
9BBD: BRK
9BBE: BRK
9BBF: BRK
9BC0: BRK
9BC1: BRK
9BC2: BRK
9BC3: BRK
9BC4: BRK
9BC5: BRK
9BC6: BRK
9BC7: BRK
9BC8: BRK
9BC9: BRK
9BCA: BRK
9BCB: BRK
9BCC: BRK
9BCD: BRK
9BCE: BRK
9BCF: BRK
9BD0: BRK
9BD1: BRK
9BD2: BRK
9BD3: BRK
9BD4: BRK
9BD5: BRK
9BD6: BRK
9BD7: BRK
9BD8: BRK
9BD9: BRK
9BDA: BRK
9BDB: BRK
9BDC: BRK
9BDD: BRK
9BDE: BRK
9BDF: BRK
9BE0: .byte $0F
9BE1: ORA ($05),Y
9BE3: BMI $9BF4
9BE5: .byte $02
9BE6: .byte $1C
9BE7: .byte $3C
9BE8: .byte $0F
9BE9: .byte $07
9BEA: ROL $38
9BEC: .byte $0F
9BED: BPL $9C21
9BEF: BMI $9C00
9BF1: ORA $30
9BF3: ROL $0F,X
9BF5: ORA $26
9BF7: BMI $9C08
9BF9: .byte $13
9BFA: BIT $30
9BFC: .byte $0F
9BFD: CLC
9BFE: .byte $27
9BFF: SEC
9C00: .byte $7C
9C01: .byte $E2
9C02: DEC $E27C,X
9C05: DEC $7CDE,X
9C08: CMP $C9C9,Y
9C0B: .byte $7C
9C0C: .byte $7C
9C0D: .byte $E2
9C0E: DEC $E27C,X
9C11: DEC $7CDE,X
9C14: CMP $DEC9,Y
9C17: .byte $7C
9C18: .byte $7C
9C19: .byte $E2
9C1A: DEC $E27C,X
9C1D: DEC $7CDE,X
9C20: .byte $E2
9C21: DEC $7CDE,X
9C24: .byte $7C
9C25: .byte $E2
9C26: DEC $E27C,X
9C29: DEC $7CDE,X
9C2C: .byte $E2
9C2D: .byte $6F
9C2E: DEC $7C7C,X
9C31: .byte $E2
9C32: DEC $E27C,X
9C35: DEC $7CDE,X
9C38: .byte $6F
9C39: .byte $6F
9C3A: DEC $7C7C,X
9C3D: .byte $E2
9C3E: DEC $E27C,X
9C41: DEC $7CDE,X
9C44: .byte $7C
9C45: .byte $6F
9C46: .byte $6F
9C47: .byte $7C
9C48: .byte $7C
9C49: .byte $E2
9C4A: DEC $E27C,X
9C4D: DEC $EF6F,X
9C50: .byte $7C
9C51: .byte $7C
9C52: .byte $6F
9C53: .byte $6F
9C54: .byte $7C
9C55: .byte $E2
9C56: DEC $E27C,X
9C59: .byte $6F
9C5A: .byte $6F
9C5B: .byte $6F
9C5C: .byte $EF
9C5D: .byte $7C
9C5E: .byte $6F
9C5F: .byte $6F
9C60: .byte $7C
9C61: .byte $E2
9C62: DEC $6F7C,X
9C65: .byte $6F
9C66: .byte $6F
9C67: .byte $6F
9C68: .byte $6F
9C69: .byte $6F
9C6A: .byte $6F
9C6B: .byte $6F
9C6C: .byte $7C
9C6D: .byte $E2
9C6E: .byte $E3
9C6F: .byte $6F
9C70: .byte $6F
9C71: .byte $6F
9C72: .byte $6F
9C73: .byte $6F
9C74: .byte $6F
9C75: .byte $6F
9C76: .byte $6F
9C77: .byte $6F
9C78: .byte $7C
9C79: .byte $E2
9C7A: .byte $7C
9C7B: .byte $7C
9C7C: .byte $7C
9C7D: .byte $7C
9C7E: .byte $7C
9C7F: .byte $7C
9C80: .byte $7C
9C81: .byte $7C
9C82: .byte $6F
9C83: .byte $6F
9C84: .byte $7C
9C85: .byte $E2
9C86: .byte $7C
9C87: .byte $6F
9C88: .byte $6F
9C89: .byte $6F
9C8A: .byte $6F
9C8B: .byte $6F
9C8C: .byte $6F
9C8D: .byte $7C
9C8E: .byte $6F
9C8F: .byte $6F
9C90: .byte $7C
9C91: .byte $E2
9C92: .byte $7C
9C93: .byte $6F
9C94: .byte $6F
9C95: .byte $AF
9C96: .byte $7C
9C97: .byte $7C
9C98: .byte $7C
9C99: .byte $7C
9C9A: .byte $6F
9C9B: .byte $6F
9C9C: .byte $7C
9C9D: .byte $6F
9C9E: .byte $7C
9C9F: .byte $6F
9CA0: .byte $6F
9CA1: .byte $6F
9CA2: .byte $7C
9CA3: .byte $6F
9CA4: .byte $6F
9CA5: .byte $6F
9CA6: .byte $6F
9CA7: .byte $6F
9CA8: .byte $7C
9CA9: .byte $6F
9CAA: .byte $7C
9CAB: .byte $6F
9CAC: .byte $6F
9CAD: .byte $6F
9CAE: .byte $6F
9CAF: .byte $6F
9CB0: .byte $6F
9CB1: .byte $6F
9CB2: .byte $6F
9CB3: .byte $6F
9CB4: .byte $7C
9CB5: .byte $6F
9CB6: .byte $7C
9CB7: .byte $6F
9CB8: .byte $6F
9CB9: .byte $6F
9CBA: .byte $6F
9CBB: .byte $6F
9CBC: .byte $6F
9CBD: .byte $6F
9CBE: .byte $6F
9CBF: .byte $6F
9CC0: .byte $7C
9CC1: .byte $6F
9CC2: .byte $7C
9CC3: .byte $6F
9CC4: .byte $6F
9CC5: .byte $6F
9CC6: .byte $6F
9CC7: .byte $6F
9CC8: .byte $6F
9CC9: .byte $6F
9CCA: .byte $6F
9CCB: .byte $6F
9CCC: CPY #$BE
9CCE: .byte $7C
9CCF: .byte $6F
9CD0: .byte $6F
9CD1: .byte $6F
9CD2: LDX $6F6F,Y
9CD5: .byte $6F
9CD6: .byte $6F
9CD7: .byte $6F
9CD8: .byte $6F
9CD9: .byte $6F
9CDA: .byte $7C
9CDB: .byte $6F
9CDC: .byte $6F
9CDD: .byte $6F
9CDE: .byte $6F
9CDF: .byte $6F
9CE0: .byte $7C
9CE1: .byte $7C
9CE2: .byte $7C
9CE3: .byte $7C
9CE4: .byte $7C
9CE5: .byte $7C
9CE6: .byte $7C
9CE7: .byte $6F
9CE8: .byte $6F
9CE9: .byte $6F
9CEA: .byte $6F
9CEB: .byte $6F
9CEC: .byte $6F
9CED: .byte $6F
9CEE: .byte $6F
9CEF: .byte $6F
9CF0: .byte $7C
9CF1: .byte $6F
9CF2: .byte $6F
9CF3: .byte $6F
9CF4: .byte $6F
9CF5: .byte $6F
9CF6: LDX $6F6F,Y
9CF9: .byte $6F
9CFA: .byte $6F
9CFB: .byte $6F
9CFC: .byte $7C
9CFD: .byte $6F
9CFE: .byte $6F
9CFF: .byte $6F
9D00: .byte $6F
9D01: .byte $6F
9D02: .byte $6F
9D03: .byte $6F
9D04: .byte $6F
9D05: .byte $6F
9D06: .byte $6F
9D07: .byte $6F
9D08: .byte $7C
9D09: .byte $6F
9D0A: .byte $6F
9D0B: .byte $6F
9D0C: .byte $6F
9D0D: .byte $6F
9D0E: .byte $6F
9D0F: .byte $6F
9D10: .byte $6F
9D11: .byte $6F
9D12: .byte $6F
9D13: .byte $6F
9D14: .byte $7C
9D15: .byte $6F
9D16: .byte $6F
9D17: .byte $6F
9D18: .byte $6F
9D19: .byte $6F
9D1A: LDX $6F6F,Y
9D1D: .byte $6F
9D1E: .byte $6F
9D1F: .byte $6F
9D20: .byte $7C
9D21: .byte $6F
9D22: .byte $6F
9D23: .byte $6F
9D24: .byte $6F
9D25: .byte $6F
9D26: .byte $6F
9D27: .byte $6F
9D28: .byte $6F
9D29: .byte $6F
9D2A: .byte $6F
9D2B: .byte $6F
9D2C: .byte $7C
9D2D: .byte $6F
9D2E: .byte $6F
9D2F: .byte $6F
9D30: .byte $6F
9D31: .byte $6F
9D32: .byte $6F
9D33: .byte $6F
9D34: .byte $6F
9D35: .byte $6F
9D36: .byte $6F
9D37: .byte $6F
9D38: .byte $7C
9D39: .byte $6F
9D3A: .byte $6F
9D3B: .byte $6F
9D3C: .byte $6F
9D3D: .byte $6F
9D3E: LDX $FD6F,Y
9D41: .byte $6F
9D42: .byte $6F
9D43: .byte $6F
9D44: .byte $7C
9D45: .byte $6F
9D46: .byte $6F
9D47: .byte $6F
9D48: .byte $6F
9D49: .byte $6F
9D4A: .byte $6F
9D4B: .byte $6F
9D4C: SBC $F1F2,X
9D4F: SBC ($7C),Y
9D51: .byte $6F
9D52: .byte $6F
9D53: .byte $6F
9D54: .byte $6F
9D55: .byte $6F
9D56: .byte $6F
9D57: .byte $6F
9D58: SBC $6F6F,X
9D5B: .byte $6F
9D5C: .byte $7C
9D5D: .byte $6F
9D5E: .byte $6F
9D5F: .byte $6F
9D60: .byte $6F
9D61: .byte $6F
9D62: LDX $6F6F,Y
9D65: .byte $6F
9D66: .byte $7C
9D67: .byte $6F
9D68: .byte $7C
9D69: .byte $6F
9D6A: .byte $6F
9D6B: .byte $6F
9D6C: .byte $6F
9D6D: .byte $6F
9D6E: .byte $6F
9D6F: .byte $6F
9D70: .byte $6F
9D71: .byte $6F
9D72: .byte $6F
9D73: .byte $6F
9D74: .byte $7C
9D75: .byte $6F
9D76: .byte $6F
9D77: .byte $6F
9D78: .byte $7C
9D79: .byte $6F
9D7A: .byte $6F
9D7B: .byte $6F
9D7C: .byte $6F
9D7D: .byte $6F
9D7E: .byte $6F
9D7F: .byte $6F
9D80: .byte $7C
9D81: .byte $6F
9D82: .byte $6F
9D83: .byte $7C
9D84: .byte $6F
9D85: .byte $6F
9D86: .byte $6F
9D87: .byte $6F
9D88: .byte $6F
9D89: .byte $6F
9D8A: .byte $6F
9D8B: .byte $6F
9D8C: .byte $7C
9D8D: .byte $6F
9D8E: .byte $6F
9D8F: .byte $7C
9D90: .byte $EF
9D91: .byte $EF
9D92: .byte $7C
9D93: .byte $7C
9D94: .byte $EF
9D95: .byte $6F
9D96: .byte $6F
9D97: .byte $6F
9D98: .byte $7C
9D99: .byte $6F
9D9A: .byte $6F
9D9B: .byte $7C
9D9C: .byte $7C
9D9D: .byte $7C
9D9E: .byte $7C
9D9F: .byte $7C
9DA0: .byte $7C
9DA1: .byte $6F
9DA2: .byte $6F
9DA3: .byte $6F
9DA4: .byte $7C
9DA5: .byte $6F
9DA6: .byte $6F
9DA7: .byte $6F
9DA8: .byte $6F
9DA9: .byte $6F
9DAA: .byte $6F
9DAB: .byte $7C
9DAC: .byte $7C
9DAD: .byte $6F
9DAE: .byte $6F
9DAF: .byte $6F
9DB0: .byte $EF
9DB1: .byte $6F
9DB2: .byte $6F
9DB3: .byte $6F
9DB4: .byte $6F
9DB5: ADC $7C6F,Y
9DB8: .byte $7C
9DB9: .byte $7C
9DBA: .byte $6F
9DBB: .byte $EF
9DBC: .byte $EF
9DBD: .byte $6F
9DBE: .byte $6F
9DBF: .byte $6F
9DC0: .byte $6F
9DC1: .byte $6F
9DC2: .byte $6F
9DC3: .byte $6F
9DC4: .byte $7C
9DC5: .byte $7C
9DC6: .byte $6F
9DC7: .byte $EF
9DC8: .byte $EF
9DC9: .byte $6F
9DCA: .byte $6F
9DCB: .byte $7C
9DCC: .byte $6F
9DCD: .byte $6F
9DCE: .byte $6F
9DCF: .byte $6F
9DD0: .byte $7C
9DD1: .byte $7C
9DD2: .byte $6F
9DD3: .byte $6F
9DD4: .byte $EF
9DD5: .byte $6F
9DD6: .byte $6F
9DD7: .byte $7C
9DD8: .byte $B2
9DD9: LDA ($B1),Y
9DDB: LDA ($B8),Y
9DDD: .byte $7C
9DDE: .byte $6F
9DDF: .byte $6F
9DE0: .byte $EF
9DE1: .byte $6F
9DE2: .byte $7C
9DE3: .byte $7C
9DE4: .byte $6F
9DE5: .byte $6F
9DE6: .byte $6F
9DE7: .byte $6F
9DE8: .byte $AF
9DE9: .byte $7C
9DEA: .byte $6F
9DEB: .byte $6F
9DEC: .byte $EF
9DED: .byte $6F
9DEE: .byte $EF
9DEF: .byte $7C
9DF0: .byte $4B
9DF1: .byte $4B
9DF2: .byte $4B
9DF3: .byte $4B
9DF4: .byte $4B
9DF5: .byte $7C
9DF6: .byte $6F
9DF7: .byte $6F
9DF8: .byte $6F
9DF9: .byte $6F
9DFA: .byte $6F
9DFB: .byte $7C
9DFC: EOR #$49
9DFE: STX $49,Y
9E00: EOR #$7C
9E02: .byte $6F
9E03: .byte $6F
9E04: .byte $6F
9E05: .byte $6F
9E06: .byte $6F
9E07: .byte $7C
9E08: LSR A
9E09: LSR A
9E0A: LSR A
9E0B: LSR A
9E0C: LSR A
9E0D: .byte $7C
9E0E: .byte $6F
9E0F: .byte $6F
9E10: .byte $6F
9E11: .byte $6F
9E12: .byte $6F
9E13: .byte $7C
9E14: .byte $AF
9E15: .byte $8B
9E16: .byte $8B
9E17: .byte $8B
9E18: .byte $AF
9E19: .byte $7C
9E1A: .byte $6F
9E1B: .byte $6F
9E1C: .byte $6F
9E1D: .byte $6F
9E1E: .byte $6F
9E1F: .byte $7C
9E20: .byte $AF
9E21: .byte $89
9E22: .byte $43
9E23: .byte $89
9E24: .byte $EF
9E25: .byte $7C
9E26: .byte $6F
9E27: .byte $6F
9E28: .byte $6F
9E29: .byte $6F
9E2A: .byte $6F
9E2B: .byte $7C
9E2C: .byte $AF
9E2D: TXA
9E2E: TXA
9E2F: TXA
9E30: .byte $AF
9E31: .byte $7C
9E32: .byte $6F
9E33: .byte $6F
9E34: .byte $6F
9E35: .byte $6F
9E36: .byte $6F
9E37: .byte $7C
9E38: .byte $4B
9E39: .byte $4B
9E3A: .byte $4B
9E3B: .byte $4B
9E3C: .byte $4B
9E3D: .byte $7C
9E3E: .byte $6F
9E3F: .byte $6F
9E40: .byte $6F
9E41: .byte $6F
9E42: .byte $6F
9E43: .byte $7C
9E44: EOR #$49
9E46: STX $49,Y
9E48: EOR #$7C
9E4A: .byte $6F
9E4B: .byte $6F
9E4C: .byte $6F
9E4D: .byte $6F
9E4E: .byte $6F
9E4F: .byte $7C
9E50: LSR A
9E51: LSR A
9E52: LSR A
9E53: LSR A
9E54: LSR A
9E55: .byte $7C
9E56: .byte $6F
9E57: .byte $6F
9E58: .byte $6F
9E59: .byte $6F
9E5A: .byte $7C
9E5B: .byte $7C
9E5C: .byte $AF
9E5D: .byte $6F
9E5E: .byte $6F
9E5F: .byte $6F
9E60: .byte $6F
9E61: .byte $7C
9E62: .byte $6F
9E63: .byte $6F
9E64: .byte $6F
9E65: .byte $6F
9E66: .byte $6F
9E67: .byte $7C
9E68: .byte $B2
9E69: LDA ($B1),Y
9E6B: LDA ($B8),Y
9E6D: .byte $7C
9E6E: .byte $6F
9E6F: .byte $6F
9E70: .byte $6F
9E71: .byte $6F
9E72: .byte $6F
9E73: .byte $7C
9E74: .byte $AF
9E75: .byte $6F
9E76: .byte $6F
9E77: .byte $6F
9E78: .byte $7C
9E79: .byte $7C
9E7A: .byte $6F
9E7B: .byte $6F
9E7C: .byte $6F
9E7D: .byte $6F
9E7E: .byte $6F
9E7F: .byte $AF
9E80: .byte $6F
9E81: ADC $6F6F,Y
9E84: .byte $7C
9E85: .byte $7C
9E86: .byte $6F
9E87: .byte $6F
9E88: .byte $6F
9E89: .byte $6F
9E8A: .byte $6F
9E8B: .byte $AF
9E8C: .byte $6F
9E8D: .byte $6F
9E8E: .byte $6F
9E8F: .byte $6F
9E90: .byte $7C
9E91: .byte $7C
9E92: .byte $6F
9E93: .byte $6F
9E94: .byte $6F
9E95: .byte $6F
9E96: .byte $6F
9E97: .byte $7C
9E98: .byte $7C
9E99: .byte $6F
9E9A: .byte $6F
9E9B: .byte $7C
9E9C: .byte $7C
9E9D: .byte $7C
9E9E: .byte $6F
9E9F: .byte $6F
9EA0: .byte $6F
9EA1: .byte $6F
9EA2: .byte $6F
9EA3: .byte $7C
9EA4: .byte $6F
9EA5: .byte $6F
9EA6: .byte $6F
9EA7: .byte $7C
9EA8: .byte $7C
9EA9: .byte $EF
9EAA: .byte $6F
9EAB: .byte $6F
9EAC: LDA $6F6F,X
9EAF: .byte $7C
9EB0: .byte $7C
9EB1: .byte $7C
9EB2: .byte $7C
9EB3: .byte $7C
9EB4: .byte $7C
9EB5: .byte $6F
9EB6: .byte $6F
9EB7: .byte $6F
9EB8: .byte $6F
9EB9: .byte $6F
9EBA: .byte $6F
9EBB: .byte $6F
9EBC: .byte $6F
9EBD: .byte $7C
9EBE: .byte $7C
9EBF: .byte $7C
9EC0: .byte $6F
9EC1: .byte $6F
9EC2: .byte $6F
9EC3: .byte $6F
9EC4: .byte $6F
9EC5: .byte $6F
9EC6: .byte $6F
9EC7: .byte $6F
9EC8: .byte $6F
9EC9: .byte $6F
9ECA: .byte $6F
9ECB: .byte $6F
9ECC: .byte $6F
9ECD: .byte $6F
9ECE: LDA $6F6F,X
9ED1: .byte $6F
9ED2: .byte $6F
9ED3: .byte $6F
9ED4: .byte $6F
9ED5: LDA $6F6F,X
9ED8: .byte $6F
9ED9: .byte $6F
9EDA: .byte $6F
9EDB: .byte $6F
9EDC: .byte $6F
9EDD: LDA $6F6F,X
9EE0: .byte $6F
9EE1: .byte $6F
9EE2: .byte $6F
9EE3: .byte $6F
9EE4: .byte $6F
9EE5: .byte $6F
9EE6: .byte $6F
9EE7: .byte $6F
9EE8: .byte $6F
9EE9: .byte $6F
9EEA: .byte $6F
9EEB: .byte $6F
9EEC: .byte $6F
9EED: .byte $6F
9EEE: .byte $6F
9EEF: .byte $6F
9EF0: LDA $6F6F,X
9EF3: .byte $6F
9EF4: .byte $7C
9EF5: .byte $7C
9EF6: .byte $7C
9EF7: .byte $7C
9EF8: .byte $7C
9EF9: .byte $7C
9EFA: .byte $7C
9EFB: .byte $7C
9EFC: .byte $7C
9EFD: .byte $7C
9EFE: .byte $7C
9EFF: .byte $7C
9F00: BRK
9F01: BIT $3B
9F03: .byte $EF
9F04: .byte $EF
9F05: .byte $04
9F06: ASL $01
9F08: AND $1780
9F0B: .byte $03
9F0C: .byte $02
9F0D: ORA #$38
9F0F: BVC $9F11
9F11: INY
9F12: ORA ($C8,X)
9F14: PHP
9F15: PHP
9F16: BRK
9F17: BRK
9F18: BRK
9F19: BRK
9F1A: BRK
9F1B: BRK
9F1C: BRK
9F1D: BRK
9F1E: BRK
9F1F: BRK
9F20: ADC ($03),Y
9F22: ASL $1930,X
9F25: .byte $02
9F26: ADC $0402,X
9F29: ORA $00
9F2B: BRK
9F2C: BRK
9F2D: BRK
9F2E: BRK
9F2F: BRK
9F30: ADC ($03),Y
9F32: BRK
9F33: BRK
9F34: ORA $7D02,Y
9F37: ORA ($01,X)
9F39: .byte $02
9F3A: BRK
9F3B: BRK
9F3C: BRK
9F3D: BRK
9F3E: BRK
9F3F: BRK
9F40: ADC ($03),Y
9F42: .byte $37
9F43: BRK
9F44: ORA $7D03,Y
9F47: .byte $02
9F48: .byte $04
9F49: ORA ($00,X)
9F4B: BRK
9F4C: BRK
9F4D: BRK
9F4E: BRK
9F4F: BRK
9F50: EOR ($03),Y
9F52: BMI $9F74
9F54: .byte $23
9F55: .byte $03
9F56: EOR $0002,X
9F59: .byte $03
9F5A: BRK
9F5B: BRK
9F5C: BRK
9F5D: BRK
9F5E: BRK
9F5F: BRK
9F60: EOR ($02),Y
9F62: .byte $04
9F63: JSR $011E
9F66: EOR $0202,X
9F69: .byte $02
9F6A: BRK
9F6B: BRK
9F6C: BRK
9F6D: BRK
9F6E: BRK
9F6F: BRK
9F70: ADC ($03),Y
9F72: ROL $70
9F74: ASL $7D02,X
9F77: .byte $02
9F78: .byte $04
9F79: ORA $00
9F7B: BRK
9F7C: BRK
9F7D: BRK
9F7E: BRK
9F7F: BRK
9F80: ADC ($03,X)
9F82: ROL $1E20
9F85: .byte $03
9F86: ADC $0201
9F89: .byte $03
9F8A: BRK
9F8B: BRK
9F8C: BRK
9F8D: BRK
9F8E: BRK
9F8F: BRK
9F90: ADC ($02,X)
9F92: .byte $02
9F93: RTS
9F94: ORA $6D01,Y
9F97: .byte $02
9F98: BRK
9F99: .byte $02
9F9A: BRK
9F9B: BRK
9F9C: BRK
9F9D: BRK
9F9E: BRK
9F9F: BRK
9FA0: EOR ($02,X)
9FA2: .byte $03
9FA3: BCC $9FBE
9FA5: ORA ($4D,X)
9FA7: .byte $02
9FA8: .byte $04
9FA9: ORA ($00,X)
9FAB: BRK
9FAC: BRK
9FAD: BRK
9FAE: BRK
9FAF: BRK
9FB0: BRK
9FB1: BRK
9FB2: BRK
9FB3: BRK
9FB4: BRK
9FB5: BRK
9FB6: BRK
9FB7: BRK
9FB8: BRK
9FB9: BRK
9FBA: BRK
9FBB: BRK
9FBC: BRK
9FBD: BRK
9FBE: BRK
9FBF: BRK
9FC0: BRK
9FC1: BRK
9FC2: BRK
9FC3: BRK
9FC4: BRK
9FC5: BRK
9FC6: BRK
9FC7: BRK
9FC8: BRK
9FC9: BRK
9FCA: BRK
9FCB: BRK
9FCC: BRK
9FCD: BRK
9FCE: BRK
9FCF: BRK
9FD0: BRK
9FD1: BRK
9FD2: BRK
9FD3: BRK
9FD4: BRK
9FD5: BRK
9FD6: BRK
9FD7: BRK
9FD8: BRK
9FD9: BRK
9FDA: BRK
9FDB: BRK
9FDC: BRK
9FDD: BRK
9FDE: BRK
9FDF: BRK
9FE0: .byte $0F
9FE1: ORA ($05),Y
9FE3: BMI $9FF4
9FE5: .byte $02
9FE6: .byte $1C
9FE7: .byte $3C
9FE8: .byte $0F
9FE9: .byte $07
9FEA: ROL $38
9FEC: .byte $0F
9FED: BPL $A021
9FEF: BMI $A000
9FF1: ORA $30
9FF3: ROL $0F,X
9FF5: ORA $26
9FF7: BMI $A008
9FF9: .byte $07
9FFA: ROL $38
9FFC: .byte $0F
9FFD: ASL $27
9FFF: .byte $30
