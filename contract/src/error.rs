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
    ClientDoesNotHaveAnyKindOfLioquidity = 20,
    ClientDoesNotHaveSpecificKindOfLioquidity = 21,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
