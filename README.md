# KZG Ceremony Participant

> ⚠️ This project is still heavy WIP.

Implemensts a participant client for the KZG powers of tau ceremony according this [spec](https://github.com/ethereum/kzg-ceremony-specs).

## Related Links
- [Official spec](https://github.com/ethereum/kzg-ceremony-specs)
- [WIP Coordinator implementation](https://github.com/recmo/kzg-ceremony-coordinator)
- [BLS12-381 Zcash spec](https://github.com/zcash/librustzcash/blob/6e0364cd42a2b3d2b958a54771ef51a8db79dd29/pairing/src/bls12_381/README.md)
- [BLS12-381 For The Rest Of Us](https://hackmd.io/@benjaminion/bls12-381)
- [How trusted setups work](https://vitalik.ca/general/2022/03/14/trustedsetup.html)

## Build instructions
### Native
- Run tests: `cargo test --target aarch64-apple-darwin`
- Build: `cargo run --release --target aarch64-apple-darwin`

### Wasm
- Build: `wasm-pack build --target web -d wasm/pkg`
- Run server: `python3 ./wasm/server.py`

## Benchmarks

### Native
- Apple M1 Max (10 threads): `2.68s`  

### Wasm
- Apple M1 Max, Chrome (10 threads): `14.70s` 
- Apple M1 Max, Chrome (1 thread): `112.33s` 

## TODO
- [ ] Create some nice issues for others to pick up 
- [ ] G2 serialization and tests (according to zcash spec)
- [x] Merge wasm implementation
- [ ] Evaluate [blst](https://github.com/supranational/blst/tree/master/bindings/rust) instead of ark_bls12_381
- [x] Add benchmarks
- [ ] Implement pot_pubkey
- [ ] Running Product Subgroup check 
- [ ] Fix wasm for firefox (not checked) and safari