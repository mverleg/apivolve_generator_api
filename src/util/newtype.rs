
//TODO @mark: numbers
//TODO @mark: non-negative?
//TODO @mark: option
//TODO @mark: vec

#[macro_export]
macro_rules! newtype_option {
    ($option_name: ident, $some_name: ident, $none_name: ident, $invariants_assertions: stmt) => {
        #[derive(Debug)]
        pub enum $option_name<T> {
            $some_name(T),
            $none_name,
        }
    };
    ($option_name: ident, $some_name: ident, $none_name: ident) => {
        newtype_option!($option_name, $some_name, $none_name, {});
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    newtype_option!(Feature, Enabled, Disabled);

    #[test]
    fn test_option() {
        let feature = Feature::Enabled(1);
        assert!(feature.is_enabled());
    }
}
