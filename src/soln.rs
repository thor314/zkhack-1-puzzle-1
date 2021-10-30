use crate::data::puzzle_data;
use nalgebra;

/// generate private key
pub fn solve() -> (/* msg */ &'static str, /* sig */ &'static str) {
    let (pk, ms, sigs) = puzzle_data();

    //let message_v = dvector!();
    //let signature_v = vec![];
    let sig = "067ffcb122c43181eb4c525d2a7b56714262aae808ae24b62aa5ec6e1035a9f6ce6473f19dc470957afa98b437c68814";
    let msg = "f2faa8b1bb0f06c6142e788ad836d1f7d1abf95458a08a55593c594056ac224d";
    //(msg, sig)
    todo!();
}
