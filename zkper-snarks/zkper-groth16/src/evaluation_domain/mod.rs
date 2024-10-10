use rug::Integer;
use zkper_curves::curves::bls12_381::{Bls12_381ScalarField, BLS12_381_SCALAR};

#[derive(Debug)]
pub struct EvaluationDomain {
    pub coeffs: Vec<Integer>,
    pub exp: u32,
    pub omega: Integer,
    pub omegainv: Integer,
    pub geninv: Integer,
    pub minv: Integer,
}

impl EvaluationDomain {
    pub fn new(mut coeffs: Vec<Integer>) -> anyhow::Result<Self> {
        // Compute the size of our evaluation domain
        let needed_size = coeffs.len().next_power_of_two();
        let exp = needed_size.trailing_zeros();

        // The pairing-friendly curve may not be able to support
        // large enough (radix2) evaluation domains.
        if exp >= Bls12_381ScalarField::TWO_ADICITY {
            return Err(anyhow::anyhow!("radix2 evaluation domain too large"));
        }

        // Compute omega, the 2^exp primitive root of unity
        let mut omega = Bls12_381ScalarField::two_adic_root_of_unity();
        for _ in exp..Bls12_381ScalarField::TWO_ADICITY {
            omega = BLS12_381_SCALAR.square(omega);
        }

        // Extend the coeffs vector with zeroes if necessary
        coeffs.resize(needed_size, Integer::from(0));

        Ok(EvaluationDomain {
            coeffs,
            exp,
            omega: omega.clone(),
            omegainv: BLS12_381_SCALAR.invert(omega.clone()).unwrap(),
            geninv: BLS12_381_SCALAR
                .invert(Bls12_381ScalarField::MULTIPLICATIVE_GENERATOR.clone())
                .unwrap(),
            minv: BLS12_381_SCALAR.invert(Integer::from(needed_size)).unwrap(),
        })
    }

    pub fn z(&self, tau: &Integer) -> Integer {
        BLS12_381_SCALAR.sub(
            BLS12_381_SCALAR.pow(tau.clone(), &Integer::from(self.coeffs.len())),
            &Integer::ONE,
        )
    }

    fn bitreverse(mut n: u32, l: u32) -> u32 {
        let mut r = 0;
        for _ in 0..l {
            r = (r << 1) | (n & 1);
            n >>= 1;
        }
        r
    }

    fn base_fft(&mut self, omega: &Integer, log_n: u32) {
        let n = self.coeffs.len();
        assert_eq!(n, 1 << log_n);

        // Bit-reversal permutation
        for k in 0..n {
            let rk = Self::bitreverse(k as u32, log_n) as usize;
            if k < rk {
                self.coeffs.swap(rk, k);
            }
        }

        let mut m: usize = 1;
        for _ in 0..log_n {
            let w_m = BLS12_381_SCALAR.pow(omega.clone(), &Integer::from(n / (2 * m)));

            let mut k = 0;
            while k < n {
                let mut w = Integer::from(1);
                for j in 0..m as usize {
                    let mut t = self.coeffs[k + j + m].clone();
                    t = BLS12_381_SCALAR.mul(t, &w);
                    let mut tmp = self.coeffs[k + j].clone();
                    tmp = BLS12_381_SCALAR.sub(tmp, &t);
                    self.coeffs[k + j + m] = tmp;
                    self.coeffs[k + j] = BLS12_381_SCALAR.add(self.coeffs[k + j].clone(), &t);
                    w = BLS12_381_SCALAR.mul(w, &w_m);
                }
                k += 2 * m;
            }
            m *= 2;
        }
    }

    pub fn fft(&mut self) {
        self.base_fft(&self.omega.clone(), self.exp);
    }

    pub fn ifft(&mut self) {
        self.base_fft(&self.omegainv.clone(), self.exp);

        let minv = &self.minv;
        for v in self.coeffs.iter_mut() {
            *v = BLS12_381_SCALAR.mul(v.clone(), minv);
        }
    }
}
