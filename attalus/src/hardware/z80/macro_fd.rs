macro_rules! attalus_z80_fd {
    ($mac: ident) => {
        attalus_z80_fd! {@ $mac
(
[ 0x00 ; x00 ;    ; nop    ()         ;  4 ; undoc ; z80   ]
[ 0x01 ; x01 ; nn ; ld16   (BC, nn)   ; 10 ; undoc ; z80   ]
[ 0x02 ; x02 ;    ; ld     ((BC), A)  ;  7 ; undoc ; z80   ]
[ 0x03 ; x03 ;    ; inc16  (BC)       ;  6 ; undoc ; z80   ]
[ 0x04 ; x04 ;    ; inc    (B)        ;  4 ; undoc ; z80   ]
[ 0x05 ; x05 ;    ; dec    (B)        ;  4 ; undoc ; z80   ]
[ 0x06 ; x06 ; n  ; ld     (B, n)     ;  7 ; undoc ; z80   ]
[ 0x07 ; x07 ;    ; rlca   ()         ;  4 ; undoc ; z80   ]
[ 0x08 ; x08 ;    ; ex     (AF, AF0)  ;  4 ; undoc ; z80   ]

[ 0x09 ; x09 ;      ; add16 (IY, BC)    ; 11 ; doc   ; z80   ]

[ 0x0A ; x0A ;    ; ld     (A, (BC))  ;  7 ; undoc ; z80   ]
[ 0x0B ; x0B ;    ; dec16  (BC)       ;  6 ; undoc ; z80   ]
[ 0x0C ; x0C ;    ; inc    (C)        ;  4 ; undoc ; z80   ]
[ 0x0D ; x0D ;    ; dec    (C)        ;  4 ; undoc ; z80   ]
[ 0x0E ; x0E ; n  ; ld     (C, n)     ;  7 ; undoc ; z80   ]
[ 0x0F ; x0F ;    ; rrca   ()         ;  4 ; undoc ; z80   ]
[ 0x10 ; x10 ; e  ; djnz   (e)        ;  8 ; undoc ; z80   ]
[ 0x11 ; x11 ; nn ; ld16   (DE, nn)   ; 10 ; undoc ; z80   ]
[ 0x12 ; x12 ;    ; ld     ((DE), A)  ;  7 ; undoc ; z80   ]
[ 0x13 ; x13 ;    ; inc16  (DE)       ;  6 ; undoc ; z80   ]
[ 0x14 ; x14 ;    ; inc    (D)        ;  4 ; undoc ; z80   ]
[ 0x15 ; x15 ;    ; dec    (D)        ;  4 ; undoc ; z80   ]
[ 0x16 ; x16 ; n  ; ld     (D, n)     ;  7 ; undoc ; z80   ]
[ 0x17 ; x17 ;    ; rla    ()         ;  4 ; undoc ; z80   ]
[ 0x18 ; x18 ; e  ; jr     (e)        ; 12 ; undoc ; z80   ]

[ 0x19 ; x19 ;      ; add16 (IY, DE)    ; 11 ; doc   ; z80   ]

[ 0x1A ; x1A ;    ; ld     (A, (DE))  ;  7 ; undoc ; z80   ]
[ 0x1B ; x1B ;    ; dec16  (DE)       ;  6 ; undoc ; z80   ]
[ 0x1C ; x1C ;    ; inc    (E)        ;  4 ; undoc ; z80   ]
[ 0x1D ; x1D ;    ; dec    (E)        ;  4 ; undoc ; z80   ]
[ 0x1E ; x1E ; n  ; ld     (E, n)     ;  7 ; undoc ; z80   ]
[ 0x1F ; x1F ;    ; rra    ()         ;  4 ; undoc ; z80   ]
[ 0x20 ; x20 ; e  ; jrcc   (NZcc, e)  ; xx ; undoc ; z80   ]

[ 0x21 ; x21 ; nn   ; ld16  (IY, nn)    ; 10 ; doc   ; z80   ]
[ 0x22 ; x22 ; nn   ; ld16  ((nn), IY)  ; 16 ; doc   ; z80   ]
[ 0x23 ; x23 ;      ; inc16 (IY)        ;  6 ; doc   ; z80   ]
[ 0x24 ; x24 ;      ; inc   (IYH)       ;  4 ; undoc ; z80   ]
[ 0x25 ; x25 ;      ; dec   (IYH)       ;  4 ; undoc ; z80   ]
[ 0x26 ; x26 ; n    ; ld    (IYH, n)    ;  7 ; undoc ; z80   ]

[ 0x27 ; x27 ;    ; daa    ()         ;  4 ; undoc ; z80   ]
[ 0x28 ; x28 ; e  ; jrcc   (Zcc, e)   ; xx ; undoc ; z80   ]

[ 0x29 ; x29 ;      ; add16 (IY, IY)    ; 11 ; doc   ; z80   ]
[ 0x2A ; x2A ; nn   ; ld16  (IY, (nn))  ; 16 ; doc   ; z80   ]
[ 0x2B ; x2B ;      ; dec16 (IY)        ;  6 ; doc   ; z80   ]
[ 0x2C ; x2C ;      ; inc   (IYL)       ;  4 ; undoc ; z80   ]
[ 0x2D ; x2D ;      ; dec   (IYL)       ;  4 ; undoc ; z80   ]
[ 0x2E ; x2E ; n    ; ld    (IYL, n)    ;  7 ; undoc ; z80   ]

[ 0x2F ; x2F ;    ; cpl    ()         ;  4 ; undoc ; z80   ]
[ 0x30 ; x30 ; e  ; jrcc   (NCcc, e)  ; xx ; undoc ; z80   ]
[ 0x31 ; x31 ; nn ; ld16   (SP, nn)   ; 10 ; undoc ; z80   ]
[ 0x32 ; x32 ; nn ; ld     ((nn), A)  ; 13 ; undoc ; z80   ]
[ 0x33 ; x33 ;    ; inc16  (SP)       ;  6 ; undoc ; z80   ]

[ 0x34 ; x34 ; d    ; inc   ((IY+d))    ; 19 ; doc   ; z80   ]
[ 0x35 ; x35 ; d    ; dec   ((IY+d))    ; 19 ; doc   ; z80   ]
[ 0x36 ; x36 ; d, n ; ld    ((IY+d), n) ; 15 ; doc   ; z80   ]

[ 0x37 ; x37 ;    ; scf    ()         ;  4 ; undoc ; z80   ]
[ 0x38 ; x38 ; e  ; jrcc   (Ccc, e)   ; xx ; undoc ; z80   ]

[ 0x39 ; x39 ;      ; add16 (IY, SP)    ; 11 ; doc   ; z80   ]

[ 0x3A ; x3A ; nn ; ld     (A, (nn))  ; 13 ; undoc ; z80   ]
[ 0x3B ; x3B ;    ; dec16  (SP)       ;  6 ; undoc ; z80   ]
[ 0x3C ; x3C ;    ; inc    (A)        ;  4 ; undoc ; z80   ]
[ 0x3D ; x3D ;    ; dec    (A)        ;  4 ; undoc ; z80   ]
[ 0x3E ; x3E ; n  ; ld     (A, n)     ;  7 ; undoc ; z80   ]
[ 0x3F ; x3F ;    ; ccf    ()         ;  4 ; undoc ; z80   ]
[ 0x40 ; x40 ;    ; ld     (B, B)     ;  4 ; undoc ; z80   ]
[ 0x41 ; x41 ;    ; ld     (B, C)     ;  4 ; undoc ; z80   ]
[ 0x42 ; x42 ;    ; ld     (B, D)     ;  4 ; undoc ; z80   ]
[ 0x43 ; x43 ;    ; ld     (B, E)     ;  4 ; undoc ; z80   ]

[ 0x44 ; x44 ;      ; ld    (B, IYH)    ;  4 ; undoc ; z80   ]
[ 0x45 ; x45 ;      ; ld    (B, IYL)    ;  4 ; undoc ; z80   ]
[ 0x46 ; x46 ; d    ; ld    (B, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x47 ; x47 ;    ; ld     (B, A)     ;  4 ; undoc ; z80   ]
[ 0x48 ; x48 ;    ; ld     (C, B)     ;  4 ; undoc ; z80   ]
[ 0x49 ; x49 ;    ; ld     (C, C)     ;  4 ; undoc ; z80   ]
[ 0x4A ; x4A ;    ; ld     (C, D)     ;  4 ; undoc ; z80   ]
[ 0x4B ; x4B ;    ; ld     (C, E)     ;  4 ; undoc ; z80   ]

[ 0x4C ; x4C ;      ; ld    (C, IYH)    ;  4 ; undoc ; z80   ]
[ 0x4D ; x4D ;      ; ld    (C, IYL)    ;  4 ; undoc ; z80   ]
[ 0x4E ; x4E ; d    ; ld    (C, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x4F ; x4F ;    ; ld     (C, A)     ;  4 ; undoc ; z80   ]
[ 0x50 ; x50 ;    ; ld     (D, B)     ;  4 ; undoc ; z80   ]
[ 0x51 ; x51 ;    ; ld     (D, C)     ;  4 ; undoc ; z80   ]
[ 0x52 ; x52 ;    ; ld     (D, D)     ;  4 ; undoc ; z80   ]
[ 0x53 ; x53 ;    ; ld     (D, E)     ;  4 ; undoc ; z80   ]

[ 0x54 ; x54 ;      ; ld    (D, IYH)    ;  4 ; undoc ; z80   ]
[ 0x55 ; x55 ;      ; ld    (D, IYL)    ;  4 ; undoc ; z80   ]
[ 0x56 ; x56 ; d    ; ld    (D, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x57 ; x57 ;    ; ld     (D, A)     ;  4 ; undoc ; z80   ]
[ 0x58 ; x58 ;    ; ld     (E, B)     ;  4 ; undoc ; z80   ]
[ 0x59 ; x59 ;    ; ld     (E, C)     ;  4 ; undoc ; z80   ]
[ 0x5A ; x5A ;    ; ld     (E, D)     ;  4 ; undoc ; z80   ]
[ 0x5B ; x5B ;    ; ld     (E, E)     ;  4 ; undoc ; z80   ]

[ 0x5C ; x5C ;      ; ld    (E, IYH)    ;  4 ; undoc ; z80   ]
[ 0x5D ; x5D ;      ; ld    (E, IYL)    ;  4 ; undoc ; z80   ]
[ 0x5E ; x5E ; d    ; ld    (E, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x5F ; x5F ;    ; ld     (E, A)     ;  4 ; undoc ; z80   ]

[ 0x60 ; x60 ;      ; ld    (IYH, B)    ;  4 ; undoc ; z80   ]
[ 0x61 ; x61 ;      ; ld    (IYH, C)    ;  4 ; undoc ; z80   ]
[ 0x62 ; x62 ;      ; ld    (IYH, D)    ;  4 ; undoc ; z80   ]
[ 0x63 ; x63 ;      ; ld    (IYH, E)    ;  4 ; undoc ; z80   ]
[ 0x64 ; x64 ;      ; ld    (IYH, IYH)  ;  4 ; undoc ; z80   ]
[ 0x65 ; x65 ;      ; ld    (IYH, IYL)  ;  4 ; undoc ; z80   ]
[ 0x66 ; x66 ; d    ; ld    (H, (IY+d)) ; 15 ; doc   ; z80   ]
[ 0x67 ; x67 ;      ; ld    (IYH, A)    ;  4 ; undoc ; z80   ]
[ 0x68 ; x68 ;      ; ld    (IYL, B)    ;  4 ; undoc ; z80   ]
[ 0x69 ; x69 ;      ; ld    (IYL, C)    ;  4 ; undoc ; z80   ]
[ 0x6A ; x6A ;      ; ld    (IYL, D)    ;  4 ; undoc ; z80   ]
[ 0x6B ; x6B ;      ; ld    (IYL, E)    ;  4 ; undoc ; z80   ]
[ 0x6C ; x6C ;      ; ld    (IYL, IYH)  ;  4 ; undoc ; z80   ]
[ 0x6D ; x6D ;      ; ld    (IYL, IYL)  ;  4 ; undoc ; z80   ]
[ 0x6E ; x6E ; d    ; ld    (L, (IY+d)) ; 15 ; doc   ; z80   ]
[ 0x6F ; x6F ;      ; ld    (IYL, A)    ;  4 ; undoc ; z80   ]
[ 0x70 ; x70 ; d    ; ld    ((IY+d), B) ; 15 ; doc   ; z80   ]
[ 0x71 ; x71 ; d    ; ld    ((IY+d), C) ; 15 ; doc   ; z80   ]
[ 0x72 ; x72 ; d    ; ld    ((IY+d), D) ; 15 ; doc   ; z80   ]
[ 0x73 ; x73 ; d    ; ld    ((IY+d), E) ; 15 ; doc   ; z80   ]
[ 0x74 ; x74 ; d    ; ld    ((IY+d), H) ; 15 ; doc   ; z80   ]
[ 0x75 ; x75 ; d    ; ld    ((IY+d), L) ; 15 ; doc   ; z80   ]

[ 0x76 ; x76 ;    ; halt   ()         ;  4 ; undoc ; z80   ]

[ 0x77 ; x77 ; d    ; ld    ((IY+d), A) ; 15 ; doc   ; z80   ]

[ 0x78 ; x78 ;    ; ld     (A, B)     ;  4 ; undoc ; z80   ]
[ 0x79 ; x79 ;    ; ld     (A, C)     ;  4 ; undoc ; z80   ]
[ 0x7A ; x7A ;    ; ld     (A, D)     ;  4 ; undoc ; z80   ]
[ 0x7B ; x7B ;    ; ld     (A, E)     ;  4 ; undoc ; z80   ]

[ 0x7C ; x7C ;      ; ld    (A, IYH)    ;  4 ; undoc ; z80   ]
[ 0x7D ; x7D ;      ; ld    (A, IYL)    ;  4 ; undoc ; z80   ]
[ 0x7E ; x7E ; d    ; ld    (A, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x7F ; x7F ;    ; ld     (A, A)     ;  4 ; undoc ; z80   ]
[ 0x80 ; x80 ;    ; add    (A, B)     ;  4 ; undoc ; z80   ]
[ 0x81 ; x81 ;    ; add    (A, C)     ;  4 ; undoc ; z80   ]
[ 0x82 ; x82 ;    ; add    (A, D)     ;  4 ; undoc ; z80   ]
[ 0x83 ; x83 ;    ; add    (A, E)     ;  4 ; undoc ; z80   ]

[ 0x84 ; x84 ;      ; add   (A, IYH)    ;  4 ; undoc ; z80   ]
[ 0x85 ; x85 ;      ; add   (A, IYL)    ;  4 ; undoc ; z80   ]
[ 0x86 ; x86 ; d    ; add   (A, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x87 ; x87 ;    ; add    (A, A)     ;  4 ; undoc ; z80   ]
[ 0x88 ; x88 ;    ; adc    (A, B)     ;  4 ; undoc ; z80   ]
[ 0x89 ; x89 ;    ; adc    (A, C)     ;  4 ; undoc ; z80   ]
[ 0x8A ; x8A ;    ; adc    (A, D)     ;  4 ; undoc ; z80   ]
[ 0x8B ; x8B ;    ; adc    (A, E)     ;  4 ; undoc ; z80   ]

[ 0x8C ; x8C ;      ; adc   (A, IYH)    ;  4 ; undoc ; z80   ]
[ 0x8D ; x8D ;      ; adc   (A, IYL)    ;  4 ; undoc ; z80   ]
[ 0x8E ; x8E ; d    ; adc   (A, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x8F ; x8F ;    ; adc    (A, A)     ;  4 ; undoc ; z80   ]
[ 0x90 ; x90 ;    ; sub    (A, B)     ;  4 ; undoc ; z80   ]
[ 0x91 ; x91 ;    ; sub    (A, C)     ;  4 ; undoc ; z80   ]
[ 0x92 ; x92 ;    ; sub    (A, D)     ;  4 ; undoc ; z80   ]
[ 0x93 ; x93 ;    ; sub    (A, E)     ;  4 ; undoc ; z80   ]

[ 0x94 ; x94 ;      ; sub   (A, IYH)    ;  4 ; undoc ; z80   ]
[ 0x95 ; x95 ;      ; sub   (A, IYL)    ;  4 ; undoc ; z80   ]
[ 0x96 ; x96 ; d    ; sub   (A, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x97 ; x97 ;    ; sub    (A, A)     ;  4 ; undoc ; z80   ]
[ 0x98 ; x98 ;    ; sbc    (A, B)     ;  4 ; undoc ; z80   ]
[ 0x99 ; x99 ;    ; sbc    (A, C)     ;  4 ; undoc ; z80   ]
[ 0x9A ; x9A ;    ; sbc    (A, D)     ;  4 ; undoc ; z80   ]
[ 0x9B ; x9B ;    ; sbc    (A, E)     ;  4 ; undoc ; z80   ]

[ 0x9C ; x9C ;      ; sbc   (A, IYH)    ;  4 ; undoc ; z80   ]
[ 0x9D ; x9D ;      ; sbc   (A, IYL)    ;  4 ; undoc ; z80   ]
[ 0x9E ; x9E ; d    ; sbc   (A, (IY+d)) ; 15 ; doc   ; z80   ]

[ 0x9F ; x9F ;    ; sbc    (A, A)     ;  4 ; undoc ; z80   ]
[ 0xA0 ; xA0 ;    ; and    (B)        ;  4 ; undoc ; z80   ]
[ 0xA1 ; xA1 ;    ; and    (C)        ;  4 ; undoc ; z80   ]
[ 0xA2 ; xA2 ;    ; and    (D)        ;  4 ; undoc ; z80   ]
[ 0xA3 ; xA3 ;    ; and    (E)        ;  4 ; undoc ; z80   ]

[ 0xA4 ; xA4 ;      ; and   (IYH)       ;  4 ; undoc ; z80   ]
[ 0xA5 ; xA5 ;      ; and   (IYL)       ;  4 ; undoc ; z80   ]
[ 0xA6 ; xA6 ; d    ; and   ((IY+d))    ; 15 ; doc   ; z80   ]

[ 0xA7 ; xA7 ;    ; and    (A)        ;  4 ; undoc ; z80   ]
[ 0xA8 ; xA8 ;    ; xor    (B)        ;  4 ; undoc ; z80   ]
[ 0xA9 ; xA9 ;    ; xor    (C)        ;  4 ; undoc ; z80   ]
[ 0xAA ; xAA ;    ; xor    (D)        ;  4 ; undoc ; z80   ]
[ 0xAB ; xAB ;    ; xor    (E)        ;  4 ; undoc ; z80   ]

[ 0xAC ; xAC ;      ; xor   (IYH)       ;  4 ; undoc ; z80   ]
[ 0xAD ; xAD ;      ; xor   (IYL)       ;  4 ; undoc ; z80   ]
[ 0xAE ; xAE ; d    ; xor   ((IY+d))    ; 15 ; doc   ; z80   ]

[ 0xAF ; xAF ;    ; xor    (A)        ;  4 ; undoc ; z80   ]
[ 0xB0 ; xB0 ;    ; or     (B)        ;  4 ; undoc ; z80   ]
[ 0xB1 ; xB1 ;    ; or     (C)        ;  4 ; undoc ; z80   ]
[ 0xB2 ; xB2 ;    ; or     (D)        ;  4 ; undoc ; z80   ]
[ 0xB3 ; xB3 ;    ; or     (E)        ;  4 ; undoc ; z80   ]

[ 0xB4 ; xB4 ;      ; or    (IYH)       ;  4 ; undoc ; z80   ]
[ 0xB5 ; xB5 ;      ; or    (IYL)       ;  4 ; undoc ; z80   ]
[ 0xB6 ; xB6 ; d    ; or    ((IY+d))    ; 15 ; doc   ; z80   ]

[ 0xB7 ; xB7 ;    ; or     (A)        ;  4 ; undoc ; z80   ]
[ 0xB8 ; xB8 ;    ; cp     (B)        ;  4 ; undoc ; z80   ]
[ 0xB9 ; xB9 ;    ; cp     (C)        ;  4 ; undoc ; z80   ]
[ 0xBA ; xBA ;    ; cp     (D)        ;  4 ; undoc ; z80   ]
[ 0xBB ; xBB ;    ; cp     (E)        ;  4 ; undoc ; z80   ]

[ 0xBC ; xBC ;      ; cp    (IYH)       ;  4 ; undoc ; z80   ]
[ 0xBD ; xBD ;      ; cp    (IYL)       ;  4 ; undoc ; z80   ]
[ 0xBE ; xBE ; d    ; cp    ((IY+d))    ; 15 ; doc   ; z80   ]

[ 0xBF ; xBF ;    ; cp     (A)        ;  4 ; undoc ; z80   ]
[ 0xC0 ; xC0 ;    ; retcc  (NZcc)     ;  5 ; undoc ; z80   ]
[ 0xC1 ; xC1 ;    ; pop    (BC)       ; 10 ; undoc ; z80   ]
[ 0xC2 ; xC2 ; nn ; jpcc   (NZcc, nn) ; 10 ; undoc ; z80   ]
[ 0xC3 ; xC3 ; nn ; jp     (nn)       ; 10 ; undoc ; z80   ]
[ 0xC4 ; xC4 ; nn ; callcc (NZcc, nn) ; xx ; undoc ; z80   ]
[ 0xC5 ; xC5 ;    ; push   (BC)       ; 11 ; undoc ; z80   ]
[ 0xC6 ; xC6 ; n  ; add    (A, n)     ;  7 ; undoc ; z80   ]
[ 0xC7 ; xC7 ;    ; rst    (0x00)     ; 11 ; undoc ; z80   ]
[ 0xC8 ; xC8 ;    ; retcc  (Zcc)      ;  5 ; undoc ; z80   ]
[ 0xC9 ; xC9 ;    ; ret    ()         ; 10 ; undoc ; z80   ]
[ 0xCA ; xCA ; nn ; jpcc   (Zcc, nn)  ; 10 ; undoc ; z80   ]

[ 0xCB ; xCB ;      ; fdcb  ()          ;  0 ; doc ; z80   ]

[ 0xCC ; xCC ; nn ; callcc (Zcc, nn)  ; xx ; undoc ; z80   ]
[ 0xCD ; xCD ; nn ; call   (nn)       ; 17 ; undoc ; z80   ]
[ 0xCE ; xCE ; n  ; adc    (A, n)     ;  7 ; undoc ; z80   ]
[ 0xCF ; xCF ;    ; rst    (0x08)     ; 11 ; undoc ; z80   ]
[ 0xD0 ; xD0 ;    ; retcc  (NCcc)     ;  5 ; undoc ; z80   ]
[ 0xD1 ; xD1 ;    ; pop    (DE)       ; 10 ; undoc ; z80   ]
[ 0xD2 ; xD2 ; nn ; jpcc   (NCcc, nn) ; 10 ; undoc ; z80   ]
[ 0xD3 ; xD3 ; n  ; out_n  (n , A)    ; 11 ; undoc ; z80   ]
[ 0xD4 ; xD4 ; nn ; callcc (NCcc, nn) ; xx ; undoc ; z80   ]
[ 0xD5 ; xD5 ;    ; push   (DE)       ; 11 ; undoc ; z80   ]
[ 0xD6 ; xD6 ; n  ; sub    (A, n)     ;  7 ; undoc ; z80   ]
[ 0xD7 ; xD7 ;    ; rst    (0x10)     ; 11 ; undoc ; z80   ]
[ 0xD8 ; xD8 ;    ; retcc  (Ccc)      ;  5 ; undoc ; z80   ]
[ 0xD9 ; xD9 ;    ; exx    ()         ;  4 ; undoc ; z80   ]
[ 0xDA ; xDA ; nn ; jpcc   (Ccc, nn)  ; 10 ; undoc ; z80   ]
[ 0xDB ; xDB ; n  ; in_n   (A, n)     ; 11 ; undoc ; z80   ]
[ 0xDC ; xDC ; nn ; callcc (Ccc, nn)  ; xx ; undoc ; z80   ]
[ 0xDD ; xDD ;    ; dd     ()         ;  4 ; undoc ; z80   ]
[ 0xDE ; xDE ; n  ; sbc    (A, n)     ;  7 ; undoc ; z80   ]
[ 0xDF ; xDF ;    ; rst    (0x18)     ; 11 ; undoc ; z80   ]
[ 0xE0 ; xE0 ;    ; retcc  (POcc)     ;  5 ; undoc ; z80   ]

[ 0xE1 ; xE1 ;      ; pop   (IY)        ; 10 ; doc   ; z80   ]

[ 0xE2 ; xE2 ; nn ; jpcc   (POcc, nn) ; 10 ; undoc ; z80   ]

[ 0xE3 ; xE3 ;      ; ex    ((SP), IY)  ; 19 ; doc   ; z80   ]

[ 0xE4 ; xE4 ; nn ; callcc (POcc, nn) ; xx ; undoc ; z80   ]

[ 0xE5 ; xE5 ;      ; push  (IY)        ; 11 ; doc   ; z80   ]

[ 0xE6 ; xE6 ; n  ; and    (n)        ;  7 ; undoc ; z80   ]
[ 0xE7 ; xE7 ;    ; rst    (0x20)     ; 11 ; undoc ; z80   ]
[ 0xE8 ; xE8 ;    ; retcc  (PEcc)     ;  5 ; undoc ; z80   ]

[ 0xE9 ; xE9 ;      ; jp    (IY)        ;  4 ; doc   ; z80   ]

[ 0xEA ; xEA ; nn ; jpcc   (PEcc, nn) ; 10 ; undoc ; z80   ]
[ 0xEB ; xEB ;    ; ex     (DE, HL)   ;  4 ; undoc ; z80   ]
[ 0xEC ; xEC ; nn ; callcc (PEcc, nn) ; xx ; undoc ; z80   ]
[ 0xED ; xED ;    ; ed     ()         ;  4 ; undoc ; z80   ]
[ 0xEE ; xEE ; n  ; xor    (n)        ;  7 ; undoc ; z80   ]
[ 0xEF ; xEF ;    ; rst    (0x28)     ; 11 ; undoc ; z80   ]
[ 0xF0 ; xF0 ;    ; retcc  (Pcc)      ;  5 ; undoc ; z80   ]
[ 0xF1 ; xF1 ;    ; pop    (AF)       ; 10 ; undoc ; z80   ]
[ 0xF2 ; xF2 ; nn ; jpcc   (Pcc, nn)  ; 10 ; undoc ; z80   ]
[ 0xF3 ; xF3 ;    ; di     ()         ;  4 ; undoc ; z80   ]
[ 0xF4 ; xF4 ; nn ; callcc (Pcc, nn)  ; xx ; undoc ; z80   ]
[ 0xF5 ; xF5 ;    ; push   (AF)       ; 11 ; undoc ; z80   ]
[ 0xF6 ; xF6 ; n  ; or     (n)        ;  7 ; undoc ; z80   ]
[ 0xF7 ; xF7 ;    ; rst    (0x30)     ; 11 ; undoc ; z80   ]
[ 0xF8 ; xF8 ;    ; retcc  (Mcc)      ;  5 ; undoc ; z80   ]

[ 0xF9 ; xF9 ;      ; ld16  (SP, IY)    ;  6 ; doc   ; z80   ]

[ 0xFA ; xFA ; nn ; jpcc   (Mcc, nn)  ; 10 ; undoc ; z80   ]
[ 0xFB ; xFB ;    ; ei     ()         ;  4 ; undoc ; z80   ]
[ 0xFC ; xFC ; nn ; callcc (Mcc, nn)  ; xx ; undoc ; z80   ]
[ 0xFD ; xFD ;    ; fd     ()         ;  4 ; undoc ; z80   ]
[ 0xFE ; xFE ; n  ; cp     (n)        ;  7 ; undoc ; z80   ]
[ 0xFF ; xFF ;    ; rst    (0x38)     ; 11 ; undoc ; z80   ]
)
        }
    };
    (@ $mac: ident ($($inst: tt)*)) => {
        $(
            $mac!{$inst}
         )*
    };
}
