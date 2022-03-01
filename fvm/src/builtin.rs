//! The builtin module exports identifiers and convenience functions related
//! with built-in actors.

use cid::multihash::{Code, MultihashDigest};
use cid::Cid;
use fvm_shared::encoding::{to_vec, DAG_CBOR};
use lazy_static::lazy_static;

/// The multicodec value for raw data.
const IPLD_CODEC_RAW: u64 = 0x55;

lazy_static! {
    /// Cid of the empty array Cbor bytes (`EMPTY_ARR_BYTES`).
    pub static ref EMPTY_ARR_CID: Cid = {
        let empty = to_vec::<[(); 0]>(&[]).unwrap();
        Cid::new_v1(DAG_CBOR, Code::Blake2b256.digest(&empty))
    };

    // TODO these CIDs may need to be versioned with SnapDeals; and maybe
    //  a few more times before some of these actors are moved to user-land.
    pub static ref SYSTEM_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/system");
    pub static ref INIT_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/init");
    pub static ref CRON_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/cron");
    pub static ref ACCOUNT_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/account");
    pub static ref POWER_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/storagepower");
    pub static ref MINER_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/storageminer");
    pub static ref MARKET_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/storagemarket");
    pub static ref PAYCH_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/paymentchannel");
    pub static ref MULTISIG_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/multisig");
    pub static ref REWARD_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/reward");
    pub static ref VERIFREG_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/verifiedregistry");
    pub static ref CHAOS_ACTOR_CODE_ID: Cid = make_builtin_code_cid(b"fil/7/chaos");
}

/// Returns a Code CID for a builtin actor.
fn make_builtin_code_cid(bz: &[u8]) -> Cid {
    Cid::new_v1(IPLD_CODEC_RAW, Code::Identity.digest(bz))
}

/// Returns true if the code `Cid` belongs to a builtin actor.
pub fn is_builtin_actor(code: &Cid) -> bool {
    code == &*SYSTEM_ACTOR_CODE_ID
        || code == &*INIT_ACTOR_CODE_ID
        || code == &*CRON_ACTOR_CODE_ID
        || code == &*ACCOUNT_ACTOR_CODE_ID
        || code == &*POWER_ACTOR_CODE_ID
        || code == &*MINER_ACTOR_CODE_ID
        || code == &*MARKET_ACTOR_CODE_ID
        || code == &*PAYCH_ACTOR_CODE_ID
        || code == &*MULTISIG_ACTOR_CODE_ID
        || code == &*REWARD_ACTOR_CODE_ID
        || code == &*VERIFREG_ACTOR_CODE_ID
}

/// Returns true if the code belongs to a singleton actor. That is, an actor
/// that cannot be constructed by a user.
pub fn is_singleton_actor(code: &Cid) -> bool {
    code == &*SYSTEM_ACTOR_CODE_ID
        || code == &*INIT_ACTOR_CODE_ID
        || code == &*REWARD_ACTOR_CODE_ID
        || code == &*CRON_ACTOR_CODE_ID
        || code == &*POWER_ACTOR_CODE_ID
        || code == &*MARKET_ACTOR_CODE_ID
        || code == &*VERIFREG_ACTOR_CODE_ID
}
