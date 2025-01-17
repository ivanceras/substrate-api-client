/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

//! This examples floats the node with a series of transactions.

// run this against test node with
// > substrate-test-node --dev --execution native --ws-port 9979 -ltxpool=debug

use node_template_runtime::{BalancesCall, Call};
use sp_core::crypto::Pair;
use sp_keyring::AccountKeyring;

use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::ExtrinsicParams;
use substrate_api_client::{
    compose_extrinsic_offline, Api, PlainTipExtrinsicParams, UncheckedExtrinsicV4, XtStatus,
};

fn main() {
    env_logger::init();

    // initialize api and set the signer (sender) that is used to sign the extrinsics
    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new("ws://127.0.0.1:9944");
    let api = Api::<_, _, PlainTipExtrinsicParams>::new(client)
        .map(|api| api.set_signer(from))
        .unwrap();

    println!(
        "[+] Alice's Account Nonce is {}\n",
        api.get_nonce().unwrap()
    );

    // define the recipient
    let to = AccountKeyring::Bob.to_account_id();

    let mut nonce = api.get_nonce().unwrap();
    let first_nonce = nonce;
    while nonce < first_nonce + 500 {
        // compose the extrinsic with all the element
        #[allow(clippy::redundant_clone)]
        let xt: UncheckedExtrinsicV4<_> = compose_extrinsic_offline!(
            api.clone().signer.unwrap(),
            Call::Balances(BalancesCall::transfer {
                dest: GenericAddress::Id(to.clone()),
                value: 1_000_000
            }),
            nonce,
            api.genesis_hash,
            api.genesis_hash,
            api.runtime_version.spec_version,
            api.runtime_version.transaction_version,
            api.extrinsic_params
        );
        // send and watch extrinsic until finalized
        println!("sending extrinsic with nonce {}", nonce);
        let _blockh = api
            .send_extrinsic(xt.hex_encode(), XtStatus::Ready)
            .unwrap();

        nonce += 1;
    }
}
