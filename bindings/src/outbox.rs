pub use outbox::*;
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
pub mod outbox {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("currentMessageId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currentMessageId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SentMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SentMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fromChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OUTBOX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80Ua\x02\x81\x80a\0$`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x9F<\xE5Z\x14a\0;W\x80c\xF8\xE4\xE73\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\0\xDBV[a\0kV[\0[a\0Y`\0T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0\x80T\x90\x80a\0z\x83a\x01\xB4V[\x91\x90PUPF`\0T\x7F\x8F\xAA\xDD@k\xA1\xF9/\x9FL\x0F1\x9E\xA6\xF8'\xA4x?X\xD3e\xA7}\xDF\xAF\x17\x967M\x96\x99\x843\x87\x86`@Qa\0\xB8\x94\x93\x92\x91\x90a\x01\xDBV[`@Q\x80\x91\x03\x90\xA3PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\xF0W`\0\x80\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x07W`\0\x80\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01+W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01?W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01QWa\x01Qa\0\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01yWa\x01ya\0\xC5V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x01\x92W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0`\x01\x82\x01a\x01\xD4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x84\x81R`\0` `\x01\x80`\xA0\x1B\x03\x80\x87\x16\x82\x85\x01R\x80\x86\x16`@\x85\x01RP`\x80``\x84\x01R\x83Q\x80`\x80\x85\x01R`\0[\x81\x81\x10\x15a\x02'W\x85\x81\x01\x83\x01Q\x85\x82\x01`\xA0\x01R\x82\x01a\x02\x0BV[P`\0`\xA0\x82\x86\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xB2\x98(\xDAv\xD2\xAB$\xBA\x85h\xB0\xA8\xBA\n\x081\xFAd\xADh/\x86\xFFrO=\xE9\x7F\x04\xD5\x90dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static OUTBOX_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x9F<\xE5Z\x14a\0;W\x80c\xF8\xE4\xE73\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\0\xDBV[a\0kV[\0[a\0Y`\0T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0\x80T\x90\x80a\0z\x83a\x01\xB4V[\x91\x90PUPF`\0T\x7F\x8F\xAA\xDD@k\xA1\xF9/\x9FL\x0F1\x9E\xA6\xF8'\xA4x?X\xD3e\xA7}\xDF\xAF\x17\x967M\x96\x99\x843\x87\x86`@Qa\0\xB8\x94\x93\x92\x91\x90a\x01\xDBV[`@Q\x80\x91\x03\x90\xA3PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\xF0W`\0\x80\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x07W`\0\x80\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01+W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01?W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01QWa\x01Qa\0\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x01yWa\x01ya\0\xC5V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x01\x92W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[`\0`\x01\x82\x01a\x01\xD4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x84\x81R`\0` `\x01\x80`\xA0\x1B\x03\x80\x87\x16\x82\x85\x01R\x80\x86\x16`@\x85\x01RP`\x80``\x84\x01R\x83Q\x80`\x80\x85\x01R`\0[\x81\x81\x10\x15a\x02'W\x85\x81\x01\x83\x01Q\x85\x82\x01`\xA0\x01R\x82\x01a\x02\x0BV[P`\0`\xA0\x82\x86\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xB2\x98(\xDAv\xD2\xAB$\xBA\x85h\xB0\xA8\xBA\n\x081\xFAd\xADh/\x86\xFFrO=\xE9\x7F\x04\xD5\x90dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static OUTBOX_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Outbox<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Outbox<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Outbox<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Outbox<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Outbox<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Outbox)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Outbox<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OUTBOX_ABI.clone(),
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
                OUTBOX_ABI.clone(),
                OUTBOX_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `currentMessageId` (0xf8e4e733) function
        pub fn current_message_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 228, 231, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendMessage` (0x9f3ce55a) function
        pub fn send_message(
            &self,
            to: ::ethers::core::types::Address,
            chain_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 60, 229, 90], (to, chain_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SentMessage` event
        pub fn sent_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SentMessageFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SentMessageFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Outbox<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "SentMessage",
        abi = "SentMessage(uint256,uint256,uint256,address,address,bytes)"
    )]
    pub struct SentMessageFilter {
        #[ethevent(indexed)]
        pub message_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub from_chain_id: ::ethers::core::types::U256,
        pub to_chain_id: ::ethers::core::types::U256,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `currentMessageId` function with signature `currentMessageId()` and selector `0xf8e4e733`
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
    #[ethcall(name = "currentMessageId", abi = "currentMessageId()")]
    pub struct CurrentMessageIdCall;
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(address,uint256,bytes)` and selector `0x9f3ce55a`
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
    #[ethcall(name = "sendMessage", abi = "sendMessage(address,uint256,bytes)")]
    pub struct SendMessageCall {
        pub to: ::ethers::core::types::Address,
        pub chain_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum OutboxCalls {
        CurrentMessageId(CurrentMessageIdCall),
        SendMessage(SendMessageCall),
    }
    impl ::ethers::core::abi::AbiDecode for OutboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CurrentMessageIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentMessageId(decoded));
            }
            if let Ok(decoded) = <SendMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendMessage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OutboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CurrentMessageId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OutboxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CurrentMessageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendMessage(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CurrentMessageIdCall> for OutboxCalls {
        fn from(value: CurrentMessageIdCall) -> Self {
            Self::CurrentMessageId(value)
        }
    }
    impl ::core::convert::From<SendMessageCall> for OutboxCalls {
        fn from(value: SendMessageCall) -> Self {
            Self::SendMessage(value)
        }
    }
    ///Container type for all return fields from the `currentMessageId` function with signature `currentMessageId()` and selector `0xf8e4e733`
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
    pub struct CurrentMessageIdReturn(pub ::ethers::core::types::U256);
}
