pub struct With<C, T> {
    pub ctx: C,
    pub item: T,
}

impl<C, T> With<C, T> {
    pub fn new(ctx: C, item: T) -> With<C, T> {
        With { ctx, item }
    }

    pub fn with_item<U>(self, item: U) -> With<C, U> {
        With::new(self.ctx, item)
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> With<C, U> {
        With::new(self.ctx, f(self.item))
    }

    pub fn as_ref(&self) -> With<C, &T>
    where
        C: Copy,
    {
        With::new(self.ctx, &self.item)
    }
}
