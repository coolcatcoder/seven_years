#[cfg(test)]
extern crate std;
use core::fmt::Write;

macro_rules! test_with_features {
    attr($($features:literal),*) ($visibility:vis mod $module:ident {$($body:tt)*}) => {
        $(
            #[cfg(feature = $features)]
        )*
        $visibility mod $module {$($body)*}

        #[test]
        fn $module() {
            run_with_features(
                &std::format!("{}::main", stringify!($module)),
                [
                    "generic_const_arguments",
                    "type_info",
                    "ptr_metadata",
                    "maybe_uninit_array_assume_init",
                ],
            );
        }
    };
}

fn run_with_features<const LENGTH: usize>(module: &str, features: [&str; LENGTH]) {
    let mut features_as_string = std::string::String::new();
    for feature in features {
        write!(&mut features_as_string, "{feature},").expect("Writing to a String will work.");
    }
    let output = std::process::Command::new("cargo")
        .args(["test", module, "--features", &features_as_string])
        .output()
        .expect("The command will work.");

    let stderr = str::from_utf8(&output.stderr).expect("Valid utf8.");
    //panic!("START START START\n{stderr}\nEND END END");
    assert!(
        !(stderr.contains("error: test failed") || stderr.contains("error: could not compile"))
    );
}

#[test_with_features(
    "generic_const_arguments",
    "type_info",
    "ptr_metadata",
    "maybe_uninit_array_assume_init"
)]
mod fields {
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

        for _display in (0_i32, 1.0_f32, "Two.").for_each_field::<dyn Display>() {
            //println!("{display}");
        }
    }
}
