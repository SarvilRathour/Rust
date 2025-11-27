use std::ops::Deref;
impl<T> Deref for Mybox<T>{
    type Target =T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct Mybox<T>(T);
impl<T> Mybox<T>{
    fn new(x:T)->Mybox<T>{
        Mybox(x)
    }
}
fn main() {
    let x=5;
    let y=Mybox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*(y.deref()));
}

