use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, _ms, _sigs) = puzzle_data();
    /*
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
        println!("{:?},{}", m, sig);
    }*/

    /* Your solution here! */
    use ark_bls12_381::G1Affine;
    use ark_serialize::CanonicalDeserialize;
    use std::io::Cursor;
    // copied from puzzle data
    let sig = "067ffcb122c43181eb4c525d2a7b56714262aae808ae24b62aa5ec6e1035a9f6ce6473f19dc470957afa98b437c68814"; // figure out what to put here
    let sig = G1Affine::deserialize(&mut Cursor::new(hex::decode(sig).unwrap())).unwrap();
    let m = "f2faa8b1bb0f06c6142e788ad836d1f7d1abf95458a08a55593c594056ac224d";
    let ms = hex::decode(m).unwrap();
    verify(pk, &ms, sig);
}
