
#[allow(unused_macros)] // it is not unused, don't know why it's not being detected
macro_rules! make_gen_test {
    ($test_name: ident, $gen_test_ident: ident, $accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        #[test]
        fn $test_name() {
            #[inline]
            fn make_generator_expr_type_must_satisfy_this_signature<G, F>(make_generator: F) -> F
                    where G: Generator, F: FnOnce(GenerationPreferences) -> Result<G, String> {
                make_generator
            }

            #[inline]
            fn verify_func_ident_type_must_satisfy_this_signature<F>(verify_func: F) -> F
                    where F: FnOnce(TempDir) -> GenResult {
                verify_func
            }

            let accepts_config: AcceptsConfig = $accepts_config_expr;
            let make_generator = make_generator_expr_type_must_satisfy_this_signature($make_generator_expr);
            let verify_func = verify_func_ident_type_must_satisfy_this_signature($verify_func_ident);
            match $gen_test_ident(accepts_config, make_generator) {
                Ok(path) => if let Err(err) = verify_func(path) {
                    panic!("apivolve generator output was not valid: {}", err)
                },
                Err(err) => panic!("apivolve generator failed: {}", err),
            };
        }
    };
}

pub(crate) use make_gen_test;
