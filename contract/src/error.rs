use casper_types::ApiError;

#[derive(Debug)]
#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    WrongArguments = 2,
    NotRequiredStake = 3,
    BadTiming = 4,
    InvalidContext = 5,
    NegativeReward = 6,
    NegativeWithdrawableReward = 7,
    NegativeAmount = 8,
    MissingContractPackageHash = 9,
    InvalidContractPackageHash = 10,
    InvalidContractHash = 11,
    WithdrawCheckErrorEarly = 12,
    WithdrawCheckError = 13,
    NeitherAccountHashNorNeitherContractPackageHash = 14,
    UnexpectedContractHash = 15,
    NotContractPackageHash = 16,
    DictTargetTokenNotEqualTargetToken = 17,
    NoTargetNetworkDictForThisToken = 18,
    NoTargetTokenInAllowedTargetsDict = 19,
    ClientDoesNotHaveAnyKindOfLiquidity = 20,
    ClientDoesNotHaveSpecificKindOfLiquidity = 21,
    AlreadyInThisTargetTokenDict = 22,
    MessageAlreadyUsed = 23,
    NoValueInSignersDict = 24,
    InvalidSigner = 25,
    CasperAccountHashParsing = 26,
    WrongTokenName = 27,
    NoTokenInTokenContractPackageHashDict = 28,
    RecoverableSignatureTryFromFail = 29,
    NonRecoverableSignatureTryFromFail = 30,
    RecoverVerifyKeyFail = 31,
    CheckedSubFail = 32,
    SaltHexFail = 33,
    SaltWrongSize = 34,
    SignatureHexFail = 35,
    NotBridgePoolContractPackageHash = 36,
    EcdsaPublicKeyRecoveryFail = 37,
    MessageHashHexDecodingFail = 38,
    PublicKeyTryIntoFail = 39,
    ImmediateCallerFail = 40,
    SignerWrongFormat = 41,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
