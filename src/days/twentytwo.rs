use scan_fmt::scan_fmt;
use std::{
    mem::take,
    ops::{Add, AddAssign, BitAnd, Sub, SubAssign},
};

const INPUT: &str = include_str!("../../inputs/22");

type N = isize;
type Coord = (N, N, N);

fn range_overlaps(start1: N, end1: N, start2: N, end2: N) -> bool {
    end1 >= start2 && end2 >= start1
}

fn range_bitand(start1: N, end1: N, start2: N, end2: N) -> Option<(N, N)> {
    if range_overlaps(start1, end1, start2, end2) {
        Some((start1.max(start2), end1.min(end2)))
    } else {
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Cuboid {
    pub x1: N,
    pub y1: N,
    pub z1: N,
    pub x2: N,
    pub y2: N,
    pub z2: N,
}

impl Cuboid {
    #[allow(dead_code)]
    pub fn overlaps(&self, other: &Self) -> bool {
        range_overlaps(self.x1, self.x2, other.x1, other.x2)
            && range_overlaps(self.y1, self.y2, other.y1, other.y2)
            && range_overlaps(self.z1, self.z2, other.z1, other.z2)
    }

    #[allow(dead_code)]
    pub fn iter<'a>(&'a self) -> CuboidIter<'a> {
        CuboidIter::new(&self)
    }

    pub fn len(&self) -> usize {
        ((self.x2 - self.x1) as usize + 1)
            * ((self.y2 - self.y1) as usize + 1)
            * ((self.z2 - self.z1) as usize + 1)
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub(crate) struct CuboidIter<'a> {
    cuboid: &'a Cuboid,
    current: Option<Coord>,
}

fn iter_next(cuboid: &Cuboid, ocurrent: &mut Option<Coord>) -> Option<Coord> {
    if let Some(current) = take(ocurrent) {
        *ocurrent = if current.2 == cuboid.z2 {
            if current.1 == cuboid.y2 {
                if current.0 == cuboid.x2 {
                    None
                } else {
                    Some((current.0 + 1, cuboid.y1, cuboid.z1))
                }
            } else {
                Some((current.0, current.1 + 1, cuboid.z1))
            }
        } else {
            Some((current.0, current.1, current.2 + 1))
        };

        Some(current)
    } else {
        None
    }
}

impl<'a> CuboidIter<'a> {
    pub fn new(cuboid: &'a Cuboid) -> Self {
        Self {
            cuboid,
            current: Some((cuboid.x1, cuboid.y1, cuboid.z1)),
        }
    }
}

impl<'a> Iterator for CuboidIter<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        iter_next(&self.cuboid, &mut self.current)
    }
}

pub(crate) struct CuboidIntoIter {
    cuboid: Cuboid,
    current: Option<Coord>,
}

impl CuboidIntoIter {
    pub fn new(cuboid: Cuboid) -> Self {
        Self {
            cuboid,
            current: Some((cuboid.x1, cuboid.y1, cuboid.z1)),
        }
    }
}

impl Iterator for CuboidIntoIter {
    type Item = Coord;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        iter_next(&self.cuboid, &mut self.current)
    }
}

impl IntoIterator for Cuboid {
    type Item = Coord;
    type IntoIter = CuboidIntoIter;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        CuboidIntoIter::new(self)
    }
}

impl BitAnd for &Cuboid {
    type Output = Option<Cuboid>;

    fn bitand(self, other: Self) -> <Self as BitAnd<Self>>::Output {
        let (x1, x2) = range_bitand(self.x1, self.x2, other.x1, other.x2)?;
        let (y1, y2) = range_bitand(self.y1, self.y2, other.y1, other.y2)?;
        let (z1, z2) = range_bitand(self.z1, self.z2, other.z1, other.z2)?;

        Some(Cuboid {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        })
    }
}

impl BitAnd for Cuboid {
    type Output = Option<Self>;

    fn bitand(self, other: Self) -> <Self as BitAnd<Self>>::Output {
        BitAnd::bitand(&self, &other)
    }
}

fn merge_cuboids(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    let same_x = c1.x1 == c2.x1 && c1.x2 == c2.x2;
    let same_y = c1.y1 == c2.y1 && c1.y2 == c2.y2;
    let same_z = c1.z1 == c2.z1 && c1.z2 == c2.z2;

    if same_x
        && same_y
        // overlaps or contiguos
        && (range_overlaps(c1.z1, c1.z2, c2.z1, c2.z2) || c1.z2 + 1 == c2.z1 || c2.z2 + 1 == c1.z1)
    {
        Some(Cuboid {
            x1: c1.x1,
            x2: c1.x2,
            y1: c1.y1,
            y2: c1.y2,
            z1: c1.z1.min(c2.z1),
            z2: c1.z2.max(c2.z2),
        })
    } else if same_x
        && same_z
        && (range_overlaps(c1.y1, c1.y2, c2.y1, c2.y2) || c1.y2 + 1 == c2.y1 || c2.y2 + 1 == c1.y1)
    {
        Some(Cuboid {
            x1: c1.x1,
            x2: c1.x2,
            y1: c1.y1.min(c2.y1),
            y2: c1.y2.max(c2.y2),
            z1: c1.z1,
            z2: c1.z2,
        })
    } else if same_y
        && same_z
        && (range_overlaps(c1.x1, c1.x2, c2.x1, c2.x2) || c1.x2 + 1 == c2.x1 || c2.x2 + 1 == c1.x1)
    {
        Some(Cuboid {
            x1: c1.x1.min(c2.x1),
            x2: c1.x2.max(c2.x2),
            y1: c1.y1,
            y2: c1.y2,
            z1: c1.z1,
            z2: c1.z2,
        })
    } else {
        None
    }
}

fn sub_cuboids(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
    let mut slices = Vec::new();
    if let Some(b) = a & b {
        if a.y1 != b.y1 {
            slices.push(Cuboid {
                x1: a.x1,
                x2: a.x2,
                y1: a.y1,
                y2: b.y1 - 1,
                z1: a.z1,
                z2: a.z2,
            });
        }

        if a.x1 != b.x1 {
            slices.push(Cuboid {
                x1: a.x1,
                x2: b.x1 - 1,
                y1: b.y1,
                y2: b.y2,
                z1: a.z1,
                z2: a.z2,
            });
        }

        if a.x2 != b.x2 {
            slices.push(Cuboid {
                x1: b.x2 + 1,
                x2: a.x2,
                y1: b.y1,
                y2: b.y2,
                z1: a.z1,
                z2: a.z2,
            });
        }

        if a.y2 != b.y2 {
            slices.push(Cuboid {
                x1: a.x1,
                x2: a.x2,
                y1: b.y2 + 1,
                y2: a.y2,
                z1: a.z1,
                z2: a.z2,
            });
        }

        if a.z1 != b.z1 {
            slices.push(Cuboid {
                x1: b.x1,
                x2: b.x2,
                y1: b.y1,
                y2: b.y2,
                z1: a.z1,
                z2: b.z1 - 1,
            });
        }

        if a.z2 != b.z2 {
            slices.push(Cuboid {
                x1: b.x1,
                x2: b.x2,
                y1: b.y1,
                y2: b.y2,
                z1: b.z2 + 1,
                z2: a.z2,
            });
        }
    } else {
        slices.push(*a);
    }

    slices
}

fn step_reduce_cuboids(cuboids: &mut Vec<Cuboid>) -> bool {
    if cuboids.len() < 2 {
        return false;
    }

    for i in 0..(cuboids.len() - 1) {
        for j in (i + 1)..cuboids.len() {
            if i != j {
                if let Some(c) = merge_cuboids(&cuboids[i], &cuboids[j]) {
                    cuboids.remove(i);
                    cuboids.remove(if j > i { j - 1 } else { j });
                    cuboids.push(c);

                    return true;
                } else if cuboids[i].overlaps(&cuboids[j]) {
                    let (a, b) = if i < j {
                        (cuboids.remove(i), cuboids.remove(j - 1))
                    } else {
                        (cuboids.remove(i), cuboids.remove(j))
                    };

                    let res = sub_cuboids(&b, &a);
                    cuboids.push(a);
                    cuboids.extend(res);

                    return true;
                }
            }
        }
    }

    false
}

#[allow(dead_code)]
fn reduce_cuboids(cuboids: &mut Vec<Cuboid>) {
    while step_reduce_cuboids(cuboids) {}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SpaceSlice {
    slices: Vec<Cuboid>,
}

impl SpaceSlice {
    pub fn new() -> Self {
        Self { slices: Vec::new() }
    }

    pub fn cuboids(&self) -> &Vec<Cuboid> {
        &self.slices
    }

    pub fn cuboids_mut(&mut self) -> &mut Vec<Cuboid> {
        &mut self.slices
    }

    #[inline]
    pub fn into_cuboids(self) -> Vec<Cuboid> {
        self.slices
    }

    #[inline]
    pub fn len(&self) -> usize {
        raw_len(self.cuboids())
    }

    #[allow(dead_code)]
    #[inline]
    pub fn is_empty(&self) -> bool {
        raw_is_empty(self.cuboids())
    }

    #[allow(dead_code)]
    #[inline]
    pub fn reduce(&mut self) {
        reduce_cuboids(&mut self.slices)
    }
}

fn raw_sub_single(source: &mut Vec<Cuboid>, rem: &Cuboid) {
    let len = source.len();
    for _ in 0..len {
        let a = source.remove(0);
        source.extend(sub_cuboids(&a, rem));
    }
}

fn raw_sub(source: &mut Vec<Cuboid>, rem: &Vec<Cuboid>) {
    for cuboid in rem {
        let len = source.len();
        for _ in 0..len {
            let a = source.remove(0);
            source.extend(sub_cuboids(&a, cuboid));
        }
    }
}

fn raw_add_single(source: &mut Vec<Cuboid>, add: Cuboid) {
    let mut a = vec![add];
    raw_sub(&mut a, source);
    if !raw_is_empty(&a) {
        source.extend(a);
    }
}

fn raw_add(source: &mut Vec<Cuboid>, mut add: Vec<Cuboid>) {
    raw_sub(&mut add, source);
    if !raw_is_empty(&add) {
        source.extend(add);
    }
}

fn raw_len(slices: &Vec<Cuboid>) -> usize {
    slices.iter().map(Cuboid::len).sum()
}

fn raw_is_empty(slices: &Vec<Cuboid>) -> bool {
    raw_len(slices) == 0
}

impl AddAssign<Cuboid> for SpaceSlice {
    fn add_assign(&mut self, cuboid: Cuboid) {
        raw_add_single(self.cuboids_mut(), cuboid)
    }
}

impl AddAssign<&Cuboid> for SpaceSlice {
    #[inline]
    fn add_assign(&mut self, cuboid: &Cuboid) {
        self.add_assign(*cuboid)
    }
}

impl AddAssign<&Self> for SpaceSlice {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.add_assign(other.clone())
    }
}

impl AddAssign<Self> for SpaceSlice {
    fn add_assign(&mut self, other: Self) {
        raw_add(self.cuboids_mut(), other.into_cuboids())
    }
}

impl Add<Cuboid> for SpaceSlice {
    type Output = Self;

    fn add(self, cuboid: Cuboid) -> <Self as Add<Cuboid>>::Output {
        self.add(&cuboid)
    }
}

impl Add<&Cuboid> for SpaceSlice {
    type Output = Self;

    fn add(mut self, cuboid: &Cuboid) -> <Self as Add<&Cuboid>>::Output {
        self.add_assign(cuboid);
        self
    }
}

impl Add<&Self> for SpaceSlice {
    type Output = Self;

    fn add(mut self, other: &Self) -> <Self as Add<&Self>>::Output {
        self.add_assign(other);
        self
    }
}

impl Add<Self> for SpaceSlice {
    type Output = Self;

    fn add(mut self, other: Self) -> <Self as Add<Self>>::Output {
        self.add_assign(other);
        self
    }
}

impl SubAssign<Cuboid> for SpaceSlice {
    fn sub_assign(&mut self, cuboid: Cuboid) {
        raw_sub_single(&mut self.slices, &cuboid);
    }
}

impl SubAssign<Self> for SpaceSlice {
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other)
    }
}

impl SubAssign<&Self> for SpaceSlice {
    fn sub_assign(&mut self, other: &Self) {
        raw_sub(&mut self.slices, &other.slices);
    }
}

impl Sub<Cuboid> for SpaceSlice {
    type Output = Self;

    fn sub(mut self, cuboid: Cuboid) -> <Self as Sub<Cuboid>>::Output {
        self.sub_assign(cuboid);
        self
    }
}

impl Sub<Self> for SpaceSlice {
    type Output = Self;

    fn sub(mut self, other: Self) -> <Self as Sub<Self>>::Output {
        self.sub_assign(other);
        self
    }
}

impl Sub<Self> for &SpaceSlice {
    type Output = SpaceSlice;

    fn sub(self, other: Self) -> <Self as Sub<Self>>::Output {
        let mut me = self.clone();
        me.sub_assign(other);
        me
    }
}

impl Sub<&Self> for SpaceSlice {
    type Output = Self;

    fn sub(mut self, other: &Self) -> <Self as Sub<Self>>::Output {
        self.sub_assign(other);
        self
    }
}

impl Add<Self> for Cuboid {
    type Output = SpaceSlice;

    fn add(self, other: Cuboid) -> <Self as Add<Self>>::Output {
        SpaceSlice {
            slices: vec![self, other],
        }
    }
}

impl Add<SpaceSlice> for Cuboid {
    type Output = SpaceSlice;

    fn add(self, mut slice: SpaceSlice) -> <Self as Add<SpaceSlice>>::Output {
        slice.add_assign(self);
        slice
    }
}

impl Sub<Self> for Cuboid {
    type Output = SpaceSlice;

    fn sub(self, other: Self) -> <Self as Sub<Self>>::Output {
        SpaceSlice {
            slices: sub_cuboids(&self, &other),
        }
    }
}

impl Sub<SpaceSlice> for Cuboid {
    type Output = SpaceSlice;

    fn sub(self, slice: SpaceSlice) -> <Self as Sub<SpaceSlice>>::Output {
        let mut me: SpaceSlice = self.into();
        me.sub_assign(slice);
        me
    }
}

impl Sub<&SpaceSlice> for Cuboid {
    type Output = SpaceSlice;

    fn sub(self, slice: &SpaceSlice) -> <Self as Sub<&SpaceSlice>>::Output {
        let mut me: SpaceSlice = self.into();
        me.sub_assign(slice);
        me
    }
}

impl Sub<&SpaceSlice> for &Cuboid {
    type Output = SpaceSlice;

    fn sub(self, slice: &SpaceSlice) -> <Self as Sub<&SpaceSlice>>::Output {
        let mut me: SpaceSlice = self.into();
        me.sub_assign(slice);
        me
    }
}

impl Into<SpaceSlice> for Cuboid {
    fn into(self) -> SpaceSlice {
        SpaceSlice { slices: vec![self] }
    }
}

impl Into<SpaceSlice> for &Cuboid {
    fn into(self) -> SpaceSlice {
        SpaceSlice {
            slices: vec![*self],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    On(Cuboid),
    Off(Cuboid),
}

impl Operation {
    pub fn cuboid(&self) -> Cuboid {
        *match self {
            Self::On(c) => c,
            Self::Off(c) => c,
        }
    }
    pub fn is_on(&self) -> bool {
        match self {
            Self::On(_) => true,
            _ => false,
        }
    }

    pub fn apply(&self, slice: &mut SpaceSlice) {
        if self.is_on() {
            slice.add_assign(self.cuboid())
        } else {
            slice.sub_assign(self.cuboid())
        }
    }
}

fn parse_line(text: &str) -> Operation {
    let text = text.trim();

    if let Ok((x1, x2, y1, y2, z1, z2)) =
        scan_fmt!(text, "on x={}..{},y={}..{},z={}..{}", N, N, N, N, N, N)
    {
        Operation::On(Cuboid {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        })
    } else if let Ok((x1, x2, y1, y2, z1, z2)) =
        scan_fmt!(text, "off x={}..{},y={}..{},z={}..{}", N, N, N, N, N, N)
    {
        Operation::Off(Cuboid {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        })
    } else {
        unreachable!()
    }
}

fn parse(text: &str) -> Vec<Operation> {
    text.trim().lines().map(|l| parse_line(l)).collect()
}

pub(crate) fn solve(text: &str, limits: Option<Cuboid>) -> usize {
    let mut operations: Vec<Operation> = parse(text);
    if let Some(limits) = limits {
        operations = operations
            .into_iter()
            .filter_map(|op| {
                (op.cuboid() & limits).map(|c| {
                    if op.is_on() {
                        Operation::On(c)
                    } else {
                        Operation::Off(c)
                    }
                })
            })
            .collect();
    }
    operations
        .into_iter()
        .fold(SpaceSlice::new(), |mut acc, op| {
            op.apply(&mut acc);
            acc
        })
        .len()
}

pub(crate) fn solution1(text: &str) -> usize {
    solve(
        text,
        Some(Cuboid {
            x1: -50,
            x2: 50,
            y1: -50,
            y2: 50,
            z1: -50,
            z2: 50,
        }),
    )
}

pub(crate) fn solution2(text: &str) -> usize {
    solve(text, None)
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}

#[cfg(test)]
mod twentytwo_tests {
    use super::{solution1, solution2};

    const INPUT1: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const INPUT2: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    const INPUT3: &str = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn test1() {
        assert_eq!(solution1(INPUT1), 39);
    }

    #[test]
    fn test2() {
        assert_eq!(solution1(INPUT2), 590784);
    }

    #[test]
    fn test3() {
        assert_eq!(solution2(INPUT3), 2758514936282235);
    }
}
