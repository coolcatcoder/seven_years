use super::std;
use crate::fields::{AllFieldsImplement, IterateFields};
use core::fmt::{Debug, Display};

const fn tester<T: AllFieldsImplement<Trait>, Trait: 'static + ?Sized>() {}

#[test]
pub fn main() {
    tester::<(), dyn Send>();
    tester::<(i32, i32, [i32; 5]), dyn Sync>();
    // Does not compile.
    // tester::<(*const i32, f32), dyn Sync>();

    for _debug in 0.5_f64.for_each_field::<dyn Debug>() {
        //println!("{debug:?}");
    }

    let expected = ["0", "1", "Two."];
    for (index, display) in (0_i32, 1.0_f32, "Two.")
        .for_each_field::<dyn Display>()
        .into_iter()
        .enumerate()
    {
        assert_eq!(expected[index], std::format!("{display}"));
    }
}
