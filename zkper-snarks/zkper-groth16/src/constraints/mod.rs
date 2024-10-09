use std::str::FromStr;

use rug::Integer;
use zkper_curves::curves::bls12_381::{Bls12_381ScalarField, BLS12_381_SCALAR};

use self::linear_combination::LinearCombination;

pub mod linear_combination;
pub mod namespace;

/// Represents the different kinds of variables present in a constraint system.
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Variable {
    /// Represents a public instance variable.
    Public(usize),
    /// Represents a private witness variable.
    Private(usize),
}

/// An Rank-One `ConstraintSystem`.
/// Used to synthesize the circuit into a QAP.
#[derive(Debug)]
pub struct ConstraintSystem {
    /// Number of public inputs to the constraint system.
    pub num_public_inputs: usize,
    /// Number of private inputs to the constraint system.
    pub num_private_inputs: usize,
    /// Number of constraints in the constraint system.
    pub num_constraints: usize,

    pub at_public: Vec<Vec<(Integer, usize)>>,
    pub bt_public: Vec<Vec<(Integer, usize)>>,
    pub ct_public: Vec<Vec<(Integer, usize)>>,

    pub at_private: Vec<Vec<(Integer, usize)>>,
    pub bt_private: Vec<Vec<(Integer, usize)>>,
    pub ct_private: Vec<Vec<(Integer, usize)>>,
}

impl ConstraintSystem {
    /// Creates a new empty `ConstraintSystem`.
    pub fn new() -> Self {
        ConstraintSystem {
            num_public_inputs: 1,
            num_private_inputs: 0,
            num_constraints: 0,
            at_public: vec![vec![]],
            bt_public: vec![vec![]],
            ct_public: vec![vec![]],
            at_private: vec![],
            bt_private: vec![],
            ct_private: vec![],
        }
    }

    /// Returns
    pub fn one() -> Variable {
        Variable::Public(0)
    }

    pub fn new_private(&mut self) -> anyhow::Result<Variable> {
        let current = self.num_private_inputs;

        self.at_private.push(vec![]);
        self.bt_private.push(vec![]);
        self.ct_private.push(vec![]);

        self.num_private_inputs += 1;

        Ok(Variable::Private(current))
    }

    pub fn new_public(&mut self) -> anyhow::Result<Variable> {
        let current = self.num_public_inputs;

        self.at_public.push(vec![]);
        self.bt_public.push(vec![]);
        self.ct_public.push(vec![]);

        self.num_public_inputs += 1;

        Ok(Variable::Public(current))
    }

    fn eval(
        linear_combination: LinearCombination,
        public_variables: &mut [Vec<(Integer, usize)>],
        private_variables: &mut [Vec<(Integer, usize)>],
        this_constraint: usize,
    ) {
        for (index, coeff) in &linear_combination.0 {
            match index {
                &Variable::Public(id) => {
                    public_variables[id].push((coeff.clone(), this_constraint))
                }
                &Variable::Private(id) => {
                    private_variables[id].push((coeff.clone(), this_constraint))
                }
            }
        }
    }

    pub fn enforce_constraint(
        &mut self,
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
    ) {
        Self::eval(
            a,
            &mut self.at_public,
            &mut self.at_private,
            self.num_constraints,
        );
        Self::eval(
            b,
            &mut self.bt_public,
            &mut self.bt_private,
            self.num_constraints,
        );
        Self::eval(
            c,
            &mut self.ct_public,
            &mut self.ct_private,
            self.num_constraints,
        );

        self.num_constraints += 1;
    }
}

#[test]
fn test_hex() {
    // let t = Integer::from_str("2ab00961a08a499d84dd396c349d9b3cc5e433d6fa78eb2b54cc39d9bb30bbb7")
    //     .unwrap();
    // println!("{}", t.to_string_radix(16));

    // let t = Integer::from_str_radix(
    //     "2ab00961a08a499d84dd396c349d9b3cc5e433d6fa78eb2b54cc39d9bb30bbb7",
    //     16,
    // )
    // .unwrap();

    // let t = BLS12_381_SCALAR.to_montgomery(&t);

    // println!("{}", t.to_string_radix(16));

    let t = Integer::from_str_radix(
        "16a2a19edfe81f20d09b681922c813b4b63683508c2280b93829971f439f0d2b",
        16,
    )
    .unwrap();
    println!("{}", t.to_string_radix(16));
    let t = BLS12_381_SCALAR.from_montgomery(&t);

    println!("{}", t.to_string_radix(16));

    let t = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();

    println!("{}", t.to_string_radix(16));
}
