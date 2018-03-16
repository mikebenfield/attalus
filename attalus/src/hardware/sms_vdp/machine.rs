// Copyright 2017 Michael Benfield <mike.benfield@gmail.com>
// This file is part of Attalus. You may distribute and/or modify Attalus under
// the terms of the GNU General Public License as published by the Free Sofware
// Foundation, either version 3 of the license or (at your option) any later
// version. You should have received a copy of the GNU General Public License
// along with Attalus. If not, see <http://www.gnu.org/licenses/>.

/// The actual video functionality of the VDP.

use std;

use failure::Error;

use super::{higher, FRAME_INTERRUPT_FLAG};

pub type Result<T> = std::result::Result<T, Error>;

/// The video functionality of the VDP.
pub trait T: higher::T {
    /// Called to draw each line and update the VDP's registers appropriately.
    fn draw_line(&mut self) -> Result<()>;
}

/// Update the VDP after drawing a line.
///
/// This function is provided for convenience so that implementations of
/// `T::draw_line` can call it. It does the following:
/// * set the frame interrupt flag, if we're on the line past the last active one
/// * if we're in the active region or one line past it, decrement the line
///   counter, and we wrap to 0xFF, set the line interrupt flag and reload the
///   line counter from register 10.
/// * if we're more than one line past the active region, reload the line counter
///   from register 10.
/// * increment the v counter, wrapping at `total_lines()`.
/// * increase the number of cycles by 342
pub fn finish_line<S: higher::T>(vdp: &mut S) -> Result<()> {
    let v = vdp.v();

    if v == vdp.active_lines() {
        // YYY - it's not completely clear to me whether this is the right
        // line on which to trigger a frame interrupt.
        let flags = vdp.status_flags();
        vdp.set_status_flags(flags | FRAME_INTERRUPT_FLAG);
    }

    if v <= vdp.active_lines() {
        let line_counter = vdp.line_counter();
        vdp.set_line_counter(line_counter.wrapping_sub(1));
        if vdp.line_counter() == 0xFF {
            let reg_line_counter = vdp.reg_line_counter();
            vdp.set_line_counter(reg_line_counter);
            vdp.set_line_interrupt_pending(true);
        }
    } else {
        let reg_line_counter = vdp.reg_line_counter();
        vdp.set_line_counter(reg_line_counter);
    }

    let new_v = (v + 1) % vdp.total_lines();

    vdp.set_v(new_v);

    if new_v == 0 {
        let reg9 = unsafe { vdp.register_unchecked(9) };
        vdp.set_y_scroll(reg9);
    }

    let cycles = vdp.cycles();
    vdp.set_cycles(cycles + 342);

    return Ok(());
}

pub trait Impler<S>
where
    S: ?Sized,
{
    fn draw_line(&mut S) -> Result<()>;
}

pub trait Impl {
    type Impler: Impler<Self>;
}

impl<S> T for S
where
    S: Impl + higher::T,
{
    #[inline]
    fn draw_line(&mut self) -> Result<()> {
        <S::Impler as Impler<Self>>::draw_line(self)
    }
}

/// The easiest way to implement `machine::T` is to use the type in this
/// module.
pub mod simple {
    use host_multimedia::{SimpleColor, SimpleGraphics};

    use utilities;

    use super::Result;

    use super::higher;

    /// Easiest way to implement `machine::T`.
    ///
    /// If your type implements `SimpleGraphics` and `higher::T`, you can just
    /// implement `machine::Impl` with `type Impler = machine::simple::T`.
    pub struct T;

    #[inline]
    pub fn vdp_color_to_simple_color(color: u8) -> SimpleColor {
        let blue = (0x30 & color) << 2;
        let green = (0x0C & color) << 4;
        let red = (0x03 & color) << 6;
        SimpleColor { red, green, blue }
    }

    fn real_finish_line<S: SimpleGraphics + higher::T>(vdp: &mut S) -> Result<()> {
        super::finish_line(vdp).and_then(|()| {
            if vdp.v() == 0 {
                vdp.render()
            } else {
                Ok(())
            }
        })
    }

    impl<S> super::Impler<S> for T
    where
        S: SimpleGraphics + higher::T,
    {
        fn draw_line(s: &mut S) -> Result<()> {
            use super::super::Resolution::*;

            let v = s.v();

            if v >= s.active_lines() {
                return real_finish_line(s);
            }

            let active_lines = s.active_lines() as u32;
            s.set_resolution(256, active_lines)?;

            if !s.display_visible() {
                for x in 0..256 {
                    s.paint(
                        x,
                        v as u32,
                        SimpleColor {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                    );
                }
                return real_finish_line(s);
            }

            let mut line_buffer = [0x80u8; 256];

            let v = s.v();

            // draw sprites
            let sprite_height = if s.tall_sprites() { 16 } else { 8 };
            let sprites_rendered = 0u8;
            for i in 0..64 {
                let sprite_y = unsafe { s.sprite_y(i) } as u16;
                if sprite_y == 0xD1 && higher::T::resolution(s) == Low {
                    break;
                }
                let sprite_line = v.wrapping_sub(sprite_y);
                if sprite_line >= sprite_height {
                    continue;
                }
                if sprites_rendered == 8 {
                    s.trigger_sprite_overflow();
                    break;
                }

                let pattern_addr = unsafe { s.sprite_pattern_address(i) };

                let palette_indices: [u8; 8] =
                    unsafe { s.pattern_address_to_palette_indices(pattern_addr, sprite_line) };
                let sprite_x = unsafe { s.sprite_x(i) } as usize;
                let shift_x = if s.shift_sprites() { 8 } else { 0 };
                for i in 0..8 {
                    let render_x = sprite_x.wrapping_add(i).wrapping_sub(shift_x);
                    if render_x > 255 {
                        break;
                    }
                    if line_buffer[render_x] != 0x80 {
                        s.trigger_sprite_collision();
                        continue;
                    }
                    if palette_indices[i] != 0 {
                        line_buffer[render_x] = s.cram(palette_indices[i] as u16 + 16) as u8;
                    }
                }
            }

            // draw tiles
            let scrolled_v = (v + s.y_scroll() as u16) % if higher::T::resolution(s) == Low {
                28 * 8
            } else {
                32 * 8
            };
            let tile_index_base = (scrolled_v / 8) * 32;
            let tile_line = scrolled_v % 8;
            for tile in 0..32u16 {
                let current_tile_address = s.name_table_address() + 2 * (tile + tile_index_base);
                let low_byte = s.vram(current_tile_address);
                let high_byte = s.vram(current_tile_address.wrapping_add(1));
                let vert_flip = 4 & high_byte != 0;
                let horiz_flip = 2 & high_byte != 0;
                let priority = 0x10 & high_byte != 0;
                let palette = ((high_byte & 8) << 1) as u16;
                let pattern_index = utilities::to16(low_byte, high_byte & 1);
                let tile_line_really = if vert_flip { 7 - tile_line } else { tile_line };
                let palette_indices: [u8; 8] = unsafe {
                    s.pattern_address_to_palette_indices(pattern_index * 32, tile_line_really)
                };
                for j in 0..8 {
                    let x = if horiz_flip {
                        tile as usize * 8 + (7 - j)
                    } else {
                        tile as usize * 8 + j
                    } as u8;

                    let scrolled_x = x.wrapping_add(s.x_scroll()) as usize;
                    if line_buffer[scrolled_x] == 0x80
                        || (priority && palette_indices[j] as usize > 0)
                    {
                        line_buffer[scrolled_x] = s.cram(palette_indices[j] as u16 + palette) as u8;
                    }
                }
            }

            if s.left_column_blank() {
                let color = s.cram(16 + s.backdrop_color_index() as u16);
                for i in 0..8 {
                    line_buffer[i] = color as u8;
                }
            }

            for x in 0..256 {
                let color = vdp_color_to_simple_color(line_buffer[x as usize]);
                s.paint(x, v as u32, color);
            }

            return real_finish_line(s);
        }
    }
}
