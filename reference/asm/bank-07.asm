; LotW PRG bank 7 (8KB), disassembled at $8000
8000: .byte $B7
8001: .byte $B7
8002: .byte $B7
8003: .byte $B7
8004: .byte $B7
8005: .byte $B7
8006: .byte $B7
8007: .byte $B7
8008: .byte $B7
8009: .byte $B7
800A: .byte $B7
800B: .byte $B7
800C: RTI
800D: LDY $B4,X
800F: .byte $B7
8010: .byte $80
8011: .byte $80
8012: .byte $80
8013: .byte $80
8014: .byte $80
8015: .byte $80
8016: .byte $80
8017: .byte $B7
8018: .byte $B7
8019: .byte $B7
801A: LDY $B7,X
801C: EOR $B7B7,X
801F: LSR $B7,X
8021: LSR $5E,X
8023: .byte $B7
8024: .byte $B7
8025: .byte $B7
8026: LDY $B7,X
8028: EOR $B7B7,X
802B: EOR $B7,X
802D: EOR $5D,X
802F: .byte $B7
8030: .byte $B7
8031: .byte $B7
8032: .byte $B7
8033: .byte $B7
8034: LSR $B7B7,X
8037: LSR $B7,X
8039: LSR $5E,X
803B: LDY $B7,X
803D: .byte $B7
803E: .byte $B7
803F: EOR $5D,X
8041: .byte $B7
8042: .byte $B7
8043: EOR $B7,X
8045: EOR $5D,X
8047: .byte $B7
8048: .byte $B7
8049: .byte $B7
804A: LSR $5E56,X
804D: .byte $B7
804E: .byte $B7
804F: LSR $B7,X
8051: LSR $5E,X
8053: .byte $B7
8054: .byte $B7
8055: EOR $5D,X
8057: EOR $5D,X
8059: .byte $B7
805A: .byte $B7
805B: EOR $B7,X
805D: EOR $5D,X
805F: .byte $B7
8060: .byte $B7
8061: LSR $5E,X
8063: LSR $5E,X
8065: .byte $B7
8066: .byte $B7
8067: LSR $B7,X
8069: LSR $5E,X
806B: .byte $B7
806C: .byte $B7
806D: EOR $5D,X
806F: EOR $5D,X
8071: .byte $B7
8072: .byte $B7
8073: EOR $80,X
8075: .byte $80
8076: .byte $80
8077: .byte $B7
8078: .byte $B7
8079: LSR $5E,X
807B: LSR $5E,X
807D: .byte $B7
807E: .byte $B7
807F: .byte $B7
8080: LSR $5D56,X
8083: .byte $B7
8084: .byte $B7
8085: EOR $5D,X
8087: EOR $5D,X
8089: EOR $B7,X
808B: .byte $B7
808C: .byte $B7
808D: EOR $5D,X
808F: .byte $B7
8090: .byte $B7
8091: LSR $5E,X
8093: EOR $5E,X
8095: LSR $5E,X
8097: .byte $B7
8098: .byte $B7
8099: LSR $56,X
809B: .byte $B7
809C: .byte $B7
809D: EOR $5D,X
809F: EOR $5D,X
80A1: EOR $5D,X
80A3: .byte $B7
80A4: .byte $B7
80A5: EOR $5D,X
80A7: .byte $B7
80A8: .byte $B7
80A9: EOR $5E,X
80AB: LSR $5E,X
80AD: LSR $5E,X
80AF: .byte $B7
80B0: .byte $B7
80B1: LSR $5E,X
80B3: .byte $B7
80B4: .byte $B7
80B5: .byte $B7
80B6: EOR $5D55,X
80B9: BEQ $80F6
80BB: .byte $B7
80BC: .byte $B7
80BD: EOR $5D,X
80BF: .byte $B7
80C0: .byte $B7
80C1: .byte $B7
80C2: LSR $5E56,X
80C5: LSR $5E,X
80C7: .byte $B7
80C8: .byte $B7
80C9: LSR $5E,X
80CB: .byte $B7
80CC: .byte $B7
80CD: .byte $B7
80CE: .byte $B7
80CF: EOR $5D,X
80D1: EOR $5D,X
80D3: .byte $B7
80D4: .byte $B7
80D5: EOR $5D,X
80D7: .byte $B7
80D8: .byte $B7
80D9: .byte $B7
80DA: .byte $B7
80DB: JMP $4C4C
80DE: JMP $B7B7
80E1: JMP $B74C
80E4: .byte $B7
80E5: .byte $B7
80E6: .byte $B7
80E7: EOR $4D4D
80EA: EOR $B7B7
80ED: EOR $B74D
80F0: .byte $B7
80F1: .byte $B7
80F2: .byte $B7
80F3: LSR $5E,X
80F5: LSR $5E,X
80F7: .byte $B7
80F8: .byte $B7
80F9: LSR $5E,X
80FB: .byte $B7
80FC: .byte $B7
80FD: .byte $B7
80FE: EOR $5D55,X
8101: EOR $5D,X
8103: .byte $B7
8104: .byte $B7
8105: EOR $5D,X
8107: .byte $B7
8108: .byte $B7
8109: .byte $B7
810A: LSR $5E56,X
810D: BEQ $814A
810F: .byte $B7
8110: .byte $B7
8111: LSR $5E,X
8113: .byte $B7
8114: .byte $B7
8115: EOR $5D,X
8117: EOR $5D,X
8119: EOR $5D,X
811B: .byte $B7
811C: .byte $B7
811D: EOR $5D,X
811F: .byte $B7
8120: .byte $B7
8121: LSR $5E,X
8123: LSR $56,X
8125: LSR $5E,X
8127: .byte $B7
8128: .byte $B7
8129: LSR $5E,X
812B: .byte $B7
812C: .byte $B7
812D: EOR $5D,X
812F: EOR $5D,X
8131: EOR $5D,X
8133: .byte $B7
8134: .byte $B7
8135: EOR $5D,X
8137: .byte $B7
8138: .byte $B7
8139: LSR $5E,X
813B: LSR $5E,X
813D: LSR $B7,X
813F: .byte $B7
8140: .byte $B7
8141: LSR $5E,X
8143: .byte $B7
8144: .byte $B7
8145: EOR $5D,X
8147: EOR $5D,X
8149: .byte $B7
814A: .byte $B7
814B: .byte $B7
814C: EOR $5D55,X
814F: .byte $B7
8150: .byte $B7
8151: LSR $5E,X
8153: LSR $5E,X
8155: .byte $B7
8156: .byte $B7
8157: LSR $5E,X
8159: LSR $5E,X
815B: .byte $B7
815C: .byte $B7
815D: EOR $5D,X
815F: EOR $5D,X
8161: .byte $B7
8162: .byte $B7
8163: EOR $5D,X
8165: EOR $5D,X
8167: .byte $B7
8168: .byte $B7
8169: LSR $5E,X
816B: LSR $5E,X
816D: .byte $B7
816E: .byte $B7
816F: LSR $5E,X
8171: BEQ $81AE
8173: .byte $B7
8174: .byte $B7
8175: EOR $5D,X
8177: EOR $5D,X
8179: .byte $B7
817A: .byte $B7
817B: EOR $5D,X
817D: EOR $5D,X
817F: .byte $B7
8180: .byte $B7
8181: LSR $5E,X
8183: LSR $5E,X
8185: .byte $B7
8186: .byte $B7
8187: LSR $5E,X
8189: LSR $5E,X
818B: .byte $B7
818C: .byte $80
818D: .byte $80
818E: .byte $80
818F: .byte $80
8190: .byte $80
8191: .byte $B7
8192: .byte $B7
8193: EOR $5D,X
8195: EOR $5D,X
8197: .byte $B7
8198: .byte $B7
8199: LSR $5E,X
819B: LSR $B7,X
819D: .byte $B7
819E: .byte $B7
819F: LSR $5E,X
81A1: LSR $5E,X
81A3: .byte $B7
81A4: .byte $B7
81A5: EOR $5D,X
81A7: .byte $B7
81A8: .byte $B7
81A9: .byte $B7
81AA: EOR $5E55,X
81AD: BEQ $81EA
81AF: .byte $B7
81B0: .byte $B7
81B1: LSR $5E,X
81B3: .byte $B7
81B4: .byte $B7
81B5: LSR $5E,X
81B7: LSR $5E,X
81B9: LSR $5E,X
81BB: .byte $B7
81BC: .byte $B7
81BD: EOR $5D,X
81BF: .byte $B7
81C0: .byte $B7
81C1: EOR $5D,X
81C3: EOR $5D,X
81C5: EOR $5D,X
81C7: .byte $B7
81C8: .byte $B7
81C9: LSR $5E,X
81CB: .byte $B7
81CC: .byte $B7
81CD: LSR $5E,X
81CF: LSR $56,X
81D1: LSR $5E,X
81D3: .byte $B7
81D4: .byte $B7
81D5: EOR $5D,X
81D7: .byte $B7
81D8: .byte $B7
81D9: BVC $8227
81DB: JMP $4C4C
81DE: JMP $B7B7
81E1: LSR $5E,X
81E3: .byte $B7
81E4: .byte $B7
81E5: EOR ($4E),Y
81E7: LSR $B74E
81EA: LSR $B7B7
81ED: EOR $5E,X
81EF: .byte $B7
81F0: .byte $B7
81F1: BVC $8241
81F3: .byte $04
81F4: STA ($B7,X)
81F6: LSR $B7B7
81F9: LSR $5E,X
81FB: .byte $B7
81FC: .byte $B7
81FD: EOR ($4E),Y
81FF: LSR $B74E
8202: LSR $B7B7
8205: EOR $5D,X
8207: .byte $B7
8208: .byte $B7
8209: BVC $8259
820B: LSR $804E
820E: .byte $80
820F: .byte $B7
8210: .byte $B7
8211: .byte $B7
8212: .byte $B7
8213: .byte $B7
8214: .byte $B7
8215: EOR ($4E),Y
8217: .byte $53
8218: .byte $54
8219: .byte $54
821A: LSR $B7B7
821D: .byte $B7
821E: .byte $B7
821F: .byte $B7
8220: .byte $B7
8221: BVC $8271
8223: LSR $4E4E
8226: LSR $B7B7
8229: LSR $5E,X
822B: LSR $5E,X
822D: EOR ($4E),Y
822F: .byte $5C
8230: .byte $52
8231: ROR $52
8233: .byte $B7
8234: .byte $B7
8235: BVC $8283
8237: JMP $4C4C
823A: LSR $4A5F
823D: .byte $67
823E: LSR A
823F: .byte $B7
8240: .byte $B7
8241: EOR ($4E),Y
8243: LSR $4E4E
8246: LSR $4E4E
8249: LSR $B74E
824C: .byte $B7
824D: BVC $829D
824F: EOR $4E4E,Y
8252: LSR $4E4E
8255: LSR $B74E
8258: .byte $B7
8259: EOR ($4E),Y
825B: .byte $5A
825C: .byte $53
825D: .byte $54
825E: LSR $535A
8261: .byte $54
8262: LSR $B7B7
8265: BVC $82B5
8267: .byte $5B
8268: LSR $4E4E
826B: .byte $5B
826C: LSR $4E4E
826F: .byte $B7
8270: .byte $B7
8271: EOR ($4E),Y
8273: EOR $4E4E,Y
8276: LSR $4E59
8279: LSR $B74E
827C: .byte $B7
827D: BVC $82CD
827F: .byte $5A
8280: .byte $53
8281: .byte $54
8282: LSR $535A
8285: .byte $54
8286: LSR $B7B7
8289: EOR ($4E),Y
828B: .byte $5B
828C: LSR $4E4E
828F: .byte $5B
8290: LSR $4E4E
8293: .byte $B7
8294: .byte $B7
8295: BVC $82E5
8297: EOR $4E4E,Y
829A: LSR $4E4E
829D: LSR $B74E
82A0: .byte $B7
82A1: EOR ($4E),Y
82A3: .byte $5A
82A4: .byte $53
82A5: .byte $54
82A6: LSR $8080
82A9: .byte $80
82AA: .byte $80
82AB: .byte $B7
82AC: .byte $B7
82AD: BVC $82FD
82AF: .byte $5B
82B0: LSR $4E4E
82B3: .byte $B7
82B4: .byte $B7
82B5: .byte $B7
82B6: .byte $B7
82B7: .byte $B7
82B8: .byte $B7
82B9: EOR ($4D),Y
82BB: EOR $665C
82BE: .byte $52
82BF: LDY $B4,X
82C1: .byte $B7
82C2: .byte $B7
82C3: .byte $B7
82C4: .byte $B7
82C5: EOR $5D,X
82C7: .byte $62
82C8: .byte $5F
82C9: .byte $67
82CA: LSR A
82CB: .byte $B7
82CC: LDY $B4,X
82CE: .byte $B7
82CF: .byte $B7
82D0: .byte $B7
82D1: LSR $5E,X
82D3: .byte $63
82D4: LSR $4E4E
82D7: .byte $B7
82D8: .byte $B7
82D9: LDY $B7,X
82DB: .byte $B7
82DC: .byte $B7
82DD: EOR $5D,X
82DF: .byte $62
82E0: LSR $8105
82E3: .byte $B7
82E4: .byte $B7
82E5: LDY $B7,X
82E7: .byte $B7
82E8: .byte $B7
82E9: LSR $5E,X
82EB: .byte $63
82EC: LSR $4D4D
82EF: .byte $B7
82F0: EOR $B4,X
82F2: .byte $B7
82F3: .byte $B7
82F4: .byte $B7
82F5: .byte $B7
82F6: .byte $B7
82F7: .byte $B7
82F8: .byte $B7
82F9: .byte $B7
82FA: .byte $B7
82FB: .byte $B7
82FC: .byte $B7
82FD: .byte $B7
82FE: .byte $B7
82FF: .byte $B7
8300: ORA ($25,X)
8302: .byte $34
8303: .byte $47
8304: EOR $04,X
8306: ASL $01
8308: ROL $0080,X
830B: .byte $02
830C: BRK
830D: BRK
830E: BRK
830F: BRK
8310: ORA ($46,X)
8312: .byte $02
8313: .byte $3C
8314: .byte $02
8315: .byte $04
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
8320: EOR ($01,X)
8322: PHP
8323: RTI
8324: ASL $4D01,X
8327: .byte $02
8328: .byte $03
8329: .byte $02
832A: BRK
832B: BRK
832C: BRK
832D: BRK
832E: BRK
832F: BRK
8330: ADC ($02,X)
8332: ASL A
8333: LDY #$1E
8335: ORA ($6D,X)
8337: .byte $02
8338: BRK
8339: .byte $02
833A: BRK
833B: BRK
833C: BRK
833D: BRK
833E: BRK
833F: BRK
8340: EOR ($03,X)
8342: .byte $1A
8343: LDY #$1E
8345: ORA ($4D,X)
8347: .byte $02
8348: .byte $03
8349: .byte $02
834A: BRK
834B: BRK
834C: BRK
834D: BRK
834E: BRK
834F: BRK
8350: EOR ($01),Y
8352: BIT $20
8354: AND $5D01
8357: .byte $02
8358: BRK
8359: ORA ($00,X)
835B: BRK
835C: BRK
835D: BRK
835E: BRK
835F: BRK
8360: EOR ($02),Y
8362: .byte $3C
8363: JSR $012D
8366: EOR $0002,X
8369: ORA ($00,X)
836B: BRK
836C: BRK
836D: BRK
836E: BRK
836F: BRK
8370: EOR ($03),Y
8372: AND ($A0),Y
8374: ROL $5D01
8377: .byte $02
8378: BRK
8379: ORA ($00,X)
837B: BRK
837C: BRK
837D: BRK
837E: BRK
837F: BRK
8380: ADC ($01),Y
8382: .byte $14
8383: RTS
8384: ASL $7D02,X
8387: ORA ($02,X)
8389: .byte $02
838A: BRK
838B: BRK
838C: BRK
838D: BRK
838E: BRK
838F: BRK
8390: ADC ($02),Y
8392: AND $A0
8394: ASL $7D01,X
8397: ORA ($02,X)
8399: .byte $02
839A: BRK
839B: BRK
839C: BRK
839D: BRK
839E: BRK
839F: BRK
83A0: ADC ($03),Y
83A2: ROL A
83A3: .byte $80
83A4: ASL $7D01,X
83A7: ORA ($02,X)
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
83E5: BRK
83E6: ASL $27,X
83E8: .byte $0F
83E9: .byte $07
83EA: .byte $17
83EB: .byte $37
83EC: .byte $0F
83ED: .byte $0C
83EE: ORA $26,X
83F0: .byte $0F
83F1: ORA $30
83F3: ROL $0F,X
83F5: ORA $26
83F7: BMI $8408
83F9: .byte $1A
83FA: ROL A
83FB: BMI $840C
83FD: ORA ($3C),Y
83FF: BMI $847D
8401: .byte $7C
8402: .byte $7C
8403: .byte $7C
8404: .byte $7C
8405: .byte $7C
8406: .byte $7C
8407: .byte $7C
8408: .byte $7C
8409: .byte $7C
840A: .byte $7C
840B: .byte $7C
840C: .byte $7C
840D: STA $7C7C,X
8410: STA $9D9D,X
8413: STA $9D9D,X
8416: STA $7C80,X
8419: .byte $C2
841A: .byte $C2
841B: .byte $7C
841C: STA $9D9D,X
841F: STA $7C9D,X
8422: .byte $7C
8423: .byte $7C
8424: .byte $7C
8425: .byte $7C
8426: .byte $C2
8427: .byte $52
8428: STA $7C9D,X
842B: STA $9D9D,X
842E: STA $7C7C,X
8431: .byte $C2
8432: .byte $C2
8433: .byte $7C
8434: STA $7C9D,X
8437: STA $9D9D,X
843A: STA $7C7C,X
843D: .byte $C2
843E: .byte $7C
843F: .byte $7C
8440: .byte $7C
8441: INC $9D7C,X
8444: STA $9D9D,X
8447: .byte $7C
8448: .byte $7C
8449: .byte $C2
844A: .byte $7C
844B: STA $9D7C,X
844E: .byte $7C
844F: STA $9D9D,X
8452: STA $7C7C,X
8455: .byte $C2
8456: .byte $7C
8457: STA $FE7C,X
845A: .byte $7C
845B: STA $9D9D,X
845E: STA $7C7C,X
8461: .byte $C2
8462: .byte $7C
8463: STA $9D7C,X
8466: .byte $7C
8467: STA $9D9D,X
846A: STA $7C7C,X
846D: .byte $C2
846E: .byte $7C
846F: STA $9D7C,X
8472: .byte $7C
8473: STA $9D9D,X
8476: STA $7C7C,X
8479: .byte $C2
847A: .byte $7C
847B: STA $9D52,X
847E: .byte $7C
847F: STA $9D7C,X
8482: STA $7C7C,X
8485: .byte $C2
8486: .byte $7C
8487: STA $7C7C,X
848A: .byte $7C
848B: STA $7C7C,X
848E: .byte $7C
848F: .byte $7C
8490: .byte $7C
8491: .byte $C2
8492: .byte $7C
8493: STA $9D7C,X
8496: STA $7C9D,X
8499: STA $7C9D,X
849C: .byte $7C
849D: .byte $C2
849E: .byte $7C
849F: STA $9D7C,X
84A2: .byte $7C
84A3: STA $9D9D,X
84A6: STA $7C7C,X
84A9: .byte $C2
84AA: .byte $7C
84AB: STA $9D7C,X
84AE: .byte $7C
84AF: STA $9D9D,X
84B2: STA $7C7C,X
84B5: .byte $C2
84B6: .byte $7C
84B7: STA $9D7C,X
84BA: .byte $7C
84BB: .byte $7C
84BC: .byte $7C
84BD: .byte $7C
84BE: STA $7C7C,X
84C1: .byte $C2
84C2: .byte $7C
84C3: STA $9D7C,X
84C6: STA $9D9D,X
84C9: .byte $7C
84CA: STA $7C7C,X
84CD: .byte $C2
84CE: .byte $7C
84CF: STA $7C7C,X
84D2: .byte $7C
84D3: .byte $7C
84D4: STA $9D7C,X
84D7: .byte $7C
84D8: .byte $7C
84D9: .byte $C2
84DA: .byte $7C
84DB: STA $9D9D,X
84DE: STA $9D9D,X
84E1: .byte $7C
84E2: STA $7C7C,X
84E5: .byte $C2
84E6: .byte $7C
84E7: STA $7C9D,X
84EA: .byte $7C
84EB: .byte $7C
84EC: .byte $7C
84ED: .byte $7C
84EE: STA $7C7C,X
84F1: .byte $C2
84F2: .byte $7C
84F3: STA $7C9D,X
84F6: STA $9D9D,X
84F9: STA $7C9D,X
84FC: .byte $7C
84FD: .byte $C2
84FE: .byte $7C
84FF: STA $9D9D,X
8502: STA $9D7C,X
8505: STA $7C9D,X
8508: .byte $7C
8509: .byte $C2
850A: .byte $7C
850B: STA $9D9D,X
850E: STA $9D9D,X
8511: STA $7C9D,X
8514: .byte $7C
8515: .byte $C2
8516: .byte $7C
8517: STA $9D9D,X
851A: STA $9D9D,X
851D: STA $7C9D,X
8520: .byte $7C
8521: .byte $C2
8522: .byte $7C
8523: STA $9D9D,X
8526: STA $9D7C,X
8529: STA $7C9D,X
852C: .byte $7C
852D: .byte $C2
852E: .byte $7C
852F: STA $9D9D,X
8532: STA $9D9D,X
8535: STA $7C9D,X
8538: .byte $7C
8539: .byte $C2
853A: .byte $7C
853B: STA $9D9D,X
853E: STA $9D9D,X
8541: STA $7C9D,X
8544: .byte $7C
8545: .byte $C2
8546: .byte $7C
8547: STA $9D9D,X
854A: STA $9D7C,X
854D: STA $7C9D,X
8550: .byte $7C
8551: .byte $C2
8552: .byte $7C
8553: .byte $7C
8554: STA $9D9D,X
8557: STA $9D9D,X
855A: STA $7C7C,X
855D: .byte $9E
855E: .byte $7C
855F: .byte $7C
8560: STA $9D9D,X
8563: STA $9D9D,X
8566: .byte $7C
8567: .byte $7C
8568: .byte $7C
8569: .byte $9E
856A: STA $9D7C,X
856D: STA $7C9D,X
8570: STA $9D9D,X
8573: .byte $7C
8574: .byte $7C
8575: .byte $7C
8576: STA $7C9D,X
8579: .byte $7C
857A: STA $9D7C,X
857D: STA $7C9D,X
8580: .byte $7C
8581: .byte $7C
8582: .byte $7C
8583: STA $7C7C,X
8586: STA $9D7C,X
8589: STA $7C9D,X
858C: .byte $7C
858D: .byte $9E
858E: STA $7C9D,X
8591: .byte $7C
8592: STA $9D7C,X
8595: STA $7C9D,X
8598: .byte $7C
8599: .byte $9E
859A: .byte $7C
859B: STA $7C9D,X
859E: STA $9D7C,X
85A1: STA $7C9D,X
85A4: .byte $7C
85A5: .byte $9E
85A6: .byte $7C
85A7: .byte $7C
85A8: STA $9D9D,X
85AB: .byte $7C
85AC: STA $9D9D,X
85AF: .byte $7C
85B0: .byte $7C
85B1: .byte $9E
85B2: .byte $7C
85B3: .byte $7C
85B4: .byte $7C
85B5: STA $7C9D,X
85B8: .byte $7C
85B9: STA $7C7C,X
85BC: .byte $7C
85BD: .byte $9E
85BE: .byte $7C
85BF: .byte $7C
85C0: .byte $7C
85C1: STA $9D9D,X
85C4: .byte $80
85C5: .byte $80
85C6: .byte $80
85C7: .byte $7C
85C8: .byte $7C
85C9: .byte $9E
85CA: STA $7C7C,X
85CD: STA $7C7C,X
85D0: STA $7C9D,X
85D3: .byte $7C
85D4: .byte $7C
85D5: .byte $7C
85D6: STA $7C7C,X
85D9: STA $9D9D,X
85DC: .byte $7C
85DD: STA $7C7C,X
85E0: .byte $7C
85E1: .byte $9E
85E2: STA $7C7C,X
85E5: STA $9D9D,X
85E8: .byte $7C
85E9: STA $7C7C,X
85EC: .byte $7C
85ED: .byte $9E
85EE: .byte $7C
85EF: .byte $7C
85F0: .byte $7C
85F1: STA $7C9D,X
85F4: .byte $7C
85F5: .byte $7C
85F6: .byte $7C
85F7: .byte $7C
85F8: .byte $7C
85F9: .byte $9E
85FA: STA $7C9D,X
85FD: STA $9D7C,X
8600: STA $7C9D,X
8603: .byte $7C
8604: .byte $7C
8605: .byte $7C
8606: .byte $7C
8607: STA $9D7C,X
860A: .byte $7C
860B: STA $9D7C,X
860E: STA $7C7C,X
8611: .byte $7C
8612: STA $7C9D,X
8615: STA $9D9D,X
8618: STA $9D7C,X
861B: .byte $7C
861C: .byte $7C
861D: .byte $7C
861E: STA $9D7C,X
8621: .byte $7C
8622: .byte $7C
8623: .byte $7C
8624: .byte $7C
8625: .byte $7C
8626: STA $7C7C,X
8629: .byte $7C
862A: STA $9D7C,X
862D: .byte $7C
862E: STA $9D9D,X
8631: .byte $7C
8632: STA $7C7C,X
8635: .byte $7C
8636: STA $9D7C,X
8639: .byte $7C
863A: STA $7C7C,X
863D: STA $7C9D,X
8640: .byte $7C
8641: .byte $7C
8642: STA $9D7C,X
8645: .byte $7C
8646: STA $7C7C,X
8649: STA $7C9D,X
864C: .byte $7C
864D: .byte $9E
864E: STA $9D7C,X
8651: .byte $7C
8652: STA $7C9D,X
8655: STA $7C9D,X
8658: .byte $7C
8659: .byte $9E
865A: .byte $7C
865B: STA $9D9D,X
865E: .byte $7C
865F: STA $9D7C,X
8662: STA $7C7C,X
8665: .byte $9E
8666: .byte $7C
8667: STA $9D7C,X
866A: .byte $7C
866B: STA $9D7C,X
866E: STA $7C7C,X
8671: .byte $9E
8672: .byte $7C
8673: STA $9D7C,X
8676: .byte $7C
8677: STA $9D7C,X
867A: STA $7C7C,X
867D: .byte $9E
867E: .byte $7C
867F: STA $9D7C,X
8682: .byte $7C
8683: STA $9D7C,X
8686: STA $7C80,X
8689: .byte $9E
868A: .byte $7C
868B: STA $9D7C,X
868E: .byte $7C
868F: STA $9D7C,X
8692: STA $7C9D,X
8695: .byte $9E
8696: .byte $7C
8697: STA $9D7C,X
869A: .byte $7C
869B: STA $9D7C,X
869E: STA $7C9D,X
86A1: .byte $9E
86A2: .byte $7C
86A3: STA $9D7C,X
86A6: .byte $7C
86A7: STA $7C7C,X
86AA: .byte $7C
86AB: .byte $7C
86AC: .byte $7C
86AD: .byte $9E
86AE: .byte $7C
86AF: STA $9D7C,X
86B2: .byte $7C
86B3: STA $7C7C,X
86B6: .byte $7C
86B7: .byte $7C
86B8: .byte $7C
86B9: .byte $9E
86BA: .byte $7C
86BB: STA $9D7C,X
86BE: .byte $7C
86BF: STA $7C7C,X
86C2: .byte $7C
86C3: .byte $7C
86C4: .byte $7C
86C5: .byte $9E
86C6: .byte $80
86C7: STA $9D80,X
86CA: .byte $80
86CB: STA $9D80,X
86CE: STA $7C7C,X
86D1: .byte $9E
86D2: STA $9D9D,X
86D5: STA $9D9D,X
86D8: STA $9D9D,X
86DB: .byte $7C
86DC: .byte $7C
86DD: .byte $9E
86DE: STA $9D9D,X
86E1: STA $9D9D,X
86E4: STA $9D9D,X
86E7: .byte $7C
86E8: .byte $7C
86E9: .byte $9E
86EA: STA $9D9D,X
86ED: STA $9D9D,X
86F0: STA $9D9D,X
86F3: .byte $7C
86F4: .byte $7C
86F5: .byte $7C
86F6: STA $9D9D,X
86F9: STA $9D9D,X
86FC: .byte $7C
86FD: STA $7C7C,X
8700: .byte $07
8701: .byte $2B
8702: .byte $12
8703: STA $009E,X
8706: .byte $02
8707: ORA ($01,X)
8709: BPL $8720
870B: .byte $02
870C: BRK
870D: BRK
870E: BRK
870F: BRK
8710: BRK
8711: INY
8712: BRK
8713: INY
8714: .byte $02
8715: .byte $04
8716: BRK
8717: BRK
8718: BRK
8719: BRK
871A: BRK
871B: BRK
871C: BRK
871D: BRK
871E: BRK
871F: BRK
8720: ADC ($01),Y
8722: ASL $30,X
8724: .byte $23
8725: ORA ($7D,X)
8727: .byte $02
8728: ORA $02
872A: BRK
872B: BRK
872C: BRK
872D: BRK
872E: BRK
872F: BRK
8730: ADC ($01),Y
8732: AND $2310,X
8735: ORA ($7D,X)
8737: .byte $02
8738: ORA $02
873A: BRK
873B: BRK
873C: BRK
873D: BRK
873E: BRK
873F: BRK
8740: ADC ($03),Y
8742: .byte $1A
8743: BMI $8768
8745: ORA ($7D,X)
8747: .byte $02
8748: ORA $04
874A: BRK
874B: BRK
874C: BRK
874D: BRK
874E: BRK
874F: BRK
8750: ADC ($03),Y
8752: .byte $03
8753: RTI
8754: .byte $23
8755: ORA ($7D,X)
8757: .byte $02
8758: ORA $04
875A: BRK
875B: BRK
875C: BRK
875D: BRK
875E: BRK
875F: BRK
8760: EOR ($03,X)
8762: .byte $17
8763: LDY #$23
8765: .byte $02
8766: EOR $0002
8769: ORA ($00,X)
876B: BRK
876C: BRK
876D: BRK
876E: BRK
876F: BRK
8770: EOR ($03,X)
8772: .byte $2F
8773: LDY #$23
8775: .byte $02
8776: EOR $0002
8779: ORA ($00,X)
877B: BRK
877C: BRK
877D: BRK
877E: BRK
877F: BRK
8780: ADC ($03,X)
8782: ORA $60
8784: .byte $23
8785: .byte $02
8786: ADC $0302
8789: .byte $02
878A: BRK
878B: BRK
878C: BRK
878D: BRK
878E: BRK
878F: BRK
8790: ADC ($03,X)
8792: PHP
8793: LDY #$23
8795: .byte $02
8796: ADC $0302
8799: .byte $02
879A: BRK
879B: BRK
879C: BRK
879D: BRK
879E: BRK
879F: BRK
87A0: ADC ($03,X)
87A2: .byte $3C
87A3: LDY #$23
87A5: .byte $02
87A6: ADC $0002
87A9: .byte $02
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
87E6: .byte $17
87E7: .byte $27
87E8: .byte $0F
87E9: ORA ($12,X)
87EB: AND ($0F),Y
87ED: BRK
87EE: BPL $8820
87F0: .byte $0F
87F1: ORA $30
87F3: ROL $0F,X
87F5: ORA $26
87F7: BMI $8808
87F9: .byte $07
87FA: ASL $27,X
87FC: .byte $0F
87FD: .byte $02
87FE: .byte $1B
87FF: BMI $887D
8801: .byte $7C
8802: .byte $9E
8803: STA $9D9D,X
8806: STA $7C9D,X
8809: STA $7C7C,X
880C: .byte $7C
880D: CMP $9D9E
8810: STA $9D9D,X
8813: STA $9D9D,X
8816: BEQ $8894
8818: .byte $7C
8819: CMP $529E
881C: STA $9D9D,X
881F: STA $9D52,X
8822: BEQ $88A0
8824: .byte $7C
8825: CMP $529E
8828: STA $529D,X
882B: STA $9D52,X
882E: BEQ $88AC
8830: .byte $7C
8831: CMP $529E
8834: STA $529D,X
8837: STA $9D9D,X
883A: BEQ $88B8
883C: .byte $7C
883D: .byte $CB
883E: .byte $CB
883F: .byte $CB
8840: .byte $CB
8841: .byte $CB
8842: .byte $7C
8843: .byte $CB
8844: .byte $CB
8845: .byte $CB
8846: .byte $CB
8847: .byte $7C
8848: .byte $7C
8849: CPY $CCCC
884C: CPY $CCCC
884F: CPY $CCCC
8852: CPY $7C7C
8855: CMP #$DB
8857: .byte $D7
8858: CMP #$DB
885A: .byte $D7
885B: CMP #$DB
885D: .byte $D7
885E: CMP #$7C
8860: .byte $7C
8861: CMP #$52
8863: .byte $DC
8864: CMP #$52
8866: .byte $DC
8867: CMP #$52
8869: .byte $DC
886A: CMP #$7C
886C: .byte $7C
886D: CPY $CCCC
8870: CPY $CCCC
8873: CPY $CCCC
8876: CPY $7C7C
8879: DEX
887A: DEX
887B: DEX
887C: DEX
887D: DEX
887E: DEX
887F: DEX
8880: DEX
8881: DEX
8882: DEX
8883: .byte $7C
8884: .byte $7C
8885: .byte $7C
8886: .byte $7C
8887: .byte $7C
8888: .byte $7C
8889: .byte $7C
888A: .byte $7C
888B: .byte $7C
888C: .byte $7C
888D: STA $7C7C,X
8890: RTI
8891: RTI
8892: .byte $7C
8893: .byte $7C
8894: .byte $7C
8895: .byte $7C
8896: .byte $7C
8897: .byte $52
8898: .byte $52
8899: STA $7C7C,X
889C: .byte $7C
889D: .byte $9E
889E: .byte $7C
889F: .byte $7C
88A0: .byte $7C
88A1: .byte $7C
88A2: .byte $52
88A3: .byte $52
88A4: .byte $52
88A5: STA $7CF0,X
88A8: .byte $7C
88A9: .byte $9E
88AA: .byte $7C
88AB: .byte $7C
88AC: .byte $7C
88AD: .byte $7C
88AE: .byte $52
88AF: .byte $52
88B0: .byte $7C
88B1: STA $7CF0,X
88B4: .byte $7C
88B5: .byte $9E
88B6: STA $7C7C,X
88B9: .byte $7C
88BA: .byte $52
88BB: .byte $7C
88BC: .byte $7C
88BD: STA $7CF0,X
88C0: .byte $7C
88C1: .byte $9E
88C2: STA $7C7C,X
88C5: .byte $52
88C6: .byte $52
88C7: .byte $7C
88C8: .byte $7C
88C9: STA $7CF0,X
88CC: .byte $7C
88CD: .byte $9E
88CE: STA $7C9D,X
88D1: .byte $52
88D2: .byte $7C
88D3: .byte $7C
88D4: .byte $7C
88D5: STA $7CF0,X
88D8: .byte $7C
88D9: .byte $9E
88DA: STA $7C9D,X
88DD: .byte $52
88DE: .byte $7C
88DF: .byte $7C
88E0: .byte $7C
88E1: STA $7CF0,X
88E4: .byte $7C
88E5: .byte $9E
88E6: STA $7C9D,X
88E9: .byte $52
88EA: .byte $7C
88EB: .byte $7C
88EC: .byte $7C
88ED: STA $7CF0,X
88F0: .byte $7C
88F1: .byte $9E
88F2: STA $7C9D,X
88F5: .byte $52
88F6: .byte $7C
88F7: .byte $7C
88F8: .byte $7C
88F9: STA $7CF0,X
88FC: .byte $7C
88FD: .byte $9E
88FE: STA $9D9D,X
8901: STA $4040,X
8904: RTI
8905: RTI
8906: LDY $7C7C,X
8909: .byte $9E
890A: STA $BCBC,X
890D: LDY $BCBC,X
8910: LDY $BCBC,X
8913: .byte $7C
8914: .byte $7C
8915: .byte $9E
8916: STA $BCBC,X
8919: LDY $BCBC,X
891C: LDY $BCBC,X
891F: .byte $7C
8920: .byte $7C
8921: .byte $9E
8922: STA $9D9D,X
8925: LDY $BCBC,X
8928: LDY $BCBC,X
892B: .byte $7C
892C: .byte $7C
892D: .byte $9E
892E: STA $9D9D,X
8931: STA $9D9D,X
8934: STA $BC9D,X
8937: .byte $7C
8938: .byte $7C
8939: DEC $CBCB
893C: .byte $CB
893D: .byte $CB
893E: .byte $CB
893F: .byte $CB
8940: .byte $CB
8941: .byte $CB
8942: .byte $CB
8943: .byte $CB
8944: .byte $7C
8945: BNE $8913
8947: CPY $CCCC
894A: CPY $CCCC
894D: CPY $CCCC
8950: .byte $7C
8951: .byte $CF
8952: DEX
8953: DEX
8954: DEX
8955: DEX
8956: DEX
8957: DEX
8958: DEX
8959: DEX
895A: DEX
895B: .byte $7C
895C: .byte $7C
895D: .byte $9E
895E: STA $C9D0,X
8961: CMP #$C8
8963: CMP #$C8
8965: CMP #$C8
8967: .byte $7C
8968: .byte $7C
8969: .byte $9E
896A: STA $C9D0,X
896D: CMP #$C9
896F: .byte $DB
8970: CLD
8971: .byte $D7
8972: .byte $DB
8973: .byte $7C
8974: .byte $7C
8975: .byte $9E
8976: STA $C9D0,X
8979: CMP #$C8
897B: LDY $D98D,X
897E: BCS $89FC
8980: .byte $7C
8981: .byte $9E
8982: STA $DBD0,X
8985: CLD
8986: CLD
8987: LDY $D98D,X
898A: BCS $8A08
898C: .byte $7C
898D: .byte $72
898E: ADC ($71),Y
8990: ADC ($BC),Y
8992: .byte $92
8993: LDY $7171,X
8996: SEI
8997: .byte $7C
8998: .byte $7C
8999: .byte $7C
899A: .byte $7C
899B: BNE $8966
899D: LDY $BC92,X
89A0: STA $B0D9
89A3: .byte $7C
89A4: .byte $7C
89A5: .byte $7C
89A6: .byte $7C
89A7: BNE $8972
89A9: .byte $92
89AA: .byte $92
89AB: LDY $D98D,X
89AE: BCS $8A2C
89B0: .byte $7C
89B1: .byte $7C
89B2: .byte $7C
89B3: BNE $897E
89B5: LDY $BC92,X
89B8: STA $B0D9
89BB: .byte $7C
89BC: .byte $7C
89BD: .byte $7C
89BE: .byte $7C
89BF: BNE $899C
89C1: LDY $9292,X
89C4: STA $B0D9
89C7: .byte $7C
89C8: .byte $7C
89C9: .byte $7C
89CA: .byte $72
89CB: ADC ($71),Y
89CD: ADC ($71),Y
89CF: LDY $7171,X
89D2: SEI
89D3: .byte $7C
89D4: .byte $7C
89D5: .byte $7C
89D6: .byte $7C
89D7: BNE $89B4
89D9: CLD
89DA: .byte $D7
89DB: LDY $DC8D,X
89DE: BCS $8A5C
89E0: .byte $7C
89E1: .byte $7C
89E2: .byte $7C
89E3: .byte $7C
89E4: .byte $7C
89E5: .byte $7C
89E6: .byte $DA
89E7: RTI
89E8: RTI
89E9: RTI
89EA: RTI
89EB: RTI
89EC: .byte $7C
89ED: .byte $CB
89EE: .byte $CB
89EF: .byte $CB
89F0: .byte $CB
89F1: .byte $CB
89F2: .byte $CB
89F3: .byte $CB
89F4: .byte $CB
89F5: .byte $CB
89F6: .byte $CB
89F7: .byte $7C
89F8: .byte $7C
89F9: CPY $CCCC
89FC: CPY $CCCC
89FF: CPY $CCCC
8A02: CPY $7C7C
8A05: DEX
8A06: DEX
8A07: DEX
8A08: DEX
8A09: DEX
8A0A: DEX
8A0B: DEX
8A0C: DEX
8A0D: DEX
8A0E: DEX
8A0F: .byte $7C
8A10: .byte $7C
8A11: CMP $C9C9,Y
8A14: CMP #$C9
8A16: CMP #$C9
8A18: CMP #$C9
8A1A: CMP #$7C
8A1C: .byte $7C
8A1D: .byte $DA
8A1E: .byte $D7
8A1F: .byte $DB
8A20: CLD
8A21: CLD
8A22: CLD
8A23: CLD
8A24: CLD
8A25: CLD
8A26: .byte $D7
8A27: .byte $7C
8A28: .byte $7C
8A29: .byte $7C
8A2A: .byte $82
8A2B: .byte $7C
8A2C: ROR $7E7C,X
8A2F: .byte $7C
8A30: ROR $D97C,X
8A33: .byte $7C
8A34: .byte $7C
8A35: .byte $7C
8A36: .byte $82
8A37: .byte $7C
8A38: CMP $D97C,Y
8A3B: .byte $7C
8A3C: CMP $D97C,Y
8A3F: .byte $7C
8A40: .byte $7C
8A41: .byte $7C
8A42: .byte $82
8A43: .byte $7C
8A44: CMP $D97C,Y
8A47: .byte $7C
8A48: CMP $D97C,Y
8A4B: .byte $7C
8A4C: .byte $7C
8A4D: .byte $7C
8A4E: .byte $82
8A4F: .byte $7C
8A50: CMP $D97C,Y
8A53: .byte $7C
8A54: CMP $D97C,Y
8A57: .byte $7C
8A58: .byte $7C
8A59: .byte $7C
8A5A: .byte $82
8A5B: .byte $7C
8A5C: CMP $D97C,Y
8A5F: .byte $7C
8A60: CMP $D97C,Y
8A63: .byte $7C
8A64: .byte $7C
8A65: .byte $7C
8A66: .byte $82
8A67: .byte $7C
8A68: CMP $D952,Y
8A6B: .byte $7C
8A6C: CMP $D97C,Y
8A6F: .byte $7C
8A70: .byte $7C
8A71: .byte $7C
8A72: .byte $82
8A73: .byte $7C
8A74: CMP $D97C,Y
8A77: .byte $7C
8A78: CMP $D97C,Y
8A7B: .byte $7C
8A7C: .byte $7C
8A7D: .byte $7C
8A7E: .byte $82
8A7F: .byte $7C
8A80: CMP $D97C,Y
8A83: .byte $7C
8A84: CMP $D97C,Y
8A87: .byte $7C
8A88: .byte $7C
8A89: .byte $7C
8A8A: .byte $82
8A8B: .byte $7C
8A8C: CMP $D97C,Y
8A8F: .byte $7C
8A90: CMP $D97C,Y
8A93: .byte $7C
8A94: .byte $7C
8A95: .byte $7C
8A96: .byte $82
8A97: .byte $7C
8A98: CMP $D97C,Y
8A9B: .byte $7C
8A9C: CMP $D97C,Y
8A9F: .byte $7C
8AA0: .byte $7C
8AA1: .byte $7C
8AA2: .byte $82
8AA3: .byte $7C
8AA4: CMP $D97C,Y
8AA7: .byte $7C
8AA8: CMP $D97C,Y
8AAB: .byte $7C
8AAC: .byte $7C
8AAD: .byte $7C
8AAE: .byte $82
8AAF: .byte $7C
8AB0: CMP $D97C,Y
8AB3: .byte $7C
8AB4: CMP $D97C,Y
8AB7: .byte $7C
8AB8: .byte $7C
8AB9: .byte $7C
8ABA: .byte $82
8ABB: .byte $7C
8ABC: CMP $D97C,Y
8ABF: .byte $7C
8AC0: CMP $D97C,Y
8AC3: .byte $7C
8AC4: .byte $7C
8AC5: .byte $7C
8AC6: .byte $82
8AC7: .byte $7C
8AC8: ROR $D97C,X
8ACB: .byte $7C
8ACC: CMP $D97C,Y
8ACF: .byte $7C
8AD0: .byte $7C
8AD1: .byte $7C
8AD2: .byte $82
8AD3: .byte $7C
8AD4: CMP $DA7C,Y
8AD7: .byte $7C
8AD8: .byte $DA
8AD9: .byte $7C
8ADA: CMP $7C7C,Y
8ADD: .byte $7C
8ADE: .byte $82
8ADF: .byte $7C
8AE0: CMP $7E7C,Y
8AE3: .byte $7C
8AE4: ROR $DC7C,X
8AE7: .byte $7C
8AE8: .byte $7C
8AE9: .byte $7C
8AEA: .byte $82
8AEB: .byte $7C
8AEC: .byte $DA
8AED: CLD
8AEE: RTI
8AEF: RTI
8AF0: RTI
8AF1: RTI
8AF2: RTI
8AF3: .byte $7C
8AF4: .byte $7C
8AF5: .byte $7C
8AF6: CMP $7C7C,Y
8AF9: .byte $7C
8AFA: .byte $7C
8AFB: .byte $7C
8AFC: .byte $7C
8AFD: .byte $7C
8AFE: .byte $7C
8AFF: .byte $7C
8B00: .byte $07
8B01: ROL $12
8B03: .byte $C7
8B04: .byte $C7
8B05: BRK
8B06: .byte $02
8B07: ORA ($15,X)
8B09: BCC $8B0B
8B0B: .byte $02
8B0C: BRK
8B0D: BRK
8B0E: BRK
8B0F: BRK
8B10: BRK
8B11: INY
8B12: BRK
8B13: INY
8B14: .byte $02
8B15: .byte $04
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
8B20: EOR ($02,X)
8B22: BRK
8B23: BRK
8B24: .byte $32
8B25: ORA ($4D,X)
8B27: .byte $03
8B28: ORA ($0B,X)
8B2A: BRK
8B2B: BRK
8B2C: BRK
8B2D: BRK
8B2E: BRK
8B2F: BRK
8B30: EOR ($02,X)
8B32: BRK
8B33: BRK
8B34: .byte $32
8B35: ORA ($4D,X)
8B37: .byte $03
8B38: ORA ($0B,X)
8B3A: BRK
8B3B: BRK
8B3C: BRK
8B3D: BRK
8B3E: BRK
8B3F: BRK
8B40: EOR ($03,X)
8B42: ORA $3240,Y
8B45: ORA ($4D,X)
8B47: .byte $03
8B48: .byte $07
8B49: .byte $0B
8B4A: BRK
8B4B: BRK
8B4C: BRK
8B4D: BRK
8B4E: BRK
8B4F: BRK
8B50: ADC ($01,X)
8B52: BMI $8B74
8B54: .byte $3C
8B55: ORA ($6D,X)
8B57: .byte $02
8B58: .byte $02
8B59: .byte $02
8B5A: BRK
8B5B: BRK
8B5C: BRK
8B5D: BRK
8B5E: BRK
8B5F: BRK
8B60: ADC ($01,X)
8B62: AND $20,X
8B64: .byte $32
8B65: ORA ($6D,X)
8B67: .byte $02
8B68: .byte $02
8B69: .byte $02
8B6A: BRK
8B6B: BRK
8B6C: BRK
8B6D: BRK
8B6E: BRK
8B6F: BRK
8B70: ADC ($03,X)
8B72: SEC
8B73: JSR $0137
8B76: ADC $0202
8B79: .byte $02
8B7A: BRK
8B7B: BRK
8B7C: BRK
8B7D: BRK
8B7E: BRK
8B7F: BRK
8B80: EOR ($01),Y
8B82: .byte $03
8B83: BCC $8BA8
8B85: ORA ($5D,X)
8B87: .byte $02
8B88: .byte $02
8B89: ORA ($00,X)
8B8B: BRK
8B8C: BRK
8B8D: BRK
8B8E: BRK
8B8F: BRK
8B90: EOR ($03),Y
8B92: BPL $8B24
8B94: .byte $23
8B95: ORA ($5D,X)
8B97: .byte $02
8B98: .byte $02
8B99: .byte $02
8B9A: BRK
8B9B: BRK
8B9C: BRK
8B9D: BRK
8B9E: BRK
8B9F: BRK
8BA0: EOR ($01),Y
8BA2: AND $2360,Y
8BA5: ORA ($5D,X)
8BA7: .byte $02
8BA8: .byte $02
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
8BE5: ASL A
8BE6: .byte $17
8BE7: .byte $27
8BE8: .byte $0F
8BE9: ORA ($12,X)
8BEB: AND ($0F),Y
8BED: BRK
8BEE: BPL $8C20
8BF0: .byte $0F
8BF1: ORA $30
8BF3: ROL $0F,X
8BF5: ORA $26
8BF7: BMI $8C08
8BF9: .byte $3C
8BFA: ORA ($30),Y
8BFC: .byte $0F
8BFD: ORA $26
8BFF: SEC
8C00: .byte $7C
8C01: .byte $7C
8C02: .byte $6F
8C03: .byte $7C
8C04: .byte $7C
8C05: .byte $7C
8C06: .byte $7C
8C07: .byte $7C
8C08: .byte $7C
8C09: .byte $7C
8C0A: .byte $7C
8C0B: .byte $7C
8C0C: .byte $7C
8C0D: .byte $6F
8C0E: .byte $6F
8C0F: .byte $6F
8C10: .byte $7C
8C11: .byte $6F
8C12: .byte $6F
8C13: .byte $6F
8C14: .byte $6F
8C15: .byte $6F
8C16: .byte $6F
8C17: .byte $7C
8C18: .byte $7C
8C19: .byte $CB
8C1A: .byte $CB
8C1B: .byte $CB
8C1C: .byte $CB
8C1D: .byte $7C
8C1E: .byte $CB
8C1F: .byte $CB
8C20: .byte $CB
8C21: .byte $CB
8C22: .byte $CB
8C23: CPY #$7C
8C25: CPY $CCCC
8C28: CPY $7CCC
8C2B: CPY $CCCC
8C2E: CPY $7C7C
8C31: DEX
8C32: DEX
8C33: DEX
8C34: DEX
8C35: DEX
8C36: .byte $7C
8C37: DEX
8C38: DEX
8C39: DEX
8C3A: DEX
8C3B: .byte $7C
8C3C: .byte $7C
8C3D: .byte $6F
8C3E: SBC #$D9
8C40: .byte $7C
8C41: CMP $D97C,Y
8C44: CMP #$C9
8C46: CMP #$7C
8C48: .byte $7C
8C49: .byte $6F
8C4A: SBC #$D9
8C4C: .byte $7C
8C4D: CMP $D97C,Y
8C50: .byte $E7
8C51: DEC $7C,X
8C53: .byte $7C
8C54: .byte $7C
8C55: .byte $6F
8C56: SBC #$D9
8C58: .byte $7C
8C59: CMP $D97C,Y
8C5C: CMP #$C9
8C5E: .byte $7C
8C5F: .byte $7C
8C60: .byte $7C
8C61: .byte $6F
8C62: SBC #$D9
8C64: .byte $7C
8C65: CMP $D97C,Y
8C68: .byte $E7
8C69: DEC $7C,X
8C6B: .byte $7C
8C6C: .byte $7C
8C6D: .byte $6F
8C6E: SBC #$DA
8C70: .byte $7C
8C71: .byte $DA
8C72: .byte $7C
8C73: .byte $DA
8C74: CLD
8C75: CLD
8C76: .byte $7C
8C77: .byte $7C
8C78: .byte $7C
8C79: DEC $CBCB
8C7C: .byte $CB
8C7D: .byte $CB
8C7E: .byte $7C
8C7F: .byte $CB
8C80: .byte $CB
8C81: .byte $CB
8C82: .byte $7C
8C83: .byte $7C
8C84: .byte $7C
8C85: CPY $CCCC
8C88: CPY $7CCC
8C8B: CPY $CCCC
8C8E: CPY $7C7C
8C91: .byte $CF
8C92: DEX
8C93: DEX
8C94: DEX
8C95: DEX
8C96: .byte $7C
8C97: DEX
8C98: DEX
8C99: DEX
8C9A: DEX
8C9B: .byte $7C
8C9C: .byte $7C
8C9D: .byte $6F
8C9E: SBC #$7C
8CA0: CMP $7CC8,Y
8CA3: CMP $C9C9,Y
8CA6: CMP #$7C
8CA8: .byte $7C
8CA9: .byte $6F
8CAA: SBC #$7C
8CAC: CMP $7CC8,Y
8CAF: CMP $C9C9,Y
8CB2: CMP #$7C
8CB4: .byte $7C
8CB5: .byte $6F
8CB6: SBC #$7C
8CB8: CMP $7CC8,Y
8CBB: CPY $C1
8CBD: CMP #$C9
8CBF: .byte $7C
8CC0: .byte $7C
8CC1: .byte $6F
8CC2: SBC #$7C
8CC4: CMP $7CC8,Y
8CC7: CMP $C9C9,Y
8CCA: CMP #$7C
8CCC: .byte $7C
8CCD: .byte $6F
8CCE: SBC #$7C
8CD0: .byte $DA
8CD1: CLD
8CD2: .byte $7C
8CD3: .byte $DA
8CD4: CLD
8CD5: CLD
8CD6: CLD
8CD7: .byte $7C
8CD8: .byte $7C
8CD9: .byte $CB
8CDA: .byte $CB
8CDB: .byte $CB
8CDC: .byte $CB
8CDD: .byte $CB
8CDE: .byte $7C
8CDF: .byte $CB
8CE0: .byte $CB
8CE1: .byte $CB
8CE2: .byte $CB
8CE3: .byte $7C
8CE4: .byte $7C
8CE5: CPY $CCCC
8CE8: .byte $7C
8CE9: CPY $CC7C
8CEC: CPY $7CCC
8CEF: .byte $7C
8CF0: .byte $7C
8CF1: DEX
8CF2: DEX
8CF3: DEX
8CF4: .byte $7C
8CF5: DEX
8CF6: .byte $7C
8CF7: DEX
8CF8: DEX
8CF9: DEX
8CFA: .byte $7C
8CFB: .byte $7C
8CFC: .byte $7C
8CFD: .byte $6F
8CFE: .byte $6F
8CFF: .byte $EF
8D00: .byte $7C
8D01: SBC #$7C
8D03: CMP $C9C9,Y
8D06: .byte $7C
8D07: .byte $7C
8D08: .byte $7C
8D09: .byte $6F
8D0A: .byte $6F
8D0B: .byte $EF
8D0C: .byte $7C
8D0D: SBC #$7C
8D0F: .byte $DC
8D10: .byte $E7
8D11: DEC $7C,X
8D13: .byte $7C
8D14: .byte $7C
8D15: .byte $6F
8D16: .byte $6F
8D17: .byte $EF
8D18: .byte $7C
8D19: NOP
8D1A: .byte $7C
8D1B: DEX
8D1C: DEX
8D1D: DEX
8D1E: .byte $7C
8D1F: .byte $7C
8D20: .byte $7C
8D21: CPX $EBEB
8D24: .byte $7C
8D25: .byte $EB
8D26: .byte $EB
8D27: .byte $EB
8D28: .byte $EB
8D29: .byte $EB
8D2A: .byte $EB
8D2B: .byte $7C
8D2C: .byte $7C
8D2D: .byte $6F
8D2E: CPX $EBEB
8D31: .byte $EB
8D32: .byte $EB
8D33: .byte $EB
8D34: .byte $EB
8D35: .byte $EB
8D36: .byte $EB
8D37: .byte $7C
8D38: .byte $7C
8D39: .byte $EF
8D3A: .byte $EF
8D3B: .byte $EF
8D3C: .byte $EF
8D3D: .byte $EF
8D3E: .byte $EF
8D3F: .byte $EF
8D40: .byte $EF
8D41: .byte $EF
8D42: .byte $EF
8D43: .byte $7C
8D44: .byte $7C
8D45: .byte $7C
8D46: .byte $7C
8D47: .byte $7C
8D48: .byte $7C
8D49: .byte $7C
8D4A: .byte $7C
8D4B: .byte $7C
8D4C: .byte $7C
8D4D: .byte $7C
8D4E: .byte $7C
8D4F: .byte $7C
8D50: .byte $7C
8D51: .byte $54
8D52: .byte $54
8D53: .byte $54
8D54: .byte $7A
8D55: ADC $7A7A,Y
8D58: .byte $7A
8D59: .byte $7A
8D5A: .byte $7A
8D5B: .byte $7C
8D5C: .byte $7C
8D5D: ADC $7A7A,Y
8D60: .byte $54
8D61: .byte $54
8D62: .byte $7A
8D63: ADC $797A,Y
8D66: .byte $7A
8D67: .byte $7C
8D68: .byte $7C
8D69: .byte $7A
8D6A: .byte $54
8D6B: .byte $54
8D6C: .byte $54
8D6D: .byte $54
8D6E: .byte $7A
8D6F: .byte $7A
8D70: .byte $7A
8D71: .byte $7A
8D72: .byte $7A
8D73: .byte $7C
8D74: .byte $7C
8D75: .byte $7A
8D76: .byte $54
8D77: .byte $54
8D78: .byte $7A
8D79: .byte $54
8D7A: .byte $54
8D7B: .byte $54
8D7C: .byte $54
8D7D: .byte $54
8D7E: ADC $7C7C,Y
8D81: ADC $547A,Y
8D84: .byte $54
8D85: .byte $54
8D86: .byte $7A
8D87: .byte $7A
8D88: .byte $7A
8D89: .byte $54
8D8A: .byte $7A
8D8B: .byte $7C
8D8C: .byte $7C
8D8D: .byte $7A
8D8E: .byte $54
8D8F: .byte $54
8D90: .byte $7A
8D91: .byte $7A
8D92: .byte $7A
8D93: ADC $547A,Y
8D96: .byte $54
8D97: .byte $7C
8D98: CPY #$C0
8D9A: .byte $54
8D9B: .byte $7A
8D9C: .byte $7A
8D9D: CPX $7A
8D9F: .byte $7A
8DA0: .byte $7A
8DA1: .byte $7A
8DA2: .byte $54
8DA3: .byte $7C
8DA4: .byte $7C
8DA5: .byte $54
8DA6: .byte $7A
8DA7: CPX #$6F
8DA9: .byte $6F
8DAA: .byte $6F
8DAB: .byte $7A
8DAC: CPX $7A
8DAE: CPX $7C
8DB0: .byte $7C
8DB1: .byte $54
8DB2: .byte $54
8DB3: .byte $7A
8DB4: .byte $E2
8DB5: .byte $7A
8DB6: CPX #$6F
8DB8: .byte $6F
8DB9: .byte $6F
8DBA: .byte $E3
8DBB: .byte $7C
8DBC: .byte $7C
8DBD: .byte $7A
8DBE: .byte $54
8DBF: .byte $7A
8DC0: .byte $E2
8DC1: .byte $7A
8DC2: .byte $7A
8DC3: CPX #$7A
8DC5: CPX #$7A
8DC7: .byte $7C
8DC8: .byte $7C
8DC9: ADC $7A54,Y
8DCC: .byte $E2
8DCD: .byte $6F
8DCE: .byte $7A
8DCF: .byte $7A
8DD0: ADC $797A,Y
8DD3: .byte $7C
8DD4: .byte $7C
8DD5: .byte $6F
8DD6: .byte $7A
8DD7: .byte $7A
8DD8: .byte $E2
8DD9: .byte $6F
8DDA: .byte $E3
8DDB: .byte $6F
8DDC: .byte $7A
8DDD: .byte $7A
8DDE: CPX $7C
8DE0: .byte $7C
8DE1: CPX #$6F
8DE3: .byte $7A
8DE4: .byte $82
8DE5: ADC $E07A,Y
8DE8: SBC ($6F,X)
8DEA: .byte $E3
8DEB: .byte $7C
8DEC: .byte $7C
8DED: .byte $7A
8DEE: .byte $82
8DEF: .byte $7A
8DF0: .byte $82
8DF1: .byte $7A
8DF2: .byte $7A
8DF3: .byte $7A
8DF4: .byte $7A
8DF5: CPX #$7A
8DF7: .byte $7C
8DF8: .byte $7C
8DF9: .byte $7A
8DFA: .byte $82
8DFB: .byte $7A
8DFC: .byte $82
8DFD: .byte $6F
8DFE: .byte $7A
8DFF: .byte $7A
8E00: ADC $7A7A,Y
8E03: .byte $7C
8E04: .byte $7C
8E05: ADC $7A82,Y
8E08: .byte $7A
8E09: CPX #$E1
8E0B: .byte $6F
8E0C: .byte $7A
8E0D: .byte $7A
8E0E: .byte $7A
8E0F: .byte $7C
8E10: .byte $7C
8E11: .byte $7A
8E12: .byte $E2
8E13: .byte $6F
8E14: .byte $7A
8E15: .byte $7A
8E16: .byte $7A
8E17: .byte $E2
8E18: .byte $6F
8E19: .byte $E3
8E1A: .byte $7A
8E1B: .byte $7C
8E1C: .byte $7C
8E1D: .byte $7A
8E1E: CPX $E3
8E20: .byte $7A
8E21: .byte $E2
8E22: .byte $7A
8E23: CPX $E3
8E25: .byte $7A
8E26: .byte $7A
8E27: .byte $7C
8E28: .byte $7C
8E29: .byte $E2
8E2A: .byte $E3
8E2B: .byte $7A
8E2C: ADC $6FE2,Y
8E2F: .byte $E3
8E30: .byte $7A
8E31: ADC $7CE2,Y
8E34: .byte $7C
8E35: .byte $E2
8E36: .byte $7A
8E37: .byte $7A
8E38: CPX $6F
8E3A: .byte $6F
8E3B: .byte $7A
8E3C: .byte $7A
8E3D: .byte $7A
8E3E: .byte $E2
8E3F: .byte $7C
8E40: .byte $7C
8E41: .byte $82
8E42: .byte $7A
8E43: .byte $E2
8E44: .byte $6F
8E45: .byte $7A
8E46: .byte $E2
8E47: .byte $E3
8E48: .byte $6F
8E49: .byte $7A
8E4A: .byte $E2
8E4B: .byte $7C
8E4C: .byte $7C
8E4D: .byte $82
8E4E: .byte $7A
8E4F: CPX #$6F
8E51: .byte $7A
8E52: CPX #$7A
8E54: CPX #$7A
8E56: .byte $E2
8E57: CPY #$7C
8E59: .byte $82
8E5A: ADC $E47A,Y
8E5D: .byte $7A
8E5E: .byte $7A
8E5F: .byte $7A
8E60: .byte $7A
8E61: .byte $7A
8E62: .byte $82
8E63: .byte $7C
8E64: .byte $7C
8E65: .byte $82
8E66: .byte $7A
8E67: .byte $7A
8E68: .byte $6F
8E69: .byte $7A
8E6A: ADC $796F,Y
8E6D: .byte $7A
8E6E: .byte $82
8E6F: .byte $7C
8E70: .byte $7C
8E71: .byte $E2
8E72: .byte $6F
8E73: .byte $7A
8E74: CPX $7A
8E76: .byte $7A
8E77: .byte $6F
8E78: .byte $7A
8E79: .byte $7A
8E7A: .byte $82
8E7B: .byte $7C
8E7C: .byte $7C
8E7D: .byte $E2
8E7E: .byte $E3
8E7F: .byte $6F
8E80: .byte $E3
8E81: .byte $7A
8E82: .byte $7A
8E83: .byte $6F
8E84: .byte $E3
8E85: .byte $7A
8E86: .byte $82
8E87: .byte $7C
8E88: .byte $7C
8E89: .byte $7A
8E8A: .byte $6F
8E8B: CPX #$7A
8E8D: ADC $6F54,Y
8E90: .byte $6F
8E91: .byte $7A
8E92: .byte $82
8E93: .byte $7C
8E94: .byte $7C
8E95: ADC $6F6F,Y
8E98: .byte $54
8E99: .byte $54
8E9A: .byte $54
8E9B: .byte $6F
8E9C: .byte $6F
8E9D: .byte $6F
8E9E: .byte $82
8E9F: .byte $7C
8EA0: .byte $7C
8EA1: .byte $7A
8EA2: .byte $6F
8EA3: CPX #$54
8EA5: .byte $7A
8EA6: .byte $7A
8EA7: .byte $7A
8EA8: .byte $7A
8EA9: .byte $7A
8EAA: .byte $82
8EAB: .byte $7C
8EAC: .byte $7C
8EAD: .byte $7A
8EAE: .byte $6F
8EAF: .byte $7A
8EB0: .byte $7A
8EB1: .byte $7A
8EB2: CPX #$6F
8EB4: .byte $6F
8EB5: .byte $6F
8EB6: .byte $82
8EB7: .byte $7C
8EB8: .byte $7C
8EB9: .byte $7A
8EBA: CPX $6F
8EBC: .byte $7A
8EBD: ADC $E47A,Y
8EC0: .byte $E3
8EC1: .byte $7A
8EC2: .byte $7A
8EC3: .byte $7C
8EC4: .byte $7C
8EC5: .byte $E2
8EC6: .byte $E3
8EC7: SBC ($6F,X)
8EC9: .byte $7A
8ECA: CPX $E3
8ECC: .byte $7A
8ECD: .byte $7A
8ECE: CPX $7C
8ED0: .byte $7C
8ED1: CPX #$7A
8ED3: .byte $7A
8ED4: CPX $6F
8ED6: .byte $E3
8ED7: .byte $7A
8ED8: .byte $7A
8ED9: CPX $E3
8EDB: CPY #$7C
8EDD: .byte $7A
8EDE: ADC $6FE4,Y
8EE1: .byte $E3
8EE2: .byte $7A
8EE3: .byte $7A
8EE4: CPX $E3
8EE6: .byte $7A
8EE7: .byte $7C
8EE8: .byte $7C
8EE9: CPX #$E1
8EEB: SBC ($E1,X)
8EED: .byte $7A
8EEE: .byte $7A
8EEF: CPX #$E1
8EF1: .byte $7A
8EF2: .byte $7A
8EF3: .byte $7C
8EF4: .byte $7C
8EF5: .byte $7C
8EF6: .byte $7C
8EF7: .byte $7C
8EF8: .byte $7C
8EF9: .byte $7C
8EFA: .byte $7C
8EFB: .byte $7C
8EFC: .byte $7C
8EFD: .byte $7C
8EFE: .byte $7C
8EFF: .byte $7C
8F00: .byte $07
8F01: AND #$14
8F03: .byte $C7
8F04: CPX $00
8F06: .byte $02
8F07: ORA ($27,X)
8F09: BPL $8F0C
8F0B: .byte $02
8F0C: BRK
8F0D: BRK
8F0E: BRK
8F0F: BRK
8F10: .byte $07
8F11: .byte $3C
8F12: .byte $0C
8F13: .byte $23
8F14: .byte $02
8F15: .byte $04
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
8F20: EOR ($01),Y
8F22: .byte $0B
8F23: BPL $8F4D
8F25: ORA ($5D,X)
8F27: .byte $02
8F28: BRK
8F29: .byte $02
8F2A: BRK
8F2B: BRK
8F2C: BRK
8F2D: BRK
8F2E: BRK
8F2F: BRK
8F30: EOR ($03),Y
8F32: .byte $03
8F33: BVC $8F5D
8F35: ORA ($5D,X)
8F37: .byte $02
8F38: BRK
8F39: .byte $02
8F3A: BRK
8F3B: BRK
8F3C: BRK
8F3D: BRK
8F3E: BRK
8F3F: BRK
8F40: EOR ($03),Y
8F42: .byte $17
8F43: BMI $8F6D
8F45: ORA ($5D,X)
8F47: .byte $02
8F48: BRK
8F49: .byte $02
8F4A: BRK
8F4B: BRK
8F4C: BRK
8F4D: BRK
8F4E: BRK
8F4F: BRK
8F50: ADC ($03,X)
8F52: ASL $80,X
8F54: .byte $23
8F55: ORA ($6D,X)
8F57: .byte $02
8F58: .byte $03
8F59: .byte $02
8F5A: BRK
8F5B: BRK
8F5C: BRK
8F5D: BRK
8F5E: BRK
8F5F: BRK
8F60: ADC ($02,X)
8F62: ORA $80
8F64: .byte $23
8F65: ORA ($6D,X)
8F67: .byte $02
8F68: .byte $03
8F69: .byte $02
8F6A: BRK
8F6B: BRK
8F6C: BRK
8F6D: BRK
8F6E: BRK
8F6F: BRK
8F70: ADC ($03,X)
8F72: .byte $0F
8F73: BCC $8F98
8F75: ORA ($6D,X)
8F77: .byte $02
8F78: .byte $03
8F79: .byte $02
8F7A: BRK
8F7B: BRK
8F7C: BRK
8F7D: BRK
8F7E: BRK
8F7F: BRK
8F80: EOR ($01,X)
8F82: .byte $2F
8F83: BPL $8FA3
8F85: ORA ($4D,X)
8F87: .byte $02
8F88: .byte $02
8F89: .byte $02
8F8A: BRK
8F8B: BRK
8F8C: BRK
8F8D: BRK
8F8E: BRK
8F8F: BRK
8F90: EOR ($02,X)
8F92: .byte $3C
8F93: RTS
8F94: ASL $4D01,X
8F97: .byte $02
8F98: BRK
8F99: .byte $02
8F9A: BRK
8F9B: BRK
8F9C: BRK
8F9D: BRK
8F9E: BRK
8F9F: BRK
8FA0: EOR ($01,X)
8FA2: SEC
8FA3: BMI $8FC3
8FA5: ORA ($4D,X)
8FA7: .byte $02
8FA8: BRK
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
8FE5: ASL A
8FE6: .byte $17
8FE7: .byte $27
8FE8: .byte $0F
8FE9: ORA ($12,X)
8FEB: AND ($0F),Y
8FED: BRK
8FEE: BPL $9020
8FF0: .byte $0F
8FF1: ORA $30
8FF3: ROL $0F,X
8FF5: ORA $26
8FF7: BMI $9008
8FF9: .byte $04
8FFA: AND $30
8FFC: .byte $0F
8FFD: .byte $02
8FFE: .byte $3C
8FFF: ROL $FC,X
9001: .byte $FC
9002: .byte $FC
9003: .byte $FC
9004: .byte $FC
9005: .byte $FC
9006: .byte $FC
9007: .byte $FC
9008: .byte $FC
9009: .byte $FC
900A: .byte $FC
900B: .byte $FC
900C: .byte $FC
900D: .byte $FC
900E: .byte $FC
900F: .byte $FC
9010: .byte $FC
9011: .byte $FC
9012: CMP $9D9D
9015: BEQ $9013
9017: .byte $FC
9018: .byte $FC
9019: .byte $FC
901A: .byte $FC
901B: .byte $FC
901C: .byte $FC
901D: CMP $9D9D
9020: STA $F09D,X
9023: .byte $FC
9024: .byte $FC
9025: .byte $FC
9026: .byte $FC
9027: CMP $9D9E
902A: STA $9D9D,X
902D: STA $FC9D,X
9030: .byte $AF
9031: .byte $AF
9032: .byte $AF
9033: .byte $9E
9034: STA $9D9D,X
9037: STA $9D9D,X
903A: STA $FCFC,X
903D: .byte $FC
903E: .byte $AF
903F: .byte $9E
9040: STA $9D9D,X
9043: STA $9D9D,X
9046: STA $FCFC,X
9049: .byte $FC
904A: .byte $FC
904B: .byte $FC
904C: .byte $FC
904D: .byte $FC
904E: CMP $9D9E
9051: STA $FC9D,X
9054: .byte $FC
9055: .byte $FC
9056: .byte $FC
9057: .byte $FC
9058: .byte $FC
9059: .byte $FC
905A: .byte $FC
905B: CMP $9D9E
905E: STA $FCFC,X
9061: .byte $FC
9062: .byte $FC
9063: .byte $FC
9064: .byte $FC
9065: .byte $FC
9066: .byte $FC
9067: .byte $FC
9068: CMP $9D9E
906B: .byte $FC
906C: .byte $FC
906D: .byte $FC
906E: .byte $FC
906F: .byte $FC
9070: .byte $FC
9071: .byte $FC
9072: .byte $FC
9073: .byte $FC
9074: .byte $FC
9075: CMP $FC9E
9078: .byte $FC
9079: .byte $FC
907A: .byte $FC
907B: .byte $FC
907C: .byte $FC
907D: .byte $FC
907E: .byte $FC
907F: .byte $FC
9080: .byte $FC
9081: CMP $FC9E
9084: .byte $FC
9085: .byte $FC
9086: .byte $FC
9087: .byte $FC
9088: .byte $FC
9089: .byte $FC
908A: .byte $FC
908B: .byte $FC
908C: .byte $FC
908D: CMP $FC9E
9090: .byte $FC
9091: .byte $FC
9092: .byte $FC
9093: .byte $FC
9094: .byte $FC
9095: .byte $FC
9096: .byte $FC
9097: .byte $FC
9098: .byte $FC
9099: CMP $FC9E
909C: .byte $FC
909D: .byte $FC
909E: .byte $FC
909F: .byte $FC
90A0: .byte $FC
90A1: .byte $FC
90A2: .byte $FC
90A3: .byte $FC
90A4: CMP $9D9E
90A7: .byte $FC
90A8: .byte $FC
90A9: .byte $FC
90AA: .byte $FC
90AB: .byte $FC
90AC: .byte $FC
90AD: .byte $FC
90AE: CMP $9D9E
90B1: STA $FC9D,X
90B4: .byte $FC
90B5: .byte $FC
90B6: CMP $9D9E
90B9: STA $9D9D,X
90BC: STA $FC9D,X
90BF: .byte $FC
90C0: .byte $FC
90C1: CMP $9D9E
90C4: STA $9D9D,X
90C7: STA $FC9D,X
90CA: .byte $FC
90CB: .byte $FC
90CC: .byte $FC
90CD: CMP $409E
90D0: RTI
90D1: RTI
90D2: RTI
90D3: RTI
90D4: .byte $FC
90D5: .byte $FC
90D6: .byte $FC
90D7: .byte $FC
90D8: .byte $FC
90D9: CMP $FC9E
90DC: CMP $9D9E
90DF: .byte $FC
90E0: .byte $FC
90E1: .byte $FC
90E2: .byte $FC
90E3: .byte $FC
90E4: .byte $FC
90E5: CMP $FC9E
90E8: .byte $FC
90E9: CMP $9D9E
90EC: STA $FCFC,X
90EF: .byte $FC
90F0: .byte $FC
90F1: CMP $FC9E
90F4: .byte $FC
90F5: .byte $FC
90F6: CMP $9D9E
90F9: STA $FCFC,X
90FC: .byte $FC
90FD: CMP $FC9E
9100: .byte $FC
9101: .byte $FC
9102: .byte $FC
9103: .byte $FC
9104: CMP $9D9E
9107: .byte $FC
9108: .byte $FC
9109: CMP $FC9E
910C: .byte $FC
910D: .byte $FC
910E: .byte $FC
910F: .byte $FC
9110: .byte $FC
9111: CMP $FC9E
9114: .byte $FC
9115: CMP $FC9E
9118: .byte $FC
9119: .byte $FC
911A: CMP $FCF0
911D: .byte $FC
911E: .byte $9E
911F: .byte $FC
9120: .byte $FC
9121: CMP $D29E
9124: .byte $D2
9125: CMP $9D9E
9128: BEQ $9126
912A: .byte $9E
912B: .byte $FC
912C: .byte $FC
912D: CMP $D29E
9130: .byte $D2
9131: CMP $9D9E
9134: BEQ $9132
9136: .byte $9E
9137: .byte $FC
9138: .byte $FC
9139: CMP $D29E
913C: .byte $D2
913D: CMP $F09E
9140: .byte $FC
9141: .byte $9E
9142: STA $FCFC,X
9145: CMP $FC9E
9148: .byte $FC
9149: CMP $9D9E
914C: STA $FC9D,X
914F: .byte $FC
9150: .byte $FC
9151: CMP $FC9E
9154: .byte $FC
9155: .byte $FC
9156: CMP $9D9D
9159: STA $FCFC,X
915C: .byte $FC
915D: CMP $FC42
9160: .byte $FC
9161: .byte $FC
9162: .byte $FC
9163: CMP $9D9D
9166: STA $FCFC,X
9169: CMP $FC9E
916C: .byte $FC
916D: .byte $FC
916E: .byte $FC
916F: STA $9D9D,X
9172: STA $FCFC,X
9175: CMP $FC9E
9178: .byte $FC
9179: CMP $9D9E
917C: STA $FC9D,X
917F: .byte $FC
9180: .byte $FC
9181: CMP $FC9E
9184: .byte $FC
9185: CMP $9D9E
9188: STA $F09D,X
918B: .byte $FC
918C: .byte $FC
918D: CMP $FC9E
9190: .byte $FC
9191: .byte $FC
9192: CMP $9D9D
9195: STA $FCF0,X
9198: .byte $FC
9199: CMP $9D9E
919C: .byte $FC
919D: .byte $FC
919E: CMP $9D9D
91A1: STA $FCFC,X
91A4: .byte $FC
91A5: .byte $FC
91A6: CMP $9D9D
91A9: .byte $FC
91AA: STA $9D9D,X
91AD: STA $FCFC,X
91B0: .byte $FC
91B1: .byte $FC
91B2: CMP $9D9D
91B5: STA $9D9D,X
91B8: .byte $FC
91B9: .byte $9E
91BA: .byte $FC
91BB: .byte $FC
91BC: .byte $FC
91BD: .byte $FC
91BE: CMP $9D9D
91C1: STA $FC9D,X
91C4: .byte $FC
91C5: .byte $9E
91C6: .byte $FC
91C7: .byte $FC
91C8: .byte $FC
91C9: .byte $FC
91CA: CMP $9D9D
91CD: STA $FCFC,X
91D0: CMP $FC9E
91D3: .byte $FC
91D4: .byte $FC
91D5: CMP $9D9E
91D8: STA $FC9D,X
91DB: .byte $FC
91DC: CMP $FC9E
91DF: .byte $FC
91E0: .byte $FC
91E1: CMP $9EEF
91E4: STA $9D9D,X
91E7: .byte $FC
91E8: CMP $FC9E
91EB: .byte $FC
91EC: .byte $FC
91ED: .byte $FC
91EE: .byte $FC
91EF: CMP $9D9E
91F2: STA $9EFC,X
91F5: STA $FCFC,X
91F8: .byte $FC
91F9: .byte $FC
91FA: .byte $FC
91FB: .byte $FC
91FC: .byte $FC
91FD: CMP $9D9D
9200: STA $9D9D,X
9203: .byte $FC
9204: .byte $FC
9205: .byte $FC
9206: .byte $FC
9207: .byte $FC
9208: .byte $FC
9209: CMP $9D9D
920C: STA $9D9D,X
920F: .byte $FC
9210: .byte $FC
9211: .byte $FC
9212: .byte $FC
9213: .byte $FC
9214: CMP $9D9D
9217: STA $9D9D,X
921A: STA $FCFC,X
921D: .byte $FC
921E: CMP $9D9E
9221: STA $409D,X
9224: RTI
9225: RTI
9226: RTI
9227: .byte $FC
9228: .byte $FC
9229: CMP $9D9E
922C: STA $9D9D,X
922F: .byte $FC
9230: CMP $FC9E
9233: .byte $FC
9234: .byte $FC
9235: .byte $FC
9236: CMP $9D9D
9239: STA $FC9D,X
923C: CMP $FC9E
923F: .byte $FC
9240: .byte $FC
9241: CMP $9D9E
9244: STA $9D9D,X
9247: .byte $FC
9248: CMP $FC9E
924B: .byte $FC
924C: .byte $FC
924D: .byte $FC
924E: CMP $9D9E
9251: STA $FCFC,X
9254: CMP $FC9E
9257: .byte $FC
9258: .byte $FC
9259: .byte $FC
925A: .byte $FC
925B: CMP $9D9D
925E: .byte $FC
925F: .byte $FC
9260: CMP $FC9E
9263: .byte $FC
9264: .byte $FC
9265: .byte $FC
9266: CMP $9D9E
9269: STA $FCFC,X
926C: CMP $FC9E
926F: .byte $FC
9270: .byte $FC
9271: .byte $FC
9272: .byte $FC
9273: CMP $9D9D
9276: .byte $FC
9277: .byte $FC
9278: CMP $9D9E
927B: .byte $FC
927C: .byte $FC
927D: .byte $FC
927E: .byte $FC
927F: CMP $9D9D
9282: .byte $FC
9283: .byte $FC
9284: .byte $FC
9285: CMP $FC9E
9288: .byte $FC
9289: .byte $FC
928A: .byte $FC
928B: CMP $FC9D
928E: .byte $FC
928F: .byte $FC
9290: .byte $FC
9291: CMP $FC9E
9294: .byte $FC
9295: .byte $FC
9296: CMP $9D9E
9299: .byte $FC
929A: .byte $FC
929B: .byte $FC
929C: .byte $FC
929D: CMP $FC9E
92A0: .byte $FC
92A1: .byte $FC
92A2: CMP $9D9E
92A5: STA $FCFC,X
92A8: .byte $FC
92A9: CMP $FC9E
92AC: .byte $FC
92AD: .byte $FC
92AE: CMP $9D9E
92B1: STA $FCFC,X
92B4: .byte $FC
92B5: .byte $FC
92B6: .byte $9E
92B7: .byte $FC
92B8: .byte $FC
92B9: .byte $FC
92BA: CMP $9D9E
92BD: STA $FC9D,X
92C0: .byte $FC
92C1: .byte $FC
92C2: .byte $FC
92C3: .byte $FC
92C4: .byte $FC
92C5: .byte $FC
92C6: CMP $9D9E
92C9: STA $9D9D,X
92CC: .byte $FC
92CD: .byte $FC
92CE: .byte $FC
92CF: .byte $FC
92D0: .byte $FC
92D1: .byte $FC
92D2: .byte $FC
92D3: CMP $9D9E
92D6: STA $FC9D,X
92D9: .byte $FC
92DA: .byte $FC
92DB: .byte $FC
92DC: .byte $FC
92DD: .byte $FC
92DE: .byte $FC
92DF: .byte $FC
92E0: CMP $9D9E
92E3: STA $FC9D,X
92E6: .byte $FC
92E7: .byte $FC
92E8: .byte $FC
92E9: .byte $FC
92EA: .byte $FC
92EB: .byte $FC
92EC: .byte $FC
92ED: .byte $FC
92EE: CMP $9D9E
92F1: STA $FCFC,X
92F4: .byte $FC
92F5: .byte $FC
92F6: .byte $FC
92F7: .byte $FC
92F8: .byte $FC
92F9: .byte $FC
92FA: .byte $FC
92FB: .byte $FC
92FC: .byte $9E
92FD: STA $FCFC,X
9300: .byte $07
9301: BIT $C712
9304: .byte $9E
9305: BRK
9306: .byte $02
9307: ORA ($20,X)
9309: JSR $0200
930C: BRK
930D: BRK
930E: BRK
930F: BRK
9310: BRK
9311: INY
9312: BRK
9313: INY
9314: .byte $02
9315: .byte $04
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
9320: ADC ($01),Y
9322: .byte $22
9323: BMI $9352
9325: ORA $49
9327: .byte $03
9328: ASL $00
932A: BRK
932B: BRK
932C: BRK
932D: BRK
932E: BRK
932F: BRK
9330: ADC ($01),Y
9332: .byte $1C
9333: JSR $052D
9336: EOR #$03
9338: ASL $00
933A: BRK
933B: BRK
933C: BRK
933D: BRK
933E: BRK
933F: BRK
9340: ADC ($01),Y
9342: .byte $3B
9343: BVS $9372
9345: ORA $49
9347: .byte $03
9348: ASL $00
934A: BRK
934B: BRK
934C: BRK
934D: BRK
934E: BRK
934F: BRK
9350: ADC ($03,X)
9352: AND ($90,X)
9354: .byte $32
9355: .byte $02
9356: ADC $0302
9359: ORA ($00,X)
935B: BRK
935C: BRK
935D: BRK
935E: BRK
935F: BRK
9360: ADC ($03,X)
9362: .byte $27
9363: JSR $0232
9366: ADC $0302
9369: ORA ($00,X)
936B: BRK
936C: BRK
936D: BRK
936E: BRK
936F: BRK
9370: ADC ($02,X)
9372: ORA $28A0
9375: ORA ($6D,X)
9377: .byte $02
9378: BRK
9379: .byte $02
937A: BRK
937B: BRK
937C: BRK
937D: BRK
937E: BRK
937F: BRK
9380: ADC ($02,X)
9382: .byte $34
9383: BVC $93AD
9385: ORA ($6D,X)
9387: .byte $02
9388: .byte $02
9389: .byte $02
938A: BRK
938B: BRK
938C: BRK
938D: BRK
938E: BRK
938F: BRK
9390: ADC ($01,X)
9392: .byte $03
9393: BVS $93BD
9395: ORA ($6D,X)
9397: .byte $02
9398: BRK
9399: .byte $02
939A: BRK
939B: BRK
939C: BRK
939D: BRK
939E: BRK
939F: BRK
93A0: ADC ($01,X)
93A2: ASL $20,X
93A4: PLP
93A5: ORA ($6D,X)
93A7: .byte $02
93A8: .byte $02
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
93E5: .byte $0B
93E6: .byte $32
93E7: BMI $93F8
93E9: ORA #$19
93EB: ROL A
93EC: .byte $0F
93ED: .byte $02
93EE: .byte $1C
93EF: BIT $050F
93F2: BMI $942A
93F4: .byte $0F
93F5: ORA $26
93F7: BMI $9408
93F9: .byte $02
93FA: AND $30
93FC: .byte $0F
93FD: ORA $27
93FF: SEC
9400: .byte $FC
9401: .byte $FC
9402: .byte $FC
9403: .byte $FC
9404: .byte $FC
9405: .byte $FC
9406: .byte $FC
9407: .byte $FC
9408: STA $FC9D,X
940B: .byte $FC
940C: RTI
940D: RTI
940E: RTI
940F: RTI
9410: RTI
9411: RTI
9412: .byte $FC
9413: .byte $FC
9414: CMP $9D9D
9417: .byte $FC
9418: .byte $FC
9419: .byte $FC
941A: .byte $9E
941B: STA $9D9D,X
941E: STA $FCFC,X
9421: CMP $FC9D
9424: .byte $FC
9425: CMP $4E9E
9428: .byte $4B
9429: .byte $4B
942A: .byte $4B
942B: .byte $FC
942C: .byte $FC
942D: CMP $FC9D
9430: .byte $FC
9431: CMP $509E
9434: JMP $4C4C
9437: .byte $FC
9438: .byte $FC
9439: CMP $FC9D
943C: .byte $FC
943D: CMP $509E
9440: EOR #$49
9442: EOR #$FC
9444: .byte $FC
9445: CMP $FC9D
9448: .byte $FC
9449: CMP $509E
944C: EOR #$45
944E: STA ($FC,X)
9450: .byte $FC
9451: CMP $FC9D
9454: .byte $FC
9455: .byte $FC
9456: .byte $9E
9457: BVC $94A5
9459: JMP $FC4C
945C: .byte $FC
945D: CMP $FC9D
9460: .byte $FC
9461: .byte $FC
9462: .byte $9E
9463: .byte $4F
9464: LSR A
9465: LSR A
9466: LSR A
9467: .byte $FC
9468: CMP $9D9D
946B: .byte $FC
946C: .byte $FC
946D: .byte $FC
946E: .byte $FC
946F: .byte $FC
9470: CMP $FC9E
9473: .byte $FC
9474: CMP $9D9D
9477: .byte $FC
9478: .byte $FC
9479: .byte $FC
947A: .byte $FC
947B: .byte $FC
947C: .byte $FC
947D: .byte $FC
947E: .byte $FC
947F: CMP $9D9D
9482: STA $FCFC,X
9485: .byte $FC
9486: .byte $FC
9487: .byte $FC
9488: .byte $FC
9489: .byte $FC
948A: .byte $FC
948B: .byte $FC
948C: CMP $9D9D
948F: .byte $FC
9490: .byte $FC
9491: .byte $FC
9492: .byte $FC
9493: CMP $FC9E
9496: .byte $FC
9497: .byte $FC
9498: .byte $FC
9499: CMP $FC9D
949C: .byte $FC
949D: .byte $FC
949E: .byte $FC
949F: CMP $9D9E
94A2: .byte $FC
94A3: .byte $FC
94A4: CMP $9DCD
94A7: .byte $FC
94A8: .byte $FC
94A9: .byte $FC
94AA: CMP $9D9E
94AD: STA $FC9D,X
94B0: CMP $9D9D
94B3: .byte $FC
94B4: .byte $FC
94B5: CMP $9D9E
94B8: STA $9D9D,X
94BB: .byte $FC
94BC: CMP $9D9D
94BF: .byte $FC
94C0: .byte $FC
94C1: CMP $9D9E
94C4: STA $9D9D,X
94C7: .byte $FC
94C8: .byte $FC
94C9: CMP $FC9D
94CC: .byte $FC
94CD: CMP $F69E
94D0: STA $9DF6,X
94D3: STA $CDFC,X
94D6: STA $FCFC,X
94D9: CMP $F79E
94DC: INC $F7,X
94DE: INC $9D,X
94E0: RTI
94E1: RTI
94E2: RTI
94E3: .byte $FC
94E4: .byte $FC
94E5: CMP $F69E
94E8: .byte $F7
94E9: INC $F7,X
94EB: INC $9D,X
94ED: INC $9D,X
94EF: .byte $FC
94F0: .byte $FC
94F1: CMP $F79E
94F4: INC $F7,X
94F6: INC $F7,X
94F8: INC $F7,X
94FA: INC $FC,X
94FC: .byte $FC
94FD: CMP $F69E
9500: .byte $F7
9501: INC $F7,X
9503: INC $F7,X
9505: INC $F7,X
9507: .byte $FC
9508: .byte $FC
9509: CMP $F79E
950C: INC $F7,X
950E: STA $F6F7,X
9511: .byte $F7
9512: INC $FC,X
9514: .byte $FC
9515: CMP $9D9E
9518: .byte $F7
9519: STA $9D9D,X
951C: .byte $F7
951D: INC $F7,X
951F: .byte $FC
9520: .byte $FC
9521: .byte $FC
9522: CMP $9D9E
9525: STA $9D9D,X
9528: STA $9DF7,X
952B: .byte $FC
952C: .byte $FC
952D: .byte $FC
952E: .byte $FC
952F: CMP $9D9E
9532: STA $9D9D,X
9535: STA $FC9D,X
9538: .byte $FC
9539: .byte $FC
953A: .byte $FC
953B: .byte $FC
953C: CMP $9D9D
953F: STA $9D9D,X
9542: STA $FCFC,X
9545: .byte $FC
9546: .byte $FC
9547: CMP $9D9E
954A: STA $9D9D,X
954D: STA $FC9D,X
9550: .byte $FC
9551: .byte $FC
9552: CMP $9D9E
9555: STA $9D9D,X
9558: STA $9D9D,X
955B: .byte $FC
955C: .byte $FC
955D: .byte $FC
955E: CMP $9D9E
9561: .byte $FC
9562: CMP $9D9D
9565: STA $FCFC,X
9568: .byte $FC
9569: .byte $FC
956A: CMP $9D9E
956D: .byte $FC
956E: .byte $FC
956F: CMP $FC9D
9572: .byte $FC
9573: .byte $FC
9574: .byte $FC
9575: .byte $FC
9576: CMP $FC9E
9579: .byte $FC
957A: .byte $FC
957B: .byte $FC
957C: STA $FCFC,X
957F: .byte $FC
9580: .byte $FC
9581: CMP $9D9E
9584: .byte $FC
9585: .byte $FC
9586: .byte $FC
9587: .byte $FC
9588: STA $FC9D,X
958B: .byte $FC
958C: .byte $FC
958D: CMP $9D9E
9590: .byte $FC
9591: .byte $FC
9592: .byte $FC
9593: .byte $FC
9594: STA $FC9D,X
9597: .byte $FC
9598: .byte $FC
9599: CMP $FC9E
959C: .byte $FC
959D: .byte $FC
959E: .byte $FC
959F: .byte $FC
95A0: .byte $FC
95A1: STA $FCFC,X
95A4: .byte $FC
95A5: CMP $FC9E
95A8: .byte $FC
95A9: .byte $FC
95AA: .byte $FC
95AB: .byte $FC
95AC: .byte $FC
95AD: ROR $FC9D,X
95B0: .byte $FC
95B1: CMP $9D9E
95B4: STA $FCFC,X
95B7: .byte $FC
95B8: .byte $FC
95B9: STA $FCFC,X
95BC: .byte $FC
95BD: CMP $9D9E
95C0: STA $FC9D,X
95C3: .byte $FC
95C4: .byte $FC
95C5: STA $FCFC,X
95C8: .byte $FC
95C9: .byte $FC
95CA: CMP $9D9E
95CD: STA $FCFC,X
95D0: .byte $FC
95D1: STA $FCFC,X
95D4: .byte $FC
95D5: .byte $FC
95D6: .byte $FC
95D7: CMP $9D9E
95DA: STA $FCFC,X
95DD: STA $FCFC,X
95E0: .byte $FC
95E1: .byte $FC
95E2: .byte $FC
95E3: CMP $9D9E
95E6: STA $FCFC,X
95E9: STA $FCFC,X
95EC: .byte $FC
95ED: .byte $FC
95EE: .byte $FC
95EF: .byte $FC
95F0: CMP $9D9E
95F3: .byte $FC
95F4: .byte $FC
95F5: STA $FCFC,X
95F8: .byte $FC
95F9: .byte $FC
95FA: .byte $9E
95FB: STA $CD9D,X
95FE: STA $FC9D,X
9601: STA $FC9D,X
9604: .byte $FC
9605: .byte $9E
9606: STA $9D9D,X
9609: CMP $9D9D
960C: .byte $FC
960D: .byte $FC
960E: .byte $FC
960F: .byte $FC
9610: .byte $FC
9611: CMP $9D9E
9614: STA $9DCD,X
9617: STA $FCFC,X
961A: .byte $FC
961B: .byte $FC
961C: .byte $FC
961D: CMP $FC9E
9620: STA $9DFC,X
9623: STA $FCF0,X
9626: .byte $FC
9627: .byte $FC
9628: .byte $FC
9629: CMP $FC9E
962C: STA $9DFC,X
962F: STA $FCF0,X
9632: .byte $FC
9633: .byte $FC
9634: .byte $FC
9635: CMP $FC9E
9638: STA $9DFC,X
963B: STA $FCF0,X
963E: .byte $FC
963F: .byte $FC
9640: .byte $FC
9641: CMP $FC9E
9644: STA $9DFC,X
9647: STA $FCF0,X
964A: STA $FCFC,X
964D: CMP $FC9E
9650: STA $9DFC,X
9653: STA $FCF0,X
9656: STA $FCFC,X
9659: CMP $FC9E
965C: STA $9DFC,X
965F: STA $FC9D,X
9662: STA $FCFC,X
9665: CMP $FC9E
9668: STA $9DFC,X
966B: STA $9DFC,X
966E: STA $FCFC,X
9671: CMP $FC9E
9674: STA $9DFC,X
9677: STA $9DFC,X
967A: STA $AFFC,X
967D: .byte $AF
967E: .byte $9E
967F: .byte $FC
9680: STA $9DFC,X
9683: STA $9DFC,X
9686: STA $AFFC,X
9689: .byte $AF
968A: .byte $9E
968B: .byte $FC
968C: STA $9DFC,X
968F: STA $9DFC,X
9692: .byte $FC
9693: .byte $FC
9694: .byte $AF
9695: .byte $AF
9696: .byte $9E
9697: .byte $FC
9698: STA $9DFC,X
969B: STA $9DFC,X
969E: .byte $FC
969F: .byte $FC
96A0: .byte $FC
96A1: CMP $FC9E
96A4: STA $9DFC,X
96A7: STA $9D9D,X
96AA: .byte $FC
96AB: .byte $FC
96AC: .byte $FC
96AD: CMP $FC9E
96B0: STA $CDFC,X
96B3: STA $9D9D,X
96B6: .byte $FC
96B7: .byte $FC
96B8: .byte $FC
96B9: CMP $FC9E
96BC: STA $FCFC,X
96BF: CMP $9D9D
96C2: STA $FCFC,X
96C5: CMP $FC9E
96C8: STA $FC9D,X
96CB: .byte $FC
96CC: .byte $FC
96CD: CMP $FC9D
96D0: .byte $FC
96D1: CMP $9D9E
96D4: STA $9D9D,X
96D7: .byte $FC
96D8: CMP $FC9D
96DB: .byte $FC
96DC: .byte $FC
96DD: .byte $FC
96DE: .byte $9E
96DF: STA $9D9D,X
96E2: STA $409D,X
96E5: RTI
96E6: .byte $FC
96E7: .byte $FC
96E8: .byte $FC
96E9: .byte $FC
96EA: .byte $FC
96EB: .byte $FC
96EC: STA $9D9D,X
96EF: STA $FC9D,X
96F2: .byte $FC
96F3: .byte $FC
96F4: .byte $FC
96F5: .byte $FC
96F6: .byte $FC
96F7: .byte $FC
96F8: .byte $FC
96F9: .byte $FC
96FA: .byte $FC
96FB: .byte $FC
96FC: .byte $FC
96FD: .byte $FC
96FE: .byte $FC
96FF: .byte $FC
9700: .byte $07
9701: PLP
9702: ORA $44
9704: STA $0200,X
9707: ORA ($2A,X)
9709: LDY #$01
970B: .byte $02
970C: BRK
970D: BRK
970E: BRK
970F: BRK
9710: .byte $0B
9711: ASL A
9712: .byte $0C
9713: .byte $14
9714: .byte $02
9715: .byte $04
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
9720: EOR ($01),Y
9722: ORA #$50
9724: .byte $14
9725: ORA $5D
9727: .byte $03
9728: ASL $00
972A: BRK
972B: BRK
972C: BRK
972D: BRK
972E: BRK
972F: BRK
9730: ADC ($03),Y
9732: ROL $20,X
9734: PLP
9735: ORA ($7D,X)
9737: .byte $02
9738: .byte $02
9739: .byte $02
973A: BRK
973B: BRK
973C: BRK
973D: BRK
973E: BRK
973F: BRK
9740: ADC ($02),Y
9742: .byte $1A
9743: LDY #$28
9745: ORA ($7D,X)
9747: .byte $02
9748: .byte $03
9749: .byte $02
974A: BRK
974B: BRK
974C: BRK
974D: BRK
974E: BRK
974F: BRK
9750: EOR ($02,X)
9752: AND ($90,X)
9754: ASL $4D01,X
9757: .byte $02
9758: BRK
9759: .byte $02
975A: BRK
975B: BRK
975C: BRK
975D: BRK
975E: BRK
975F: BRK
9760: EOR ($03,X)
9762: .byte $03
9763: LDY #$1E
9765: ORA ($4D,X)
9767: .byte $02
9768: BRK
9769: .byte $02
976A: BRK
976B: BRK
976C: BRK
976D: BRK
976E: BRK
976F: BRK
9770: EOR ($02,X)
9772: .byte $13
9773: JSR $011E
9776: EOR $0202
9779: .byte $02
977A: BRK
977B: BRK
977C: BRK
977D: BRK
977E: BRK
977F: BRK
9780: ADC ($01,X)
9782: ASL A
9783: .byte $80
9784: ASL $6D01,X
9787: .byte $02
9788: .byte $03
9789: .byte $02
978A: BRK
978B: BRK
978C: BRK
978D: BRK
978E: BRK
978F: BRK
9790: ADC ($01,X)
9792: .byte $1B
9793: BVC $97B3
9795: ORA ($6D,X)
9797: .byte $02
9798: .byte $03
9799: .byte $02
979A: BRK
979B: BRK
979C: BRK
979D: BRK
979E: BRK
979F: BRK
97A0: ADC ($01,X)
97A2: AND ($70),Y
97A4: ASL $6D01,X
97A7: .byte $02
97A8: .byte $03
97A9: .byte $02
97AA: BRK
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
97E5: .byte $0B
97E6: .byte $32
97E7: BMI $97F8
97E9: ORA #$19
97EB: ROL A
97EC: .byte $0F
97ED: .byte $02
97EE: .byte $1C
97EF: BIT $050F
97F2: BMI $982A
97F4: .byte $0F
97F5: ORA $26
97F7: BMI $9808
97F9: .byte $12
97FA: .byte $27
97FB: BMI $980C
97FD: ORA #$29
97FF: ROL $F6,X
9801: INC $F6,X
9803: .byte $F7
9804: .byte $F7
9805: .byte $F7
9806: .byte $F7
9807: .byte $F7
9808: INC $F6,X
980A: INC $F7,X
980C: .byte $F7
980D: .byte $F7
980E: .byte $F7
980F: .byte $7C
9810: .byte $7C
9811: .byte $7C
9812: .byte $7C
9813: .byte $7C
9814: .byte $F7
9815: .byte $F7
9816: .byte $F7
9817: INC $F6,X
9819: .byte $7C
981A: .byte $7C
981B: ADC $65
981D: ADC $65
981F: ADC $65
9821: .byte $F7
9822: INC $F7,X
9824: .byte $F7
9825: .byte $7C
9826: .byte $7C
9827: ADC $7C
9829: .byte $7C
982A: .byte $7C
982B: .byte $7C
982C: ADC $65
982E: .byte $F7
982F: INC $F6,X
9831: ADC $65
9833: ADC $65
9835: ADC $65
9837: ADC $7C
9839: ADC $F6
983B: .byte $F7
983C: .byte $F7
983D: ADC $7C
983F: .byte $7C
9840: .byte $7C
9841: .byte $7C
9842: .byte $7C
9843: ADC $7C
9845: ADC $F7
9847: INC $F6,X
9849: ADC $7C
984B: .byte $7C
984C: .byte $7C
984D: ADC $65
984F: ADC $7C
9851: ADC $65
9853: .byte $F7
9854: .byte $F7
9855: ADC $7C
9857: .byte $7C
9858: .byte $7C
9859: ADC $7C
985B: ADC $65
985D: .byte $7C
985E: ADC $F6
9860: INC $65,X
9862: RTI
9863: RTI
9864: RTI
9865: RTI
9866: .byte $7C
9867: .byte $7C
9868: ADC $7C
986A: ADC $F7
986C: .byte $F7
986D: .byte $7C
986E: .byte $7C
986F: .byte $7C
9870: .byte $7C
9871: ADC $7C
9873: .byte $7C
9874: ADC $7C
9876: ADC $F6
9878: INC $7C,X
987A: ADC $65
987C: ADC $65
987E: .byte $7C
987F: .byte $7C
9880: ADC $7C
9882: ADC $F7
9884: .byte $F7
9885: .byte $7C
9886: ADC $7C
9888: .byte $7C
9889: .byte $7C
988A: .byte $7C
988B: .byte $7C
988C: ADC $7C
988E: ADC $F6
9890: INC $65,X
9892: ADC $65
9894: ADC $65
9896: ADC $65
9898: ADC $7C
989A: ADC $F7
989C: .byte $F7
989D: .byte $7C
989E: ADC $7C
98A0: ADC $65
98A2: ADC $7C
98A4: .byte $7C
98A5: .byte $7C
98A6: ADC $F6
98A8: INC $7C,X
98AA: ADC $7C
98AC: ADC $7C
98AE: .byte $7C
98AF: .byte $7C
98B0: .byte $7C
98B1: .byte $7C
98B2: ADC $F7
98B4: .byte $F7
98B5: ADC $65
98B7: ADC $65
98B9: ADC $65
98BB: ADC $65
98BD: .byte $7C
98BE: ADC $F6
98C0: INC $65,X
98C2: ADC $7C
98C4: .byte $7C
98C5: .byte $7C
98C6: .byte $7C
98C7: ADC $65
98C9: .byte $7C
98CA: ADC $F7
98CC: .byte $F7
98CD: ADC $7C
98CF: ADC $65
98D1: ADC $65
98D3: ADC $7C
98D5: .byte $7C
98D6: ADC $F6
98D8: INC $65,X
98DA: .byte $7C
98DB: ADC $7C
98DD: .byte $7C
98DE: .byte $7C
98DF: ADC $7C
98E1: .byte $7C
98E2: ADC $F7
98E4: .byte $F7
98E5: ADC $7C
98E7: ADC $65
98E9: .byte $7C
98EA: ADC $65
98EC: .byte $7C
98ED: .byte $7C
98EE: ADC $F6
98F0: INC $65,X
98F2: .byte $7C
98F3: .byte $7C
98F4: ADC $65
98F6: ADC $65
98F8: ADC $7C
98FA: ADC $F7
98FC: .byte $F7
98FD: RTI
98FE: RTI
98FF: RTI
9900: RTI
9901: RTI
9902: ADC $F6
9904: ADC $F6
9906: ADC $F6
9908: INC $F6,X
990A: INC $F7,X
990C: INC $F7,X
990E: INC $F7,X
9910: .byte $9C
9911: .byte $F7
9912: EOR $F7F7
9915: .byte $F7
9916: .byte $F7
9917: INC $F7,X
9919: INC $F7,X
991B: .byte $9C
991C: STA $4DF6,X
991F: INC $F6,X
9921: .byte $F7
9922: .byte $9C
9923: .byte $F7
9924: .byte $9C
9925: .byte $F7
9926: .byte $9C
9927: STA $F7F6,X
992A: EOR $F7F7
992D: .byte $9C
992E: .byte $89
992F: .byte $89
9930: .byte $89
9931: .byte $89
9932: .byte $89
9933: INC $F7,X
9935: INC $4D,X
9937: INC $89,X
9939: .byte $89
993A: .byte $89
993B: .byte $89
993C: .byte $89
993D: .byte $89
993E: .byte $89
993F: .byte $F7
9940: INC $F7,X
9942: EOR $98F7
9945: .byte $9B
9946: .byte $89
9947: .byte $89
9948: .byte $89
9949: .byte $89
994A: STA $F79B,X
994D: INC $4D,X
994F: INC $F6,X
9951: TXS
9952: .byte $9B
9953: STA $9D9B,X
9956: INC $9A,X
9958: INC $F7,X
995A: EOR $F7F7
995D: INC $9A,X
995F: INC $9A,X
9961: INC $F7,X
9963: INC $F7,X
9965: INC $4D,X
9967: INC $F6,X
9969: .byte $F7
996A: INC $F7,X
996C: INC $F7,X
996E: INC $F7,X
9970: INC $F7,X
9972: EOR $F7F7
9975: INC $F7,X
9977: INC $F7,X
9979: INC $F7,X
997B: INC $F7,X
997D: EOR $F699
9980: INC $F7,X
9982: INC $F7,X
9984: INC $F7,X
9986: INC $F7,X
9988: INC $4D,X
998A: STA $F7F7,Y
998D: INC $F7,X
998F: INC $F7,X
9991: INC $F7,X
9993: INC $F7,X
9995: EOR $F699
9998: INC $F7,X
999A: INC $F7,X
999C: INC $F7,X
999E: INC $F7,X
99A0: INC $4D,X
99A2: STA $F7F7,Y
99A5: INC $F7,X
99A7: INC $F7,X
99A9: INC $F7,X
99AB: INC $F7,X
99AD: EOR $F699
99B0: INC $F7,X
99B2: INC $F7,X
99B4: STA $F6F7,Y
99B7: .byte $F7
99B8: INC $4D,X
99BA: STA $F7F7,Y
99BD: INC $F7,X
99BF: INC $99,X
99C1: INC $F7,X
99C3: INC $F7,X
99C5: EOR $F699
99C8: INC $F7,X
99CA: INC $F7,X
99CC: .byte $9C
99CD: .byte $F7
99CE: INC $F7,X
99D0: INC $4D,X
99D2: STA $F7F7,Y
99D5: STA $9CF7,Y
99D8: .byte $89
99D9: INC $F7,X
99DB: INC $F7,X
99DD: EOR $F69C
99E0: RTI
99E1: RTI
99E2: RTI
99E3: RTI
99E4: RTI
99E5: .byte $F7
99E6: INC $F7,X
99E8: STA $8989,Y
99EB: .byte $F7
99EC: INC $99,X
99EE: STA $F69B,X
99F1: INC $F7,X
99F3: INC $9A,X
99F5: .byte $9B
99F6: .byte $89
99F7: INC $F7,X
99F9: .byte $42
99FA: INC $9A,X
99FC: .byte $F7
99FD: .byte $F7
99FE: INC $F7,X
9A00: INC $99,X
9A02: .byte $89
9A03: .byte $F7
9A04: INC $42,X
9A06: .byte $F7
9A07: INC $F6,X
9A09: INC $F7,X
9A0B: INC $F7,X
9A0D: .byte $9C
9A0E: .byte $89
9A0F: INC $F7,X
9A11: .byte $42
9A12: INC $F7,X
9A14: .byte $F7
9A15: .byte $F7
9A16: INC $F7,X
9A18: .byte $9C
9A19: .byte $89
9A1A: .byte $89
9A1B: .byte $F7
9A1C: INC $99,X
9A1E: .byte $F7
9A1F: INC $99,X
9A21: INC $F7,X
9A23: .byte $9C
9A24: .byte $89
9A25: .byte $89
9A26: .byte $89
9A27: INC $F7,X
9A29: STA $F789,Y
9A2C: .byte $9C
9A2D: .byte $F7
9A2E: .byte $9C
9A2F: .byte $89
9A30: .byte $89
9A31: .byte $89
9A32: .byte $89
9A33: .byte $F7
9A34: INC $99,X
9A36: .byte $89
9A37: RTI
9A38: RTI
9A39: RTI
9A3A: RTI
9A3B: RTI
9A3C: RTI
9A3D: RTI
9A3E: RTI
9A3F: INC $F7,X
9A41: TXS
9A42: TYA
9A43: TYA
9A44: TYA
9A45: TYA
9A46: TYA
9A47: TYA
9A48: TYA
9A49: TYA
9A4A: INC $F7,X
9A4C: INC $8E,X
9A4E: .byte $8B
9A4F: .byte $8B
9A50: .byte $8B
9A51: .byte $8B
9A52: .byte $8B
9A53: .byte $8B
9A54: .byte $8B
9A55: .byte $8B
9A56: .byte $F7
9A57: INC $F7,X
9A59: .byte $8F
9A5A: TXA
9A5B: TXA
9A5C: TXA
9A5D: TXA
9A5E: TXA
9A5F: TXA
9A60: TXA
9A61: TXA
9A62: INC $F7,X
9A64: INC $99,X
9A66: .byte $89
9A67: .byte $89
9A68: .byte $89
9A69: .byte $89
9A6A: .byte $89
9A6B: .byte $89
9A6C: .byte $89
9A6D: .byte $89
9A6E: .byte $F7
9A6F: INC $F7,X
9A71: STA $8989,Y
9A74: LSR $89,X
9A76: LSR $89,X
9A78: .byte $44
9A79: EOR ($F6,X)
9A7B: .byte $F7
9A7C: INC $99,X
9A7E: .byte $89
9A7F: .byte $89
9A80: .byte $89
9A81: .byte $89
9A82: .byte $89
9A83: .byte $89
9A84: .byte $89
9A85: .byte $89
9A86: .byte $F7
9A87: INC $F7,X
9A89: STA $8989,Y
9A8C: LSR $89,X
9A8E: LSR $89,X
9A90: .byte $89
9A91: STA $F7F6,X
9A94: INC $99,X
9A96: .byte $89
9A97: .byte $89
9A98: .byte $89
9A99: .byte $89
9A9A: .byte $89
9A9B: .byte $89
9A9C: .byte $89
9A9D: INC $F7,X
9A9F: INC $F7,X
9AA1: STA $8989,Y
9AA4: LSR $89,X
9AA6: LSR $89,X
9AA8: .byte $89
9AA9: .byte $F7
9AAA: INC $F7,X
9AAC: INC $99,X
9AAE: .byte $89
9AAF: .byte $89
9AB0: .byte $89
9AB1: .byte $89
9AB2: .byte $89
9AB3: .byte $89
9AB4: .byte $89
9AB5: .byte $89
9AB6: .byte $F7
9AB7: INC $F7,X
9AB9: STA $8A8A,Y
9ABC: TXA
9ABD: TXA
9ABE: TXA
9ABF: TXA
9AC0: TXA
9AC1: TXA
9AC2: TXA
9AC3: .byte $F7
9AC4: INC $F6,X
9AC6: TXS
9AC7: .byte $9B
9AC8: .byte $89
9AC9: .byte $89
9ACA: STA $899B,X
9ACD: .byte $89
9ACE: STA $F7F6,X
9AD1: .byte $F7
9AD2: INC $9A,X
9AD4: .byte $9B
9AD5: STA $9AF6,X
9AD8: .byte $9B
9AD9: .byte $89
9ADA: INC $F7,X
9ADC: INC $F6,X
9ADE: .byte $F7
9ADF: INC $9A,X
9AE1: INC $F7,X
9AE3: INC $9A,X
9AE5: .byte $9B
9AE6: .byte $F7
9AE7: INC $F7,X
9AE9: .byte $F7
9AEA: INC $F7,X
9AEC: INC $F7,X
9AEE: INC $F7,X
9AF0: INC $9A,X
9AF2: INC $F7,X
9AF4: .byte $F7
9AF5: INC $F7,X
9AF7: INC $F7,X
9AF9: INC $F7,X
9AFB: INC $F7,X
9AFD: INC $F7,X
9AFF: INC $00,X
9B01: AND $8725
9B04: STA $0604,Y
9B07: ORA ($2D,X)
9B09: RTI
9B0A: BRK
9B0B: .byte $02
9B0C: BRK
9B0D: BRK
9B0E: BRK
9B0F: BRK
9B10: BRK
9B11: .byte $5F
9B12: .byte $07
9B13: BVC $9B17
9B15: .byte $04
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
9B20: EOR ($03,X)
9B22: .byte $1A
9B23: RTS
9B24: .byte $32
9B25: .byte $02
9B26: EOR $0602
9B29: .byte $02
9B2A: BRK
9B2B: BRK
9B2C: BRK
9B2D: BRK
9B2E: BRK
9B2F: BRK
9B30: ADC ($01,X)
9B32: ROL $30,X
9B34: AND $6502
9B37: BRK
9B38: .byte $04
9B39: ASL $00
9B3B: BRK
9B3C: BRK
9B3D: BRK
9B3E: BRK
9B3F: BRK
9B40: ADC ($21,X)
9B42: ORA $30,X
9B44: AND $6502
9B47: BRK
9B48: .byte $04
9B49: ASL $00
9B4B: BRK
9B4C: BRK
9B4D: BRK
9B4E: BRK
9B4F: BRK
9B50: ADC ($01,X)
9B52: PHP
9B53: RTI
9B54: AND $6502
9B57: BRK
9B58: .byte $07
9B59: ASL $00
9B5B: BRK
9B5C: BRK
9B5D: BRK
9B5E: BRK
9B5F: BRK
9B60: ADC ($03,X)
9B62: .byte $02
9B63: RTS
9B64: AND $6502
9B67: BRK
9B68: .byte $04
9B69: ASL $00
9B6B: BRK
9B6C: BRK
9B6D: BRK
9B6E: BRK
9B6F: BRK
9B70: ADC ($03,X)
9B72: PLP
9B73: BCC $9BA2
9B75: .byte $02
9B76: ADC $00
9B78: .byte $04
9B79: ASL $00
9B7B: BRK
9B7C: BRK
9B7D: BRK
9B7E: BRK
9B7F: BRK
9B80: EOR ($03),Y
9B82: BMI $9B14
9B84: .byte $32
9B85: .byte $03
9B86: ADC $0201
9B89: ORA ($00,X)
9B8B: BRK
9B8C: BRK
9B8D: BRK
9B8E: BRK
9B8F: BRK
9B90: EOR ($02),Y
9B92: ASL $A0,X
9B94: .byte $32
9B95: ORA ($6D,X)
9B97: ORA ($02,X)
9B99: ORA ($00,X)
9B9B: BRK
9B9C: BRK
9B9D: BRK
9B9E: BRK
9B9F: BRK
9BA0: EOR ($01),Y
9BA2: BIT $40
9BA4: .byte $32
9BA5: .byte $03
9BA6: ADC $0201
9BA9: ORA ($00,X)
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
9BE6: BIT $0F30
9BE9: .byte $0B
9BEA: .byte $1B
9BEB: ROL $0F,X
9BED: .byte $0C
9BEE: .byte $1B
9BEF: .byte $2B
9BF0: .byte $0F
9BF1: ORA $30
9BF3: ROL $0F,X
9BF5: ORA $26
9BF7: BMI $9C08
9BF9: ORA $27
9BFB: SEC
9BFC: .byte $0F
9BFD: .byte $02
9BFE: ORA $30
9C00: LDX $B7,Y
9C02: LDX $B7,Y
9C04: LDX $B7,Y
9C06: LDX $B7,Y
9C08: LDX $B7,Y
9C0A: LDX $B7,Y
9C0C: .byte $B7
9C0D: DEC $DDB7,X
9C10: .byte $B7
9C11: CMP $DDB7,X
9C14: .byte $B7
9C15: LDX $B7,Y
9C17: LDX $40,Y
9C19: DEC $DDDD,X
9C1C: CMP $DDDD,X
9C1F: CMP $B7DD,X
9C22: LDX $B7,Y
9C24: LDX $DE,Y
9C26: CMP $DDDD,X
9C29: CMP $DDB6,X
9C2C: CMP $B7DD,X
9C2F: LDX $B7,Y
9C31: LDX $DD,Y
9C33: LDX $DD,Y
9C35: LDX $B7,Y
9C37: CMP $DDDD,X
9C3A: CMP $B6B7,X
9C3D: .byte $B7
9C3E: LDX $B7,Y
9C40: LDX $B7,Y
9C42: LDX $DD,Y
9C44: CMP $DDDD,X
9C47: LDX $B7,Y
9C49: LDX $B7,Y
9C4B: CMP $DDB7,X
9C4E: .byte $B7
9C4F: LDX $DD,Y
9C51: CMP $B7B6,X
9C54: LDX $B7,Y
9C56: CMP $40DD,X
9C59: RTI
9C5A: RTI
9C5B: .byte $B7
9C5C: CMP $B7B6,X
9C5F: LDX $B7,Y
9C61: DEC $DDDD,X
9C64: LDX $DD,Y
9C66: CMP $DDDD,X
9C69: .byte $B7
9C6A: LDX $B7,Y
9C6C: LDX $DE,Y
9C6E: CMP $B7B6,X
9C71: LDX $DD,Y
9C73: CMP $B6DD,X
9C76: .byte $B7
9C77: LDX $B7,Y
9C79: DEC $B7DD,X
9C7C: LDX $B7,Y
9C7E: LDX $DD,Y
9C80: LDX $B7,Y
9C82: LDX $B7,Y
9C84: LDX $DE,Y
9C86: CMP $B7DD,X
9C89: LDX $B7,Y
9C8B: LDX $B7,Y
9C8D: LDX $B7,Y
9C8F: LDX $B7,Y
9C91: LDX $DD,Y
9C93: LSR $B74B
9C96: .byte $4B
9C97: .byte $B7
9C98: .byte $4B
9C99: .byte $B7
9C9A: LDX $B7,Y
9C9C: LDX $B7,Y
9C9E: LDX $50,Y
9CA0: JMP $4C4C
9CA3: JMP $4C4C
9CA6: .byte $B7
9CA7: LDX $B7,Y
9CA9: LDX $B7,Y
9CAB: LDX $4B,Y
9CAD: .byte $4B
9CAE: .byte $4B
9CAF: .byte $4B
9CB0: .byte $4B
9CB1: .byte $4B
9CB2: LDX $B7,Y
9CB4: LDX $B7,Y
9CB6: LDX $B7,Y
9CB8: JMP $4C4C
9CBB: JMP $4C4C
9CBE: .byte $B7
9CBF: LDX $B7,Y
9CC1: LDX $B7,Y
9CC3: LSR A
9CC4: LSR A
9CC5: LSR A
9CC6: LSR A
9CC7: LDX $4A,Y
9CC9: RTI
9CCA: RTI
9CCB: .byte $B7
9CCC: LDX $B7,Y
9CCE: CMP $DDDD,X
9CD1: CMP $B7B6,X
9CD4: CMP $DDDD,X
9CD7: LDX $B7,Y
9CD9: DEC $DDDD,X
9CDC: CMP $B7B6,X
9CDF: LDX $4B,Y
9CE1: .byte $4B
9CE2: .byte $4B
9CE3: .byte $B7
9CE4: LDX $DE,Y
9CE6: LDX $DD,Y
9CE8: LDX $B7,Y
9CEA: LDX $B7,Y
9CEC: JMP $4C4C
9CEF: LDX $B7,Y
9CF1: LDX $B7,Y
9CF3: CMP $B6B7,X
9CF6: .byte $B7
9CF7: LDX $4A,Y
9CF9: LSR A
9CFA: LSR A
9CFB: .byte $B7
9CFC: LDX $B7,Y
9CFE: EOR $41
9D00: LDX $B7,Y
9D02: LDX $B7,Y
9D04: CMP $DDDD,X
9D07: LDX $B7,Y
9D09: LDX $B7,Y
9D0B: LDX $B7,Y
9D0D: LDX $B7,Y
9D0F: LDX $4B,Y
9D11: .byte $4B
9D12: .byte $4B
9D13: .byte $B7
9D14: LDX $B7,Y
9D16: LDX $B7,Y
9D18: LDX $B7,Y
9D1A: LDX $B7,Y
9D1C: JMP $4C4C
9D1F: LDX $B7,Y
9D21: LDX $B7,Y
9D23: LDX $B7,Y
9D25: LDX $B7,Y
9D27: LDX $4A,Y
9D29: LSR A
9D2A: LSR A
9D2B: .byte $B7
9D2C: LDX $B7,Y
9D2E: LDX $B7,Y
9D30: LDX $B7,Y
9D32: LDX $B7,Y
9D34: CMP $DDDD,X
9D37: LDX $B7,Y
9D39: LDX $B7,Y
9D3B: LDX $B7,Y
9D3D: CMP $DDB7,X
9D40: CMP $B6DD,X
9D43: .byte $B7
9D44: LDX $B7,Y
9D46: LDX $B7,Y
9D48: CMP $DDDD,X
9D4B: CMP $DDDD,X
9D4E: .byte $B7
9D4F: LDX $B7,Y
9D51: DEC $DDB7,X
9D54: CMP $DDDD,X
9D57: CMP $DDDD,X
9D5A: LDX $B7,Y
9D5C: LDX $DE,Y
9D5E: CMP $DDDD,X
9D61: CMP $DDDD,X
9D64: CMP $B7B6,X
9D67: LDX $B7,Y
9D69: DEC $DDDD,X
9D6C: CMP $B6DD,X
9D6F: CMP $B7DD,X
9D72: LDX $B7,Y
9D74: LDX $DE,Y
9D76: CMP $DDB6,X
9D79: LDX $B7,Y
9D7B: LDX $7E,Y
9D7D: LDX $B7,Y
9D7F: LDX $B7,Y
9D81: DEC $B7DD,X
9D84: LDX $B7,Y
9D86: LDX $B7,Y
9D88: CMP $B6B7,X
9D8B: .byte $B7
9D8C: LDX $DE,Y
9D8E: CMP $B7B6,X
9D91: LDX $B7,Y
9D93: LDX $DD,Y
9D95: CMP $B6B7,X
9D98: .byte $B7
9D99: DEC $B7DD,X
9D9C: LDX $B7,Y
9D9E: LDX $B7,Y
9DA0: .byte $4B
9DA1: .byte $4B
9DA2: LDX $B7,Y
9DA4: LDX $DE,Y
9DA6: CMP $B7B6,X
9DA9: LDX $B7,Y
9DAB: LDX $4C,Y
9DAD: JMP $B6B7
9DB0: .byte $B7
9DB1: DEC $B7DD,X
9DB4: LDX $B7,Y
9DB6: LDX $B7,Y
9DB8: .byte $5C
9DB9: PHA
9DBA: LDX $B7,Y
9DBC: LDX $DE,Y
9DBE: CMP $B7B6,X
9DC1: LDX $B7,Y
9DC3: EOR $4949,Y
9DC6: .byte $B7
9DC7: LDX $B7,Y
9DC9: DEC $B7DD,X
9DCC: LDX $B7,Y
9DCE: LDX $4C,Y
9DD0: JMP $4C4C
9DD3: .byte $B7
9DD4: LDX $DE,Y
9DD6: CMP $B770,X
9DD9: LDX $B7,Y
9DDB: LSR A
9DDC: LSR A
9DDD: LSR A
9DDE: LSR A
9DDF: LDX $B7,Y
9DE1: DEC $DDDD,X
9DE4: BVS $9D9D
9DE6: CMP $DDDD,X
9DE9: CMP $B7B6,X
9DEC: LDX $DE,Y
9DEE: CMP $DDDD,X
9DF1: CMP $DDDD,X
9DF4: CMP $B7DD,X
9DF7: LDX $B7,Y
9DF9: DEC $DDDD,X
9DFC: CMP $DDDD,X
9DFF: LDX $DD,Y
9E01: CMP $B770,X
9E04: LDX $DE,Y
9E06: LDX $DD,Y
9E08: CMP $B6DD,X
9E0B: .byte $B7
9E0C: LDX $DD,Y
9E0E: BVS $9DC6
9E10: .byte $B7
9E11: LDX $B7,Y
9E13: LDX $DD,Y
9E15: LDX $B7,Y
9E17: LDX $B7,Y
9E19: CMP $B770,X
9E1C: LDX $B7,Y
9E1E: LDX $B7,Y
9E20: LDX $B7,Y
9E22: LDX $B7,Y
9E24: LDX $DD,Y
9E26: BVS $9DDE
9E28: .byte $B7
9E29: LDX $B7,Y
9E2B: LDX $B7,Y
9E2D: LDX $B7,Y
9E2F: LDX $B7,Y
9E31: CMP $B7B6,X
9E34: LDX $B7,Y
9E36: LDX $B7,Y
9E38: LDX $B7,Y
9E3A: LDX $B7,Y
9E3C: CMP $B7DD,X
9E3F: LDX $B7,Y
9E41: DEC $B6B7,X
9E44: .byte $B7
9E45: CMP $DDB7,X
9E48: CMP $B6DD,X
9E4B: .byte $B7
9E4C: RTI
9E4D: RTI
9E4E: RTI
9E4F: .byte $B7
9E50: LDX $DD,Y
9E52: CMP $DDDD,X
9E55: CMP $B6B7,X
9E58: LDX $DE,Y
9E5A: CMP $B7B6,X
9E5D: CMP $DDDD,X
9E60: CMP $B6DD,X
9E63: .byte $B7
9E64: .byte $B7
9E65: .byte $4B
9E66: .byte $4B
9E67: .byte $B7
9E68: LDX $4B,Y
9E6A: .byte $4B
9E6B: .byte $4B
9E6C: .byte $4B
9E6D: .byte $4B
9E6E: .byte $B7
9E6F: LDX $B6,Y
9E71: JMP $B64C
9E74: .byte $B7
9E75: JMP $4C4C
9E78: JMP $B64C
9E7B: .byte $B7
9E7C: .byte $B7
9E7D: LSR A
9E7E: LSR A
9E7F: .byte $B7
9E80: LDX $4A,Y
9E82: LSR A
9E83: LSR A
9E84: LSR A
9E85: LSR A
9E86: .byte $B7
9E87: LDX $B6,Y
9E89: EOR $B648,Y
9E8C: .byte $B7
9E8D: LDX $59,Y
9E8F: EOR #$48
9E91: EOR #$48
9E93: .byte $B7
9E94: .byte $B7
9E95: .byte $5A
9E96: CLI
9E97: .byte $B7
9E98: LDX $B7,Y
9E9A: .byte $5A
9E9B: CLI
9E9C: CLI
9E9D: CLI
9E9E: CLI
9E9F: LDX $B6,Y
9EA1: .byte $4B
9EA2: .byte $4B
9EA3: LDX $B7,Y
9EA5: .byte $4B
9EA6: .byte $4B
9EA7: .byte $4B
9EA8: .byte $4B
9EA9: .byte $4B
9EAA: .byte $4B
9EAB: .byte $B7
9EAC: .byte $B7
9EAD: JMP $B74C
9EB0: LDX $4C,Y
9EB2: JMP $4C4C
9EB5: JMP $B64C
9EB8: LDX $4A,Y
9EBA: LSR A
9EBB: LDX $B7,Y
9EBD: LSR A
9EBE: LSR A
9EBF: LSR A
9EC0: LSR A
9EC1: LSR A
9EC2: LDX $B7,Y
9EC4: .byte $B7
9EC5: DEC $B7DD,X
9EC8: LDX $DD,Y
9ECA: CMP $DDDD,X
9ECD: LDX $B7,Y
9ECF: LDX $40,Y
9ED1: RTI
9ED2: CMP $B7DD,X
9ED5: CMP $DDDD,X
9ED8: LDX $B7,Y
9EDA: LDX $B7,Y
9EDC: .byte $B7
9EDD: LDX $DD,Y
9EDF: CMP $DDDD,X
9EE2: CMP $B7B6,X
9EE5: LDX $B7,Y
9EE7: LDX $B6,Y
9EE9: .byte $B7
9EEA: LDX $DD,Y
9EEC: LDX $DD,Y
9EEE: LDX $B7,Y
9EF0: LDX $B7,Y
9EF2: LDX $B7,Y
9EF4: .byte $B7
9EF5: LDX $B7,Y
9EF7: LDX $B7,Y
9EF9: LDX $B7,Y
9EFB: LDX $B7,Y
9EFD: LDX $B7,Y
9EFF: LDX $07,Y
9F01: BIT $4405
9F04: CMP $0200,X
9F07: ORA ($39,X)
9F09: LDY #$04
9F0B: .byte $02
9F0C: BRK
9F0D: BRK
9F0E: BRK
9F0F: BRK
9F10: BRK
9F11: PLP
9F12: .byte $07
9F13: PLP
9F14: .byte $02
9F15: .byte $04
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
9F22: .byte $13
9F23: BMI $9F61
9F25: .byte $04
9F26: EOR #$03
9F28: ASL $00
9F2A: BRK
9F2B: BRK
9F2C: BRK
9F2D: BRK
9F2E: BRK
9F2F: BRK
9F30: ADC ($03),Y
9F32: AND $3CA0,Y
9F35: .byte $04
9F36: EOR #$03
9F38: .byte $02
9F39: BRK
9F3A: BRK
9F3B: BRK
9F3C: BRK
9F3D: BRK
9F3E: BRK
9F3F: BRK
9F40: ADC ($03),Y
9F42: .byte $03
9F43: BVC $9F81
9F45: .byte $03
9F46: EOR #$03
9F48: .byte $02
9F49: BRK
9F4A: BRK
9F4B: BRK
9F4C: BRK
9F4D: BRK
9F4E: BRK
9F4F: BRK
9F50: ADC ($03,X)
9F52: CLC
9F53: BCC $9F77
9F55: .byte $02
9F56: ADC $0202
9F59: .byte $02
9F5A: BRK
9F5B: BRK
9F5C: BRK
9F5D: BRK
9F5E: BRK
9F5F: BRK
9F60: ADC ($02,X)
9F62: ORA ($80),Y
9F64: .byte $22
9F65: .byte $02
9F66: ADC $0002
9F69: ORA ($00,X)
9F6B: BRK
9F6C: BRK
9F6D: BRK
9F6E: BRK
9F6F: BRK
9F70: EOR ($01),Y
9F72: PHP
9F73: .byte $80
9F74: JSR $5D02
9F77: .byte $02
9F78: BRK
9F79: .byte $02
9F7A: BRK
9F7B: BRK
9F7C: BRK
9F7D: BRK
9F7E: BRK
9F7F: BRK
9F80: EOR ($01),Y
9F82: .byte $03
9F83: .byte $80
9F84: ASL $5D02,X
9F87: .byte $02
9F88: BRK
9F89: .byte $02
9F8A: BRK
9F8B: BRK
9F8C: BRK
9F8D: BRK
9F8E: BRK
9F8F: BRK
9F90: EOR ($02),Y
9F92: BIT $1E90
9F95: .byte $02
9F96: EOR $0202,X
9F99: .byte $02
9F9A: BRK
9F9B: BRK
9F9C: BRK
9F9D: BRK
9F9E: BRK
9F9F: BRK
9FA0: EOR ($02),Y
9FA2: BIT $20
9FA4: EOR ($02,X)
9FA6: EOR $0002,X
9FA9: .byte $02
9FAA: BRK
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
9FE5: ORA #$10
9FE7: BMI $9FF8
9FE9: .byte $07
9FEA: .byte $17
9FEB: .byte $27
9FEC: .byte $0F
9FED: ORA ($12,X)
9FEF: .byte $3C
9FF0: .byte $0F
9FF1: ORA $30
9FF3: ROL $0F,X
9FF5: ORA $26
9FF7: BMI $A008
9FF9: .byte $12
9FFA: ROL $30
9FFC: .byte $0F
9FFD: ORA $27
9FFF: SEC
