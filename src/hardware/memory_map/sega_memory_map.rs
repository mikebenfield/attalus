// Copyright 2017 Michael Benfield <mike.benfield@gmail.com>
// This file is part of Attalus. You may distribute and/or modify Attalus under
// the terms of the GNU General Public License as published by the Free Sofware
// Foundation, either version 3 of the license or (at your option) any later
// version. You should have received a copy of the GNU General Public License
// along with Attalus. If not, see <http://www.gnu.org/licenses/>.

use ::message::{Receiver, Sender};
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum RamPagesAllocated {
    Zero, One, Two
}

use self::RamPagesAllocated::*;

pub struct SegaMemoryMap {
    // memory is a sequence of 8 KiB implementation-pages. The first
    // implementation-page corresponds to the 8 KiB of system memory.
    // Then successive pairs of implementation-pages correspond to
    // 16 KiB sega-pages of cartridge ROM. Then finally, depending on
    // the value of the `ram_pages` field, there may be zero, two, or
    // four implementation pages corresponding to the zero, one, or two
    // sega-pages of cartridge RAM. If there are two sega-pages
    // of cartridge RAM, the first page is at the very end of the sequence,
    // and the second page comes just before it.
    //
    // We don't allocate the cartridge RAM until/unless we need it, which we
    // detect by observing writes to the memory control register at logical
    // address 0xFFFC.
    //
    // Justification: we could store system RAM, cartridge RAM, and cartridge
    // ROM in separate fields. But for read and write access to be efficient, we
    // would like fields of `SegaMemoryMap` to index, for each logical slot of
    // memory, directly into the physical memory required. This can't be safely
    // done with references in Rust, so we put all pages of memory into a
    // slice and let our slot indices be slice indices.
    //
    // We use 8 KiB implementation-pages, although sega-pages are 16 KiB,
    // because the system RAM is only 8 KiB, and the last two logical memory
    // slots are only 8 KiB, so this way we don't have to make special cases.
    memory: Vec<[u8; 0x2000]>,

    // How many sega-pages of cartridge RAM have we allocated?
    ram_pages_allocated: RamPagesAllocated,

    // The special registers that control the memory map. When writing to
    // `reg_fffc`, we set `reg_fffc` the actual value written. For the others,
    // which are selectors for the ROM slots, we instead set the register to the
    // sega-page selected, which may be a modulus of the actual value written.
    // (In actual hardware these registers are not readable anyway.)
    reg_fffc: u8,
    reg_fffd: u8,
    reg_fffe: u8,
    reg_ffff: u8,

    // the indices of this array correspond to implementation-slots (8 KiB pages of
    // logical memory), and the members of the array indicate which
    // implementation-page (8 KiB pages of physical memory) that slot should be
    // mapped to, as an index into the `memory` field. Pairs of
    // implementation-slots correspond to 16 KiB sega-slots, except that the
    // last portion of logical memory is divided into two 8 KiB slots, which
    // normally are both mapped to System RAM.
    // These need to be `u16` since conceivably there could be up to 256 sega-pages
    // of cartridge ROM, and we would then need more implementation-pages than that.
    pages: [u16; 8],

    // bitmask, with each bit indicating whether the corresponding slot in the pages field
    // can be written to
    slot_writable: u8,

    id: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum SegaMemoryMapRegister {
    FFFC,
    FFFD,
    FFFE,
    FFFF,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum MemoryLocation {
    RomAddress(u32),
    SystemRamAddress(u16),
    CartridgeRamAddress(u16),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SegaMemoryMapMessage<R> {
    AllocateFirstPage,
    AllocateSecondPage,
    InvalidWrite {
        logical_address: u16,
        value: u8,
        location: MemoryLocation,
    },
    RegisterWrite {
        register: R,
        value: u8,
    },
    MapRom {
        slot: u8,
        page: u8,
    },
    MapCartridgeRam {
        page: u8,
        slot: u8,
    },
    Read {
        logical_address: u16,
        value: u8,
        location: MemoryLocation,
    },
    Write {
        logical_address: u16,
        value: u8,
        location: MemoryLocation,
    },
}

impl Sender for SegaMemoryMap {
    type Message = SegaMemoryMapMessage<SegaMemoryMapRegister>;

    fn id(&self) -> u32 { self.id }
    fn set_id(&mut self, id: u32) { self.id = id; }
}

/// If the `logical_address` being written to corresponds to one of the memory
/// registers, update the memory map to reflect the new value.
fn write_check_register<R>(
    receiver: &mut R,
    smm: &mut SegaMemoryMap,
    logical_address: u16,
    value: u8
) where
    R: Receiver<SegaMemoryMapMessage<SegaMemoryMapRegister>>
{
    macro_rules! ensure_one_page_allocated {
        () => {
            if smm.ram_pages_allocated == Zero {
                receiver.receive(
                    smm.id(),
                    SegaMemoryMapMessage::AllocateFirstPage::<SegaMemoryMapRegister>
                );
                smm.memory.push([0; 0x2000]);
                smm.memory.push([0; 0x2000]);
                smm.ram_pages_allocated = One;
                smm.memory.shrink_to_fit();
            }
        }
    }

    macro_rules! ensure_two_pages_allocated {
        () => {
            if smm.ram_pages_allocated == Zero {
                receiver.receive(
                    smm.id(),
                    SegaMemoryMapMessage::AllocateFirstPage::<SegaMemoryMapRegister>
                );
                receiver.receive(
                    smm.id(),
                    SegaMemoryMapMessage::AllocateSecondPage::<SegaMemoryMapRegister>
                );
                smm.memory.push([0; 0x2000]);
                smm.memory.push([0; 0x2000]);
                smm.memory.push([0; 0x2000]);
                smm.memory.push([0; 0x2000]);
                smm.memory.shrink_to_fit();
            } else if smm.ram_pages_allocated == One {
                receiver.receive(
                    smm.id(),
                    SegaMemoryMapMessage::AllocateSecondPage::<SegaMemoryMapRegister>
                );
                assert!(smm.memory.len() >= 3);
                // the first sega-page of cartridge RAM needs to come last, so
                // push it over
                let first_position = smm.memory.len() - 2;
                smm.memory.insert(first_position, [0; 0x2000]);
                smm.memory.insert(first_position + 1, [0; 0x2000]);
                smm.memory.shrink_to_fit();
            }
            smm.ram_pages_allocated = Two;
        }
    }

    let rom_impl_page_count = match smm.ram_pages_allocated {
        // subtract off 1 for the system memory impl_page, and two for each
        // sega_page of ram allocated
        Zero => smm.memory.len() - 1,
        One => smm.memory.len() - 3,
        Two => smm.memory.len() - 5,
    };

    // debug_assert!(rom_impl_page_count % 2 == 0);

    // there are at most 0x100 sega-pages of ROM, so there should be at most
    // 0x200 implementation-pages
    debug_assert!(rom_impl_page_count < 0x200);

    let rom_sega_page_count = (rom_impl_page_count / 2) as u8;

    if rom_sega_page_count.count_ones() != 1 {
        // XXX Since I'm not really sure what is the right thing to do in this
        // case, I'll log it as a fault
        // log_fault!(
        //     "SegaMemoryMap: ROM size not a power of two: {:0>2X} sega-pages",
        //     rom_sega_page_count
        // );
    }

    // XXX is this the right thing to do?
    // It's correct when `rom_sega_page_count` is a power of two, but who knows
    // what happens in actual hardware when it's not?
    let sega_page = if rom_sega_page_count == 0 {
        0
    } else {
        value % rom_sega_page_count
    };

    let impl_page = (sega_page as u16) * 2 + 1;

    match logical_address {
        0xFFFC => {
            // RAM mapping and misc register
            // XXX - there is an unimplemented feature in which, if bit 4 is
            // set, the fist sega-page of Cartridge RAM is mapped into sega-slot
            // 3. But "no known software" uses this feature.
            receiver.receive(
                smm.id(),
                SegaMemoryMapMessage::RegisterWrite {
                    register: SegaMemoryMapRegister::FFFC,
                    value: value,
                },
            );
            let impl_page = match value & 0b1100 {
                0b1000 => {
                    // sega-slot 2 mapped to sega-page 0 of cartridge RAM
                    ensure_one_page_allocated!();
                    receiver.receive(
                        smm.id(),
                        SegaMemoryMapMessage::MapCartridgeRam {
                            page: 0,
                            slot: 2,
                        },
                    );
                    smm.slot_writable |= 1 << 4;
                    smm.slot_writable |= 1 << 5;
                    (smm.memory.len() - 2) as u16
                },
                0b1100 => {
                    // sega-slot 2 mapped to sega-page 1 of cartridge RAM
                    ensure_two_pages_allocated!();
                    receiver.receive(
                        smm.id(),
                        SegaMemoryMapMessage::MapCartridgeRam {
                            page: 1,
                            slot: 2,
                        },
                    );
                    smm.slot_writable |= 1 << 4;
                    smm.slot_writable |= 1 << 5;
                    (smm.memory.len() - 4) as u16
                },
                _ => {
                    // sega-slot 2 mapped to page of ROM indicated by register
                    // 0xFFFF
                    receiver.receive(
                        smm.id(),
                        SegaMemoryMapMessage::MapRom {
                            page: smm.reg_ffff,
                            slot: 2,
                        },
                    );
                    smm.slot_writable &= !(1 << 4);
                    smm.slot_writable &= !(1 << 5);
                    (smm.reg_ffff as u16) * 2 + 1
                }
            };
            smm.pages[4] = impl_page;
            smm.pages[5] = impl_page + 1;
            smm.reg_fffc = value;
        }
        0xFFFD => {
            receiver.receive(
                smm.id(),
                SegaMemoryMapMessage::RegisterWrite {
                    register: SegaMemoryMapRegister::FFFD,
                    value: value,
                },
            );
            receiver.receive(
                smm.id(),
                SegaMemoryMapMessage::MapRom {
                    page: sega_page,
                    slot: 0,
                },
            );
            smm.pages[0] = impl_page;
            smm.pages[1] = impl_page + 1;
            smm.slot_writable &= !(1 << 0);
            smm.slot_writable &= !(1 << 1);
            smm.reg_fffd = sega_page;
        }
        0xFFFE => {
            receiver.receive(
                smm.id(),
                SegaMemoryMapMessage::RegisterWrite {
                    register: SegaMemoryMapRegister::FFFE,
                    value: value,
                },
            );
            receiver.receive(
                smm.id(),
                SegaMemoryMapMessage::MapRom {
                    page: sega_page,
                    slot: 1,
                },
            );
            smm.pages[2] = impl_page;
            smm.pages[3] = impl_page + 1;
            smm.slot_writable &= !(1 << 2);
            smm.slot_writable &= !(1 << 3);
            smm.reg_fffe = sega_page;
        }
        0xFFFF => {
            receiver.receive(
                smm.id(),
                SegaMemoryMapMessage::RegisterWrite {
                    register: SegaMemoryMapRegister::FFFF,
                    value: value,
                },
            );
            if smm.reg_ffff & 0b1000 == 0 {
                receiver.receive(
                    smm.id(),
                    SegaMemoryMapMessage::MapRom {
                        page: sega_page,
                        slot: 1,
                    },
                );
                smm.pages[4] = impl_page;
                smm.pages[5] = impl_page + 1;
            }
            smm.reg_ffff = sega_page;
        }
        _ => {
        }
    }
}

impl SegaMemoryMap {
    #[inline(always)]
    fn logical_address_to_memory_location(&self, logical_address: u16) -> MemoryLocation {
        if logical_address < 0x400 {
            return MemoryLocation::RomAddress(logical_address as u32);
        }
        let sega_slot = (logical_address & 0xC000) >> 14; // high order 2 bits
        let physical_address = logical_address & 0x3FFF; // low order 14 bits
        match sega_slot {
            0 => {
                // ROM, page determined by register fffd
                let page = self.reg_fffd as u32;
                return MemoryLocation::RomAddress(page * physical_address as u32);
            },
            1 => {
                // ROM, page determined by register fffe
                let page = self.reg_fffe as u32;
                return MemoryLocation::RomAddress(page * physical_address as u32);
            },
            2 => {
                match self.reg_ffff & 0b1100 {
                    0b1000 => {
                        // mapped to sega-page 0 of cartridge RAM
                        return MemoryLocation::CartridgeRamAddress(physical_address);
                    },
                    0b1100 => {
                        // mapped to sega-page 1 of cartridge RAM
                        return MemoryLocation::CartridgeRamAddress(0x4000 | physical_address);
                    },
                    _ => {
                        // ROM, page determined by register ffff
                        let page = self.reg_fffe as u32;
                        return MemoryLocation::RomAddress(page * physical_address as u32);
                    }
                }
            },
            3 => {
                // System RAM, which is only 8 KiB, mirrored
                return MemoryLocation::SystemRamAddress(physical_address & 0x1FFF);
            },
            _ => {
                unreachable!();
            }
        }
    }
}

impl MemoryMap for SegaMemoryMap {
    fn read<R>(&self, receiver: &mut R, logical_address: u16) -> u8
    where
        R: Receiver<SegaMemoryMapMessage<SegaMemoryMapRegister>>
    {
        let result = if logical_address < 0x400 {
            // first KiB of logical memory is always mapped to the first KiB of
            // the first page of ROM
            // Some options for the future to avoid this check:
            // - Store an extra 8 KiB for the zeroth impl-slot, and copy the
            // desired page into it, but replacing the first KiB with the first
            // KiB of the zeroth impl-page.
            // - Use 1 KiB impl-pages, and never remap the zeroth slot. (This is
            // probably the best option.)
            self.memory[1][logical_address as usize]
        } else {
            let physical_address = logical_address & 0x1FFF; // low order 13 bits
            let impl_slot = (logical_address & 0xE000) >> 13; // high order 3 bits
            let impl_page = self.pages[impl_slot as usize];
            self.memory[impl_page as usize][physical_address as usize]
        };
        receiver.receive(
            self.id(),
            SegaMemoryMapMessage::Read {
                logical_address: logical_address,
                value: result,
                location: self.logical_address_to_memory_location(logical_address),
            },
        );
        result
    }

    fn write<R>(&mut self, receiver: &mut R, logical_address: u16, value: u8)
    where
        R: Receiver<SegaMemoryMapMessage<SegaMemoryMapRegister>>
    {
        write_check_register(receiver, self, logical_address, value);
        let physical_address = logical_address & 0x1FFF; // low order 13 bits
        let impl_slot = (logical_address & 0xE000) >> 13; // high order 3 bits
        if self.slot_writable & (1 << impl_slot) != 0 {
            receiver.receive(
                self.id(),
                SegaMemoryMapMessage::Write {
                    logical_address: logical_address,
                    value: value,
                    location: self.logical_address_to_memory_location(logical_address),
                },
            );
            let impl_page = self.pages[impl_slot as usize];
            self.memory[impl_page as usize][physical_address as usize] = value;
        } else {
            receiver.receive(
                self.id(),
                SegaMemoryMapMessage::InvalidWrite {
                    logical_address: logical_address,
                    value: value,
                    location: self.logical_address_to_memory_location(logical_address),
                },
            );
        }
    }
}

impl SegaMemoryMap {
    pub fn new(rom: &[u8]) -> Result<SegaMemoryMap, MemoryMapError> {
        if rom.len() % 0x2000 != 0 || rom.len() == 0 {
            return Err(MemoryMapError {
                msg: format!(
                    "Invalid ROM size 0x{:0>6X} (must be a positive multiple of 0x2000)",
                    rom.len()
                ),
            });
        }

        let rom_impl_page_count = rom.len() / 0x2000;

        let mut memory = Vec::with_capacity(1 + rom_impl_page_count);

        // push the system RAM
        memory.push([0; 0x2000]);

        // push the ROM
        for i in 0..rom_impl_page_count {
            let mut impl_page = [0u8; 0x2000];
            impl_page.copy_from_slice(&rom[0x2000*i .. 0x2000*(i+1)]);
            memory.push(impl_page);
        }

        Ok(
            SegaMemoryMap {
                memory: memory,
                ram_pages_allocated: Zero,
                // supposedly these registers are undefined after a reset, but
                // in the 315-5235 mapper they take these values
                reg_fffc: 0,
                reg_fffd: 0,
                reg_fffe: 1,
                reg_ffff: 2,
                // which means these are the implementation-pages we map to
                pages: [1, 2, 3, 4, 5, 6, 0, 0],
                // only the system RAM is writable
                slot_writable: 0b11000000,
                id: 0,
            }
        )
    }

    pub fn new_from_file(
        filename: &str,
    ) -> Result<SegaMemoryMap, MemoryMapError> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open(filename)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;

        SegaMemoryMap::new(&buf[0..])
    }
}

mod tests {
    use super::*;

    #[allow(dead_code)]
    fn build_mmap() -> SegaMemoryMap {
        let mut rom = [0u8; 0x10000]; // 64 KiB (8 8KiB impl-pages or 4 16KiB sega-pages)
        rom[0x2000] = 1;
        rom[0x4000] = 2;
        rom[0x6000] = 3;
        rom[0x8000] = 4;
        rom[0xA000] = 5;
        rom[0xC000] = 6;
        rom[0xE000] = 7;
        SegaMemoryMap::new(&rom).unwrap()
    }

    #[test]
    fn read() {
        let smm = &mut build_mmap();

        // read impl-slot 0
        assert!(smm.read(0) == 0);

        // read impl-slot 1
        assert!(smm.read(0x2000) == 1);

        // read impl-slot 2
        assert!(smm.read(0x4000) == 2);

        // read impl-slot 3
        assert!(smm.read(0x6000) == 3);

        // read impl-slot 4
        assert!(smm.read(0x8000) == 4);

        // read impl-slot 5
        assert!(smm.read(0xA000) == 5);

        // read impl-slot 6 (should be system memory)
        assert!(smm.read(0xC000) == 0);

        // read impl-slot 7 (should be system memory)
        assert!(smm.read(0xE000) == 0);
    }

    #[test]
    fn reg_ffff() {
        let smm = &mut build_mmap();
        smm.write(0xFFFF, 3); // sega-slot 2 should now map to sega-page 3
        assert!(smm.read(0x8000) == 6);
        assert!(smm.read(0xA000) == 7);
        smm.write(0xFFFF, 0); // sega-slot 2 should now map to sega-page 0
        assert!(smm.read(0x8000) == 0);
        assert!(smm.read(0xA000) == 1);
    }

    #[test]
    fn reg_fffe() {
        let smm = &mut build_mmap();
        smm.write(0xFFFE, 3); // sega-slot 1 should now map to sega-page 3
        assert!(smm.read(0x4000) == 6);
        assert!(smm.read(0x6000) == 7);
        smm.write(0xFFFE, 0); // sega-slot 1 should now map to sega-page 0
        assert!(smm.read(0x4000) == 0);
        assert!(smm.read(0x6000) == 1);
    }

    #[test]
    fn reg_fffd() {
        let smm = &mut build_mmap();
        smm.write(0xFFFD, 1); // sega-slot 0 should now map to sega-page 1
        assert!(smm.read(0x0000) == 0); // except the first KiB
        assert!(smm.read(0x2000) == 3);
        smm.write(0xFFFD, 0); // sega-slot 0 should now map to sega-page 0
        assert!(smm.read(0x0000) == 0);
        assert!(smm.read(0x2000) == 1);
    }

    #[test]
    fn reg_fffc() {
        let smm = &mut build_mmap();
        smm.write(0xFFFC, 0b1000); // sega-slot 2 mapped to sega-page 0 of cartridge RAM
        assert!(smm.read(0x8000) == 0);
        smm.write(0x8000, 102);
        assert!(smm.read(0x8000) == 102);

        smm.write(0xFFFC, 0); // sega-slot 2 mapped back to sega-page 2 of ROM
        assert!(smm.read(0x8000) == 4);
        smm.write(0, 17);
        assert!(smm.read(0x8000) == 4); // which should not be writable

        smm.write(0xFFFC, 0b1000); // back to sega-page 0 of cartridge RAM
        assert!(smm.read(0x8000) == 102);

        smm.write(0xFFFC, 0b1100); // to sega-page 1 of cartridge RAM
        assert!(smm.read(0x8000) == 0);
        smm.write(0x8000, 103);
        assert!(smm.read(0x8000) == 103);

        smm.write(0xFFFC, 0b1000); // back to sega-page 0 of cartridge RAM
        assert!(smm.read(0x8000) == 102);
    }
}
