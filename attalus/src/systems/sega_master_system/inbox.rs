use std::collections::VecDeque;
use std::fmt::Write;
use std::time::Instant;

use hardware::z80::memo::Opcode;
use memo::{InboxImpler, Memo, HoldableImpler};

use super::emulator::TimeStatus;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Query {
    Disassemble(u16),
    RecentMemos,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QueryResult {
    Ok(String),
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Command {
    Hold,
    Resume,
    Step,
    BreakAtPc(u16),
    RemovePcBreakpoints,
    // BreakAtMemo(MemoPattern),
    // RemoveBreakMemos,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum CommandResult {
    Ok,
    Unsupported,
}

pub trait Debugger {
    fn command(&mut self, command: Command) -> CommandResult;
    fn query(&mut self, query: Query) -> QueryResult;
}

pub trait DebuggerImpler<S: ?Sized> {
    fn command(&mut S, command: Command) -> CommandResult;
    fn query(&mut S, query: Query) -> QueryResult;
}

pub trait DebuggerImpl {
    type Impler: DebuggerImpler<Self>;
}

impl<S> Debugger for S
where
    S: DebuggerImpl + ?Sized,
{
    #[inline]
    fn command(&mut self, command: Command) -> CommandResult {
        <S::Impler as DebuggerImpler<Self>>::command(self, command)
    }

    #[inline]
    fn query(&mut self, query: Query) -> QueryResult {
        <S::Impler as DebuggerImpler<Self>>::query(self, query)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HoldingDebugger;

impl HoldingDebugger {
    pub fn new() -> HoldingDebugger {
        HoldingDebugger
    }
}

impl<S> HoldableImpler<S> for HoldingDebugger
where
    S: ?Sized + AsRef<TimeStatus>,
{
    #[inline]
    fn holding(s: &S) -> bool {
        s.as_ref().hold.is_some()
    }
}

impl<S> InboxImpler<S> for HoldingDebugger
where
    S: ?Sized,
{
    fn receive(_s: &mut S, _memo: Memo) {}
}

impl<S> DebuggerImpler<S> for HoldingDebugger
where
    S: ?Sized
        + AsRef<HoldingDebugger>
        + AsMut<HoldingDebugger>
        + AsRef<TimeStatus>
        + AsMut<TimeStatus>,
{
    #[inline]
    fn command(s: &mut S, command: Command) -> CommandResult {
        use self::Command::*;
        let time_status = AsMut::<TimeStatus>::as_mut(s);
        match (command, time_status.hold) {
            (Hold, None) => time_status.hold = Some(Instant::now()),
            (Resume, Some(instant)) => {
                time_status.hold = None;
                let elapsed = Instant::now().duration_since(instant);
                time_status.hold_duration += elapsed;
            }
            (Hold, _) => {}
            _ => return CommandResult::Unsupported,
        }
        return CommandResult::Ok;
    }

    #[inline]
    fn query(_s: &mut S, _query: Query) -> QueryResult {
        QueryResult::Unsupported
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
enum DebugStatus {
    None,
    Step,
}

impl Default for DebugStatus {
    fn default() -> Self {
        DebugStatus::None
    }
}

const MAX_MESSAGES: usize = 50;

#[derive(Clone)]
pub struct DebuggingInbox {
    last_pc: u16,
    opcodes: [Option<Opcode>; 0x10000],
    status: DebugStatus,
    pc_breakpoints: Vec<u16>,
    // memo_patterns: Vec<MemoPattern>,
    recent_memos: VecDeque<Memo>,
}

impl DebuggingInbox {
    fn new() -> Self {
        DebuggingInbox {
            last_pc: 0,
            opcodes: [None; 0x10000],
            status: DebugStatus::None,
            pc_breakpoints: Vec::new(),
            recent_memos: VecDeque::new(),
        }
    }

    /// Find the PC pointing at the instruction immediately before pc, if it exists
    fn back_1(&self, pc: u16) -> Option<u16> {
        for i in 1..5 {
            if pc < i {
                return None;
            }
            match (self.opcodes[(pc - i) as usize], i) {
                (Some(Opcode::OneByte(_)), 1) => return Some(pc - i),
                (Some(Opcode::TwoBytes(_)), 2) => return Some(pc - i),
                (Some(Opcode::ThreeBytes(_)), 3) => return Some(pc - i),
                (Some(Opcode::FourBytes(_)), 4) => return Some(pc - i),
                _ => {}
            }
        }
        return None;
    }

    /// Find the earliest PC pointing at an opcode, at most n steps back
    fn back_n(&self, n: u16, pc: u16) -> u16 {
        let mut pc_current = pc;
        for _ in 0..n {
            match self.back_1(pc_current) {
                None => return pc_current,
                Some(pc_new) => pc_current = pc_new,
            }
        }
        return pc_current;
    }

    fn disassembly_around(&self, pc: u16) -> String {
        let mut pc_current = self.back_n(5, pc);
        let mut result = "".to_owned();
        for _ in 0..10 {
            let opcode = match self.opcodes[pc_current as usize] {
                None => return result,
                Some(x) => x,
            };
            result.push_str(&format!(
                "{:0>4X}: {: <width$}",
                pc_current,
                opcode,
                width = 12
            ));
            match opcode.mnemonic() {
                None => result.push_str("Unknown opcode"),
                Some(x) => result.push_str(&format!("{}", x)),
            }
            if pc_current == pc {
                result.push_str(" <<<");
            }
            result.push('\n');
            pc_current = pc_current.wrapping_add(match opcode {
                Opcode::OneByte(_) => 1,
                Opcode::TwoBytes(_) => 2,
                Opcode::ThreeBytes(_) => 3,
                Opcode::FourBytes(_) => 4,
            });
        }
        result
    }
}

impl Default for DebuggingInbox {
    fn default() -> Self {
        DebuggingInbox::new()
    }
}

impl<S> InboxImpler<S> for DebuggingInbox
where
    S: ?Sized + AsMut<DebuggingInbox> + AsRef<DebuggingInbox>,
{
    fn receive(s: &mut S, memo: Memo) {
        use hardware::z80::memo::manifests::INSTRUCTION;
        use memo::Payload;
        use std::mem::transmute;

        if s.as_ref().recent_memos.len() >= MAX_MESSAGES {
            s.as_mut().recent_memos.pop_front();
        }

        if memo.has_manifest(INSTRUCTION) {
            let payload = match memo.payload() {
                Payload::U8(x) => x,
                _ => unreachable!("INSTRUCTION payload not of U8 type?"),
            };
            let pc_array: [u8; 2] = [payload[0], payload[1]];
            let pc: u16 = unsafe { transmute(pc_array) };
            let opcode = Opcode::from_payload(payload);
            s.as_mut().opcodes[pc as usize] = Some(opcode);
            s.as_mut().last_pc = pc;
        }

        // if the new memo matches a pattern, hold

        s.as_mut().recent_memos.push_back(memo);
    }
}

impl<S> HoldableImpler<S> for DebuggingInbox
where
    S: ?Sized + AsRef<TimeStatus>,
{
    #[inline]
    fn holding(s: &S) -> bool {
        s.as_ref().hold.is_some()
    }
}

impl<S> DebuggerImpler<S> for DebuggingInbox
where
    S: ?Sized
        + AsRef<DebuggingInbox>
        + AsMut<DebuggingInbox>
        + AsRef<TimeStatus>
        + AsMut<TimeStatus>,
{
    fn query(s: &mut S, query: Query) -> QueryResult {
        use self::Query::*;
        let result = match query {
            RecentMemos => {
                let mut result = String::new();
                for memo in AsRef::<DebuggingInbox>::as_ref(s).recent_memos.iter() {
                    writeln!(result, "{}", memo).unwrap();
                }
                result
            }
            Disassemble(pc) => AsRef::<DebuggingInbox>::as_ref(s).disassembly_around(pc),
        };
        QueryResult::Ok(result)
    }

    fn command(s: &mut S, command: Command) -> CommandResult {
        use self::Command::*;

        match (command, AsRef::<TimeStatus>::as_ref(s).hold) {
            (Hold, None) => AsMut::<TimeStatus>::as_mut(s).hold = Some(Instant::now()),
            (Resume, Some(instant)) => {
                AsMut::<TimeStatus>::as_mut(s).hold = None;
                let elapsed = Instant::now().duration_since(instant);
                AsMut::<TimeStatus>::as_mut(s).hold_duration += elapsed;
            }
            (BreakAtPc(pc), _) => AsMut::<DebuggingInbox>::as_mut(s).pc_breakpoints.push(pc),
            (RemovePcBreakpoints, _) => {
                AsMut::<DebuggingInbox>::as_mut(s).pc_breakpoints = Vec::new()
            }
            (Step, _) => AsMut::<DebuggingInbox>::as_mut(s).status = DebugStatus::Step,
            // BreakAtMemo(pattern) => self.memo_patterns.push(pattern),
            // RemoveBreakMemos => self.memo_patterns = Vec::new(),
            _ => {}
        }

        CommandResult::Ok
    }
}
