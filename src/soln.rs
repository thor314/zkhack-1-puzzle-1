use crate::data::puzzle_data;
use nalgebra;

/// BLS scheme: pubkey: g^x; secretkey: x; signatures: h(m)^x
/// try to attack private key, generate new message pair
pub fn solve() -> (/* msg */ &'static str, /* sig */ &'static str) {
    //let (pk, ms, sigs) = puzzle_data();

    // x: secret key
    let x = 1;
    let msg = "f2faa8b1bb0f06c6142e788ad836d1f7d1abf95458a08a55593c594056ac224d";
    // sig: h(m)^x
    let sig = crate::hash::hash_to_curve(&hex::decode(msg).unwrap()); // how take pow: .pow(x);
                                                                      //(msg, sig)
    todo!();
}
