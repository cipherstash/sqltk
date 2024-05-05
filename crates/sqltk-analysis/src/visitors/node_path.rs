//! Trait and types for tracking the path of an AST node from the root node.

use core::convert::Infallible;
use core::fmt::{self, Debug, Display};
use std::ops::Deref;

use sqlparser::ast::{Ident, IdentWithAlias, Join, ObjectName, TableFactor};
use sqltk::prelude::{Node, VisitorControlFlow};
use sqltk::{flow, Visitable, Visitor};

/// Operations for manipulating and retreiving the current node path.
pub trait NodePathOps<'ast> {
    fn push_path_entry<N>(&mut self, node: &'ast N)
    where
        &'ast N: Into<Node<'ast>>;

    fn pop_path_entry(&mut self) -> Option<NodePathEntry<'ast>>;

    fn peek_path_entry(&self) -> Option<&NodePathEntry<'ast>>;

    fn get_node_path(&self) -> &NodePath<'ast>;
}

/// A `NodePath` represents the path to the node within the AST.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct NodePath<'ast> {
    entries: Vec<NodePathEntry<'ast>>,
}

impl<'ast> Deref for NodePath<'ast> {
    type Target = Vec<NodePathEntry<'ast>>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<'ast> NodePath<'ast> {
    /// Creates a new empty `NodePath`
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(128),
        }
    }

    /// Returns a [`Visitor`] implementation that keeps the current `NodePath`
    /// up to date within every `Visitor::enter` and `Visitor::exit` call.
    pub fn track<State>() -> impl Visitor<'ast, State, Infallible>
    where
        State: NodePathOps<'ast>,
    {
        NodePathVisitor
    }

    /// Logs the full `NodePath` to stderr every time the current node path changes.
    ///
    /// Only available if `cfg(test)` is satisfied.
    #[cfg(test)]
    pub fn log_full_node_path<State>() -> impl Visitor<'ast, State, Infallible>
    where
        State: NodePathOps<'ast>,
    {
        LogFullNodePathVisitor
    }

    /// Logs the top entry of the `NodePath` to stderr every time the current node path changes.
    ///
    /// Only available if `cfg(test)` is satisfied.
    #[cfg(test)]
    pub fn log_top_entry<State>() -> impl Visitor<'ast, State, Infallible>
    where
        State: NodePathOps<'ast>,
    {
        LogTopEntryVisitor
    }
}

impl<'ast> Display for NodePath<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            self.entries
                .iter()
                .map(|entry| entry.to_string())
                .collect::<Vec<_>>()
                .join("/")
        ))
    }
}

struct NodePathVisitor;
struct LogFullNodePathVisitor;
struct LogTopEntryVisitor;

impl<'ast, State> Visitor<'ast, State, Infallible> for NodePathVisitor
where
    State: NodePathOps<'ast>,
{
    fn enter<N: 'static>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        state.push_path_entry(node);
        flow::cont(state)
    }

    fn exit<N: 'static>(
        &self,
        _: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        state.pop_path_entry();
        flow::cont(state)
    }
}

impl<'ast, State> Visitor<'ast, State, Infallible> for LogFullNodePathVisitor
where
    State: NodePathOps<'ast>,
{
    fn enter<N: 'static>(
        &self,
        _: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        eprintln!("ENTER: {}", state.get_node_path());
        flow::cont(state)
    }

    fn exit<N: 'static>(
        &self,
        _: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        eprintln!("EXIT:  {}", state.get_node_path());
        flow::cont(state)
    }
}

impl<'ast, State> Visitor<'ast, State, Infallible> for LogTopEntryVisitor
where
    State: NodePathOps<'ast>,
{
    fn enter<N: 'static>(
        &self,
        _: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        if let Some(entry) = state.peek_path_entry() {
            eprintln!("ENTER: {}", entry);
        }
        flow::cont(state)
    }

    fn exit<N: 'static>(
        &self,
        _: &'ast N,
        state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        if let Some(entry) = state.peek_path_entry() {
            eprintln!("EXIT:  {}", entry);
        }
        flow::cont(state)
    }
}

impl<'ast> NodePathOps<'ast> for NodePath<'ast> {
    fn push_path_entry<N>(&mut self, node: &'ast N)
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.entries.push(NodePathEntry {
            node: node.into(),
            depth: self.entries.len() as usize,
        })
    }

    fn pop_path_entry(&mut self) -> Option<NodePathEntry<'ast>> {
        self.entries.pop()
    }

    fn peek_path_entry(&self) -> Option<&NodePathEntry<'ast>> {
        self.entries.last()
    }

    fn get_node_path(&self) -> &NodePath<'ast> {
        self
    }
}

/// An entry for a single [`Node`] in a [`NodePath`].
///
/// Note that the `Display` implementation is useful only for debugging tests
/// and only provides custom formatting for a handful of [`Node`] variants.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodePathEntry<'ast> {
    pub depth: usize,
    pub node: Node<'ast>,
}

impl<'ast> NodePathEntry<'ast> {
    pub fn new<N>(node: &'ast N, depth: usize) -> Self
    where
        N: Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        Self {
            node: node.into(),
            depth,
        }
    }
}

impl<'ast> Debug for NodePathEntry<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodePathEntry")
            .field("node", &self.node)
            .field("depth", &self.depth)
            .finish()
    }
}

impl<'ast> Display for NodePathEntry<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node {
            Node::Ident(Ident { value: ident, .. }) => {
                f.write_fmt(format_args!("{} [{}]", self.node, ident))
            }
            Node::IdentWithAlias(IdentWithAlias {
                ident: Ident { value: ident, .. },
                alias,
            }) => f.write_fmt(format_args!("{} [{} as {}]", self.node, ident, alias)),
            Node::ObjectName(ObjectName(idents)) => f.write_fmt(format_args!(
                "{} [{}]",
                self.node,
                idents
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            )),
            Node::Value(value) => f.write_fmt(format_args!("{} ({})", self.node, value)),
            Node::Join(Join {
                relation: TableFactor::Table { name, alias, .. },
                ..
            }) => f.write_fmt(format_args!(
                "{} [{} as {}]",
                self.node,
                name,
                alias
                    .clone()
                    .map(|a| a.name.to_string())
                    .unwrap_or("Unaliased".to_owned())
            )),
            _ => f.write_str(&self.node.to_string()),
        }
    }
}
