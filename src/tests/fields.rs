use super::std;
use crate::fields::{AllFieldsImplement, FieldsToArray};
use core::fmt::{Debug, Display};

#[allow(dead_code)]
trait Has {
    fn do_something(&self);
}
impl<T: AllFieldsImplement<dyn Has>> Has for T {
    fn do_something(&self) {
        for has in self.fields_to_array_ref::<dyn Has>() {
            has.do_something();
        }
    }
}

const fn tester<T: AllFieldsImplement<Trait>, Trait: 'static + ?Sized>() {}

#[test]
pub fn main() {
    tester::<(), dyn Send>();
    tester::<(i32, i32, [i32; 5]), dyn Sync>();
    // Does not compile.
    // tester::<(*const i32, f32), dyn Sync>();

    for _debug in 0.5_f64.fields_to_array_ref::<dyn Debug>() {
        //println!("{debug:?}");
    }

    let expected = ["0", "1", "Two."];
    for (index, display) in (0_i32, 1.0_f32, "Two.")
        .fields_to_array_ref::<dyn Display>()
        .into_iter()
        .enumerate()
    {
        assert_eq!(expected[index], std::format!("{display}"));
    }
}
