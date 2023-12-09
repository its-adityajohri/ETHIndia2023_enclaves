pub use kyc_wallet::*;
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
pub mod kyc_wallet {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Verifier"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pA"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ),
                            2usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256[2]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pB"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    2usize,
                                ),
                            ),
                            2usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pC"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ),
                            2usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256[2]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_pubSignals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ),
                            34usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256[34]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("operator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("verifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Verifier"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    pub static KYCWALLET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x04w8\x03\x80a\x04w\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02\x05V[`@Qc\x85%\xA6\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x85%\xA6\x1D\x90a\0a\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x02\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xA2\x91\x90a\x03\x93V[a\0\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl$\xB7;0\xB64\xB2\x10(97\xB7\xB3`\x99\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x90UPa\x03\xBC\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01BWa\x01Ba\x01\nV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x01YW`\0\x80\xFD[a\x01aa\x01 V[\x80`@\x84\x01\x85\x81\x11\x15a\x01sW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x01\x8DW\x80Q\x84R` \x93\x84\x01\x93\x01a\x01uV[P\x90\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a\x01\xA9W`\0\x80\xFD[`@Qa\x04@\x80\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17\x15a\x01\xCDWa\x01\xCDa\x01\nV[`@R\x83\x01\x81\x85\x82\x11\x15a\x01\xE0W`\0\x80\xFD[\x84[\x82\x81\x10\x15a\x01\xFAW\x80Q\x82R` \x91\x82\x01\x91\x01a\x01\xE2V[P\x91\x95\x94PPPPPV[`\0\x80`\0\x80`\0a\x05`\x86\x88\x03\x12\x15a\x02\x1EW`\0\x80\xFD[\x85Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x025W`\0\x80\xFD[\x94P` a\x02E\x88\x88\x83\x01a\x01HV[\x94P\x87`\x7F\x88\x01\x12a\x02VW`\0\x80\xFD[a\x02^a\x01 V[\x80`\xE0\x89\x01\x8A\x81\x11\x15a\x02pW`\0\x80\xFD[``\x8A\x01[\x81\x81\x10\x15a\x02\x95Wa\x02\x87\x8C\x82a\x01HV[\x84R\x92\x84\x01\x92`@\x01a\x02uV[P\x81\x96Pa\x02\xA3\x8B\x82a\x01HV[\x95PPPPPa\x02\xB7\x87a\x01 \x88\x01a\x01\x98V[\x90P\x92\x95P\x92\x95\x90\x93PV[\x80`\0[`\x02\x81\x10\x15a\x02\xE6W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x02\xC7V[PPPPV[a\x05@\x81\x01a\x02\xFB\x82\x87a\x02\xC3V[`@\x80\x83\x01\x86`\0[`\x02\x80\x82\x10a\x03\x13WPa\x03NV[\x82Q\x84`\0[\x83\x81\x10\x15a\x037W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x03\x19V[PPP\x92\x84\x01\x92P` \x91\x90\x91\x01\x90`\x01\x01a\x03\x04V[PPPPa\x03_`\xC0\x83\x01\x85a\x02\xC3V[a\x01\0\x82\x01\x83`\0[`\"\x81\x10\x15a\x03\x87W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x03hV[PPP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x03\xA5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xB5W`\0\x80\xFD[\x93\x92PPPV[`\xAD\x80a\x03\xCA`\09`\0\xF3\xFE`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`2W`\x005`\xE0\x1C\x80c+z\xC3\xF3\x14`7W\x80cW\x0C\xA75\x14`eW[`\0\x80\xFD[`\x01T`I\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0T`I\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V\xFE\xA2dipfsX\"\x12 \xB2=\xE1\xF0\x14E\xBA1\xB0W\xD5\xC1\x80$)!\xF1\x03\xBBy\x07\xC8D\xDC\xAC\xE5\xDD\x1B-\x9F\xFD}dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static KYCWALLET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`2W`\x005`\xE0\x1C\x80c+z\xC3\xF3\x14`7W\x80cW\x0C\xA75\x14`eW[`\0\x80\xFD[`\x01T`I\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0T`I\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V\xFE\xA2dipfsX\"\x12 \xB2=\xE1\xF0\x14E\xBA1\xB0W\xD5\xC1\x80$)!\xF1\x03\xBBy\x07\xC8D\xDC\xAC\xE5\xDD\x1B-\x9F\xFD}dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static KYCWALLET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct KycWallet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KycWallet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KycWallet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KycWallet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KycWallet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(KycWallet)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KycWallet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KYCWALLET_ABI.clone(),
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
                KYCWALLET_ABI.clone(),
                KYCWALLET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `operator` (0x570ca735) function
        pub fn operator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([87, 12, 167, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifier` (0x2b7ac3f3) function
        pub fn verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KycWallet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `operator` function with signature `operator()` and selector `0x570ca735`
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
    #[ethcall(name = "operator", abi = "operator()")]
    pub struct OperatorCall;
    ///Container type for all input parameters for the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
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
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
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
    pub enum KycWalletCalls {
        Operator(OperatorCall),
        Verifier(VerifierCall),
    }
    impl ::ethers::core::abi::AbiDecode for KycWalletCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <OperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Operator(decoded));
            }
            if let Ok(decoded) = <VerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verifier(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KycWalletCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Operator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for KycWalletCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Operator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verifier(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OperatorCall> for KycWalletCalls {
        fn from(value: OperatorCall) -> Self {
            Self::Operator(value)
        }
    }
    impl ::core::convert::From<VerifierCall> for KycWalletCalls {
        fn from(value: VerifierCall) -> Self {
            Self::Verifier(value)
        }
    }
    ///Container type for all return fields from the `operator` function with signature `operator()` and selector `0x570ca735`
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
    pub struct OperatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `verifier` function with signature `verifier()` and selector `0x2b7ac3f3`
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
    pub struct VerifierReturn(pub ::ethers::core::types::Address);
}
