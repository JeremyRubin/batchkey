use crate::*;
use std::sync::mpsc::*;
use std::thread;
pub fn run(beta: &scalars::scalar, peer: &mut dyn ReadWrite) -> scalars::scalar {
    let ctx = &secp256k1::Secp256k1::new();
    let (tx, rx) = channel();
    let r = thread::spawn(move || {
        let mut sigma_beta = [0u64; 4];
        for (mut v, shift) in rx.iter().take(32).zip((0u8..32u8).rev()) {
            scalars::non_constant_time_shift(&mut v, shift);
            scalars::secp256k1_scalar_add_assign(&mut sigma_beta, &v);
        }
        sigma_beta
    });
    // MSB to LSB
    for (count, choice) in scalars::bytes_from_scalar(beta).iter().enumerate() {
        tx.send(protocol::ot::receiver::run(
            ctx,
            *choice,
            xor_decipher_scalar,
            peer,
        ));
    }
    r.join().unwrap()
}
