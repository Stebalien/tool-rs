pub struct Every {
    every: u32,
    progress: u32,
}

impl Every {
    pub fn plus(self, i: u32) -> Self {
        self.progress += i;
        self.progress %= self.every;
    }
}

pub fn every(n: u32) -> Every {
    Every {
        every: n,
        progress: 0,
    }
}

pub fn even() -> Every {
    every(2)
}

pub fn odd() -> Every {
    even().plus(1)
}
