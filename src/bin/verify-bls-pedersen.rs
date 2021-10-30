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
    let (msg, sig) = bls_pedersen::soln::solve();
    let sig = G1Affine::deserialize(&mut Cursor::new(hex::decode(sig).unwrap())).unwrap();
    let msg = hex::decode(msg).unwrap();
    verify(pk, &msg, sig);
}
