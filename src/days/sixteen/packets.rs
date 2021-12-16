use super::Operation;

macro_rules! impl_try_from_vec {
    ($ty:ty) => {
        impl TryFrom<super::RawPacket> for $ty {
            type Error = ();

            fn try_from(
                raw: super::RawPacket,
            ) -> Result<Self, <Self as TryFrom<super::RawPacket>>::Error> {
                if let Some(cl) = raw.children().map(|c| c.len()) {
                    if cl < 1 {
                        return Err(());
                    }
                } else {
                    return Err(());
                }

                if let super::RawPacket::Operation {
                    version, children, ..
                } = raw
                {
                    let mut operands: Vec<super::Packet> = Vec::new();
                    for c in children.into_iter() {
                        operands.push(<super::Packet as TryFrom<super::RawPacket>>::try_from(c)?);
                    }

                    Ok(Self { version, operands })
                } else {
                    Err(())
                }
            }
        }
    };
}

macro_rules! impl_try_from_bin {
    ($ty:ty) => {
        impl TryFrom<super::RawPacket> for $ty {
            type Error = ();

            fn try_from(
                raw: super::RawPacket,
            ) -> Result<Self, <Self as TryFrom<super::RawPacket>>::Error> {
                match raw.children().map(|c| c.len()) {
                    Some(2) => (),
                    _ => {
                        return Err(());
                    }
                }

                if let super::RawPacket::Operation {
                    version,
                    mut children,
                    ..
                } = raw
                {
                    let left = Box::new(<super::Packet as TryFrom<super::RawPacket>>::try_from(
                        children.remove(0),
                    )?);
                    let right = Box::new(<super::Packet as TryFrom<super::RawPacket>>::try_from(
                        children.remove(0),
                    )?);

                    Ok(Self {
                        version,
                        left,
                        right,
                    })
                } else {
                    Err(())
                }
            }
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Literal {
    version: u8,
    literal: u64,
}

impl Operation for Literal {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        self.literal
    }
}

impl TryFrom<super::RawPacket> for Literal {
    type Error = ();

    fn try_from(raw: super::RawPacket) -> Result<Self, <Self as TryFrom<super::RawPacket>>::Error> {
        if let super::RawPacket::Literal {
            version, literal, ..
        } = raw
        {
            Ok(Self { version, literal })
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Sum {
    version: u8,
    operands: Vec<super::Packet>,
}

impl Operation for Sum {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        self.operands.iter().map(super::Packet::eval).sum()
    }
}

impl_try_from_vec!(Sum);

#[derive(Clone, Debug)]
pub(crate) struct Product {
    version: u8,
    operands: Vec<super::Packet>,
}

impl Operation for Product {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        self.operands.iter().fold(1, |acc, v| acc * v.eval())
    }
}

impl_try_from_vec!(Product);

#[derive(Clone, Debug)]
pub(crate) struct Minimum {
    version: u8,
    operands: Vec<super::Packet>,
}

impl Operation for Minimum {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        self.operands
            .iter()
            .fold(None, |acc: Option<u64>, v| {
                acc.map_or_else(|| Some(v.eval()), |acc| Some(acc.min(v.eval())))
            })
            .unwrap()
    }
}

impl_try_from_vec!(Minimum);

#[derive(Clone, Debug)]
pub(crate) struct Maximum {
    version: u8,
    operands: Vec<super::Packet>,
}

impl Operation for Maximum {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        self.operands
            .iter()
            .fold(None, |acc: Option<u64>, v| {
                acc.map_or_else(|| Some(v.eval()), |acc| Some(acc.max(v.eval())))
            })
            .unwrap()
    }
}

impl_try_from_vec!(Maximum);

#[derive(Clone, Debug)]
pub(crate) struct GreaterThan {
    version: u8,
    left: Box<super::Packet>,
    right: Box<super::Packet>,
}

impl Operation for GreaterThan {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        if self.left.eval() > self.right.eval() {
            1
        } else {
            0
        }
    }
}

impl_try_from_bin!(GreaterThan);

#[derive(Clone, Debug)]
pub(crate) struct LessThan {
    version: u8,
    left: Box<super::Packet>,
    right: Box<super::Packet>,
}

impl Operation for LessThan {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        if self.left.eval() < self.right.eval() {
            1
        } else {
            0
        }
    }
}

impl_try_from_bin!(LessThan);

#[derive(Clone, Debug)]
pub(crate) struct EqualTo {
    version: u8,
    left: Box<super::Packet>,
    right: Box<super::Packet>,
}

impl Operation for EqualTo {
    fn version(&self) -> u8 {
        self.version
    }

    fn eval(&self) -> u64 {
        if self.left.eval() == self.right.eval() {
            1
        } else {
            0
        }
    }
}

impl_try_from_bin!(EqualTo);
