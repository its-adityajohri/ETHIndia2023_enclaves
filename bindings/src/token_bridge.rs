pub use token_bridge::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod token_bridge {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_inbox"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Inbox"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_outbox"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Outbox"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bridgeToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bridgeToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("crossTokenAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("crossTokenAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("inbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inbox"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Inbox"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("outbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("outbox"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Outbox"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("releaseToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("releaseToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCrossTokenAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setCrossTokenAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sourceTokenAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetTokenAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TOKENBRIDGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t58\x03\x80a\t5\x839\x81\x01`@\x81\x90Ra\0/\x91a\0xV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\0\x80T\x92\x90\x93\x16\x91\x16\x17\x90Ua\0\xB2V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0uW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\0\x8BW`\0\x80\xFD[\x82Qa\0\x96\x81a\0`V[` \x84\x01Q\x90\x92Pa\0\xA7\x81a\0`V[\x80\x91PP\x92P\x92\x90PV[a\x08t\x80a\0\xC1`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x07[,\x0C\x14a\0gW\x80c\x19\t\xE8\x06\x14a\0|W\x80cN\x83\x04D\x14a\0\x8FW\x80c\xAE\x02\x88\n\x14a\0\xA2W\x80c\xCE\x11\xE6\xAB\x14a\0\xF2W\x80c\xFB\x0Er+\x14a\x01\x05W[`\0\x80\xFD[a\0za\0u6`\x04a\x04\xEBV[a\x01\x18V[\0[a\0za\0\x8A6`\x04a\x05%V[a\x02\xA2V[a\0za\0\x9D6`\x04a\x05mV[a\x04HV[a\0\xD6a\0\xB06`\x04a\x05\xAFV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xD6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\0\xD6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`@Qc\xDA\x062\xAD`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x062\xAD\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\x91\x91\x90\x81\x01\x90a\x06\x88V[\x90P`\0\x80`\0\x83`\x80\x01Q\x80` \x01\x90Q\x81\x01\x90a\x01\xB0\x91\x90a\x07\x90V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x163\x14a\x02&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FYou are not the recipient of the`D\x82\x01Rhse tokens`\xB8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x99\x91\x90a\x07\xD3V[PPPPPPPV[`\0\x84\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x85R\x92R\x90\x91 T\x16a\x03\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid token for target chain\0\0`D\x82\x01R`d\x01a\x02\x1DV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8A\x91\x90a\x07\xD3V[P`\0\x84\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x85R\x90\x83R\x81\x84 T\x82Q\x90\x82\x16\x93\x81\x01\x93\x90\x93R\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0TcO\x9Er\xAD`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9F<\xE5Z\x90a\x04\x0F\x90\x86\x90\x89\x90\x86\x90`\x04\x01a\x07\xF5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04)W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04=W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x85R\x92R\x90\x91 T\x16\x15a\x04\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10Y\x19\x1C\x99\\\xDC\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]`j\x1B`D\x82\x01R`d\x01a\x02\x1DV[`\0\x92\x83R`\x02` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x86R\x90\x91R\x90\x92 \x80T\x91\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90UV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xFEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\"W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05;W`\0\x80\xFD[\x845\x93P` \x85\x015a\x05M\x81a\x05\rV[\x92P`@\x85\x015a\x05]\x81a\x05\rV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\x82W`\0\x80\xFD[\x835\x92P` \x84\x015a\x05\x94\x81a\x05\rV[\x91P`@\x84\x015a\x05\xA4\x81a\x05\rV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x05\xC2W`\0\x80\xFD[\x825\x91P` \x83\x015a\x05\xD4\x81a\x05\rV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x18Wa\x06\x18a\x05\xDFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06GWa\x06Ga\x05\xDFV[`@R\x91\x90PV[`\0[\x83\x81\x10\x15a\x06jW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06RV[PP`\0\x91\x01RV[\x80Q\x80\x15\x15\x81\x14a\x06\x83W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x06\x9BW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xB3W`\0\x80\xFD[\x90\x84\x01\x90`\xC0\x82\x87\x03\x12\x15a\x06\xC7W`\0\x80\xFD[a\x06\xCFa\x05\xF5V[\x82Q\x81R\x83\x83\x01Q\x84\x82\x01R`@\x83\x01Qa\x06\xE9\x81a\x05\rV[`@\x82\x01R``\x83\x01Qa\x06\xFC\x81a\x05\rV[``\x82\x01R`\x80\x83\x01Q\x82\x81\x11\x15a\x07\x13W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x88\x13a\x07$W`\0\x80\xFD[\x80Q\x83\x81\x11\x15a\x076Wa\x076a\x05\xDFV[a\x07H`\x1F\x82\x01`\x1F\x19\x16\x87\x01a\x06\x1EV[\x93P\x80\x84R\x88\x86\x82\x84\x01\x01\x11\x15a\x07^W`\0\x80\xFD[a\x07m\x81\x87\x86\x01\x88\x85\x01a\x06OV[PP\x81`\x80\x82\x01Ra\x07\x81`\xA0\x84\x01a\x06sV[`\xA0\x82\x01R\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07\xA5W`\0\x80\xFD[\x83Qa\x07\xB0\x81a\x05\rV[` \x85\x01Q\x90\x93Pa\x07\xC1\x81a\x05\rV[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x07\xE5W`\0\x80\xFD[a\x07\xEE\x82a\x06sV[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0\x82Q\x80``\x84\x01Ra\x08(\x81`\x80\x85\x01` \x87\x01a\x06OV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`\x80\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 g\x19\xF0U\xC7\x170\x80\xF0\xA40\xE7\xAF(\x92\x84\x12t\xAB5\xAC\xE3T\xA6\xFA\x1Be\x90\xC7\xC8}\x8AdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static TOKENBRIDGE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x07[,\x0C\x14a\0gW\x80c\x19\t\xE8\x06\x14a\0|W\x80cN\x83\x04D\x14a\0\x8FW\x80c\xAE\x02\x88\n\x14a\0\xA2W\x80c\xCE\x11\xE6\xAB\x14a\0\xF2W\x80c\xFB\x0Er+\x14a\x01\x05W[`\0\x80\xFD[a\0za\0u6`\x04a\x04\xEBV[a\x01\x18V[\0[a\0za\0\x8A6`\x04a\x05%V[a\x02\xA2V[a\0za\0\x9D6`\x04a\x05mV[a\x04HV[a\0\xD6a\0\xB06`\x04a\x05\xAFV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xD6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01Ta\0\xD6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01T`@Qc\xDA\x062\xAD`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x062\xAD\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\x91\x91\x90\x81\x01\x90a\x06\x88V[\x90P`\0\x80`\0\x83`\x80\x01Q\x80` \x01\x90Q\x81\x01\x90a\x01\xB0\x91\x90a\x07\x90V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x163\x14a\x02&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FYou are not the recipient of the`D\x82\x01Rhse tokens`\xB8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x99\x91\x90a\x07\xD3V[PPPPPPPV[`\0\x84\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x85R\x92R\x90\x91 T\x16a\x03\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid token for target chain\0\0`D\x82\x01R`d\x01a\x02\x1DV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8A\x91\x90a\x07\xD3V[P`\0\x84\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x85R\x90\x83R\x81\x84 T\x82Q\x90\x82\x16\x93\x81\x01\x93\x90\x93R\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0TcO\x9Er\xAD`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9F<\xE5Z\x90a\x04\x0F\x90\x86\x90\x89\x90\x86\x90`\x04\x01a\x07\xF5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04)W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04=W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x85R\x92R\x90\x91 T\x16\x15a\x04\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10Y\x19\x1C\x99\\\xDC\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]`j\x1B`D\x82\x01R`d\x01a\x02\x1DV[`\0\x92\x83R`\x02` \x90\x81R`@\x80\x85 `\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x86R\x90\x91R\x90\x92 \x80T\x91\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90UV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xFEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\"W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05;W`\0\x80\xFD[\x845\x93P` \x85\x015a\x05M\x81a\x05\rV[\x92P`@\x85\x015a\x05]\x81a\x05\rV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\x82W`\0\x80\xFD[\x835\x92P` \x84\x015a\x05\x94\x81a\x05\rV[\x91P`@\x84\x015a\x05\xA4\x81a\x05\rV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x05\xC2W`\0\x80\xFD[\x825\x91P` \x83\x015a\x05\xD4\x81a\x05\rV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x18Wa\x06\x18a\x05\xDFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06GWa\x06Ga\x05\xDFV[`@R\x91\x90PV[`\0[\x83\x81\x10\x15a\x06jW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06RV[PP`\0\x91\x01RV[\x80Q\x80\x15\x15\x81\x14a\x06\x83W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x06\x9BW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xB3W`\0\x80\xFD[\x90\x84\x01\x90`\xC0\x82\x87\x03\x12\x15a\x06\xC7W`\0\x80\xFD[a\x06\xCFa\x05\xF5V[\x82Q\x81R\x83\x83\x01Q\x84\x82\x01R`@\x83\x01Qa\x06\xE9\x81a\x05\rV[`@\x82\x01R``\x83\x01Qa\x06\xFC\x81a\x05\rV[``\x82\x01R`\x80\x83\x01Q\x82\x81\x11\x15a\x07\x13W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x88\x13a\x07$W`\0\x80\xFD[\x80Q\x83\x81\x11\x15a\x076Wa\x076a\x05\xDFV[a\x07H`\x1F\x82\x01`\x1F\x19\x16\x87\x01a\x06\x1EV[\x93P\x80\x84R\x88\x86\x82\x84\x01\x01\x11\x15a\x07^W`\0\x80\xFD[a\x07m\x81\x87\x86\x01\x88\x85\x01a\x06OV[PP\x81`\x80\x82\x01Ra\x07\x81`\xA0\x84\x01a\x06sV[`\xA0\x82\x01R\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07\xA5W`\0\x80\xFD[\x83Qa\x07\xB0\x81a\x05\rV[` \x85\x01Q\x90\x93Pa\x07\xC1\x81a\x05\rV[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x07\xE5W`\0\x80\xFD[a\x07\xEE\x82a\x06sV[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0\x82Q\x80``\x84\x01Ra\x08(\x81`\x80\x85\x01` \x87\x01a\x06OV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`\x80\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 g\x19\xF0U\xC7\x170\x80\xF0\xA40\xE7\xAF(\x92\x84\x12t\xAB5\xAC\xE3T\xA6\xFA\x1Be\x90\xC7\xC8}\x8AdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static TOKENBRIDGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TokenBridge<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TokenBridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TokenBridge<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TokenBridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TokenBridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TokenBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TokenBridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TOKENBRIDGE_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                TOKENBRIDGE_ABI.clone(),
                TOKENBRIDGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `bridgeToken` (0x1909e806) function
        pub fn bridge_token(
            &self,
            chain_id: ::ethers::core::types::U256,
            token_address: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 9, 232, 6], (chain_id, token_address, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crossTokenAddress` (0xae02880a) function
        pub fn cross_token_address(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([174, 2, 136, 10], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inbox` (0xfb0e722b) function
        pub fn inbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 14, 114, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `outbox` (0xce11e6ab) function
        pub fn outbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([206, 17, 230, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `releaseToken` (0x075b2c0c) function
        pub fn release_token(
            &self,
            chain_id: ::ethers::core::types::U256,
            message_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 91, 44, 12], (chain_id, message_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCrossTokenAddress` (0x4e830444) function
        pub fn set_cross_token_address(
            &self,
            chain_id: ::ethers::core::types::U256,
            source_token_address: ::ethers::core::types::Address,
            target_token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [78, 131, 4, 68],
                    (chain_id, source_token_address, target_token_address),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TokenBridge<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `bridgeToken` function with signature `bridgeToken(uint256,address,address,uint256)` and selector `0x1909e806`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "bridgeToken",
        abi = "bridgeToken(uint256,address,address,uint256)"
    )]
    pub struct BridgeTokenCall {
        pub chain_id: ::ethers::core::types::U256,
        pub token_address: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `crossTokenAddress` function with signature `crossTokenAddress(uint256,address)` and selector `0xae02880a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "crossTokenAddress", abi = "crossTokenAddress(uint256,address)")]
    pub struct CrossTokenAddressCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `inbox` function with signature `inbox()` and selector `0xfb0e722b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "inbox", abi = "inbox()")]
    pub struct InboxCall;
    ///Container type for all input parameters for the `outbox` function with signature `outbox()` and selector `0xce11e6ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "outbox", abi = "outbox()")]
    pub struct OutboxCall;
    ///Container type for all input parameters for the `releaseToken` function with signature `releaseToken(uint256,uint256)` and selector `0x075b2c0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "releaseToken", abi = "releaseToken(uint256,uint256)")]
    pub struct ReleaseTokenCall {
        pub chain_id: ::ethers::core::types::U256,
        pub message_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setCrossTokenAddress` function with signature `setCrossTokenAddress(uint256,address,address)` and selector `0x4e830444`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCrossTokenAddress",
        abi = "setCrossTokenAddress(uint256,address,address)"
    )]
    pub struct SetCrossTokenAddressCall {
        pub chain_id: ::ethers::core::types::U256,
        pub source_token_address: ::ethers::core::types::Address,
        pub target_token_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum TokenBridgeCalls {
        BridgeToken(BridgeTokenCall),
        CrossTokenAddress(CrossTokenAddressCall),
        Inbox(InboxCall),
        Outbox(OutboxCall),
        ReleaseToken(ReleaseTokenCall),
        SetCrossTokenAddress(SetCrossTokenAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for TokenBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BridgeTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BridgeToken(decoded));
            }
            if let Ok(decoded) = <CrossTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CrossTokenAddress(decoded));
            }
            if let Ok(decoded) = <InboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Inbox(decoded));
            }
            if let Ok(decoded) = <OutboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Outbox(decoded));
            }
            if let Ok(decoded) = <ReleaseTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseToken(decoded));
            }
            if let Ok(decoded) = <SetCrossTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetCrossTokenAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TokenBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BridgeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CrossTokenAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Inbox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Outbox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReleaseToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCrossTokenAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TokenBridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BridgeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrossTokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Inbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Outbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCrossTokenAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BridgeTokenCall> for TokenBridgeCalls {
        fn from(value: BridgeTokenCall) -> Self {
            Self::BridgeToken(value)
        }
    }
    impl ::core::convert::From<CrossTokenAddressCall> for TokenBridgeCalls {
        fn from(value: CrossTokenAddressCall) -> Self {
            Self::CrossTokenAddress(value)
        }
    }
    impl ::core::convert::From<InboxCall> for TokenBridgeCalls {
        fn from(value: InboxCall) -> Self {
            Self::Inbox(value)
        }
    }
    impl ::core::convert::From<OutboxCall> for TokenBridgeCalls {
        fn from(value: OutboxCall) -> Self {
            Self::Outbox(value)
        }
    }
    impl ::core::convert::From<ReleaseTokenCall> for TokenBridgeCalls {
        fn from(value: ReleaseTokenCall) -> Self {
            Self::ReleaseToken(value)
        }
    }
    impl ::core::convert::From<SetCrossTokenAddressCall> for TokenBridgeCalls {
        fn from(value: SetCrossTokenAddressCall) -> Self {
            Self::SetCrossTokenAddress(value)
        }
    }
    ///Container type for all return fields from the `crossTokenAddress` function with signature `crossTokenAddress(uint256,address)` and selector `0xae02880a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CrossTokenAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `inbox` function with signature `inbox()` and selector `0xfb0e722b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct InboxReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `outbox` function with signature `outbox()` and selector `0xce11e6ab`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OutboxReturn(pub ::ethers::core::types::Address);
}
