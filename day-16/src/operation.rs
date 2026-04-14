use crate::input::State;
use anyhow::{Result, ensure};

pub const OPERATIONS: [Operation; 16] = [
    Operation::Addr,
    Operation::Addi,
    Operation::Mulr,
    Operation::Muli,
    Operation::Banr,
    Operation::Bani,
    Operation::Borr,
    Operation::Bori,
    Operation::Setr,
    Operation::Seti,
    Operation::Gtir,
    Operation::Gtri,
    Operation::Gtrr,
    Operation::Eqir,
    Operation::Eqri,
    Operation::Eqrr,
];

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

impl Operation {
    pub fn apply(self, mut state: State, a: u32, b: u32, c: u32) -> Result<State> {
        match self {
            Operation::Addr => state.addr(a, b, c)?,
            Operation::Addi => state.addi(a, b, c)?,
            Operation::Mulr => state.mulr(a, b, c)?,
            Operation::Muli => state.muli(a, b, c)?,
            Operation::Banr => state.banr(a, b, c)?,
            Operation::Bani => state.bani(a, b, c)?,
            Operation::Borr => state.borr(a, b, c)?,
            Operation::Bori => state.bori(a, b, c)?,
            Operation::Setr => state.setr(a, b, c)?,
            Operation::Seti => state.seti(a, b, c)?,
            Operation::Gtir => state.gtir(a, b, c)?,
            Operation::Gtri => state.gtri(a, b, c)?,
            Operation::Gtrr => state.gtrr(a, b, c)?,
            Operation::Eqir => state.eqir(a, b, c)?,
            Operation::Eqri => state.eqri(a, b, c)?,
            Operation::Eqrr => state.eqrr(a, b, c)?,
        };
        Ok(state)
    }
}

impl State {
    fn reg(&mut self, idx: u32) -> Result<&mut u32> {
        ensure!(idx < 4);
        let idx = usize::try_from(idx).unwrap();
        Ok(&mut self.registers[idx])
    }
}

impl State {
    fn addr(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? + *self.reg(b)?;
        Ok(())
    }

    fn addi(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? + b;
        Ok(())
    }

    fn mulr(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? * *self.reg(b)?;
        Ok(())
    }

    fn muli(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? * b;
        Ok(())
    }

    fn banr(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? & *self.reg(b)?;
        Ok(())
    }

    fn bani(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? & b;
        Ok(())
    }

    fn borr(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? | *self.reg(b)?;
        Ok(())
    }

    fn bori(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)? | b;
        Ok(())
    }

    fn setr(&mut self, a: u32, _b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = *self.reg(a)?;
        Ok(())
    }

    fn seti(&mut self, a: u32, _b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = a;
        Ok(())
    }

    fn gtir(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = u32::from(a > *self.reg(b)?);
        Ok(())
    }

    fn gtri(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = u32::from(*self.reg(a)? > b);
        Ok(())
    }

    fn gtrr(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = u32::from(*self.reg(a)? > *self.reg(b)?);
        Ok(())
    }

    fn eqir(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = u32::from(a == *self.reg(b)?);
        Ok(())
    }

    fn eqri(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = u32::from(*self.reg(a)? == b);
        Ok(())
    }

    fn eqrr(&mut self, a: u32, b: u32, c: u32) -> Result<()> {
        *self.reg(c)? = u32::from(*self.reg(a)? == *self.reg(b)?);
        Ok(())
    }
}
