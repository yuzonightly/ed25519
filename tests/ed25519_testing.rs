// Author:
// - Eduardo Yuzo Nakai <yuzonakai@gmail.com>

// Regression testing.

extern crate ed25519_fun;
extern crate hex;
extern crate rand;
extern crate sha2;

#[cfg(test)]
mod test_vectors {
    use ed25519_fun::{Keypair, PublicKey, SecretKey, Signature};
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    // http://ed25519.cr.yp.to/python/sign.input
    #[test]
    pub fn ed25519_cr_yp_to_regression_test() {
        let file = File::open("./tests/sign.input");
        if file.is_err() {
            println!("Where are the test vectors? :(");
            panic!();
        }
        let buffer = BufReader::new(file.unwrap());

        let mut lineno: usize = 0;
        for line in buffer.lines() {
            lineno += 1;

            let l = line.unwrap();
            let slices: Vec<&str> = l.split(":").collect();

            let secret_bytes: Vec<u8> = hex::decode(&slices[0]).unwrap();
            let public_bytes: Vec<u8> = hex::decode(&slices[1]).unwrap();
            let message_bytes: Vec<u8> = hex::decode(&slices[2]).unwrap();
            let signature_bytes: Vec<u8> = hex::decode(&slices[3]).unwrap();

            let secret: SecretKey = SecretKey::from_bytes(&secret_bytes[..32]).unwrap();

            let pk1: PublicKey = PublicKey::from_bytes(&public_bytes[..32]).unwrap();
            let sign1: Signature = Signature::from_bytes(&signature_bytes[..64]).unwrap();

            let keypair: Keypair = Keypair::generate_public_key(secret);

            let pk2: PublicKey = keypair.public;
            let sign2: Signature = keypair.sign(&message_bytes);

            assert!(
                pk1.as_bytes() == pk2.as_bytes(),
                "Public keys do not match: {}",
                lineno
            );
            assert!(
                sign1.as_bytes() == sign2.as_bytes(),
                "Signatures do not match: {}",
                lineno
            );
            assert!(
                keypair.verify(&message_bytes, sign1).is_ok(),
                "Verification failed: {}",
                lineno
            );
        }
    }
}
