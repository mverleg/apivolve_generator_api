use ::semver::Version;
use ::smallvec::smallvec;
use ::tempdir::TempDir;

use crate::gen1::evolution::Identifier;
use crate::gen1::evolution::Message;
use crate::gen1::evolution::RecordType;
use crate::gen1::tests::generate::test_with_data;
use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator};

pub fn generate_no_versions<G, GenFn>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String>
where
    G: Generator,
    GenFn: FnOnce(GenerationPreferences) -> Result<G, String>,
{
    test_with_data(accepts_config, make_generator, None, vec![])
}

pub fn generate_core_features<
    G: Generator,
    GenFn: FnOnce(GenerationPreferences) -> Result<G, String>,
>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String> {
    test_with_data(
        accepts_config,
        make_generator,
        None,
        vec![
            (
                Version::new(0, 2, 0),
                Evolution {
                    messages: smallvec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                },
            ),
            (
                Version::new(0, 1, 0),
                Evolution {
                    messages: smallvec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                },
            ),
        ],
    )
}

pub fn generate_with_pending<
    G: Generator,
    GenFn: FnOnce(GenerationPreferences) -> Result<G, String>,
>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String> {
    test_with_data(
        accepts_config,
        make_generator,
        Some(Evolution {
            messages: smallvec![
                //TODO @mark: fields
                Message::new(RecordType::named(
                    Identifier::new("person").unwrap(),
                    vec![]
                )),
            ],
        }),
        vec![
            (
                Version::new(0, 2, 0),
                Evolution {
                    messages: smallvec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                },
            ),
            (
                Version::new(0, 1, 0),
                Evolution {
                    messages: smallvec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                },
            ),
        ],
    )
}
