pub trait Checkable {
    fn check(&self);
}

impl<T> Checkable for Vec<T>
where
    T: Checkable,
{
    fn check(&self) {
        for t in self.iter() {
            t.check();
        }
    }
}

impl<T> Checkable for Option<T>
where
    T: Checkable,
{
    fn check(&self) {
        self.as_ref().map(|t| t.check());
    }
}

impl Checkable for String {
    fn check(&self) {
        // no-op. All Strings check just fine.
    }
}
