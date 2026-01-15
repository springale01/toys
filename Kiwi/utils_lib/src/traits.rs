// I have no idea what to put in here yet

pub trait FlipSign {
    fn flip_sign(&mut self) -> &mut Self;
}

pub trait Round {
    fn round_to(&mut self, digits: usize) -> &mut Self;
}
