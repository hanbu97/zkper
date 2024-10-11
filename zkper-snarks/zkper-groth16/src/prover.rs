use std::{collections::BTreeMap, sync::Arc};

use crate::{
    circuit::Circuit,
    constraints::{linear_combination::LinearCombination, Variable},
    evaluation_domain::EvaluationDomain,
    models::{proof::Proof, proving_parameters::ProvingParameters},
};
use anyhow::Ok;
use rand::RngCore;
use rug::Integer;
use zkper_curves::curves::bls12_381::{Bls12_381ScalarField, BLS12_381_SCALAR};
use zkper_curves::traits::field::FieldTrait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Density(pub Vec<bool>);

impl Density {
    pub fn new() -> Self {
        Density(vec![])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn set(&mut self, index: usize) {
        self.0[index] = true;
    }

    pub fn get(&self, index: usize) -> bool {
        self.0[index]
    }

    pub fn add_element(&mut self) {
        self.0.push(false);
    }

    pub fn inc(&mut self, idx: usize) {
        self.0[idx] = true;
    }

    pub fn count(&self) -> usize {
        self.0.iter().filter(|&&x| x).count()
    }
}

#[derive(Debug)]
pub struct ProvingSystem {
    // Density of queries
    pub a_private_density: Density,
    pub b_public_density: Density,
    pub b_private_density: Density,

    // Evaluations of A, B, C polynomials
    pub a: Vec<Integer>,
    pub b: Vec<Integer>,
    pub c: Vec<Integer>,

    // Assignments of variables
    pub public_assignment: Vec<Integer>,
    pub private_assignment: Vec<Integer>,
}

impl ProvingSystem {
    pub fn new_private(&mut self, val: Integer) -> anyhow::Result<Variable> {
        self.private_assignment.push(val);
        self.a_private_density.add_element();
        self.b_private_density.add_element();

        Ok(Variable::Private(self.private_assignment.len() - 1))
    }

    pub fn new_public(&mut self, val: Integer) -> anyhow::Result<Variable> {
        self.public_assignment.push(val);
        self.b_public_density.add_element();

        Ok(Variable::Public(self.public_assignment.len() - 1))
    }

    fn eval(
        lc: &LinearCombination,
        mut input_density: Option<&mut Density>,
        mut aux_density: Option<&mut Density>,
        input_assignment: &[Integer],
        aux_assignment: &[Integer],
    ) -> Integer {
        let mut acc = Integer::ZERO;

        for (index, coeff) in &lc.0 {
            let mut tmp;

            if !coeff.is_zero() {
                match index {
                    Variable::Public(i) => {
                        tmp = input_assignment[*i].clone();
                        if let Some(ref mut v) = input_density {
                            v.inc(*i);
                        }
                    }
                    Variable::Private(i) => {
                        tmp = aux_assignment[*i].clone();
                        if let Some(ref mut v) = aux_density {
                            v.inc(*i);
                        }
                    }
                }

                if coeff != Integer::ONE {
                    tmp = BLS12_381_SCALAR.mul(tmp, coeff);
                }

                acc = BLS12_381_SCALAR.add(acc, &tmp);
            }
        }

        acc
    }

    pub fn enforce(&mut self, a: LinearCombination, b: LinearCombination, c: LinearCombination) {
        self.a.push(Self::eval(
            &a,
            // Inputs have full density in the A query
            // because there are constraints of the
            // form x * 0 = 0 for each input.
            None,
            Some(&mut self.a_private_density),
            &self.public_assignment,
            &self.private_assignment,
        ));

        self.b.push(Self::eval(
            &b,
            Some(&mut self.b_public_density),
            Some(&mut self.b_private_density),
            &self.public_assignment,
            &self.private_assignment,
        ));

        self.c.push(Self::eval(
            &c,
            // There is no C polynomial query,
            // though there is an (beta)A + (alpha)B + C
            // query for all aux variables.
            // However, that query has full density.
            None,
            None,
            &self.public_assignment,
            &self.private_assignment,
        ));
    }
}

/// Create a Groth16 proof using randomness `r` and `s` and the provided
/// R1CS-to-QAP reduction.
pub fn create_proof<C: Circuit, R: RngCore>(
    circuit: C,
    params: &ProvingParameters,
    mut rng: &mut R,
    // ) -> anyhow::Result<Proof>
) -> anyhow::Result<()>
where
{
    let r = Bls12_381ScalarField::random(&mut rng);
    let s = Bls12_381ScalarField::random(&mut rng);

    let mut prover = ProvingSystem {
        a_private_density: Density::new(),
        b_public_density: Density::new(),
        b_private_density: Density::new(),
        a: vec![],
        b: vec![],
        c: vec![],
        public_assignment: vec![],
        private_assignment: vec![],
    };

    prover.new_public(Integer::from(1))?;

    circuit.synthesize_proof(&mut prover)?;

    for i in 0..prover.public_assignment.len() {
        let a = LinearCombination::new_variable(Variable::Public(i));
        let b: LinearCombination = LinearCombination::zero();
        let c = LinearCombination::zero();

        prover.enforce(a, b, c);
    }

    let verify_key = params.vk.clone();

    let mut a = EvaluationDomain::new(prover.a)?;
    let mut b = EvaluationDomain::new(prover.b)?;
    let mut c = EvaluationDomain::new(prover.c)?;

    a.ifft();
    a.coset_fft();
    b.ifft();
    b.coset_fft();
    c.ifft();
    c.coset_fft();

    a.mul_assign(&b);
    drop(b);
    a.sub_assign(&c);
    drop(c);
    a.divide_by_z_on_coset();
    a.icoset_fft();

    let mut a = a.coeffs;
    let a_len = a.len() - 1;
    a.truncate(a_len);

    // let a = Arc::new(a.into_iter().map(|s| s.into()).collect::<Vec<_>>());

    // create_proof(circuit, params, r, s)

    Ok(())
}
