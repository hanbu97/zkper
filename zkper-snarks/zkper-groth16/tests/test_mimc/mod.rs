use rug::Integer;
use zkper_groth16::circuit::Circuit;

pub struct MiMCDemo<'a> {
    pub xl: Option<Integer>,
    pub xr: Option<Integer>,
    pub constants: &'a [Integer],
}

impl Circuit for MiMCDemo<'_> {
    fn synthesize(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
