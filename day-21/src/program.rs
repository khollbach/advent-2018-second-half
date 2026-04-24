use std::fmt;

use anyhow::{Result, ensure};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub op: Operation,
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

pub fn run(ip_register: usize, program: Vec<Instruction>, input: u64) -> Result<u64> {
    Machine::new(ip_register, program, input)?.run()
}

const NUM_REGS: usize = 6;

struct Machine {
    ip_register: usize,
    registers: [u64; NUM_REGS],
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CpuStatus {
    Continue,
    Halt,
}

impl Machine {
    fn new(ip_register: usize, instructions: Vec<Instruction>, input: u64) -> Result<Self> {
        ensure!(ip_register < NUM_REGS);

        let mut registers = [0; _];
        registers[0] = input;

        Ok(Self {
            ip_register,
            registers,
            instructions,
        })
    }

    fn run(mut self) -> Result<u64> {
        loop {
            // eprintln!("{:?}", self);
            // std::thread::sleep(std::time::Duration::from_millis(400));
            match self.step()? {
                CpuStatus::Continue => (),
                CpuStatus::Halt => return Ok(self.registers[0]),
            }
        }
    }

    fn ip(&self) -> Option<usize> {
        let ip = self.registers[self.ip_register].try_into().unwrap();
        if ip < self.instructions.len() {
            Some(ip)
        } else {
            None
        }
    }

    fn step(&mut self) -> Result<CpuStatus> {
        let Some(ip) = self.ip() else {
            return Ok(CpuStatus::Halt);
        };

        let Instruction { op, a, b, c } = self.instructions[ip];

        if op == Operation::Seti && c == 2 {
            eprintln!("{:?}", self);
        }

        match op {
            Operation::Addr => self.addr(a, b, c)?,
            Operation::Addi => self.addi(a, b, c)?,
            Operation::Mulr => self.mulr(a, b, c)?,
            Operation::Muli => self.muli(a, b, c)?,
            Operation::Banr => self.banr(a, b, c)?,
            Operation::Bani => self.bani(a, b, c)?,
            Operation::Borr => self.borr(a, b, c)?,
            Operation::Bori => self.bori(a, b, c)?,
            Operation::Setr => self.setr(a, b, c)?,
            Operation::Seti => self.seti(a, b, c)?,
            Operation::Gtir => self.gtir(a, b, c)?,
            Operation::Gtri => self.gtri(a, b, c)?,
            Operation::Gtrr => self.gtrr(a, b, c)?,
            Operation::Eqir => self.eqir(a, b, c)?,
            Operation::Eqri => self.eqri(a, b, c)?,
            Operation::Eqrr => self.eqrr(a, b, c)?,
        };

        self.registers[self.ip_register] += 1;

        Ok(if self.ip().is_some() {
            CpuStatus::Continue
        } else {
            CpuStatus::Halt
        })
    }
}

impl Machine {
    fn reg(&mut self, idx: u64) -> Result<&mut u64> {
        ensure!(idx < NUM_REGS.try_into().unwrap());
        Ok(&mut self.registers[usize::try_from(idx).unwrap()])
    }
}

impl Machine {
    fn addr(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? + *self.reg(b)?;
        Ok(())
    }

    fn addi(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? + b;
        Ok(())
    }

    fn mulr(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? * *self.reg(b)?;
        Ok(())
    }

    fn muli(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? * b;
        Ok(())
    }

    fn banr(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? & *self.reg(b)?;
        Ok(())
    }

    fn bani(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? & b;
        Ok(())
    }

    fn borr(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? | *self.reg(b)?;
        Ok(())
    }

    fn bori(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? | b;
        Ok(())
    }

    fn setr(&mut self, a: u64, _b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = *self.reg(a)?;
        Ok(())
    }

    fn seti(&mut self, a: u64, _b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = a;
        Ok(())
    }

    fn gtir(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = u64::from(a > *self.reg(b)?);
        Ok(())
    }

    fn gtri(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = u64::from(*self.reg(a)? > b);
        Ok(())
    }

    fn gtrr(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = u64::from(*self.reg(a)? > *self.reg(b)?);
        Ok(())
    }

    fn eqir(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = u64::from(a == *self.reg(b)?);
        Ok(())
    }

    fn eqri(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = u64::from(*self.reg(a)? == b);
        Ok(())
    }

    fn eqrr(&mut self, a: u64, b: u64, c: u64) -> Result<()> {
        *self.reg(c)? = u64::from(*self.reg(a)? == *self.reg(b)?);
        Ok(())
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08x?}", self.registers)
    }
}
