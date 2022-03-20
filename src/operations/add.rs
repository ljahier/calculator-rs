pub struct Add {
    nbrOne: i32,
    nbrTwo: i32,
}

impl Add {
    pub fn new(nbrOne: i32, nbrTwo: i32) -> Add {
        Add {
            nbrOne,
            nbrTwo,
        }
    }
    pub fn compute(&self) -> i32 {
        self.nbrOne + self.nbrTwo
    }
}