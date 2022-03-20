pub struct Divide {
    nbrOne: i32,
    nbrTwo: i32,
}

impl Divide {
    fn compute(&self) -> i32 {
        self.nbrOne / self.nbrTwo
    }
}