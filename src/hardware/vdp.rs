
use std;
use std::error::Error;

use ::bits::*;
use ::log;

use super::irq::Irq;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ScreenError(String);

impl<T: Error> From<T> for ScreenError {
    fn from(t: T) -> ScreenError {
        ScreenError(t.description().to_string())
    }
}

pub trait Screen {
    fn paint(&mut self, x: usize, y: usize, color: u8);
    fn render(&mut self) -> Result<(), ScreenError>;
    fn set_resolution(&mut self, width: usize, height: usize) -> Result<(), ScreenError>;
}

pub struct NoScreen;

impl Screen for NoScreen {
    fn paint(&mut self, _: usize, _: usize, _: u8) {}
    fn render(&mut self) -> Result<(), ScreenError> { Ok(()) }
    fn set_resolution(&mut self, _: usize, _: usize) -> Result<(), ScreenError> { Ok(()) }
}

#[derive(Copy)]
pub struct Vdp {
    status_flags: u8,
    irq_counter: u8,
    h: u16,
    v0: u16,
    read: u8,
    code_address: u16,
    registers: [u8; 16],
    cram: [u8; 32],
    vram: [u8; 0x4000],
}

impl std::fmt::Debug for Vdp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Vdp \
            {{ \n\
                status_flags: {:?}, \n\
                irq_counter: {:?}, h: {:?}, \n\
                v0: {:?}, read: {:?}, code_address: {:?}, \n\
                registers: {:?}, \n\
                cram: {:?}, \n\
                vram: {:?} (...) \n
            }}",
            self.status_flags,
            self.irq_counter,
            self.h,
            self.v0,
            self.read,
            self.code_address,
            self.registers,
            self.cram,
            &self.vram[0..32]
        )
    }
}

impl Default for Vdp {
    fn default() -> Vdp {
        Vdp {
            status_flags: 0,
            irq_counter: 0,
            h: 0,
            v0: 0,
            read: 0,
            code_address: 0,
            registers: [0; 16],
            cram: [0; 32],
            vram: [0; 0x4000],
        }
    }
}

impl Clone for Vdp {
    fn clone(&self) -> Vdp {
        *self
    }
}

impl Vdp {
    pub fn write_control(&mut self, x: u8) {
        log_minor!("Vdp: write control: {:0>2X}", x);

        if self.status_flags & (1 << CFLAG) != 0 {
            self.code_address = (x as u16) << 8
                | (self.code_address & 0xFF);
            let code = self.code_address & 0xC000;
            if code == 0 {
                // code value 0: read vram
                let addr = self.code_address & 0x3FFF;
                self.read = self.vram[addr as usize];
                if addr == 0x3FFF { // wrap
                    self.code_address &= 0xC000
                } else {
                    self.code_address = code | (addr + 1);
                }
            } else if code == 0x8000 {
                // code value 2: register write
                let reg_number = (self.code_address & 0xF00) >> 8;
                let val = self.code_address & 0xFF;
                self.registers[reg_number as usize] = val as u8;
            }
        } else {
            self.code_address = (x as u16)
                | (self.code_address & 0xFF00);
        }
        self.status_flags ^= 1 << CFLAG;
    }

    pub fn read_control(&mut self) -> u8 {
        let result = self.status_flags;
        self.status_flags = 0;
        log_minor!("Vdp: read control: {:0>2X}", result);
        result
    }

    pub fn write_data(&mut self, x: u8) {
        log_minor!("Vdp: write data: {:0>2X}", x);

        clear_bit(&mut self.status_flags, CFLAG);
        let code = self.code_address & 0xC000;
        if code == 0xC000 {
            // CRAM
            let addr = self.code_address & 0x1F;
            self.cram[addr as usize] = x;
        } else {
            // VRAM
            let addr = self.code_address & 0x3FFF;
            self.vram[addr as usize] = x;
        }
        let addr = self.code_address & 0x3FFF;
        if addr == 0x3FFF {
            self.code_address = code;
        } else {
            self.code_address = code | (addr + 1);
        }
    }

    pub fn read_data(&mut self) -> u8 {
        clear_bit(&mut self.status_flags, CFLAG);
        let result = self.read;
        let addr = self.code_address & 0x3FFF;
        let code = self.code_address & 0xC000;
        if addr == 0x3FFF {
            self.code_address = code;
        } else {
            self.code_address = code | (addr + 1);
        }
        self.read = self.vram[addr as usize];

        log_minor!("Vdp: read data: {:0>4X}", result);

        result
    }

    pub fn read_v(&self) -> u8 {
        let v0 = self.v0;
        if v0 <= 0xDA {
            v0 as u8
        } else {
            (v0 - 6) as u8
        }
    }

    pub fn read_h(&self) -> u8 {
        let h = self.h;
        (h >> 1) as u8
    }
}

impl Irq for Vdp {
    fn requesting_mi(&self) -> bool {
        let frame_irq =
            (self.registers[1] & 0x20) != 0 && self.status_flags & (1 << INT) != 0;
        let line_irq =
            (self.registers[0] & 0x10) != 0 && self.status_flags & (1 << LINT) != 0;
        frame_irq || line_irq
    }

    fn requesting_nmi(&self) -> bool {
        false
    }
}

// bits in status_flags
// note that only INT, OVR, and COL are part of the VDP hardware. The rest are
// garbage bits on the SMS VDP. We're using them for other purposes.
const INT: u8 = 7; // Frame interrupt pending
const OVR: u8 = 6; // Sprite overflow
const COL: u8 = 5; // Sprite collision
const LINT: u8 = 4; // Line interrupt pending
const CFLAG: u8 = 3; // Control flag - set after first write to control port

// pub fn draw_frame<C: Canvas, V: Vdp>(
//     v: &mut V,
//     canvas: &mut C,
// ) -> Result<u64, CanvasError> {
//     log_minor!(v, "Vdp: drawing frame");
//     v.request_maskable_interrupt();
//     let vdp = v.get_mut_vdp_hardware();
//     let nt_address = ((vdp.registers[2] & 0x0E) as usize) << 10;
//     let sat_address = ((vdp.registers[5] & 0x7E) as usize) << 7;
//     let overscan_color: u16 = (vdp.registers[7] & 0x0F) as u16;
//     let x_starting_column: u16 = 32 - ((vdp.registers[8] & 0xF8) as u16 >> 3);
//     let x_scroll: u16 = (vdp.registers[8] & 0x07) as u16;
//     let y_starting_column: u16 = (vdp.registers[9] & 0xF8) as u16 >> 3;
//     let y_scroll: u16 = (vdp.registers[9] & 0x07) as u16;

//     vdp.v0 = 0;

//     let bit8 = ((vdp.registers[6] & 0x04) as usize) << 6;
//     for sprite_index in 0..64 {
//         let y = vdp.vram[sat_address + sprite_index] as usize + 1;
//         let x = (vdp.vram[sat_address + 0x80 + 2*sprite_index] as usize) -
//             if 0 != vdp.registers[0] & 0x08 {
//                 8
//             } else {
//                 0
//             };
//         let n = vdp.vram[sat_address + 0x81 + 2*sprite_index] as usize;
//         let pattern_address = 32*n | bit8;
//         for pixel_y in 0..8 {
//             let pattern_byte0 = vdp.vram[pattern_address + 8*pixel_y];
//             let pattern_byte1 = vdp.vram[pattern_address + 8*pixel_y + 1];
//             let pattern_byte2 = vdp.vram[pattern_address + 8*pixel_y + 2];
//             let pattern_byte3 = vdp.vram[pattern_address + 8*pixel_y + 3];
//             for pixel_x in 0..8 {
//                 let mut palette_index = 0u8;

//                 assign_bit(&mut palette_index, 0, pattern_byte0, 7 - pixel_x);
//                 assign_bit(&mut palette_index, 1, pattern_byte1, 7 - pixel_x);
//                 assign_bit(&mut palette_index, 2, pattern_byte2, 7 - pixel_x);
//                 assign_bit(&mut palette_index, 3, pattern_byte3, 7 - pixel_x);

//                 if palette_index == 0 {
//                     continue;
//                 }

//                 let color = vdp.cram[0x10 + palette_index as usize];
//                 canvas.paint(x + pixel_x as usize, y + pixel_y as usize, color);
//             }
//         }
//     }
//     vdp.status_flags |= 1 << INT;
//     Ok(684*262)
// }

// #[allow(unused_variables)]
// pub fn draw_line<C: Canvas, V: Vdp>(
//     v: &mut V,
//     canvas: &mut C,
// ) -> Result<u64, CanvasError> {
//     let line = v.get_vdp_hardware().v0;
//     log_minor!(v, "Vdp: draw line {}", line);
//     v.get_mut_vdp_hardware().registers[2] = 0xFF; // XXX
//     // let ntaddr = ((v.get_vdp_hardware().registers[2] & 0x0E) as usize) << 10;
//     // log_major!(v, "Vdp: ntaddr {}", ntaddr);
//     // let sat_address = ((v.get_vdp_hardware().registers[5] & 0x7E) as usize) << 7;
//     // log_major!(v, "Vdp: sataddr {}", sat_address);

//     fn draw_line0<C: Canvas>(vdp: &mut VdpHardware, canvas: &mut C) {
//         let nt_address = ((vdp.registers[2] & 0x0E) as usize) << 10;
//         let sat_address = ((vdp.registers[5] & 0x7E) as usize) << 7;
//         let overscan_color: u16 = (vdp.registers[7] & 0x0F) as u16;
//         let x_starting_column: u16 = 32 - ((vdp.registers[8] & 0xF8) as u16 >> 3);
//         let x_scroll: u16 = (vdp.registers[8] & 0x07) as u16;
//         let y_starting_column: u16 = (vdp.registers[9] & 0xF8) as u16 >> 3;
//         let y_scroll: u16 = (vdp.registers[9] & 0x07) as u16;

//         let line = vdp.v0 as usize;

//         if vdp.v0 <= 192 { // yes, 192 (one past active display region) is right
//             if vdp.irq_counter == 0 {
//                 // line interrupt
//                 vdp.status_flags |= 1 << LINT;
//                 vdp.irq_counter = vdp.registers[10];
//             } else {
//                 vdp.irq_counter -= 1;
//             }
//         } else {
//             vdp.irq_counter = vdp.registers[10];
//         }
//         if vdp.v0 == 0xC1 {
//             // frame interrupt
//             vdp.status_flags |= 1 << INT;
//         }
//         vdp.v0 += 1;
//         if vdp.v0 == 262 {
//             vdp.v0 = 0;
//         }

//         if line >= 192 {
//             // we are out of the active display region
//             return;
//         }

//         let mut line_colors = [0x80u8; 256];

//         //// first, draw sprites to line_colors
//         let mut sprites_drawn = 0;
//         let bit8 = ((vdp.registers[6] & 0x04) as usize) << 6;
//         for sprite_index in 0..64 {
//             let y = vdp.vram[sat_address + sprite_index] as usize + 1;
//             let x = (vdp.vram[sat_address + 0x80 + 2*sprite_index] as isize) -
//                 if 0 != vdp.registers[0] & 0x08 {
//                     8
//                 } else {
//                     0
//                 };

//             let n = vdp.vram[sat_address + 0x81 + 2*sprite_index] as usize;
//             let pattern_index = n | bit8;
//             if y == 0xD1 {
//                 // such a y coordinate has a special meaning in 192-line mode: don't
//                 // render any more sprites this line
//                 break;
//             }
//             if line < y || line >= y + 8 {
//                 continue;
//             }
//             if sprites_drawn == 8 {
//                 // only draw 8 sprites per line, and if more are scheduled to be
//                 // drawn, set the overflow flag
//                 vdp.status_flags |= 1 << OVR;
//                 break;
//             }

//             sprites_drawn += 1;

//             // the index of the line within the pattern we need to draw
//             let sprite_line = line - y;

//             // we have 8 pixels, each of which needs a byte of color
//             let pattern_byte0 = vdp.vram[32*pattern_index + sprite_line];
//             let pattern_byte1 = vdp.vram[32*pattern_index + sprite_line + 1];
//             let pattern_byte2 = vdp.vram[32*pattern_index + sprite_line + 2];
//             let pattern_byte3 = vdp.vram[32*pattern_index + sprite_line + 3];
//             for pixel in 0..8u8 {
//                 let mut palette_index = 0u8;
//                 // pixel 0 will be the leftmost pixel to draw... but that is
//                 // found in the most significant bit of each byte
//                 assign_bit(&mut palette_index, 0, pattern_byte0, 7 - pixel);
//                 assign_bit(&mut palette_index, 1, pattern_byte1, 7 - pixel);
//                 assign_bit(&mut palette_index, 2, pattern_byte2, 7 - pixel);
//                 assign_bit(&mut palette_index, 3, pattern_byte3, 7 - pixel);

//                 if palette_index == 0 {
//                     // for sprites, this means transparency
//                     continue;
//                 }

//                 // sprites use the second palette
//                 let color = vdp.cram[0x10 + palette_index as usize];

//                 // the x coordinate of the canvas where this pixel will be drawn
//                 let x0 = x + 7 - (pixel as isize);

//                 if x0 < 0 || x0 > 255 {
//                     // I *think* pixels outside this range don't count for sprite
//                     // collision
//                     continue;
//                 }

//                 if line_colors[x0 as usize] != 0x80 {
//                     // sprite collision
//                     // also, the earlier sprite gets priority
//                     set_bit(&mut vdp.status_flags, COL);
//                 } else {
//                     line_colors[x0 as usize] = color;
//                 }
//             }
//         }

//         // 256 width, 192 height
//         // 28 rows of tiles
//         // Now draw background tiles - no scrolling yet
//         let tile_row = line / 28;
//         let tile_line = line % 28;
//         for tile_index in 0..32 {
//             let tile_address = nt_address + tile_index * tile_row * 2;
//             let low_byte = vdp.vram[tile_address];
//             let high_byte = vdp.vram[tile_address + 1];
//             let pattern_index = (low_byte as usize) | ((high_byte & 1) as usize) << 8;
//             let horiz_flip = 0 != high_byte & 0x02;
//             let vert_flip = 0 != high_byte & 0x04;
//             let palette_index0 = (high_byte & 0x08) << 1;
//             let priority = 0 != high_byte & 0x10;

//             let pattern_byte0 = vdp.vram[32*pattern_index + tile_line];
//             let pattern_byte1 = vdp.vram[32*pattern_index + tile_line + 1];
//             let pattern_byte2 = vdp.vram[32*pattern_index + tile_line + 2];
//             let pattern_byte3 = vdp.vram[32*pattern_index + tile_line + 3];

//             for pixel in 0..8u8 {
//                 let mut palette_index = palette_index0;
//                 // pixel 0 will be the leftmost pixel to draw... but that is
//                 // found in the most significant bit of each byte
//                 assign_bit(&mut palette_index, 0, pattern_byte0, 7 - pixel);
//                 assign_bit(&mut palette_index, 1, pattern_byte1, 7 - pixel);
//                 assign_bit(&mut palette_index, 2, pattern_byte2, 7 - pixel);
//                 assign_bit(&mut palette_index, 3, pattern_byte3, 7 - pixel);

//                 let color = vdp.cram[palette_index as usize];

//                 // the x coordinate of the canvas where this pixel will be drawn
//                 // let x0 = (tile_index*8 + 7) as isize - (pixel as isize);
//                 let x0 = tile_index*8 + pixel as usize;

//                 if priority || line_colors[x0 as usize] == 0x80 {
//                     line_colors[x0 as usize] = color;
//                 }
//             }

//             // Now we can actually draw
//             for i in 0..256usize {
//                 canvas.paint(i, line, line_colors[i]);
//             }
//         }
//     }

//     draw_line0(v.get_mut_vdp_hardware(), canvas);

//     if line == 261 {
//         log_major!(v, "Vdp: rendering frame");
//         canvas.render()?;
//     }

//     if v.is_requesting_interrupt() {
//         v.request_maskable_interrupt();
//     }

//     Ok(684)
// }
