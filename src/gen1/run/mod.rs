mod protocol;
mod gen_trait;
mod run;

pub use self::gen_trait::ErrMsg;
pub use self::gen_trait::GenResult;
pub use self::gen_trait::Generator;
pub use self::protocol::GeneratorProtocol;
pub use self::run::run_generator;
