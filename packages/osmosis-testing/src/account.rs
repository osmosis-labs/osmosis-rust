use cosmrs::{
    crypto::{secp256k1::SigningKey, PublicKey},
    AccountId,
};

const ADDRESS_PREFIX: &str = "osmo";

pub trait Account {
    fn public_key(&self) -> PublicKey;
    fn address(&self) -> String {
        self.account_id().to_string()
    }
    fn account_id(&self) -> AccountId {
        self.public_key()
            .account_id(ADDRESS_PREFIX)
            .expect("ADDRESS_PREFIX is constant and must valid")
    }
}
pub struct SigningAccount {
    signing_key: SigningKey,
}

impl From<SigningKey> for SigningAccount {
    fn from(signing_key: SigningKey) -> Self {
        SigningAccount { signing_key }
    }
}

impl Account for SigningAccount {
    fn public_key(&self) -> PublicKey {
        self.signing_key.public_key()
    }
}

impl SigningAccount {
    pub fn signing_key(&'_ self) -> &'_ SigningKey {
        &self.signing_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonSigningAccount {
    public_key: PublicKey,
}

impl From<PublicKey> for NonSigningAccount {
    fn from(public_key: PublicKey) -> Self {
        NonSigningAccount { public_key }
    }
}
impl From<SigningAccount> for NonSigningAccount {
    fn from(signing_account: SigningAccount) -> Self {
        NonSigningAccount {
            public_key: signing_account.public_key(),
        }
    }
}

impl Account for NonSigningAccount {
    fn public_key(&self) -> PublicKey {
        self.public_key
    }
}
