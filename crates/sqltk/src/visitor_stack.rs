use core::marker::PhantomData;
use std::error::Error;
use std::fmt::Debug;

use crate::{flow, BoxOf, Break, Node, OptionOf, VecOf, Visitable, Visitor, VisitorControlFlow};

/// Composes multiple [`Visitor`] implementations into a single `Visitor`.
///
/// All `Visitor` implementations that are pushed onto the stack must have an
/// error type that satisifies `ErrorConversion<VE>: Into<E>`.
///
/// `VisitorStack` implements `Visitor`.
///
/// [`Visitor::enter`] is implemented such that `enter` is called on the child
/// visitors in the order they were pushed onto the stack.
///
/// For [`Visitor::exit`] the order is reversed.
pub struct VisitorStack<'ast, State, E>
where
    State: 'ast,
    E: 'static + Error + Debug,
{
    visitors: Vec<Box<dyn ObjectSafeVisitor<'ast, State, E> + 'ast>>,
}

impl<'ast, State, E> Default for VisitorStack<'ast, State, E>
where
    State: 'ast,
    E: 'static + Error + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'ast, State, E> VisitorStack<'ast, State, E>
where
    State: 'ast,
    E: 'static + Error + Debug,
{
    /// Creates a new empty `VisitorStack`.
    ///
    /// This allocates capacity for 16 visitors which will grow on demand. A
    /// [`Vec`] is used for internal storage.
    pub fn new() -> Self {
        Self {
            visitors: Vec::with_capacity(16),
        }
    }

    /// Pushes a new [`Visitor`] onto this `VisitorStack`.
    pub fn push<V, VE>(&mut self, visitor: V)
    where
        V: 'ast + Visitor<'ast, State, VE>,
        VE: 'ast + Error + Debug,
        ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
            Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
    {
        self.visitors
            .push(Box::new(ErrorConvertingVisitor::new(visitor))
                as Box<dyn ObjectSafeVisitor<'ast, State, E> + 'ast>);
    }
}

pub struct ConvertFrom<T>(T);
pub struct ConvertInto<T>(T);

impl<T> ConvertInto<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'ast, State, EFrom, EInto> From<ConvertFrom<VisitorControlFlow<'ast, State, EFrom>>>
    for ConvertInto<VisitorControlFlow<'ast, State, EInto>>
where
    EFrom: Error + Into<EInto>,
    EInto: Error,
{
    fn from(value: ConvertFrom<VisitorControlFlow<'ast, State, EFrom>>) -> Self {
        match value.0 {
            VisitorControlFlow::Continue(state) => ConvertInto(VisitorControlFlow::Continue(state)),
            VisitorControlFlow::Break(brk) => match brk {
                Break::SkipChildren(state) => {
                    ConvertInto(VisitorControlFlow::Break(Break::SkipChildren(state)))
                }
                Break::Finished(state) => {
                    ConvertInto(VisitorControlFlow::Break(Break::Finished(state)))
                }
                Break::Err(err, state) => {
                    ConvertInto(VisitorControlFlow::Break(Break::Err(err.into(), state)))
                }
            },
        }
    }
}

impl<'ast, State, E> Visitor<'ast, State, E> for VisitorStack<'ast, State, E>
where
    E: Error + Debug,
{
    fn enter<N>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, E>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        for visitor in self.visitors.iter() {
            state = visitor.object_safe_enter(node.into(), state)?;
        }
        flow::cont(state)
    }

    fn exit<N>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, E>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        for visitor in self.visitors.iter().rev() {
            state = visitor.object_safe_exit(node.into(), state)?;
        }
        flow::cont(state)
    }
}

/// A [`Visitor`] implementation that converts between error types.
struct ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    E: Error + Debug,
    VE: Error + Debug,
    V: Visitor<'ast, State, VE>,
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    visitor: V,
    _phantom: PhantomData<(&'ast (), State, E, VE)>,
}

impl<'ast, State, E, V, VE> ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    E: Error + Debug,
    VE: Error + Debug,
    V: Visitor<'ast, State, VE>,
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    fn new(visitor: V) -> Self {
        Self {
            visitor,
            _phantom: PhantomData,
        }
    }
}

impl<'ast, State, E, V, VE> ObjectSafeVisitor<'ast, State, E>
    for ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    V: Visitor<'ast, State, VE>,
    E: Error + Debug,
    VE: Error + Debug,
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    fn object_safe_enter(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E> {
        let result = match node {
            Node::Action(node) => self.visitor.enter(node, state),
            Node::AddDropSync(node) => self.visitor.enter(node, state),
            Node::AlterColumnOperation(node) => self.visitor.enter(node, state),
            Node::AlterIndexOperation(node) => self.visitor.enter(node, state),
            Node::AlterRoleOperation(node) => self.visitor.enter(node, state),
            Node::AlterTableOperation(node) => self.visitor.enter(node, state),
            Node::AnalyzeFormat(node) => self.visitor.enter(node, state),
            Node::ArgMode(node) => self.visitor.enter(node, state),
            Node::Array(node) => self.visitor.enter(node, state),
            Node::ArrayAgg(node) => self.visitor.enter(node, state),
            Node::ArrayElemTypeDef(node) => self.visitor.enter(node, state),
            Node::Assignment(node) => self.visitor.enter(node, state),
            Node::BinaryOperator(node) => self.visitor.enter(node, state),
            Node::CastFormat(node) => self.visitor.enter(node, state),
            Node::CharLengthUnits(node) => self.visitor.enter(node, state),
            Node::CharacterLength(node) => self.visitor.enter(node, state),
            Node::CloseCursor(node) => self.visitor.enter(node, state),
            Node::ColumnDef(node) => self.visitor.enter(node, state),
            Node::ColumnOption(node) => self.visitor.enter(node, state),
            Node::ColumnOptionDef(node) => self.visitor.enter(node, state),
            Node::CommentObject(node) => self.visitor.enter(node, state),
            Node::ConflictTarget(node) => self.visitor.enter(node, state),
            Node::ConstraintCharacteristics(node) => self.visitor.enter(node, state),
            Node::ContextModifier(node) => self.visitor.enter(node, state),
            Node::CopyLegacyCsvOption(node) => self.visitor.enter(node, state),
            Node::CopyLegacyOption(node) => self.visitor.enter(node, state),
            Node::CopyOption(node) => self.visitor.enter(node, state),
            Node::CopySource(node) => self.visitor.enter(node, state),
            Node::CopyTarget(node) => self.visitor.enter(node, state),
            Node::CreateFunctionBody(node) => self.visitor.enter(node, state),
            Node::CreateFunctionUsing(node) => self.visitor.enter(node, state),
            Node::CreateTableOptions(node) => self.visitor.enter(node, state),
            Node::Cte(node) => self.visitor.enter(node, state),
            Node::CteAsMaterialized(node) => self.visitor.enter(node, state),
            Node::DataType(node) => self.visitor.enter(node, state),
            Node::DateTimeField(node) => self.visitor.enter(node, state),
            Node::Declare(node) => self.visitor.enter(node, state),
            Node::DeclareAssignment(node) => self.visitor.enter(node, state),
            Node::DeclareType(node) => self.visitor.enter(node, state),
            Node::DeferrableInitial(node) => self.visitor.enter(node, state),
            Node::DescribeAlias(node) => self.visitor.enter(node, state),
            Node::DiscardObject(node) => self.visitor.enter(node, state),
            Node::Distinct(node) => self.visitor.enter(node, state),
            Node::DoUpdate(node) => self.visitor.enter(node, state),
            Node::DollarQuotedString(node) => self.visitor.enter(node, state),
            Node::DropFunctionDesc(node) => self.visitor.enter(node, state),
            Node::ExactNumberInfo(node) => self.visitor.enter(node, state),
            Node::ExceptSelectItem(node) => self.visitor.enter(node, state),
            Node::ExcludeSelectItem(node) => self.visitor.enter(node, state),
            Node::Expr(node) => self.visitor.enter(node, state),
            Node::Fetch(node) => self.visitor.enter(node, state),
            Node::FetchDirection(node) => self.visitor.enter(node, state),
            Node::FileFormat(node) => self.visitor.enter(node, state),
            Node::FlushLocation(node) => self.visitor.enter(node, state),
            Node::FlushType(node) => self.visitor.enter(node, state),
            Node::ForClause(node) => self.visitor.enter(node, state),
            Node::ForJson(node) => self.visitor.enter(node, state),
            Node::ForXml(node) => self.visitor.enter(node, state),
            Node::FromTable(node) => self.visitor.enter(node, state),
            Node::Function(node) => self.visitor.enter(node, state),
            Node::FunctionArg(node) => self.visitor.enter(node, state),
            Node::FunctionArgExpr(node) => self.visitor.enter(node, state),
            Node::FunctionArgOperator(node) => self.visitor.enter(node, state),
            Node::FunctionBehavior(node) => self.visitor.enter(node, state),
            Node::FunctionDefinition(node) => self.visitor.enter(node, state),
            Node::GeneratedAs(node) => self.visitor.enter(node, state),
            Node::GeneratedExpressionMode(node) => self.visitor.enter(node, state),
            Node::GrantObjects(node) => self.visitor.enter(node, state),
            Node::GroupByExpr(node) => self.visitor.enter(node, state),
            Node::HiveDelimiter(node) => self.visitor.enter(node, state),
            Node::HiveDescribeFormat(node) => self.visitor.enter(node, state),
            Node::HiveDistributionStyle(node) => self.visitor.enter(node, state),
            Node::HiveFormat(node) => self.visitor.enter(node, state),
            Node::HiveIOFormat(node) => self.visitor.enter(node, state),
            Node::HiveRowDelimiter(node) => self.visitor.enter(node, state),
            Node::HiveRowFormat(node) => self.visitor.enter(node, state),
            Node::HiveSetLocation(node) => self.visitor.enter(node, state),
            Node::Ident(node) => self.visitor.enter(node, state),
            Node::IdentWithAlias(node) => self.visitor.enter(node, state),
            Node::IndexType(node) => self.visitor.enter(node, state),
            Node::Interval(node) => self.visitor.enter(node, state),
            Node::Join(node) => self.visitor.enter(node, state),
            Node::JoinConstraint(node) => self.visitor.enter(node, state),
            Node::JoinOperator(node) => self.visitor.enter(node, state),
            Node::JsonOperator(node) => self.visitor.enter(node, state),
            Node::JsonTableColumn(node) => self.visitor.enter(node, state),
            Node::JsonTableColumnErrorHandling(node) => self.visitor.enter(node, state),
            Node::KeyOrIndexDisplay(node) => self.visitor.enter(node, state),
            Node::KillType(node) => self.visitor.enter(node, state),
            Node::LateralView(node) => self.visitor.enter(node, state),
            Node::ListAgg(node) => self.visitor.enter(node, state),
            Node::ListAggOnOverflow(node) => self.visitor.enter(node, state),
            Node::LockClause(node) => self.visitor.enter(node, state),
            Node::LockTable(node) => self.visitor.enter(node, state),
            Node::LockTableType(node) => self.visitor.enter(node, state),
            Node::LockType(node) => self.visitor.enter(node, state),
            Node::MacroArg(node) => self.visitor.enter(node, state),
            Node::MacroDefinition(node) => self.visitor.enter(node, state),
            Node::MergeClause(node) => self.visitor.enter(node, state),
            Node::MinMaxValue(node) => self.visitor.enter(node, state),
            Node::MysqlInsertPriority(node) => self.visitor.enter(node, state),
            Node::NamedWindowDefinition(node) => self.visitor.enter(node, state),
            Node::NonBlock(node) => self.visitor.enter(node, state),
            Node::NullTreatment(node) => self.visitor.enter(node, state),
            Node::ObjectName(node) => self.visitor.enter(node, state),
            Node::ObjectType(node) => self.visitor.enter(node, state),
            Node::Offset(node) => self.visitor.enter(node, state),
            Node::OffsetRows(node) => self.visitor.enter(node, state),
            Node::OnCommit(node) => self.visitor.enter(node, state),
            Node::OnConflict(node) => self.visitor.enter(node, state),
            Node::OnConflictAction(node) => self.visitor.enter(node, state),
            Node::OnInsert(node) => self.visitor.enter(node, state),
            Node::OperateFunctionArg(node) => self.visitor.enter(node, state),
            Node::OrderByExpr(node) => self.visitor.enter(node, state),
            Node::Partition(node) => self.visitor.enter(node, state),
            Node::Password(node) => self.visitor.enter(node, state),
            Node::Privileges(node) => self.visitor.enter(node, state),
            Node::ProcedureParam(node) => self.visitor.enter(node, state),
            Node::Query(node) => self.visitor.enter(node, state),
            Node::ReferentialAction(node) => self.visitor.enter(node, state),
            Node::RenameSelectItem(node) => self.visitor.enter(node, state),
            Node::ReplaceSelectElement(node) => self.visitor.enter(node, state),
            Node::ReplaceSelectItem(node) => self.visitor.enter(node, state),
            Node::ResetConfig(node) => self.visitor.enter(node, state),
            Node::RoleOption(node) => self.visitor.enter(node, state),
            Node::SchemaName(node) => self.visitor.enter(node, state),
            Node::SearchModifier(node) => self.visitor.enter(node, state),
            Node::Select(node) => self.visitor.enter(node, state),
            Node::SelectInto(node) => self.visitor.enter(node, state),
            Node::SelectItem(node) => self.visitor.enter(node, state),
            Node::SequenceOptions(node) => self.visitor.enter(node, state),
            Node::SetConfigValue(node) => self.visitor.enter(node, state),
            Node::SetExpr(node) => self.visitor.enter(node, state),
            Node::SetOperator(node) => self.visitor.enter(node, state),
            Node::SetQuantifier(node) => self.visitor.enter(node, state),
            Node::ShowCreateObject(node) => self.visitor.enter(node, state),
            Node::ShowStatementFilter(node) => self.visitor.enter(node, state),
            Node::SqlOption(node) => self.visitor.enter(node, state),
            Node::SqliteOnConflict(node) => self.visitor.enter(node, state),
            Node::Statement(node) => self.visitor.enter(node, state),
            Node::StructField(node) => self.visitor.enter(node, state),
            Node::Table(node) => self.visitor.enter(node, state),
            Node::TableAlias(node) => self.visitor.enter(node, state),
            Node::TableConstraint(node) => self.visitor.enter(node, state),
            Node::TableFactor(node) => self.visitor.enter(node, state),
            Node::TableVersion(node) => self.visitor.enter(node, state),
            Node::TableWithJoins(node) => self.visitor.enter(node, state),
            Node::TimezoneInfo(node) => self.visitor.enter(node, state),
            Node::Top(node) => self.visitor.enter(node, state),
            Node::TopQuantity(node) => self.visitor.enter(node, state),
            Node::TransactionAccessMode(node) => self.visitor.enter(node, state),
            Node::TransactionIsolationLevel(node) => self.visitor.enter(node, state),
            Node::TransactionMode(node) => self.visitor.enter(node, state),
            Node::TransactionModifier(node) => self.visitor.enter(node, state),
            Node::TrimWhereField(node) => self.visitor.enter(node, state),
            Node::UnaryOperator(node) => self.visitor.enter(node, state),
            Node::UserDefinedTypeCompositeAttributeDef(node) => self.visitor.enter(node, state),
            Node::UserDefinedTypeRepresentation(node) => self.visitor.enter(node, state),
            Node::Value(node) => self.visitor.enter(node, state),
            Node::ValueTableMode(node) => self.visitor.enter(node, state),
            Node::Values(node) => self.visitor.enter(node, state),
            Node::ViewColumnDef(node) => self.visitor.enter(node, state),
            Node::WildcardAdditionalOptions(node) => self.visitor.enter(node, state),
            Node::WindowFrame(node) => self.visitor.enter(node, state),
            Node::WindowFrameBound(node) => self.visitor.enter(node, state),
            Node::WindowFrameUnits(node) => self.visitor.enter(node, state),
            Node::WindowSpec(node) => self.visitor.enter(node, state),
            Node::WindowType(node) => self.visitor.enter(node, state),
            Node::With(node) => self.visitor.enter(node, state),
            Node::DataLoadingOption(node) => self.visitor.enter(node, state),
            Node::DataLoadingOptionType(node) => self.visitor.enter(node, state),
            Node::DataLoadingOptions(node) => self.visitor.enter(node, state),
            Node::StageLoadSelectItem(node) => self.visitor.enter(node, state),
            Node::StageParamsObject(node) => self.visitor.enter(node, state),
            Node::Keyword(node) => self.visitor.enter(node, state),
            Node::Token(node) => self.visitor.enter(node, state),
            Node::Whitespace(node) => self.visitor.enter(node, state),
            Node::Word(node) => self.visitor.enter(node, state),
            Node::BigDecimal(node) => self.visitor.enter(node, state),
            Node::Bool(node) => self.visitor.enter(node, state),
            Node::Char(node) => self.visitor.enter(node, state),
            Node::I16(node) => self.visitor.enter(node, state),
            Node::I32(node) => self.visitor.enter(node, state),
            Node::I64(node) => self.visitor.enter(node, state),
            Node::I8(node) => self.visitor.enter(node, state),
            Node::String(node) => self.visitor.enter(node, state),
            Node::U16(node) => self.visitor.enter(node, state),
            Node::U32(node) => self.visitor.enter(node, state),
            Node::U64(node) => self.visitor.enter(node, state),
            Node::U8(node) => self.visitor.enter(node, state),
            Node::Box(node) => match node {
                BoxOf::DataType(node) => self.visitor.enter(node, state),
                BoxOf::Expr(node) => self.visitor.enter(node, state),
                BoxOf::Query(node) => self.visitor.enter(node, state),
                BoxOf::ReplaceSelectElement(node) => self.visitor.enter(node, state),
                BoxOf::Select(node) => self.visitor.enter(node, state),
                BoxOf::SetExpr(node) => self.visitor.enter(node, state),
                BoxOf::Statement(node) => self.visitor.enter(node, state),
                BoxOf::Table(node) => self.visitor.enter(node, state),
                BoxOf::TableFactor(node) => self.visitor.enter(node, state),
                BoxOf::TableWithJoins(node) => self.visitor.enter(node, state),
            },
            Node::Option(node) => match node {
                OptionOf::BoxOfExpr(node) => self.visitor.enter(node, state),
                OptionOf::BoxOfQuery(node) => self.visitor.enter(node, state),
                OptionOf::String(node) => self.visitor.enter(node, state),
                OptionOf::VecOfString(node) => self.visitor.enter(node, state),
                OptionOf::VecOfExpr(node) => self.visitor.enter(node, state),
                OptionOf::VecOfFunctionArg(node) => self.visitor.enter(node, state),
                OptionOf::VecOfIdent(node) => self.visitor.enter(node, state),
                OptionOf::VecOfMacroArg(node) => self.visitor.enter(node, state),
                OptionOf::VecOfOperateFunctionArg(node) => self.visitor.enter(node, state),
                OptionOf::VecOfOrderByExpr(node) => self.visitor.enter(node, state),
                OptionOf::VecOfProcedureParam(node) => self.visitor.enter(node, state),
                OptionOf::VecOfSelectItem(node) => self.visitor.enter(node, state),
                OptionOf::VecOfSequenceOptions(node) => self.visitor.enter(node, state),
                OptionOf::VecOfSqlOption(node) => self.visitor.enter(node, state),
                OptionOf::VecOfTableWithJoins(node) => self.visitor.enter(node, state),
                OptionOf::VecOfStageLoadSelectItem(node) => self.visitor.enter(node, state),
                OptionOf::Bool(node) => self.visitor.enter(node, state),
                OptionOf::Char(node) => self.visitor.enter(node, state),
                OptionOf::AddDropSync(node) => self.visitor.enter(node, state),
                OptionOf::AnalyzeFormat(node) => self.visitor.enter(node, state),
                OptionOf::ArgMode(node) => self.visitor.enter(node, state),
                OptionOf::CastFormat(node) => self.visitor.enter(node, state),
                OptionOf::CharLengthUnits(node) => self.visitor.enter(node, state),
                OptionOf::CharacterLength(node) => self.visitor.enter(node, state),
                OptionOf::ConflictTarget(node) => self.visitor.enter(node, state),
                OptionOf::ConstraintCharacteristics(node) => self.visitor.enter(node, state),
                OptionOf::CreateFunctionUsing(node) => self.visitor.enter(node, state),
                OptionOf::CteAsMaterialized(node) => self.visitor.enter(node, state),
                OptionOf::DataType(node) => self.visitor.enter(node, state),
                OptionOf::DateTimeField(node) => self.visitor.enter(node, state),
                OptionOf::DeclareAssignment(node) => self.visitor.enter(node, state),
                OptionOf::DeclareType(node) => self.visitor.enter(node, state),
                OptionOf::DeferrableInitial(node) => self.visitor.enter(node, state),
                OptionOf::Distinct(node) => self.visitor.enter(node, state),
                OptionOf::ExceptSelectItem(node) => self.visitor.enter(node, state),
                OptionOf::ExcludeSelectItem(node) => self.visitor.enter(node, state),
                OptionOf::Expr(node) => self.visitor.enter(node, state),
                OptionOf::Fetch(node) => self.visitor.enter(node, state),
                OptionOf::FileFormat(node) => self.visitor.enter(node, state),
                OptionOf::FlushLocation(node) => self.visitor.enter(node, state),
                OptionOf::ForClause(node) => self.visitor.enter(node, state),
                OptionOf::FunctionBehavior(node) => self.visitor.enter(node, state),
                OptionOf::FunctionDefinition(node) => self.visitor.enter(node, state),
                OptionOf::GeneratedAs(node) => self.visitor.enter(node, state),
                OptionOf::GeneratedExpressionMode(node) => self.visitor.enter(node, state),
                OptionOf::HiveDescribeFormat(node) => self.visitor.enter(node, state),
                OptionOf::HiveFormat(node) => self.visitor.enter(node, state),
                OptionOf::HiveIOFormat(node) => self.visitor.enter(node, state),
                OptionOf::HiveRowFormat(node) => self.visitor.enter(node, state),
                OptionOf::HiveSetLocation(node) => self.visitor.enter(node, state),
                OptionOf::Ident(node) => self.visitor.enter(node, state),
                OptionOf::IndexType(node) => self.visitor.enter(node, state),
                OptionOf::JsonTableColumnErrorHandling(node) => self.visitor.enter(node, state),
                OptionOf::KillType(node) => self.visitor.enter(node, state),
                OptionOf::ListAggOnOverflow(node) => self.visitor.enter(node, state),
                OptionOf::MysqlInsertPriority(node) => self.visitor.enter(node, state),
                OptionOf::NonBlock(node) => self.visitor.enter(node, state),
                OptionOf::NullTreatment(node) => self.visitor.enter(node, state),
                OptionOf::ObjectName(node) => self.visitor.enter(node, state),
                OptionOf::Offset(node) => self.visitor.enter(node, state),
                OptionOf::OnCommit(node) => self.visitor.enter(node, state),
                OptionOf::OnInsert(node) => self.visitor.enter(node, state),
                OptionOf::Password(node) => self.visitor.enter(node, state),
                OptionOf::Query(node) => self.visitor.enter(node, state),
                OptionOf::ReferentialAction(node) => self.visitor.enter(node, state),
                OptionOf::RenameSelectItem(node) => self.visitor.enter(node, state),
                OptionOf::ReplaceSelectItem(node) => self.visitor.enter(node, state),
                OptionOf::SearchModifier(node) => self.visitor.enter(node, state),
                OptionOf::SelectInto(node) => self.visitor.enter(node, state),
                OptionOf::ShowStatementFilter(node) => self.visitor.enter(node, state),
                OptionOf::SqliteOnConflict(node) => self.visitor.enter(node, state),
                OptionOf::TableAlias(node) => self.visitor.enter(node, state),
                OptionOf::TableVersion(node) => self.visitor.enter(node, state),
                OptionOf::TableWithJoins(node) => self.visitor.enter(node, state),
                OptionOf::Top(node) => self.visitor.enter(node, state),
                OptionOf::TopQuantity(node) => self.visitor.enter(node, state),
                OptionOf::TransactionModifier(node) => self.visitor.enter(node, state),
                OptionOf::TrimWhereField(node) => self.visitor.enter(node, state),
                OptionOf::Value(node) => self.visitor.enter(node, state),
                OptionOf::ValueTableMode(node) => self.visitor.enter(node, state),
                OptionOf::WindowFrame(node) => self.visitor.enter(node, state),
                OptionOf::WindowFrameBound(node) => self.visitor.enter(node, state),
                OptionOf::WindowType(node) => self.visitor.enter(node, state),
                OptionOf::With(node) => self.visitor.enter(node, state),
                OptionOf::U32(node) => self.visitor.enter(node, state),
                OptionOf::U64(node) => self.visitor.enter(node, state),
            },
            Node::Vec(node) => match node {
                VecOf::BoxOfReplaceSelectElement(node) => self.visitor.enter(node, state),
                VecOf::OptionOfString(node) => self.visitor.enter(node, state),
                VecOf::String(node) => self.visitor.enter(node, state),
                VecOf::VecOfExpr(node) => self.visitor.enter(node, state),
                VecOf::Action(node) => self.visitor.enter(node, state),
                VecOf::AlterTableOperation(node) => self.visitor.enter(node, state),
                VecOf::Assignment(node) => self.visitor.enter(node, state),
                VecOf::ColumnDef(node) => self.visitor.enter(node, state),
                VecOf::ColumnOption(node) => self.visitor.enter(node, state),
                VecOf::ColumnOptionDef(node) => self.visitor.enter(node, state),
                VecOf::CopyLegacyCsvOption(node) => self.visitor.enter(node, state),
                VecOf::CopyLegacyOption(node) => self.visitor.enter(node, state),
                VecOf::CopyOption(node) => self.visitor.enter(node, state),
                VecOf::Cte(node) => self.visitor.enter(node, state),
                VecOf::DataType(node) => self.visitor.enter(node, state),
                VecOf::Declare(node) => self.visitor.enter(node, state),
                VecOf::DropFunctionDesc(node) => self.visitor.enter(node, state),
                VecOf::Expr(node) => self.visitor.enter(node, state),
                VecOf::FunctionArg(node) => self.visitor.enter(node, state),
                VecOf::HiveRowDelimiter(node) => self.visitor.enter(node, state),
                VecOf::Ident(node) => self.visitor.enter(node, state),
                VecOf::IdentWithAlias(node) => self.visitor.enter(node, state),
                VecOf::Join(node) => self.visitor.enter(node, state),
                VecOf::JsonTableColumn(node) => self.visitor.enter(node, state),
                VecOf::LateralView(node) => self.visitor.enter(node, state),
                VecOf::LockClause(node) => self.visitor.enter(node, state),
                VecOf::LockTable(node) => self.visitor.enter(node, state),
                VecOf::MacroArg(node) => self.visitor.enter(node, state),
                VecOf::MergeClause(node) => self.visitor.enter(node, state),
                VecOf::NamedWindowDefinition(node) => self.visitor.enter(node, state),
                VecOf::ObjectName(node) => self.visitor.enter(node, state),
                VecOf::OperateFunctionArg(node) => self.visitor.enter(node, state),
                VecOf::OrderByExpr(node) => self.visitor.enter(node, state),
                VecOf::Partition(node) => self.visitor.enter(node, state),
                VecOf::ProcedureParam(node) => self.visitor.enter(node, state),
                VecOf::RoleOption(node) => self.visitor.enter(node, state),
                VecOf::SelectItem(node) => self.visitor.enter(node, state),
                VecOf::SequenceOptions(node) => self.visitor.enter(node, state),
                VecOf::SqlOption(node) => self.visitor.enter(node, state),
                VecOf::Statement(node) => self.visitor.enter(node, state),
                VecOf::StructField(node) => self.visitor.enter(node, state),
                VecOf::TableConstraint(node) => self.visitor.enter(node, state),
                VecOf::TableWithJoins(node) => self.visitor.enter(node, state),
                VecOf::TransactionMode(node) => self.visitor.enter(node, state),
                VecOf::UserDefinedTypeCompositeAttributeDef(node) => {
                    self.visitor.enter(node, state)
                }
                VecOf::Value(node) => self.visitor.enter(node, state),
                VecOf::ViewColumnDef(node) => self.visitor.enter(node, state),
                VecOf::DataLoadingOption(node) => self.visitor.enter(node, state),
                VecOf::StageLoadSelectItem(node) => self.visitor.enter(node, state),
                VecOf::Token(node) => self.visitor.enter(node, state),
            },
        };
        ConvertFrom(result).into().into_inner()
    }

    fn object_safe_exit(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E> {
        let result = match node {
            Node::Action(node) => self.visitor.exit(node, state),
            Node::AddDropSync(node) => self.visitor.exit(node, state),
            Node::AlterColumnOperation(node) => self.visitor.exit(node, state),
            Node::AlterIndexOperation(node) => self.visitor.exit(node, state),
            Node::AlterRoleOperation(node) => self.visitor.exit(node, state),
            Node::AlterTableOperation(node) => self.visitor.exit(node, state),
            Node::AnalyzeFormat(node) => self.visitor.exit(node, state),
            Node::ArgMode(node) => self.visitor.exit(node, state),
            Node::Array(node) => self.visitor.exit(node, state),
            Node::ArrayAgg(node) => self.visitor.exit(node, state),
            Node::ArrayElemTypeDef(node) => self.visitor.exit(node, state),
            Node::Assignment(node) => self.visitor.exit(node, state),
            Node::BinaryOperator(node) => self.visitor.exit(node, state),
            Node::CastFormat(node) => self.visitor.exit(node, state),
            Node::CharLengthUnits(node) => self.visitor.exit(node, state),
            Node::CharacterLength(node) => self.visitor.exit(node, state),
            Node::CloseCursor(node) => self.visitor.exit(node, state),
            Node::ColumnDef(node) => self.visitor.exit(node, state),
            Node::ColumnOption(node) => self.visitor.exit(node, state),
            Node::ColumnOptionDef(node) => self.visitor.exit(node, state),
            Node::CommentObject(node) => self.visitor.exit(node, state),
            Node::ConflictTarget(node) => self.visitor.exit(node, state),
            Node::ConstraintCharacteristics(node) => self.visitor.exit(node, state),
            Node::ContextModifier(node) => self.visitor.exit(node, state),
            Node::CopyLegacyCsvOption(node) => self.visitor.exit(node, state),
            Node::CopyLegacyOption(node) => self.visitor.exit(node, state),
            Node::CopyOption(node) => self.visitor.exit(node, state),
            Node::CopySource(node) => self.visitor.exit(node, state),
            Node::CopyTarget(node) => self.visitor.exit(node, state),
            Node::CreateFunctionBody(node) => self.visitor.exit(node, state),
            Node::CreateFunctionUsing(node) => self.visitor.exit(node, state),
            Node::CreateTableOptions(node) => self.visitor.exit(node, state),
            Node::Cte(node) => self.visitor.exit(node, state),
            Node::CteAsMaterialized(node) => self.visitor.exit(node, state),
            Node::DataType(node) => self.visitor.exit(node, state),
            Node::DateTimeField(node) => self.visitor.exit(node, state),
            Node::Declare(node) => self.visitor.exit(node, state),
            Node::DeclareAssignment(node) => self.visitor.exit(node, state),
            Node::DeclareType(node) => self.visitor.exit(node, state),
            Node::DeferrableInitial(node) => self.visitor.exit(node, state),
            Node::DescribeAlias(node) => self.visitor.exit(node, state),
            Node::DiscardObject(node) => self.visitor.exit(node, state),
            Node::Distinct(node) => self.visitor.exit(node, state),
            Node::DoUpdate(node) => self.visitor.exit(node, state),
            Node::DollarQuotedString(node) => self.visitor.exit(node, state),
            Node::DropFunctionDesc(node) => self.visitor.exit(node, state),
            Node::ExactNumberInfo(node) => self.visitor.exit(node, state),
            Node::ExceptSelectItem(node) => self.visitor.exit(node, state),
            Node::ExcludeSelectItem(node) => self.visitor.exit(node, state),
            Node::Expr(node) => self.visitor.exit(node, state),
            Node::Fetch(node) => self.visitor.exit(node, state),
            Node::FetchDirection(node) => self.visitor.exit(node, state),
            Node::FileFormat(node) => self.visitor.exit(node, state),
            Node::FlushLocation(node) => self.visitor.exit(node, state),
            Node::FlushType(node) => self.visitor.exit(node, state),
            Node::ForClause(node) => self.visitor.exit(node, state),
            Node::ForJson(node) => self.visitor.exit(node, state),
            Node::ForXml(node) => self.visitor.exit(node, state),
            Node::FromTable(node) => self.visitor.exit(node, state),
            Node::Function(node) => self.visitor.exit(node, state),
            Node::FunctionArg(node) => self.visitor.exit(node, state),
            Node::FunctionArgExpr(node) => self.visitor.exit(node, state),
            Node::FunctionArgOperator(node) => self.visitor.exit(node, state),
            Node::FunctionBehavior(node) => self.visitor.exit(node, state),
            Node::FunctionDefinition(node) => self.visitor.exit(node, state),
            Node::GeneratedAs(node) => self.visitor.exit(node, state),
            Node::GeneratedExpressionMode(node) => self.visitor.exit(node, state),
            Node::GrantObjects(node) => self.visitor.exit(node, state),
            Node::GroupByExpr(node) => self.visitor.exit(node, state),
            Node::HiveDelimiter(node) => self.visitor.exit(node, state),
            Node::HiveDescribeFormat(node) => self.visitor.exit(node, state),
            Node::HiveDistributionStyle(node) => self.visitor.exit(node, state),
            Node::HiveFormat(node) => self.visitor.exit(node, state),
            Node::HiveIOFormat(node) => self.visitor.exit(node, state),
            Node::HiveRowDelimiter(node) => self.visitor.exit(node, state),
            Node::HiveRowFormat(node) => self.visitor.exit(node, state),
            Node::HiveSetLocation(node) => self.visitor.exit(node, state),
            Node::Ident(node) => self.visitor.exit(node, state),
            Node::IdentWithAlias(node) => self.visitor.exit(node, state),
            Node::IndexType(node) => self.visitor.exit(node, state),
            Node::Interval(node) => self.visitor.exit(node, state),
            Node::Join(node) => self.visitor.exit(node, state),
            Node::JoinConstraint(node) => self.visitor.exit(node, state),
            Node::JoinOperator(node) => self.visitor.exit(node, state),
            Node::JsonOperator(node) => self.visitor.exit(node, state),
            Node::JsonTableColumn(node) => self.visitor.exit(node, state),
            Node::JsonTableColumnErrorHandling(node) => self.visitor.exit(node, state),
            Node::KeyOrIndexDisplay(node) => self.visitor.exit(node, state),
            Node::KillType(node) => self.visitor.exit(node, state),
            Node::LateralView(node) => self.visitor.exit(node, state),
            Node::ListAgg(node) => self.visitor.exit(node, state),
            Node::ListAggOnOverflow(node) => self.visitor.exit(node, state),
            Node::LockClause(node) => self.visitor.exit(node, state),
            Node::LockTable(node) => self.visitor.exit(node, state),
            Node::LockTableType(node) => self.visitor.exit(node, state),
            Node::LockType(node) => self.visitor.exit(node, state),
            Node::MacroArg(node) => self.visitor.exit(node, state),
            Node::MacroDefinition(node) => self.visitor.exit(node, state),
            Node::MergeClause(node) => self.visitor.exit(node, state),
            Node::MinMaxValue(node) => self.visitor.exit(node, state),
            Node::MysqlInsertPriority(node) => self.visitor.exit(node, state),
            Node::NamedWindowDefinition(node) => self.visitor.exit(node, state),
            Node::NonBlock(node) => self.visitor.exit(node, state),
            Node::NullTreatment(node) => self.visitor.exit(node, state),
            Node::ObjectName(node) => self.visitor.exit(node, state),
            Node::ObjectType(node) => self.visitor.exit(node, state),
            Node::Offset(node) => self.visitor.exit(node, state),
            Node::OffsetRows(node) => self.visitor.exit(node, state),
            Node::OnCommit(node) => self.visitor.exit(node, state),
            Node::OnConflict(node) => self.visitor.exit(node, state),
            Node::OnConflictAction(node) => self.visitor.exit(node, state),
            Node::OnInsert(node) => self.visitor.exit(node, state),
            Node::OperateFunctionArg(node) => self.visitor.exit(node, state),
            Node::OrderByExpr(node) => self.visitor.exit(node, state),
            Node::Partition(node) => self.visitor.exit(node, state),
            Node::Password(node) => self.visitor.exit(node, state),
            Node::Privileges(node) => self.visitor.exit(node, state),
            Node::ProcedureParam(node) => self.visitor.exit(node, state),
            Node::Query(node) => self.visitor.exit(node, state),
            Node::ReferentialAction(node) => self.visitor.exit(node, state),
            Node::RenameSelectItem(node) => self.visitor.exit(node, state),
            Node::ReplaceSelectElement(node) => self.visitor.exit(node, state),
            Node::ReplaceSelectItem(node) => self.visitor.exit(node, state),
            Node::ResetConfig(node) => self.visitor.exit(node, state),
            Node::RoleOption(node) => self.visitor.exit(node, state),
            Node::SchemaName(node) => self.visitor.exit(node, state),
            Node::SearchModifier(node) => self.visitor.exit(node, state),
            Node::Select(node) => self.visitor.exit(node, state),
            Node::SelectInto(node) => self.visitor.exit(node, state),
            Node::SelectItem(node) => self.visitor.exit(node, state),
            Node::SequenceOptions(node) => self.visitor.exit(node, state),
            Node::SetConfigValue(node) => self.visitor.exit(node, state),
            Node::SetExpr(node) => self.visitor.exit(node, state),
            Node::SetOperator(node) => self.visitor.exit(node, state),
            Node::SetQuantifier(node) => self.visitor.exit(node, state),
            Node::ShowCreateObject(node) => self.visitor.exit(node, state),
            Node::ShowStatementFilter(node) => self.visitor.exit(node, state),
            Node::SqlOption(node) => self.visitor.exit(node, state),
            Node::SqliteOnConflict(node) => self.visitor.exit(node, state),
            Node::Statement(node) => self.visitor.exit(node, state),
            Node::StructField(node) => self.visitor.exit(node, state),
            Node::Table(node) => self.visitor.exit(node, state),
            Node::TableAlias(node) => self.visitor.exit(node, state),
            Node::TableConstraint(node) => self.visitor.exit(node, state),
            Node::TableFactor(node) => self.visitor.exit(node, state),
            Node::TableVersion(node) => self.visitor.exit(node, state),
            Node::TableWithJoins(node) => self.visitor.exit(node, state),
            Node::TimezoneInfo(node) => self.visitor.exit(node, state),
            Node::Top(node) => self.visitor.exit(node, state),
            Node::TopQuantity(node) => self.visitor.exit(node, state),
            Node::TransactionAccessMode(node) => self.visitor.exit(node, state),
            Node::TransactionIsolationLevel(node) => self.visitor.exit(node, state),
            Node::TransactionMode(node) => self.visitor.exit(node, state),
            Node::TransactionModifier(node) => self.visitor.exit(node, state),
            Node::TrimWhereField(node) => self.visitor.exit(node, state),
            Node::UnaryOperator(node) => self.visitor.exit(node, state),
            Node::UserDefinedTypeCompositeAttributeDef(node) => self.visitor.exit(node, state),
            Node::UserDefinedTypeRepresentation(node) => self.visitor.exit(node, state),
            Node::Value(node) => self.visitor.exit(node, state),
            Node::ValueTableMode(node) => self.visitor.exit(node, state),
            Node::Values(node) => self.visitor.exit(node, state),
            Node::ViewColumnDef(node) => self.visitor.exit(node, state),
            Node::WildcardAdditionalOptions(node) => self.visitor.exit(node, state),
            Node::WindowFrame(node) => self.visitor.exit(node, state),
            Node::WindowFrameBound(node) => self.visitor.exit(node, state),
            Node::WindowFrameUnits(node) => self.visitor.exit(node, state),
            Node::WindowSpec(node) => self.visitor.exit(node, state),
            Node::WindowType(node) => self.visitor.exit(node, state),
            Node::With(node) => self.visitor.exit(node, state),
            Node::DataLoadingOption(node) => self.visitor.exit(node, state),
            Node::DataLoadingOptionType(node) => self.visitor.exit(node, state),
            Node::DataLoadingOptions(node) => self.visitor.exit(node, state),
            Node::StageLoadSelectItem(node) => self.visitor.exit(node, state),
            Node::StageParamsObject(node) => self.visitor.exit(node, state),
            Node::Keyword(node) => self.visitor.exit(node, state),
            Node::Token(node) => self.visitor.exit(node, state),
            Node::Whitespace(node) => self.visitor.exit(node, state),
            Node::Word(node) => self.visitor.exit(node, state),
            Node::BigDecimal(node) => self.visitor.exit(node, state),
            Node::Bool(node) => self.visitor.exit(node, state),
            Node::Char(node) => self.visitor.exit(node, state),
            Node::I16(node) => self.visitor.exit(node, state),
            Node::I32(node) => self.visitor.exit(node, state),
            Node::I64(node) => self.visitor.exit(node, state),
            Node::I8(node) => self.visitor.exit(node, state),
            Node::String(node) => self.visitor.exit(node, state),
            Node::U16(node) => self.visitor.exit(node, state),
            Node::U32(node) => self.visitor.exit(node, state),
            Node::U64(node) => self.visitor.exit(node, state),
            Node::U8(node) => self.visitor.exit(node, state),
            Node::Box(node) => match node {
                BoxOf::DataType(node) => self.visitor.exit(node, state),
                BoxOf::Expr(node) => self.visitor.exit(node, state),
                BoxOf::Query(node) => self.visitor.exit(node, state),
                BoxOf::ReplaceSelectElement(node) => self.visitor.exit(node, state),
                BoxOf::Select(node) => self.visitor.exit(node, state),
                BoxOf::SetExpr(node) => self.visitor.exit(node, state),
                BoxOf::Statement(node) => self.visitor.exit(node, state),
                BoxOf::Table(node) => self.visitor.exit(node, state),
                BoxOf::TableFactor(node) => self.visitor.exit(node, state),
                BoxOf::TableWithJoins(node) => self.visitor.exit(node, state),
            },
            Node::Option(node) => match node {
                OptionOf::BoxOfExpr(node) => self.visitor.exit(node, state),
                OptionOf::BoxOfQuery(node) => self.visitor.exit(node, state),
                OptionOf::String(node) => self.visitor.exit(node, state),
                OptionOf::VecOfString(node) => self.visitor.exit(node, state),
                OptionOf::VecOfExpr(node) => self.visitor.exit(node, state),
                OptionOf::VecOfFunctionArg(node) => self.visitor.exit(node, state),
                OptionOf::VecOfIdent(node) => self.visitor.exit(node, state),
                OptionOf::VecOfMacroArg(node) => self.visitor.exit(node, state),
                OptionOf::VecOfOperateFunctionArg(node) => self.visitor.exit(node, state),
                OptionOf::VecOfOrderByExpr(node) => self.visitor.exit(node, state),
                OptionOf::VecOfProcedureParam(node) => self.visitor.exit(node, state),
                OptionOf::VecOfSelectItem(node) => self.visitor.exit(node, state),
                OptionOf::VecOfSequenceOptions(node) => self.visitor.exit(node, state),
                OptionOf::VecOfSqlOption(node) => self.visitor.exit(node, state),
                OptionOf::VecOfTableWithJoins(node) => self.visitor.exit(node, state),
                OptionOf::VecOfStageLoadSelectItem(node) => self.visitor.exit(node, state),
                OptionOf::Bool(node) => self.visitor.exit(node, state),
                OptionOf::Char(node) => self.visitor.exit(node, state),
                OptionOf::AddDropSync(node) => self.visitor.exit(node, state),
                OptionOf::AnalyzeFormat(node) => self.visitor.exit(node, state),
                OptionOf::ArgMode(node) => self.visitor.exit(node, state),
                OptionOf::CastFormat(node) => self.visitor.exit(node, state),
                OptionOf::CharLengthUnits(node) => self.visitor.exit(node, state),
                OptionOf::CharacterLength(node) => self.visitor.exit(node, state),
                OptionOf::ConflictTarget(node) => self.visitor.exit(node, state),
                OptionOf::ConstraintCharacteristics(node) => self.visitor.exit(node, state),
                OptionOf::CreateFunctionUsing(node) => self.visitor.exit(node, state),
                OptionOf::CteAsMaterialized(node) => self.visitor.exit(node, state),
                OptionOf::DataType(node) => self.visitor.exit(node, state),
                OptionOf::DateTimeField(node) => self.visitor.exit(node, state),
                OptionOf::DeclareAssignment(node) => self.visitor.exit(node, state),
                OptionOf::DeclareType(node) => self.visitor.exit(node, state),
                OptionOf::DeferrableInitial(node) => self.visitor.exit(node, state),
                OptionOf::Distinct(node) => self.visitor.exit(node, state),
                OptionOf::ExceptSelectItem(node) => self.visitor.exit(node, state),
                OptionOf::ExcludeSelectItem(node) => self.visitor.exit(node, state),
                OptionOf::Expr(node) => self.visitor.exit(node, state),
                OptionOf::Fetch(node) => self.visitor.exit(node, state),
                OptionOf::FileFormat(node) => self.visitor.exit(node, state),
                OptionOf::FlushLocation(node) => self.visitor.exit(node, state),
                OptionOf::ForClause(node) => self.visitor.exit(node, state),
                OptionOf::FunctionBehavior(node) => self.visitor.exit(node, state),
                OptionOf::FunctionDefinition(node) => self.visitor.exit(node, state),
                OptionOf::GeneratedAs(node) => self.visitor.exit(node, state),
                OptionOf::GeneratedExpressionMode(node) => self.visitor.exit(node, state),
                OptionOf::HiveDescribeFormat(node) => self.visitor.exit(node, state),
                OptionOf::HiveFormat(node) => self.visitor.exit(node, state),
                OptionOf::HiveIOFormat(node) => self.visitor.exit(node, state),
                OptionOf::HiveRowFormat(node) => self.visitor.exit(node, state),
                OptionOf::HiveSetLocation(node) => self.visitor.exit(node, state),
                OptionOf::Ident(node) => self.visitor.exit(node, state),
                OptionOf::IndexType(node) => self.visitor.exit(node, state),
                OptionOf::JsonTableColumnErrorHandling(node) => self.visitor.exit(node, state),
                OptionOf::KillType(node) => self.visitor.exit(node, state),
                OptionOf::ListAggOnOverflow(node) => self.visitor.exit(node, state),
                OptionOf::MysqlInsertPriority(node) => self.visitor.exit(node, state),
                OptionOf::NonBlock(node) => self.visitor.exit(node, state),
                OptionOf::NullTreatment(node) => self.visitor.exit(node, state),
                OptionOf::ObjectName(node) => self.visitor.exit(node, state),
                OptionOf::Offset(node) => self.visitor.exit(node, state),
                OptionOf::OnCommit(node) => self.visitor.exit(node, state),
                OptionOf::OnInsert(node) => self.visitor.exit(node, state),
                OptionOf::Password(node) => self.visitor.exit(node, state),
                OptionOf::Query(node) => self.visitor.exit(node, state),
                OptionOf::ReferentialAction(node) => self.visitor.exit(node, state),
                OptionOf::RenameSelectItem(node) => self.visitor.exit(node, state),
                OptionOf::ReplaceSelectItem(node) => self.visitor.exit(node, state),
                OptionOf::SearchModifier(node) => self.visitor.exit(node, state),
                OptionOf::SelectInto(node) => self.visitor.exit(node, state),
                OptionOf::ShowStatementFilter(node) => self.visitor.exit(node, state),
                OptionOf::SqliteOnConflict(node) => self.visitor.exit(node, state),
                OptionOf::TableAlias(node) => self.visitor.exit(node, state),
                OptionOf::TableVersion(node) => self.visitor.exit(node, state),
                OptionOf::TableWithJoins(node) => self.visitor.exit(node, state),
                OptionOf::Top(node) => self.visitor.exit(node, state),
                OptionOf::TopQuantity(node) => self.visitor.exit(node, state),
                OptionOf::TransactionModifier(node) => self.visitor.exit(node, state),
                OptionOf::TrimWhereField(node) => self.visitor.exit(node, state),
                OptionOf::Value(node) => self.visitor.exit(node, state),
                OptionOf::ValueTableMode(node) => self.visitor.exit(node, state),
                OptionOf::WindowFrame(node) => self.visitor.exit(node, state),
                OptionOf::WindowFrameBound(node) => self.visitor.exit(node, state),
                OptionOf::WindowType(node) => self.visitor.exit(node, state),
                OptionOf::With(node) => self.visitor.exit(node, state),
                OptionOf::U32(node) => self.visitor.exit(node, state),
                OptionOf::U64(node) => self.visitor.exit(node, state),
            },
            Node::Vec(node) => match node {
                VecOf::BoxOfReplaceSelectElement(node) => self.visitor.exit(node, state),
                VecOf::OptionOfString(node) => self.visitor.exit(node, state),
                VecOf::String(node) => self.visitor.exit(node, state),
                VecOf::VecOfExpr(node) => self.visitor.exit(node, state),
                VecOf::Action(node) => self.visitor.exit(node, state),
                VecOf::AlterTableOperation(node) => self.visitor.exit(node, state),
                VecOf::Assignment(node) => self.visitor.exit(node, state),
                VecOf::ColumnDef(node) => self.visitor.exit(node, state),
                VecOf::ColumnOption(node) => self.visitor.exit(node, state),
                VecOf::ColumnOptionDef(node) => self.visitor.exit(node, state),
                VecOf::CopyLegacyCsvOption(node) => self.visitor.exit(node, state),
                VecOf::CopyLegacyOption(node) => self.visitor.exit(node, state),
                VecOf::CopyOption(node) => self.visitor.exit(node, state),
                VecOf::Cte(node) => self.visitor.exit(node, state),
                VecOf::DataType(node) => self.visitor.exit(node, state),
                VecOf::Declare(node) => self.visitor.exit(node, state),
                VecOf::DropFunctionDesc(node) => self.visitor.exit(node, state),
                VecOf::Expr(node) => self.visitor.exit(node, state),
                VecOf::FunctionArg(node) => self.visitor.exit(node, state),
                VecOf::HiveRowDelimiter(node) => self.visitor.exit(node, state),
                VecOf::Ident(node) => self.visitor.exit(node, state),
                VecOf::IdentWithAlias(node) => self.visitor.exit(node, state),
                VecOf::Join(node) => self.visitor.exit(node, state),
                VecOf::JsonTableColumn(node) => self.visitor.exit(node, state),
                VecOf::LateralView(node) => self.visitor.exit(node, state),
                VecOf::LockClause(node) => self.visitor.exit(node, state),
                VecOf::LockTable(node) => self.visitor.exit(node, state),
                VecOf::MacroArg(node) => self.visitor.exit(node, state),
                VecOf::MergeClause(node) => self.visitor.exit(node, state),
                VecOf::NamedWindowDefinition(node) => self.visitor.exit(node, state),
                VecOf::ObjectName(node) => self.visitor.exit(node, state),
                VecOf::OperateFunctionArg(node) => self.visitor.exit(node, state),
                VecOf::OrderByExpr(node) => self.visitor.exit(node, state),
                VecOf::Partition(node) => self.visitor.exit(node, state),
                VecOf::ProcedureParam(node) => self.visitor.exit(node, state),
                VecOf::RoleOption(node) => self.visitor.exit(node, state),
                VecOf::SelectItem(node) => self.visitor.exit(node, state),
                VecOf::SequenceOptions(node) => self.visitor.exit(node, state),
                VecOf::SqlOption(node) => self.visitor.exit(node, state),
                VecOf::Statement(node) => self.visitor.exit(node, state),
                VecOf::StructField(node) => self.visitor.exit(node, state),
                VecOf::TableConstraint(node) => self.visitor.exit(node, state),
                VecOf::TableWithJoins(node) => self.visitor.exit(node, state),
                VecOf::TransactionMode(node) => self.visitor.exit(node, state),
                VecOf::UserDefinedTypeCompositeAttributeDef(node) => self.visitor.exit(node, state),
                VecOf::Value(node) => self.visitor.exit(node, state),
                VecOf::ViewColumnDef(node) => self.visitor.exit(node, state),
                VecOf::DataLoadingOption(node) => self.visitor.exit(node, state),
                VecOf::StageLoadSelectItem(node) => self.visitor.exit(node, state),
                VecOf::Token(node) => self.visitor.exit(node, state),
            },
        };
        ConvertFrom(result).into().into_inner()
    }
}

/// Variation of the [`Visitor`] trait where the node is passed as an enum
/// instead of as a generically typed reference. This trait is therefore object
/// safe.
trait ObjectSafeVisitor<'ast, State, E>
where
    E: Error + Debug,
{
    fn object_safe_enter(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>;

    fn object_safe_exit(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>;
}

#[cfg(test)]
mod test {
    use core::convert::Infallible;

    use crate::{flow, Visitable, VisitorStack};

    #[test]
    fn basic() {
        let ast = "OH HAI!".to_owned();

        let mut stack = VisitorStack::<usize, Infallible>::new();

        stack.push(String::on_enter(|_, mut state| {
            state += 1;
            flow::infallible::cont(state)
        }));

        match ast.evaluate(&stack, 0) {
            Ok(result) => assert_eq!(result, 1),
            Err(_) => panic!("Infallible!"),
        };
    }
}
