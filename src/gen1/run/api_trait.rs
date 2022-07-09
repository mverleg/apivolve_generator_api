use crate::gen1::AcceptedFormat;
use crate::gen1::Generator;
use crate::gen1::UserPreferences;
use crate::gen1::FunctionalityRequest;

/// The data types T and U can be used to transfer data between steps,
/// in case you prefer that to mutating the api struct itself.
pub trait GeneratorApi<G: Generator, T, U> {
    fn accepts(&mut self) -> Result<(AcceptedFormat, T), String>;

    fn features(&mut self, user_pref: UserPreferences, data: T) -> Result<(FunctionalityRequest, U), String>;

    fn make_generator(self, data: U) -> Result<G, String>;
}

//TODO @mark: add a way to do warnings?
