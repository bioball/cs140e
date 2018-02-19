// FIXME: Make me pass! Diff budget: 30 lines.

#[derive(Clone)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    pub fn default() -> Box<Builder> {
        Box::new(Builder {
            string: None,
            number: None,
        })
    }

    pub fn string<'a, S: Into<String>>(&'a mut self, value: S) -> &'a mut Self {
        self.string = Some(String::from(value.into()));
        self
    }

    pub fn number<'a>(&'a mut self, value: usize) -> &'a mut Self {
        self.number = Some(value);
        self
    }

    pub fn to_string<'a>(&'a mut self) -> String {
        let mut list = Vec::new();
        if let Some(st) = self.clone().string {
            list.push(st);
        }
        if let Some(num) = self.number {
            list.push(num.to_string());
        }
        list.join(" ")
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
