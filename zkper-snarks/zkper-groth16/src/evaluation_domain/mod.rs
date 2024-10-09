use rug::Integer;

pub struct EvaluationDomain {
    pub coeffs: Vec<Integer>,
    pub exp: u32,
    pub omega: Integer,
    pub omegainv: Integer,
    pub geninv: Integer,
    pub minv: Integer,
}

// impl EvaluationDomain {
//     pub fn new(exp: u32, coeffs: Vec<Integer>) -> Self {
//         let n = 1 << exp;
//         let omega = Integer::from(2).pow(n).invert().unwrap();
//         let omegainv = omega.clone().invert().unwrap();
//         let geninv = Integer::from(2).pow(n).invert().unwrap();
//         let minv = Integer::from(n).invert().unwrap();

//         EvaluationDomain {
//             coeffs,
//             exp,
//             omega,
//             omegainv,
//             geninv,
//             minv,
//         }
//     }
// }
