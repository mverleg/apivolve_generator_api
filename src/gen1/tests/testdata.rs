use ::semver::Version;
use ::smallvec::smallvec;
use ::tempdir::TempDir;

use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator};
use crate::gen1::evolution::{Identifier, IntWidth, Length, Signed, Typ};
use crate::gen1::evolution::IntType;
use crate::gen1::evolution::Message;
use crate::gen1::evolution::NamedType;
use crate::gen1::evolution::RecordType;
use crate::gen1::evolution::TextType;
use crate::gen1::tests::generate::test_with_data;

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
                    declarations: vec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                    messages: vec![],
                },
            ),
            (
                Version::new(0, 1, 0),
                Evolution {
                    declarations: vec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                    messages: vec![],
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
            declarations: vec![
                //TODO @mark: fields
                Message::new(RecordType::named(
                    Identifier::new("person").unwrap(),
                    vec![]
                )),
            ],
            messages: vec![],
        }),
        vec![
            (
                Version::new(0, 2, 0),
                Evolution {
                    declarations: vec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![
                                NamedType::new(Identifier::new("name").unwrap(), Typ::Text(TextType::new(Length::unknown()))),
                                NamedType::new(Identifier::new("balance").unwrap(), Typ::Int(IntType::new(IntWidth::Bit32, Signed::Signed))),
                            ]
                        )),
                    ],
                    messages: vec![],
                },
            ),
            (
                Version::new(0, 1, 0),
                Evolution {
                    declarations: vec![
                        //TODO @mark: fields
                        Message::new(RecordType::named(
                            Identifier::new("person").unwrap(),
                            vec![]
                        )),
                    ],
                    messages: vec![],
                },
            ),
        ],
    )
}
