use bitbuffer::BigEndian as NetworkEndian;

use bitbuffer::{BitError, BitRead, BitReadBuffer, BitReadStream, Endianness, Result};

pub(crate) mod packets;

const INPUT: &str = include_str!("../../../inputs/16");

#[derive(Clone, Debug)]
pub(crate) enum RawPacket {
    Operation {
        version: u8,
        _type: u8,
        children: Vec<RawPacket>,
    },
    Literal {
        version: u8,
        literal: u64,
    },
}

fn read_header<'a, E: Endianness>(stream: &mut BitReadStream<'a, E>) -> Result<(u8, u8)> {
    Ok((stream.read_int(3)?, stream.read_int(3)?))
}

fn read_literal_is_last<'a, E: Endianness>(stream: &mut BitReadStream<'a, E>) -> Result<bool> {
    Ok(if stream.read_int::<u8>(1)? == 1 {
        false
    } else {
        true
    })
}

fn read_literal<'a, E: Endianness>(stream: &mut BitReadStream<'a, E>) -> Result<u64> {
    let mut res = 0;

    while {
        let last = read_literal_is_last(stream)?;
        let next = stream.read_int::<u64>(4)?;
        res = (res << 4) | next;
        !last
    } {}

    Ok(res)
}

fn read_children<'a, E: Endianness>(stream: &mut BitReadStream<'a, E>) -> Result<Vec<RawPacket>> {
    if stream.read_int::<u8>(1)? == 0 {
        let len = stream.read_int::<usize>(15)?;
        let packets = read_all_packets(&mut stream.read_bits(len)?)?;
        Ok(packets)
    } else {
        let len = stream.read_int::<usize>(11)?;
        let mut packets = Vec::new();
        for _ in 0..len {
            packets.push(read_packet(stream)?);
        }
        Ok(packets)
    }
}

fn read_packet<'a, E: Endianness>(stream: &mut BitReadStream<'a, E>) -> Result<RawPacket> {
    let (version, _type) = read_header(stream)?;
    Ok(if _type == 4 {
        let literal = read_literal(stream)?;
        RawPacket::Literal { version, literal }
    } else {
        let children = read_children(stream)?;
        RawPacket::Operation {
            version,
            _type,
            children,
        }
    })
}

fn eop<'a, E: Endianness>(stream: &mut BitReadStream<'a, E>) -> Result<bool> {
    if stream.bits_left() >= 8 {
        return Ok(false);
    }
    let pos = stream.pos();

    while stream.bits_left() > 0 {
        if stream.read_int::<u8>(1)? == 1 {
            stream.set_pos(pos)?;
            return Ok(false);
        }
    }

    Ok(true)
}

fn read_all_packets<'a, E: Endianness>(
    stream: &mut BitReadStream<'a, E>,
) -> Result<Vec<RawPacket>> {
    let mut packets = Vec::new();

    while !eop(stream)? {
        let packet = stream.read()?;
        packets.push(packet);
    }

    Ok(packets)
}

impl RawPacket {
    pub fn version(&self) -> u8 {
        match self {
            RawPacket::Operation { version, .. } => *version,
            RawPacket::Literal { version, .. } => *version,
        }
    }

    pub fn get_type(&self) -> u8 {
        match self {
            RawPacket::Operation { _type, .. } => *_type,
            RawPacket::Literal { .. } => 4,
        }
    }

    #[cfg(test)]
    #[inline]
    pub fn is_literal(&self) -> bool {
        self.get_type() == 4
    }

    #[cfg(test)]
    #[inline]
    pub fn is_operation(&self) -> bool {
        !self.is_literal()
    }

    #[cfg(test)]
    pub fn literal(&self) -> Option<u64> {
        match self {
            RawPacket::Literal { literal, .. } => Some(*literal),
            _ => None,
        }
    }

    pub fn children(&self) -> Option<&Vec<RawPacket>> {
        match self {
            RawPacket::Operation { children, .. } => Some(children),
            RawPacket::Literal { .. } => None,
        }
    }
}

impl<'a, E: Endianness> BitRead<'a, E> for RawPacket {
    fn read(stream: &mut BitReadStream<'a, E>) -> Result<Self, BitError> {
        read_packet(stream)
    }
}

pub(crate) fn parse(text: &str) -> Vec<RawPacket> {
    let ints = text
        .trim()
        .as_bytes()
        .chunks(2)
        .map(|x| u8::from_str_radix(unsafe { std::str::from_utf8_unchecked(x) }, 16).unwrap())
        .collect::<Vec<u8>>();
    let buffer = BitReadBuffer::new(&ints, NetworkEndian);
    let mut stream = BitReadStream::new(buffer);
    read_all_packets(&mut stream).unwrap()
}

fn sum_raw_packet_versions(packet: &RawPacket) -> usize {
    (packet.version() as usize)
        + packet
            .children()
            .map(|children| sum_raw_packets_versions(children))
            .unwrap_or(0)
}

pub(crate) fn sum_raw_packets_versions(packets: &Vec<RawPacket>) -> usize {
    packets.iter().map(sum_raw_packet_versions).sum()
}

pub(crate) fn solution1(text: &str) -> usize {
    sum_raw_packets_versions(&parse(text))
}

#[derive(Clone, Debug)]
pub(crate) enum Packet {
    Sum(self::packets::Sum),
    Product(self::packets::Product),
    Minimum(self::packets::Minimum),
    Maximum(self::packets::Maximum),
    Literal(self::packets::Literal),
    GreaterThan(self::packets::GreaterThan),
    LessThan(self::packets::LessThan),
    EqualTo(self::packets::EqualTo),
}

impl Packet {
    pub fn eval(&self) -> u64 {
        match self {
            Self::Sum(op) => op.eval(),
            Self::Product(op) => op.eval(),
            Self::Minimum(op) => op.eval(),
            Self::Maximum(op) => op.eval(),
            Self::Literal(op) => op.eval(),
            Self::GreaterThan(op) => op.eval(),
            Self::LessThan(op) => op.eval(),
            Self::EqualTo(op) => op.eval(),
        }
    }
}

impl TryFrom<RawPacket> for Packet {
    type Error = ();

    fn try_from(raw: RawPacket) -> Result<Self, <Self as TryFrom<RawPacket>>::Error> {
        match raw.get_type() {
            0 => Ok(Packet::Sum(
                <self::packets::Sum as TryFrom<RawPacket>>::try_from(raw)?,
            )),
            1 => Ok(Packet::Product(<self::packets::Product as TryFrom<
                RawPacket,
            >>::try_from(raw)?)),
            2 => Ok(Packet::Minimum(<self::packets::Minimum as TryFrom<
                RawPacket,
            >>::try_from(raw)?)),
            3 => Ok(Packet::Maximum(<self::packets::Maximum as TryFrom<
                RawPacket,
            >>::try_from(raw)?)),
            4 => Ok(Packet::Literal(<self::packets::Literal as TryFrom<
                RawPacket,
            >>::try_from(raw)?)),
            5 => Ok(Packet::GreaterThan(
                <self::packets::GreaterThan as TryFrom<RawPacket>>::try_from(raw)?,
            )),
            6 => Ok(Packet::LessThan(<self::packets::LessThan as TryFrom<
                RawPacket,
            >>::try_from(raw)?)),
            7 => Ok(Packet::EqualTo(<self::packets::EqualTo as TryFrom<
                RawPacket,
            >>::try_from(raw)?)),
            _ => Err(()),
        }
    }
}

pub(crate) trait Operation {
    fn version(&self) -> u8;

    fn eval(&self) -> u64;
}

pub(crate) fn eval(text: &str) -> u64 {
    let packet: Packet = parse(text).remove(0).try_into().unwrap();
    packet.eval()
}

pub(crate) fn solution2(text: &str) -> u64 {
    eval(text)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod sixteen_tests {
    use super::{eval, parse, sum_raw_packets_versions};

    #[test]
    fn test1() {
        let packets = parse("D2FE28");
        assert_eq!(packets.len(), 1);
        let packet = &packets[0];
        assert!(packet.is_literal());
        assert_eq!(packet.version(), 6);
        assert_eq!(packet.literal(), Some(2021));
    }

    #[test]
    fn test2() {
        let packets = parse("38006F45291200");
        assert_eq!(packets.len(), 1);
        let packet = &packets[0];
        assert!(packet.is_operation());
        assert_eq!(packet.version(), 1);
        assert_eq!(packet.get_type(), 6);
        assert_eq!(packet.children().map(|c| c.len()), Some(2));
        let first = &packet.children().unwrap()[0];
        assert!(first.is_literal());
        assert_eq!(first.version(), 6);
        assert_eq!(first.literal(), Some(10));
        let second = &packet.children().unwrap()[1];
        assert!(second.is_literal());
        assert_eq!(second.version(), 2);
        assert_eq!(second.literal(), Some(20));
    }

    #[test]
    fn test3() {
        let packets = parse("EE00D40C823060");
        assert_eq!(packets.len(), 1);
        let packet = &packets[0];
        assert!(packet.is_operation());
        assert_eq!(packet.version(), 7);
        assert_eq!(packet.get_type(), 3);
        assert_eq!(packet.children().map(|x| x.len()), Some(3));
        let first = &packet.children().unwrap()[0];
        assert!(first.is_literal());
        assert_eq!(first.literal(), Some(1));
        let second = &packet.children().unwrap()[1];
        assert!(second.is_literal());
        assert_eq!(second.literal(), Some(2));
        let third = &packet.children().unwrap()[2];
        assert!(third.is_literal());
        assert_eq!(third.literal(), Some(3));
    }

    #[test]
    fn test4() {
        let packets = parse("8A004A801A8002F478");
        assert_eq!(packets.len(), 1);
        let packet = &packets[0];
        assert!(packet.is_operation());
        assert_eq!(packet.version(), 4);
        assert_eq!(packet.children().map(|c| c.len()), Some(1));
        let packet = &packet.children().unwrap()[0];
        assert!(packet.is_operation());
        assert_eq!(packet.version(), 1);
        assert_eq!(packet.children().map(|c| c.len()), Some(1));
        let packet = &packet.children().unwrap()[0];
        assert!(packet.is_operation());
        assert_eq!(packet.version(), 5);
        assert_eq!(packet.children().map(|c| c.len()), Some(1));
        let packet = &packet.children().unwrap()[0];
        assert!(packet.is_literal());
        assert_eq!(packet.version(), 6);
        assert_eq!(sum_raw_packets_versions(&packets), 16);
    }

    #[test]
    fn test5() {
        let packets = parse("620080001611562C8802118E34");
        assert_eq!(packets.len(), 1);
        let packet = &packets[0];
        assert!(packet.is_operation());
        assert_eq!(packet.children().map(|c| c.len()), Some(2));
        for packet in packet.children().unwrap() {
            assert!(packet.is_operation());
            assert_eq!(packet.children().map(|c| c.len()), Some(2));
            for packet in packet.children().unwrap() {
                assert!(packet.is_literal());
            }
        }
        assert_eq!(sum_raw_packets_versions(&packets), 12);
    }

    #[test]
    fn test6() {
        let packets = parse("C0015000016115A2E0802F182340");
        assert_eq!(packets.len(), 1);
        let packet = &packets[0];
        assert!(packet.is_operation());
        assert_eq!(packet.children().map(|c| c.len()), Some(2));
        for packet in packet.children().unwrap() {
            assert!(packet.is_operation());
            assert_eq!(packet.children().map(|c| c.len()), Some(2));
            for packet in packet.children().unwrap() {
                assert!(packet.is_literal());
            }
        }
        assert_eq!(sum_raw_packets_versions(&packets), 23);
    }

    #[test]
    fn test7() {
        let packets = parse("A0016C880162017C3686B18A3D4780");
        assert_eq!(packets.len(), 1);

        let packet = &packets[0];
        assert!(packet.is_operation());
        assert_eq!(packet.children().map(|c| c.len()), Some(1));

        let packet = &packet.children().unwrap()[0];
        assert!(packet.is_operation());
        assert_eq!(packet.children().map(|c| c.len()), Some(1));

        let packet = &packet.children().unwrap()[0];
        assert!(packet.is_operation());
        assert_eq!(packet.children().map(|c| c.len()), Some(5));

        for packet in packet.children().unwrap() {
            assert!(packet.is_literal());
        }

        assert_eq!(sum_raw_packets_versions(&packets), 31);
    }

    #[test]
    fn test8() {
        assert_eq!(eval("C200B40A82"), 3);
    }

    #[test]
    fn test9() {
        assert_eq!(eval("04005AC33890"), 54);
    }

    #[test]
    fn test10() {
        assert_eq!(eval("880086C3E88112"), 7);
    }

    #[test]
    fn test11() {
        assert_eq!(eval("CE00C43D881120"), 9);
    }

    #[test]
    fn test12() {
        assert_eq!(eval("D8005AC2A8F0"), 1);
    }

    #[test]
    fn test13() {
        assert_eq!(eval("F600BC2D8F"), 0);
    }

    #[test]
    fn test14() {
        assert_eq!(eval("9C005AC2F8F0"), 0);
    }

    #[test]
    fn test15() {
        assert_eq!(eval("9C0141080250320F1802104A08"), 1);
    }
}
