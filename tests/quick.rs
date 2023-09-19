macro_rules! quick {
    ($value:expr) => {
        mod inner {
            #[benchy::benchmark([("", 1), ("", 2)])]
            fn parametrized(_b: &mut benchy::BenchmarkRun, p: usize) {
                if p == 2 {
                    panic!("Reached parameter 2");
                }
            }

            benchy::main!(
                parametrized,
                config = benchy::BenchmarkConfig {
                    quick: $value,
                    ..Default::default()
                }
            );

            pub fn call_main() {
                main();
            }
        }

        inner::call_main();
    };
}

#[test]
fn quick_true() {
    quick!(true);
}

#[test]
#[should_panic]
fn quick_false() {
    quick!(false);
}
