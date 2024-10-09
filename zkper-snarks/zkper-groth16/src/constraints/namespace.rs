use super::{ConstraintSystem, Variable};

/// A namespaced mutable reference to a constraint system.
#[derive(Debug)]
pub struct Namespace<'a> {
    pub inner: &'a mut ConstraintSystem,
}

impl<'a> From<&'a mut ConstraintSystem> for Namespace<'a> {
    fn from(inner: &'a mut ConstraintSystem) -> Self {
        Namespace { inner }
    }
}

impl<'a> Namespace<'a> {
    pub fn new(inner: &'a mut ConstraintSystem) -> Self {
        Namespace { inner }
    }

    /// Obtain the inner ConstraintSystem
    pub fn cs(&mut self) -> &mut ConstraintSystem {
        self.inner
    }

    /// Manually drop the namespace.
    pub fn drop(self) {
        drop(self)
    }

    pub fn one() -> Variable {
        ConstraintSystem::one()
    }

    pub fn new_private(&mut self) -> anyhow::Result<Variable> {
        self.inner.new_private()
    }

    pub fn new_public(&mut self) -> anyhow::Result<Variable> {
        self.inner.new_public()
    }
}
