use crate::{
    circuit::Circuit,
    constraints::{linear_combination::LinearCombination, Variable},
    evaluation_domain::EvaluationDomain,
    models::{proof::Proof, proving_parameters::ProvingParameters},
    multiexp::{multiexp, multiexp_g2},
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
) -> anyhow::Result<Proof> {
    if params.vk.delta_g1.is_identity() || params.vk.delta_g2.is_identity() {
        return Err(anyhow::anyhow!("Invalid verification key, ATTACK"));
    }

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

    let h_query = multiexp(params.h_query.clone(), None, None, a.into())?;
    let l_query = multiexp(
        params.l_query.clone(),
        None,
        None,
        prover.private_assignment.clone().into(),
    )?;

    let a_public = multiexp(
        params.a_query.clone(),
        None,
        None,
        prover.public_assignment.clone().into(),
    )?;
    let a_private = multiexp(
        params.a_query.clone(),
        prover.public_assignment.len().into(),
        Some(&prover.a_private_density),
        prover.private_assignment.clone().into(),
    )?;

    let b_g1_public = multiexp(
        params.b_g1_query.clone(),
        None,
        Some(&prover.b_public_density),
        prover.public_assignment.clone().into(),
    )?;
    let b_g1_private = multiexp(
        params.b_g1_query.clone(),
        prover.b_public_density.count().into(),
        Some(&prover.b_private_density),
        prover.private_assignment.clone().into(),
    )?;

    let b_g2_public = multiexp_g2(
        params.b_g2_query.clone(),
        None,
        Some(&prover.b_public_density),
        prover.public_assignment.clone().into(),
    )?;
    let b_g2_private = multiexp_g2(
        params.b_g2_query.clone(),
        prover.b_public_density.count().into(),
        Some(&prover.b_private_density),
        prover.private_assignment.clone().into(),
    )?;

    let r = Bls12_381ScalarField::random(&mut rng);
    let s = Bls12_381ScalarField::random(&mut rng);

    let mut g_a = verify_key.delta_g1.to_curve().mul_scalar(&r);
    g_a = g_a.add(&verify_key.alpha_g1.to_curve());

    let mut g_b = verify_key.delta_g2.to_curve().mul_scalar(&s);
    g_b = g_b.add(&verify_key.beta_g2.to_curve());

    let rs = BLS12_381_SCALAR.mul(r.clone(), &s);
    let mut g_c = verify_key.delta_g1.to_curve().mul_scalar(&rs);
    g_c = g_c.add(&(verify_key.alpha_g1.to_curve().mul_scalar(&s)));
    g_c = g_c.add(&(verify_key.beta_g1.to_curve().mul_scalar(&r)));

    let a_answer = a_public.add(&a_private);
    let g_a = g_a.add(&a_answer);

    let a_answer = a_answer.mul_scalar(&s);
    let g_c = g_c.add(&a_answer);

    let b1_answer = b_g1_public.add(&b_g1_private);
    let b2_answer = b_g2_public.add(&b_g2_private);

    let g_b = g_b.add(&b2_answer);

    let b1_answer = b1_answer.mul_scalar(&r);
    let g_c = g_c.add(&b1_answer);

    let g_c = g_c.add(&h_query);
    let g_c = g_c.add(&l_query);

    Ok(Proof {
        a: g_a.to_affine(),
        b: g_b.to_affine(),
        c: g_c.to_affine(),
    })
}
