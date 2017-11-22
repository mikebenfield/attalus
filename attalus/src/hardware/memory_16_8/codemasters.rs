// Copyright 2017 Michael Benfield <mike.benfield@gmail.com>
// This file is part of Attalus. You may distribute and/or modify Attalus under
// the terms of the GNU General Public License as published by the Free Sofware
// Foundation, either version 3 of the license or (at your option) any later
// version. You should have received a copy of the GNU General Public License
// along with Attalus. If not, see <http://www.gnu.org/licenses/>.

use ::errors::*;
use ::has::Has;
use ::memo::{Inbox, Outbox};

use super::*;
use super::sega::{Memo, MasterSystemMemory};

/// The Codemasters memory map, used in Sega Master System games created by
/// British game developer Codemasters.
#[derive(Clone)]
pub struct Component {
    // As in the `sega::Component`, memory is a sequence of 8 KiB
    // implementation-pages. The first implementation-page corresponds to the
    // console RAM, and then pairs of pages correspond to 16 KiB
    // codemasters-pages of cartridge ROM. Finally, there *may* be a final 8 KiB
    // page of cartridge RAM.
    memory: Vec<[u8; 0x2000]>,

    cartridge_ram_allocated: bool,

    // The `pages` field works identically to the corresponding field in
    // `sega::Component`.
    pages: [u16; 8],

    reg_0000: u8,
    reg_4000: u8,
    reg_8000: u8,

    slot_writable: u8,

    id: u32,
}

impl Outbox for Component {
    type Memo = Memo;

    fn id(&self) -> u32 {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

fn write_check_register<T>(
    t: &mut T,
    logical_address: u16,
    value: u8,
) where
    T: Inbox<Memo> + Has<Component>,
{
    macro_rules! receive {
        ($x: expr) => {
            {
                let id = t.get().id();
                let __y = $x;
                t.receive(id, __y);
            }
        }
    }

    fn swap_slot<T>(t: &mut T, sega_slot: usize, value: u8)
    where
        T: Inbox<Memo> + Has<Component>,
    {
        macro_rules! receive {
            ($x: expr) => {
                {
                    let id = t.get().id();
                    let __y = $x;
                    t.receive(id, __y);
                }
            }
        }

        debug_assert!(sega_slot <= 3);
        let (upper_bit_set, lower_bits) = ((0x80 & value) != 0, 0x7F & value);
        let impl_slot0 = 2 * sega_slot;
        let impl_slot1 = impl_slot0 + 1;
        let rom_impl_page_count = if t.get().cartridge_ram_allocated {
            t.get().memory.len() - 2
        } else {
            t.get().memory.len() - 1
        };
        let rom_sega_page_count = (rom_impl_page_count / 2) as u8;
        let sega_page = if rom_sega_page_count == 0 {
            0u8
        } else {
            lower_bits % rom_sega_page_count
        };
        receive! (
            Memo::MapRom {
                slot: sega_slot as u8,
                page: sega_page,
            }
        );
        let impl_page = (sega_page as u16) * 2 + 1;
        if upper_bit_set {
            // RAM goes into the second implementation-slot
            if !t.get().cartridge_ram_allocated {
                receive!(Memo::AllocateFirstPage);
                t.get_mut().memory.push([0; 0x2000]);
                t.get_mut().memory.shrink_to_fit();
            }
            receive!(
                Memo::MapCartridgeRam {
                    slot: sega_slot as u8,
                    page: sega_page,
                }
            );
            let cmm = t.get_mut();
            cmm.pages[impl_slot1] = (cmm.memory.len() - 1) as u16;
            cmm.slot_writable |= 1 << impl_slot1;
        } else {
            let cmm = t.get_mut();
            cmm.pages[impl_slot1] = impl_page + 1;
            cmm.slot_writable &= !(1 << impl_slot1);
        }
        t.get_mut().pages[impl_slot0] = impl_page;
        // even impl_slots will never be marked as writable anyway
    }

    let slot = match logical_address {
        0 => {
            receive!(
                Memo::RegisterWrite {
                    register: 0,
                    value: value,
                }
            );
            t.get_mut().reg_0000 = value;
            0
        },
        0x4000 => {
            receive!(
                Memo::RegisterWrite {
                    register: 0x4000,
                    value: value,
                }
            );
            t.get_mut().reg_4000 = value;
            1
        },
        0x8000 => {
            receive!(
                Memo::RegisterWrite {
                    register: 0x8000,
                    value: value,
                }
            );
            t.get_mut().reg_8000 = value;
            2
        },
        _ => return,
    };

    swap_slot(t, slot as usize, value);
}

impl<T> ComponentOf<T> for Component
where
    T: Inbox<Memo> + Has<Component>
{
    fn read(t: &mut T, logical_address: u16) -> u8 {
        let physical_address = logical_address & 0x1FFF; // low order 13 bits
        let impl_slot = (logical_address & 0xE000) >> 13; // high order 3 bits
        let impl_page = t.get().pages[impl_slot as usize];
        let result = t.get().memory[impl_page as usize][physical_address as usize];
        result
    }

    fn write(t: &mut T, logical_address: u16, value: u8) {
        write_check_register(t, logical_address, value);
        if logical_address == 0 || logical_address == 0x4000 || logical_address == 0x8000 {
            return;
        }
        let physical_address = logical_address & 0x1FFF; // low order 13 bits
        let impl_slot = (logical_address & 0xE000) >> 13; // high order 3 bits
        if t.get().slot_writable & (1 << impl_slot) != 0 {
            let impl_page = t.get().pages[impl_slot as usize];
            t.get_mut().memory[impl_page as usize][physical_address as usize] = value;
        } else {
        }
    }
}

impl MasterSystemMemory for Component {
    fn new(rom: &[u8]) -> Result<Self> {
        if rom.len() % 0x2000 != 0 || rom.len() == 0 {
            bail! {
                ErrorKind::Rom(
                    format!("Invalid Sega Master System ROM size 0x{:0>6X} (should be a positive multiple of 0x2000)", rom.len())
                )
            }
        }

        let rom_impl_page_count = rom.len() / 0x2000;

        let mut memory = Vec::with_capacity(1 + rom_impl_page_count);

        // push the system RAM
        memory.push([0; 0x2000]);

        // push the ROM
        for i in 0..rom_impl_page_count {
            let mut impl_page = [0u8; 0x2000];
            impl_page.copy_from_slice(&rom[0x2000 * i..0x2000 * (i + 1)]);
            memory.push(impl_page);
        }

        Ok(Component {
            memory: memory,
            cartridge_ram_allocated: false,
            // according to smspower.org, the mapper is initialized with
            // sega_slot 0 mapped to sega_page 0, slot 1 mapped to 1, and
            // slot 2 mapped to 0
            pages: [1, 2, 3, 4, 1, 2, 0, 0],
            reg_0000: 0,
            reg_4000: 1,
            reg_8000: 0,
            // only the system RAM is writable
            slot_writable: 0b11000000,
            id: 0,
        })
    }
}

// mod tests {
//     use super::*;

//     #[allow(dead_code)]
//     fn build_mmap() -> Component {
//         let mut rom = [0u8; 0x10000]; // 64 KiB (8 8KiB impl-pages or 4 16KiB sega-pages)
//         rom[0x2000] = 1;
//         rom[0x4000] = 2;
//         rom[0x6000] = 3;
//         rom[0x8000] = 4;
//         rom[0xA000] = 5;
//         rom[0xC000] = 6;
//         rom[0xE000] = 7;
//         Component::new(&rom).unwrap()
//     }

//     #[test]
//     fn read() {
//         let cmm = &mut build_mmap();

//         // read impl-slot 0
//         assert!(cmm.read(0) == 0);

//         // read impl-slot 1
//         assert!(cmm.read(0x2000) == 1);

//         // read impl-slot 2
//         assert!(cmm.read(0x4000) == 2);

//         // read impl-slot 3
//         assert!(cmm.read(0x6000) == 3);

//         // read impl-slot 4
//         assert!(cmm.read(0x8000) == 0);

//         // read impl-slot 5
//         assert!(cmm.read(0xA000) == 1);

//         // read impl-slot 6 (should be system memory)
//         assert!(cmm.read(0xC000) == 0);

//         // read impl-slot 7 (should be system memory)
//         assert!(cmm.read(0xE000) == 0);
//     }

//     #[test]
//     fn slot0() {
//         let smm = &mut build_mmap();

//         smm.write(0, 3); // sega-slot 0 should now map to sega-page 3
//         assert!(smm.read(0) == 6);
//         assert!(smm.read(0x2000) == 7);

//         smm.write(0, 0); // sega-slot 0 should now map to sega-page 0
//         assert!(smm.read(0) == 0);
//         assert!(smm.read(0x2000) == 1);
//     }

//     #[test]
//     fn slot1() {
//         let smm = &mut build_mmap();

//         smm.write(0x4000, 3); // sega-slot 1 should now map to sega-page 3
//         assert!(smm.read(0x4000) == 6);
//         assert!(smm.read(0x6000) == 7);

//         smm.write(0x4000, 0); // sega-slot 1 should now map to sega-page 0
//         assert!(smm.read(0x4000) == 0);
//         assert!(smm.read(0x6000) == 1);
//     }

//     #[test]
//     fn slot2() {
//         let smm = &mut build_mmap();

//         smm.write(0x8000, 3); // sega-slot 2 should now map to sega-page 3
//         assert!(smm.read(0x8000) == 6);
//         assert!(smm.read(0xA000) == 7);

//         smm.write(0x8000, 0); // sega-slot 2 should now map to sega-page 0
//         assert!(smm.read(0x8000) == 0);
//         assert!(smm.read(0xA000) == 1);
//     }
// }