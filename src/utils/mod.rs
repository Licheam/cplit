pub trait Flag {
    fn set(&mut self, val: bool);
    fn get(&self) -> bool;
}

impl Flag for bool {
    fn set(&mut self, val: bool) {
        *self = val;
    }

    fn get(&self) -> bool {
        *self
    }
}
