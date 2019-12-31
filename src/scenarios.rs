pub struct Scenarios {
    counter: u64,
}

impl Scenarios {
    fn next(&mut self) {
        self.count += 1;
    }
}
