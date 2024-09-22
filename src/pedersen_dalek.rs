use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use std::convert::TryInto;
use core::ops::Sub;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Commitment {
    commitment: RistrettoPoint,
    blinder: Scalar,
    g: RistrettoPoint,
    h: RistrettoPoint,
}

impl Commitment {
    fn new(
        value: u64,          // Single value (e.g., a timestamp)
        blinder: Scalar,     // Blinding factor
        g: RistrettoPoint,   // Base point for the value
        h: RistrettoPoint,   // Base point for the blinder
    ) -> Self {
        let value_scalar = Scalar::from(value);
        let commitment = value_scalar * g + blinder * h;

        Commitment {
            commitment,
            blinder,
            g,
            h,
        }
    }

    fn verify(&self, value: u64) -> bool {
        let value_scalar = Scalar::from(value);
        let computed_commitment = value_scalar * self.g + self.blinder * self.h;
        self.commitment == computed_commitment
    }
    /* 
    modulo is wrong
    fn timestamp_difference(&self, other_blinder: Scalar) -> u64 {
        // we just work of the scalars which are blinded
        // Calculate the nonce difference as u64
        let t1_bytes = self.blinder.to_bytes();
        let t2_bytes = other_blinder.to_bytes();

        let t1_u64 = u64::from_le_bytes(t1_bytes[0..8].try_into().unwrap());
        let t2_u64 = u64::from_le_bytes(t2_bytes[0..8].try_into().unwrap());

        let r1_u64 = u64::from_le_bytes(r1_bytes[0..8].try_into().unwrap());
        let r2_u64 = u64::from_le_bytes(r2_bytes[0..8].try_into().unwrap());
    

        t1_u64.wrapping_sub(t2_u64)
    }
    */

    //Subtract two commitments homomorphically
    // don't use this, takes forever
    fn force_recover_timestamp_difference(&self, subtracted_commitment: RistrettoPoint) -> u64 {
        // To recover the difference as a u64, we use a brute-force over possible u64 values until we find the match
        for i in 0..u64::MAX {
            let scalar_i = Scalar::from(i);
            if scalar_i * self.g == subtracted_commitment {
                return i;
            }
        }
        panic!("Could not recover timestamp difference");
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Serialize commitment (RistrettoPoint)
        bytes.extend(self.commitment.compress().as_bytes());
        // Serialize blinder (Scalar)
        bytes.extend(self.blinder.to_bytes());
        // Serialize g and h (RistrettoPoint)
        bytes.extend(self.g.compress().as_bytes());
        bytes.extend(self.h.compress().as_bytes());
        bytes
    }

    /// Deserialize a Commitment struct from bytes
    fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        // Check if input is the correct length
        // RistrettoPoint (32 bytes each for commitment, g, h) and 32 byte Scalar
        if bytes.len() < 32 * 4 { 
            return Err("Input bytes are too short for deserialization");
        }

        // Deserialize the commitment (RistrettoPoint)
        let commitment_bytes: [u8; 32] = bytes[0..32].try_into().map_err(|_| "Failed to parse commitment bytes")?;
        let compressed_point = CompressedRistretto::from_slice(&commitment_bytes).expect("Failed to decompress RistrettoPoint");
        let commitment = compressed_point.decompress().expect("Failed to decompress commitment RistrettoPoint"); 

        // Deserialize the blinder (Scalar)
        let blinder_bytes: [u8; 32] = bytes[32..64].try_into().map_err(|_| "Failed to parse blinder bytes")?;
        let blinder = Scalar::from_canonical_bytes(blinder_bytes).expect("Failed to reconstitute Scalar");;

        // Deserialize g and h (RistrettoPoint)
        let g_bytes: [u8; 32] = bytes[64..96].try_into().map_err(|_| "Failed to parse g bytes")?;
        let compressed_point = CompressedRistretto::from_slice(&g_bytes).expect("Failed to decompress RistrettoPoint");
        let g = compressed_point.decompress().expect("Failed to decompress g RistrettoPoint"); 

        let h_bytes: [u8; 32] = bytes[96..128].try_into().map_err(|_| "Failed to parse h bytes")?;
        let compressed_point = CompressedRistretto::from_slice(&h_bytes).expect("Failed to decompress RistrettoPoint");
        let h = compressed_point.decompress().expect("Failed to decompress h RistrettoPoint"); 

        Ok(Commitment {
            commitment,
            blinder,
            g,
            h,
        })
    }

}

mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_commitment() {
        let timestamps: [u64; 2] = [1632549200, 1632549260];

        let mut rng = OsRng;
        
        let b_factor_1 = Scalar::random(&mut rng);
        let b_factor_2 = Scalar::random(&mut rng);

        let g = RistrettoPoint::random(&mut rng); // Base point for the timestamp
        let h = RistrettoPoint::random(&mut rng);

        let commitment1 = Commitment::new(timestamps[0].clone(), b_factor_1, g, h);
        let commitment2 = Commitment::new(timestamps[1].clone(), b_factor_2, g, h);

        assert!(commitment1.verify(timestamps[0].clone()));
        assert!(!commitment1.verify(timestamps[1].clone()));
        assert!(commitment2.verify(timestamps[1].clone()));
        assert!(!commitment2.verify(timestamps[0].clone()));

        let c1_ser = commitment1.to_bytes();
        let c1_deser = Commitment::from_bytes(&c1_ser).unwrap();
        assert_eq!(c1_deser, commitment1);

        let c2_ser = commitment2.to_bytes();
        let c2_deser = Commitment::from_bytes(&c2_ser).unwrap();
        assert_eq!(c2_deser, commitment2);

;
        // let timestamp_diff = commitment1.timestamp_difference(commitment2.blinder);
        //assert_eq!(timestamp_diff, timestamps[1]-timestamps[0]);

    }
}