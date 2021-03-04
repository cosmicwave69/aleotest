// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Enforces an array index expression in a compiled Leo program.

use crate::errors::ExpressionError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_asg::Expression;
use leo_asg::Span;

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub(crate) fn enforce_index<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        index: &'a Expression<'a>,
        span: &Span,
    ) -> Result<usize, ExpressionError> {
        match self.enforce_expression(cs, index)? {
            ConstrainedValue::Integer(number) => Ok(number.to_usize(span)?),
            value => Err(ExpressionError::invalid_index(value.to_string(), span)),
        }
    }
}
