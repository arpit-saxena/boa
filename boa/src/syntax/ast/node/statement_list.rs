//! Statement list node.

use super::Node;
use gc::{unsafe_empty_trace, Finalize, Trace};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// List of statements.
///
/// Similar to `Node::Block` but without the braces.
///
/// More information:
///  - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-StatementList
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct StatementList {
    #[cfg_attr(feature = "serde", serde(flatten))]
    statements: Box<[Node]>,
}

impl StatementList {
    /// Gets the list of statements.
    pub fn statements(&self) -> &[Node] {
        &self.statements
    }

    /// Implements the display formatting with indentation.
    pub(super) fn display(&self, f: &mut fmt::Formatter<'_>, indentation: usize) -> fmt::Result {
        let indent = "    ".repeat(indentation);
        // Print statements
        for node in self.statements.iter() {
            f.write_str(&indent)?;
            node.display(f, indentation + 1)?;

            match node {
                Node::Block(_) | Node::If(_) | Node::Switch(_) | Node::WhileLoop(_) => {}
                _ => write!(f, ";")?,
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> From<T> for StatementList
where
    T: Into<Box<[Node]>>,
{
    fn from(stm: T) -> Self {
        Self {
            statements: stm.into(),
        }
    }
}

impl fmt::Display for StatementList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

// List of statements wrapped with Rc. We need this for self mutating functions.
// Since we need to cheaply clone the function body and drop the borrow of the function object to
// mutably borrow the function object and call this cloned function body
#[derive(Clone, Debug, Finalize, PartialEq)]
pub struct RcStatementList(Rc<StatementList>);

impl Deref for RcStatementList {
    type Target = StatementList;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<StatementList> for RcStatementList {
    #[inline]
    fn from(statementlist: StatementList) -> Self {
        Self(Rc::from(statementlist))
    }
}

// SAFETY: This is safe for types not containing any `Trace` types.
unsafe impl Trace for RcStatementList {
    unsafe_empty_trace!();
}
