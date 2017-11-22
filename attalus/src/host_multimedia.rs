// Copyright 2017 Michael Benfield <mike.benfield@gmail.com>
// This file is part of Attalus. You may distribute and/or modify Attalus under
// the terms of the GNU General Public License as published by the Free Sofware
// Foundation, either version 3 of the license or (at your option) any later
// version. You should have received a copy of the GNU General Public License
// along with Attalus. If not, see <http://www.gnu.org/licenses/>.

use ::errors::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SimpleColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub trait SimpleGraphics {
    fn set_resolution(&mut self, width: u32, height: u32) -> Result<()>;

    fn resolution(&self) -> (u32, u32);

    fn paint(&mut self, x: u32, y: u32, color: SimpleColor) -> Result<()>;

    fn get(&self, x: u32, y: u32) -> Result<SimpleColor>;

    fn render(&mut self) -> Result<()>;
}


pub trait SimpleAudio {
    fn configure(&mut self, frequency: u32, buffer_size: u16) -> Result<()>;

    fn play(&mut self) -> Result<()>;

    fn pause(&mut self) -> Result<()>;

    fn buffer(&mut self) -> Result<&mut [i16]>;

    fn queue_buffer(&mut self) -> Result<()>;

    fn clear(&mut self) -> Result<()>;
}