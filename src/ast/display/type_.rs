use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::ast::Type;

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut stack = Vec::new();
        let mut var = 0;
        TypeDisplay(self, &mut stack, &mut var, 0).fmt(fmt)
    }
}

struct Name(usize);

impl Display for Name {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut n = self.0;
        let mut digits: Vec<u8> = Vec::new();
        while n > 0 {
            digits.push((n % 26) as u8);
            n /= 26;
        }
        if digits.is_empty() {
            digits.push(0);
        }

        for d in digits {
            write!(fmt, "{}", char::from(d + 97))?;
        }
        Ok(())
    }
}

struct TypeDisplay<'a>(&'a Type, &'a mut Vec<usize>, &'a mut usize, u8);

// TODO: Clean this up!!!
impl<'a> TypeDisplay<'a> {
    fn fmt(&mut self, fmt: &mut Formatter) -> FmtResult {
        let mut expr = self.0;
        let mut pops = 0;
        let mut prec = self.3;
        if let Type::Forall(_) = self.0 {
            if self.3 > 0 {
                write!(fmt, "(")?;
            }
            let mut first = true;
            while let Type::Forall(ref t) = expr {
                if first {
                    first = false;
                } else {
                    write!(fmt, " ")?;
                }

                let n = *self.2;
                *self.2 = n + 1;
                self.1.push(n);
                pops += 1;
                expr = t;
                write!(fmt, "'{}", Name(n))?;
            }
            write!(fmt, ". ")?;
            prec = 0;
        }

        match *expr {
            Type::Bool => {
                write!(fmt, "bool")?;
            }
            Type::Forall(_) => unreachable!(),
            Type::Func(ref l, ref r) => {
                if prec > 0 {
                    write!(fmt, "(")?;
                }
                TypeDisplay(&**l, self.1, self.2, 1).fmt(fmt)?;
                write!(fmt, " -> ")?;
                TypeDisplay(&**r, self.1, self.2, 0).fmt(fmt)?;
                if prec > 0 {
                    write!(fmt, ")")?;
                }
            }
            Type::Int => {
                write!(fmt, "int")?;
            }
            Type::List(ref t) => {
                if prec > 1 {
                    write!(fmt, "(")?;
                }
                TypeDisplay(&**t, self.1, self.2, 1).fmt(fmt)?;
                write!(fmt, " list")?;
                if prec > 1 {
                    write!(fmt, ")")?;
                }
            }
            Type::Var(n) => {
                let n = self.1.len() - n - 1;
                write!(fmt, "'{}", Name(self.1[n]))?;
            }
        }

        for _ in 0..pops {
            self.1.pop();
        }
        if let Type::Forall(_) = self.0 {
            if self.3 > 0 {
                write!(fmt, ")")?;
            }
        }
        Ok(())
    }
}
