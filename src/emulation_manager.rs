use std;

use sdl_wrap;
use log::*;
use hardware::z80::*;
use hardware::irq::*;
use hardware::vdp::*;
use hardware::memory_mapper;
use hardware::memory_mapper::implementation::*;
use hardware::io::*;

pub struct EmulationManager<L: Log, M: MemoryMapperHardware> {
    log: L,
    memory_mapper_hardware: M,
    io_hardware: IoHardware,
    z80_hardware: Z80Hardware,
    vdp_hardware: VdpHardware,
    cycles_by_z80: u64,
}

impl<L: Log, M: MemoryMapperHardware> EmulationManager<L, M> {
    pub fn new(log: L, m: M) -> EmulationManager<L, M> {
        EmulationManager {
            log: log,
            memory_mapper_hardware: m,
            io_hardware: Default::default(),
            z80_hardware: Default::default(),
            vdp_hardware: Default::default(),
            cycles_by_z80: 0,
        }
    }
}

impl<L: Log, M: MemoryMapperHardware> Log for EmulationManager<L, M> {
    fn log_minor0(&mut self, s: String) {
        self.log.log_minor0(s)
    }
    fn log_major0(&mut self, s: String) {
        self.log.log_major0(s)
    }
    fn log_fault0(&mut self, s: String) {
        self.log.log_fault0(s)
    }
    fn does_log_minor(&self) -> bool {
        self.log.does_log_minor()
    }
    fn does_log_major(&self) -> bool {
        self.log.does_log_major()
    }
    fn does_log_fault(&self) -> bool {
        self.log.does_log_fault()
    }
    fn check_fault(&self) -> Option<String> {
        self.log.check_fault()
    }
}

impl<L: Log, M: MemoryMapperHardware> Irq for EmulationManager<L, M> {
    fn request_maskable_interrupt(&mut self) -> bool {
        maskable_interrupt(self)
    }
    fn request_nonmaskable_interrupt(&mut self) {
        nonmaskable_interrupt(self);
    }
}

impl<L: Log, M: MemoryMapperHardware> MemoryMapper0 for EmulationManager<L, M> {
    type Hardware = M;
    fn get_memory_mapper_hardware(&self) -> &M {
        &self.memory_mapper_hardware
    }
    fn get_mut_memory_mapper_hardware(&mut self) -> &mut M {
        &mut self.memory_mapper_hardware
    }
}

impl<L: Log, M: MemoryMapperHardware> Vdp for EmulationManager<L, M> {
    fn get_vdp_hardware(&self) -> &VdpHardware {
        &self.vdp_hardware
    }
    fn get_mut_vdp_hardware(&mut self) -> &mut VdpHardware {
        &mut self.vdp_hardware
    }
}

impl<L: Log, M: MemoryMapperHardware> Io for EmulationManager<L, M> {
    fn get_io_hardware(&self) -> &IoHardware {
        &self.io_hardware
    }
    fn get_mut_io_hardware(&mut self) -> &mut IoHardware {
        &mut self.io_hardware
    }
}

impl<L: Log, M: MemoryMapperHardware> Z80 for EmulationManager<L, M> {
    fn get_z80_hardware(&self) -> &Z80Hardware {
        &self.z80_hardware
    }
    fn get_mut_z80_hardware(&mut self) -> &mut Z80Hardware {
        &mut self.z80_hardware
    }
    fn cycles(&mut self, i: u64) {
        self.cycles_by_z80 += 3 * i;
    }
}

pub fn main_loop<L: Log, M: MemoryMapperHardware, C: Canvas>(
  em: &mut EmulationManager<L, M>,
  canvas: &mut C,
  n: usize
) {
    let mut vdp_cycles: u64 = 0;

    for i in 0usize..n {
        log_major!(em, "EM: loop {}", i);
        // println!("EM: loop {} of {}", i, n);
        // vdp_cycles += draw_line(em, canvas).unwrap();
        vdp_cycles += draw_frame(em, canvas).unwrap();

        while em.cycles_by_z80 < vdp_cycles {
            interpreter::execute1(em);
        }
        // canvas.paint(5, i % 256, 0b101000);
        // canvas.paint(10, i % 256, 0xFF);
        // canvas.paint(15, i % 256, 0b110011);
        // canvas.paint(20, i % 256, 0b000011);

        // std::thread::sleep(std::time::Duration::from_millis(20));

        if sdl_wrap::event::check_quit() {
            break;
        }
    }
}
