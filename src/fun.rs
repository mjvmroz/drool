pub trait CopyExtensions: Copy {
    fn post_mut(&mut self, eff: fn(&mut Self) -> ()) -> Self {
        let res: Self = *self;
        eff(self);
        res
    }
}

impl<A> CopyExtensions for A where A: Copy {}
