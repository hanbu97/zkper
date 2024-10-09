use rug::Integer;
use zkper_curves::curves::bls12_381::Bls12_381ScalarField;

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

    pub at_public: Vec<Vec<(Bls12_381ScalarField, usize)>>,
    pub bt_public: Vec<Vec<(Bls12_381ScalarField, usize)>>,
    pub ct_public: Vec<Vec<(Bls12_381ScalarField, usize)>>,

    pub at_private: Vec<Vec<(Bls12_381ScalarField, usize)>>,
    pub bt_private: Vec<Vec<(Bls12_381ScalarField, usize)>>,
    pub ct_private: Vec<Vec<(Bls12_381ScalarField, usize)>>,
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

    // fn eval(
    //     l: LinearCombination<Scalar>,
    //     inputs: &mut [Vec<(Scalar, usize)>],
    //     aux: &mut [Vec<(Scalar, usize)>],
    //     this_constraint: usize,
    // ) {
    //     for (index, coeff) in l.as_ref() {
    //         match index.get_unchecked() {
    //             Index::Input(id) => inputs[id].push((*coeff, this_constraint)),
    //             Index::Aux(id) => aux[id].push((*coeff, this_constraint)),
    //         }
    //     }
    // }

    pub fn enforce_constraint() {
        unimplemented!()
    }
}
