use crate::keygen::{PrivateKey, PublicKey};

#[derive(Debug)]
pub(crate) struct  Paillier {
    n_length: i8,
    public_key: PublicKey,
    private_key: PrivateKey
}