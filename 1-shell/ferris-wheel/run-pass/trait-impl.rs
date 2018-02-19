// FIXME: Make me pass! Diff budget: 25 lines.

enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

#[derive(Debug, PartialEq)]
struct Seconds(u32);
#[derive(Debug, PartialEq)]
struct Minutes(u16);
#[derive(Debug, PartialEq)]
struct MilliSeconds(u64);

impl PartialEq<Minutes> for Seconds {
    fn eq(&self, other: &Minutes) -> bool {
        self.0 == (other.0 as u32) * 60
    }
}

impl Eq for Seconds {}

impl PartialEq<Minutes> for MilliSeconds {
    fn eq(&self, other: &Minutes) -> bool {
        self.0 == (other.0 as u64) * 60000
    }
}

impl PartialEq<Seconds> for MilliSeconds {
    fn eq(&self, other: &Seconds) -> bool {
        self.0 == (other.0 as u64) * 1000
    }
}

impl Eq for MilliSeconds {}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
