use crate::visitable_impls::visit;
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Action {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Action::Connect => {}
                    sqlparser::ast::Action::Create => {}
                    sqlparser::ast::Action::Delete => {}
                    sqlparser::ast::Action::Execute => {}
                    sqlparser::ast::Action::Insert { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::Action::References { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::Action::Select { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::Action::Temporary => {}
                    sqlparser::ast::Action::Trigger => {}
                    sqlparser::ast::Action::Truncate => {}
                    sqlparser::ast::Action::Update { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::Action::Usage => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Action {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AddDropSync {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AddDropSync::ADD => {}
                    sqlparser::ast::AddDropSync::DROP => {}
                    sqlparser::ast::AddDropSync::SYNC => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AddDropSync {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AfterMatchSkip {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AfterMatchSkip::PastLastRow => {}
                    sqlparser::ast::AfterMatchSkip::ToNextRow => {}
                    sqlparser::ast::AfterMatchSkip::ToFirst(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::AfterMatchSkip::ToLast(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AfterMatchSkip {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AlterColumnOperation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AlterColumnOperation::SetNotNull => {}
                    sqlparser::ast::AlterColumnOperation::DropNotNull => {}
                    sqlparser::ast::AlterColumnOperation::SetDefault { value } => {
                        value.accept(visitor)?;
                    }
                    sqlparser::ast::AlterColumnOperation::DropDefault => {}
                    sqlparser::ast::AlterColumnOperation::SetDataType {
                        data_type,
                        using,
                    } => {
                        data_type.accept(visitor)?;
                        using.accept(visitor)?;
                    }
                    sqlparser::ast::AlterColumnOperation::AddGenerated {
                        generated_as,
                        sequence_options,
                    } => {
                        generated_as.accept(visitor)?;
                        sequence_options.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AlterColumnOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AlterIndexOperation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AlterIndexOperation::RenameIndex { index_name } => {
                        index_name.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AlterIndexOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AlterPolicyOperation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AlterPolicyOperation::Rename { new_name } => {
                        new_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterPolicyOperation::Apply {
                        to,
                        using,
                        with_check,
                    } => {
                        to.accept(visitor)?;
                        using.accept(visitor)?;
                        with_check.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AlterPolicyOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AlterRoleOperation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AlterRoleOperation::RenameRole { role_name } => {
                        role_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterRoleOperation::AddMember { member_name } => {
                        member_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterRoleOperation::DropMember { member_name } => {
                        member_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterRoleOperation::WithOptions { options } => {
                        options.accept(visitor)?;
                    }
                    sqlparser::ast::AlterRoleOperation::Set {
                        config_name,
                        config_value,
                        in_database,
                    } => {
                        config_name.accept(visitor)?;
                        config_value.accept(visitor)?;
                        in_database.accept(visitor)?;
                    }
                    sqlparser::ast::AlterRoleOperation::Reset {
                        config_name,
                        in_database,
                    } => {
                        config_name.accept(visitor)?;
                        in_database.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AlterRoleOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AlterTableOperation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AlterTableOperation::AddConstraint(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::AddColumn {
                        column_keyword,
                        if_not_exists,
                        column_def,
                        column_position,
                    } => {
                        column_keyword.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        column_def.accept(visitor)?;
                        column_position.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::AddProjection {
                        if_not_exists,
                        name,
                        select,
                    } => {
                        if_not_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        select.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DropProjection {
                        if_exists,
                        name,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::MaterializeProjection {
                        if_exists,
                        name,
                        partition,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        partition.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::ClearProjection {
                        if_exists,
                        name,
                        partition,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        partition.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity => {}
                    sqlparser::ast::AlterTableOperation::DisableRule { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DisableTrigger { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DropConstraint {
                        if_exists,
                        name,
                        cascade,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        cascade.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DropColumn {
                        column_name,
                        if_exists,
                        cascade,
                    } => {
                        column_name.accept(visitor)?;
                        if_exists.accept(visitor)?;
                        cascade.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::AttachPartition {
                        partition,
                    } => {
                        partition.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DetachPartition {
                        partition,
                    } => {
                        partition.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::FreezePartition {
                        partition,
                        with_name,
                    } => {
                        partition.accept(visitor)?;
                        with_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::UnfreezePartition {
                        partition,
                        with_name,
                    } => {
                        partition.accept(visitor)?;
                        with_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DropPrimaryKey => {}
                    sqlparser::ast::AlterTableOperation::EnableAlwaysRule { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger {
                        name,
                    } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::EnableReplicaRule { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::EnableReplicaTrigger {
                        name,
                    } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity => {}
                    sqlparser::ast::AlterTableOperation::EnableRule { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::EnableTrigger { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::RenamePartitions {
                        old_partitions,
                        new_partitions,
                    } => {
                        old_partitions.accept(visitor)?;
                        new_partitions.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::AddPartitions {
                        if_not_exists,
                        new_partitions,
                    } => {
                        if_not_exists.accept(visitor)?;
                        new_partitions.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::DropPartitions {
                        partitions,
                        if_exists,
                    } => {
                        partitions.accept(visitor)?;
                        if_exists.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::RenameColumn {
                        old_column_name,
                        new_column_name,
                    } => {
                        old_column_name.accept(visitor)?;
                        new_column_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::RenameTable { table_name } => {
                        table_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::ChangeColumn {
                        old_name,
                        new_name,
                        data_type,
                        options,
                        column_position,
                    } => {
                        old_name.accept(visitor)?;
                        new_name.accept(visitor)?;
                        data_type.accept(visitor)?;
                        options.accept(visitor)?;
                        column_position.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::ModifyColumn {
                        col_name,
                        data_type,
                        options,
                        column_position,
                    } => {
                        col_name.accept(visitor)?;
                        data_type.accept(visitor)?;
                        options.accept(visitor)?;
                        column_position.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::RenameConstraint {
                        old_name,
                        new_name,
                    } => {
                        old_name.accept(visitor)?;
                        new_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::AlterColumn {
                        column_name,
                        op,
                    } => {
                        column_name.accept(visitor)?;
                        op.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::SwapWith { table_name } => {
                        table_name.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::SetTblProperties {
                        table_properties,
                    } => {
                        table_properties.accept(visitor)?;
                    }
                    sqlparser::ast::AlterTableOperation::OwnerTo { new_owner } => {
                        new_owner.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AlterTableOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AnalyzeFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AnalyzeFormat::TEXT => {}
                    sqlparser::ast::AnalyzeFormat::GRAPHVIZ => {}
                    sqlparser::ast::AnalyzeFormat::JSON => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AnalyzeFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ArgMode {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ArgMode::In => {}
                    sqlparser::ast::ArgMode::Out => {}
                    sqlparser::ast::ArgMode::InOut => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ArgMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Array {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.elem.accept(visitor)?;
                self.named.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Array {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ArrayElemTypeDef {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ArrayElemTypeDef::None => {}
                    sqlparser::ast::ArrayElemTypeDef::AngleBracket(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ArrayElemTypeDef::SquareBracket(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::ArrayElemTypeDef::Parenthesis(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ArrayElemTypeDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Assignment {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.target.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Assignment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AssignmentTarget {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AssignmentTarget::ColumnName(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::AssignmentTarget::Tuple(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AssignmentTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::AttachDuckDBDatabaseOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::AttachDuckDBDatabaseOption::Type(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::AttachDuckDBDatabaseOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::BinaryOperator {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::BinaryOperator::Plus => {}
                    sqlparser::ast::BinaryOperator::Minus => {}
                    sqlparser::ast::BinaryOperator::Multiply => {}
                    sqlparser::ast::BinaryOperator::Divide => {}
                    sqlparser::ast::BinaryOperator::Modulo => {}
                    sqlparser::ast::BinaryOperator::StringConcat => {}
                    sqlparser::ast::BinaryOperator::Gt => {}
                    sqlparser::ast::BinaryOperator::Lt => {}
                    sqlparser::ast::BinaryOperator::GtEq => {}
                    sqlparser::ast::BinaryOperator::LtEq => {}
                    sqlparser::ast::BinaryOperator::Spaceship => {}
                    sqlparser::ast::BinaryOperator::Eq => {}
                    sqlparser::ast::BinaryOperator::NotEq => {}
                    sqlparser::ast::BinaryOperator::And => {}
                    sqlparser::ast::BinaryOperator::Or => {}
                    sqlparser::ast::BinaryOperator::Xor => {}
                    sqlparser::ast::BinaryOperator::BitwiseOr => {}
                    sqlparser::ast::BinaryOperator::BitwiseAnd => {}
                    sqlparser::ast::BinaryOperator::BitwiseXor => {}
                    sqlparser::ast::BinaryOperator::DuckIntegerDivide => {}
                    sqlparser::ast::BinaryOperator::MyIntegerDivide => {}
                    sqlparser::ast::BinaryOperator::Custom(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::BinaryOperator::PGBitwiseXor => {}
                    sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft => {}
                    sqlparser::ast::BinaryOperator::PGBitwiseShiftRight => {}
                    sqlparser::ast::BinaryOperator::PGExp => {}
                    sqlparser::ast::BinaryOperator::PGOverlap => {}
                    sqlparser::ast::BinaryOperator::PGRegexMatch => {}
                    sqlparser::ast::BinaryOperator::PGRegexIMatch => {}
                    sqlparser::ast::BinaryOperator::PGRegexNotMatch => {}
                    sqlparser::ast::BinaryOperator::PGRegexNotIMatch => {}
                    sqlparser::ast::BinaryOperator::PGLikeMatch => {}
                    sqlparser::ast::BinaryOperator::PGILikeMatch => {}
                    sqlparser::ast::BinaryOperator::PGNotLikeMatch => {}
                    sqlparser::ast::BinaryOperator::PGNotILikeMatch => {}
                    sqlparser::ast::BinaryOperator::PGStartsWith => {}
                    sqlparser::ast::BinaryOperator::Arrow => {}
                    sqlparser::ast::BinaryOperator::LongArrow => {}
                    sqlparser::ast::BinaryOperator::HashArrow => {}
                    sqlparser::ast::BinaryOperator::HashLongArrow => {}
                    sqlparser::ast::BinaryOperator::AtAt => {}
                    sqlparser::ast::BinaryOperator::AtArrow => {}
                    sqlparser::ast::BinaryOperator::ArrowAt => {}
                    sqlparser::ast::BinaryOperator::HashMinus => {}
                    sqlparser::ast::BinaryOperator::AtQuestion => {}
                    sqlparser::ast::BinaryOperator::Question => {}
                    sqlparser::ast::BinaryOperator::QuestionAnd => {}
                    sqlparser::ast::BinaryOperator::QuestionPipe => {}
                    sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::BinaryOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CastFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CastFormat::Value(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CastFormat::ValueAtTimeZone(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CastFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CastKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CastKind::Cast => {}
                    sqlparser::ast::CastKind::TryCast => {}
                    sqlparser::ast::CastKind::SafeCast => {}
                    sqlparser::ast::CastKind::DoubleColon => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CastKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CeilFloorKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CeilFloorKind::DateTimeField(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CeilFloorKind::Scale(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CeilFloorKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CharLengthUnits {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CharLengthUnits::Characters => {}
                    sqlparser::ast::CharLengthUnits::Octets => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CharLengthUnits {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CharacterLength {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CharacterLength::IntegerLength { length, unit } => {
                        length.accept(visitor)?;
                        unit.accept(visitor)?;
                    }
                    sqlparser::ast::CharacterLength::Max => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CharacterLength {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CloseCursor {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CloseCursor::All => {}
                    sqlparser::ast::CloseCursor::Specific { name } => {
                        name.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CloseCursor {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ClusteredBy {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.columns.accept(visitor)?;
                self.sorted_by.accept(visitor)?;
                self.num_buckets.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ClusteredBy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ClusteredIndex {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.asc.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ClusteredIndex {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ColumnDef {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.data_type.accept(visitor)?;
                self.collation.accept(visitor)?;
                self.options.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ColumnDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ColumnOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ColumnOption::Null => {}
                    sqlparser::ast::ColumnOption::NotNull => {}
                    sqlparser::ast::ColumnOption::Default(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Materialized(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Ephemeral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Alias(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Unique {
                        is_primary,
                        characteristics,
                    } => {
                        is_primary.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::ForeignKey {
                        foreign_table,
                        referred_columns,
                        on_delete,
                        on_update,
                        characteristics,
                    } => {
                        foreign_table.accept(visitor)?;
                        referred_columns.accept(visitor)?;
                        on_delete.accept(visitor)?;
                        on_update.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Check(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::DialectSpecific(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::CharacterSet(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Comment(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::OnUpdate(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Generated {
                        generated_as,
                        sequence_options,
                        generation_expr,
                        generation_expr_mode,
                        generated_keyword,
                    } => {
                        generated_as.accept(visitor)?;
                        sequence_options.accept(visitor)?;
                        generation_expr.accept(visitor)?;
                        generation_expr_mode.accept(visitor)?;
                        generated_keyword.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Options(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Identity(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::OnConflict(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Policy(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnOption::Tags(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ColumnOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ColumnOptionDef {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.option.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ColumnOptionDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ColumnPolicy {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ColumnPolicy::MaskingPolicy(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ColumnPolicy::ProjectionPolicy(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ColumnPolicy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ColumnPolicyProperty {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.with.accept(visitor)?;
                self.policy_name.accept(visitor)?;
                self.using_columns.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ColumnPolicyProperty {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CommentDef {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CommentDef::WithEq(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CommentDef::WithoutEq(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CommentDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CommentObject {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CommentObject::Column => {}
                    sqlparser::ast::CommentObject::Table => {}
                    sqlparser::ast::CommentObject::Extension => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CommentObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ConflictTarget {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ConflictTarget::Columns(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ConflictTarget::OnConstraint(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ConflictTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ConnectBy {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.condition.accept(visitor)?;
                self.relationships.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ConnectBy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ConstraintCharacteristics {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.deferrable.accept(visitor)?;
                self.initially.accept(visitor)?;
                self.enforced.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ConstraintCharacteristics {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ContextModifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ContextModifier::None => {}
                    sqlparser::ast::ContextModifier::Local => {}
                    sqlparser::ast::ContextModifier::Session => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ContextModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CopyLegacyCsvOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CopyLegacyCsvOption::Header => {}
                    sqlparser::ast::CopyLegacyCsvOption::Quote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyLegacyCsvOption::Escape(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyLegacyCsvOption::ForceQuote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CopyLegacyCsvOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CopyLegacyOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CopyLegacyOption::Binary => {}
                    sqlparser::ast::CopyLegacyOption::Delimiter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyLegacyOption::Null(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyLegacyOption::Csv(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CopyLegacyOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CopyOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CopyOption::Format(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Freeze(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Delimiter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Null(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Header(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Quote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Escape(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::ForceQuote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::ForceNotNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::ForceNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CopyOption::Encoding(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CopyOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CopySource {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CopySource::Table { table_name, columns } => {
                        table_name.accept(visitor)?;
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::CopySource::Query(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CopySource {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CopyTarget {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CopyTarget::Stdin => {}
                    sqlparser::ast::CopyTarget::Stdout => {}
                    sqlparser::ast::CopyTarget::File { filename } => {
                        filename.accept(visitor)?;
                    }
                    sqlparser::ast::CopyTarget::Program { command } => {
                        command.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CopyTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreateFunctionBody {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CreateFunctionBody::AsBeforeOptions(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CreateFunctionBody::AsAfterOptions(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CreateFunctionBody::Return(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreateFunctionBody {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreateFunctionUsing {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CreateFunctionUsing::Jar(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CreateFunctionUsing::File(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CreateFunctionUsing::Archive(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreateFunctionUsing {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreateIndex {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.table_name.accept(visitor)?;
                self.using.accept(visitor)?;
                self.columns.accept(visitor)?;
                self.unique.accept(visitor)?;
                self.concurrently.accept(visitor)?;
                self.if_not_exists.accept(visitor)?;
                self.include.accept(visitor)?;
                self.nulls_distinct.accept(visitor)?;
                self.with.accept(visitor)?;
                self.predicate.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreateIndex {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreatePolicyCommand {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CreatePolicyCommand::All => {}
                    sqlparser::ast::CreatePolicyCommand::Select => {}
                    sqlparser::ast::CreatePolicyCommand::Insert => {}
                    sqlparser::ast::CreatePolicyCommand::Update => {}
                    sqlparser::ast::CreatePolicyCommand::Delete => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreatePolicyCommand {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreatePolicyType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CreatePolicyType::Permissive => {}
                    sqlparser::ast::CreatePolicyType::Restrictive => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreatePolicyType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreateTable {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.or_replace.accept(visitor)?;
                self.temporary.accept(visitor)?;
                self.external.accept(visitor)?;
                self.global.accept(visitor)?;
                self.if_not_exists.accept(visitor)?;
                self.transient.accept(visitor)?;
                self.volatile.accept(visitor)?;
                self.name.accept(visitor)?;
                self.columns.accept(visitor)?;
                self.constraints.accept(visitor)?;
                self.hive_distribution.accept(visitor)?;
                self.hive_formats.accept(visitor)?;
                self.table_properties.accept(visitor)?;
                self.with_options.accept(visitor)?;
                self.file_format.accept(visitor)?;
                self.location.accept(visitor)?;
                self.query.accept(visitor)?;
                self.without_rowid.accept(visitor)?;
                self.like.accept(visitor)?;
                self.clone.accept(visitor)?;
                self.engine.accept(visitor)?;
                self.comment.accept(visitor)?;
                self.auto_increment_offset.accept(visitor)?;
                self.default_charset.accept(visitor)?;
                self.collation.accept(visitor)?;
                self.on_commit.accept(visitor)?;
                self.on_cluster.accept(visitor)?;
                self.primary_key.accept(visitor)?;
                self.order_by.accept(visitor)?;
                self.partition_by.accept(visitor)?;
                self.cluster_by.accept(visitor)?;
                self.clustered_by.accept(visitor)?;
                self.options.accept(visitor)?;
                self.strict.accept(visitor)?;
                self.copy_grants.accept(visitor)?;
                self.enable_schema_evolution.accept(visitor)?;
                self.change_tracking.accept(visitor)?;
                self.data_retention_time_in_days.accept(visitor)?;
                self.max_data_extension_time_in_days.accept(visitor)?;
                self.default_ddl_collation.accept(visitor)?;
                self.with_aggregation_policy.accept(visitor)?;
                self.with_row_access_policy.accept(visitor)?;
                self.with_tags.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreateTable {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CreateTableOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CreateTableOptions::None => {}
                    sqlparser::ast::CreateTableOptions::With(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::CreateTableOptions::Options(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CreateTableOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Cte {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.alias.accept(visitor)?;
                self.query.accept(visitor)?;
                self.from.accept(visitor)?;
                self.materialized.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Cte {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::CteAsMaterialized {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::CteAsMaterialized::Materialized => {}
                    sqlparser::ast::CteAsMaterialized::NotMaterialized => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::CteAsMaterialized {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DataType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DataType::Character(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Char(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::CharacterVarying(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::CharVarying(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Varchar(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Nvarchar(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Uuid => {}
                    sqlparser::ast::DataType::CharacterLargeObject(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::CharLargeObject(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Clob(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Binary(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Varbinary(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Blob(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Bytes(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Numeric(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Decimal(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::BigNumeric(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::BigDecimal(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Dec(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Float(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::TinyInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedTinyInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Int2(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedInt2(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::SmallInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedSmallInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::MediumInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedMediumInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Int(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Int4(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Int8(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Int16 => {}
                    sqlparser::ast::DataType::Int32 => {}
                    sqlparser::ast::DataType::Int64 => {}
                    sqlparser::ast::DataType::Int128 => {}
                    sqlparser::ast::DataType::Int256 => {}
                    sqlparser::ast::DataType::Integer(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedInt4(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedInteger(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UInt8 => {}
                    sqlparser::ast::DataType::UInt16 => {}
                    sqlparser::ast::DataType::UInt32 => {}
                    sqlparser::ast::DataType::UInt64 => {}
                    sqlparser::ast::DataType::UInt128 => {}
                    sqlparser::ast::DataType::UInt256 => {}
                    sqlparser::ast::DataType::BigInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedBigInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::UnsignedInt8(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Float4 => {}
                    sqlparser::ast::DataType::Float32 => {}
                    sqlparser::ast::DataType::Float64 => {}
                    sqlparser::ast::DataType::Real => {}
                    sqlparser::ast::DataType::Float8 => {}
                    sqlparser::ast::DataType::Double => {}
                    sqlparser::ast::DataType::DoublePrecision => {}
                    sqlparser::ast::DataType::Bool => {}
                    sqlparser::ast::DataType::Boolean => {}
                    sqlparser::ast::DataType::Date => {}
                    sqlparser::ast::DataType::Date32 => {}
                    sqlparser::ast::DataType::Time(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Datetime(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Datetime64(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Timestamp(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Interval => {}
                    sqlparser::ast::DataType::JSON => {}
                    sqlparser::ast::DataType::JSONB => {}
                    sqlparser::ast::DataType::Regclass => {}
                    sqlparser::ast::DataType::Text => {}
                    sqlparser::ast::DataType::String(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::FixedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Bytea => {}
                    sqlparser::ast::DataType::Custom(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Array(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Map(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Tuple(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Nested(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Enum(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Set(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Struct(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Union(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Nullable(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::LowCardinality(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DataType::Unspecified => {}
                    sqlparser::ast::DataType::Trigger => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DataType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DateTimeField {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DateTimeField::Year => {}
                    sqlparser::ast::DateTimeField::Month => {}
                    sqlparser::ast::DateTimeField::Week(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DateTimeField::Day => {}
                    sqlparser::ast::DateTimeField::DayOfWeek => {}
                    sqlparser::ast::DateTimeField::DayOfYear => {}
                    sqlparser::ast::DateTimeField::Date => {}
                    sqlparser::ast::DateTimeField::Datetime => {}
                    sqlparser::ast::DateTimeField::Hour => {}
                    sqlparser::ast::DateTimeField::Minute => {}
                    sqlparser::ast::DateTimeField::Second => {}
                    sqlparser::ast::DateTimeField::Century => {}
                    sqlparser::ast::DateTimeField::Decade => {}
                    sqlparser::ast::DateTimeField::Dow => {}
                    sqlparser::ast::DateTimeField::Doy => {}
                    sqlparser::ast::DateTimeField::Epoch => {}
                    sqlparser::ast::DateTimeField::Isodow => {}
                    sqlparser::ast::DateTimeField::IsoWeek => {}
                    sqlparser::ast::DateTimeField::Isoyear => {}
                    sqlparser::ast::DateTimeField::Julian => {}
                    sqlparser::ast::DateTimeField::Microsecond => {}
                    sqlparser::ast::DateTimeField::Microseconds => {}
                    sqlparser::ast::DateTimeField::Millenium => {}
                    sqlparser::ast::DateTimeField::Millennium => {}
                    sqlparser::ast::DateTimeField::Millisecond => {}
                    sqlparser::ast::DateTimeField::Milliseconds => {}
                    sqlparser::ast::DateTimeField::Nanosecond => {}
                    sqlparser::ast::DateTimeField::Nanoseconds => {}
                    sqlparser::ast::DateTimeField::Quarter => {}
                    sqlparser::ast::DateTimeField::Time => {}
                    sqlparser::ast::DateTimeField::Timezone => {}
                    sqlparser::ast::DateTimeField::TimezoneAbbr => {}
                    sqlparser::ast::DateTimeField::TimezoneHour => {}
                    sqlparser::ast::DateTimeField::TimezoneMinute => {}
                    sqlparser::ast::DateTimeField::TimezoneRegion => {}
                    sqlparser::ast::DateTimeField::NoDateTime => {}
                    sqlparser::ast::DateTimeField::Custom(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DateTimeField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Declare {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.names.accept(visitor)?;
                self.data_type.accept(visitor)?;
                self.assignment.accept(visitor)?;
                self.declare_type.accept(visitor)?;
                self.binary.accept(visitor)?;
                self.sensitive.accept(visitor)?;
                self.scroll.accept(visitor)?;
                self.hold.accept(visitor)?;
                self.for_query.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Declare {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DeclareAssignment {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DeclareAssignment::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DeclareAssignment::Default(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DeclareAssignment::DuckAssignment(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DeclareAssignment::For(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::DeclareAssignment::MsSqlAssignment(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DeclareAssignment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DeclareType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DeclareType::Cursor => {}
                    sqlparser::ast::DeclareType::ResultSet => {}
                    sqlparser::ast::DeclareType::Exception => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DeclareType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Deduplicate {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Deduplicate::All => {}
                    sqlparser::ast::Deduplicate::ByExpression(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Deduplicate {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DeferrableInitial {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DeferrableInitial::Immediate => {}
                    sqlparser::ast::DeferrableInitial::Deferred => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DeferrableInitial {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Delete {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.using.accept(visitor)?;
                self.tables.accept(visitor)?;
                self.from.accept(visitor)?;
                self.selection.accept(visitor)?;
                self.returning.accept(visitor)?;
                self.order_by.accept(visitor)?;
                self.limit.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Delete {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DescribeAlias {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DescribeAlias::Describe => {}
                    sqlparser::ast::DescribeAlias::Explain => {}
                    sqlparser::ast::DescribeAlias::Desc => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DescribeAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DictionaryField {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.key.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DictionaryField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DiscardObject {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DiscardObject::ALL => {}
                    sqlparser::ast::DiscardObject::PLANS => {}
                    sqlparser::ast::DiscardObject::SEQUENCES => {}
                    sqlparser::ast::DiscardObject::TEMP => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DiscardObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Distinct {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Distinct::Distinct => {}
                    sqlparser::ast::Distinct::On(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Distinct {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DoUpdate {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.assignments.accept(visitor)?;
                self.selection.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DoUpdate {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DollarQuotedString {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.value.accept(visitor)?;
                self.tag.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DollarQuotedString {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::DuplicateTreatment {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::DuplicateTreatment::Distinct => {}
                    sqlparser::ast::DuplicateTreatment::All => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::DuplicateTreatment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::EmptyMatchesMode {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::EmptyMatchesMode::Show => {}
                    sqlparser::ast::EmptyMatchesMode::Omit => {}
                    sqlparser::ast::EmptyMatchesMode::WithUnmatched => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::EmptyMatchesMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ExactNumberInfo {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ExactNumberInfo::None => {}
                    sqlparser::ast::ExactNumberInfo::Precision(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ExactNumberInfo::PrecisionAndScale(
                        field0,
                        field1,
                    ) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ExactNumberInfo {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ExceptSelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.first_element.accept(visitor)?;
                self.additional_elements.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ExceptSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ExcludeSelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ExcludeSelectItem::Single(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ExcludeSelectItem::Multiple(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ExcludeSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Expr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Expr::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::CompoundIdentifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::JsonAccess { value, path } => {
                        value.accept(visitor)?;
                        path.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::CompositeAccess { expr, key } => {
                        expr.accept(visitor)?;
                        key.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsFalse(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsNotFalse(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsTrue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsNotTrue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsNotNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsUnknown(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsNotUnknown(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsDistinctFrom(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IsNotDistinctFrom(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::InList { expr, list, negated } => {
                        expr.accept(visitor)?;
                        list.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::InSubquery { expr, subquery, negated } => {
                        expr.accept(visitor)?;
                        subquery.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::InUnnest { expr, array_expr, negated } => {
                        expr.accept(visitor)?;
                        array_expr.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Between { expr, negated, low, high } => {
                        expr.accept(visitor)?;
                        negated.accept(visitor)?;
                        low.accept(visitor)?;
                        high.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::BinaryOp { left, op, right } => {
                        left.accept(visitor)?;
                        op.accept(visitor)?;
                        right.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Like {
                        negated,
                        any,
                        expr,
                        pattern,
                        escape_char,
                    } => {
                        negated.accept(visitor)?;
                        any.accept(visitor)?;
                        expr.accept(visitor)?;
                        pattern.accept(visitor)?;
                        escape_char.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::ILike {
                        negated,
                        any,
                        expr,
                        pattern,
                        escape_char,
                    } => {
                        negated.accept(visitor)?;
                        any.accept(visitor)?;
                        expr.accept(visitor)?;
                        pattern.accept(visitor)?;
                        escape_char.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::SimilarTo {
                        negated,
                        expr,
                        pattern,
                        escape_char,
                    } => {
                        negated.accept(visitor)?;
                        expr.accept(visitor)?;
                        pattern.accept(visitor)?;
                        escape_char.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::RLike { negated, expr, pattern, regexp } => {
                        negated.accept(visitor)?;
                        expr.accept(visitor)?;
                        pattern.accept(visitor)?;
                        regexp.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::AnyOp { left, compare_op, right, is_some } => {
                        left.accept(visitor)?;
                        compare_op.accept(visitor)?;
                        right.accept(visitor)?;
                        is_some.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::AllOp { left, compare_op, right } => {
                        left.accept(visitor)?;
                        compare_op.accept(visitor)?;
                        right.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::UnaryOp { op, expr } => {
                        op.accept(visitor)?;
                        expr.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Convert {
                        is_try,
                        expr,
                        data_type,
                        charset,
                        target_before_value,
                        styles,
                    } => {
                        is_try.accept(visitor)?;
                        expr.accept(visitor)?;
                        data_type.accept(visitor)?;
                        charset.accept(visitor)?;
                        target_before_value.accept(visitor)?;
                        styles.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Cast { kind, expr, data_type, format } => {
                        kind.accept(visitor)?;
                        expr.accept(visitor)?;
                        data_type.accept(visitor)?;
                        format.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::AtTimeZone { timestamp, time_zone } => {
                        timestamp.accept(visitor)?;
                        time_zone.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Extract { field, syntax, expr } => {
                        field.accept(visitor)?;
                        syntax.accept(visitor)?;
                        expr.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Ceil { expr, field } => {
                        expr.accept(visitor)?;
                        field.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Floor { expr, field } => {
                        expr.accept(visitor)?;
                        field.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Position { expr, r#in } => {
                        expr.accept(visitor)?;
                        r#in.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Substring {
                        expr,
                        substring_from,
                        substring_for,
                        special,
                    } => {
                        expr.accept(visitor)?;
                        substring_from.accept(visitor)?;
                        substring_for.accept(visitor)?;
                        special.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Trim {
                        expr,
                        trim_where,
                        trim_what,
                        trim_characters,
                    } => {
                        expr.accept(visitor)?;
                        trim_where.accept(visitor)?;
                        trim_what.accept(visitor)?;
                        trim_characters.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Overlay {
                        expr,
                        overlay_what,
                        overlay_from,
                        overlay_for,
                    } => {
                        expr.accept(visitor)?;
                        overlay_what.accept(visitor)?;
                        overlay_from.accept(visitor)?;
                        overlay_for.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Collate { expr, collation } => {
                        expr.accept(visitor)?;
                        collation.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Nested(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Value(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::IntroducedString { introducer, value } => {
                        introducer.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::TypedString { data_type, value } => {
                        data_type.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::MapAccess { column, keys } => {
                        column.accept(visitor)?;
                        keys.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Function(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Case {
                        operand,
                        conditions,
                        results,
                        else_result,
                    } => {
                        operand.accept(visitor)?;
                        conditions.accept(visitor)?;
                        results.accept(visitor)?;
                        else_result.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Exists { subquery, negated } => {
                        subquery.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Subquery(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::GroupingSets(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Cube(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Rollup(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Tuple(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Struct { values, fields } => {
                        values.accept(visitor)?;
                        fields.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Named { expr, name } => {
                        expr.accept(visitor)?;
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Dictionary(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Map(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Subscript { expr, subscript } => {
                        expr.accept(visitor)?;
                        subscript.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Array(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Interval(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::MatchAgainst {
                        columns,
                        match_value,
                        opt_search_modifier,
                    } => {
                        columns.accept(visitor)?;
                        match_value.accept(visitor)?;
                        opt_search_modifier.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Wildcard => {}
                    sqlparser::ast::Expr::QualifiedWildcard(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::OuterJoin(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Prior(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Expr::Lambda(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Expr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ExprWithAlias {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.expr.accept(visitor)?;
                self.alias.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ExprWithAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ExtractSyntax {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ExtractSyntax::From => {}
                    sqlparser::ast::ExtractSyntax::Comma => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ExtractSyntax {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Fetch {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.with_ties.accept(visitor)?;
                self.percent.accept(visitor)?;
                self.quantity.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Fetch {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FetchDirection {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FetchDirection::Count { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqlparser::ast::FetchDirection::Next => {}
                    sqlparser::ast::FetchDirection::Prior => {}
                    sqlparser::ast::FetchDirection::First => {}
                    sqlparser::ast::FetchDirection::Last => {}
                    sqlparser::ast::FetchDirection::Absolute { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqlparser::ast::FetchDirection::Relative { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqlparser::ast::FetchDirection::All => {}
                    sqlparser::ast::FetchDirection::Forward { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqlparser::ast::FetchDirection::ForwardAll => {}
                    sqlparser::ast::FetchDirection::Backward { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqlparser::ast::FetchDirection::BackwardAll => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FetchDirection {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FileFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FileFormat::TEXTFILE => {}
                    sqlparser::ast::FileFormat::SEQUENCEFILE => {}
                    sqlparser::ast::FileFormat::ORC => {}
                    sqlparser::ast::FileFormat::PARQUET => {}
                    sqlparser::ast::FileFormat::AVRO => {}
                    sqlparser::ast::FileFormat::RCFILE => {}
                    sqlparser::ast::FileFormat::JSONFILE => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FileFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FlushLocation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FlushLocation::NoWriteToBinlog => {}
                    sqlparser::ast::FlushLocation::Local => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FlushLocation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FlushType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FlushType::BinaryLogs => {}
                    sqlparser::ast::FlushType::EngineLogs => {}
                    sqlparser::ast::FlushType::ErrorLogs => {}
                    sqlparser::ast::FlushType::GeneralLogs => {}
                    sqlparser::ast::FlushType::Hosts => {}
                    sqlparser::ast::FlushType::Logs => {}
                    sqlparser::ast::FlushType::Privileges => {}
                    sqlparser::ast::FlushType::OptimizerCosts => {}
                    sqlparser::ast::FlushType::RelayLogs => {}
                    sqlparser::ast::FlushType::SlowLogs => {}
                    sqlparser::ast::FlushType::Status => {}
                    sqlparser::ast::FlushType::UserResources => {}
                    sqlparser::ast::FlushType::Tables => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FlushType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ForClause {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ForClause::Browse => {}
                    sqlparser::ast::ForClause::Json {
                        for_json,
                        root,
                        include_null_values,
                        without_array_wrapper,
                    } => {
                        for_json.accept(visitor)?;
                        root.accept(visitor)?;
                        include_null_values.accept(visitor)?;
                        without_array_wrapper.accept(visitor)?;
                    }
                    sqlparser::ast::ForClause::Xml {
                        for_xml,
                        elements,
                        binary_base64,
                        root,
                        r#type,
                    } => {
                        for_xml.accept(visitor)?;
                        elements.accept(visitor)?;
                        binary_base64.accept(visitor)?;
                        root.accept(visitor)?;
                        r#type.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ForClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ForJson {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ForJson::Auto => {}
                    sqlparser::ast::ForJson::Path => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ForJson {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ForXml {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ForXml::Raw(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ForXml::Auto => {}
                    sqlparser::ast::ForXml::Explicit => {}
                    sqlparser::ast::ForXml::Path(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ForXml {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FormatClause {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FormatClause::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FormatClause::Null => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FormatClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FromTable {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FromTable::WithFromKeyword(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FromTable::WithoutKeyword(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FromTable {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Function {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.parameters.accept(visitor)?;
                self.args.accept(visitor)?;
                self.filter.accept(visitor)?;
                self.null_treatment.accept(visitor)?;
                self.over.accept(visitor)?;
                self.within_group.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Function {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionArg {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionArg::Named { name, arg, operator } => {
                        name.accept(visitor)?;
                        arg.accept(visitor)?;
                        operator.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArg::Unnamed(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionArg {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionArgExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionArgExpr::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgExpr::QualifiedWildcard(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgExpr::Wildcard => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionArgExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionArgOperator {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionArgOperator::Equals => {}
                    sqlparser::ast::FunctionArgOperator::RightArrow => {}
                    sqlparser::ast::FunctionArgOperator::Assignment => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionArgOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionArgumentClause {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgumentClause::OrderBy(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgumentClause::Limit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgumentClause::OnOverflow(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgumentClause::Having(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArgumentClause::Separator(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionArgumentClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionArgumentList {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.duplicate_treatment.accept(visitor)?;
                self.args.accept(visitor)?;
                self.clauses.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionArgumentList {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionArguments {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionArguments::None => {}
                    sqlparser::ast::FunctionArguments::Subquery(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::FunctionArguments::List(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionArguments {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionBehavior {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionBehavior::Immutable => {}
                    sqlparser::ast::FunctionBehavior::Stable => {}
                    sqlparser::ast::FunctionBehavior::Volatile => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionBehavior {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionCalledOnNull {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput => {}
                    sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput => {}
                    sqlparser::ast::FunctionCalledOnNull::Strict => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionCalledOnNull {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionDesc {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.args.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionDesc {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionDeterminismSpecifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionDeterminismSpecifier::Deterministic => {}
                    sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionDeterminismSpecifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::FunctionParallel {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::FunctionParallel::Unsafe => {}
                    sqlparser::ast::FunctionParallel::Restricted => {}
                    sqlparser::ast::FunctionParallel::Safe => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::FunctionParallel {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::GeneratedAs {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::GeneratedAs::Always => {}
                    sqlparser::ast::GeneratedAs::ByDefault => {}
                    sqlparser::ast::GeneratedAs::ExpStored => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::GeneratedAs {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::GeneratedExpressionMode {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::GeneratedExpressionMode::Virtual => {}
                    sqlparser::ast::GeneratedExpressionMode::Stored => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::GeneratedExpressionMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::GrantObjects {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::GrantObjects::AllSequencesInSchema { schemas } => {
                        schemas.accept(visitor)?;
                    }
                    sqlparser::ast::GrantObjects::AllTablesInSchema { schemas } => {
                        schemas.accept(visitor)?;
                    }
                    sqlparser::ast::GrantObjects::Schemas(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::GrantObjects::Sequences(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::GrantObjects::Tables(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::GrantObjects {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::GroupByExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::GroupByExpr::All(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::GroupByExpr::Expressions(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::GroupByExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::GroupByWithModifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::GroupByWithModifier::Rollup => {}
                    sqlparser::ast::GroupByWithModifier::Cube => {}
                    sqlparser::ast::GroupByWithModifier::Totals => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::GroupByWithModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HavingBound {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.0.accept(visitor)?;
                self.1.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HavingBound {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HavingBoundKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::HavingBoundKind::Min => {}
                    sqlparser::ast::HavingBoundKind::Max => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HavingBoundKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveDelimiter {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::HiveDelimiter::FieldsTerminatedBy => {}
                    sqlparser::ast::HiveDelimiter::FieldsEscapedBy => {}
                    sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy => {}
                    sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy => {}
                    sqlparser::ast::HiveDelimiter::LinesTerminatedBy => {}
                    sqlparser::ast::HiveDelimiter::NullDefinedAs => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveDelimiter {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveDescribeFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::HiveDescribeFormat::Extended => {}
                    sqlparser::ast::HiveDescribeFormat::Formatted => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveDescribeFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveDistributionStyle {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::HiveDistributionStyle::PARTITIONED { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::HiveDistributionStyle::SKEWED {
                        columns,
                        on,
                        stored_as_directories,
                    } => {
                        columns.accept(visitor)?;
                        on.accept(visitor)?;
                        stored_as_directories.accept(visitor)?;
                    }
                    sqlparser::ast::HiveDistributionStyle::NONE => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveDistributionStyle {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.row_format.accept(visitor)?;
                self.serde_properties.accept(visitor)?;
                self.storage.accept(visitor)?;
                self.location.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveIOFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::HiveIOFormat::IOF {
                        input_format,
                        output_format,
                    } => {
                        input_format.accept(visitor)?;
                        output_format.accept(visitor)?;
                    }
                    sqlparser::ast::HiveIOFormat::FileFormat { format } => {
                        format.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveIOFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveRowDelimiter {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.delimiter.accept(visitor)?;
                self.char.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveRowDelimiter {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveRowFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::HiveRowFormat::SERDE { class } => {
                        class.accept(visitor)?;
                    }
                    sqlparser::ast::HiveRowFormat::DELIMITED { delimiters } => {
                        delimiters.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveRowFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::HiveSetLocation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.has_set.accept(visitor)?;
                self.location.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::HiveSetLocation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Ident {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.value.accept(visitor)?;
                self.quote_style.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Ident {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IdentWithAlias {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.ident.accept(visitor)?;
                self.alias.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IdentWithAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IdentityParameters {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.seed.accept(visitor)?;
                self.increment.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IdentityParameters {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IdentityProperty {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.parameters.accept(visitor)?;
                self.order.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IdentityProperty {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IdentityPropertyFormatKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IdentityPropertyFormatKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IdentityPropertyKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::IdentityPropertyKind::Autoincrement(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::IdentityPropertyKind::Identity(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IdentityPropertyKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IdentityPropertyOrder {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::IdentityPropertyOrder::Order => {}
                    sqlparser::ast::IdentityPropertyOrder::NoOrder => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IdentityPropertyOrder {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IlikeSelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.pattern.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IlikeSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IndexOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::IndexOption::Using(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::IndexOption::Comment(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IndexOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::IndexType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::IndexType::BTree => {}
                    sqlparser::ast::IndexType::Hash => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::IndexType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Insert {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.or.accept(visitor)?;
                self.ignore.accept(visitor)?;
                self.into.accept(visitor)?;
                self.table_name.accept(visitor)?;
                self.table_alias.accept(visitor)?;
                self.columns.accept(visitor)?;
                self.overwrite.accept(visitor)?;
                self.source.accept(visitor)?;
                self.partitioned.accept(visitor)?;
                self.after_columns.accept(visitor)?;
                self.table.accept(visitor)?;
                self.on.accept(visitor)?;
                self.returning.accept(visitor)?;
                self.replace_into.accept(visitor)?;
                self.priority.accept(visitor)?;
                self.insert_alias.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Insert {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::InsertAliases {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.row_alias.accept(visitor)?;
                self.col_aliases.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::InsertAliases {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Interpolate {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.exprs.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Interpolate {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::InterpolateExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.column.accept(visitor)?;
                self.expr.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::InterpolateExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Interval {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.value.accept(visitor)?;
                self.leading_field.accept(visitor)?;
                self.leading_precision.accept(visitor)?;
                self.last_field.accept(visitor)?;
                self.fractional_seconds_precision.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Interval {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Join {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.relation.accept(visitor)?;
                self.global.accept(visitor)?;
                self.join_operator.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Join {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JoinConstraint {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::JoinConstraint::On(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinConstraint::Using(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinConstraint::Natural => {}
                    sqlparser::ast::JoinConstraint::None => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JoinConstraint {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JoinOperator {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::JoinOperator::Inner(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::LeftOuter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::RightOuter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::FullOuter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::CrossJoin => {}
                    sqlparser::ast::JoinOperator::LeftSemi(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::RightSemi(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::LeftAnti(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::RightAnti(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JoinOperator::CrossApply => {}
                    sqlparser::ast::JoinOperator::OuterApply => {}
                    sqlparser::ast::JoinOperator::AsOf {
                        match_condition,
                        constraint,
                    } => {
                        match_condition.accept(visitor)?;
                        constraint.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JoinOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JsonPath {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.path.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JsonPath {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JsonPathElem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::JsonPathElem::Dot { key, quoted } => {
                        key.accept(visitor)?;
                        quoted.accept(visitor)?;
                    }
                    sqlparser::ast::JsonPathElem::Bracket { key } => {
                        key.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JsonPathElem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JsonTableColumn {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::JsonTableColumn::Named(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JsonTableColumn::ForOrdinality(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JsonTableColumn::Nested(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JsonTableColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JsonTableColumnErrorHandling {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::JsonTableColumnErrorHandling::Null => {}
                    sqlparser::ast::JsonTableColumnErrorHandling::Default(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::JsonTableColumnErrorHandling::Error => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JsonTableColumnErrorHandling {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JsonTableNamedColumn {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.r#type.accept(visitor)?;
                self.path.accept(visitor)?;
                self.exists.accept(visitor)?;
                self.on_empty.accept(visitor)?;
                self.on_error.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JsonTableNamedColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::JsonTableNestedColumn {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.path.accept(visitor)?;
                self.columns.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::JsonTableNestedColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::KeyOrIndexDisplay {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::KeyOrIndexDisplay::None => {}
                    sqlparser::ast::KeyOrIndexDisplay::Key => {}
                    sqlparser::ast::KeyOrIndexDisplay::Index => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::KeyOrIndexDisplay {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::KillType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::KillType::Connection => {}
                    sqlparser::ast::KillType::Query => {}
                    sqlparser::ast::KillType::Mutation => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::KillType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::LambdaFunction {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.params.accept(visitor)?;
                self.body.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::LambdaFunction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::LateralView {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.lateral_view.accept(visitor)?;
                self.lateral_view_name.accept(visitor)?;
                self.lateral_col_alias.accept(visitor)?;
                self.outer.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::LateralView {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ListAggOnOverflow {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ListAggOnOverflow::Error => {}
                    sqlparser::ast::ListAggOnOverflow::Truncate {
                        filler,
                        with_count,
                    } => {
                        filler.accept(visitor)?;
                        with_count.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ListAggOnOverflow {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::LockClause {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.lock_type.accept(visitor)?;
                self.of.accept(visitor)?;
                self.nonblock.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::LockClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::LockTable {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.table.accept(visitor)?;
                self.alias.accept(visitor)?;
                self.lock_type.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::LockTable {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::LockTableType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::LockTableType::Read { local } => {
                        local.accept(visitor)?;
                    }
                    sqlparser::ast::LockTableType::Write { low_priority } => {
                        low_priority.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::LockTableType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::LockType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::LockType::Share => {}
                    sqlparser::ast::LockType::Update => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::LockType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MacroArg {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.default_expr.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MacroArg {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MacroDefinition {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MacroDefinition::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MacroDefinition::Table(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MacroDefinition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Map {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.entries.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Map {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MapAccessKey {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.key.accept(visitor)?;
                self.syntax.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MapAccessKey {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MapAccessSyntax {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MapAccessSyntax::Bracket => {}
                    sqlparser::ast::MapAccessSyntax::Period => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MapAccessSyntax {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MapEntry {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.key.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MapEntry {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MatchRecognizePattern {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MatchRecognizePattern::Symbol(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizePattern::Exclude(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizePattern::Permute(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizePattern::Concat(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizePattern::Group(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizePattern::Alternation(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizePattern::Repetition(
                        field0,
                        field1,
                    ) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MatchRecognizePattern {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MatchRecognizeSymbol {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MatchRecognizeSymbol::Named(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MatchRecognizeSymbol::Start => {}
                    sqlparser::ast::MatchRecognizeSymbol::End => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MatchRecognizeSymbol {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Measure {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.expr.accept(visitor)?;
                self.alias.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Measure {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MergeAction {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MergeAction::Insert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MergeAction::Update { assignments } => {
                        assignments.accept(visitor)?;
                    }
                    sqlparser::ast::MergeAction::Delete => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MergeAction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MergeClause {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.clause_kind.accept(visitor)?;
                self.predicate.accept(visitor)?;
                self.action.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MergeClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MergeClauseKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MergeClauseKind::Matched => {}
                    sqlparser::ast::MergeClauseKind::NotMatched => {}
                    sqlparser::ast::MergeClauseKind::NotMatchedByTarget => {}
                    sqlparser::ast::MergeClauseKind::NotMatchedBySource => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MergeClauseKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MergeInsertExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.columns.accept(visitor)?;
                self.kind.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MergeInsertExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MergeInsertKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MergeInsertKind::Values(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::MergeInsertKind::Row => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MergeInsertKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MinMaxValue {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MinMaxValue::Empty => {}
                    sqlparser::ast::MinMaxValue::None => {}
                    sqlparser::ast::MinMaxValue::Some(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MinMaxValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MySQLColumnPosition {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MySQLColumnPosition::First => {}
                    sqlparser::ast::MySQLColumnPosition::After(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MySQLColumnPosition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::MysqlInsertPriority {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::MysqlInsertPriority::LowPriority => {}
                    sqlparser::ast::MysqlInsertPriority::Delayed => {}
                    sqlparser::ast::MysqlInsertPriority::HighPriority => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::MysqlInsertPriority {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::NamedWindowDefinition {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.0.accept(visitor)?;
                self.1.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::NamedWindowDefinition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::NamedWindowExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::NamedWindowExpr::NamedWindow(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::NamedWindowExpr::WindowSpec(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::NamedWindowExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::NonBlock {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::NonBlock::Nowait => {}
                    sqlparser::ast::NonBlock::SkipLocked => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::NonBlock {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::NullTreatment {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::NullTreatment::IgnoreNulls => {}
                    sqlparser::ast::NullTreatment::RespectNulls => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::NullTreatment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ObjectName {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.0.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ObjectName {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ObjectType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ObjectType::Table => {}
                    sqlparser::ast::ObjectType::View => {}
                    sqlparser::ast::ObjectType::Index => {}
                    sqlparser::ast::ObjectType::Schema => {}
                    sqlparser::ast::ObjectType::Database => {}
                    sqlparser::ast::ObjectType::Role => {}
                    sqlparser::ast::ObjectType::Sequence => {}
                    sqlparser::ast::ObjectType::Stage => {}
                    sqlparser::ast::ObjectType::Type => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ObjectType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Offset {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.value.accept(visitor)?;
                self.rows.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Offset {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OffsetRows {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::OffsetRows::None => {}
                    sqlparser::ast::OffsetRows::Row => {}
                    sqlparser::ast::OffsetRows::Rows => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OffsetRows {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OnCommit {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::OnCommit::DeleteRows => {}
                    sqlparser::ast::OnCommit::PreserveRows => {}
                    sqlparser::ast::OnCommit::Drop => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OnCommit {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OnConflict {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.conflict_target.accept(visitor)?;
                self.action.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OnConflict {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OnConflictAction {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::OnConflictAction::DoNothing => {}
                    sqlparser::ast::OnConflictAction::DoUpdate(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OnConflictAction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OnInsert {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::OnInsert::DuplicateKeyUpdate(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::OnInsert::OnConflict(field0) => {
                        field0.accept(visitor)?;
                    }
                    _ => unreachable!(),
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OnInsert {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OperateFunctionArg {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.mode.accept(visitor)?;
                self.name.accept(visitor)?;
                self.data_type.accept(visitor)?;
                self.default_expr.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OperateFunctionArg {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OrderBy {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.exprs.accept(visitor)?;
                self.interpolate.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OrderBy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::OrderByExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.expr.accept(visitor)?;
                self.asc.accept(visitor)?;
                self.nulls_first.accept(visitor)?;
                self.with_fill.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::OrderByExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Owner {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Owner::Ident(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Owner::CurrentRole => {}
                    sqlparser::ast::Owner::CurrentUser => {}
                    sqlparser::ast::Owner::SessionUser => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Owner {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Partition {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Partition::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Partition::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Partition::Part(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Partition::Partitions(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Partition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::PartitionRangeDirection {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::PartitionRangeDirection::Left => {}
                    sqlparser::ast::PartitionRangeDirection::Right => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::PartitionRangeDirection {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Password {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Password::Password(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Password::NullPassword => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Password {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::PivotValueSource {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::PivotValueSource::List(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::PivotValueSource::Any(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::PivotValueSource::Subquery(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::PivotValueSource {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Privileges {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Privileges::All { with_privileges_keyword } => {
                        with_privileges_keyword.accept(visitor)?;
                    }
                    sqlparser::ast::Privileges::Actions(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Privileges {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ProcedureParam {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.data_type.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ProcedureParam {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ProjectionSelect {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.projection.accept(visitor)?;
                self.order_by.accept(visitor)?;
                self.group_by.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ProjectionSelect {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Query {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.with.accept(visitor)?;
                self.body.accept(visitor)?;
                self.order_by.accept(visitor)?;
                self.limit.accept(visitor)?;
                self.limit_by.accept(visitor)?;
                self.offset.accept(visitor)?;
                self.fetch.accept(visitor)?;
                self.locks.accept(visitor)?;
                self.for_clause.accept(visitor)?;
                self.settings.accept(visitor)?;
                self.format_clause.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Query {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ReferentialAction {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ReferentialAction::Restrict => {}
                    sqlparser::ast::ReferentialAction::Cascade => {}
                    sqlparser::ast::ReferentialAction::SetNull => {}
                    sqlparser::ast::ReferentialAction::NoAction => {}
                    sqlparser::ast::ReferentialAction::SetDefault => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ReferentialAction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::RenameSelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::RenameSelectItem::Single(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RenameSelectItem::Multiple(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::RenameSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::RepetitionQuantifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::RepetitionQuantifier::ZeroOrMore => {}
                    sqlparser::ast::RepetitionQuantifier::OneOrMore => {}
                    sqlparser::ast::RepetitionQuantifier::AtMostOne => {}
                    sqlparser::ast::RepetitionQuantifier::Exactly(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RepetitionQuantifier::AtLeast(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RepetitionQuantifier::AtMost(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RepetitionQuantifier::Range(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::RepetitionQuantifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ReplaceSelectElement {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.expr.accept(visitor)?;
                self.column_name.accept(visitor)?;
                self.as_keyword.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ReplaceSelectElement {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ReplaceSelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.items.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ReplaceSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ResetConfig {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ResetConfig::ALL => {}
                    sqlparser::ast::ResetConfig::ConfigName(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ResetConfig {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::RoleOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::RoleOption::BypassRLS(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::ConnectionLimit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::CreateDB(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::CreateRole(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::Inherit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::Login(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::Password(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::Replication(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::SuperUser(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::RoleOption::ValidUntil(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::RoleOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::RowAccessPolicy {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.policy.accept(visitor)?;
                self.on.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::RowAccessPolicy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::RowsPerMatch {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::RowsPerMatch::OneRow => {}
                    sqlparser::ast::RowsPerMatch::AllRows(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::RowsPerMatch {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SchemaName {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SchemaName::Simple(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SchemaName::UnnamedAuthorization(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SchemaName::NamedAuthorization(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SchemaName {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SearchModifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SearchModifier::InNaturalLanguageMode => {}
                    sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion => {}
                    sqlparser::ast::SearchModifier::InBooleanMode => {}
                    sqlparser::ast::SearchModifier::WithQueryExpansion => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SearchModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SecretOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.key.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SecretOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Select {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.from.accept(visitor)?;
                self.distinct.accept(visitor)?;
                self.top.accept(visitor)?;
                self.top_before_distinct.accept(visitor)?;
                self.projection.accept(visitor)?;
                self.into.accept(visitor)?;
                self.lateral_views.accept(visitor)?;
                self.prewhere.accept(visitor)?;
                self.selection.accept(visitor)?;
                self.group_by.accept(visitor)?;
                self.cluster_by.accept(visitor)?;
                self.distribute_by.accept(visitor)?;
                self.sort_by.accept(visitor)?;
                self.having.accept(visitor)?;
                self.named_window.accept(visitor)?;
                self.qualify.accept(visitor)?;
                self.window_before_qualify.accept(visitor)?;
                self.value_table_mode.accept(visitor)?;
                self.connect_by.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Select {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SelectInto {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.temporary.accept(visitor)?;
                self.unlogged.accept(visitor)?;
                self.table.accept(visitor)?;
                self.name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SelectInto {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SelectItem::UnnamedExpr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                        expr.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::SelectItem::QualifiedWildcard(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::SelectItem::Wildcard(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SequenceOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SequenceOptions::IncrementBy(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::SequenceOptions::MinValue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SequenceOptions::MaxValue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SequenceOptions::StartWith(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::SequenceOptions::Cache(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SequenceOptions::Cycle(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SequenceOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SetConfigValue {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SetConfigValue::Default => {}
                    sqlparser::ast::SetConfigValue::FromCurrent => {}
                    sqlparser::ast::SetConfigValue::Value(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SetConfigValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SetExpr {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SetExpr::Select(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SetExpr::Query(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SetExpr::SetOperation {
                        op,
                        set_quantifier,
                        left,
                        right,
                    } => {
                        left.accept(visitor)?;
                        right.accept(visitor)?;
                        op.accept(visitor)?;
                        set_quantifier.accept(visitor)?;
                    }
                    sqlparser::ast::SetExpr::Values(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SetExpr::Insert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SetExpr::Update(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SetExpr::Table(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SetExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SetOperator {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SetOperator::Union => {}
                    sqlparser::ast::SetOperator::Except => {}
                    sqlparser::ast::SetOperator::Intersect => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SetOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SetQuantifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SetQuantifier::All => {}
                    sqlparser::ast::SetQuantifier::Distinct => {}
                    sqlparser::ast::SetQuantifier::ByName => {}
                    sqlparser::ast::SetQuantifier::AllByName => {}
                    sqlparser::ast::SetQuantifier::DistinctByName => {}
                    sqlparser::ast::SetQuantifier::None => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SetQuantifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Setting {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.key.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Setting {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ShowClause {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ShowClause::IN => {}
                    sqlparser::ast::ShowClause::FROM => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ShowClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ShowCreateObject {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ShowCreateObject::Event => {}
                    sqlparser::ast::ShowCreateObject::Function => {}
                    sqlparser::ast::ShowCreateObject::Procedure => {}
                    sqlparser::ast::ShowCreateObject::Table => {}
                    sqlparser::ast::ShowCreateObject::Trigger => {}
                    sqlparser::ast::ShowCreateObject::View => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ShowCreateObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ShowStatementFilter {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ShowStatementFilter::Like(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ShowStatementFilter::ILike(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ShowStatementFilter::Where(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::ShowStatementFilter::NoKeyword(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ShowStatementFilter {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SqlOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SqlOption::Clustered(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SqlOption::Ident(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::SqlOption::KeyValue { key, value } => {
                        key.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqlparser::ast::SqlOption::Partition {
                        column_name,
                        range_direction,
                        for_values,
                    } => {
                        column_name.accept(visitor)?;
                        range_direction.accept(visitor)?;
                        for_values.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SqlOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SqliteOnConflict {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::SqliteOnConflict::Rollback => {}
                    sqlparser::ast::SqliteOnConflict::Abort => {}
                    sqlparser::ast::SqliteOnConflict::Fail => {}
                    sqlparser::ast::SqliteOnConflict::Ignore => {}
                    sqlparser::ast::SqliteOnConflict::Replace => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SqliteOnConflict {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Statement {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Statement::Analyze {
                        table_name,
                        partitions,
                        for_columns,
                        columns,
                        cache_metadata,
                        noscan,
                        compute_statistics,
                    } => {
                        table_name.accept(visitor)?;
                        partitions.accept(visitor)?;
                        for_columns.accept(visitor)?;
                        columns.accept(visitor)?;
                        cache_metadata.accept(visitor)?;
                        noscan.accept(visitor)?;
                        compute_statistics.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Truncate {
                        table_names,
                        partitions,
                        table,
                        only,
                        identity,
                        cascade,
                        on_cluster,
                    } => {
                        table_names.accept(visitor)?;
                        partitions.accept(visitor)?;
                        table.accept(visitor)?;
                        only.accept(visitor)?;
                        identity.accept(visitor)?;
                        cascade.accept(visitor)?;
                        on_cluster.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Msck {
                        table_name,
                        repair,
                        partition_action,
                    } => {
                        table_name.accept(visitor)?;
                        repair.accept(visitor)?;
                        partition_action.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Query(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Insert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Install { extension_name } => {
                        extension_name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Load { extension_name } => {
                        extension_name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Directory {
                        overwrite,
                        local,
                        path,
                        file_format,
                        source,
                    } => {
                        overwrite.accept(visitor)?;
                        local.accept(visitor)?;
                        path.accept(visitor)?;
                        file_format.accept(visitor)?;
                        source.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Call(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Copy {
                        source,
                        to,
                        target,
                        options,
                        legacy_options,
                        values,
                    } => {
                        source.accept(visitor)?;
                        to.accept(visitor)?;
                        target.accept(visitor)?;
                        options.accept(visitor)?;
                        legacy_options.accept(visitor)?;
                        values.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CopyIntoSnowflake {
                        into,
                        from_stage,
                        from_stage_alias,
                        stage_params,
                        from_transformations,
                        files,
                        pattern,
                        file_format,
                        copy_options,
                        validation_mode,
                    } => {
                        into.accept(visitor)?;
                        from_stage.accept(visitor)?;
                        from_stage_alias.accept(visitor)?;
                        stage_params.accept(visitor)?;
                        from_transformations.accept(visitor)?;
                        files.accept(visitor)?;
                        pattern.accept(visitor)?;
                        file_format.accept(visitor)?;
                        copy_options.accept(visitor)?;
                        validation_mode.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Close { cursor } => {
                        cursor.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Update {
                        table,
                        assignments,
                        from,
                        selection,
                        returning,
                    } => {
                        table.accept(visitor)?;
                        from.accept(visitor)?;
                        assignments.accept(visitor)?;
                        selection.accept(visitor)?;
                        returning.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Delete(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateView {
                        or_replace,
                        materialized,
                        name,
                        columns,
                        query,
                        options,
                        cluster_by,
                        comment,
                        with_no_schema_binding,
                        if_not_exists,
                        temporary,
                        to,
                    } => {
                        or_replace.accept(visitor)?;
                        materialized.accept(visitor)?;
                        name.accept(visitor)?;
                        columns.accept(visitor)?;
                        query.accept(visitor)?;
                        options.accept(visitor)?;
                        cluster_by.accept(visitor)?;
                        comment.accept(visitor)?;
                        with_no_schema_binding.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        temporary.accept(visitor)?;
                        to.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateTable(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateVirtualTable {
                        name,
                        if_not_exists,
                        module_name,
                        module_args,
                    } => {
                        name.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        module_name.accept(visitor)?;
                        module_args.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateIndex(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateRole {
                        names,
                        if_not_exists,
                        login,
                        inherit,
                        bypassrls,
                        password,
                        superuser,
                        create_db,
                        create_role,
                        replication,
                        connection_limit,
                        valid_until,
                        in_role,
                        in_group,
                        role,
                        user,
                        admin,
                        authorization_owner,
                    } => {
                        names.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        login.accept(visitor)?;
                        inherit.accept(visitor)?;
                        bypassrls.accept(visitor)?;
                        password.accept(visitor)?;
                        superuser.accept(visitor)?;
                        create_db.accept(visitor)?;
                        create_role.accept(visitor)?;
                        replication.accept(visitor)?;
                        connection_limit.accept(visitor)?;
                        valid_until.accept(visitor)?;
                        in_role.accept(visitor)?;
                        in_group.accept(visitor)?;
                        role.accept(visitor)?;
                        user.accept(visitor)?;
                        admin.accept(visitor)?;
                        authorization_owner.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateSecret {
                        or_replace,
                        temporary,
                        if_not_exists,
                        name,
                        storage_specifier,
                        secret_type,
                        options,
                    } => {
                        or_replace.accept(visitor)?;
                        temporary.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        storage_specifier.accept(visitor)?;
                        secret_type.accept(visitor)?;
                        options.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreatePolicy {
                        name,
                        table_name,
                        policy_type,
                        command,
                        to,
                        using,
                        with_check,
                    } => {
                        name.accept(visitor)?;
                        table_name.accept(visitor)?;
                        policy_type.accept(visitor)?;
                        command.accept(visitor)?;
                        to.accept(visitor)?;
                        using.accept(visitor)?;
                        with_check.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AlterTable {
                        name,
                        if_exists,
                        only,
                        operations,
                        location,
                        on_cluster,
                    } => {
                        name.accept(visitor)?;
                        if_exists.accept(visitor)?;
                        only.accept(visitor)?;
                        operations.accept(visitor)?;
                        location.accept(visitor)?;
                        on_cluster.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AlterIndex { name, operation } => {
                        name.accept(visitor)?;
                        operation.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AlterView {
                        name,
                        columns,
                        query,
                        with_options,
                    } => {
                        name.accept(visitor)?;
                        columns.accept(visitor)?;
                        query.accept(visitor)?;
                        with_options.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AlterRole { name, operation } => {
                        name.accept(visitor)?;
                        operation.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AlterPolicy {
                        name,
                        table_name,
                        operation,
                    } => {
                        name.accept(visitor)?;
                        table_name.accept(visitor)?;
                        operation.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AttachDatabase {
                        schema_name,
                        database_file_name,
                        database,
                    } => {
                        schema_name.accept(visitor)?;
                        database_file_name.accept(visitor)?;
                        database.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::AttachDuckDBDatabase {
                        if_not_exists,
                        database,
                        database_path,
                        database_alias,
                        attach_options,
                    } => {
                        if_not_exists.accept(visitor)?;
                        database.accept(visitor)?;
                        database_path.accept(visitor)?;
                        database_alias.accept(visitor)?;
                        attach_options.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::DetachDuckDBDatabase {
                        if_exists,
                        database,
                        database_alias,
                    } => {
                        if_exists.accept(visitor)?;
                        database.accept(visitor)?;
                        database_alias.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Drop {
                        object_type,
                        if_exists,
                        names,
                        cascade,
                        restrict,
                        purge,
                        temporary,
                    } => {
                        object_type.accept(visitor)?;
                        if_exists.accept(visitor)?;
                        names.accept(visitor)?;
                        cascade.accept(visitor)?;
                        restrict.accept(visitor)?;
                        purge.accept(visitor)?;
                        temporary.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::DropFunction {
                        if_exists,
                        func_desc,
                        option,
                    } => {
                        if_exists.accept(visitor)?;
                        func_desc.accept(visitor)?;
                        option.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::DropProcedure {
                        if_exists,
                        proc_desc,
                        option,
                    } => {
                        if_exists.accept(visitor)?;
                        proc_desc.accept(visitor)?;
                        option.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::DropSecret {
                        if_exists,
                        temporary,
                        name,
                        storage_specifier,
                    } => {
                        if_exists.accept(visitor)?;
                        temporary.accept(visitor)?;
                        name.accept(visitor)?;
                        storage_specifier.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::DropPolicy {
                        if_exists,
                        name,
                        table_name,
                        option,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        table_name.accept(visitor)?;
                        option.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Declare { stmts } => {
                        stmts.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateExtension {
                        name,
                        if_not_exists,
                        cascade,
                        schema,
                        version,
                    } => {
                        name.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        cascade.accept(visitor)?;
                        schema.accept(visitor)?;
                        version.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Fetch { name, direction, into } => {
                        name.accept(visitor)?;
                        direction.accept(visitor)?;
                        into.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Flush {
                        object_type,
                        location,
                        channel,
                        read_lock,
                        export,
                        tables,
                    } => {
                        object_type.accept(visitor)?;
                        location.accept(visitor)?;
                        channel.accept(visitor)?;
                        read_lock.accept(visitor)?;
                        export.accept(visitor)?;
                        tables.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Discard { object_type } => {
                        object_type.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::SetRole {
                        context_modifier,
                        role_name,
                    } => {
                        context_modifier.accept(visitor)?;
                        role_name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::SetVariable {
                        local,
                        hivevar,
                        variables,
                        value,
                    } => {
                        local.accept(visitor)?;
                        hivevar.accept(visitor)?;
                        variables.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::SetTimeZone { local, value } => {
                        local.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::SetNames {
                        charset_name,
                        collation_name,
                    } => {
                        charset_name.accept(visitor)?;
                        collation_name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::SetNamesDefault {} => {}
                    sqlparser::ast::Statement::ShowFunctions { filter } => {
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowVariable { variable } => {
                        variable.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowStatus {
                        filter,
                        global,
                        session,
                    } => {
                        filter.accept(visitor)?;
                        global.accept(visitor)?;
                        session.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowVariables {
                        filter,
                        global,
                        session,
                    } => {
                        filter.accept(visitor)?;
                        global.accept(visitor)?;
                        session.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowCreate { obj_type, obj_name } => {
                        obj_type.accept(visitor)?;
                        obj_name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowColumns {
                        extended,
                        full,
                        table_name,
                        filter,
                    } => {
                        extended.accept(visitor)?;
                        full.accept(visitor)?;
                        table_name.accept(visitor)?;
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowDatabases { filter } => {
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowSchemas { filter } => {
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowTables {
                        extended,
                        full,
                        clause,
                        db_name,
                        filter,
                    } => {
                        extended.accept(visitor)?;
                        full.accept(visitor)?;
                        clause.accept(visitor)?;
                        db_name.accept(visitor)?;
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowViews {
                        materialized,
                        clause,
                        db_name,
                        filter,
                    } => {
                        materialized.accept(visitor)?;
                        clause.accept(visitor)?;
                        db_name.accept(visitor)?;
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ShowCollation { filter } => {
                        filter.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Use(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::StartTransaction {
                        modes,
                        begin,
                        modifier,
                    } => {
                        modes.accept(visitor)?;
                        begin.accept(visitor)?;
                        modifier.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::SetTransaction {
                        modes,
                        snapshot,
                        session,
                    } => {
                        modes.accept(visitor)?;
                        snapshot.accept(visitor)?;
                        session.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Comment {
                        object_type,
                        object_name,
                        comment,
                        if_exists,
                    } => {
                        object_type.accept(visitor)?;
                        object_name.accept(visitor)?;
                        comment.accept(visitor)?;
                        if_exists.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Commit { chain } => {
                        chain.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Rollback { chain, savepoint } => {
                        chain.accept(visitor)?;
                        savepoint.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateSchema {
                        schema_name,
                        if_not_exists,
                    } => {
                        schema_name.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateDatabase {
                        db_name,
                        if_not_exists,
                        location,
                        managed_location,
                    } => {
                        db_name.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        location.accept(visitor)?;
                        managed_location.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateFunction {
                        or_replace,
                        temporary,
                        if_not_exists,
                        name,
                        args,
                        return_type,
                        function_body,
                        behavior,
                        called_on_null,
                        parallel,
                        using,
                        language,
                        determinism_specifier,
                        options,
                        remote_connection,
                    } => {
                        or_replace.accept(visitor)?;
                        temporary.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        args.accept(visitor)?;
                        return_type.accept(visitor)?;
                        function_body.accept(visitor)?;
                        behavior.accept(visitor)?;
                        called_on_null.accept(visitor)?;
                        parallel.accept(visitor)?;
                        using.accept(visitor)?;
                        language.accept(visitor)?;
                        determinism_specifier.accept(visitor)?;
                        options.accept(visitor)?;
                        remote_connection.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateTrigger {
                        or_replace,
                        is_constraint,
                        name,
                        period,
                        events,
                        table_name,
                        referenced_table_name,
                        referencing,
                        trigger_object,
                        include_each,
                        condition,
                        exec_body,
                        characteristics,
                    } => {
                        or_replace.accept(visitor)?;
                        is_constraint.accept(visitor)?;
                        name.accept(visitor)?;
                        period.accept(visitor)?;
                        events.accept(visitor)?;
                        table_name.accept(visitor)?;
                        referenced_table_name.accept(visitor)?;
                        referencing.accept(visitor)?;
                        trigger_object.accept(visitor)?;
                        include_each.accept(visitor)?;
                        condition.accept(visitor)?;
                        exec_body.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::DropTrigger {
                        if_exists,
                        trigger_name,
                        table_name,
                        option,
                    } => {
                        if_exists.accept(visitor)?;
                        trigger_name.accept(visitor)?;
                        table_name.accept(visitor)?;
                        option.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateProcedure {
                        or_alter,
                        name,
                        params,
                        body,
                    } => {
                        body.accept(visitor)?;
                        or_alter.accept(visitor)?;
                        name.accept(visitor)?;
                        params.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateMacro {
                        or_replace,
                        temporary,
                        name,
                        args,
                        definition,
                    } => {
                        or_replace.accept(visitor)?;
                        temporary.accept(visitor)?;
                        name.accept(visitor)?;
                        args.accept(visitor)?;
                        definition.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateStage {
                        or_replace,
                        temporary,
                        if_not_exists,
                        name,
                        stage_params,
                        directory_table_params,
                        file_format,
                        copy_options,
                        comment,
                    } => {
                        or_replace.accept(visitor)?;
                        temporary.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        stage_params.accept(visitor)?;
                        directory_table_params.accept(visitor)?;
                        file_format.accept(visitor)?;
                        copy_options.accept(visitor)?;
                        comment.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Assert { condition, message } => {
                        condition.accept(visitor)?;
                        message.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Grant {
                        privileges,
                        objects,
                        grantees,
                        with_grant_option,
                        granted_by,
                    } => {
                        privileges.accept(visitor)?;
                        objects.accept(visitor)?;
                        grantees.accept(visitor)?;
                        with_grant_option.accept(visitor)?;
                        granted_by.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Revoke {
                        privileges,
                        objects,
                        grantees,
                        granted_by,
                        cascade,
                    } => {
                        privileges.accept(visitor)?;
                        objects.accept(visitor)?;
                        grantees.accept(visitor)?;
                        granted_by.accept(visitor)?;
                        cascade.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Deallocate { name, prepare } => {
                        name.accept(visitor)?;
                        prepare.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Execute {
                        name,
                        parameters,
                        has_parentheses,
                        using,
                    } => {
                        name.accept(visitor)?;
                        parameters.accept(visitor)?;
                        has_parentheses.accept(visitor)?;
                        using.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Prepare {
                        name,
                        data_types,
                        statement,
                    } => {
                        statement.accept(visitor)?;
                        name.accept(visitor)?;
                        data_types.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Kill { modifier, id } => {
                        modifier.accept(visitor)?;
                        id.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ExplainTable {
                        describe_alias,
                        hive_format,
                        has_table_keyword,
                        table_name,
                    } => {
                        describe_alias.accept(visitor)?;
                        hive_format.accept(visitor)?;
                        has_table_keyword.accept(visitor)?;
                        table_name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Explain {
                        describe_alias,
                        analyze,
                        verbose,
                        query_plan,
                        statement,
                        format,
                        options,
                    } => {
                        statement.accept(visitor)?;
                        describe_alias.accept(visitor)?;
                        analyze.accept(visitor)?;
                        verbose.accept(visitor)?;
                        query_plan.accept(visitor)?;
                        format.accept(visitor)?;
                        options.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Savepoint { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::ReleaseSavepoint { name } => {
                        name.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Merge {
                        into,
                        table,
                        source,
                        on,
                        clauses,
                    } => {
                        table.accept(visitor)?;
                        source.accept(visitor)?;
                        into.accept(visitor)?;
                        on.accept(visitor)?;
                        clauses.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Cache {
                        table_flag,
                        table_name,
                        has_as,
                        options,
                        query,
                    } => {
                        table_flag.accept(visitor)?;
                        table_name.accept(visitor)?;
                        has_as.accept(visitor)?;
                        options.accept(visitor)?;
                        query.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::UNCache { table_name, if_exists } => {
                        table_name.accept(visitor)?;
                        if_exists.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateSequence {
                        temporary,
                        if_not_exists,
                        name,
                        data_type,
                        sequence_options,
                        owned_by,
                    } => {
                        temporary.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        data_type.accept(visitor)?;
                        sequence_options.accept(visitor)?;
                        owned_by.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::CreateType { name, representation } => {
                        name.accept(visitor)?;
                        representation.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::Pragma { name, value, is_eq } => {
                        name.accept(visitor)?;
                        value.accept(visitor)?;
                        is_eq.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::LockTables { tables } => {
                        tables.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::UnlockTables => {}
                    sqlparser::ast::Statement::Unload { query, to, with } => {
                        query.accept(visitor)?;
                        to.accept(visitor)?;
                        with.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::OptimizeTable {
                        name,
                        on_cluster,
                        partition,
                        include_final,
                        deduplicate,
                    } => {
                        name.accept(visitor)?;
                        on_cluster.accept(visitor)?;
                        partition.accept(visitor)?;
                        include_final.accept(visitor)?;
                        deduplicate.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::LISTEN { channel } => {
                        channel.accept(visitor)?;
                    }
                    sqlparser::ast::Statement::NOTIFY { channel, payload } => {
                        channel.accept(visitor)?;
                        payload.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Statement {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::StructBracketKind {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::StructBracketKind::Parentheses => {}
                    sqlparser::ast::StructBracketKind::AngleBrackets => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::StructBracketKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::StructField {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.field_name.accept(visitor)?;
                self.field_type.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::StructField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Subscript {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Subscript::Index { index } => {
                        index.accept(visitor)?;
                    }
                    sqlparser::ast::Subscript::Slice {
                        lower_bound,
                        upper_bound,
                        stride,
                    } => {
                        lower_bound.accept(visitor)?;
                        upper_bound.accept(visitor)?;
                        stride.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Subscript {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::SymbolDefinition {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.symbol.accept(visitor)?;
                self.definition.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::SymbolDefinition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Table {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.table_name.accept(visitor)?;
                self.schema_name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Table {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableAlias {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.columns.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableConstraint {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TableConstraint::Unique {
                        name,
                        index_name,
                        index_type_display,
                        index_type,
                        columns,
                        index_options,
                        characteristics,
                    } => {
                        name.accept(visitor)?;
                        index_name.accept(visitor)?;
                        index_type_display.accept(visitor)?;
                        index_type.accept(visitor)?;
                        columns.accept(visitor)?;
                        index_options.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqlparser::ast::TableConstraint::PrimaryKey {
                        name,
                        index_name,
                        index_type,
                        columns,
                        index_options,
                        characteristics,
                    } => {
                        name.accept(visitor)?;
                        index_name.accept(visitor)?;
                        index_type.accept(visitor)?;
                        columns.accept(visitor)?;
                        index_options.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqlparser::ast::TableConstraint::ForeignKey {
                        name,
                        columns,
                        foreign_table,
                        referred_columns,
                        on_delete,
                        on_update,
                        characteristics,
                    } => {
                        name.accept(visitor)?;
                        columns.accept(visitor)?;
                        foreign_table.accept(visitor)?;
                        referred_columns.accept(visitor)?;
                        on_delete.accept(visitor)?;
                        on_update.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqlparser::ast::TableConstraint::Check { name, expr } => {
                        name.accept(visitor)?;
                        expr.accept(visitor)?;
                    }
                    sqlparser::ast::TableConstraint::Index {
                        display_as_key,
                        name,
                        index_type,
                        columns,
                    } => {
                        display_as_key.accept(visitor)?;
                        name.accept(visitor)?;
                        index_type.accept(visitor)?;
                        columns.accept(visitor)?;
                    }
                    sqlparser::ast::TableConstraint::FulltextOrSpatial {
                        fulltext,
                        index_type_display,
                        opt_index_name,
                        columns,
                    } => {
                        fulltext.accept(visitor)?;
                        index_type_display.accept(visitor)?;
                        opt_index_name.accept(visitor)?;
                        columns.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableConstraint {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableEngine {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.parameters.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableEngine {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableFactor {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TableFactor::Table {
                        name,
                        alias,
                        args,
                        with_hints,
                        version,
                        with_ordinality,
                        partitions,
                    } => {
                        name.accept(visitor)?;
                        alias.accept(visitor)?;
                        args.accept(visitor)?;
                        with_hints.accept(visitor)?;
                        version.accept(visitor)?;
                        with_ordinality.accept(visitor)?;
                        partitions.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::Derived {
                        lateral,
                        subquery,
                        alias,
                    } => {
                        lateral.accept(visitor)?;
                        subquery.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::TableFunction { expr, alias } => {
                        expr.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::Function {
                        lateral,
                        name,
                        args,
                        alias,
                    } => {
                        lateral.accept(visitor)?;
                        name.accept(visitor)?;
                        args.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::UNNEST {
                        alias,
                        array_exprs,
                        with_offset,
                        with_offset_alias,
                        with_ordinality,
                    } => {
                        alias.accept(visitor)?;
                        array_exprs.accept(visitor)?;
                        with_offset.accept(visitor)?;
                        with_offset_alias.accept(visitor)?;
                        with_ordinality.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::JsonTable {
                        json_expr,
                        json_path,
                        columns,
                        alias,
                    } => {
                        json_expr.accept(visitor)?;
                        json_path.accept(visitor)?;
                        columns.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::NestedJoin {
                        table_with_joins,
                        alias,
                    } => {
                        table_with_joins.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::Pivot {
                        table,
                        aggregate_functions,
                        value_column,
                        value_source,
                        default_on_null,
                        alias,
                    } => {
                        table.accept(visitor)?;
                        aggregate_functions.accept(visitor)?;
                        value_column.accept(visitor)?;
                        value_source.accept(visitor)?;
                        default_on_null.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::Unpivot {
                        table,
                        value,
                        name,
                        columns,
                        alias,
                    } => {
                        table.accept(visitor)?;
                        value.accept(visitor)?;
                        name.accept(visitor)?;
                        columns.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqlparser::ast::TableFactor::MatchRecognize {
                        table,
                        partition_by,
                        order_by,
                        measures,
                        rows_per_match,
                        after_match_skip,
                        pattern,
                        symbols,
                        alias,
                    } => {
                        table.accept(visitor)?;
                        partition_by.accept(visitor)?;
                        order_by.accept(visitor)?;
                        measures.accept(visitor)?;
                        rows_per_match.accept(visitor)?;
                        after_match_skip.accept(visitor)?;
                        pattern.accept(visitor)?;
                        symbols.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableFactor {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableFunctionArgs {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.args.accept(visitor)?;
                self.settings.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableFunctionArgs {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableOptionsClustered {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TableOptionsClustered::ColumnstoreIndex => {}
                    sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::TableOptionsClustered::Index(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableOptionsClustered {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableVersion {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TableVersion::ForSystemTimeAsOf(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableVersion {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TableWithJoins {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.relation.accept(visitor)?;
                self.joins.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TableWithJoins {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Tag {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.key.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Tag {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TagsColumnOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.with.accept(visitor)?;
                self.tags.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TagsColumnOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TimezoneInfo {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TimezoneInfo::None => {}
                    sqlparser::ast::TimezoneInfo::WithTimeZone => {}
                    sqlparser::ast::TimezoneInfo::WithoutTimeZone => {}
                    sqlparser::ast::TimezoneInfo::Tz => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TimezoneInfo {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Top {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.with_ties.accept(visitor)?;
                self.percent.accept(visitor)?;
                self.quantity.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Top {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TopQuantity {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TopQuantity::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::TopQuantity::Constant(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TopQuantity {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TransactionAccessMode {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TransactionAccessMode::ReadOnly => {}
                    sqlparser::ast::TransactionAccessMode::ReadWrite => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TransactionAccessMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TransactionIsolationLevel {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TransactionIsolationLevel::ReadUncommitted => {}
                    sqlparser::ast::TransactionIsolationLevel::ReadCommitted => {}
                    sqlparser::ast::TransactionIsolationLevel::RepeatableRead => {}
                    sqlparser::ast::TransactionIsolationLevel::Serializable => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TransactionIsolationLevel {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TransactionMode {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TransactionMode::AccessMode(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::TransactionMode::IsolationLevel(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TransactionMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TransactionModifier {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TransactionModifier::Deferred => {}
                    sqlparser::ast::TransactionModifier::Immediate => {}
                    sqlparser::ast::TransactionModifier::Exclusive => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TransactionModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerEvent {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TriggerEvent::Insert => {}
                    sqlparser::ast::TriggerEvent::Update(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::TriggerEvent::Delete => {}
                    sqlparser::ast::TriggerEvent::Truncate => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerEvent {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerExecBody {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.exec_type.accept(visitor)?;
                self.func_desc.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerExecBody {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerExecBodyType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TriggerExecBodyType::Function => {}
                    sqlparser::ast::TriggerExecBodyType::Procedure => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerExecBodyType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerObject {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TriggerObject::Row => {}
                    sqlparser::ast::TriggerObject::Statement => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerPeriod {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TriggerPeriod::After => {}
                    sqlparser::ast::TriggerPeriod::Before => {}
                    sqlparser::ast::TriggerPeriod::InsteadOf => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerPeriod {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerReferencing {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.refer_type.accept(visitor)?;
                self.is_as.accept(visitor)?;
                self.transition_relation_name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerReferencing {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TriggerReferencingType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TriggerReferencingType::OldTable => {}
                    sqlparser::ast::TriggerReferencingType::NewTable => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TriggerReferencingType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TrimWhereField {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TrimWhereField::Both => {}
                    sqlparser::ast::TrimWhereField::Leading => {}
                    sqlparser::ast::TrimWhereField::Trailing => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TrimWhereField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TruncateCascadeOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TruncateCascadeOption::Cascade => {}
                    sqlparser::ast::TruncateCascadeOption::Restrict => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TruncateCascadeOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TruncateIdentityOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::TruncateIdentityOption::Restart => {}
                    sqlparser::ast::TruncateIdentityOption::Continue => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TruncateIdentityOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::TruncateTableTarget {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::TruncateTableTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::UnaryOperator {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::UnaryOperator::Plus => {}
                    sqlparser::ast::UnaryOperator::Minus => {}
                    sqlparser::ast::UnaryOperator::Not => {}
                    sqlparser::ast::UnaryOperator::PGBitwiseNot => {}
                    sqlparser::ast::UnaryOperator::PGSquareRoot => {}
                    sqlparser::ast::UnaryOperator::PGCubeRoot => {}
                    sqlparser::ast::UnaryOperator::PGPostfixFactorial => {}
                    sqlparser::ast::UnaryOperator::PGPrefixFactorial => {}
                    sqlparser::ast::UnaryOperator::PGAbs => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::UnaryOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::UnionField {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.field_name.accept(visitor)?;
                self.field_type.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::UnionField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Use {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Use::Catalog(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Use::Schema(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Use::Database(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Use::Warehouse(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Use::Object(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Use::Default => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Use {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::UserDefinedTypeCompositeAttributeDef {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.data_type.accept(visitor)?;
                self.collation.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::UserDefinedTypeCompositeAttributeDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::UserDefinedTypeRepresentation {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::UserDefinedTypeRepresentation::Composite {
                        attributes,
                    } => {
                        attributes.accept(visitor)?;
                    }
                    sqlparser::ast::UserDefinedTypeRepresentation::Enum { labels } => {
                        labels.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::UserDefinedTypeRepresentation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::UtilityOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.arg.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::UtilityOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Value {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::Value::Number(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::ast::Value::SingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::DollarQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::TripleSingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::TripleDoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::EscapedStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::UnicodeStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::SingleQuotedByteStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::DoubleQuotedByteStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::SingleQuotedRawStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::DoubleQuotedRawStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::NationalStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::HexStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::DoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::Boolean(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::Value::Null => {}
                    sqlparser::ast::Value::Placeholder(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Value {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ValueTableMode {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::ValueTableMode::AsStruct => {}
                    sqlparser::ast::ValueTableMode::AsValue => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ValueTableMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::Values {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.explicit_row.accept(visitor)?;
                self.rows.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::Values {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::ViewColumnDef {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.name.accept(visitor)?;
                self.data_type.accept(visitor)?;
                self.options.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::ViewColumnDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WildcardAdditionalOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.opt_ilike.accept(visitor)?;
                self.opt_exclude.accept(visitor)?;
                self.opt_except.accept(visitor)?;
                self.opt_replace.accept(visitor)?;
                self.opt_rename.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WildcardAdditionalOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WindowFrame {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.units.accept(visitor)?;
                self.start_bound.accept(visitor)?;
                self.end_bound.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WindowFrame {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WindowFrameBound {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::WindowFrameBound::CurrentRow => {}
                    sqlparser::ast::WindowFrameBound::Preceding(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::WindowFrameBound::Following(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WindowFrameBound {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WindowFrameUnits {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::WindowFrameUnits::Rows => {}
                    sqlparser::ast::WindowFrameUnits::Range => {}
                    sqlparser::ast::WindowFrameUnits::Groups => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WindowFrameUnits {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WindowSpec {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.window_name.accept(visitor)?;
                self.partition_by.accept(visitor)?;
                self.order_by.accept(visitor)?;
                self.window_frame.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WindowSpec {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WindowType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::WindowType::WindowSpec(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::ast::WindowType::NamedWindow(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WindowType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::With {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.recursive.accept(visitor)?;
                self.cte_tables.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::With {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::WithFill {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.from.accept(visitor)?;
                self.to.accept(visitor)?;
                self.step.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::WithFill {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOption {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.option_name.accept(visitor)?;
                self.option_type.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING => {}
                    sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN => {}
                    sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.options.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.alias.accept(visitor)?;
                self.file_col_num.accept(visitor)?;
                self.element.accept(visitor)?;
                self.item_as.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey
for sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::ast::helpers::stmt_data_loading::StageParamsObject {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.url.accept(visitor)?;
                self.encryption.accept(visitor)?;
                self.endpoint.accept(visitor)?;
                self.storage_integration.accept(visitor)?;
                self.credentials.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::ast::helpers::stmt_data_loading::StageParamsObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::keywords::Keyword {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::keywords::Keyword::NoKeyword => {}
                    sqlparser::keywords::Keyword::ABORT => {}
                    sqlparser::keywords::Keyword::ABS => {}
                    sqlparser::keywords::Keyword::ABSOLUTE => {}
                    sqlparser::keywords::Keyword::ACCESS => {}
                    sqlparser::keywords::Keyword::ACTION => {}
                    sqlparser::keywords::Keyword::ADD => {}
                    sqlparser::keywords::Keyword::ADMIN => {}
                    sqlparser::keywords::Keyword::AFTER => {}
                    sqlparser::keywords::Keyword::AGAINST => {}
                    sqlparser::keywords::Keyword::AGGREGATION => {}
                    sqlparser::keywords::Keyword::ALIAS => {}
                    sqlparser::keywords::Keyword::ALL => {}
                    sqlparser::keywords::Keyword::ALLOCATE => {}
                    sqlparser::keywords::Keyword::ALTER => {}
                    sqlparser::keywords::Keyword::ALWAYS => {}
                    sqlparser::keywords::Keyword::ANALYZE => {}
                    sqlparser::keywords::Keyword::AND => {}
                    sqlparser::keywords::Keyword::ANTI => {}
                    sqlparser::keywords::Keyword::ANY => {}
                    sqlparser::keywords::Keyword::APPLY => {}
                    sqlparser::keywords::Keyword::ARCHIVE => {}
                    sqlparser::keywords::Keyword::ARE => {}
                    sqlparser::keywords::Keyword::ARRAY => {}
                    sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY => {}
                    sqlparser::keywords::Keyword::AS => {}
                    sqlparser::keywords::Keyword::ASC => {}
                    sqlparser::keywords::Keyword::ASENSITIVE => {}
                    sqlparser::keywords::Keyword::ASOF => {}
                    sqlparser::keywords::Keyword::ASSERT => {}
                    sqlparser::keywords::Keyword::ASYMMETRIC => {}
                    sqlparser::keywords::Keyword::AT => {}
                    sqlparser::keywords::Keyword::ATOMIC => {}
                    sqlparser::keywords::Keyword::ATTACH => {}
                    sqlparser::keywords::Keyword::AUTHORIZATION => {}
                    sqlparser::keywords::Keyword::AUTO => {}
                    sqlparser::keywords::Keyword::AUTOINCREMENT => {}
                    sqlparser::keywords::Keyword::AUTO_INCREMENT => {}
                    sqlparser::keywords::Keyword::AVG => {}
                    sqlparser::keywords::Keyword::AVRO => {}
                    sqlparser::keywords::Keyword::BACKWARD => {}
                    sqlparser::keywords::Keyword::BASE64 => {}
                    sqlparser::keywords::Keyword::BEFORE => {}
                    sqlparser::keywords::Keyword::BEGIN => {}
                    sqlparser::keywords::Keyword::BEGIN_FRAME => {}
                    sqlparser::keywords::Keyword::BEGIN_PARTITION => {}
                    sqlparser::keywords::Keyword::BETWEEN => {}
                    sqlparser::keywords::Keyword::BIGDECIMAL => {}
                    sqlparser::keywords::Keyword::BIGINT => {}
                    sqlparser::keywords::Keyword::BIGNUMERIC => {}
                    sqlparser::keywords::Keyword::BINARY => {}
                    sqlparser::keywords::Keyword::BINDING => {}
                    sqlparser::keywords::Keyword::BLOB => {}
                    sqlparser::keywords::Keyword::BLOOMFILTER => {}
                    sqlparser::keywords::Keyword::BOOL => {}
                    sqlparser::keywords::Keyword::BOOLEAN => {}
                    sqlparser::keywords::Keyword::BOTH => {}
                    sqlparser::keywords::Keyword::BROWSE => {}
                    sqlparser::keywords::Keyword::BTREE => {}
                    sqlparser::keywords::Keyword::BUCKETS => {}
                    sqlparser::keywords::Keyword::BY => {}
                    sqlparser::keywords::Keyword::BYPASSRLS => {}
                    sqlparser::keywords::Keyword::BYTEA => {}
                    sqlparser::keywords::Keyword::BYTES => {}
                    sqlparser::keywords::Keyword::CACHE => {}
                    sqlparser::keywords::Keyword::CALL => {}
                    sqlparser::keywords::Keyword::CALLED => {}
                    sqlparser::keywords::Keyword::CARDINALITY => {}
                    sqlparser::keywords::Keyword::CASCADE => {}
                    sqlparser::keywords::Keyword::CASCADED => {}
                    sqlparser::keywords::Keyword::CASE => {}
                    sqlparser::keywords::Keyword::CAST => {}
                    sqlparser::keywords::Keyword::CATALOG => {}
                    sqlparser::keywords::Keyword::CEIL => {}
                    sqlparser::keywords::Keyword::CEILING => {}
                    sqlparser::keywords::Keyword::CENTURY => {}
                    sqlparser::keywords::Keyword::CHAIN => {}
                    sqlparser::keywords::Keyword::CHANGE => {}
                    sqlparser::keywords::Keyword::CHANGE_TRACKING => {}
                    sqlparser::keywords::Keyword::CHANNEL => {}
                    sqlparser::keywords::Keyword::CHAR => {}
                    sqlparser::keywords::Keyword::CHARACTER => {}
                    sqlparser::keywords::Keyword::CHARACTERS => {}
                    sqlparser::keywords::Keyword::CHARACTER_LENGTH => {}
                    sqlparser::keywords::Keyword::CHARSET => {}
                    sqlparser::keywords::Keyword::CHAR_LENGTH => {}
                    sqlparser::keywords::Keyword::CHECK => {}
                    sqlparser::keywords::Keyword::CLEAR => {}
                    sqlparser::keywords::Keyword::CLOB => {}
                    sqlparser::keywords::Keyword::CLONE => {}
                    sqlparser::keywords::Keyword::CLOSE => {}
                    sqlparser::keywords::Keyword::CLUSTER => {}
                    sqlparser::keywords::Keyword::CLUSTERED => {}
                    sqlparser::keywords::Keyword::COALESCE => {}
                    sqlparser::keywords::Keyword::COLLATE => {}
                    sqlparser::keywords::Keyword::COLLATION => {}
                    sqlparser::keywords::Keyword::COLLECT => {}
                    sqlparser::keywords::Keyword::COLLECTION => {}
                    sqlparser::keywords::Keyword::COLUMN => {}
                    sqlparser::keywords::Keyword::COLUMNS => {}
                    sqlparser::keywords::Keyword::COLUMNSTORE => {}
                    sqlparser::keywords::Keyword::COMMENT => {}
                    sqlparser::keywords::Keyword::COMMIT => {}
                    sqlparser::keywords::Keyword::COMMITTED => {}
                    sqlparser::keywords::Keyword::COMPRESSION => {}
                    sqlparser::keywords::Keyword::COMPUTE => {}
                    sqlparser::keywords::Keyword::CONCURRENTLY => {}
                    sqlparser::keywords::Keyword::CONDITION => {}
                    sqlparser::keywords::Keyword::CONFLICT => {}
                    sqlparser::keywords::Keyword::CONNECT => {}
                    sqlparser::keywords::Keyword::CONNECTION => {}
                    sqlparser::keywords::Keyword::CONSTRAINT => {}
                    sqlparser::keywords::Keyword::CONTAINS => {}
                    sqlparser::keywords::Keyword::CONTINUE => {}
                    sqlparser::keywords::Keyword::CONVERT => {}
                    sqlparser::keywords::Keyword::COPY => {}
                    sqlparser::keywords::Keyword::COPY_OPTIONS => {}
                    sqlparser::keywords::Keyword::CORR => {}
                    sqlparser::keywords::Keyword::CORRESPONDING => {}
                    sqlparser::keywords::Keyword::COUNT => {}
                    sqlparser::keywords::Keyword::COVAR_POP => {}
                    sqlparser::keywords::Keyword::COVAR_SAMP => {}
                    sqlparser::keywords::Keyword::CREATE => {}
                    sqlparser::keywords::Keyword::CREATEDB => {}
                    sqlparser::keywords::Keyword::CREATEROLE => {}
                    sqlparser::keywords::Keyword::CREDENTIALS => {}
                    sqlparser::keywords::Keyword::CROSS => {}
                    sqlparser::keywords::Keyword::CSV => {}
                    sqlparser::keywords::Keyword::CUBE => {}
                    sqlparser::keywords::Keyword::CUME_DIST => {}
                    sqlparser::keywords::Keyword::CURRENT => {}
                    sqlparser::keywords::Keyword::CURRENT_CATALOG => {}
                    sqlparser::keywords::Keyword::CURRENT_DATE => {}
                    sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP => {}
                    sqlparser::keywords::Keyword::CURRENT_PATH => {}
                    sqlparser::keywords::Keyword::CURRENT_ROLE => {}
                    sqlparser::keywords::Keyword::CURRENT_ROW => {}
                    sqlparser::keywords::Keyword::CURRENT_SCHEMA => {}
                    sqlparser::keywords::Keyword::CURRENT_TIME => {}
                    sqlparser::keywords::Keyword::CURRENT_TIMESTAMP => {}
                    sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE => {}
                    sqlparser::keywords::Keyword::CURRENT_USER => {}
                    sqlparser::keywords::Keyword::CURSOR => {}
                    sqlparser::keywords::Keyword::CYCLE => {}
                    sqlparser::keywords::Keyword::DATA => {}
                    sqlparser::keywords::Keyword::DATABASE => {}
                    sqlparser::keywords::Keyword::DATABASES => {}
                    sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS => {}
                    sqlparser::keywords::Keyword::DATE => {}
                    sqlparser::keywords::Keyword::DATE32 => {}
                    sqlparser::keywords::Keyword::DATETIME => {}
                    sqlparser::keywords::Keyword::DATETIME64 => {}
                    sqlparser::keywords::Keyword::DAY => {}
                    sqlparser::keywords::Keyword::DAYOFWEEK => {}
                    sqlparser::keywords::Keyword::DAYOFYEAR => {}
                    sqlparser::keywords::Keyword::DEALLOCATE => {}
                    sqlparser::keywords::Keyword::DEC => {}
                    sqlparser::keywords::Keyword::DECADE => {}
                    sqlparser::keywords::Keyword::DECIMAL => {}
                    sqlparser::keywords::Keyword::DECLARE => {}
                    sqlparser::keywords::Keyword::DEDUPLICATE => {}
                    sqlparser::keywords::Keyword::DEFAULT => {}
                    sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION => {}
                    sqlparser::keywords::Keyword::DEFERRABLE => {}
                    sqlparser::keywords::Keyword::DEFERRED => {}
                    sqlparser::keywords::Keyword::DEFINE => {}
                    sqlparser::keywords::Keyword::DEFINED => {}
                    sqlparser::keywords::Keyword::DELAYED => {}
                    sqlparser::keywords::Keyword::DELETE => {}
                    sqlparser::keywords::Keyword::DELIMITED => {}
                    sqlparser::keywords::Keyword::DELIMITER => {}
                    sqlparser::keywords::Keyword::DELTA => {}
                    sqlparser::keywords::Keyword::DENSE_RANK => {}
                    sqlparser::keywords::Keyword::DEREF => {}
                    sqlparser::keywords::Keyword::DESC => {}
                    sqlparser::keywords::Keyword::DESCRIBE => {}
                    sqlparser::keywords::Keyword::DETACH => {}
                    sqlparser::keywords::Keyword::DETAIL => {}
                    sqlparser::keywords::Keyword::DETERMINISTIC => {}
                    sqlparser::keywords::Keyword::DIRECTORY => {}
                    sqlparser::keywords::Keyword::DISABLE => {}
                    sqlparser::keywords::Keyword::DISCARD => {}
                    sqlparser::keywords::Keyword::DISCONNECT => {}
                    sqlparser::keywords::Keyword::DISTINCT => {}
                    sqlparser::keywords::Keyword::DISTRIBUTE => {}
                    sqlparser::keywords::Keyword::DIV => {}
                    sqlparser::keywords::Keyword::DO => {}
                    sqlparser::keywords::Keyword::DOUBLE => {}
                    sqlparser::keywords::Keyword::DOW => {}
                    sqlparser::keywords::Keyword::DOY => {}
                    sqlparser::keywords::Keyword::DROP => {}
                    sqlparser::keywords::Keyword::DRY => {}
                    sqlparser::keywords::Keyword::DUPLICATE => {}
                    sqlparser::keywords::Keyword::DYNAMIC => {}
                    sqlparser::keywords::Keyword::EACH => {}
                    sqlparser::keywords::Keyword::ELEMENT => {}
                    sqlparser::keywords::Keyword::ELEMENTS => {}
                    sqlparser::keywords::Keyword::ELSE => {}
                    sqlparser::keywords::Keyword::EMPTY => {}
                    sqlparser::keywords::Keyword::ENABLE => {}
                    sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION => {}
                    sqlparser::keywords::Keyword::ENCODING => {}
                    sqlparser::keywords::Keyword::ENCRYPTION => {}
                    sqlparser::keywords::Keyword::END => {}
                    sqlparser::keywords::Keyword::END_EXEC => {}
                    sqlparser::keywords::Keyword::ENDPOINT => {}
                    sqlparser::keywords::Keyword::END_FRAME => {}
                    sqlparser::keywords::Keyword::END_PARTITION => {}
                    sqlparser::keywords::Keyword::ENFORCED => {}
                    sqlparser::keywords::Keyword::ENGINE => {}
                    sqlparser::keywords::Keyword::ENUM => {}
                    sqlparser::keywords::Keyword::EPHEMERAL => {}
                    sqlparser::keywords::Keyword::EPOCH => {}
                    sqlparser::keywords::Keyword::EQUALS => {}
                    sqlparser::keywords::Keyword::ERROR => {}
                    sqlparser::keywords::Keyword::ESCAPE => {}
                    sqlparser::keywords::Keyword::ESCAPED => {}
                    sqlparser::keywords::Keyword::EVENT => {}
                    sqlparser::keywords::Keyword::EVERY => {}
                    sqlparser::keywords::Keyword::EXCEPT => {}
                    sqlparser::keywords::Keyword::EXCEPTION => {}
                    sqlparser::keywords::Keyword::EXCLUDE => {}
                    sqlparser::keywords::Keyword::EXCLUSIVE => {}
                    sqlparser::keywords::Keyword::EXEC => {}
                    sqlparser::keywords::Keyword::EXECUTE => {}
                    sqlparser::keywords::Keyword::EXISTS => {}
                    sqlparser::keywords::Keyword::EXP => {}
                    sqlparser::keywords::Keyword::EXPANSION => {}
                    sqlparser::keywords::Keyword::EXPLAIN => {}
                    sqlparser::keywords::Keyword::EXPLICIT => {}
                    sqlparser::keywords::Keyword::EXPORT => {}
                    sqlparser::keywords::Keyword::EXTENDED => {}
                    sqlparser::keywords::Keyword::EXTENSION => {}
                    sqlparser::keywords::Keyword::EXTERNAL => {}
                    sqlparser::keywords::Keyword::EXTRACT => {}
                    sqlparser::keywords::Keyword::FAIL => {}
                    sqlparser::keywords::Keyword::FALSE => {}
                    sqlparser::keywords::Keyword::FETCH => {}
                    sqlparser::keywords::Keyword::FIELDS => {}
                    sqlparser::keywords::Keyword::FILE => {}
                    sqlparser::keywords::Keyword::FILES => {}
                    sqlparser::keywords::Keyword::FILE_FORMAT => {}
                    sqlparser::keywords::Keyword::FILL => {}
                    sqlparser::keywords::Keyword::FILTER => {}
                    sqlparser::keywords::Keyword::FINAL => {}
                    sqlparser::keywords::Keyword::FIRST => {}
                    sqlparser::keywords::Keyword::FIRST_VALUE => {}
                    sqlparser::keywords::Keyword::FIXEDSTRING => {}
                    sqlparser::keywords::Keyword::FLOAT => {}
                    sqlparser::keywords::Keyword::FLOAT32 => {}
                    sqlparser::keywords::Keyword::FLOAT4 => {}
                    sqlparser::keywords::Keyword::FLOAT64 => {}
                    sqlparser::keywords::Keyword::FLOAT8 => {}
                    sqlparser::keywords::Keyword::FLOOR => {}
                    sqlparser::keywords::Keyword::FLUSH => {}
                    sqlparser::keywords::Keyword::FOLLOWING => {}
                    sqlparser::keywords::Keyword::FOR => {}
                    sqlparser::keywords::Keyword::FORCE => {}
                    sqlparser::keywords::Keyword::FORCE_NOT_NULL => {}
                    sqlparser::keywords::Keyword::FORCE_NULL => {}
                    sqlparser::keywords::Keyword::FORCE_QUOTE => {}
                    sqlparser::keywords::Keyword::FOREIGN => {}
                    sqlparser::keywords::Keyword::FORMAT => {}
                    sqlparser::keywords::Keyword::FORMATTED => {}
                    sqlparser::keywords::Keyword::FORWARD => {}
                    sqlparser::keywords::Keyword::FRAME_ROW => {}
                    sqlparser::keywords::Keyword::FREE => {}
                    sqlparser::keywords::Keyword::FREEZE => {}
                    sqlparser::keywords::Keyword::FROM => {}
                    sqlparser::keywords::Keyword::FSCK => {}
                    sqlparser::keywords::Keyword::FULL => {}
                    sqlparser::keywords::Keyword::FULLTEXT => {}
                    sqlparser::keywords::Keyword::FUNCTION => {}
                    sqlparser::keywords::Keyword::FUNCTIONS => {}
                    sqlparser::keywords::Keyword::FUSION => {}
                    sqlparser::keywords::Keyword::GENERAL => {}
                    sqlparser::keywords::Keyword::GENERATE => {}
                    sqlparser::keywords::Keyword::GENERATED => {}
                    sqlparser::keywords::Keyword::GEOGRAPHY => {}
                    sqlparser::keywords::Keyword::GET => {}
                    sqlparser::keywords::Keyword::GLOBAL => {}
                    sqlparser::keywords::Keyword::GRANT => {}
                    sqlparser::keywords::Keyword::GRANTED => {}
                    sqlparser::keywords::Keyword::GRANTS => {}
                    sqlparser::keywords::Keyword::GRAPHVIZ => {}
                    sqlparser::keywords::Keyword::GROUP => {}
                    sqlparser::keywords::Keyword::GROUPING => {}
                    sqlparser::keywords::Keyword::GROUPS => {}
                    sqlparser::keywords::Keyword::HASH => {}
                    sqlparser::keywords::Keyword::HAVING => {}
                    sqlparser::keywords::Keyword::HEADER => {}
                    sqlparser::keywords::Keyword::HEAP => {}
                    sqlparser::keywords::Keyword::HIGH_PRIORITY => {}
                    sqlparser::keywords::Keyword::HISTORY => {}
                    sqlparser::keywords::Keyword::HIVEVAR => {}
                    sqlparser::keywords::Keyword::HOLD => {}
                    sqlparser::keywords::Keyword::HOSTS => {}
                    sqlparser::keywords::Keyword::HOUR => {}
                    sqlparser::keywords::Keyword::HOURS => {}
                    sqlparser::keywords::Keyword::ID => {}
                    sqlparser::keywords::Keyword::IDENTITY => {}
                    sqlparser::keywords::Keyword::IF => {}
                    sqlparser::keywords::Keyword::IGNORE => {}
                    sqlparser::keywords::Keyword::ILIKE => {}
                    sqlparser::keywords::Keyword::IMMEDIATE => {}
                    sqlparser::keywords::Keyword::IMMUTABLE => {}
                    sqlparser::keywords::Keyword::IN => {}
                    sqlparser::keywords::Keyword::INCLUDE => {}
                    sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES => {}
                    sqlparser::keywords::Keyword::INCREMENT => {}
                    sqlparser::keywords::Keyword::INDEX => {}
                    sqlparser::keywords::Keyword::INDICATOR => {}
                    sqlparser::keywords::Keyword::INHERIT => {}
                    sqlparser::keywords::Keyword::INITIALLY => {}
                    sqlparser::keywords::Keyword::INNER => {}
                    sqlparser::keywords::Keyword::INOUT => {}
                    sqlparser::keywords::Keyword::INPUT => {}
                    sqlparser::keywords::Keyword::INPUTFORMAT => {}
                    sqlparser::keywords::Keyword::INSENSITIVE => {}
                    sqlparser::keywords::Keyword::INSERT => {}
                    sqlparser::keywords::Keyword::INSTALL => {}
                    sqlparser::keywords::Keyword::INSTEAD => {}
                    sqlparser::keywords::Keyword::INT => {}
                    sqlparser::keywords::Keyword::INT128 => {}
                    sqlparser::keywords::Keyword::INT16 => {}
                    sqlparser::keywords::Keyword::INT2 => {}
                    sqlparser::keywords::Keyword::INT256 => {}
                    sqlparser::keywords::Keyword::INT32 => {}
                    sqlparser::keywords::Keyword::INT4 => {}
                    sqlparser::keywords::Keyword::INT64 => {}
                    sqlparser::keywords::Keyword::INT8 => {}
                    sqlparser::keywords::Keyword::INTEGER => {}
                    sqlparser::keywords::Keyword::INTERPOLATE => {}
                    sqlparser::keywords::Keyword::INTERSECT => {}
                    sqlparser::keywords::Keyword::INTERSECTION => {}
                    sqlparser::keywords::Keyword::INTERVAL => {}
                    sqlparser::keywords::Keyword::INTO => {}
                    sqlparser::keywords::Keyword::IS => {}
                    sqlparser::keywords::Keyword::ISODOW => {}
                    sqlparser::keywords::Keyword::ISOLATION => {}
                    sqlparser::keywords::Keyword::ISOWEEK => {}
                    sqlparser::keywords::Keyword::ISOYEAR => {}
                    sqlparser::keywords::Keyword::ITEMS => {}
                    sqlparser::keywords::Keyword::JAR => {}
                    sqlparser::keywords::Keyword::JOIN => {}
                    sqlparser::keywords::Keyword::JSON => {}
                    sqlparser::keywords::Keyword::JSONB => {}
                    sqlparser::keywords::Keyword::JSONFILE => {}
                    sqlparser::keywords::Keyword::JSON_TABLE => {}
                    sqlparser::keywords::Keyword::JULIAN => {}
                    sqlparser::keywords::Keyword::KEY => {}
                    sqlparser::keywords::Keyword::KEYS => {}
                    sqlparser::keywords::Keyword::KILL => {}
                    sqlparser::keywords::Keyword::LAG => {}
                    sqlparser::keywords::Keyword::LANGUAGE => {}
                    sqlparser::keywords::Keyword::LARGE => {}
                    sqlparser::keywords::Keyword::LAST => {}
                    sqlparser::keywords::Keyword::LAST_VALUE => {}
                    sqlparser::keywords::Keyword::LATERAL => {}
                    sqlparser::keywords::Keyword::LEAD => {}
                    sqlparser::keywords::Keyword::LEADING => {}
                    sqlparser::keywords::Keyword::LEFT => {}
                    sqlparser::keywords::Keyword::LEVEL => {}
                    sqlparser::keywords::Keyword::LIKE => {}
                    sqlparser::keywords::Keyword::LIKE_REGEX => {}
                    sqlparser::keywords::Keyword::LIMIT => {}
                    sqlparser::keywords::Keyword::LINES => {}
                    sqlparser::keywords::Keyword::LISTEN => {}
                    sqlparser::keywords::Keyword::LN => {}
                    sqlparser::keywords::Keyword::LOAD => {}
                    sqlparser::keywords::Keyword::LOCAL => {}
                    sqlparser::keywords::Keyword::LOCALTIME => {}
                    sqlparser::keywords::Keyword::LOCALTIMESTAMP => {}
                    sqlparser::keywords::Keyword::LOCATION => {}
                    sqlparser::keywords::Keyword::LOCK => {}
                    sqlparser::keywords::Keyword::LOCKED => {}
                    sqlparser::keywords::Keyword::LOGIN => {}
                    sqlparser::keywords::Keyword::LOGS => {}
                    sqlparser::keywords::Keyword::LOWCARDINALITY => {}
                    sqlparser::keywords::Keyword::LOWER => {}
                    sqlparser::keywords::Keyword::LOW_PRIORITY => {}
                    sqlparser::keywords::Keyword::MACRO => {}
                    sqlparser::keywords::Keyword::MANAGEDLOCATION => {}
                    sqlparser::keywords::Keyword::MAP => {}
                    sqlparser::keywords::Keyword::MASKING => {}
                    sqlparser::keywords::Keyword::MATCH => {}
                    sqlparser::keywords::Keyword::MATCHED => {}
                    sqlparser::keywords::Keyword::MATCHES => {}
                    sqlparser::keywords::Keyword::MATCH_CONDITION => {}
                    sqlparser::keywords::Keyword::MATCH_RECOGNIZE => {}
                    sqlparser::keywords::Keyword::MATERIALIZE => {}
                    sqlparser::keywords::Keyword::MATERIALIZED => {}
                    sqlparser::keywords::Keyword::MAX => {}
                    sqlparser::keywords::Keyword::MAXVALUE => {}
                    sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS => {}
                    sqlparser::keywords::Keyword::MEASURES => {}
                    sqlparser::keywords::Keyword::MEDIUMINT => {}
                    sqlparser::keywords::Keyword::MEMBER => {}
                    sqlparser::keywords::Keyword::MERGE => {}
                    sqlparser::keywords::Keyword::METADATA => {}
                    sqlparser::keywords::Keyword::METHOD => {}
                    sqlparser::keywords::Keyword::MICROSECOND => {}
                    sqlparser::keywords::Keyword::MICROSECONDS => {}
                    sqlparser::keywords::Keyword::MILLENIUM => {}
                    sqlparser::keywords::Keyword::MILLENNIUM => {}
                    sqlparser::keywords::Keyword::MILLISECOND => {}
                    sqlparser::keywords::Keyword::MILLISECONDS => {}
                    sqlparser::keywords::Keyword::MIN => {}
                    sqlparser::keywords::Keyword::MINUTE => {}
                    sqlparser::keywords::Keyword::MINVALUE => {}
                    sqlparser::keywords::Keyword::MOD => {}
                    sqlparser::keywords::Keyword::MODE => {}
                    sqlparser::keywords::Keyword::MODIFIES => {}
                    sqlparser::keywords::Keyword::MODIFY => {}
                    sqlparser::keywords::Keyword::MODULE => {}
                    sqlparser::keywords::Keyword::MONTH => {}
                    sqlparser::keywords::Keyword::MSCK => {}
                    sqlparser::keywords::Keyword::MULTISET => {}
                    sqlparser::keywords::Keyword::MUTATION => {}
                    sqlparser::keywords::Keyword::NAME => {}
                    sqlparser::keywords::Keyword::NANOSECOND => {}
                    sqlparser::keywords::Keyword::NANOSECONDS => {}
                    sqlparser::keywords::Keyword::NATIONAL => {}
                    sqlparser::keywords::Keyword::NATURAL => {}
                    sqlparser::keywords::Keyword::NCHAR => {}
                    sqlparser::keywords::Keyword::NCLOB => {}
                    sqlparser::keywords::Keyword::NESTED => {}
                    sqlparser::keywords::Keyword::NEW => {}
                    sqlparser::keywords::Keyword::NEXT => {}
                    sqlparser::keywords::Keyword::NO => {}
                    sqlparser::keywords::Keyword::NOBYPASSRLS => {}
                    sqlparser::keywords::Keyword::NOCREATEDB => {}
                    sqlparser::keywords::Keyword::NOCREATEROLE => {}
                    sqlparser::keywords::Keyword::NOINHERIT => {}
                    sqlparser::keywords::Keyword::NOLOGIN => {}
                    sqlparser::keywords::Keyword::NONE => {}
                    sqlparser::keywords::Keyword::NOORDER => {}
                    sqlparser::keywords::Keyword::NOREPLICATION => {}
                    sqlparser::keywords::Keyword::NORMALIZE => {}
                    sqlparser::keywords::Keyword::NOSCAN => {}
                    sqlparser::keywords::Keyword::NOSUPERUSER => {}
                    sqlparser::keywords::Keyword::NOT => {}
                    sqlparser::keywords::Keyword::NOTHING => {}
                    sqlparser::keywords::Keyword::NOTIFY => {}
                    sqlparser::keywords::Keyword::NOWAIT => {}
                    sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG => {}
                    sqlparser::keywords::Keyword::NTH_VALUE => {}
                    sqlparser::keywords::Keyword::NTILE => {}
                    sqlparser::keywords::Keyword::NULL => {}
                    sqlparser::keywords::Keyword::NULLABLE => {}
                    sqlparser::keywords::Keyword::NULLIF => {}
                    sqlparser::keywords::Keyword::NULLS => {}
                    sqlparser::keywords::Keyword::NUMERIC => {}
                    sqlparser::keywords::Keyword::NVARCHAR => {}
                    sqlparser::keywords::Keyword::OBJECT => {}
                    sqlparser::keywords::Keyword::OCCURRENCES_REGEX => {}
                    sqlparser::keywords::Keyword::OCTETS => {}
                    sqlparser::keywords::Keyword::OCTET_LENGTH => {}
                    sqlparser::keywords::Keyword::OF => {}
                    sqlparser::keywords::Keyword::OFFSET => {}
                    sqlparser::keywords::Keyword::OLD => {}
                    sqlparser::keywords::Keyword::OMIT => {}
                    sqlparser::keywords::Keyword::ON => {}
                    sqlparser::keywords::Keyword::ONE => {}
                    sqlparser::keywords::Keyword::ONLY => {}
                    sqlparser::keywords::Keyword::OPEN => {}
                    sqlparser::keywords::Keyword::OPERATOR => {}
                    sqlparser::keywords::Keyword::OPTIMIZE => {}
                    sqlparser::keywords::Keyword::OPTIMIZER_COSTS => {}
                    sqlparser::keywords::Keyword::OPTION => {}
                    sqlparser::keywords::Keyword::OPTIONS => {}
                    sqlparser::keywords::Keyword::OR => {}
                    sqlparser::keywords::Keyword::ORC => {}
                    sqlparser::keywords::Keyword::ORDER => {}
                    sqlparser::keywords::Keyword::ORDINALITY => {}
                    sqlparser::keywords::Keyword::OUT => {}
                    sqlparser::keywords::Keyword::OUTER => {}
                    sqlparser::keywords::Keyword::OUTPUTFORMAT => {}
                    sqlparser::keywords::Keyword::OVER => {}
                    sqlparser::keywords::Keyword::OVERFLOW => {}
                    sqlparser::keywords::Keyword::OVERLAPS => {}
                    sqlparser::keywords::Keyword::OVERLAY => {}
                    sqlparser::keywords::Keyword::OVERWRITE => {}
                    sqlparser::keywords::Keyword::OWNED => {}
                    sqlparser::keywords::Keyword::OWNER => {}
                    sqlparser::keywords::Keyword::PARALLEL => {}
                    sqlparser::keywords::Keyword::PARAMETER => {}
                    sqlparser::keywords::Keyword::PARQUET => {}
                    sqlparser::keywords::Keyword::PART => {}
                    sqlparser::keywords::Keyword::PARTITION => {}
                    sqlparser::keywords::Keyword::PARTITIONED => {}
                    sqlparser::keywords::Keyword::PARTITIONS => {}
                    sqlparser::keywords::Keyword::PASSWORD => {}
                    sqlparser::keywords::Keyword::PAST => {}
                    sqlparser::keywords::Keyword::PATH => {}
                    sqlparser::keywords::Keyword::PATTERN => {}
                    sqlparser::keywords::Keyword::PER => {}
                    sqlparser::keywords::Keyword::PERCENT => {}
                    sqlparser::keywords::Keyword::PERCENTILE_CONT => {}
                    sqlparser::keywords::Keyword::PERCENTILE_DISC => {}
                    sqlparser::keywords::Keyword::PERCENT_RANK => {}
                    sqlparser::keywords::Keyword::PERIOD => {}
                    sqlparser::keywords::Keyword::PERMISSIVE => {}
                    sqlparser::keywords::Keyword::PERSISTENT => {}
                    sqlparser::keywords::Keyword::PIVOT => {}
                    sqlparser::keywords::Keyword::PLACING => {}
                    sqlparser::keywords::Keyword::PLAN => {}
                    sqlparser::keywords::Keyword::PLANS => {}
                    sqlparser::keywords::Keyword::POLICY => {}
                    sqlparser::keywords::Keyword::PORTION => {}
                    sqlparser::keywords::Keyword::POSITION => {}
                    sqlparser::keywords::Keyword::POSITION_REGEX => {}
                    sqlparser::keywords::Keyword::POWER => {}
                    sqlparser::keywords::Keyword::PRAGMA => {}
                    sqlparser::keywords::Keyword::PRECEDES => {}
                    sqlparser::keywords::Keyword::PRECEDING => {}
                    sqlparser::keywords::Keyword::PRECISION => {}
                    sqlparser::keywords::Keyword::PREPARE => {}
                    sqlparser::keywords::Keyword::PRESERVE => {}
                    sqlparser::keywords::Keyword::PREWHERE => {}
                    sqlparser::keywords::Keyword::PRIMARY => {}
                    sqlparser::keywords::Keyword::PRIOR => {}
                    sqlparser::keywords::Keyword::PRIVILEGES => {}
                    sqlparser::keywords::Keyword::PROCEDURE => {}
                    sqlparser::keywords::Keyword::PROGRAM => {}
                    sqlparser::keywords::Keyword::PROJECTION => {}
                    sqlparser::keywords::Keyword::PURGE => {}
                    sqlparser::keywords::Keyword::QUALIFY => {}
                    sqlparser::keywords::Keyword::QUARTER => {}
                    sqlparser::keywords::Keyword::QUERY => {}
                    sqlparser::keywords::Keyword::QUOTE => {}
                    sqlparser::keywords::Keyword::RANGE => {}
                    sqlparser::keywords::Keyword::RANK => {}
                    sqlparser::keywords::Keyword::RAW => {}
                    sqlparser::keywords::Keyword::RCFILE => {}
                    sqlparser::keywords::Keyword::READ => {}
                    sqlparser::keywords::Keyword::READS => {}
                    sqlparser::keywords::Keyword::READ_ONLY => {}
                    sqlparser::keywords::Keyword::REAL => {}
                    sqlparser::keywords::Keyword::RECURSIVE => {}
                    sqlparser::keywords::Keyword::REF => {}
                    sqlparser::keywords::Keyword::REFERENCES => {}
                    sqlparser::keywords::Keyword::REFERENCING => {}
                    sqlparser::keywords::Keyword::REGCLASS => {}
                    sqlparser::keywords::Keyword::REGEXP => {}
                    sqlparser::keywords::Keyword::REGR_AVGX => {}
                    sqlparser::keywords::Keyword::REGR_AVGY => {}
                    sqlparser::keywords::Keyword::REGR_COUNT => {}
                    sqlparser::keywords::Keyword::REGR_INTERCEPT => {}
                    sqlparser::keywords::Keyword::REGR_R2 => {}
                    sqlparser::keywords::Keyword::REGR_SLOPE => {}
                    sqlparser::keywords::Keyword::REGR_SXX => {}
                    sqlparser::keywords::Keyword::REGR_SXY => {}
                    sqlparser::keywords::Keyword::REGR_SYY => {}
                    sqlparser::keywords::Keyword::RELATIVE => {}
                    sqlparser::keywords::Keyword::RELAY => {}
                    sqlparser::keywords::Keyword::RELEASE => {}
                    sqlparser::keywords::Keyword::REMOTE => {}
                    sqlparser::keywords::Keyword::RENAME => {}
                    sqlparser::keywords::Keyword::REORG => {}
                    sqlparser::keywords::Keyword::REPAIR => {}
                    sqlparser::keywords::Keyword::REPEATABLE => {}
                    sqlparser::keywords::Keyword::REPLACE => {}
                    sqlparser::keywords::Keyword::REPLICA => {}
                    sqlparser::keywords::Keyword::REPLICATION => {}
                    sqlparser::keywords::Keyword::RESET => {}
                    sqlparser::keywords::Keyword::RESPECT => {}
                    sqlparser::keywords::Keyword::RESTART => {}
                    sqlparser::keywords::Keyword::RESTRICT => {}
                    sqlparser::keywords::Keyword::RESTRICTED => {}
                    sqlparser::keywords::Keyword::RESTRICTIVE => {}
                    sqlparser::keywords::Keyword::RESULT => {}
                    sqlparser::keywords::Keyword::RESULTSET => {}
                    sqlparser::keywords::Keyword::RETAIN => {}
                    sqlparser::keywords::Keyword::RETURN => {}
                    sqlparser::keywords::Keyword::RETURNING => {}
                    sqlparser::keywords::Keyword::RETURNS => {}
                    sqlparser::keywords::Keyword::REVOKE => {}
                    sqlparser::keywords::Keyword::RIGHT => {}
                    sqlparser::keywords::Keyword::RLIKE => {}
                    sqlparser::keywords::Keyword::ROLE => {}
                    sqlparser::keywords::Keyword::ROLLBACK => {}
                    sqlparser::keywords::Keyword::ROLLUP => {}
                    sqlparser::keywords::Keyword::ROOT => {}
                    sqlparser::keywords::Keyword::ROW => {}
                    sqlparser::keywords::Keyword::ROWID => {}
                    sqlparser::keywords::Keyword::ROWS => {}
                    sqlparser::keywords::Keyword::ROW_NUMBER => {}
                    sqlparser::keywords::Keyword::RULE => {}
                    sqlparser::keywords::Keyword::RUN => {}
                    sqlparser::keywords::Keyword::SAFE => {}
                    sqlparser::keywords::Keyword::SAFE_CAST => {}
                    sqlparser::keywords::Keyword::SAVEPOINT => {}
                    sqlparser::keywords::Keyword::SCHEMA => {}
                    sqlparser::keywords::Keyword::SCHEMAS => {}
                    sqlparser::keywords::Keyword::SCOPE => {}
                    sqlparser::keywords::Keyword::SCROLL => {}
                    sqlparser::keywords::Keyword::SEARCH => {}
                    sqlparser::keywords::Keyword::SECOND => {}
                    sqlparser::keywords::Keyword::SECRET => {}
                    sqlparser::keywords::Keyword::SECURITY => {}
                    sqlparser::keywords::Keyword::SELECT => {}
                    sqlparser::keywords::Keyword::SEMI => {}
                    sqlparser::keywords::Keyword::SENSITIVE => {}
                    sqlparser::keywords::Keyword::SEPARATOR => {}
                    sqlparser::keywords::Keyword::SEQUENCE => {}
                    sqlparser::keywords::Keyword::SEQUENCEFILE => {}
                    sqlparser::keywords::Keyword::SEQUENCES => {}
                    sqlparser::keywords::Keyword::SERDE => {}
                    sqlparser::keywords::Keyword::SERDEPROPERTIES => {}
                    sqlparser::keywords::Keyword::SERIALIZABLE => {}
                    sqlparser::keywords::Keyword::SESSION => {}
                    sqlparser::keywords::Keyword::SESSION_USER => {}
                    sqlparser::keywords::Keyword::SET => {}
                    sqlparser::keywords::Keyword::SETS => {}
                    sqlparser::keywords::Keyword::SETTINGS => {}
                    sqlparser::keywords::Keyword::SHARE => {}
                    sqlparser::keywords::Keyword::SHOW => {}
                    sqlparser::keywords::Keyword::SIMILAR => {}
                    sqlparser::keywords::Keyword::SKIP => {}
                    sqlparser::keywords::Keyword::SLOW => {}
                    sqlparser::keywords::Keyword::SMALLINT => {}
                    sqlparser::keywords::Keyword::SNAPSHOT => {}
                    sqlparser::keywords::Keyword::SOME => {}
                    sqlparser::keywords::Keyword::SORT => {}
                    sqlparser::keywords::Keyword::SORTED => {}
                    sqlparser::keywords::Keyword::SOURCE => {}
                    sqlparser::keywords::Keyword::SPATIAL => {}
                    sqlparser::keywords::Keyword::SPECIFIC => {}
                    sqlparser::keywords::Keyword::SPECIFICTYPE => {}
                    sqlparser::keywords::Keyword::SQL => {}
                    sqlparser::keywords::Keyword::SQLEXCEPTION => {}
                    sqlparser::keywords::Keyword::SQLSTATE => {}
                    sqlparser::keywords::Keyword::SQLWARNING => {}
                    sqlparser::keywords::Keyword::SQRT => {}
                    sqlparser::keywords::Keyword::STABLE => {}
                    sqlparser::keywords::Keyword::STAGE => {}
                    sqlparser::keywords::Keyword::START => {}
                    sqlparser::keywords::Keyword::STATEMENT => {}
                    sqlparser::keywords::Keyword::STATIC => {}
                    sqlparser::keywords::Keyword::STATISTICS => {}
                    sqlparser::keywords::Keyword::STATUS => {}
                    sqlparser::keywords::Keyword::STDDEV_POP => {}
                    sqlparser::keywords::Keyword::STDDEV_SAMP => {}
                    sqlparser::keywords::Keyword::STDIN => {}
                    sqlparser::keywords::Keyword::STDOUT => {}
                    sqlparser::keywords::Keyword::STEP => {}
                    sqlparser::keywords::Keyword::STORAGE_INTEGRATION => {}
                    sqlparser::keywords::Keyword::STORED => {}
                    sqlparser::keywords::Keyword::STRICT => {}
                    sqlparser::keywords::Keyword::STRING => {}
                    sqlparser::keywords::Keyword::STRUCT => {}
                    sqlparser::keywords::Keyword::SUBMULTISET => {}
                    sqlparser::keywords::Keyword::SUBSTRING => {}
                    sqlparser::keywords::Keyword::SUBSTRING_REGEX => {}
                    sqlparser::keywords::Keyword::SUCCEEDS => {}
                    sqlparser::keywords::Keyword::SUM => {}
                    sqlparser::keywords::Keyword::SUPER => {}
                    sqlparser::keywords::Keyword::SUPERUSER => {}
                    sqlparser::keywords::Keyword::SWAP => {}
                    sqlparser::keywords::Keyword::SYMMETRIC => {}
                    sqlparser::keywords::Keyword::SYNC => {}
                    sqlparser::keywords::Keyword::SYSTEM => {}
                    sqlparser::keywords::Keyword::SYSTEM_TIME => {}
                    sqlparser::keywords::Keyword::SYSTEM_USER => {}
                    sqlparser::keywords::Keyword::TABLE => {}
                    sqlparser::keywords::Keyword::TABLES => {}
                    sqlparser::keywords::Keyword::TABLESAMPLE => {}
                    sqlparser::keywords::Keyword::TAG => {}
                    sqlparser::keywords::Keyword::TARGET => {}
                    sqlparser::keywords::Keyword::TBLPROPERTIES => {}
                    sqlparser::keywords::Keyword::TEMP => {}
                    sqlparser::keywords::Keyword::TEMPORARY => {}
                    sqlparser::keywords::Keyword::TERMINATED => {}
                    sqlparser::keywords::Keyword::TEXT => {}
                    sqlparser::keywords::Keyword::TEXTFILE => {}
                    sqlparser::keywords::Keyword::THEN => {}
                    sqlparser::keywords::Keyword::TIES => {}
                    sqlparser::keywords::Keyword::TIME => {}
                    sqlparser::keywords::Keyword::TIMESTAMP => {}
                    sqlparser::keywords::Keyword::TIMESTAMPTZ => {}
                    sqlparser::keywords::Keyword::TIMETZ => {}
                    sqlparser::keywords::Keyword::TIMEZONE => {}
                    sqlparser::keywords::Keyword::TIMEZONE_ABBR => {}
                    sqlparser::keywords::Keyword::TIMEZONE_HOUR => {}
                    sqlparser::keywords::Keyword::TIMEZONE_MINUTE => {}
                    sqlparser::keywords::Keyword::TIMEZONE_REGION => {}
                    sqlparser::keywords::Keyword::TINYINT => {}
                    sqlparser::keywords::Keyword::TO => {}
                    sqlparser::keywords::Keyword::TOP => {}
                    sqlparser::keywords::Keyword::TOTALS => {}
                    sqlparser::keywords::Keyword::TRAILING => {}
                    sqlparser::keywords::Keyword::TRANSACTION => {}
                    sqlparser::keywords::Keyword::TRANSIENT => {}
                    sqlparser::keywords::Keyword::TRANSLATE => {}
                    sqlparser::keywords::Keyword::TRANSLATE_REGEX => {}
                    sqlparser::keywords::Keyword::TRANSLATION => {}
                    sqlparser::keywords::Keyword::TREAT => {}
                    sqlparser::keywords::Keyword::TRIGGER => {}
                    sqlparser::keywords::Keyword::TRIM => {}
                    sqlparser::keywords::Keyword::TRIM_ARRAY => {}
                    sqlparser::keywords::Keyword::TRUE => {}
                    sqlparser::keywords::Keyword::TRUNCATE => {}
                    sqlparser::keywords::Keyword::TRY_CAST => {}
                    sqlparser::keywords::Keyword::TRY_CONVERT => {}
                    sqlparser::keywords::Keyword::TUPLE => {}
                    sqlparser::keywords::Keyword::TYPE => {}
                    sqlparser::keywords::Keyword::UESCAPE => {}
                    sqlparser::keywords::Keyword::UINT128 => {}
                    sqlparser::keywords::Keyword::UINT16 => {}
                    sqlparser::keywords::Keyword::UINT256 => {}
                    sqlparser::keywords::Keyword::UINT32 => {}
                    sqlparser::keywords::Keyword::UINT64 => {}
                    sqlparser::keywords::Keyword::UINT8 => {}
                    sqlparser::keywords::Keyword::UNBOUNDED => {}
                    sqlparser::keywords::Keyword::UNCACHE => {}
                    sqlparser::keywords::Keyword::UNCOMMITTED => {}
                    sqlparser::keywords::Keyword::UNFREEZE => {}
                    sqlparser::keywords::Keyword::UNION => {}
                    sqlparser::keywords::Keyword::UNIQUE => {}
                    sqlparser::keywords::Keyword::UNKNOWN => {}
                    sqlparser::keywords::Keyword::UNLOAD => {}
                    sqlparser::keywords::Keyword::UNLOCK => {}
                    sqlparser::keywords::Keyword::UNLOGGED => {}
                    sqlparser::keywords::Keyword::UNMATCHED => {}
                    sqlparser::keywords::Keyword::UNNEST => {}
                    sqlparser::keywords::Keyword::UNPIVOT => {}
                    sqlparser::keywords::Keyword::UNSAFE => {}
                    sqlparser::keywords::Keyword::UNSIGNED => {}
                    sqlparser::keywords::Keyword::UNTIL => {}
                    sqlparser::keywords::Keyword::UPDATE => {}
                    sqlparser::keywords::Keyword::UPPER => {}
                    sqlparser::keywords::Keyword::URL => {}
                    sqlparser::keywords::Keyword::USAGE => {}
                    sqlparser::keywords::Keyword::USE => {}
                    sqlparser::keywords::Keyword::USER => {}
                    sqlparser::keywords::Keyword::USER_RESOURCES => {}
                    sqlparser::keywords::Keyword::USING => {}
                    sqlparser::keywords::Keyword::UUID => {}
                    sqlparser::keywords::Keyword::VACUUM => {}
                    sqlparser::keywords::Keyword::VALID => {}
                    sqlparser::keywords::Keyword::VALIDATION_MODE => {}
                    sqlparser::keywords::Keyword::VALUE => {}
                    sqlparser::keywords::Keyword::VALUES => {}
                    sqlparser::keywords::Keyword::VALUE_OF => {}
                    sqlparser::keywords::Keyword::VARBINARY => {}
                    sqlparser::keywords::Keyword::VARCHAR => {}
                    sqlparser::keywords::Keyword::VARIABLES => {}
                    sqlparser::keywords::Keyword::VARYING => {}
                    sqlparser::keywords::Keyword::VAR_POP => {}
                    sqlparser::keywords::Keyword::VAR_SAMP => {}
                    sqlparser::keywords::Keyword::VERBOSE => {}
                    sqlparser::keywords::Keyword::VERSION => {}
                    sqlparser::keywords::Keyword::VERSIONING => {}
                    sqlparser::keywords::Keyword::VIEW => {}
                    sqlparser::keywords::Keyword::VIEWS => {}
                    sqlparser::keywords::Keyword::VIRTUAL => {}
                    sqlparser::keywords::Keyword::VOLATILE => {}
                    sqlparser::keywords::Keyword::WAREHOUSE => {}
                    sqlparser::keywords::Keyword::WEEK => {}
                    sqlparser::keywords::Keyword::WHEN => {}
                    sqlparser::keywords::Keyword::WHENEVER => {}
                    sqlparser::keywords::Keyword::WHERE => {}
                    sqlparser::keywords::Keyword::WIDTH_BUCKET => {}
                    sqlparser::keywords::Keyword::WINDOW => {}
                    sqlparser::keywords::Keyword::WITH => {}
                    sqlparser::keywords::Keyword::WITHIN => {}
                    sqlparser::keywords::Keyword::WITHOUT => {}
                    sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER => {}
                    sqlparser::keywords::Keyword::WORK => {}
                    sqlparser::keywords::Keyword::WRITE => {}
                    sqlparser::keywords::Keyword::XML => {}
                    sqlparser::keywords::Keyword::XOR => {}
                    sqlparser::keywords::Keyword::YEAR => {}
                    sqlparser::keywords::Keyword::ZONE => {}
                    sqlparser::keywords::Keyword::ZORDER => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::keywords::Keyword {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::tokenizer::Token {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::tokenizer::Token::EOF => {}
                    sqlparser::tokenizer::Token::Word(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::Number(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::Char(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::SingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::DoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::TripleSingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::TripleDoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::DollarQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::NationalStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::EscapedStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::UnicodeStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::HexStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::Comma => {}
                    sqlparser::tokenizer::Token::Whitespace(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::DoubleEq => {}
                    sqlparser::tokenizer::Token::Eq => {}
                    sqlparser::tokenizer::Token::Neq => {}
                    sqlparser::tokenizer::Token::Lt => {}
                    sqlparser::tokenizer::Token::Gt => {}
                    sqlparser::tokenizer::Token::LtEq => {}
                    sqlparser::tokenizer::Token::GtEq => {}
                    sqlparser::tokenizer::Token::Spaceship => {}
                    sqlparser::tokenizer::Token::Plus => {}
                    sqlparser::tokenizer::Token::Minus => {}
                    sqlparser::tokenizer::Token::Mul => {}
                    sqlparser::tokenizer::Token::Div => {}
                    sqlparser::tokenizer::Token::DuckIntDiv => {}
                    sqlparser::tokenizer::Token::Mod => {}
                    sqlparser::tokenizer::Token::StringConcat => {}
                    sqlparser::tokenizer::Token::LParen => {}
                    sqlparser::tokenizer::Token::RParen => {}
                    sqlparser::tokenizer::Token::Period => {}
                    sqlparser::tokenizer::Token::Colon => {}
                    sqlparser::tokenizer::Token::DoubleColon => {}
                    sqlparser::tokenizer::Token::Assignment => {}
                    sqlparser::tokenizer::Token::SemiColon => {}
                    sqlparser::tokenizer::Token::Backslash => {}
                    sqlparser::tokenizer::Token::LBracket => {}
                    sqlparser::tokenizer::Token::RBracket => {}
                    sqlparser::tokenizer::Token::Ampersand => {}
                    sqlparser::tokenizer::Token::Pipe => {}
                    sqlparser::tokenizer::Token::Caret => {}
                    sqlparser::tokenizer::Token::LBrace => {}
                    sqlparser::tokenizer::Token::RBrace => {}
                    sqlparser::tokenizer::Token::RArrow => {}
                    sqlparser::tokenizer::Token::Sharp => {}
                    sqlparser::tokenizer::Token::Tilde => {}
                    sqlparser::tokenizer::Token::TildeAsterisk => {}
                    sqlparser::tokenizer::Token::ExclamationMarkTilde => {}
                    sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk => {}
                    sqlparser::tokenizer::Token::DoubleTilde => {}
                    sqlparser::tokenizer::Token::DoubleTildeAsterisk => {}
                    sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde => {}
                    sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk => {}
                    sqlparser::tokenizer::Token::ShiftLeft => {}
                    sqlparser::tokenizer::Token::ShiftRight => {}
                    sqlparser::tokenizer::Token::Overlap => {}
                    sqlparser::tokenizer::Token::ExclamationMark => {}
                    sqlparser::tokenizer::Token::DoubleExclamationMark => {}
                    sqlparser::tokenizer::Token::AtSign => {}
                    sqlparser::tokenizer::Token::CaretAt => {}
                    sqlparser::tokenizer::Token::PGSquareRoot => {}
                    sqlparser::tokenizer::Token::PGCubeRoot => {}
                    sqlparser::tokenizer::Token::Placeholder(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Token::Arrow => {}
                    sqlparser::tokenizer::Token::LongArrow => {}
                    sqlparser::tokenizer::Token::HashArrow => {}
                    sqlparser::tokenizer::Token::HashLongArrow => {}
                    sqlparser::tokenizer::Token::AtArrow => {}
                    sqlparser::tokenizer::Token::ArrowAt => {}
                    sqlparser::tokenizer::Token::HashMinus => {}
                    sqlparser::tokenizer::Token::AtQuestion => {}
                    sqlparser::tokenizer::Token::AtAt => {}
                    sqlparser::tokenizer::Token::Question => {}
                    sqlparser::tokenizer::Token::QuestionAnd => {}
                    sqlparser::tokenizer::Token::QuestionPipe => {}
                    sqlparser::tokenizer::Token::CustomBinaryOperator(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::tokenizer::Token {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::tokenizer::Whitespace {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                match self {
                    sqlparser::tokenizer::Whitespace::Space => {}
                    sqlparser::tokenizer::Whitespace::Newline => {}
                    sqlparser::tokenizer::Whitespace::Tab => {}
                    sqlparser::tokenizer::Whitespace::SingleLineComment {
                        comment,
                        prefix,
                    } => {
                        comment.accept(visitor)?;
                        prefix.accept(visitor)?;
                    }
                    sqlparser::tokenizer::Whitespace::MultiLineComment(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::tokenizer::Whitespace {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqlparser::tokenizer::Word {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.value.accept(visitor)?;
                self.quote_style.accept(visitor)?;
                self.keyword.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqlparser::tokenizer::Word {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for bigdecimal::BigDecimal {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for bigdecimal::BigDecimal {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for bool {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for bool {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for char {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for char {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for i16 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for i16 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for i32 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for i32 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for i64 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for i64 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for i8 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for i8 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for String {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for String {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for u16 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for u16 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for u32 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for u32 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for u64 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for u64 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for u8 {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| { std::ops::ControlFlow::Continue(()) },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for u8 {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
