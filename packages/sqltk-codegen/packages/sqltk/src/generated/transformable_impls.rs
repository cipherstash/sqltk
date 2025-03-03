#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Action {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Action::Connect => {
                transformer.transform(self, sqlparser::ast::Action::Connect)
            }
            sqlparser::ast::Action::Create => {
                transformer.transform(self, sqlparser::ast::Action::Create)
            }
            sqlparser::ast::Action::Delete => {
                transformer.transform(self, sqlparser::ast::Action::Delete)
            }
            sqlparser::ast::Action::Execute => {
                transformer.transform(self, sqlparser::ast::Action::Execute)
            }
            sqlparser::ast::Action::Insert { columns } => {
                let transformed = sqlparser::ast::Action::Insert {
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Action::References { columns } => {
                let transformed = sqlparser::ast::Action::References {
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Action::Select { columns } => {
                let transformed = sqlparser::ast::Action::Select {
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Action::Temporary => {
                transformer.transform(self, sqlparser::ast::Action::Temporary)
            }
            sqlparser::ast::Action::Trigger => {
                transformer.transform(self, sqlparser::ast::Action::Trigger)
            }
            sqlparser::ast::Action::Truncate => {
                transformer.transform(self, sqlparser::ast::Action::Truncate)
            }
            sqlparser::ast::Action::Update { columns } => {
                let transformed = sqlparser::ast::Action::Update {
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Action::Usage => {
                transformer.transform(self, sqlparser::ast::Action::Usage)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AddDropSync {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AddDropSync::ADD => {
                transformer.transform(self, sqlparser::ast::AddDropSync::ADD)
            }
            sqlparser::ast::AddDropSync::DROP => {
                transformer.transform(self, sqlparser::ast::AddDropSync::DROP)
            }
            sqlparser::ast::AddDropSync::SYNC => {
                transformer.transform(self, sqlparser::ast::AddDropSync::SYNC)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AfterMatchSkip {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AfterMatchSkip::PastLastRow => {
                transformer.transform(self, sqlparser::ast::AfterMatchSkip::PastLastRow)
            }
            sqlparser::ast::AfterMatchSkip::ToNextRow => {
                transformer.transform(self, sqlparser::ast::AfterMatchSkip::ToNextRow)
            }
            sqlparser::ast::AfterMatchSkip::ToFirst(field0) => {
                let transformed = sqlparser::ast::AfterMatchSkip::ToFirst(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AfterMatchSkip::ToLast(field0) => {
                let transformed = sqlparser::ast::AfterMatchSkip::ToLast(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterColumnOperation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AlterColumnOperation::SetNotNull => {
                transformer
                    .transform(self, sqlparser::ast::AlterColumnOperation::SetNotNull)
            }
            sqlparser::ast::AlterColumnOperation::DropNotNull => {
                transformer
                    .transform(self, sqlparser::ast::AlterColumnOperation::DropNotNull)
            }
            sqlparser::ast::AlterColumnOperation::SetDefault { value } => {
                let transformed = sqlparser::ast::AlterColumnOperation::SetDefault {
                    value: value.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterColumnOperation::DropDefault => {
                transformer
                    .transform(self, sqlparser::ast::AlterColumnOperation::DropDefault)
            }
            sqlparser::ast::AlterColumnOperation::SetDataType { data_type, using } => {
                let transformed = sqlparser::ast::AlterColumnOperation::SetDataType {
                    data_type: data_type.apply_transform(transformer)?,
                    using: using.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterColumnOperation::AddGenerated {
                generated_as,
                sequence_options,
            } => {
                let transformed = sqlparser::ast::AlterColumnOperation::AddGenerated {
                    generated_as: generated_as.apply_transform(transformer)?,
                    sequence_options: sequence_options.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterIndexOperation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AlterIndexOperation::RenameIndex { index_name } => {
                let transformed = sqlparser::ast::AlterIndexOperation::RenameIndex {
                    index_name: index_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterPolicyOperation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AlterPolicyOperation::Rename { new_name } => {
                let transformed = sqlparser::ast::AlterPolicyOperation::Rename {
                    new_name: new_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterPolicyOperation::Apply { to, using, with_check } => {
                let transformed = sqlparser::ast::AlterPolicyOperation::Apply {
                    to: to.apply_transform(transformer)?,
                    using: using.apply_transform(transformer)?,
                    with_check: with_check.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterRoleOperation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AlterRoleOperation::RenameRole { role_name } => {
                let transformed = sqlparser::ast::AlterRoleOperation::RenameRole {
                    role_name: role_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterRoleOperation::AddMember { member_name } => {
                let transformed = sqlparser::ast::AlterRoleOperation::AddMember {
                    member_name: member_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterRoleOperation::DropMember { member_name } => {
                let transformed = sqlparser::ast::AlterRoleOperation::DropMember {
                    member_name: member_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterRoleOperation::WithOptions { options } => {
                let transformed = sqlparser::ast::AlterRoleOperation::WithOptions {
                    options: options.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterRoleOperation::Set {
                config_name,
                config_value,
                in_database,
            } => {
                let transformed = sqlparser::ast::AlterRoleOperation::Set {
                    config_name: config_name.apply_transform(transformer)?,
                    config_value: config_value.apply_transform(transformer)?,
                    in_database: in_database.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterRoleOperation::Reset { config_name, in_database } => {
                let transformed = sqlparser::ast::AlterRoleOperation::Reset {
                    config_name: config_name.apply_transform(transformer)?,
                    in_database: in_database.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterTableOperation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AlterTableOperation::AddConstraint(field0) => {
                let transformed = sqlparser::ast::AlterTableOperation::AddConstraint(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::AddColumn {
                column_keyword,
                if_not_exists,
                column_def,
                column_position,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::AddColumn {
                    column_keyword: column_keyword.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    column_def: column_def.apply_transform(transformer)?,
                    column_position: column_position.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::AddProjection {
                if_not_exists,
                name,
                select,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::AddProjection {
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    select: select.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DropProjection { if_exists, name } => {
                let transformed = sqlparser::ast::AlterTableOperation::DropProjection {
                    if_exists: if_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::MaterializeProjection {
                if_exists,
                name,
                partition,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::MaterializeProjection {
                    if_exists: if_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    partition: partition.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::ClearProjection {
                if_exists,
                name,
                partition,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::ClearProjection {
                    if_exists: if_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    partition: partition.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity,
                    )
            }
            sqlparser::ast::AlterTableOperation::DisableRule { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::DisableRule {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DisableTrigger { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::DisableTrigger {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DropConstraint {
                if_exists,
                name,
                cascade,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::DropConstraint {
                    if_exists: if_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    cascade: cascade.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DropColumn {
                column_name,
                if_exists,
                cascade,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::DropColumn {
                    column_name: column_name.apply_transform(transformer)?,
                    if_exists: if_exists.apply_transform(transformer)?,
                    cascade: cascade.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::AttachPartition { partition } => {
                let transformed = sqlparser::ast::AlterTableOperation::AttachPartition {
                    partition: partition.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DetachPartition { partition } => {
                let transformed = sqlparser::ast::AlterTableOperation::DetachPartition {
                    partition: partition.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::FreezePartition {
                partition,
                with_name,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::FreezePartition {
                    partition: partition.apply_transform(transformer)?,
                    with_name: with_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::UnfreezePartition {
                partition,
                with_name,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::UnfreezePartition {
                    partition: partition.apply_transform(transformer)?,
                    with_name: with_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DropPrimaryKey => {
                transformer
                    .transform(self, sqlparser::ast::AlterTableOperation::DropPrimaryKey)
            }
            sqlparser::ast::AlterTableOperation::EnableAlwaysRule { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::EnableAlwaysRule {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::EnableReplicaRule { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::EnableReplicaRule {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::EnableReplicaTrigger { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::EnableReplicaTrigger {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity,
                    )
            }
            sqlparser::ast::AlterTableOperation::EnableRule { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::EnableRule {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::EnableTrigger { name } => {
                let transformed = sqlparser::ast::AlterTableOperation::EnableTrigger {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::RenamePartitions {
                old_partitions,
                new_partitions,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::RenamePartitions {
                    old_partitions: old_partitions.apply_transform(transformer)?,
                    new_partitions: new_partitions.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::AddPartitions {
                if_not_exists,
                new_partitions,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::AddPartitions {
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    new_partitions: new_partitions.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::DropPartitions {
                partitions,
                if_exists,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::DropPartitions {
                    partitions: partitions.apply_transform(transformer)?,
                    if_exists: if_exists.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::RenameColumn {
                old_column_name,
                new_column_name,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::RenameColumn {
                    old_column_name: old_column_name.apply_transform(transformer)?,
                    new_column_name: new_column_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::RenameTable { table_name } => {
                let transformed = sqlparser::ast::AlterTableOperation::RenameTable {
                    table_name: table_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::ChangeColumn {
                old_name,
                new_name,
                data_type,
                options,
                column_position,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::ChangeColumn {
                    old_name: old_name.apply_transform(transformer)?,
                    new_name: new_name.apply_transform(transformer)?,
                    data_type: data_type.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                    column_position: column_position.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::ModifyColumn {
                col_name,
                data_type,
                options,
                column_position,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::ModifyColumn {
                    col_name: col_name.apply_transform(transformer)?,
                    data_type: data_type.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                    column_position: column_position.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::RenameConstraint {
                old_name,
                new_name,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::RenameConstraint {
                    old_name: old_name.apply_transform(transformer)?,
                    new_name: new_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::AlterColumn { column_name, op } => {
                let transformed = sqlparser::ast::AlterTableOperation::AlterColumn {
                    column_name: column_name.apply_transform(transformer)?,
                    op: op.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::SwapWith { table_name } => {
                let transformed = sqlparser::ast::AlterTableOperation::SwapWith {
                    table_name: table_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::SetTblProperties {
                table_properties,
            } => {
                let transformed = sqlparser::ast::AlterTableOperation::SetTblProperties {
                    table_properties: table_properties.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AlterTableOperation::OwnerTo { new_owner } => {
                let transformed = sqlparser::ast::AlterTableOperation::OwnerTo {
                    new_owner: new_owner.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AnalyzeFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AnalyzeFormat::TEXT => {
                transformer.transform(self, sqlparser::ast::AnalyzeFormat::TEXT)
            }
            sqlparser::ast::AnalyzeFormat::GRAPHVIZ => {
                transformer.transform(self, sqlparser::ast::AnalyzeFormat::GRAPHVIZ)
            }
            sqlparser::ast::AnalyzeFormat::JSON => {
                transformer.transform(self, sqlparser::ast::AnalyzeFormat::JSON)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ArgMode {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ArgMode::In => {
                transformer.transform(self, sqlparser::ast::ArgMode::In)
            }
            sqlparser::ast::ArgMode::Out => {
                transformer.transform(self, sqlparser::ast::ArgMode::Out)
            }
            sqlparser::ast::ArgMode::InOut => {
                transformer.transform(self, sqlparser::ast::ArgMode::InOut)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Array {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { elem, named } = self;
        let transformed = Self {
            elem: elem.apply_transform(transformer)?,
            named: named.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ArrayElemTypeDef {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ArrayElemTypeDef::None => {
                transformer.transform(self, sqlparser::ast::ArrayElemTypeDef::None)
            }
            sqlparser::ast::ArrayElemTypeDef::AngleBracket(field0) => {
                let transformed = sqlparser::ast::ArrayElemTypeDef::AngleBracket(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ArrayElemTypeDef::SquareBracket(field0, field1) => {
                let transformed = sqlparser::ast::ArrayElemTypeDef::SquareBracket(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ArrayElemTypeDef::Parenthesis(field0) => {
                let transformed = sqlparser::ast::ArrayElemTypeDef::Parenthesis(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Assignment {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { target, value } = self;
        let transformed = Self {
            target: target.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AssignmentTarget {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AssignmentTarget::ColumnName(field0) => {
                let transformed = sqlparser::ast::AssignmentTarget::ColumnName(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AssignmentTarget::Tuple(field0) => {
                let transformed = sqlparser::ast::AssignmentTarget::Tuple(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AttachDuckDBDatabaseOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(field0) => {
                let transformed = sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::AttachDuckDBDatabaseOption::Type(field0) => {
                let transformed = sqlparser::ast::AttachDuckDBDatabaseOption::Type(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::BinaryOperator {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::BinaryOperator::Plus => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Plus)
            }
            sqlparser::ast::BinaryOperator::Minus => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Minus)
            }
            sqlparser::ast::BinaryOperator::Multiply => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Multiply)
            }
            sqlparser::ast::BinaryOperator::Divide => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Divide)
            }
            sqlparser::ast::BinaryOperator::Modulo => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Modulo)
            }
            sqlparser::ast::BinaryOperator::StringConcat => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::StringConcat)
            }
            sqlparser::ast::BinaryOperator::Gt => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Gt)
            }
            sqlparser::ast::BinaryOperator::Lt => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Lt)
            }
            sqlparser::ast::BinaryOperator::GtEq => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::GtEq)
            }
            sqlparser::ast::BinaryOperator::LtEq => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::LtEq)
            }
            sqlparser::ast::BinaryOperator::Spaceship => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Spaceship)
            }
            sqlparser::ast::BinaryOperator::Eq => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Eq)
            }
            sqlparser::ast::BinaryOperator::NotEq => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::NotEq)
            }
            sqlparser::ast::BinaryOperator::And => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::And)
            }
            sqlparser::ast::BinaryOperator::Or => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Or)
            }
            sqlparser::ast::BinaryOperator::Xor => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Xor)
            }
            sqlparser::ast::BinaryOperator::BitwiseOr => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::BitwiseOr)
            }
            sqlparser::ast::BinaryOperator::BitwiseAnd => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::BitwiseAnd)
            }
            sqlparser::ast::BinaryOperator::BitwiseXor => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::BitwiseXor)
            }
            sqlparser::ast::BinaryOperator::DuckIntegerDivide => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::DuckIntegerDivide)
            }
            sqlparser::ast::BinaryOperator::MyIntegerDivide => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::MyIntegerDivide)
            }
            sqlparser::ast::BinaryOperator::Custom(field0) => {
                let transformed = sqlparser::ast::BinaryOperator::Custom(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::BinaryOperator::PGBitwiseXor => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGBitwiseXor)
            }
            sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft)
            }
            sqlparser::ast::BinaryOperator::PGBitwiseShiftRight => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGBitwiseShiftRight)
            }
            sqlparser::ast::BinaryOperator::PGExp => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGExp)
            }
            sqlparser::ast::BinaryOperator::PGOverlap => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGOverlap)
            }
            sqlparser::ast::BinaryOperator::PGRegexMatch => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGRegexMatch)
            }
            sqlparser::ast::BinaryOperator::PGRegexIMatch => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGRegexIMatch)
            }
            sqlparser::ast::BinaryOperator::PGRegexNotMatch => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGRegexNotMatch)
            }
            sqlparser::ast::BinaryOperator::PGRegexNotIMatch => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGRegexNotIMatch)
            }
            sqlparser::ast::BinaryOperator::PGLikeMatch => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGLikeMatch)
            }
            sqlparser::ast::BinaryOperator::PGILikeMatch => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGILikeMatch)
            }
            sqlparser::ast::BinaryOperator::PGNotLikeMatch => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGNotLikeMatch)
            }
            sqlparser::ast::BinaryOperator::PGNotILikeMatch => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::PGNotILikeMatch)
            }
            sqlparser::ast::BinaryOperator::PGStartsWith => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::PGStartsWith)
            }
            sqlparser::ast::BinaryOperator::Arrow => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Arrow)
            }
            sqlparser::ast::BinaryOperator::LongArrow => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::LongArrow)
            }
            sqlparser::ast::BinaryOperator::HashArrow => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::HashArrow)
            }
            sqlparser::ast::BinaryOperator::HashLongArrow => {
                transformer
                    .transform(self, sqlparser::ast::BinaryOperator::HashLongArrow)
            }
            sqlparser::ast::BinaryOperator::AtAt => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::AtAt)
            }
            sqlparser::ast::BinaryOperator::AtArrow => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::AtArrow)
            }
            sqlparser::ast::BinaryOperator::ArrowAt => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::ArrowAt)
            }
            sqlparser::ast::BinaryOperator::HashMinus => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::HashMinus)
            }
            sqlparser::ast::BinaryOperator::AtQuestion => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::AtQuestion)
            }
            sqlparser::ast::BinaryOperator::Question => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::Question)
            }
            sqlparser::ast::BinaryOperator::QuestionAnd => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::QuestionAnd)
            }
            sqlparser::ast::BinaryOperator::QuestionPipe => {
                transformer.transform(self, sqlparser::ast::BinaryOperator::QuestionPipe)
            }
            sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(field0) => {
                let transformed = sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CastFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CastFormat::Value(field0) => {
                let transformed = sqlparser::ast::CastFormat::Value(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CastFormat::ValueAtTimeZone(field0, field1) => {
                let transformed = sqlparser::ast::CastFormat::ValueAtTimeZone(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CastKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CastKind::Cast => {
                transformer.transform(self, sqlparser::ast::CastKind::Cast)
            }
            sqlparser::ast::CastKind::TryCast => {
                transformer.transform(self, sqlparser::ast::CastKind::TryCast)
            }
            sqlparser::ast::CastKind::SafeCast => {
                transformer.transform(self, sqlparser::ast::CastKind::SafeCast)
            }
            sqlparser::ast::CastKind::DoubleColon => {
                transformer.transform(self, sqlparser::ast::CastKind::DoubleColon)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CeilFloorKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CeilFloorKind::DateTimeField(field0) => {
                let transformed = sqlparser::ast::CeilFloorKind::DateTimeField(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CeilFloorKind::Scale(field0) => {
                let transformed = sqlparser::ast::CeilFloorKind::Scale(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CharLengthUnits {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CharLengthUnits::Characters => {
                transformer.transform(self, sqlparser::ast::CharLengthUnits::Characters)
            }
            sqlparser::ast::CharLengthUnits::Octets => {
                transformer.transform(self, sqlparser::ast::CharLengthUnits::Octets)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CharacterLength {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CharacterLength::IntegerLength { length, unit } => {
                let transformed = sqlparser::ast::CharacterLength::IntegerLength {
                    length: length.apply_transform(transformer)?,
                    unit: unit.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CharacterLength::Max => {
                transformer.transform(self, sqlparser::ast::CharacterLength::Max)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CloseCursor {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CloseCursor::All => {
                transformer.transform(self, sqlparser::ast::CloseCursor::All)
            }
            sqlparser::ast::CloseCursor::Specific { name } => {
                let transformed = sqlparser::ast::CloseCursor::Specific {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ClusteredBy {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { columns, sorted_by, num_buckets } = self;
        let transformed = Self {
            columns: columns.apply_transform(transformer)?,
            sorted_by: sorted_by.apply_transform(transformer)?,
            num_buckets: num_buckets.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ClusteredIndex {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, asc } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            asc: asc.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnDef {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, data_type, collation, options } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            data_type: data_type.apply_transform(transformer)?,
            collation: collation.apply_transform(transformer)?,
            options: options.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ColumnOption::Null => {
                transformer.transform(self, sqlparser::ast::ColumnOption::Null)
            }
            sqlparser::ast::ColumnOption::NotNull => {
                transformer.transform(self, sqlparser::ast::ColumnOption::NotNull)
            }
            sqlparser::ast::ColumnOption::Default(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Default(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Materialized(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Materialized(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Ephemeral(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Ephemeral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Alias(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Alias(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Unique { is_primary, characteristics } => {
                let transformed = sqlparser::ast::ColumnOption::Unique {
                    is_primary: is_primary.apply_transform(transformer)?,
                    characteristics: characteristics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::ForeignKey {
                foreign_table,
                referred_columns,
                on_delete,
                on_update,
                characteristics,
            } => {
                let transformed = sqlparser::ast::ColumnOption::ForeignKey {
                    foreign_table: foreign_table.apply_transform(transformer)?,
                    referred_columns: referred_columns.apply_transform(transformer)?,
                    on_delete: on_delete.apply_transform(transformer)?,
                    on_update: on_update.apply_transform(transformer)?,
                    characteristics: characteristics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Check(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Check(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::DialectSpecific(field0) => {
                let transformed = sqlparser::ast::ColumnOption::DialectSpecific(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::CharacterSet(field0) => {
                let transformed = sqlparser::ast::ColumnOption::CharacterSet(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Comment(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Comment(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::OnUpdate(field0) => {
                let transformed = sqlparser::ast::ColumnOption::OnUpdate(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Generated {
                generated_as,
                sequence_options,
                generation_expr,
                generation_expr_mode,
                generated_keyword,
            } => {
                let transformed = sqlparser::ast::ColumnOption::Generated {
                    generated_as: generated_as.apply_transform(transformer)?,
                    sequence_options: sequence_options.apply_transform(transformer)?,
                    generation_expr: generation_expr.apply_transform(transformer)?,
                    generation_expr_mode: generation_expr_mode
                        .apply_transform(transformer)?,
                    generated_keyword: generated_keyword.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Options(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Options(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Identity(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Identity(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::OnConflict(field0) => {
                let transformed = sqlparser::ast::ColumnOption::OnConflict(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Policy(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Policy(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnOption::Tags(field0) => {
                let transformed = sqlparser::ast::ColumnOption::Tags(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnOptionDef {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, option } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            option: option.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnPolicy {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ColumnPolicy::MaskingPolicy(field0) => {
                let transformed = sqlparser::ast::ColumnPolicy::MaskingPolicy(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ColumnPolicy::ProjectionPolicy(field0) => {
                let transformed = sqlparser::ast::ColumnPolicy::ProjectionPolicy(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnPolicyProperty {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { with, policy_name, using_columns } = self;
        let transformed = Self {
            with: with.apply_transform(transformer)?,
            policy_name: policy_name.apply_transform(transformer)?,
            using_columns: using_columns.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CommentDef {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CommentDef::WithEq(field0) => {
                let transformed = sqlparser::ast::CommentDef::WithEq(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CommentDef::WithoutEq(field0) => {
                let transformed = sqlparser::ast::CommentDef::WithoutEq(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(field0) => {
                let transformed = sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CommentObject {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CommentObject::Column => {
                transformer.transform(self, sqlparser::ast::CommentObject::Column)
            }
            sqlparser::ast::CommentObject::Table => {
                transformer.transform(self, sqlparser::ast::CommentObject::Table)
            }
            sqlparser::ast::CommentObject::Extension => {
                transformer.transform(self, sqlparser::ast::CommentObject::Extension)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConflictTarget {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ConflictTarget::Columns(field0) => {
                let transformed = sqlparser::ast::ConflictTarget::Columns(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ConflictTarget::OnConstraint(field0) => {
                let transformed = sqlparser::ast::ConflictTarget::OnConstraint(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConnectBy {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { condition, relationships } = self;
        let transformed = Self {
            condition: condition.apply_transform(transformer)?,
            relationships: relationships.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConstraintCharacteristics {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { deferrable, initially, enforced } = self;
        let transformed = Self {
            deferrable: deferrable.apply_transform(transformer)?,
            initially: initially.apply_transform(transformer)?,
            enforced: enforced.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ContextModifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ContextModifier::None => {
                transformer.transform(self, sqlparser::ast::ContextModifier::None)
            }
            sqlparser::ast::ContextModifier::Local => {
                transformer.transform(self, sqlparser::ast::ContextModifier::Local)
            }
            sqlparser::ast::ContextModifier::Session => {
                transformer.transform(self, sqlparser::ast::ContextModifier::Session)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyLegacyCsvOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CopyLegacyCsvOption::Header => {
                transformer.transform(self, sqlparser::ast::CopyLegacyCsvOption::Header)
            }
            sqlparser::ast::CopyLegacyCsvOption::Quote(field0) => {
                let transformed = sqlparser::ast::CopyLegacyCsvOption::Quote(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyLegacyCsvOption::Escape(field0) => {
                let transformed = sqlparser::ast::CopyLegacyCsvOption::Escape(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyLegacyCsvOption::ForceQuote(field0) => {
                let transformed = sqlparser::ast::CopyLegacyCsvOption::ForceQuote(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(field0) => {
                let transformed = sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyLegacyOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CopyLegacyOption::Binary => {
                transformer.transform(self, sqlparser::ast::CopyLegacyOption::Binary)
            }
            sqlparser::ast::CopyLegacyOption::Delimiter(field0) => {
                let transformed = sqlparser::ast::CopyLegacyOption::Delimiter(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyLegacyOption::Null(field0) => {
                let transformed = sqlparser::ast::CopyLegacyOption::Null(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyLegacyOption::Csv(field0) => {
                let transformed = sqlparser::ast::CopyLegacyOption::Csv(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CopyOption::Format(field0) => {
                let transformed = sqlparser::ast::CopyOption::Format(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Freeze(field0) => {
                let transformed = sqlparser::ast::CopyOption::Freeze(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Delimiter(field0) => {
                let transformed = sqlparser::ast::CopyOption::Delimiter(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Null(field0) => {
                let transformed = sqlparser::ast::CopyOption::Null(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Header(field0) => {
                let transformed = sqlparser::ast::CopyOption::Header(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Quote(field0) => {
                let transformed = sqlparser::ast::CopyOption::Quote(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Escape(field0) => {
                let transformed = sqlparser::ast::CopyOption::Escape(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::ForceQuote(field0) => {
                let transformed = sqlparser::ast::CopyOption::ForceQuote(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::ForceNotNull(field0) => {
                let transformed = sqlparser::ast::CopyOption::ForceNotNull(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::ForceNull(field0) => {
                let transformed = sqlparser::ast::CopyOption::ForceNull(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyOption::Encoding(field0) => {
                let transformed = sqlparser::ast::CopyOption::Encoding(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopySource {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CopySource::Table { table_name, columns } => {
                let transformed = sqlparser::ast::CopySource::Table {
                    table_name: table_name.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopySource::Query(field0) => {
                let transformed = sqlparser::ast::CopySource::Query(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyTarget {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CopyTarget::Stdin => {
                transformer.transform(self, sqlparser::ast::CopyTarget::Stdin)
            }
            sqlparser::ast::CopyTarget::Stdout => {
                transformer.transform(self, sqlparser::ast::CopyTarget::Stdout)
            }
            sqlparser::ast::CopyTarget::File { filename } => {
                let transformed = sqlparser::ast::CopyTarget::File {
                    filename: filename.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CopyTarget::Program { command } => {
                let transformed = sqlparser::ast::CopyTarget::Program {
                    command: command.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateFunctionBody {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CreateFunctionBody::AsBeforeOptions(field0) => {
                let transformed = sqlparser::ast::CreateFunctionBody::AsBeforeOptions(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CreateFunctionBody::AsAfterOptions(field0) => {
                let transformed = sqlparser::ast::CreateFunctionBody::AsAfterOptions(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CreateFunctionBody::Return(field0) => {
                let transformed = sqlparser::ast::CreateFunctionBody::Return(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateFunctionUsing {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CreateFunctionUsing::Jar(field0) => {
                let transformed = sqlparser::ast::CreateFunctionUsing::Jar(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CreateFunctionUsing::File(field0) => {
                let transformed = sqlparser::ast::CreateFunctionUsing::File(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CreateFunctionUsing::Archive(field0) => {
                let transformed = sqlparser::ast::CreateFunctionUsing::Archive(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateIndex {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            name,
            table_name,
            using,
            columns,
            unique,
            concurrently,
            if_not_exists,
            include,
            nulls_distinct,
            with,
            predicate,
        } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            table_name: table_name.apply_transform(transformer)?,
            using: using.apply_transform(transformer)?,
            columns: columns.apply_transform(transformer)?,
            unique: unique.apply_transform(transformer)?,
            concurrently: concurrently.apply_transform(transformer)?,
            if_not_exists: if_not_exists.apply_transform(transformer)?,
            include: include.apply_transform(transformer)?,
            nulls_distinct: nulls_distinct.apply_transform(transformer)?,
            with: with.apply_transform(transformer)?,
            predicate: predicate.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreatePolicyCommand {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CreatePolicyCommand::All => {
                transformer.transform(self, sqlparser::ast::CreatePolicyCommand::All)
            }
            sqlparser::ast::CreatePolicyCommand::Select => {
                transformer.transform(self, sqlparser::ast::CreatePolicyCommand::Select)
            }
            sqlparser::ast::CreatePolicyCommand::Insert => {
                transformer.transform(self, sqlparser::ast::CreatePolicyCommand::Insert)
            }
            sqlparser::ast::CreatePolicyCommand::Update => {
                transformer.transform(self, sqlparser::ast::CreatePolicyCommand::Update)
            }
            sqlparser::ast::CreatePolicyCommand::Delete => {
                transformer.transform(self, sqlparser::ast::CreatePolicyCommand::Delete)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreatePolicyType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CreatePolicyType::Permissive => {
                transformer.transform(self, sqlparser::ast::CreatePolicyType::Permissive)
            }
            sqlparser::ast::CreatePolicyType::Restrictive => {
                transformer
                    .transform(self, sqlparser::ast::CreatePolicyType::Restrictive)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateTable {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            or_replace,
            temporary,
            external,
            global,
            if_not_exists,
            transient,
            volatile,
            name,
            columns,
            constraints,
            hive_distribution,
            hive_formats,
            table_properties,
            with_options,
            file_format,
            location,
            query,
            without_rowid,
            like,
            clone,
            engine,
            comment,
            auto_increment_offset,
            default_charset,
            collation,
            on_commit,
            on_cluster,
            primary_key,
            order_by,
            partition_by,
            cluster_by,
            clustered_by,
            options,
            strict,
            copy_grants,
            enable_schema_evolution,
            change_tracking,
            data_retention_time_in_days,
            max_data_extension_time_in_days,
            default_ddl_collation,
            with_aggregation_policy,
            with_row_access_policy,
            with_tags,
        } = self;
        let transformed = Self {
            or_replace: or_replace.apply_transform(transformer)?,
            temporary: temporary.apply_transform(transformer)?,
            external: external.apply_transform(transformer)?,
            global: global.apply_transform(transformer)?,
            if_not_exists: if_not_exists.apply_transform(transformer)?,
            transient: transient.apply_transform(transformer)?,
            volatile: volatile.apply_transform(transformer)?,
            name: name.apply_transform(transformer)?,
            columns: columns.apply_transform(transformer)?,
            constraints: constraints.apply_transform(transformer)?,
            hive_distribution: hive_distribution.apply_transform(transformer)?,
            hive_formats: hive_formats.apply_transform(transformer)?,
            table_properties: table_properties.apply_transform(transformer)?,
            with_options: with_options.apply_transform(transformer)?,
            file_format: file_format.apply_transform(transformer)?,
            location: location.apply_transform(transformer)?,
            query: query.apply_transform(transformer)?,
            without_rowid: without_rowid.apply_transform(transformer)?,
            like: like.apply_transform(transformer)?,
            clone: clone.apply_transform(transformer)?,
            engine: engine.apply_transform(transformer)?,
            comment: comment.apply_transform(transformer)?,
            auto_increment_offset: auto_increment_offset.apply_transform(transformer)?,
            default_charset: default_charset.apply_transform(transformer)?,
            collation: collation.apply_transform(transformer)?,
            on_commit: on_commit.apply_transform(transformer)?,
            on_cluster: on_cluster.apply_transform(transformer)?,
            primary_key: primary_key.apply_transform(transformer)?,
            order_by: order_by.apply_transform(transformer)?,
            partition_by: partition_by.apply_transform(transformer)?,
            cluster_by: cluster_by.apply_transform(transformer)?,
            clustered_by: clustered_by.apply_transform(transformer)?,
            options: options.apply_transform(transformer)?,
            strict: strict.apply_transform(transformer)?,
            copy_grants: copy_grants.apply_transform(transformer)?,
            enable_schema_evolution: enable_schema_evolution
                .apply_transform(transformer)?,
            change_tracking: change_tracking.apply_transform(transformer)?,
            data_retention_time_in_days: data_retention_time_in_days
                .apply_transform(transformer)?,
            max_data_extension_time_in_days: max_data_extension_time_in_days
                .apply_transform(transformer)?,
            default_ddl_collation: default_ddl_collation.apply_transform(transformer)?,
            with_aggregation_policy: with_aggregation_policy
                .apply_transform(transformer)?,
            with_row_access_policy: with_row_access_policy.apply_transform(transformer)?,
            with_tags: with_tags.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateTableOptions {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CreateTableOptions::None => {
                transformer.transform(self, sqlparser::ast::CreateTableOptions::None)
            }
            sqlparser::ast::CreateTableOptions::With(field0) => {
                let transformed = sqlparser::ast::CreateTableOptions::With(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::CreateTableOptions::Options(field0) => {
                let transformed = sqlparser::ast::CreateTableOptions::Options(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Cte {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { alias, query, from, materialized } = self;
        let transformed = Self {
            alias: alias.apply_transform(transformer)?,
            query: query.apply_transform(transformer)?,
            from: from.apply_transform(transformer)?,
            materialized: materialized.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CteAsMaterialized {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::CteAsMaterialized::Materialized => {
                transformer
                    .transform(self, sqlparser::ast::CteAsMaterialized::Materialized)
            }
            sqlparser::ast::CteAsMaterialized::NotMaterialized => {
                transformer
                    .transform(self, sqlparser::ast::CteAsMaterialized::NotMaterialized)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DataType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DataType::Character(field0) => {
                let transformed = sqlparser::ast::DataType::Character(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Char(field0) => {
                let transformed = sqlparser::ast::DataType::Char(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::CharacterVarying(field0) => {
                let transformed = sqlparser::ast::DataType::CharacterVarying(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::CharVarying(field0) => {
                let transformed = sqlparser::ast::DataType::CharVarying(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Varchar(field0) => {
                let transformed = sqlparser::ast::DataType::Varchar(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Nvarchar(field0) => {
                let transformed = sqlparser::ast::DataType::Nvarchar(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Uuid => {
                transformer.transform(self, sqlparser::ast::DataType::Uuid)
            }
            sqlparser::ast::DataType::CharacterLargeObject(field0) => {
                let transformed = sqlparser::ast::DataType::CharacterLargeObject(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::CharLargeObject(field0) => {
                let transformed = sqlparser::ast::DataType::CharLargeObject(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Clob(field0) => {
                let transformed = sqlparser::ast::DataType::Clob(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Binary(field0) => {
                let transformed = sqlparser::ast::DataType::Binary(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Varbinary(field0) => {
                let transformed = sqlparser::ast::DataType::Varbinary(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Blob(field0) => {
                let transformed = sqlparser::ast::DataType::Blob(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Bytes(field0) => {
                let transformed = sqlparser::ast::DataType::Bytes(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Numeric(field0) => {
                let transformed = sqlparser::ast::DataType::Numeric(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Decimal(field0) => {
                let transformed = sqlparser::ast::DataType::Decimal(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::BigNumeric(field0) => {
                let transformed = sqlparser::ast::DataType::BigNumeric(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::BigDecimal(field0) => {
                let transformed = sqlparser::ast::DataType::BigDecimal(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Dec(field0) => {
                let transformed = sqlparser::ast::DataType::Dec(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Float(field0) => {
                let transformed = sqlparser::ast::DataType::Float(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::TinyInt(field0) => {
                let transformed = sqlparser::ast::DataType::TinyInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedTinyInt(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedTinyInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Int2(field0) => {
                let transformed = sqlparser::ast::DataType::Int2(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedInt2(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedInt2(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::SmallInt(field0) => {
                let transformed = sqlparser::ast::DataType::SmallInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedSmallInt(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedSmallInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::MediumInt(field0) => {
                let transformed = sqlparser::ast::DataType::MediumInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedMediumInt(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedMediumInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Int(field0) => {
                let transformed = sqlparser::ast::DataType::Int(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Int4(field0) => {
                let transformed = sqlparser::ast::DataType::Int4(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Int8(field0) => {
                let transformed = sqlparser::ast::DataType::Int8(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Int16 => {
                transformer.transform(self, sqlparser::ast::DataType::Int16)
            }
            sqlparser::ast::DataType::Int32 => {
                transformer.transform(self, sqlparser::ast::DataType::Int32)
            }
            sqlparser::ast::DataType::Int64 => {
                transformer.transform(self, sqlparser::ast::DataType::Int64)
            }
            sqlparser::ast::DataType::Int128 => {
                transformer.transform(self, sqlparser::ast::DataType::Int128)
            }
            sqlparser::ast::DataType::Int256 => {
                transformer.transform(self, sqlparser::ast::DataType::Int256)
            }
            sqlparser::ast::DataType::Integer(field0) => {
                let transformed = sqlparser::ast::DataType::Integer(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedInt(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedInt4(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedInt4(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedInteger(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedInteger(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UInt8 => {
                transformer.transform(self, sqlparser::ast::DataType::UInt8)
            }
            sqlparser::ast::DataType::UInt16 => {
                transformer.transform(self, sqlparser::ast::DataType::UInt16)
            }
            sqlparser::ast::DataType::UInt32 => {
                transformer.transform(self, sqlparser::ast::DataType::UInt32)
            }
            sqlparser::ast::DataType::UInt64 => {
                transformer.transform(self, sqlparser::ast::DataType::UInt64)
            }
            sqlparser::ast::DataType::UInt128 => {
                transformer.transform(self, sqlparser::ast::DataType::UInt128)
            }
            sqlparser::ast::DataType::UInt256 => {
                transformer.transform(self, sqlparser::ast::DataType::UInt256)
            }
            sqlparser::ast::DataType::BigInt(field0) => {
                let transformed = sqlparser::ast::DataType::BigInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedBigInt(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedBigInt(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::UnsignedInt8(field0) => {
                let transformed = sqlparser::ast::DataType::UnsignedInt8(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Float4 => {
                transformer.transform(self, sqlparser::ast::DataType::Float4)
            }
            sqlparser::ast::DataType::Float32 => {
                transformer.transform(self, sqlparser::ast::DataType::Float32)
            }
            sqlparser::ast::DataType::Float64 => {
                transformer.transform(self, sqlparser::ast::DataType::Float64)
            }
            sqlparser::ast::DataType::Real => {
                transformer.transform(self, sqlparser::ast::DataType::Real)
            }
            sqlparser::ast::DataType::Float8 => {
                transformer.transform(self, sqlparser::ast::DataType::Float8)
            }
            sqlparser::ast::DataType::Double => {
                transformer.transform(self, sqlparser::ast::DataType::Double)
            }
            sqlparser::ast::DataType::DoublePrecision => {
                transformer.transform(self, sqlparser::ast::DataType::DoublePrecision)
            }
            sqlparser::ast::DataType::Bool => {
                transformer.transform(self, sqlparser::ast::DataType::Bool)
            }
            sqlparser::ast::DataType::Boolean => {
                transformer.transform(self, sqlparser::ast::DataType::Boolean)
            }
            sqlparser::ast::DataType::Date => {
                transformer.transform(self, sqlparser::ast::DataType::Date)
            }
            sqlparser::ast::DataType::Date32 => {
                transformer.transform(self, sqlparser::ast::DataType::Date32)
            }
            sqlparser::ast::DataType::Time(field0, field1) => {
                let transformed = sqlparser::ast::DataType::Time(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Datetime(field0) => {
                let transformed = sqlparser::ast::DataType::Datetime(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Datetime64(field0, field1) => {
                let transformed = sqlparser::ast::DataType::Datetime64(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Timestamp(field0, field1) => {
                let transformed = sqlparser::ast::DataType::Timestamp(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Interval => {
                transformer.transform(self, sqlparser::ast::DataType::Interval)
            }
            sqlparser::ast::DataType::JSON => {
                transformer.transform(self, sqlparser::ast::DataType::JSON)
            }
            sqlparser::ast::DataType::JSONB => {
                transformer.transform(self, sqlparser::ast::DataType::JSONB)
            }
            sqlparser::ast::DataType::Regclass => {
                transformer.transform(self, sqlparser::ast::DataType::Regclass)
            }
            sqlparser::ast::DataType::Text => {
                transformer.transform(self, sqlparser::ast::DataType::Text)
            }
            sqlparser::ast::DataType::String(field0) => {
                let transformed = sqlparser::ast::DataType::String(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::FixedString(field0) => {
                let transformed = sqlparser::ast::DataType::FixedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Bytea => {
                transformer.transform(self, sqlparser::ast::DataType::Bytea)
            }
            sqlparser::ast::DataType::Custom(field0, field1) => {
                let transformed = sqlparser::ast::DataType::Custom(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Array(field0) => {
                let transformed = sqlparser::ast::DataType::Array(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Map(field0, field1) => {
                let transformed = sqlparser::ast::DataType::Map(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Tuple(field0) => {
                let transformed = sqlparser::ast::DataType::Tuple(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Nested(field0) => {
                let transformed = sqlparser::ast::DataType::Nested(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Enum(field0) => {
                let transformed = sqlparser::ast::DataType::Enum(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Set(field0) => {
                let transformed = sqlparser::ast::DataType::Set(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Struct(field0, field1) => {
                let transformed = sqlparser::ast::DataType::Struct(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Union(field0) => {
                let transformed = sqlparser::ast::DataType::Union(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Nullable(field0) => {
                let transformed = sqlparser::ast::DataType::Nullable(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::LowCardinality(field0) => {
                let transformed = sqlparser::ast::DataType::LowCardinality(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DataType::Unspecified => {
                transformer.transform(self, sqlparser::ast::DataType::Unspecified)
            }
            sqlparser::ast::DataType::Trigger => {
                transformer.transform(self, sqlparser::ast::DataType::Trigger)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DateTimeField {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DateTimeField::Year => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Year)
            }
            sqlparser::ast::DateTimeField::Month => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Month)
            }
            sqlparser::ast::DateTimeField::Week(field0) => {
                let transformed = sqlparser::ast::DateTimeField::Week(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DateTimeField::Day => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Day)
            }
            sqlparser::ast::DateTimeField::DayOfWeek => {
                transformer.transform(self, sqlparser::ast::DateTimeField::DayOfWeek)
            }
            sqlparser::ast::DateTimeField::DayOfYear => {
                transformer.transform(self, sqlparser::ast::DateTimeField::DayOfYear)
            }
            sqlparser::ast::DateTimeField::Date => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Date)
            }
            sqlparser::ast::DateTimeField::Datetime => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Datetime)
            }
            sqlparser::ast::DateTimeField::Hour => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Hour)
            }
            sqlparser::ast::DateTimeField::Minute => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Minute)
            }
            sqlparser::ast::DateTimeField::Second => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Second)
            }
            sqlparser::ast::DateTimeField::Century => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Century)
            }
            sqlparser::ast::DateTimeField::Decade => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Decade)
            }
            sqlparser::ast::DateTimeField::Dow => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Dow)
            }
            sqlparser::ast::DateTimeField::Doy => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Doy)
            }
            sqlparser::ast::DateTimeField::Epoch => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Epoch)
            }
            sqlparser::ast::DateTimeField::Isodow => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Isodow)
            }
            sqlparser::ast::DateTimeField::IsoWeek => {
                transformer.transform(self, sqlparser::ast::DateTimeField::IsoWeek)
            }
            sqlparser::ast::DateTimeField::Isoyear => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Isoyear)
            }
            sqlparser::ast::DateTimeField::Julian => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Julian)
            }
            sqlparser::ast::DateTimeField::Microsecond => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Microsecond)
            }
            sqlparser::ast::DateTimeField::Microseconds => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Microseconds)
            }
            sqlparser::ast::DateTimeField::Millenium => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Millenium)
            }
            sqlparser::ast::DateTimeField::Millennium => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Millennium)
            }
            sqlparser::ast::DateTimeField::Millisecond => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Millisecond)
            }
            sqlparser::ast::DateTimeField::Milliseconds => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Milliseconds)
            }
            sqlparser::ast::DateTimeField::Nanosecond => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Nanosecond)
            }
            sqlparser::ast::DateTimeField::Nanoseconds => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Nanoseconds)
            }
            sqlparser::ast::DateTimeField::Quarter => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Quarter)
            }
            sqlparser::ast::DateTimeField::Time => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Time)
            }
            sqlparser::ast::DateTimeField::Timezone => {
                transformer.transform(self, sqlparser::ast::DateTimeField::Timezone)
            }
            sqlparser::ast::DateTimeField::TimezoneAbbr => {
                transformer.transform(self, sqlparser::ast::DateTimeField::TimezoneAbbr)
            }
            sqlparser::ast::DateTimeField::TimezoneHour => {
                transformer.transform(self, sqlparser::ast::DateTimeField::TimezoneHour)
            }
            sqlparser::ast::DateTimeField::TimezoneMinute => {
                transformer
                    .transform(self, sqlparser::ast::DateTimeField::TimezoneMinute)
            }
            sqlparser::ast::DateTimeField::TimezoneRegion => {
                transformer
                    .transform(self, sqlparser::ast::DateTimeField::TimezoneRegion)
            }
            sqlparser::ast::DateTimeField::NoDateTime => {
                transformer.transform(self, sqlparser::ast::DateTimeField::NoDateTime)
            }
            sqlparser::ast::DateTimeField::Custom(field0) => {
                let transformed = sqlparser::ast::DateTimeField::Custom(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Declare {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            names,
            data_type,
            assignment,
            declare_type,
            binary,
            sensitive,
            scroll,
            hold,
            for_query,
        } = self;
        let transformed = Self {
            names: names.apply_transform(transformer)?,
            data_type: data_type.apply_transform(transformer)?,
            assignment: assignment.apply_transform(transformer)?,
            declare_type: declare_type.apply_transform(transformer)?,
            binary: binary.apply_transform(transformer)?,
            sensitive: sensitive.apply_transform(transformer)?,
            scroll: scroll.apply_transform(transformer)?,
            hold: hold.apply_transform(transformer)?,
            for_query: for_query.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeclareAssignment {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DeclareAssignment::Expr(field0) => {
                let transformed = sqlparser::ast::DeclareAssignment::Expr(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DeclareAssignment::Default(field0) => {
                let transformed = sqlparser::ast::DeclareAssignment::Default(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DeclareAssignment::DuckAssignment(field0) => {
                let transformed = sqlparser::ast::DeclareAssignment::DuckAssignment(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DeclareAssignment::For(field0) => {
                let transformed = sqlparser::ast::DeclareAssignment::For(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::DeclareAssignment::MsSqlAssignment(field0) => {
                let transformed = sqlparser::ast::DeclareAssignment::MsSqlAssignment(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeclareType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DeclareType::Cursor => {
                transformer.transform(self, sqlparser::ast::DeclareType::Cursor)
            }
            sqlparser::ast::DeclareType::ResultSet => {
                transformer.transform(self, sqlparser::ast::DeclareType::ResultSet)
            }
            sqlparser::ast::DeclareType::Exception => {
                transformer.transform(self, sqlparser::ast::DeclareType::Exception)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Deduplicate {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Deduplicate::All => {
                transformer.transform(self, sqlparser::ast::Deduplicate::All)
            }
            sqlparser::ast::Deduplicate::ByExpression(field0) => {
                let transformed = sqlparser::ast::Deduplicate::ByExpression(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeferrableInitial {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DeferrableInitial::Immediate => {
                transformer.transform(self, sqlparser::ast::DeferrableInitial::Immediate)
            }
            sqlparser::ast::DeferrableInitial::Deferred => {
                transformer.transform(self, sqlparser::ast::DeferrableInitial::Deferred)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Delete {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { tables, from, using, selection, returning, order_by, limit } = self;
        let transformed = Self {
            tables: tables.apply_transform(transformer)?,
            from: from.apply_transform(transformer)?,
            using: using.apply_transform(transformer)?,
            selection: selection.apply_transform(transformer)?,
            returning: returning.apply_transform(transformer)?,
            order_by: order_by.apply_transform(transformer)?,
            limit: limit.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DescribeAlias {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DescribeAlias::Describe => {
                transformer.transform(self, sqlparser::ast::DescribeAlias::Describe)
            }
            sqlparser::ast::DescribeAlias::Explain => {
                transformer.transform(self, sqlparser::ast::DescribeAlias::Explain)
            }
            sqlparser::ast::DescribeAlias::Desc => {
                transformer.transform(self, sqlparser::ast::DescribeAlias::Desc)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DictionaryField {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { key, value } = self;
        let transformed = Self {
            key: key.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DiscardObject {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DiscardObject::ALL => {
                transformer.transform(self, sqlparser::ast::DiscardObject::ALL)
            }
            sqlparser::ast::DiscardObject::PLANS => {
                transformer.transform(self, sqlparser::ast::DiscardObject::PLANS)
            }
            sqlparser::ast::DiscardObject::SEQUENCES => {
                transformer.transform(self, sqlparser::ast::DiscardObject::SEQUENCES)
            }
            sqlparser::ast::DiscardObject::TEMP => {
                transformer.transform(self, sqlparser::ast::DiscardObject::TEMP)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Distinct {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Distinct::Distinct => {
                transformer.transform(self, sqlparser::ast::Distinct::Distinct)
            }
            sqlparser::ast::Distinct::On(field0) => {
                let transformed = sqlparser::ast::Distinct::On(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DoUpdate {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { assignments, selection } = self;
        let transformed = Self {
            assignments: assignments.apply_transform(transformer)?,
            selection: selection.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DollarQuotedString {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { value, tag } = self;
        let transformed = Self {
            value: value.apply_transform(transformer)?,
            tag: tag.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DuplicateTreatment {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::DuplicateTreatment::Distinct => {
                transformer.transform(self, sqlparser::ast::DuplicateTreatment::Distinct)
            }
            sqlparser::ast::DuplicateTreatment::All => {
                transformer.transform(self, sqlparser::ast::DuplicateTreatment::All)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::EmptyMatchesMode {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::EmptyMatchesMode::Show => {
                transformer.transform(self, sqlparser::ast::EmptyMatchesMode::Show)
            }
            sqlparser::ast::EmptyMatchesMode::Omit => {
                transformer.transform(self, sqlparser::ast::EmptyMatchesMode::Omit)
            }
            sqlparser::ast::EmptyMatchesMode::WithUnmatched => {
                transformer
                    .transform(self, sqlparser::ast::EmptyMatchesMode::WithUnmatched)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExactNumberInfo {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ExactNumberInfo::None => {
                transformer.transform(self, sqlparser::ast::ExactNumberInfo::None)
            }
            sqlparser::ast::ExactNumberInfo::Precision(field0) => {
                let transformed = sqlparser::ast::ExactNumberInfo::Precision(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ExactNumberInfo::PrecisionAndScale(field0, field1) => {
                let transformed = sqlparser::ast::ExactNumberInfo::PrecisionAndScale(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExceptSelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { first_element, additional_elements } = self;
        let transformed = Self {
            first_element: first_element.apply_transform(transformer)?,
            additional_elements: additional_elements.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExcludeSelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ExcludeSelectItem::Single(field0) => {
                let transformed = sqlparser::ast::ExcludeSelectItem::Single(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ExcludeSelectItem::Multiple(field0) => {
                let transformed = sqlparser::ast::ExcludeSelectItem::Multiple(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Expr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Expr::Identifier(field0) => {
                let transformed = sqlparser::ast::Expr::Identifier(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::CompoundIdentifier(field0) => {
                let transformed = sqlparser::ast::Expr::CompoundIdentifier(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::JsonAccess { value, path } => {
                let transformed = sqlparser::ast::Expr::JsonAccess {
                    value: value.apply_transform(transformer)?,
                    path: path.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::CompositeAccess { expr, key } => {
                let transformed = sqlparser::ast::Expr::CompositeAccess {
                    expr: expr.apply_transform(transformer)?,
                    key: key.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsFalse(field0) => {
                let transformed = sqlparser::ast::Expr::IsFalse(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsNotFalse(field0) => {
                let transformed = sqlparser::ast::Expr::IsNotFalse(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsTrue(field0) => {
                let transformed = sqlparser::ast::Expr::IsTrue(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsNotTrue(field0) => {
                let transformed = sqlparser::ast::Expr::IsNotTrue(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsNull(field0) => {
                let transformed = sqlparser::ast::Expr::IsNull(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsNotNull(field0) => {
                let transformed = sqlparser::ast::Expr::IsNotNull(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsUnknown(field0) => {
                let transformed = sqlparser::ast::Expr::IsUnknown(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsNotUnknown(field0) => {
                let transformed = sqlparser::ast::Expr::IsNotUnknown(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsDistinctFrom(field0, field1) => {
                let transformed = sqlparser::ast::Expr::IsDistinctFrom(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IsNotDistinctFrom(field0, field1) => {
                let transformed = sqlparser::ast::Expr::IsNotDistinctFrom(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::InList { expr, list, negated } => {
                let transformed = sqlparser::ast::Expr::InList {
                    expr: expr.apply_transform(transformer)?,
                    list: list.apply_transform(transformer)?,
                    negated: negated.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::InSubquery { expr, subquery, negated } => {
                let transformed = sqlparser::ast::Expr::InSubquery {
                    expr: expr.apply_transform(transformer)?,
                    subquery: subquery.apply_transform(transformer)?,
                    negated: negated.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::InUnnest { expr, array_expr, negated } => {
                let transformed = sqlparser::ast::Expr::InUnnest {
                    expr: expr.apply_transform(transformer)?,
                    array_expr: array_expr.apply_transform(transformer)?,
                    negated: negated.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Between { expr, negated, low, high } => {
                let transformed = sqlparser::ast::Expr::Between {
                    expr: expr.apply_transform(transformer)?,
                    negated: negated.apply_transform(transformer)?,
                    low: low.apply_transform(transformer)?,
                    high: high.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::BinaryOp { left, op, right } => {
                let transformed = sqlparser::ast::Expr::BinaryOp {
                    left: left.apply_transform(transformer)?,
                    op: op.apply_transform(transformer)?,
                    right: right.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Like { negated, any, expr, pattern, escape_char } => {
                let transformed = sqlparser::ast::Expr::Like {
                    negated: negated.apply_transform(transformer)?,
                    any: any.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                    pattern: pattern.apply_transform(transformer)?,
                    escape_char: escape_char.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::ILike { negated, any, expr, pattern, escape_char } => {
                let transformed = sqlparser::ast::Expr::ILike {
                    negated: negated.apply_transform(transformer)?,
                    any: any.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                    pattern: pattern.apply_transform(transformer)?,
                    escape_char: escape_char.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::SimilarTo { negated, expr, pattern, escape_char } => {
                let transformed = sqlparser::ast::Expr::SimilarTo {
                    negated: negated.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                    pattern: pattern.apply_transform(transformer)?,
                    escape_char: escape_char.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::RLike { negated, expr, pattern, regexp } => {
                let transformed = sqlparser::ast::Expr::RLike {
                    negated: negated.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                    pattern: pattern.apply_transform(transformer)?,
                    regexp: regexp.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::AnyOp { left, compare_op, right, is_some } => {
                let transformed = sqlparser::ast::Expr::AnyOp {
                    left: left.apply_transform(transformer)?,
                    compare_op: compare_op.apply_transform(transformer)?,
                    right: right.apply_transform(transformer)?,
                    is_some: is_some.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::AllOp { left, compare_op, right } => {
                let transformed = sqlparser::ast::Expr::AllOp {
                    left: left.apply_transform(transformer)?,
                    compare_op: compare_op.apply_transform(transformer)?,
                    right: right.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::UnaryOp { op, expr } => {
                let transformed = sqlparser::ast::Expr::UnaryOp {
                    op: op.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Convert {
                is_try,
                expr,
                data_type,
                charset,
                target_before_value,
                styles,
            } => {
                let transformed = sqlparser::ast::Expr::Convert {
                    is_try: is_try.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                    data_type: data_type.apply_transform(transformer)?,
                    charset: charset.apply_transform(transformer)?,
                    target_before_value: target_before_value
                        .apply_transform(transformer)?,
                    styles: styles.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Cast { kind, expr, data_type, format } => {
                let transformed = sqlparser::ast::Expr::Cast {
                    kind: kind.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                    data_type: data_type.apply_transform(transformer)?,
                    format: format.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::AtTimeZone { timestamp, time_zone } => {
                let transformed = sqlparser::ast::Expr::AtTimeZone {
                    timestamp: timestamp.apply_transform(transformer)?,
                    time_zone: time_zone.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Extract { field, syntax, expr } => {
                let transformed = sqlparser::ast::Expr::Extract {
                    field: field.apply_transform(transformer)?,
                    syntax: syntax.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Ceil { expr, field } => {
                let transformed = sqlparser::ast::Expr::Ceil {
                    expr: expr.apply_transform(transformer)?,
                    field: field.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Floor { expr, field } => {
                let transformed = sqlparser::ast::Expr::Floor {
                    expr: expr.apply_transform(transformer)?,
                    field: field.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Position { expr, r#in } => {
                let transformed = sqlparser::ast::Expr::Position {
                    expr: expr.apply_transform(transformer)?,
                    r#in: r#in.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Substring {
                expr,
                substring_from,
                substring_for,
                special,
            } => {
                let transformed = sqlparser::ast::Expr::Substring {
                    expr: expr.apply_transform(transformer)?,
                    substring_from: substring_from.apply_transform(transformer)?,
                    substring_for: substring_for.apply_transform(transformer)?,
                    special: special.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Trim {
                expr,
                trim_where,
                trim_what,
                trim_characters,
            } => {
                let transformed = sqlparser::ast::Expr::Trim {
                    expr: expr.apply_transform(transformer)?,
                    trim_where: trim_where.apply_transform(transformer)?,
                    trim_what: trim_what.apply_transform(transformer)?,
                    trim_characters: trim_characters.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Overlay {
                expr,
                overlay_what,
                overlay_from,
                overlay_for,
            } => {
                let transformed = sqlparser::ast::Expr::Overlay {
                    expr: expr.apply_transform(transformer)?,
                    overlay_what: overlay_what.apply_transform(transformer)?,
                    overlay_from: overlay_from.apply_transform(transformer)?,
                    overlay_for: overlay_for.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Collate { expr, collation } => {
                let transformed = sqlparser::ast::Expr::Collate {
                    expr: expr.apply_transform(transformer)?,
                    collation: collation.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Nested(field0) => {
                let transformed = sqlparser::ast::Expr::Nested(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Value(field0) => {
                let transformed = sqlparser::ast::Expr::Value(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::IntroducedString { introducer, value } => {
                let transformed = sqlparser::ast::Expr::IntroducedString {
                    introducer: introducer.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::TypedString { data_type, value } => {
                let transformed = sqlparser::ast::Expr::TypedString {
                    data_type: data_type.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::MapAccess { column, keys } => {
                let transformed = sqlparser::ast::Expr::MapAccess {
                    column: column.apply_transform(transformer)?,
                    keys: keys.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Function(field0) => {
                let transformed = sqlparser::ast::Expr::Function(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Case { operand, conditions, results, else_result } => {
                let transformed = sqlparser::ast::Expr::Case {
                    operand: operand.apply_transform(transformer)?,
                    conditions: conditions.apply_transform(transformer)?,
                    results: results.apply_transform(transformer)?,
                    else_result: else_result.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Exists { subquery, negated } => {
                let transformed = sqlparser::ast::Expr::Exists {
                    subquery: subquery.apply_transform(transformer)?,
                    negated: negated.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Subquery(field0) => {
                let transformed = sqlparser::ast::Expr::Subquery(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::GroupingSets(field0) => {
                let transformed = sqlparser::ast::Expr::GroupingSets(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Cube(field0) => {
                let transformed = sqlparser::ast::Expr::Cube(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Rollup(field0) => {
                let transformed = sqlparser::ast::Expr::Rollup(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Tuple(field0) => {
                let transformed = sqlparser::ast::Expr::Tuple(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Struct { values, fields } => {
                let transformed = sqlparser::ast::Expr::Struct {
                    values: values.apply_transform(transformer)?,
                    fields: fields.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Named { expr, name } => {
                let transformed = sqlparser::ast::Expr::Named {
                    expr: expr.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Dictionary(field0) => {
                let transformed = sqlparser::ast::Expr::Dictionary(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Map(field0) => {
                let transformed = sqlparser::ast::Expr::Map(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Subscript { expr, subscript } => {
                let transformed = sqlparser::ast::Expr::Subscript {
                    expr: expr.apply_transform(transformer)?,
                    subscript: subscript.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Array(field0) => {
                let transformed = sqlparser::ast::Expr::Array(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Interval(field0) => {
                let transformed = sqlparser::ast::Expr::Interval(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::MatchAgainst {
                columns,
                match_value,
                opt_search_modifier,
            } => {
                let transformed = sqlparser::ast::Expr::MatchAgainst {
                    columns: columns.apply_transform(transformer)?,
                    match_value: match_value.apply_transform(transformer)?,
                    opt_search_modifier: opt_search_modifier
                        .apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Wildcard => {
                transformer.transform(self, sqlparser::ast::Expr::Wildcard)
            }
            sqlparser::ast::Expr::QualifiedWildcard(field0) => {
                let transformed = sqlparser::ast::Expr::QualifiedWildcard(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::OuterJoin(field0) => {
                let transformed = sqlparser::ast::Expr::OuterJoin(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Prior(field0) => {
                let transformed = sqlparser::ast::Expr::Prior(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Expr::Lambda(field0) => {
                let transformed = sqlparser::ast::Expr::Lambda(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExprWithAlias {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { expr, alias } = self;
        let transformed = Self {
            expr: expr.apply_transform(transformer)?,
            alias: alias.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExtractSyntax {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ExtractSyntax::From => {
                transformer.transform(self, sqlparser::ast::ExtractSyntax::From)
            }
            sqlparser::ast::ExtractSyntax::Comma => {
                transformer.transform(self, sqlparser::ast::ExtractSyntax::Comma)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Fetch {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { with_ties, percent, quantity } = self;
        let transformed = Self {
            with_ties: with_ties.apply_transform(transformer)?,
            percent: percent.apply_transform(transformer)?,
            quantity: quantity.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FetchDirection {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FetchDirection::Count { limit } => {
                let transformed = sqlparser::ast::FetchDirection::Count {
                    limit: limit.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FetchDirection::Next => {
                transformer.transform(self, sqlparser::ast::FetchDirection::Next)
            }
            sqlparser::ast::FetchDirection::Prior => {
                transformer.transform(self, sqlparser::ast::FetchDirection::Prior)
            }
            sqlparser::ast::FetchDirection::First => {
                transformer.transform(self, sqlparser::ast::FetchDirection::First)
            }
            sqlparser::ast::FetchDirection::Last => {
                transformer.transform(self, sqlparser::ast::FetchDirection::Last)
            }
            sqlparser::ast::FetchDirection::Absolute { limit } => {
                let transformed = sqlparser::ast::FetchDirection::Absolute {
                    limit: limit.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FetchDirection::Relative { limit } => {
                let transformed = sqlparser::ast::FetchDirection::Relative {
                    limit: limit.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FetchDirection::All => {
                transformer.transform(self, sqlparser::ast::FetchDirection::All)
            }
            sqlparser::ast::FetchDirection::Forward { limit } => {
                let transformed = sqlparser::ast::FetchDirection::Forward {
                    limit: limit.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FetchDirection::ForwardAll => {
                transformer.transform(self, sqlparser::ast::FetchDirection::ForwardAll)
            }
            sqlparser::ast::FetchDirection::Backward { limit } => {
                let transformed = sqlparser::ast::FetchDirection::Backward {
                    limit: limit.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FetchDirection::BackwardAll => {
                transformer.transform(self, sqlparser::ast::FetchDirection::BackwardAll)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FileFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FileFormat::TEXTFILE => {
                transformer.transform(self, sqlparser::ast::FileFormat::TEXTFILE)
            }
            sqlparser::ast::FileFormat::SEQUENCEFILE => {
                transformer.transform(self, sqlparser::ast::FileFormat::SEQUENCEFILE)
            }
            sqlparser::ast::FileFormat::ORC => {
                transformer.transform(self, sqlparser::ast::FileFormat::ORC)
            }
            sqlparser::ast::FileFormat::PARQUET => {
                transformer.transform(self, sqlparser::ast::FileFormat::PARQUET)
            }
            sqlparser::ast::FileFormat::AVRO => {
                transformer.transform(self, sqlparser::ast::FileFormat::AVRO)
            }
            sqlparser::ast::FileFormat::RCFILE => {
                transformer.transform(self, sqlparser::ast::FileFormat::RCFILE)
            }
            sqlparser::ast::FileFormat::JSONFILE => {
                transformer.transform(self, sqlparser::ast::FileFormat::JSONFILE)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FlushLocation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FlushLocation::NoWriteToBinlog => {
                transformer
                    .transform(self, sqlparser::ast::FlushLocation::NoWriteToBinlog)
            }
            sqlparser::ast::FlushLocation::Local => {
                transformer.transform(self, sqlparser::ast::FlushLocation::Local)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FlushType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FlushType::BinaryLogs => {
                transformer.transform(self, sqlparser::ast::FlushType::BinaryLogs)
            }
            sqlparser::ast::FlushType::EngineLogs => {
                transformer.transform(self, sqlparser::ast::FlushType::EngineLogs)
            }
            sqlparser::ast::FlushType::ErrorLogs => {
                transformer.transform(self, sqlparser::ast::FlushType::ErrorLogs)
            }
            sqlparser::ast::FlushType::GeneralLogs => {
                transformer.transform(self, sqlparser::ast::FlushType::GeneralLogs)
            }
            sqlparser::ast::FlushType::Hosts => {
                transformer.transform(self, sqlparser::ast::FlushType::Hosts)
            }
            sqlparser::ast::FlushType::Logs => {
                transformer.transform(self, sqlparser::ast::FlushType::Logs)
            }
            sqlparser::ast::FlushType::Privileges => {
                transformer.transform(self, sqlparser::ast::FlushType::Privileges)
            }
            sqlparser::ast::FlushType::OptimizerCosts => {
                transformer.transform(self, sqlparser::ast::FlushType::OptimizerCosts)
            }
            sqlparser::ast::FlushType::RelayLogs => {
                transformer.transform(self, sqlparser::ast::FlushType::RelayLogs)
            }
            sqlparser::ast::FlushType::SlowLogs => {
                transformer.transform(self, sqlparser::ast::FlushType::SlowLogs)
            }
            sqlparser::ast::FlushType::Status => {
                transformer.transform(self, sqlparser::ast::FlushType::Status)
            }
            sqlparser::ast::FlushType::UserResources => {
                transformer.transform(self, sqlparser::ast::FlushType::UserResources)
            }
            sqlparser::ast::FlushType::Tables => {
                transformer.transform(self, sqlparser::ast::FlushType::Tables)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForClause {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ForClause::Browse => {
                transformer.transform(self, sqlparser::ast::ForClause::Browse)
            }
            sqlparser::ast::ForClause::Json {
                for_json,
                root,
                include_null_values,
                without_array_wrapper,
            } => {
                let transformed = sqlparser::ast::ForClause::Json {
                    for_json: for_json.apply_transform(transformer)?,
                    root: root.apply_transform(transformer)?,
                    include_null_values: include_null_values
                        .apply_transform(transformer)?,
                    without_array_wrapper: without_array_wrapper
                        .apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ForClause::Xml {
                for_xml,
                elements,
                binary_base64,
                root,
                r#type,
            } => {
                let transformed = sqlparser::ast::ForClause::Xml {
                    for_xml: for_xml.apply_transform(transformer)?,
                    elements: elements.apply_transform(transformer)?,
                    binary_base64: binary_base64.apply_transform(transformer)?,
                    root: root.apply_transform(transformer)?,
                    r#type: r#type.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForJson {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ForJson::Auto => {
                transformer.transform(self, sqlparser::ast::ForJson::Auto)
            }
            sqlparser::ast::ForJson::Path => {
                transformer.transform(self, sqlparser::ast::ForJson::Path)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForXml {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ForXml::Raw(field0) => {
                let transformed = sqlparser::ast::ForXml::Raw(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ForXml::Auto => {
                transformer.transform(self, sqlparser::ast::ForXml::Auto)
            }
            sqlparser::ast::ForXml::Explicit => {
                transformer.transform(self, sqlparser::ast::ForXml::Explicit)
            }
            sqlparser::ast::ForXml::Path(field0) => {
                let transformed = sqlparser::ast::ForXml::Path(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FormatClause {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FormatClause::Identifier(field0) => {
                let transformed = sqlparser::ast::FormatClause::Identifier(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FormatClause::Null => {
                transformer.transform(self, sqlparser::ast::FormatClause::Null)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FromTable {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FromTable::WithFromKeyword(field0) => {
                let transformed = sqlparser::ast::FromTable::WithFromKeyword(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FromTable::WithoutKeyword(field0) => {
                let transformed = sqlparser::ast::FromTable::WithoutKeyword(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Function {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            name,
            parameters,
            args,
            filter,
            null_treatment,
            over,
            within_group,
        } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            parameters: parameters.apply_transform(transformer)?,
            args: args.apply_transform(transformer)?,
            filter: filter.apply_transform(transformer)?,
            null_treatment: null_treatment.apply_transform(transformer)?,
            over: over.apply_transform(transformer)?,
            within_group: within_group.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArg {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionArg::Named { name, arg, operator } => {
                let transformed = sqlparser::ast::FunctionArg::Named {
                    name: name.apply_transform(transformer)?,
                    arg: arg.apply_transform(transformer)?,
                    operator: operator.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArg::Unnamed(field0) => {
                let transformed = sqlparser::ast::FunctionArg::Unnamed(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionArgExpr::Expr(field0) => {
                let transformed = sqlparser::ast::FunctionArgExpr::Expr(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgExpr::QualifiedWildcard(field0) => {
                let transformed = sqlparser::ast::FunctionArgExpr::QualifiedWildcard(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgExpr::Wildcard => {
                transformer.transform(self, sqlparser::ast::FunctionArgExpr::Wildcard)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgOperator {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionArgOperator::Equals => {
                transformer.transform(self, sqlparser::ast::FunctionArgOperator::Equals)
            }
            sqlparser::ast::FunctionArgOperator::RightArrow => {
                transformer
                    .transform(self, sqlparser::ast::FunctionArgOperator::RightArrow)
            }
            sqlparser::ast::FunctionArgOperator::Assignment => {
                transformer
                    .transform(self, sqlparser::ast::FunctionArgOperator::Assignment)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgumentClause {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(field0) => {
                let transformed = sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgumentClause::OrderBy(field0) => {
                let transformed = sqlparser::ast::FunctionArgumentClause::OrderBy(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgumentClause::Limit(field0) => {
                let transformed = sqlparser::ast::FunctionArgumentClause::Limit(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgumentClause::OnOverflow(field0) => {
                let transformed = sqlparser::ast::FunctionArgumentClause::OnOverflow(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgumentClause::Having(field0) => {
                let transformed = sqlparser::ast::FunctionArgumentClause::Having(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArgumentClause::Separator(field0) => {
                let transformed = sqlparser::ast::FunctionArgumentClause::Separator(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgumentList {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { duplicate_treatment, args, clauses } = self;
        let transformed = Self {
            duplicate_treatment: duplicate_treatment.apply_transform(transformer)?,
            args: args.apply_transform(transformer)?,
            clauses: clauses.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArguments {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionArguments::None => {
                transformer.transform(self, sqlparser::ast::FunctionArguments::None)
            }
            sqlparser::ast::FunctionArguments::Subquery(field0) => {
                let transformed = sqlparser::ast::FunctionArguments::Subquery(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::FunctionArguments::List(field0) => {
                let transformed = sqlparser::ast::FunctionArguments::List(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionBehavior {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionBehavior::Immutable => {
                transformer.transform(self, sqlparser::ast::FunctionBehavior::Immutable)
            }
            sqlparser::ast::FunctionBehavior::Stable => {
                transformer.transform(self, sqlparser::ast::FunctionBehavior::Stable)
            }
            sqlparser::ast::FunctionBehavior::Volatile => {
                transformer.transform(self, sqlparser::ast::FunctionBehavior::Volatile)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionCalledOnNull {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput,
                    )
            }
            sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput,
                    )
            }
            sqlparser::ast::FunctionCalledOnNull::Strict => {
                transformer.transform(self, sqlparser::ast::FunctionCalledOnNull::Strict)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionDesc {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, args } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            args: args.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionDeterminismSpecifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionDeterminismSpecifier::Deterministic => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::FunctionDeterminismSpecifier::Deterministic,
                    )
            }
            sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic,
                    )
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionParallel {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::FunctionParallel::Unsafe => {
                transformer.transform(self, sqlparser::ast::FunctionParallel::Unsafe)
            }
            sqlparser::ast::FunctionParallel::Restricted => {
                transformer.transform(self, sqlparser::ast::FunctionParallel::Restricted)
            }
            sqlparser::ast::FunctionParallel::Safe => {
                transformer.transform(self, sqlparser::ast::FunctionParallel::Safe)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GeneratedAs {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::GeneratedAs::Always => {
                transformer.transform(self, sqlparser::ast::GeneratedAs::Always)
            }
            sqlparser::ast::GeneratedAs::ByDefault => {
                transformer.transform(self, sqlparser::ast::GeneratedAs::ByDefault)
            }
            sqlparser::ast::GeneratedAs::ExpStored => {
                transformer.transform(self, sqlparser::ast::GeneratedAs::ExpStored)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GeneratedExpressionMode {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::GeneratedExpressionMode::Virtual => {
                transformer
                    .transform(self, sqlparser::ast::GeneratedExpressionMode::Virtual)
            }
            sqlparser::ast::GeneratedExpressionMode::Stored => {
                transformer
                    .transform(self, sqlparser::ast::GeneratedExpressionMode::Stored)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GrantObjects {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::GrantObjects::AllSequencesInSchema { schemas } => {
                let transformed = sqlparser::ast::GrantObjects::AllSequencesInSchema {
                    schemas: schemas.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::GrantObjects::AllTablesInSchema { schemas } => {
                let transformed = sqlparser::ast::GrantObjects::AllTablesInSchema {
                    schemas: schemas.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::GrantObjects::Schemas(field0) => {
                let transformed = sqlparser::ast::GrantObjects::Schemas(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::GrantObjects::Sequences(field0) => {
                let transformed = sqlparser::ast::GrantObjects::Sequences(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::GrantObjects::Tables(field0) => {
                let transformed = sqlparser::ast::GrantObjects::Tables(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GroupByExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::GroupByExpr::All(field0) => {
                let transformed = sqlparser::ast::GroupByExpr::All(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::GroupByExpr::Expressions(field0, field1) => {
                let transformed = sqlparser::ast::GroupByExpr::Expressions(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GroupByWithModifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::GroupByWithModifier::Rollup => {
                transformer.transform(self, sqlparser::ast::GroupByWithModifier::Rollup)
            }
            sqlparser::ast::GroupByWithModifier::Cube => {
                transformer.transform(self, sqlparser::ast::GroupByWithModifier::Cube)
            }
            sqlparser::ast::GroupByWithModifier::Totals => {
                transformer.transform(self, sqlparser::ast::GroupByWithModifier::Totals)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HavingBound {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self(field0, field1) = self;
        let transformed = Self(
            field0.apply_transform(transformer)?,
            field1.apply_transform(transformer)?,
        );
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HavingBoundKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::HavingBoundKind::Min => {
                transformer.transform(self, sqlparser::ast::HavingBoundKind::Min)
            }
            sqlparser::ast::HavingBoundKind::Max => {
                transformer.transform(self, sqlparser::ast::HavingBoundKind::Max)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDelimiter {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::HiveDelimiter::FieldsTerminatedBy => {
                transformer
                    .transform(self, sqlparser::ast::HiveDelimiter::FieldsTerminatedBy)
            }
            sqlparser::ast::HiveDelimiter::FieldsEscapedBy => {
                transformer
                    .transform(self, sqlparser::ast::HiveDelimiter::FieldsEscapedBy)
            }
            sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy,
                    )
            }
            sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy => {
                transformer
                    .transform(self, sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy)
            }
            sqlparser::ast::HiveDelimiter::LinesTerminatedBy => {
                transformer
                    .transform(self, sqlparser::ast::HiveDelimiter::LinesTerminatedBy)
            }
            sqlparser::ast::HiveDelimiter::NullDefinedAs => {
                transformer.transform(self, sqlparser::ast::HiveDelimiter::NullDefinedAs)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDescribeFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::HiveDescribeFormat::Extended => {
                transformer.transform(self, sqlparser::ast::HiveDescribeFormat::Extended)
            }
            sqlparser::ast::HiveDescribeFormat::Formatted => {
                transformer
                    .transform(self, sqlparser::ast::HiveDescribeFormat::Formatted)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDistributionStyle {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::HiveDistributionStyle::PARTITIONED { columns } => {
                let transformed = sqlparser::ast::HiveDistributionStyle::PARTITIONED {
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::HiveDistributionStyle::SKEWED {
                columns,
                on,
                stored_as_directories,
            } => {
                let transformed = sqlparser::ast::HiveDistributionStyle::SKEWED {
                    columns: columns.apply_transform(transformer)?,
                    on: on.apply_transform(transformer)?,
                    stored_as_directories: stored_as_directories
                        .apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::HiveDistributionStyle::NONE => {
                transformer.transform(self, sqlparser::ast::HiveDistributionStyle::NONE)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { row_format, serde_properties, storage, location } = self;
        let transformed = Self {
            row_format: row_format.apply_transform(transformer)?,
            serde_properties: serde_properties.apply_transform(transformer)?,
            storage: storage.apply_transform(transformer)?,
            location: location.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveIOFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::HiveIOFormat::IOF { input_format, output_format } => {
                let transformed = sqlparser::ast::HiveIOFormat::IOF {
                    input_format: input_format.apply_transform(transformer)?,
                    output_format: output_format.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::HiveIOFormat::FileFormat { format } => {
                let transformed = sqlparser::ast::HiveIOFormat::FileFormat {
                    format: format.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveRowDelimiter {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { delimiter, char } = self;
        let transformed = Self {
            delimiter: delimiter.apply_transform(transformer)?,
            char: char.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveRowFormat {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::HiveRowFormat::SERDE { class } => {
                let transformed = sqlparser::ast::HiveRowFormat::SERDE {
                    class: class.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::HiveRowFormat::DELIMITED { delimiters } => {
                let transformed = sqlparser::ast::HiveRowFormat::DELIMITED {
                    delimiters: delimiters.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveSetLocation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { has_set, location } = self;
        let transformed = Self {
            has_set: has_set.apply_transform(transformer)?,
            location: location.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Ident {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { value, quote_style } = self;
        let transformed = Self {
            value: value.apply_transform(transformer)?,
            quote_style: quote_style.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentWithAlias {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { ident, alias } = self;
        let transformed = Self {
            ident: ident.apply_transform(transformer)?,
            alias: alias.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityParameters {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { seed, increment } = self;
        let transformed = Self {
            seed: seed.apply_transform(transformer)?,
            increment: increment.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityProperty {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { parameters, order } = self;
        let transformed = Self {
            parameters: parameters.apply_transform(transformer)?,
            order: order.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyFormatKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(field0) => {
                let transformed = sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(field0) => {
                let transformed = sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::IdentityPropertyKind::Autoincrement(field0) => {
                let transformed = sqlparser::ast::IdentityPropertyKind::Autoincrement(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::IdentityPropertyKind::Identity(field0) => {
                let transformed = sqlparser::ast::IdentityPropertyKind::Identity(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyOrder {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::IdentityPropertyOrder::Order => {
                transformer.transform(self, sqlparser::ast::IdentityPropertyOrder::Order)
            }
            sqlparser::ast::IdentityPropertyOrder::NoOrder => {
                transformer
                    .transform(self, sqlparser::ast::IdentityPropertyOrder::NoOrder)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IlikeSelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { pattern } = self;
        let transformed = Self {
            pattern: pattern.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IndexOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::IndexOption::Using(field0) => {
                let transformed = sqlparser::ast::IndexOption::Using(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::IndexOption::Comment(field0) => {
                let transformed = sqlparser::ast::IndexOption::Comment(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IndexType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::IndexType::BTree => {
                transformer.transform(self, sqlparser::ast::IndexType::BTree)
            }
            sqlparser::ast::IndexType::Hash => {
                transformer.transform(self, sqlparser::ast::IndexType::Hash)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Insert {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            or,
            ignore,
            into,
            table_name,
            table_alias,
            columns,
            overwrite,
            source,
            partitioned,
            after_columns,
            table,
            on,
            returning,
            replace_into,
            priority,
            insert_alias,
        } = self;
        let transformed = Self {
            or: or.apply_transform(transformer)?,
            ignore: ignore.apply_transform(transformer)?,
            into: into.apply_transform(transformer)?,
            table_name: table_name.apply_transform(transformer)?,
            table_alias: table_alias.apply_transform(transformer)?,
            columns: columns.apply_transform(transformer)?,
            overwrite: overwrite.apply_transform(transformer)?,
            source: source.apply_transform(transformer)?,
            partitioned: partitioned.apply_transform(transformer)?,
            after_columns: after_columns.apply_transform(transformer)?,
            table: table.apply_transform(transformer)?,
            on: on.apply_transform(transformer)?,
            returning: returning.apply_transform(transformer)?,
            replace_into: replace_into.apply_transform(transformer)?,
            priority: priority.apply_transform(transformer)?,
            insert_alias: insert_alias.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::InsertAliases {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { row_alias, col_aliases } = self;
        let transformed = Self {
            row_alias: row_alias.apply_transform(transformer)?,
            col_aliases: col_aliases.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Interpolate {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { exprs } = self;
        let transformed = Self {
            exprs: exprs.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::InterpolateExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { column, expr } = self;
        let transformed = Self {
            column: column.apply_transform(transformer)?,
            expr: expr.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Interval {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            value,
            leading_field,
            leading_precision,
            last_field,
            fractional_seconds_precision,
        } = self;
        let transformed = Self {
            value: value.apply_transform(transformer)?,
            leading_field: leading_field.apply_transform(transformer)?,
            leading_precision: leading_precision.apply_transform(transformer)?,
            last_field: last_field.apply_transform(transformer)?,
            fractional_seconds_precision: fractional_seconds_precision
                .apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Join {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { relation, global, join_operator } = self;
        let transformed = Self {
            relation: relation.apply_transform(transformer)?,
            global: global.apply_transform(transformer)?,
            join_operator: join_operator.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JoinConstraint {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::JoinConstraint::On(field0) => {
                let transformed = sqlparser::ast::JoinConstraint::On(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinConstraint::Using(field0) => {
                let transformed = sqlparser::ast::JoinConstraint::Using(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinConstraint::Natural => {
                transformer.transform(self, sqlparser::ast::JoinConstraint::Natural)
            }
            sqlparser::ast::JoinConstraint::None => {
                transformer.transform(self, sqlparser::ast::JoinConstraint::None)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JoinOperator {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::JoinOperator::Inner(field0) => {
                let transformed = sqlparser::ast::JoinOperator::Inner(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::LeftOuter(field0) => {
                let transformed = sqlparser::ast::JoinOperator::LeftOuter(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::RightOuter(field0) => {
                let transformed = sqlparser::ast::JoinOperator::RightOuter(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::FullOuter(field0) => {
                let transformed = sqlparser::ast::JoinOperator::FullOuter(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::CrossJoin => {
                transformer.transform(self, sqlparser::ast::JoinOperator::CrossJoin)
            }
            sqlparser::ast::JoinOperator::LeftSemi(field0) => {
                let transformed = sqlparser::ast::JoinOperator::LeftSemi(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::RightSemi(field0) => {
                let transformed = sqlparser::ast::JoinOperator::RightSemi(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::LeftAnti(field0) => {
                let transformed = sqlparser::ast::JoinOperator::LeftAnti(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::RightAnti(field0) => {
                let transformed = sqlparser::ast::JoinOperator::RightAnti(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JoinOperator::CrossApply => {
                transformer.transform(self, sqlparser::ast::JoinOperator::CrossApply)
            }
            sqlparser::ast::JoinOperator::OuterApply => {
                transformer.transform(self, sqlparser::ast::JoinOperator::OuterApply)
            }
            sqlparser::ast::JoinOperator::AsOf { match_condition, constraint } => {
                let transformed = sqlparser::ast::JoinOperator::AsOf {
                    match_condition: match_condition.apply_transform(transformer)?,
                    constraint: constraint.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonPath {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { path } = self;
        let transformed = Self {
            path: path.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonPathElem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::JsonPathElem::Dot { key, quoted } => {
                let transformed = sqlparser::ast::JsonPathElem::Dot {
                    key: key.apply_transform(transformer)?,
                    quoted: quoted.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JsonPathElem::Bracket { key } => {
                let transformed = sqlparser::ast::JsonPathElem::Bracket {
                    key: key.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableColumn {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::JsonTableColumn::Named(field0) => {
                let transformed = sqlparser::ast::JsonTableColumn::Named(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JsonTableColumn::ForOrdinality(field0) => {
                let transformed = sqlparser::ast::JsonTableColumn::ForOrdinality(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JsonTableColumn::Nested(field0) => {
                let transformed = sqlparser::ast::JsonTableColumn::Nested(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableColumnErrorHandling {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::JsonTableColumnErrorHandling::Null => {
                transformer
                    .transform(self, sqlparser::ast::JsonTableColumnErrorHandling::Null)
            }
            sqlparser::ast::JsonTableColumnErrorHandling::Default(field0) => {
                let transformed = sqlparser::ast::JsonTableColumnErrorHandling::Default(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::JsonTableColumnErrorHandling::Error => {
                transformer
                    .transform(self, sqlparser::ast::JsonTableColumnErrorHandling::Error)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableNamedColumn {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, r#type, path, exists, on_empty, on_error } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            r#type: r#type.apply_transform(transformer)?,
            path: path.apply_transform(transformer)?,
            exists: exists.apply_transform(transformer)?,
            on_empty: on_empty.apply_transform(transformer)?,
            on_error: on_error.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableNestedColumn {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { path, columns } = self;
        let transformed = Self {
            path: path.apply_transform(transformer)?,
            columns: columns.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::KeyOrIndexDisplay {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::KeyOrIndexDisplay::None => {
                transformer.transform(self, sqlparser::ast::KeyOrIndexDisplay::None)
            }
            sqlparser::ast::KeyOrIndexDisplay::Key => {
                transformer.transform(self, sqlparser::ast::KeyOrIndexDisplay::Key)
            }
            sqlparser::ast::KeyOrIndexDisplay::Index => {
                transformer.transform(self, sqlparser::ast::KeyOrIndexDisplay::Index)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::KillType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::KillType::Connection => {
                transformer.transform(self, sqlparser::ast::KillType::Connection)
            }
            sqlparser::ast::KillType::Query => {
                transformer.transform(self, sqlparser::ast::KillType::Query)
            }
            sqlparser::ast::KillType::Mutation => {
                transformer.transform(self, sqlparser::ast::KillType::Mutation)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LambdaFunction {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { params, body } = self;
        let transformed = Self {
            params: params.apply_transform(transformer)?,
            body: body.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LateralView {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { lateral_view, lateral_view_name, lateral_col_alias, outer } = self;
        let transformed = Self {
            lateral_view: lateral_view.apply_transform(transformer)?,
            lateral_view_name: lateral_view_name.apply_transform(transformer)?,
            lateral_col_alias: lateral_col_alias.apply_transform(transformer)?,
            outer: outer.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ListAggOnOverflow {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ListAggOnOverflow::Error => {
                transformer.transform(self, sqlparser::ast::ListAggOnOverflow::Error)
            }
            sqlparser::ast::ListAggOnOverflow::Truncate { filler, with_count } => {
                let transformed = sqlparser::ast::ListAggOnOverflow::Truncate {
                    filler: filler.apply_transform(transformer)?,
                    with_count: with_count.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockClause {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { lock_type, of, nonblock } = self;
        let transformed = Self {
            lock_type: lock_type.apply_transform(transformer)?,
            of: of.apply_transform(transformer)?,
            nonblock: nonblock.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockTable {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { table, alias, lock_type } = self;
        let transformed = Self {
            table: table.apply_transform(transformer)?,
            alias: alias.apply_transform(transformer)?,
            lock_type: lock_type.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockTableType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::LockTableType::Read { local } => {
                let transformed = sqlparser::ast::LockTableType::Read {
                    local: local.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::LockTableType::Write { low_priority } => {
                let transformed = sqlparser::ast::LockTableType::Write {
                    low_priority: low_priority.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::LockType::Share => {
                transformer.transform(self, sqlparser::ast::LockType::Share)
            }
            sqlparser::ast::LockType::Update => {
                transformer.transform(self, sqlparser::ast::LockType::Update)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MacroArg {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, default_expr } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            default_expr: default_expr.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MacroDefinition {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MacroDefinition::Expr(field0) => {
                let transformed = sqlparser::ast::MacroDefinition::Expr(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MacroDefinition::Table(field0) => {
                let transformed = sqlparser::ast::MacroDefinition::Table(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Map {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { entries } = self;
        let transformed = Self {
            entries: entries.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapAccessKey {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { key, syntax } = self;
        let transformed = Self {
            key: key.apply_transform(transformer)?,
            syntax: syntax.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapAccessSyntax {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MapAccessSyntax::Bracket => {
                transformer.transform(self, sqlparser::ast::MapAccessSyntax::Bracket)
            }
            sqlparser::ast::MapAccessSyntax::Period => {
                transformer.transform(self, sqlparser::ast::MapAccessSyntax::Period)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapEntry {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { key, value } = self;
        let transformed = Self {
            key: key.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MatchRecognizePattern {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MatchRecognizePattern::Symbol(field0) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Symbol(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizePattern::Exclude(field0) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Exclude(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizePattern::Permute(field0) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Permute(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizePattern::Concat(field0) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Concat(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizePattern::Group(field0) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Group(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizePattern::Alternation(field0) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Alternation(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizePattern::Repetition(field0, field1) => {
                let transformed = sqlparser::ast::MatchRecognizePattern::Repetition(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MatchRecognizeSymbol {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MatchRecognizeSymbol::Named(field0) => {
                let transformed = sqlparser::ast::MatchRecognizeSymbol::Named(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MatchRecognizeSymbol::Start => {
                transformer.transform(self, sqlparser::ast::MatchRecognizeSymbol::Start)
            }
            sqlparser::ast::MatchRecognizeSymbol::End => {
                transformer.transform(self, sqlparser::ast::MatchRecognizeSymbol::End)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Measure {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { expr, alias } = self;
        let transformed = Self {
            expr: expr.apply_transform(transformer)?,
            alias: alias.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeAction {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MergeAction::Insert(field0) => {
                let transformed = sqlparser::ast::MergeAction::Insert(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MergeAction::Update { assignments } => {
                let transformed = sqlparser::ast::MergeAction::Update {
                    assignments: assignments.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MergeAction::Delete => {
                transformer.transform(self, sqlparser::ast::MergeAction::Delete)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeClause {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { clause_kind, predicate, action } = self;
        let transformed = Self {
            clause_kind: clause_kind.apply_transform(transformer)?,
            predicate: predicate.apply_transform(transformer)?,
            action: action.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeClauseKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MergeClauseKind::Matched => {
                transformer.transform(self, sqlparser::ast::MergeClauseKind::Matched)
            }
            sqlparser::ast::MergeClauseKind::NotMatched => {
                transformer.transform(self, sqlparser::ast::MergeClauseKind::NotMatched)
            }
            sqlparser::ast::MergeClauseKind::NotMatchedByTarget => {
                transformer
                    .transform(self, sqlparser::ast::MergeClauseKind::NotMatchedByTarget)
            }
            sqlparser::ast::MergeClauseKind::NotMatchedBySource => {
                transformer
                    .transform(self, sqlparser::ast::MergeClauseKind::NotMatchedBySource)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeInsertExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { columns, kind } = self;
        let transformed = Self {
            columns: columns.apply_transform(transformer)?,
            kind: kind.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeInsertKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MergeInsertKind::Values(field0) => {
                let transformed = sqlparser::ast::MergeInsertKind::Values(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::MergeInsertKind::Row => {
                transformer.transform(self, sqlparser::ast::MergeInsertKind::Row)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MinMaxValue {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MinMaxValue::Empty => {
                transformer.transform(self, sqlparser::ast::MinMaxValue::Empty)
            }
            sqlparser::ast::MinMaxValue::None => {
                transformer.transform(self, sqlparser::ast::MinMaxValue::None)
            }
            sqlparser::ast::MinMaxValue::Some(field0) => {
                let transformed = sqlparser::ast::MinMaxValue::Some(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MySQLColumnPosition {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MySQLColumnPosition::First => {
                transformer.transform(self, sqlparser::ast::MySQLColumnPosition::First)
            }
            sqlparser::ast::MySQLColumnPosition::After(field0) => {
                let transformed = sqlparser::ast::MySQLColumnPosition::After(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MysqlInsertPriority {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::MysqlInsertPriority::LowPriority => {
                transformer
                    .transform(self, sqlparser::ast::MysqlInsertPriority::LowPriority)
            }
            sqlparser::ast::MysqlInsertPriority::Delayed => {
                transformer.transform(self, sqlparser::ast::MysqlInsertPriority::Delayed)
            }
            sqlparser::ast::MysqlInsertPriority::HighPriority => {
                transformer
                    .transform(self, sqlparser::ast::MysqlInsertPriority::HighPriority)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NamedWindowDefinition {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self(field0, field1) = self;
        let transformed = Self(
            field0.apply_transform(transformer)?,
            field1.apply_transform(transformer)?,
        );
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NamedWindowExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::NamedWindowExpr::NamedWindow(field0) => {
                let transformed = sqlparser::ast::NamedWindowExpr::NamedWindow(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::NamedWindowExpr::WindowSpec(field0) => {
                let transformed = sqlparser::ast::NamedWindowExpr::WindowSpec(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NonBlock {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::NonBlock::Nowait => {
                transformer.transform(self, sqlparser::ast::NonBlock::Nowait)
            }
            sqlparser::ast::NonBlock::SkipLocked => {
                transformer.transform(self, sqlparser::ast::NonBlock::SkipLocked)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NullTreatment {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::NullTreatment::IgnoreNulls => {
                transformer.transform(self, sqlparser::ast::NullTreatment::IgnoreNulls)
            }
            sqlparser::ast::NullTreatment::RespectNulls => {
                transformer.transform(self, sqlparser::ast::NullTreatment::RespectNulls)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ObjectName {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self(field0) = self;
        let transformed = Self(field0.apply_transform(transformer)?);
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ObjectType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ObjectType::Table => {
                transformer.transform(self, sqlparser::ast::ObjectType::Table)
            }
            sqlparser::ast::ObjectType::View => {
                transformer.transform(self, sqlparser::ast::ObjectType::View)
            }
            sqlparser::ast::ObjectType::Index => {
                transformer.transform(self, sqlparser::ast::ObjectType::Index)
            }
            sqlparser::ast::ObjectType::Schema => {
                transformer.transform(self, sqlparser::ast::ObjectType::Schema)
            }
            sqlparser::ast::ObjectType::Database => {
                transformer.transform(self, sqlparser::ast::ObjectType::Database)
            }
            sqlparser::ast::ObjectType::Role => {
                transformer.transform(self, sqlparser::ast::ObjectType::Role)
            }
            sqlparser::ast::ObjectType::Sequence => {
                transformer.transform(self, sqlparser::ast::ObjectType::Sequence)
            }
            sqlparser::ast::ObjectType::Stage => {
                transformer.transform(self, sqlparser::ast::ObjectType::Stage)
            }
            sqlparser::ast::ObjectType::Type => {
                transformer.transform(self, sqlparser::ast::ObjectType::Type)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Offset {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { value, rows } = self;
        let transformed = Self {
            value: value.apply_transform(transformer)?,
            rows: rows.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OffsetRows {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::OffsetRows::None => {
                transformer.transform(self, sqlparser::ast::OffsetRows::None)
            }
            sqlparser::ast::OffsetRows::Row => {
                transformer.transform(self, sqlparser::ast::OffsetRows::Row)
            }
            sqlparser::ast::OffsetRows::Rows => {
                transformer.transform(self, sqlparser::ast::OffsetRows::Rows)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnCommit {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::OnCommit::DeleteRows => {
                transformer.transform(self, sqlparser::ast::OnCommit::DeleteRows)
            }
            sqlparser::ast::OnCommit::PreserveRows => {
                transformer.transform(self, sqlparser::ast::OnCommit::PreserveRows)
            }
            sqlparser::ast::OnCommit::Drop => {
                transformer.transform(self, sqlparser::ast::OnCommit::Drop)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnConflict {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { conflict_target, action } = self;
        let transformed = Self {
            conflict_target: conflict_target.apply_transform(transformer)?,
            action: action.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnConflictAction {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::OnConflictAction::DoNothing => {
                transformer.transform(self, sqlparser::ast::OnConflictAction::DoNothing)
            }
            sqlparser::ast::OnConflictAction::DoUpdate(field0) => {
                let transformed = sqlparser::ast::OnConflictAction::DoUpdate(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnInsert {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::OnInsert::DuplicateKeyUpdate(field0) => {
                let transformed = sqlparser::ast::OnInsert::DuplicateKeyUpdate(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::OnInsert::OnConflict(field0) => {
                let transformed = sqlparser::ast::OnInsert::OnConflict(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OperateFunctionArg {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { mode, name, data_type, default_expr } = self;
        let transformed = Self {
            mode: mode.apply_transform(transformer)?,
            name: name.apply_transform(transformer)?,
            data_type: data_type.apply_transform(transformer)?,
            default_expr: default_expr.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OrderBy {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { exprs, interpolate } = self;
        let transformed = Self {
            exprs: exprs.apply_transform(transformer)?,
            interpolate: interpolate.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OrderByExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { expr, asc, nulls_first, with_fill } = self;
        let transformed = Self {
            expr: expr.apply_transform(transformer)?,
            asc: asc.apply_transform(transformer)?,
            nulls_first: nulls_first.apply_transform(transformer)?,
            with_fill: with_fill.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Owner {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Owner::Ident(field0) => {
                let transformed = sqlparser::ast::Owner::Ident(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Owner::CurrentRole => {
                transformer.transform(self, sqlparser::ast::Owner::CurrentRole)
            }
            sqlparser::ast::Owner::CurrentUser => {
                transformer.transform(self, sqlparser::ast::Owner::CurrentUser)
            }
            sqlparser::ast::Owner::SessionUser => {
                transformer.transform(self, sqlparser::ast::Owner::SessionUser)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Partition {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Partition::Identifier(field0) => {
                let transformed = sqlparser::ast::Partition::Identifier(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Partition::Expr(field0) => {
                let transformed = sqlparser::ast::Partition::Expr(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Partition::Part(field0) => {
                let transformed = sqlparser::ast::Partition::Part(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Partition::Partitions(field0) => {
                let transformed = sqlparser::ast::Partition::Partitions(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::PartitionRangeDirection {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::PartitionRangeDirection::Left => {
                transformer
                    .transform(self, sqlparser::ast::PartitionRangeDirection::Left)
            }
            sqlparser::ast::PartitionRangeDirection::Right => {
                transformer
                    .transform(self, sqlparser::ast::PartitionRangeDirection::Right)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Password {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Password::Password(field0) => {
                let transformed = sqlparser::ast::Password::Password(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Password::NullPassword => {
                transformer.transform(self, sqlparser::ast::Password::NullPassword)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::PivotValueSource {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::PivotValueSource::List(field0) => {
                let transformed = sqlparser::ast::PivotValueSource::List(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::PivotValueSource::Any(field0) => {
                let transformed = sqlparser::ast::PivotValueSource::Any(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::PivotValueSource::Subquery(field0) => {
                let transformed = sqlparser::ast::PivotValueSource::Subquery(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Privileges {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Privileges::All { with_privileges_keyword } => {
                let transformed = sqlparser::ast::Privileges::All {
                    with_privileges_keyword: with_privileges_keyword
                        .apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Privileges::Actions(field0) => {
                let transformed = sqlparser::ast::Privileges::Actions(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ProcedureParam {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, data_type } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            data_type: data_type.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ProjectionSelect {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { projection, order_by, group_by } = self;
        let transformed = Self {
            projection: projection.apply_transform(transformer)?,
            order_by: order_by.apply_transform(transformer)?,
            group_by: group_by.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Query {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            with,
            body,
            order_by,
            limit,
            limit_by,
            offset,
            fetch,
            locks,
            for_clause,
            settings,
            format_clause,
        } = self;
        let transformed = Self {
            with: with.apply_transform(transformer)?,
            body: body.apply_transform(transformer)?,
            order_by: order_by.apply_transform(transformer)?,
            limit: limit.apply_transform(transformer)?,
            limit_by: limit_by.apply_transform(transformer)?,
            offset: offset.apply_transform(transformer)?,
            fetch: fetch.apply_transform(transformer)?,
            locks: locks.apply_transform(transformer)?,
            for_clause: for_clause.apply_transform(transformer)?,
            settings: settings.apply_transform(transformer)?,
            format_clause: format_clause.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReferentialAction {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ReferentialAction::Restrict => {
                transformer.transform(self, sqlparser::ast::ReferentialAction::Restrict)
            }
            sqlparser::ast::ReferentialAction::Cascade => {
                transformer.transform(self, sqlparser::ast::ReferentialAction::Cascade)
            }
            sqlparser::ast::ReferentialAction::SetNull => {
                transformer.transform(self, sqlparser::ast::ReferentialAction::SetNull)
            }
            sqlparser::ast::ReferentialAction::NoAction => {
                transformer.transform(self, sqlparser::ast::ReferentialAction::NoAction)
            }
            sqlparser::ast::ReferentialAction::SetDefault => {
                transformer
                    .transform(self, sqlparser::ast::ReferentialAction::SetDefault)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RenameSelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::RenameSelectItem::Single(field0) => {
                let transformed = sqlparser::ast::RenameSelectItem::Single(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RenameSelectItem::Multiple(field0) => {
                let transformed = sqlparser::ast::RenameSelectItem::Multiple(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RepetitionQuantifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::RepetitionQuantifier::ZeroOrMore => {
                transformer
                    .transform(self, sqlparser::ast::RepetitionQuantifier::ZeroOrMore)
            }
            sqlparser::ast::RepetitionQuantifier::OneOrMore => {
                transformer
                    .transform(self, sqlparser::ast::RepetitionQuantifier::OneOrMore)
            }
            sqlparser::ast::RepetitionQuantifier::AtMostOne => {
                transformer
                    .transform(self, sqlparser::ast::RepetitionQuantifier::AtMostOne)
            }
            sqlparser::ast::RepetitionQuantifier::Exactly(field0) => {
                let transformed = sqlparser::ast::RepetitionQuantifier::Exactly(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RepetitionQuantifier::AtLeast(field0) => {
                let transformed = sqlparser::ast::RepetitionQuantifier::AtLeast(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RepetitionQuantifier::AtMost(field0) => {
                let transformed = sqlparser::ast::RepetitionQuantifier::AtMost(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RepetitionQuantifier::Range(field0, field1) => {
                let transformed = sqlparser::ast::RepetitionQuantifier::Range(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReplaceSelectElement {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { expr, column_name, as_keyword } = self;
        let transformed = Self {
            expr: expr.apply_transform(transformer)?,
            column_name: column_name.apply_transform(transformer)?,
            as_keyword: as_keyword.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReplaceSelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { items } = self;
        let transformed = Self {
            items: items.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ResetConfig {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ResetConfig::ALL => {
                transformer.transform(self, sqlparser::ast::ResetConfig::ALL)
            }
            sqlparser::ast::ResetConfig::ConfigName(field0) => {
                let transformed = sqlparser::ast::ResetConfig::ConfigName(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RoleOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::RoleOption::BypassRLS(field0) => {
                let transformed = sqlparser::ast::RoleOption::BypassRLS(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::ConnectionLimit(field0) => {
                let transformed = sqlparser::ast::RoleOption::ConnectionLimit(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::CreateDB(field0) => {
                let transformed = sqlparser::ast::RoleOption::CreateDB(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::CreateRole(field0) => {
                let transformed = sqlparser::ast::RoleOption::CreateRole(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::Inherit(field0) => {
                let transformed = sqlparser::ast::RoleOption::Inherit(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::Login(field0) => {
                let transformed = sqlparser::ast::RoleOption::Login(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::Password(field0) => {
                let transformed = sqlparser::ast::RoleOption::Password(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::Replication(field0) => {
                let transformed = sqlparser::ast::RoleOption::Replication(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::SuperUser(field0) => {
                let transformed = sqlparser::ast::RoleOption::SuperUser(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::RoleOption::ValidUntil(field0) => {
                let transformed = sqlparser::ast::RoleOption::ValidUntil(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RowAccessPolicy {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { policy, on } = self;
        let transformed = Self {
            policy: policy.apply_transform(transformer)?,
            on: on.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RowsPerMatch {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::RowsPerMatch::OneRow => {
                transformer.transform(self, sqlparser::ast::RowsPerMatch::OneRow)
            }
            sqlparser::ast::RowsPerMatch::AllRows(field0) => {
                let transformed = sqlparser::ast::RowsPerMatch::AllRows(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SchemaName {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SchemaName::Simple(field0) => {
                let transformed = sqlparser::ast::SchemaName::Simple(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SchemaName::UnnamedAuthorization(field0) => {
                let transformed = sqlparser::ast::SchemaName::UnnamedAuthorization(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SchemaName::NamedAuthorization(field0, field1) => {
                let transformed = sqlparser::ast::SchemaName::NamedAuthorization(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SearchModifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SearchModifier::InNaturalLanguageMode => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::SearchModifier::InNaturalLanguageMode,
                    )
            }
            sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion,
                    )
            }
            sqlparser::ast::SearchModifier::InBooleanMode => {
                transformer
                    .transform(self, sqlparser::ast::SearchModifier::InBooleanMode)
            }
            sqlparser::ast::SearchModifier::WithQueryExpansion => {
                transformer
                    .transform(self, sqlparser::ast::SearchModifier::WithQueryExpansion)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SecretOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { key, value } = self;
        let transformed = Self {
            key: key.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Select {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self {
            distinct,
            top,
            top_before_distinct,
            projection,
            into,
            from,
            lateral_views,
            prewhere,
            selection,
            group_by,
            cluster_by,
            distribute_by,
            sort_by,
            having,
            named_window,
            qualify,
            window_before_qualify,
            value_table_mode,
            connect_by,
        } = self;
        let transformed = Self {
            distinct: distinct.apply_transform(transformer)?,
            top: top.apply_transform(transformer)?,
            top_before_distinct: top_before_distinct.apply_transform(transformer)?,
            projection: projection.apply_transform(transformer)?,
            into: into.apply_transform(transformer)?,
            from: from.apply_transform(transformer)?,
            lateral_views: lateral_views.apply_transform(transformer)?,
            prewhere: prewhere.apply_transform(transformer)?,
            selection: selection.apply_transform(transformer)?,
            group_by: group_by.apply_transform(transformer)?,
            cluster_by: cluster_by.apply_transform(transformer)?,
            distribute_by: distribute_by.apply_transform(transformer)?,
            sort_by: sort_by.apply_transform(transformer)?,
            having: having.apply_transform(transformer)?,
            named_window: named_window.apply_transform(transformer)?,
            qualify: qualify.apply_transform(transformer)?,
            window_before_qualify: window_before_qualify.apply_transform(transformer)?,
            value_table_mode: value_table_mode.apply_transform(transformer)?,
            connect_by: connect_by.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SelectInto {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { temporary, unlogged, table, name } = self;
        let transformed = Self {
            temporary: temporary.apply_transform(transformer)?,
            unlogged: unlogged.apply_transform(transformer)?,
            table: table.apply_transform(transformer)?,
            name: name.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SelectItem::UnnamedExpr(field0) => {
                let transformed = sqlparser::ast::SelectItem::UnnamedExpr(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                let transformed = sqlparser::ast::SelectItem::ExprWithAlias {
                    expr: expr.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SelectItem::QualifiedWildcard(field0, field1) => {
                let transformed = sqlparser::ast::SelectItem::QualifiedWildcard(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SelectItem::Wildcard(field0) => {
                let transformed = sqlparser::ast::SelectItem::Wildcard(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SequenceOptions {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SequenceOptions::IncrementBy(field0, field1) => {
                let transformed = sqlparser::ast::SequenceOptions::IncrementBy(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SequenceOptions::MinValue(field0) => {
                let transformed = sqlparser::ast::SequenceOptions::MinValue(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SequenceOptions::MaxValue(field0) => {
                let transformed = sqlparser::ast::SequenceOptions::MaxValue(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SequenceOptions::StartWith(field0, field1) => {
                let transformed = sqlparser::ast::SequenceOptions::StartWith(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SequenceOptions::Cache(field0) => {
                let transformed = sqlparser::ast::SequenceOptions::Cache(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SequenceOptions::Cycle(field0) => {
                let transformed = sqlparser::ast::SequenceOptions::Cycle(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetConfigValue {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SetConfigValue::Default => {
                transformer.transform(self, sqlparser::ast::SetConfigValue::Default)
            }
            sqlparser::ast::SetConfigValue::FromCurrent => {
                transformer.transform(self, sqlparser::ast::SetConfigValue::FromCurrent)
            }
            sqlparser::ast::SetConfigValue::Value(field0) => {
                let transformed = sqlparser::ast::SetConfigValue::Value(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetExpr {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SetExpr::Select(field0) => {
                let transformed = sqlparser::ast::SetExpr::Select(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SetExpr::Query(field0) => {
                let transformed = sqlparser::ast::SetExpr::Query(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SetExpr::SetOperation {
                op,
                set_quantifier,
                left,
                right,
            } => {
                let transformed = sqlparser::ast::SetExpr::SetOperation {
                    op: op.apply_transform(transformer)?,
                    set_quantifier: set_quantifier.apply_transform(transformer)?,
                    left: left.apply_transform(transformer)?,
                    right: right.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SetExpr::Values(field0) => {
                let transformed = sqlparser::ast::SetExpr::Values(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SetExpr::Insert(field0) => {
                let transformed = sqlparser::ast::SetExpr::Insert(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SetExpr::Update(field0) => {
                let transformed = sqlparser::ast::SetExpr::Update(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SetExpr::Table(field0) => {
                let transformed = sqlparser::ast::SetExpr::Table(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetOperator {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SetOperator::Union => {
                transformer.transform(self, sqlparser::ast::SetOperator::Union)
            }
            sqlparser::ast::SetOperator::Except => {
                transformer.transform(self, sqlparser::ast::SetOperator::Except)
            }
            sqlparser::ast::SetOperator::Intersect => {
                transformer.transform(self, sqlparser::ast::SetOperator::Intersect)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetQuantifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SetQuantifier::All => {
                transformer.transform(self, sqlparser::ast::SetQuantifier::All)
            }
            sqlparser::ast::SetQuantifier::Distinct => {
                transformer.transform(self, sqlparser::ast::SetQuantifier::Distinct)
            }
            sqlparser::ast::SetQuantifier::ByName => {
                transformer.transform(self, sqlparser::ast::SetQuantifier::ByName)
            }
            sqlparser::ast::SetQuantifier::AllByName => {
                transformer.transform(self, sqlparser::ast::SetQuantifier::AllByName)
            }
            sqlparser::ast::SetQuantifier::DistinctByName => {
                transformer
                    .transform(self, sqlparser::ast::SetQuantifier::DistinctByName)
            }
            sqlparser::ast::SetQuantifier::None => {
                transformer.transform(self, sqlparser::ast::SetQuantifier::None)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Setting {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { key, value } = self;
        let transformed = Self {
            key: key.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowClause {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ShowClause::IN => {
                transformer.transform(self, sqlparser::ast::ShowClause::IN)
            }
            sqlparser::ast::ShowClause::FROM => {
                transformer.transform(self, sqlparser::ast::ShowClause::FROM)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowCreateObject {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ShowCreateObject::Event => {
                transformer.transform(self, sqlparser::ast::ShowCreateObject::Event)
            }
            sqlparser::ast::ShowCreateObject::Function => {
                transformer.transform(self, sqlparser::ast::ShowCreateObject::Function)
            }
            sqlparser::ast::ShowCreateObject::Procedure => {
                transformer.transform(self, sqlparser::ast::ShowCreateObject::Procedure)
            }
            sqlparser::ast::ShowCreateObject::Table => {
                transformer.transform(self, sqlparser::ast::ShowCreateObject::Table)
            }
            sqlparser::ast::ShowCreateObject::Trigger => {
                transformer.transform(self, sqlparser::ast::ShowCreateObject::Trigger)
            }
            sqlparser::ast::ShowCreateObject::View => {
                transformer.transform(self, sqlparser::ast::ShowCreateObject::View)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowStatementFilter {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ShowStatementFilter::Like(field0) => {
                let transformed = sqlparser::ast::ShowStatementFilter::Like(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ShowStatementFilter::ILike(field0) => {
                let transformed = sqlparser::ast::ShowStatementFilter::ILike(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ShowStatementFilter::Where(field0) => {
                let transformed = sqlparser::ast::ShowStatementFilter::Where(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::ShowStatementFilter::NoKeyword(field0) => {
                let transformed = sqlparser::ast::ShowStatementFilter::NoKeyword(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SqlOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SqlOption::Clustered(field0) => {
                let transformed = sqlparser::ast::SqlOption::Clustered(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SqlOption::Ident(field0) => {
                let transformed = sqlparser::ast::SqlOption::Ident(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SqlOption::KeyValue { key, value } => {
                let transformed = sqlparser::ast::SqlOption::KeyValue {
                    key: key.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::SqlOption::Partition {
                column_name,
                range_direction,
                for_values,
            } => {
                let transformed = sqlparser::ast::SqlOption::Partition {
                    column_name: column_name.apply_transform(transformer)?,
                    range_direction: range_direction.apply_transform(transformer)?,
                    for_values: for_values.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SqliteOnConflict {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::SqliteOnConflict::Rollback => {
                transformer.transform(self, sqlparser::ast::SqliteOnConflict::Rollback)
            }
            sqlparser::ast::SqliteOnConflict::Abort => {
                transformer.transform(self, sqlparser::ast::SqliteOnConflict::Abort)
            }
            sqlparser::ast::SqliteOnConflict::Fail => {
                transformer.transform(self, sqlparser::ast::SqliteOnConflict::Fail)
            }
            sqlparser::ast::SqliteOnConflict::Ignore => {
                transformer.transform(self, sqlparser::ast::SqliteOnConflict::Ignore)
            }
            sqlparser::ast::SqliteOnConflict::Replace => {
                transformer.transform(self, sqlparser::ast::SqliteOnConflict::Replace)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Statement {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
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
                let transformed = sqlparser::ast::Statement::Analyze {
                    table_name: table_name.apply_transform(transformer)?,
                    partitions: partitions.apply_transform(transformer)?,
                    for_columns: for_columns.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    cache_metadata: cache_metadata.apply_transform(transformer)?,
                    noscan: noscan.apply_transform(transformer)?,
                    compute_statistics: compute_statistics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::Truncate {
                    table_names: table_names.apply_transform(transformer)?,
                    partitions: partitions.apply_transform(transformer)?,
                    table: table.apply_transform(transformer)?,
                    only: only.apply_transform(transformer)?,
                    identity: identity.apply_transform(transformer)?,
                    cascade: cascade.apply_transform(transformer)?,
                    on_cluster: on_cluster.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Msck { table_name, repair, partition_action } => {
                let transformed = sqlparser::ast::Statement::Msck {
                    table_name: table_name.apply_transform(transformer)?,
                    repair: repair.apply_transform(transformer)?,
                    partition_action: partition_action.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Query(field0) => {
                let transformed = sqlparser::ast::Statement::Query(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Insert(field0) => {
                let transformed = sqlparser::ast::Statement::Insert(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Install { extension_name } => {
                let transformed = sqlparser::ast::Statement::Install {
                    extension_name: extension_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Load { extension_name } => {
                let transformed = sqlparser::ast::Statement::Load {
                    extension_name: extension_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Directory {
                overwrite,
                local,
                path,
                file_format,
                source,
            } => {
                let transformed = sqlparser::ast::Statement::Directory {
                    overwrite: overwrite.apply_transform(transformer)?,
                    local: local.apply_transform(transformer)?,
                    path: path.apply_transform(transformer)?,
                    file_format: file_format.apply_transform(transformer)?,
                    source: source.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Call(field0) => {
                let transformed = sqlparser::ast::Statement::Call(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Copy {
                source,
                to,
                target,
                options,
                legacy_options,
                values,
            } => {
                let transformed = sqlparser::ast::Statement::Copy {
                    source: source.apply_transform(transformer)?,
                    to: to.apply_transform(transformer)?,
                    target: target.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                    legacy_options: legacy_options.apply_transform(transformer)?,
                    values: values.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CopyIntoSnowflake {
                    into: into.apply_transform(transformer)?,
                    from_stage: from_stage.apply_transform(transformer)?,
                    from_stage_alias: from_stage_alias.apply_transform(transformer)?,
                    stage_params: stage_params.apply_transform(transformer)?,
                    from_transformations: from_transformations
                        .apply_transform(transformer)?,
                    files: files.apply_transform(transformer)?,
                    pattern: pattern.apply_transform(transformer)?,
                    file_format: file_format.apply_transform(transformer)?,
                    copy_options: copy_options.apply_transform(transformer)?,
                    validation_mode: validation_mode.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Close { cursor } => {
                let transformed = sqlparser::ast::Statement::Close {
                    cursor: cursor.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Update {
                table,
                assignments,
                from,
                selection,
                returning,
            } => {
                let transformed = sqlparser::ast::Statement::Update {
                    table: table.apply_transform(transformer)?,
                    assignments: assignments.apply_transform(transformer)?,
                    from: from.apply_transform(transformer)?,
                    selection: selection.apply_transform(transformer)?,
                    returning: returning.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Delete(field0) => {
                let transformed = sqlparser::ast::Statement::Delete(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreateView {
                    or_replace: or_replace.apply_transform(transformer)?,
                    materialized: materialized.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    query: query.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                    cluster_by: cluster_by.apply_transform(transformer)?,
                    comment: comment.apply_transform(transformer)?,
                    with_no_schema_binding: with_no_schema_binding
                        .apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                    to: to.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateTable(field0) => {
                let transformed = sqlparser::ast::Statement::CreateTable(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateVirtualTable {
                name,
                if_not_exists,
                module_name,
                module_args,
            } => {
                let transformed = sqlparser::ast::Statement::CreateVirtualTable {
                    name: name.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    module_name: module_name.apply_transform(transformer)?,
                    module_args: module_args.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateIndex(field0) => {
                let transformed = sqlparser::ast::Statement::CreateIndex(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreateRole {
                    names: names.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    login: login.apply_transform(transformer)?,
                    inherit: inherit.apply_transform(transformer)?,
                    bypassrls: bypassrls.apply_transform(transformer)?,
                    password: password.apply_transform(transformer)?,
                    superuser: superuser.apply_transform(transformer)?,
                    create_db: create_db.apply_transform(transformer)?,
                    create_role: create_role.apply_transform(transformer)?,
                    replication: replication.apply_transform(transformer)?,
                    connection_limit: connection_limit.apply_transform(transformer)?,
                    valid_until: valid_until.apply_transform(transformer)?,
                    in_role: in_role.apply_transform(transformer)?,
                    in_group: in_group.apply_transform(transformer)?,
                    role: role.apply_transform(transformer)?,
                    user: user.apply_transform(transformer)?,
                    admin: admin.apply_transform(transformer)?,
                    authorization_owner: authorization_owner
                        .apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreateSecret {
                    or_replace: or_replace.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    storage_specifier: storage_specifier.apply_transform(transformer)?,
                    secret_type: secret_type.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreatePolicy {
                    name: name.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    policy_type: policy_type.apply_transform(transformer)?,
                    command: command.apply_transform(transformer)?,
                    to: to.apply_transform(transformer)?,
                    using: using.apply_transform(transformer)?,
                    with_check: with_check.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AlterTable {
                name,
                if_exists,
                only,
                operations,
                location,
                on_cluster,
            } => {
                let transformed = sqlparser::ast::Statement::AlterTable {
                    name: name.apply_transform(transformer)?,
                    if_exists: if_exists.apply_transform(transformer)?,
                    only: only.apply_transform(transformer)?,
                    operations: operations.apply_transform(transformer)?,
                    location: location.apply_transform(transformer)?,
                    on_cluster: on_cluster.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AlterIndex { name, operation } => {
                let transformed = sqlparser::ast::Statement::AlterIndex {
                    name: name.apply_transform(transformer)?,
                    operation: operation.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AlterView {
                name,
                columns,
                query,
                with_options,
            } => {
                let transformed = sqlparser::ast::Statement::AlterView {
                    name: name.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    query: query.apply_transform(transformer)?,
                    with_options: with_options.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AlterRole { name, operation } => {
                let transformed = sqlparser::ast::Statement::AlterRole {
                    name: name.apply_transform(transformer)?,
                    operation: operation.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AlterPolicy { name, table_name, operation } => {
                let transformed = sqlparser::ast::Statement::AlterPolicy {
                    name: name.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    operation: operation.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AttachDatabase {
                schema_name,
                database_file_name,
                database,
            } => {
                let transformed = sqlparser::ast::Statement::AttachDatabase {
                    schema_name: schema_name.apply_transform(transformer)?,
                    database_file_name: database_file_name.apply_transform(transformer)?,
                    database: database.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::AttachDuckDBDatabase {
                if_not_exists,
                database,
                database_path,
                database_alias,
                attach_options,
            } => {
                let transformed = sqlparser::ast::Statement::AttachDuckDBDatabase {
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    database: database.apply_transform(transformer)?,
                    database_path: database_path.apply_transform(transformer)?,
                    database_alias: database_alias.apply_transform(transformer)?,
                    attach_options: attach_options.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::DetachDuckDBDatabase {
                if_exists,
                database,
                database_alias,
            } => {
                let transformed = sqlparser::ast::Statement::DetachDuckDBDatabase {
                    if_exists: if_exists.apply_transform(transformer)?,
                    database: database.apply_transform(transformer)?,
                    database_alias: database_alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::Drop {
                    object_type: object_type.apply_transform(transformer)?,
                    if_exists: if_exists.apply_transform(transformer)?,
                    names: names.apply_transform(transformer)?,
                    cascade: cascade.apply_transform(transformer)?,
                    restrict: restrict.apply_transform(transformer)?,
                    purge: purge.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::DropFunction { if_exists, func_desc, option } => {
                let transformed = sqlparser::ast::Statement::DropFunction {
                    if_exists: if_exists.apply_transform(transformer)?,
                    func_desc: func_desc.apply_transform(transformer)?,
                    option: option.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::DropProcedure {
                if_exists,
                proc_desc,
                option,
            } => {
                let transformed = sqlparser::ast::Statement::DropProcedure {
                    if_exists: if_exists.apply_transform(transformer)?,
                    proc_desc: proc_desc.apply_transform(transformer)?,
                    option: option.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::DropSecret {
                if_exists,
                temporary,
                name,
                storage_specifier,
            } => {
                let transformed = sqlparser::ast::Statement::DropSecret {
                    if_exists: if_exists.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    storage_specifier: storage_specifier.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::DropPolicy {
                if_exists,
                name,
                table_name,
                option,
            } => {
                let transformed = sqlparser::ast::Statement::DropPolicy {
                    if_exists: if_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    option: option.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Declare { stmts } => {
                let transformed = sqlparser::ast::Statement::Declare {
                    stmts: stmts.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateExtension {
                name,
                if_not_exists,
                cascade,
                schema,
                version,
            } => {
                let transformed = sqlparser::ast::Statement::CreateExtension {
                    name: name.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    cascade: cascade.apply_transform(transformer)?,
                    schema: schema.apply_transform(transformer)?,
                    version: version.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Fetch { name, direction, into } => {
                let transformed = sqlparser::ast::Statement::Fetch {
                    name: name.apply_transform(transformer)?,
                    direction: direction.apply_transform(transformer)?,
                    into: into.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Flush {
                object_type,
                location,
                channel,
                read_lock,
                export,
                tables,
            } => {
                let transformed = sqlparser::ast::Statement::Flush {
                    object_type: object_type.apply_transform(transformer)?,
                    location: location.apply_transform(transformer)?,
                    channel: channel.apply_transform(transformer)?,
                    read_lock: read_lock.apply_transform(transformer)?,
                    export: export.apply_transform(transformer)?,
                    tables: tables.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Discard { object_type } => {
                let transformed = sqlparser::ast::Statement::Discard {
                    object_type: object_type.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::SetRole { context_modifier, role_name } => {
                let transformed = sqlparser::ast::Statement::SetRole {
                    context_modifier: context_modifier.apply_transform(transformer)?,
                    role_name: role_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::SetVariable {
                local,
                hivevar,
                variables,
                value,
            } => {
                let transformed = sqlparser::ast::Statement::SetVariable {
                    local: local.apply_transform(transformer)?,
                    hivevar: hivevar.apply_transform(transformer)?,
                    variables: variables.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::SetTimeZone { local, value } => {
                let transformed = sqlparser::ast::Statement::SetTimeZone {
                    local: local.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::SetNames { charset_name, collation_name } => {
                let transformed = sqlparser::ast::Statement::SetNames {
                    charset_name: charset_name.apply_transform(transformer)?,
                    collation_name: collation_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::SetNamesDefault {} => {
                let transformed = sqlparser::ast::Statement::SetNamesDefault {
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowFunctions { filter } => {
                let transformed = sqlparser::ast::Statement::ShowFunctions {
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowVariable { variable } => {
                let transformed = sqlparser::ast::Statement::ShowVariable {
                    variable: variable.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowStatus { filter, global, session } => {
                let transformed = sqlparser::ast::Statement::ShowStatus {
                    filter: filter.apply_transform(transformer)?,
                    global: global.apply_transform(transformer)?,
                    session: session.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowVariables { filter, global, session } => {
                let transformed = sqlparser::ast::Statement::ShowVariables {
                    filter: filter.apply_transform(transformer)?,
                    global: global.apply_transform(transformer)?,
                    session: session.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowCreate { obj_type, obj_name } => {
                let transformed = sqlparser::ast::Statement::ShowCreate {
                    obj_type: obj_type.apply_transform(transformer)?,
                    obj_name: obj_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowColumns {
                extended,
                full,
                table_name,
                filter,
            } => {
                let transformed = sqlparser::ast::Statement::ShowColumns {
                    extended: extended.apply_transform(transformer)?,
                    full: full.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowDatabases { filter } => {
                let transformed = sqlparser::ast::Statement::ShowDatabases {
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowSchemas { filter } => {
                let transformed = sqlparser::ast::Statement::ShowSchemas {
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowTables {
                extended,
                full,
                clause,
                db_name,
                filter,
            } => {
                let transformed = sqlparser::ast::Statement::ShowTables {
                    extended: extended.apply_transform(transformer)?,
                    full: full.apply_transform(transformer)?,
                    clause: clause.apply_transform(transformer)?,
                    db_name: db_name.apply_transform(transformer)?,
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowViews {
                materialized,
                clause,
                db_name,
                filter,
            } => {
                let transformed = sqlparser::ast::Statement::ShowViews {
                    materialized: materialized.apply_transform(transformer)?,
                    clause: clause.apply_transform(transformer)?,
                    db_name: db_name.apply_transform(transformer)?,
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ShowCollation { filter } => {
                let transformed = sqlparser::ast::Statement::ShowCollation {
                    filter: filter.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Use(field0) => {
                let transformed = sqlparser::ast::Statement::Use(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::StartTransaction { modes, begin, modifier } => {
                let transformed = sqlparser::ast::Statement::StartTransaction {
                    modes: modes.apply_transform(transformer)?,
                    begin: begin.apply_transform(transformer)?,
                    modifier: modifier.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::SetTransaction { modes, snapshot, session } => {
                let transformed = sqlparser::ast::Statement::SetTransaction {
                    modes: modes.apply_transform(transformer)?,
                    snapshot: snapshot.apply_transform(transformer)?,
                    session: session.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Comment {
                object_type,
                object_name,
                comment,
                if_exists,
            } => {
                let transformed = sqlparser::ast::Statement::Comment {
                    object_type: object_type.apply_transform(transformer)?,
                    object_name: object_name.apply_transform(transformer)?,
                    comment: comment.apply_transform(transformer)?,
                    if_exists: if_exists.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Commit { chain } => {
                let transformed = sqlparser::ast::Statement::Commit {
                    chain: chain.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Rollback { chain, savepoint } => {
                let transformed = sqlparser::ast::Statement::Rollback {
                    chain: chain.apply_transform(transformer)?,
                    savepoint: savepoint.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateSchema { schema_name, if_not_exists } => {
                let transformed = sqlparser::ast::Statement::CreateSchema {
                    schema_name: schema_name.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateDatabase {
                db_name,
                if_not_exists,
                location,
                managed_location,
            } => {
                let transformed = sqlparser::ast::Statement::CreateDatabase {
                    db_name: db_name.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    location: location.apply_transform(transformer)?,
                    managed_location: managed_location.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreateFunction {
                    or_replace: or_replace.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    args: args.apply_transform(transformer)?,
                    return_type: return_type.apply_transform(transformer)?,
                    function_body: function_body.apply_transform(transformer)?,
                    behavior: behavior.apply_transform(transformer)?,
                    called_on_null: called_on_null.apply_transform(transformer)?,
                    parallel: parallel.apply_transform(transformer)?,
                    using: using.apply_transform(transformer)?,
                    language: language.apply_transform(transformer)?,
                    determinism_specifier: determinism_specifier
                        .apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                    remote_connection: remote_connection.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreateTrigger {
                    or_replace: or_replace.apply_transform(transformer)?,
                    is_constraint: is_constraint.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    period: period.apply_transform(transformer)?,
                    events: events.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    referenced_table_name: referenced_table_name
                        .apply_transform(transformer)?,
                    referencing: referencing.apply_transform(transformer)?,
                    trigger_object: trigger_object.apply_transform(transformer)?,
                    include_each: include_each.apply_transform(transformer)?,
                    condition: condition.apply_transform(transformer)?,
                    exec_body: exec_body.apply_transform(transformer)?,
                    characteristics: characteristics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::DropTrigger {
                if_exists,
                trigger_name,
                table_name,
                option,
            } => {
                let transformed = sqlparser::ast::Statement::DropTrigger {
                    if_exists: if_exists.apply_transform(transformer)?,
                    trigger_name: trigger_name.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    option: option.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateProcedure {
                or_alter,
                name,
                params,
                body,
            } => {
                let transformed = sqlparser::ast::Statement::CreateProcedure {
                    or_alter: or_alter.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    params: params.apply_transform(transformer)?,
                    body: body.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateMacro {
                or_replace,
                temporary,
                name,
                args,
                definition,
            } => {
                let transformed = sqlparser::ast::Statement::CreateMacro {
                    or_replace: or_replace.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    args: args.apply_transform(transformer)?,
                    definition: definition.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::CreateStage {
                    or_replace: or_replace.apply_transform(transformer)?,
                    temporary: temporary.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    stage_params: stage_params.apply_transform(transformer)?,
                    directory_table_params: directory_table_params
                        .apply_transform(transformer)?,
                    file_format: file_format.apply_transform(transformer)?,
                    copy_options: copy_options.apply_transform(transformer)?,
                    comment: comment.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Assert { condition, message } => {
                let transformed = sqlparser::ast::Statement::Assert {
                    condition: condition.apply_transform(transformer)?,
                    message: message.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Grant {
                privileges,
                objects,
                grantees,
                with_grant_option,
                granted_by,
            } => {
                let transformed = sqlparser::ast::Statement::Grant {
                    privileges: privileges.apply_transform(transformer)?,
                    objects: objects.apply_transform(transformer)?,
                    grantees: grantees.apply_transform(transformer)?,
                    with_grant_option: with_grant_option.apply_transform(transformer)?,
                    granted_by: granted_by.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Revoke {
                privileges,
                objects,
                grantees,
                granted_by,
                cascade,
            } => {
                let transformed = sqlparser::ast::Statement::Revoke {
                    privileges: privileges.apply_transform(transformer)?,
                    objects: objects.apply_transform(transformer)?,
                    grantees: grantees.apply_transform(transformer)?,
                    granted_by: granted_by.apply_transform(transformer)?,
                    cascade: cascade.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Deallocate { name, prepare } => {
                let transformed = sqlparser::ast::Statement::Deallocate {
                    name: name.apply_transform(transformer)?,
                    prepare: prepare.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Execute {
                name,
                parameters,
                has_parentheses,
                using,
            } => {
                let transformed = sqlparser::ast::Statement::Execute {
                    name: name.apply_transform(transformer)?,
                    parameters: parameters.apply_transform(transformer)?,
                    has_parentheses: has_parentheses.apply_transform(transformer)?,
                    using: using.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Prepare { name, data_types, statement } => {
                let transformed = sqlparser::ast::Statement::Prepare {
                    name: name.apply_transform(transformer)?,
                    data_types: data_types.apply_transform(transformer)?,
                    statement: statement.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Kill { modifier, id } => {
                let transformed = sqlparser::ast::Statement::Kill {
                    modifier: modifier.apply_transform(transformer)?,
                    id: id.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ExplainTable {
                describe_alias,
                hive_format,
                has_table_keyword,
                table_name,
            } => {
                let transformed = sqlparser::ast::Statement::ExplainTable {
                    describe_alias: describe_alias.apply_transform(transformer)?,
                    hive_format: hive_format.apply_transform(transformer)?,
                    has_table_keyword: has_table_keyword.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::Statement::Explain {
                    describe_alias: describe_alias.apply_transform(transformer)?,
                    analyze: analyze.apply_transform(transformer)?,
                    verbose: verbose.apply_transform(transformer)?,
                    query_plan: query_plan.apply_transform(transformer)?,
                    statement: statement.apply_transform(transformer)?,
                    format: format.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Savepoint { name } => {
                let transformed = sqlparser::ast::Statement::Savepoint {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::ReleaseSavepoint { name } => {
                let transformed = sqlparser::ast::Statement::ReleaseSavepoint {
                    name: name.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Merge { into, table, source, on, clauses } => {
                let transformed = sqlparser::ast::Statement::Merge {
                    into: into.apply_transform(transformer)?,
                    table: table.apply_transform(transformer)?,
                    source: source.apply_transform(transformer)?,
                    on: on.apply_transform(transformer)?,
                    clauses: clauses.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Cache {
                table_flag,
                table_name,
                has_as,
                options,
                query,
            } => {
                let transformed = sqlparser::ast::Statement::Cache {
                    table_flag: table_flag.apply_transform(transformer)?,
                    table_name: table_name.apply_transform(transformer)?,
                    has_as: has_as.apply_transform(transformer)?,
                    options: options.apply_transform(transformer)?,
                    query: query.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::UNCache { table_name, if_exists } => {
                let transformed = sqlparser::ast::Statement::UNCache {
                    table_name: table_name.apply_transform(transformer)?,
                    if_exists: if_exists.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateSequence {
                temporary,
                if_not_exists,
                name,
                data_type,
                sequence_options,
                owned_by,
            } => {
                let transformed = sqlparser::ast::Statement::CreateSequence {
                    temporary: temporary.apply_transform(transformer)?,
                    if_not_exists: if_not_exists.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    data_type: data_type.apply_transform(transformer)?,
                    sequence_options: sequence_options.apply_transform(transformer)?,
                    owned_by: owned_by.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::CreateType { name, representation } => {
                let transformed = sqlparser::ast::Statement::CreateType {
                    name: name.apply_transform(transformer)?,
                    representation: representation.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::Pragma { name, value, is_eq } => {
                let transformed = sqlparser::ast::Statement::Pragma {
                    name: name.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                    is_eq: is_eq.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::LockTables { tables } => {
                let transformed = sqlparser::ast::Statement::LockTables {
                    tables: tables.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::UnlockTables => {
                transformer.transform(self, sqlparser::ast::Statement::UnlockTables)
            }
            sqlparser::ast::Statement::Unload { query, to, with } => {
                let transformed = sqlparser::ast::Statement::Unload {
                    query: query.apply_transform(transformer)?,
                    to: to.apply_transform(transformer)?,
                    with: with.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::OptimizeTable {
                name,
                on_cluster,
                partition,
                include_final,
                deduplicate,
            } => {
                let transformed = sqlparser::ast::Statement::OptimizeTable {
                    name: name.apply_transform(transformer)?,
                    on_cluster: on_cluster.apply_transform(transformer)?,
                    partition: partition.apply_transform(transformer)?,
                    include_final: include_final.apply_transform(transformer)?,
                    deduplicate: deduplicate.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::LISTEN { channel } => {
                let transformed = sqlparser::ast::Statement::LISTEN {
                    channel: channel.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Statement::NOTIFY { channel, payload } => {
                let transformed = sqlparser::ast::Statement::NOTIFY {
                    channel: channel.apply_transform(transformer)?,
                    payload: payload.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::StructBracketKind {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::StructBracketKind::Parentheses => {
                transformer
                    .transform(self, sqlparser::ast::StructBracketKind::Parentheses)
            }
            sqlparser::ast::StructBracketKind::AngleBrackets => {
                transformer
                    .transform(self, sqlparser::ast::StructBracketKind::AngleBrackets)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::StructField {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { field_name, field_type } = self;
        let transformed = Self {
            field_name: field_name.apply_transform(transformer)?,
            field_type: field_type.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Subscript {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Subscript::Index { index } => {
                let transformed = sqlparser::ast::Subscript::Index {
                    index: index.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Subscript::Slice { lower_bound, upper_bound, stride } => {
                let transformed = sqlparser::ast::Subscript::Slice {
                    lower_bound: lower_bound.apply_transform(transformer)?,
                    upper_bound: upper_bound.apply_transform(transformer)?,
                    stride: stride.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SymbolDefinition {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { symbol, definition } = self;
        let transformed = Self {
            symbol: symbol.apply_transform(transformer)?,
            definition: definition.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Table {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { table_name, schema_name } = self;
        let transformed = Self {
            table_name: table_name.apply_transform(transformer)?,
            schema_name: schema_name.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableAlias {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, columns } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            columns: columns.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableConstraint {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
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
                let transformed = sqlparser::ast::TableConstraint::Unique {
                    name: name.apply_transform(transformer)?,
                    index_name: index_name.apply_transform(transformer)?,
                    index_type_display: index_type_display.apply_transform(transformer)?,
                    index_type: index_type.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    index_options: index_options.apply_transform(transformer)?,
                    characteristics: characteristics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableConstraint::PrimaryKey {
                name,
                index_name,
                index_type,
                columns,
                index_options,
                characteristics,
            } => {
                let transformed = sqlparser::ast::TableConstraint::PrimaryKey {
                    name: name.apply_transform(transformer)?,
                    index_name: index_name.apply_transform(transformer)?,
                    index_type: index_type.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    index_options: index_options.apply_transform(transformer)?,
                    characteristics: characteristics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::TableConstraint::ForeignKey {
                    name: name.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    foreign_table: foreign_table.apply_transform(transformer)?,
                    referred_columns: referred_columns.apply_transform(transformer)?,
                    on_delete: on_delete.apply_transform(transformer)?,
                    on_update: on_update.apply_transform(transformer)?,
                    characteristics: characteristics.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableConstraint::Check { name, expr } => {
                let transformed = sqlparser::ast::TableConstraint::Check {
                    name: name.apply_transform(transformer)?,
                    expr: expr.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableConstraint::Index {
                display_as_key,
                name,
                index_type,
                columns,
            } => {
                let transformed = sqlparser::ast::TableConstraint::Index {
                    display_as_key: display_as_key.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    index_type: index_type.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableConstraint::FulltextOrSpatial {
                fulltext,
                index_type_display,
                opt_index_name,
                columns,
            } => {
                let transformed = sqlparser::ast::TableConstraint::FulltextOrSpatial {
                    fulltext: fulltext.apply_transform(transformer)?,
                    index_type_display: index_type_display.apply_transform(transformer)?,
                    opt_index_name: opt_index_name.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableEngine {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, parameters } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            parameters: parameters.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableFactor {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
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
                let transformed = sqlparser::ast::TableFactor::Table {
                    name: name.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                    args: args.apply_transform(transformer)?,
                    with_hints: with_hints.apply_transform(transformer)?,
                    version: version.apply_transform(transformer)?,
                    with_ordinality: with_ordinality.apply_transform(transformer)?,
                    partitions: partitions.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::Derived { lateral, subquery, alias } => {
                let transformed = sqlparser::ast::TableFactor::Derived {
                    lateral: lateral.apply_transform(transformer)?,
                    subquery: subquery.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::TableFunction { expr, alias } => {
                let transformed = sqlparser::ast::TableFactor::TableFunction {
                    expr: expr.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::Function { lateral, name, args, alias } => {
                let transformed = sqlparser::ast::TableFactor::Function {
                    lateral: lateral.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    args: args.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::UNNEST {
                alias,
                array_exprs,
                with_offset,
                with_offset_alias,
                with_ordinality,
            } => {
                let transformed = sqlparser::ast::TableFactor::UNNEST {
                    alias: alias.apply_transform(transformer)?,
                    array_exprs: array_exprs.apply_transform(transformer)?,
                    with_offset: with_offset.apply_transform(transformer)?,
                    with_offset_alias: with_offset_alias.apply_transform(transformer)?,
                    with_ordinality: with_ordinality.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::JsonTable {
                json_expr,
                json_path,
                columns,
                alias,
            } => {
                let transformed = sqlparser::ast::TableFactor::JsonTable {
                    json_expr: json_expr.apply_transform(transformer)?,
                    json_path: json_path.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::NestedJoin { table_with_joins, alias } => {
                let transformed = sqlparser::ast::TableFactor::NestedJoin {
                    table_with_joins: table_with_joins.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::Pivot {
                table,
                aggregate_functions,
                value_column,
                value_source,
                default_on_null,
                alias,
            } => {
                let transformed = sqlparser::ast::TableFactor::Pivot {
                    table: table.apply_transform(transformer)?,
                    aggregate_functions: aggregate_functions
                        .apply_transform(transformer)?,
                    value_column: value_column.apply_transform(transformer)?,
                    value_source: value_source.apply_transform(transformer)?,
                    default_on_null: default_on_null.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableFactor::Unpivot {
                table,
                value,
                name,
                columns,
                alias,
            } => {
                let transformed = sqlparser::ast::TableFactor::Unpivot {
                    table: table.apply_transform(transformer)?,
                    value: value.apply_transform(transformer)?,
                    name: name.apply_transform(transformer)?,
                    columns: columns.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
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
                let transformed = sqlparser::ast::TableFactor::MatchRecognize {
                    table: table.apply_transform(transformer)?,
                    partition_by: partition_by.apply_transform(transformer)?,
                    order_by: order_by.apply_transform(transformer)?,
                    measures: measures.apply_transform(transformer)?,
                    rows_per_match: rows_per_match.apply_transform(transformer)?,
                    after_match_skip: after_match_skip.apply_transform(transformer)?,
                    pattern: pattern.apply_transform(transformer)?,
                    symbols: symbols.apply_transform(transformer)?,
                    alias: alias.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableFunctionArgs {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { args, settings } = self;
        let transformed = Self {
            args: args.apply_transform(transformer)?,
            settings: settings.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableOptionsClustered {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TableOptionsClustered::ColumnstoreIndex => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::TableOptionsClustered::ColumnstoreIndex,
                    )
            }
            sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(field0) => {
                let transformed = sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TableOptionsClustered::Index(field0) => {
                let transformed = sqlparser::ast::TableOptionsClustered::Index(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableVersion {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TableVersion::ForSystemTimeAsOf(field0) => {
                let transformed = sqlparser::ast::TableVersion::ForSystemTimeAsOf(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableWithJoins {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { relation, joins } = self;
        let transformed = Self {
            relation: relation.apply_transform(transformer)?,
            joins: joins.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Tag {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { key, value } = self;
        let transformed = Self {
            key: key.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TagsColumnOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { with, tags } = self;
        let transformed = Self {
            with: with.apply_transform(transformer)?,
            tags: tags.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TimezoneInfo {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TimezoneInfo::None => {
                transformer.transform(self, sqlparser::ast::TimezoneInfo::None)
            }
            sqlparser::ast::TimezoneInfo::WithTimeZone => {
                transformer.transform(self, sqlparser::ast::TimezoneInfo::WithTimeZone)
            }
            sqlparser::ast::TimezoneInfo::WithoutTimeZone => {
                transformer
                    .transform(self, sqlparser::ast::TimezoneInfo::WithoutTimeZone)
            }
            sqlparser::ast::TimezoneInfo::Tz => {
                transformer.transform(self, sqlparser::ast::TimezoneInfo::Tz)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Top {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { with_ties, percent, quantity } = self;
        let transformed = Self {
            with_ties: with_ties.apply_transform(transformer)?,
            percent: percent.apply_transform(transformer)?,
            quantity: quantity.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TopQuantity {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TopQuantity::Expr(field0) => {
                let transformed = sqlparser::ast::TopQuantity::Expr(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TopQuantity::Constant(field0) => {
                let transformed = sqlparser::ast::TopQuantity::Constant(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionAccessMode {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TransactionAccessMode::ReadOnly => {
                transformer
                    .transform(self, sqlparser::ast::TransactionAccessMode::ReadOnly)
            }
            sqlparser::ast::TransactionAccessMode::ReadWrite => {
                transformer
                    .transform(self, sqlparser::ast::TransactionAccessMode::ReadWrite)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionIsolationLevel {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TransactionIsolationLevel::ReadUncommitted => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::TransactionIsolationLevel::ReadUncommitted,
                    )
            }
            sqlparser::ast::TransactionIsolationLevel::ReadCommitted => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::TransactionIsolationLevel::ReadCommitted,
                    )
            }
            sqlparser::ast::TransactionIsolationLevel::RepeatableRead => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::TransactionIsolationLevel::RepeatableRead,
                    )
            }
            sqlparser::ast::TransactionIsolationLevel::Serializable => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::TransactionIsolationLevel::Serializable,
                    )
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionMode {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TransactionMode::AccessMode(field0) => {
                let transformed = sqlparser::ast::TransactionMode::AccessMode(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TransactionMode::IsolationLevel(field0) => {
                let transformed = sqlparser::ast::TransactionMode::IsolationLevel(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionModifier {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TransactionModifier::Deferred => {
                transformer
                    .transform(self, sqlparser::ast::TransactionModifier::Deferred)
            }
            sqlparser::ast::TransactionModifier::Immediate => {
                transformer
                    .transform(self, sqlparser::ast::TransactionModifier::Immediate)
            }
            sqlparser::ast::TransactionModifier::Exclusive => {
                transformer
                    .transform(self, sqlparser::ast::TransactionModifier::Exclusive)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerEvent {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TriggerEvent::Insert => {
                transformer.transform(self, sqlparser::ast::TriggerEvent::Insert)
            }
            sqlparser::ast::TriggerEvent::Update(field0) => {
                let transformed = sqlparser::ast::TriggerEvent::Update(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::TriggerEvent::Delete => {
                transformer.transform(self, sqlparser::ast::TriggerEvent::Delete)
            }
            sqlparser::ast::TriggerEvent::Truncate => {
                transformer.transform(self, sqlparser::ast::TriggerEvent::Truncate)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerExecBody {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { exec_type, func_desc } = self;
        let transformed = Self {
            exec_type: exec_type.apply_transform(transformer)?,
            func_desc: func_desc.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerExecBodyType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TriggerExecBodyType::Function => {
                transformer
                    .transform(self, sqlparser::ast::TriggerExecBodyType::Function)
            }
            sqlparser::ast::TriggerExecBodyType::Procedure => {
                transformer
                    .transform(self, sqlparser::ast::TriggerExecBodyType::Procedure)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerObject {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TriggerObject::Row => {
                transformer.transform(self, sqlparser::ast::TriggerObject::Row)
            }
            sqlparser::ast::TriggerObject::Statement => {
                transformer.transform(self, sqlparser::ast::TriggerObject::Statement)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerPeriod {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TriggerPeriod::After => {
                transformer.transform(self, sqlparser::ast::TriggerPeriod::After)
            }
            sqlparser::ast::TriggerPeriod::Before => {
                transformer.transform(self, sqlparser::ast::TriggerPeriod::Before)
            }
            sqlparser::ast::TriggerPeriod::InsteadOf => {
                transformer.transform(self, sqlparser::ast::TriggerPeriod::InsteadOf)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerReferencing {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { refer_type, is_as, transition_relation_name } = self;
        let transformed = Self {
            refer_type: refer_type.apply_transform(transformer)?,
            is_as: is_as.apply_transform(transformer)?,
            transition_relation_name: transition_relation_name
                .apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerReferencingType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TriggerReferencingType::OldTable => {
                transformer
                    .transform(self, sqlparser::ast::TriggerReferencingType::OldTable)
            }
            sqlparser::ast::TriggerReferencingType::NewTable => {
                transformer
                    .transform(self, sqlparser::ast::TriggerReferencingType::NewTable)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TrimWhereField {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TrimWhereField::Both => {
                transformer.transform(self, sqlparser::ast::TrimWhereField::Both)
            }
            sqlparser::ast::TrimWhereField::Leading => {
                transformer.transform(self, sqlparser::ast::TrimWhereField::Leading)
            }
            sqlparser::ast::TrimWhereField::Trailing => {
                transformer.transform(self, sqlparser::ast::TrimWhereField::Trailing)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateCascadeOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TruncateCascadeOption::Cascade => {
                transformer
                    .transform(self, sqlparser::ast::TruncateCascadeOption::Cascade)
            }
            sqlparser::ast::TruncateCascadeOption::Restrict => {
                transformer
                    .transform(self, sqlparser::ast::TruncateCascadeOption::Restrict)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateIdentityOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::TruncateIdentityOption::Restart => {
                transformer
                    .transform(self, sqlparser::ast::TruncateIdentityOption::Restart)
            }
            sqlparser::ast::TruncateIdentityOption::Continue => {
                transformer
                    .transform(self, sqlparser::ast::TruncateIdentityOption::Continue)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateTableTarget {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UnaryOperator {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::UnaryOperator::Plus => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::Plus)
            }
            sqlparser::ast::UnaryOperator::Minus => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::Minus)
            }
            sqlparser::ast::UnaryOperator::Not => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::Not)
            }
            sqlparser::ast::UnaryOperator::PGBitwiseNot => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::PGBitwiseNot)
            }
            sqlparser::ast::UnaryOperator::PGSquareRoot => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::PGSquareRoot)
            }
            sqlparser::ast::UnaryOperator::PGCubeRoot => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::PGCubeRoot)
            }
            sqlparser::ast::UnaryOperator::PGPostfixFactorial => {
                transformer
                    .transform(self, sqlparser::ast::UnaryOperator::PGPostfixFactorial)
            }
            sqlparser::ast::UnaryOperator::PGPrefixFactorial => {
                transformer
                    .transform(self, sqlparser::ast::UnaryOperator::PGPrefixFactorial)
            }
            sqlparser::ast::UnaryOperator::PGAbs => {
                transformer.transform(self, sqlparser::ast::UnaryOperator::PGAbs)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UnionField {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { field_name, field_type } = self;
        let transformed = Self {
            field_name: field_name.apply_transform(transformer)?,
            field_type: field_type.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Use {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Use::Catalog(field0) => {
                let transformed = sqlparser::ast::Use::Catalog(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Use::Schema(field0) => {
                let transformed = sqlparser::ast::Use::Schema(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Use::Database(field0) => {
                let transformed = sqlparser::ast::Use::Database(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Use::Warehouse(field0) => {
                let transformed = sqlparser::ast::Use::Warehouse(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Use::Object(field0) => {
                let transformed = sqlparser::ast::Use::Object(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Use::Default => {
                transformer.transform(self, sqlparser::ast::Use::Default)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::UserDefinedTypeCompositeAttributeDef {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, data_type, collation } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            data_type: data_type.apply_transform(transformer)?,
            collation: collation.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UserDefinedTypeRepresentation {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::UserDefinedTypeRepresentation::Composite { attributes } => {
                let transformed = sqlparser::ast::UserDefinedTypeRepresentation::Composite {
                    attributes: attributes.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::ast::UserDefinedTypeRepresentation::Enum { labels } => {
                let transformed = sqlparser::ast::UserDefinedTypeRepresentation::Enum {
                    labels: labels.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UtilityOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, arg } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            arg: arg.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Value {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::Value::Number(field0, field1) => {
                let transformed = sqlparser::ast::Value::Number(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::SingleQuotedString(field0) => {
                let transformed = sqlparser::ast::Value::SingleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::DollarQuotedString(field0) => {
                let transformed = sqlparser::ast::Value::DollarQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::TripleSingleQuotedString(field0) => {
                let transformed = sqlparser::ast::Value::TripleSingleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::TripleDoubleQuotedString(field0) => {
                let transformed = sqlparser::ast::Value::TripleDoubleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::EscapedStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::EscapedStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::UnicodeStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::UnicodeStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::SingleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::SingleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::DoubleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::DoubleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::SingleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::SingleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::DoubleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::DoubleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::NationalStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::NationalStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::HexStringLiteral(field0) => {
                let transformed = sqlparser::ast::Value::HexStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::DoubleQuotedString(field0) => {
                let transformed = sqlparser::ast::Value::DoubleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::Boolean(field0) => {
                let transformed = sqlparser::ast::Value::Boolean(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::Value::Null => {
                transformer.transform(self, sqlparser::ast::Value::Null)
            }
            sqlparser::ast::Value::Placeholder(field0) => {
                let transformed = sqlparser::ast::Value::Placeholder(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ValueTableMode {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::ValueTableMode::AsStruct => {
                transformer.transform(self, sqlparser::ast::ValueTableMode::AsStruct)
            }
            sqlparser::ast::ValueTableMode::AsValue => {
                transformer.transform(self, sqlparser::ast::ValueTableMode::AsValue)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Values {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { explicit_row, rows } = self;
        let transformed = Self {
            explicit_row: explicit_row.apply_transform(transformer)?,
            rows: rows.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ViewColumnDef {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { name, data_type, options } = self;
        let transformed = Self {
            name: name.apply_transform(transformer)?,
            data_type: data_type.apply_transform(transformer)?,
            options: options.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WildcardAdditionalOptions {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { opt_ilike, opt_exclude, opt_except, opt_replace, opt_rename } = self;
        let transformed = Self {
            opt_ilike: opt_ilike.apply_transform(transformer)?,
            opt_exclude: opt_exclude.apply_transform(transformer)?,
            opt_except: opt_except.apply_transform(transformer)?,
            opt_replace: opt_replace.apply_transform(transformer)?,
            opt_rename: opt_rename.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrame {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { units, start_bound, end_bound } = self;
        let transformed = Self {
            units: units.apply_transform(transformer)?,
            start_bound: start_bound.apply_transform(transformer)?,
            end_bound: end_bound.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrameBound {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::WindowFrameBound::CurrentRow => {
                transformer.transform(self, sqlparser::ast::WindowFrameBound::CurrentRow)
            }
            sqlparser::ast::WindowFrameBound::Preceding(field0) => {
                let transformed = sqlparser::ast::WindowFrameBound::Preceding(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::WindowFrameBound::Following(field0) => {
                let transformed = sqlparser::ast::WindowFrameBound::Following(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrameUnits {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::WindowFrameUnits::Rows => {
                transformer.transform(self, sqlparser::ast::WindowFrameUnits::Rows)
            }
            sqlparser::ast::WindowFrameUnits::Range => {
                transformer.transform(self, sqlparser::ast::WindowFrameUnits::Range)
            }
            sqlparser::ast::WindowFrameUnits::Groups => {
                transformer.transform(self, sqlparser::ast::WindowFrameUnits::Groups)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowSpec {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { window_name, partition_by, order_by, window_frame } = self;
        let transformed = Self {
            window_name: window_name.apply_transform(transformer)?,
            partition_by: partition_by.apply_transform(transformer)?,
            order_by: order_by.apply_transform(transformer)?,
            window_frame: window_frame.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::WindowType::WindowSpec(field0) => {
                let transformed = sqlparser::ast::WindowType::WindowSpec(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::ast::WindowType::NamedWindow(field0) => {
                let transformed = sqlparser::ast::WindowType::NamedWindow(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::With {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { recursive, cte_tables } = self;
        let transformed = Self {
            recursive: recursive.apply_transform(transformer)?,
            cte_tables: cte_tables.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WithFill {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { from, to, step } = self;
        let transformed = Self {
            from: from.apply_transform(transformer)?,
            to: to.apply_transform(transformer)?,
            step: step.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOption {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { option_name, option_type, value } = self;
        let transformed = Self {
            option_name: option_name.apply_transform(transformer)?,
            option_type: option_type.apply_transform(transformer)?,
            value: value.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING,
                    )
            }
            sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN,
                    )
            }
            sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM => {
                transformer
                    .transform(
                        self,
                        sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM,
                    )
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptions {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { options } = self;
        let transformed = Self {
            options: options.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { alias, file_col_num, element, item_as } = self;
        let transformed = Self {
            alias: alias.apply_transform(transformer)?,
            file_col_num: file_col_num.apply_transform(transformer)?,
            element: element.apply_transform(transformer)?,
            item_as: item_as.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::StageParamsObject {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { url, encryption, endpoint, storage_integration, credentials } = self;
        let transformed = Self {
            url: url.apply_transform(transformer)?,
            encryption: encryption.apply_transform(transformer)?,
            endpoint: endpoint.apply_transform(transformer)?,
            storage_integration: storage_integration.apply_transform(transformer)?,
            credentials: credentials.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::keywords::Keyword {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::keywords::Keyword::NoKeyword => {
                transformer.transform(self, sqlparser::keywords::Keyword::NoKeyword)
            }
            sqlparser::keywords::Keyword::ABORT => {
                transformer.transform(self, sqlparser::keywords::Keyword::ABORT)
            }
            sqlparser::keywords::Keyword::ABS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ABS)
            }
            sqlparser::keywords::Keyword::ABSOLUTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ABSOLUTE)
            }
            sqlparser::keywords::Keyword::ACCESS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ACCESS)
            }
            sqlparser::keywords::Keyword::ACTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::ACTION)
            }
            sqlparser::keywords::Keyword::ADD => {
                transformer.transform(self, sqlparser::keywords::Keyword::ADD)
            }
            sqlparser::keywords::Keyword::ADMIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::ADMIN)
            }
            sqlparser::keywords::Keyword::AFTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::AFTER)
            }
            sqlparser::keywords::Keyword::AGAINST => {
                transformer.transform(self, sqlparser::keywords::Keyword::AGAINST)
            }
            sqlparser::keywords::Keyword::AGGREGATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::AGGREGATION)
            }
            sqlparser::keywords::Keyword::ALIAS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ALIAS)
            }
            sqlparser::keywords::Keyword::ALL => {
                transformer.transform(self, sqlparser::keywords::Keyword::ALL)
            }
            sqlparser::keywords::Keyword::ALLOCATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ALLOCATE)
            }
            sqlparser::keywords::Keyword::ALTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::ALTER)
            }
            sqlparser::keywords::Keyword::ALWAYS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ALWAYS)
            }
            sqlparser::keywords::Keyword::ANALYZE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ANALYZE)
            }
            sqlparser::keywords::Keyword::AND => {
                transformer.transform(self, sqlparser::keywords::Keyword::AND)
            }
            sqlparser::keywords::Keyword::ANTI => {
                transformer.transform(self, sqlparser::keywords::Keyword::ANTI)
            }
            sqlparser::keywords::Keyword::ANY => {
                transformer.transform(self, sqlparser::keywords::Keyword::ANY)
            }
            sqlparser::keywords::Keyword::APPLY => {
                transformer.transform(self, sqlparser::keywords::Keyword::APPLY)
            }
            sqlparser::keywords::Keyword::ARCHIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ARCHIVE)
            }
            sqlparser::keywords::Keyword::ARE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ARE)
            }
            sqlparser::keywords::Keyword::ARRAY => {
                transformer.transform(self, sqlparser::keywords::Keyword::ARRAY)
            }
            sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY)
            }
            sqlparser::keywords::Keyword::AS => {
                transformer.transform(self, sqlparser::keywords::Keyword::AS)
            }
            sqlparser::keywords::Keyword::ASC => {
                transformer.transform(self, sqlparser::keywords::Keyword::ASC)
            }
            sqlparser::keywords::Keyword::ASENSITIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ASENSITIVE)
            }
            sqlparser::keywords::Keyword::ASOF => {
                transformer.transform(self, sqlparser::keywords::Keyword::ASOF)
            }
            sqlparser::keywords::Keyword::ASSERT => {
                transformer.transform(self, sqlparser::keywords::Keyword::ASSERT)
            }
            sqlparser::keywords::Keyword::ASYMMETRIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::ASYMMETRIC)
            }
            sqlparser::keywords::Keyword::AT => {
                transformer.transform(self, sqlparser::keywords::Keyword::AT)
            }
            sqlparser::keywords::Keyword::ATOMIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::ATOMIC)
            }
            sqlparser::keywords::Keyword::ATTACH => {
                transformer.transform(self, sqlparser::keywords::Keyword::ATTACH)
            }
            sqlparser::keywords::Keyword::AUTHORIZATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::AUTHORIZATION)
            }
            sqlparser::keywords::Keyword::AUTO => {
                transformer.transform(self, sqlparser::keywords::Keyword::AUTO)
            }
            sqlparser::keywords::Keyword::AUTOINCREMENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::AUTOINCREMENT)
            }
            sqlparser::keywords::Keyword::AUTO_INCREMENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::AUTO_INCREMENT)
            }
            sqlparser::keywords::Keyword::AVG => {
                transformer.transform(self, sqlparser::keywords::Keyword::AVG)
            }
            sqlparser::keywords::Keyword::AVRO => {
                transformer.transform(self, sqlparser::keywords::Keyword::AVRO)
            }
            sqlparser::keywords::Keyword::BACKWARD => {
                transformer.transform(self, sqlparser::keywords::Keyword::BACKWARD)
            }
            sqlparser::keywords::Keyword::BASE64 => {
                transformer.transform(self, sqlparser::keywords::Keyword::BASE64)
            }
            sqlparser::keywords::Keyword::BEFORE => {
                transformer.transform(self, sqlparser::keywords::Keyword::BEFORE)
            }
            sqlparser::keywords::Keyword::BEGIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::BEGIN)
            }
            sqlparser::keywords::Keyword::BEGIN_FRAME => {
                transformer.transform(self, sqlparser::keywords::Keyword::BEGIN_FRAME)
            }
            sqlparser::keywords::Keyword::BEGIN_PARTITION => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::BEGIN_PARTITION)
            }
            sqlparser::keywords::Keyword::BETWEEN => {
                transformer.transform(self, sqlparser::keywords::Keyword::BETWEEN)
            }
            sqlparser::keywords::Keyword::BIGDECIMAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::BIGDECIMAL)
            }
            sqlparser::keywords::Keyword::BIGINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::BIGINT)
            }
            sqlparser::keywords::Keyword::BIGNUMERIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::BIGNUMERIC)
            }
            sqlparser::keywords::Keyword::BINARY => {
                transformer.transform(self, sqlparser::keywords::Keyword::BINARY)
            }
            sqlparser::keywords::Keyword::BINDING => {
                transformer.transform(self, sqlparser::keywords::Keyword::BINDING)
            }
            sqlparser::keywords::Keyword::BLOB => {
                transformer.transform(self, sqlparser::keywords::Keyword::BLOB)
            }
            sqlparser::keywords::Keyword::BLOOMFILTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::BLOOMFILTER)
            }
            sqlparser::keywords::Keyword::BOOL => {
                transformer.transform(self, sqlparser::keywords::Keyword::BOOL)
            }
            sqlparser::keywords::Keyword::BOOLEAN => {
                transformer.transform(self, sqlparser::keywords::Keyword::BOOLEAN)
            }
            sqlparser::keywords::Keyword::BOTH => {
                transformer.transform(self, sqlparser::keywords::Keyword::BOTH)
            }
            sqlparser::keywords::Keyword::BROWSE => {
                transformer.transform(self, sqlparser::keywords::Keyword::BROWSE)
            }
            sqlparser::keywords::Keyword::BTREE => {
                transformer.transform(self, sqlparser::keywords::Keyword::BTREE)
            }
            sqlparser::keywords::Keyword::BUCKETS => {
                transformer.transform(self, sqlparser::keywords::Keyword::BUCKETS)
            }
            sqlparser::keywords::Keyword::BY => {
                transformer.transform(self, sqlparser::keywords::Keyword::BY)
            }
            sqlparser::keywords::Keyword::BYPASSRLS => {
                transformer.transform(self, sqlparser::keywords::Keyword::BYPASSRLS)
            }
            sqlparser::keywords::Keyword::BYTEA => {
                transformer.transform(self, sqlparser::keywords::Keyword::BYTEA)
            }
            sqlparser::keywords::Keyword::BYTES => {
                transformer.transform(self, sqlparser::keywords::Keyword::BYTES)
            }
            sqlparser::keywords::Keyword::CACHE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CACHE)
            }
            sqlparser::keywords::Keyword::CALL => {
                transformer.transform(self, sqlparser::keywords::Keyword::CALL)
            }
            sqlparser::keywords::Keyword::CALLED => {
                transformer.transform(self, sqlparser::keywords::Keyword::CALLED)
            }
            sqlparser::keywords::Keyword::CARDINALITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::CARDINALITY)
            }
            sqlparser::keywords::Keyword::CASCADE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CASCADE)
            }
            sqlparser::keywords::Keyword::CASCADED => {
                transformer.transform(self, sqlparser::keywords::Keyword::CASCADED)
            }
            sqlparser::keywords::Keyword::CASE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CASE)
            }
            sqlparser::keywords::Keyword::CAST => {
                transformer.transform(self, sqlparser::keywords::Keyword::CAST)
            }
            sqlparser::keywords::Keyword::CATALOG => {
                transformer.transform(self, sqlparser::keywords::Keyword::CATALOG)
            }
            sqlparser::keywords::Keyword::CEIL => {
                transformer.transform(self, sqlparser::keywords::Keyword::CEIL)
            }
            sqlparser::keywords::Keyword::CEILING => {
                transformer.transform(self, sqlparser::keywords::Keyword::CEILING)
            }
            sqlparser::keywords::Keyword::CENTURY => {
                transformer.transform(self, sqlparser::keywords::Keyword::CENTURY)
            }
            sqlparser::keywords::Keyword::CHAIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHAIN)
            }
            sqlparser::keywords::Keyword::CHANGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHANGE)
            }
            sqlparser::keywords::Keyword::CHANGE_TRACKING => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::CHANGE_TRACKING)
            }
            sqlparser::keywords::Keyword::CHANNEL => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHANNEL)
            }
            sqlparser::keywords::Keyword::CHAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHAR)
            }
            sqlparser::keywords::Keyword::CHARACTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHARACTER)
            }
            sqlparser::keywords::Keyword::CHARACTERS => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHARACTERS)
            }
            sqlparser::keywords::Keyword::CHARACTER_LENGTH => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::CHARACTER_LENGTH)
            }
            sqlparser::keywords::Keyword::CHARSET => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHARSET)
            }
            sqlparser::keywords::Keyword::CHAR_LENGTH => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHAR_LENGTH)
            }
            sqlparser::keywords::Keyword::CHECK => {
                transformer.transform(self, sqlparser::keywords::Keyword::CHECK)
            }
            sqlparser::keywords::Keyword::CLEAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::CLEAR)
            }
            sqlparser::keywords::Keyword::CLOB => {
                transformer.transform(self, sqlparser::keywords::Keyword::CLOB)
            }
            sqlparser::keywords::Keyword::CLONE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CLONE)
            }
            sqlparser::keywords::Keyword::CLOSE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CLOSE)
            }
            sqlparser::keywords::Keyword::CLUSTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::CLUSTER)
            }
            sqlparser::keywords::Keyword::CLUSTERED => {
                transformer.transform(self, sqlparser::keywords::Keyword::CLUSTERED)
            }
            sqlparser::keywords::Keyword::COALESCE => {
                transformer.transform(self, sqlparser::keywords::Keyword::COALESCE)
            }
            sqlparser::keywords::Keyword::COLLATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLLATE)
            }
            sqlparser::keywords::Keyword::COLLATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLLATION)
            }
            sqlparser::keywords::Keyword::COLLECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLLECT)
            }
            sqlparser::keywords::Keyword::COLLECTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLLECTION)
            }
            sqlparser::keywords::Keyword::COLUMN => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLUMN)
            }
            sqlparser::keywords::Keyword::COLUMNS => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLUMNS)
            }
            sqlparser::keywords::Keyword::COLUMNSTORE => {
                transformer.transform(self, sqlparser::keywords::Keyword::COLUMNSTORE)
            }
            sqlparser::keywords::Keyword::COMMENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::COMMENT)
            }
            sqlparser::keywords::Keyword::COMMIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::COMMIT)
            }
            sqlparser::keywords::Keyword::COMMITTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::COMMITTED)
            }
            sqlparser::keywords::Keyword::COMPRESSION => {
                transformer.transform(self, sqlparser::keywords::Keyword::COMPRESSION)
            }
            sqlparser::keywords::Keyword::COMPUTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::COMPUTE)
            }
            sqlparser::keywords::Keyword::CONCURRENTLY => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONCURRENTLY)
            }
            sqlparser::keywords::Keyword::CONDITION => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONDITION)
            }
            sqlparser::keywords::Keyword::CONFLICT => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONFLICT)
            }
            sqlparser::keywords::Keyword::CONNECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONNECT)
            }
            sqlparser::keywords::Keyword::CONNECTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONNECTION)
            }
            sqlparser::keywords::Keyword::CONSTRAINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONSTRAINT)
            }
            sqlparser::keywords::Keyword::CONTAINS => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONTAINS)
            }
            sqlparser::keywords::Keyword::CONTINUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONTINUE)
            }
            sqlparser::keywords::Keyword::CONVERT => {
                transformer.transform(self, sqlparser::keywords::Keyword::CONVERT)
            }
            sqlparser::keywords::Keyword::COPY => {
                transformer.transform(self, sqlparser::keywords::Keyword::COPY)
            }
            sqlparser::keywords::Keyword::COPY_OPTIONS => {
                transformer.transform(self, sqlparser::keywords::Keyword::COPY_OPTIONS)
            }
            sqlparser::keywords::Keyword::CORR => {
                transformer.transform(self, sqlparser::keywords::Keyword::CORR)
            }
            sqlparser::keywords::Keyword::CORRESPONDING => {
                transformer.transform(self, sqlparser::keywords::Keyword::CORRESPONDING)
            }
            sqlparser::keywords::Keyword::COUNT => {
                transformer.transform(self, sqlparser::keywords::Keyword::COUNT)
            }
            sqlparser::keywords::Keyword::COVAR_POP => {
                transformer.transform(self, sqlparser::keywords::Keyword::COVAR_POP)
            }
            sqlparser::keywords::Keyword::COVAR_SAMP => {
                transformer.transform(self, sqlparser::keywords::Keyword::COVAR_SAMP)
            }
            sqlparser::keywords::Keyword::CREATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CREATE)
            }
            sqlparser::keywords::Keyword::CREATEDB => {
                transformer.transform(self, sqlparser::keywords::Keyword::CREATEDB)
            }
            sqlparser::keywords::Keyword::CREATEROLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CREATEROLE)
            }
            sqlparser::keywords::Keyword::CREDENTIALS => {
                transformer.transform(self, sqlparser::keywords::Keyword::CREDENTIALS)
            }
            sqlparser::keywords::Keyword::CROSS => {
                transformer.transform(self, sqlparser::keywords::Keyword::CROSS)
            }
            sqlparser::keywords::Keyword::CSV => {
                transformer.transform(self, sqlparser::keywords::Keyword::CSV)
            }
            sqlparser::keywords::Keyword::CUBE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CUBE)
            }
            sqlparser::keywords::Keyword::CUME_DIST => {
                transformer.transform(self, sqlparser::keywords::Keyword::CUME_DIST)
            }
            sqlparser::keywords::Keyword::CURRENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT)
            }
            sqlparser::keywords::Keyword::CURRENT_CATALOG => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::CURRENT_CATALOG)
            }
            sqlparser::keywords::Keyword::CURRENT_DATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_DATE)
            }
            sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP => {
                transformer
                    .transform(
                        self,
                        sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP,
                    )
            }
            sqlparser::keywords::Keyword::CURRENT_PATH => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_PATH)
            }
            sqlparser::keywords::Keyword::CURRENT_ROLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_ROLE)
            }
            sqlparser::keywords::Keyword::CURRENT_ROW => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_ROW)
            }
            sqlparser::keywords::Keyword::CURRENT_SCHEMA => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_SCHEMA)
            }
            sqlparser::keywords::Keyword::CURRENT_TIME => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_TIME)
            }
            sqlparser::keywords::Keyword::CURRENT_TIMESTAMP => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::CURRENT_TIMESTAMP)
            }
            sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE => {
                transformer
                    .transform(
                        self,
                        sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE,
                    )
            }
            sqlparser::keywords::Keyword::CURRENT_USER => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURRENT_USER)
            }
            sqlparser::keywords::Keyword::CURSOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::CURSOR)
            }
            sqlparser::keywords::Keyword::CYCLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::CYCLE)
            }
            sqlparser::keywords::Keyword::DATA => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATA)
            }
            sqlparser::keywords::Keyword::DATABASE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATABASE)
            }
            sqlparser::keywords::Keyword::DATABASES => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATABASES)
            }
            sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS => {
                transformer
                    .transform(
                        self,
                        sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS,
                    )
            }
            sqlparser::keywords::Keyword::DATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATE)
            }
            sqlparser::keywords::Keyword::DATE32 => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATE32)
            }
            sqlparser::keywords::Keyword::DATETIME => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATETIME)
            }
            sqlparser::keywords::Keyword::DATETIME64 => {
                transformer.transform(self, sqlparser::keywords::Keyword::DATETIME64)
            }
            sqlparser::keywords::Keyword::DAY => {
                transformer.transform(self, sqlparser::keywords::Keyword::DAY)
            }
            sqlparser::keywords::Keyword::DAYOFWEEK => {
                transformer.transform(self, sqlparser::keywords::Keyword::DAYOFWEEK)
            }
            sqlparser::keywords::Keyword::DAYOFYEAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::DAYOFYEAR)
            }
            sqlparser::keywords::Keyword::DEALLOCATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEALLOCATE)
            }
            sqlparser::keywords::Keyword::DEC => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEC)
            }
            sqlparser::keywords::Keyword::DECADE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DECADE)
            }
            sqlparser::keywords::Keyword::DECIMAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::DECIMAL)
            }
            sqlparser::keywords::Keyword::DECLARE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DECLARE)
            }
            sqlparser::keywords::Keyword::DEDUPLICATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEDUPLICATE)
            }
            sqlparser::keywords::Keyword::DEFAULT => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEFAULT)
            }
            sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION)
            }
            sqlparser::keywords::Keyword::DEFERRABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEFERRABLE)
            }
            sqlparser::keywords::Keyword::DEFERRED => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEFERRED)
            }
            sqlparser::keywords::Keyword::DEFINE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEFINE)
            }
            sqlparser::keywords::Keyword::DEFINED => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEFINED)
            }
            sqlparser::keywords::Keyword::DELAYED => {
                transformer.transform(self, sqlparser::keywords::Keyword::DELAYED)
            }
            sqlparser::keywords::Keyword::DELETE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DELETE)
            }
            sqlparser::keywords::Keyword::DELIMITED => {
                transformer.transform(self, sqlparser::keywords::Keyword::DELIMITED)
            }
            sqlparser::keywords::Keyword::DELIMITER => {
                transformer.transform(self, sqlparser::keywords::Keyword::DELIMITER)
            }
            sqlparser::keywords::Keyword::DELTA => {
                transformer.transform(self, sqlparser::keywords::Keyword::DELTA)
            }
            sqlparser::keywords::Keyword::DENSE_RANK => {
                transformer.transform(self, sqlparser::keywords::Keyword::DENSE_RANK)
            }
            sqlparser::keywords::Keyword::DEREF => {
                transformer.transform(self, sqlparser::keywords::Keyword::DEREF)
            }
            sqlparser::keywords::Keyword::DESC => {
                transformer.transform(self, sqlparser::keywords::Keyword::DESC)
            }
            sqlparser::keywords::Keyword::DESCRIBE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DESCRIBE)
            }
            sqlparser::keywords::Keyword::DETACH => {
                transformer.transform(self, sqlparser::keywords::Keyword::DETACH)
            }
            sqlparser::keywords::Keyword::DETAIL => {
                transformer.transform(self, sqlparser::keywords::Keyword::DETAIL)
            }
            sqlparser::keywords::Keyword::DETERMINISTIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::DETERMINISTIC)
            }
            sqlparser::keywords::Keyword::DIRECTORY => {
                transformer.transform(self, sqlparser::keywords::Keyword::DIRECTORY)
            }
            sqlparser::keywords::Keyword::DISABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DISABLE)
            }
            sqlparser::keywords::Keyword::DISCARD => {
                transformer.transform(self, sqlparser::keywords::Keyword::DISCARD)
            }
            sqlparser::keywords::Keyword::DISCONNECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::DISCONNECT)
            }
            sqlparser::keywords::Keyword::DISTINCT => {
                transformer.transform(self, sqlparser::keywords::Keyword::DISTINCT)
            }
            sqlparser::keywords::Keyword::DISTRIBUTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DISTRIBUTE)
            }
            sqlparser::keywords::Keyword::DIV => {
                transformer.transform(self, sqlparser::keywords::Keyword::DIV)
            }
            sqlparser::keywords::Keyword::DO => {
                transformer.transform(self, sqlparser::keywords::Keyword::DO)
            }
            sqlparser::keywords::Keyword::DOUBLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DOUBLE)
            }
            sqlparser::keywords::Keyword::DOW => {
                transformer.transform(self, sqlparser::keywords::Keyword::DOW)
            }
            sqlparser::keywords::Keyword::DOY => {
                transformer.transform(self, sqlparser::keywords::Keyword::DOY)
            }
            sqlparser::keywords::Keyword::DROP => {
                transformer.transform(self, sqlparser::keywords::Keyword::DROP)
            }
            sqlparser::keywords::Keyword::DRY => {
                transformer.transform(self, sqlparser::keywords::Keyword::DRY)
            }
            sqlparser::keywords::Keyword::DUPLICATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::DUPLICATE)
            }
            sqlparser::keywords::Keyword::DYNAMIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::DYNAMIC)
            }
            sqlparser::keywords::Keyword::EACH => {
                transformer.transform(self, sqlparser::keywords::Keyword::EACH)
            }
            sqlparser::keywords::Keyword::ELEMENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::ELEMENT)
            }
            sqlparser::keywords::Keyword::ELEMENTS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ELEMENTS)
            }
            sqlparser::keywords::Keyword::ELSE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ELSE)
            }
            sqlparser::keywords::Keyword::EMPTY => {
                transformer.transform(self, sqlparser::keywords::Keyword::EMPTY)
            }
            sqlparser::keywords::Keyword::ENABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENABLE)
            }
            sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION => {
                transformer
                    .transform(
                        self,
                        sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION,
                    )
            }
            sqlparser::keywords::Keyword::ENCODING => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENCODING)
            }
            sqlparser::keywords::Keyword::ENCRYPTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENCRYPTION)
            }
            sqlparser::keywords::Keyword::END => {
                transformer.transform(self, sqlparser::keywords::Keyword::END)
            }
            sqlparser::keywords::Keyword::END_EXEC => {
                transformer.transform(self, sqlparser::keywords::Keyword::END_EXEC)
            }
            sqlparser::keywords::Keyword::ENDPOINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENDPOINT)
            }
            sqlparser::keywords::Keyword::END_FRAME => {
                transformer.transform(self, sqlparser::keywords::Keyword::END_FRAME)
            }
            sqlparser::keywords::Keyword::END_PARTITION => {
                transformer.transform(self, sqlparser::keywords::Keyword::END_PARTITION)
            }
            sqlparser::keywords::Keyword::ENFORCED => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENFORCED)
            }
            sqlparser::keywords::Keyword::ENGINE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENGINE)
            }
            sqlparser::keywords::Keyword::ENUM => {
                transformer.transform(self, sqlparser::keywords::Keyword::ENUM)
            }
            sqlparser::keywords::Keyword::EPHEMERAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::EPHEMERAL)
            }
            sqlparser::keywords::Keyword::EPOCH => {
                transformer.transform(self, sqlparser::keywords::Keyword::EPOCH)
            }
            sqlparser::keywords::Keyword::EQUALS => {
                transformer.transform(self, sqlparser::keywords::Keyword::EQUALS)
            }
            sqlparser::keywords::Keyword::ERROR => {
                transformer.transform(self, sqlparser::keywords::Keyword::ERROR)
            }
            sqlparser::keywords::Keyword::ESCAPE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ESCAPE)
            }
            sqlparser::keywords::Keyword::ESCAPED => {
                transformer.transform(self, sqlparser::keywords::Keyword::ESCAPED)
            }
            sqlparser::keywords::Keyword::EVENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::EVENT)
            }
            sqlparser::keywords::Keyword::EVERY => {
                transformer.transform(self, sqlparser::keywords::Keyword::EVERY)
            }
            sqlparser::keywords::Keyword::EXCEPT => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXCEPT)
            }
            sqlparser::keywords::Keyword::EXCEPTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXCEPTION)
            }
            sqlparser::keywords::Keyword::EXCLUDE => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXCLUDE)
            }
            sqlparser::keywords::Keyword::EXCLUSIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXCLUSIVE)
            }
            sqlparser::keywords::Keyword::EXEC => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXEC)
            }
            sqlparser::keywords::Keyword::EXECUTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXECUTE)
            }
            sqlparser::keywords::Keyword::EXISTS => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXISTS)
            }
            sqlparser::keywords::Keyword::EXP => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXP)
            }
            sqlparser::keywords::Keyword::EXPANSION => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXPANSION)
            }
            sqlparser::keywords::Keyword::EXPLAIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXPLAIN)
            }
            sqlparser::keywords::Keyword::EXPLICIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXPLICIT)
            }
            sqlparser::keywords::Keyword::EXPORT => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXPORT)
            }
            sqlparser::keywords::Keyword::EXTENDED => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXTENDED)
            }
            sqlparser::keywords::Keyword::EXTENSION => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXTENSION)
            }
            sqlparser::keywords::Keyword::EXTERNAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXTERNAL)
            }
            sqlparser::keywords::Keyword::EXTRACT => {
                transformer.transform(self, sqlparser::keywords::Keyword::EXTRACT)
            }
            sqlparser::keywords::Keyword::FAIL => {
                transformer.transform(self, sqlparser::keywords::Keyword::FAIL)
            }
            sqlparser::keywords::Keyword::FALSE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FALSE)
            }
            sqlparser::keywords::Keyword::FETCH => {
                transformer.transform(self, sqlparser::keywords::Keyword::FETCH)
            }
            sqlparser::keywords::Keyword::FIELDS => {
                transformer.transform(self, sqlparser::keywords::Keyword::FIELDS)
            }
            sqlparser::keywords::Keyword::FILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FILE)
            }
            sqlparser::keywords::Keyword::FILES => {
                transformer.transform(self, sqlparser::keywords::Keyword::FILES)
            }
            sqlparser::keywords::Keyword::FILE_FORMAT => {
                transformer.transform(self, sqlparser::keywords::Keyword::FILE_FORMAT)
            }
            sqlparser::keywords::Keyword::FILL => {
                transformer.transform(self, sqlparser::keywords::Keyword::FILL)
            }
            sqlparser::keywords::Keyword::FILTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::FILTER)
            }
            sqlparser::keywords::Keyword::FINAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::FINAL)
            }
            sqlparser::keywords::Keyword::FIRST => {
                transformer.transform(self, sqlparser::keywords::Keyword::FIRST)
            }
            sqlparser::keywords::Keyword::FIRST_VALUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FIRST_VALUE)
            }
            sqlparser::keywords::Keyword::FIXEDSTRING => {
                transformer.transform(self, sqlparser::keywords::Keyword::FIXEDSTRING)
            }
            sqlparser::keywords::Keyword::FLOAT => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLOAT)
            }
            sqlparser::keywords::Keyword::FLOAT32 => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLOAT32)
            }
            sqlparser::keywords::Keyword::FLOAT4 => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLOAT4)
            }
            sqlparser::keywords::Keyword::FLOAT64 => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLOAT64)
            }
            sqlparser::keywords::Keyword::FLOAT8 => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLOAT8)
            }
            sqlparser::keywords::Keyword::FLOOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLOOR)
            }
            sqlparser::keywords::Keyword::FLUSH => {
                transformer.transform(self, sqlparser::keywords::Keyword::FLUSH)
            }
            sqlparser::keywords::Keyword::FOLLOWING => {
                transformer.transform(self, sqlparser::keywords::Keyword::FOLLOWING)
            }
            sqlparser::keywords::Keyword::FOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::FOR)
            }
            sqlparser::keywords::Keyword::FORCE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORCE)
            }
            sqlparser::keywords::Keyword::FORCE_NOT_NULL => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORCE_NOT_NULL)
            }
            sqlparser::keywords::Keyword::FORCE_NULL => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORCE_NULL)
            }
            sqlparser::keywords::Keyword::FORCE_QUOTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORCE_QUOTE)
            }
            sqlparser::keywords::Keyword::FOREIGN => {
                transformer.transform(self, sqlparser::keywords::Keyword::FOREIGN)
            }
            sqlparser::keywords::Keyword::FORMAT => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORMAT)
            }
            sqlparser::keywords::Keyword::FORMATTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORMATTED)
            }
            sqlparser::keywords::Keyword::FORWARD => {
                transformer.transform(self, sqlparser::keywords::Keyword::FORWARD)
            }
            sqlparser::keywords::Keyword::FRAME_ROW => {
                transformer.transform(self, sqlparser::keywords::Keyword::FRAME_ROW)
            }
            sqlparser::keywords::Keyword::FREE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FREE)
            }
            sqlparser::keywords::Keyword::FREEZE => {
                transformer.transform(self, sqlparser::keywords::Keyword::FREEZE)
            }
            sqlparser::keywords::Keyword::FROM => {
                transformer.transform(self, sqlparser::keywords::Keyword::FROM)
            }
            sqlparser::keywords::Keyword::FSCK => {
                transformer.transform(self, sqlparser::keywords::Keyword::FSCK)
            }
            sqlparser::keywords::Keyword::FULL => {
                transformer.transform(self, sqlparser::keywords::Keyword::FULL)
            }
            sqlparser::keywords::Keyword::FULLTEXT => {
                transformer.transform(self, sqlparser::keywords::Keyword::FULLTEXT)
            }
            sqlparser::keywords::Keyword::FUNCTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::FUNCTION)
            }
            sqlparser::keywords::Keyword::FUNCTIONS => {
                transformer.transform(self, sqlparser::keywords::Keyword::FUNCTIONS)
            }
            sqlparser::keywords::Keyword::FUSION => {
                transformer.transform(self, sqlparser::keywords::Keyword::FUSION)
            }
            sqlparser::keywords::Keyword::GENERAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::GENERAL)
            }
            sqlparser::keywords::Keyword::GENERATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::GENERATE)
            }
            sqlparser::keywords::Keyword::GENERATED => {
                transformer.transform(self, sqlparser::keywords::Keyword::GENERATED)
            }
            sqlparser::keywords::Keyword::GEOGRAPHY => {
                transformer.transform(self, sqlparser::keywords::Keyword::GEOGRAPHY)
            }
            sqlparser::keywords::Keyword::GET => {
                transformer.transform(self, sqlparser::keywords::Keyword::GET)
            }
            sqlparser::keywords::Keyword::GLOBAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::GLOBAL)
            }
            sqlparser::keywords::Keyword::GRANT => {
                transformer.transform(self, sqlparser::keywords::Keyword::GRANT)
            }
            sqlparser::keywords::Keyword::GRANTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::GRANTED)
            }
            sqlparser::keywords::Keyword::GRANTS => {
                transformer.transform(self, sqlparser::keywords::Keyword::GRANTS)
            }
            sqlparser::keywords::Keyword::GRAPHVIZ => {
                transformer.transform(self, sqlparser::keywords::Keyword::GRAPHVIZ)
            }
            sqlparser::keywords::Keyword::GROUP => {
                transformer.transform(self, sqlparser::keywords::Keyword::GROUP)
            }
            sqlparser::keywords::Keyword::GROUPING => {
                transformer.transform(self, sqlparser::keywords::Keyword::GROUPING)
            }
            sqlparser::keywords::Keyword::GROUPS => {
                transformer.transform(self, sqlparser::keywords::Keyword::GROUPS)
            }
            sqlparser::keywords::Keyword::HASH => {
                transformer.transform(self, sqlparser::keywords::Keyword::HASH)
            }
            sqlparser::keywords::Keyword::HAVING => {
                transformer.transform(self, sqlparser::keywords::Keyword::HAVING)
            }
            sqlparser::keywords::Keyword::HEADER => {
                transformer.transform(self, sqlparser::keywords::Keyword::HEADER)
            }
            sqlparser::keywords::Keyword::HEAP => {
                transformer.transform(self, sqlparser::keywords::Keyword::HEAP)
            }
            sqlparser::keywords::Keyword::HIGH_PRIORITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::HIGH_PRIORITY)
            }
            sqlparser::keywords::Keyword::HISTORY => {
                transformer.transform(self, sqlparser::keywords::Keyword::HISTORY)
            }
            sqlparser::keywords::Keyword::HIVEVAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::HIVEVAR)
            }
            sqlparser::keywords::Keyword::HOLD => {
                transformer.transform(self, sqlparser::keywords::Keyword::HOLD)
            }
            sqlparser::keywords::Keyword::HOSTS => {
                transformer.transform(self, sqlparser::keywords::Keyword::HOSTS)
            }
            sqlparser::keywords::Keyword::HOUR => {
                transformer.transform(self, sqlparser::keywords::Keyword::HOUR)
            }
            sqlparser::keywords::Keyword::HOURS => {
                transformer.transform(self, sqlparser::keywords::Keyword::HOURS)
            }
            sqlparser::keywords::Keyword::ID => {
                transformer.transform(self, sqlparser::keywords::Keyword::ID)
            }
            sqlparser::keywords::Keyword::IDENTITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::IDENTITY)
            }
            sqlparser::keywords::Keyword::IF => {
                transformer.transform(self, sqlparser::keywords::Keyword::IF)
            }
            sqlparser::keywords::Keyword::IGNORE => {
                transformer.transform(self, sqlparser::keywords::Keyword::IGNORE)
            }
            sqlparser::keywords::Keyword::ILIKE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ILIKE)
            }
            sqlparser::keywords::Keyword::IMMEDIATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::IMMEDIATE)
            }
            sqlparser::keywords::Keyword::IMMUTABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::IMMUTABLE)
            }
            sqlparser::keywords::Keyword::IN => {
                transformer.transform(self, sqlparser::keywords::Keyword::IN)
            }
            sqlparser::keywords::Keyword::INCLUDE => {
                transformer.transform(self, sqlparser::keywords::Keyword::INCLUDE)
            }
            sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES)
            }
            sqlparser::keywords::Keyword::INCREMENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INCREMENT)
            }
            sqlparser::keywords::Keyword::INDEX => {
                transformer.transform(self, sqlparser::keywords::Keyword::INDEX)
            }
            sqlparser::keywords::Keyword::INDICATOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::INDICATOR)
            }
            sqlparser::keywords::Keyword::INHERIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INHERIT)
            }
            sqlparser::keywords::Keyword::INITIALLY => {
                transformer.transform(self, sqlparser::keywords::Keyword::INITIALLY)
            }
            sqlparser::keywords::Keyword::INNER => {
                transformer.transform(self, sqlparser::keywords::Keyword::INNER)
            }
            sqlparser::keywords::Keyword::INOUT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INOUT)
            }
            sqlparser::keywords::Keyword::INPUT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INPUT)
            }
            sqlparser::keywords::Keyword::INPUTFORMAT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INPUTFORMAT)
            }
            sqlparser::keywords::Keyword::INSENSITIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::INSENSITIVE)
            }
            sqlparser::keywords::Keyword::INSERT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INSERT)
            }
            sqlparser::keywords::Keyword::INSTALL => {
                transformer.transform(self, sqlparser::keywords::Keyword::INSTALL)
            }
            sqlparser::keywords::Keyword::INSTEAD => {
                transformer.transform(self, sqlparser::keywords::Keyword::INSTEAD)
            }
            sqlparser::keywords::Keyword::INT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT)
            }
            sqlparser::keywords::Keyword::INT128 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT128)
            }
            sqlparser::keywords::Keyword::INT16 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT16)
            }
            sqlparser::keywords::Keyword::INT2 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT2)
            }
            sqlparser::keywords::Keyword::INT256 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT256)
            }
            sqlparser::keywords::Keyword::INT32 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT32)
            }
            sqlparser::keywords::Keyword::INT4 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT4)
            }
            sqlparser::keywords::Keyword::INT64 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT64)
            }
            sqlparser::keywords::Keyword::INT8 => {
                transformer.transform(self, sqlparser::keywords::Keyword::INT8)
            }
            sqlparser::keywords::Keyword::INTEGER => {
                transformer.transform(self, sqlparser::keywords::Keyword::INTEGER)
            }
            sqlparser::keywords::Keyword::INTERPOLATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::INTERPOLATE)
            }
            sqlparser::keywords::Keyword::INTERSECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::INTERSECT)
            }
            sqlparser::keywords::Keyword::INTERSECTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::INTERSECTION)
            }
            sqlparser::keywords::Keyword::INTERVAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::INTERVAL)
            }
            sqlparser::keywords::Keyword::INTO => {
                transformer.transform(self, sqlparser::keywords::Keyword::INTO)
            }
            sqlparser::keywords::Keyword::IS => {
                transformer.transform(self, sqlparser::keywords::Keyword::IS)
            }
            sqlparser::keywords::Keyword::ISODOW => {
                transformer.transform(self, sqlparser::keywords::Keyword::ISODOW)
            }
            sqlparser::keywords::Keyword::ISOLATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::ISOLATION)
            }
            sqlparser::keywords::Keyword::ISOWEEK => {
                transformer.transform(self, sqlparser::keywords::Keyword::ISOWEEK)
            }
            sqlparser::keywords::Keyword::ISOYEAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::ISOYEAR)
            }
            sqlparser::keywords::Keyword::ITEMS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ITEMS)
            }
            sqlparser::keywords::Keyword::JAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::JAR)
            }
            sqlparser::keywords::Keyword::JOIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::JOIN)
            }
            sqlparser::keywords::Keyword::JSON => {
                transformer.transform(self, sqlparser::keywords::Keyword::JSON)
            }
            sqlparser::keywords::Keyword::JSONB => {
                transformer.transform(self, sqlparser::keywords::Keyword::JSONB)
            }
            sqlparser::keywords::Keyword::JSONFILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::JSONFILE)
            }
            sqlparser::keywords::Keyword::JSON_TABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::JSON_TABLE)
            }
            sqlparser::keywords::Keyword::JULIAN => {
                transformer.transform(self, sqlparser::keywords::Keyword::JULIAN)
            }
            sqlparser::keywords::Keyword::KEY => {
                transformer.transform(self, sqlparser::keywords::Keyword::KEY)
            }
            sqlparser::keywords::Keyword::KEYS => {
                transformer.transform(self, sqlparser::keywords::Keyword::KEYS)
            }
            sqlparser::keywords::Keyword::KILL => {
                transformer.transform(self, sqlparser::keywords::Keyword::KILL)
            }
            sqlparser::keywords::Keyword::LAG => {
                transformer.transform(self, sqlparser::keywords::Keyword::LAG)
            }
            sqlparser::keywords::Keyword::LANGUAGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::LANGUAGE)
            }
            sqlparser::keywords::Keyword::LARGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::LARGE)
            }
            sqlparser::keywords::Keyword::LAST => {
                transformer.transform(self, sqlparser::keywords::Keyword::LAST)
            }
            sqlparser::keywords::Keyword::LAST_VALUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::LAST_VALUE)
            }
            sqlparser::keywords::Keyword::LATERAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::LATERAL)
            }
            sqlparser::keywords::Keyword::LEAD => {
                transformer.transform(self, sqlparser::keywords::Keyword::LEAD)
            }
            sqlparser::keywords::Keyword::LEADING => {
                transformer.transform(self, sqlparser::keywords::Keyword::LEADING)
            }
            sqlparser::keywords::Keyword::LEFT => {
                transformer.transform(self, sqlparser::keywords::Keyword::LEFT)
            }
            sqlparser::keywords::Keyword::LEVEL => {
                transformer.transform(self, sqlparser::keywords::Keyword::LEVEL)
            }
            sqlparser::keywords::Keyword::LIKE => {
                transformer.transform(self, sqlparser::keywords::Keyword::LIKE)
            }
            sqlparser::keywords::Keyword::LIKE_REGEX => {
                transformer.transform(self, sqlparser::keywords::Keyword::LIKE_REGEX)
            }
            sqlparser::keywords::Keyword::LIMIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::LIMIT)
            }
            sqlparser::keywords::Keyword::LINES => {
                transformer.transform(self, sqlparser::keywords::Keyword::LINES)
            }
            sqlparser::keywords::Keyword::LISTEN => {
                transformer.transform(self, sqlparser::keywords::Keyword::LISTEN)
            }
            sqlparser::keywords::Keyword::LN => {
                transformer.transform(self, sqlparser::keywords::Keyword::LN)
            }
            sqlparser::keywords::Keyword::LOAD => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOAD)
            }
            sqlparser::keywords::Keyword::LOCAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOCAL)
            }
            sqlparser::keywords::Keyword::LOCALTIME => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOCALTIME)
            }
            sqlparser::keywords::Keyword::LOCALTIMESTAMP => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOCALTIMESTAMP)
            }
            sqlparser::keywords::Keyword::LOCATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOCATION)
            }
            sqlparser::keywords::Keyword::LOCK => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOCK)
            }
            sqlparser::keywords::Keyword::LOCKED => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOCKED)
            }
            sqlparser::keywords::Keyword::LOGIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOGIN)
            }
            sqlparser::keywords::Keyword::LOGS => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOGS)
            }
            sqlparser::keywords::Keyword::LOWCARDINALITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOWCARDINALITY)
            }
            sqlparser::keywords::Keyword::LOWER => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOWER)
            }
            sqlparser::keywords::Keyword::LOW_PRIORITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::LOW_PRIORITY)
            }
            sqlparser::keywords::Keyword::MACRO => {
                transformer.transform(self, sqlparser::keywords::Keyword::MACRO)
            }
            sqlparser::keywords::Keyword::MANAGEDLOCATION => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::MANAGEDLOCATION)
            }
            sqlparser::keywords::Keyword::MAP => {
                transformer.transform(self, sqlparser::keywords::Keyword::MAP)
            }
            sqlparser::keywords::Keyword::MASKING => {
                transformer.transform(self, sqlparser::keywords::Keyword::MASKING)
            }
            sqlparser::keywords::Keyword::MATCH => {
                transformer.transform(self, sqlparser::keywords::Keyword::MATCH)
            }
            sqlparser::keywords::Keyword::MATCHED => {
                transformer.transform(self, sqlparser::keywords::Keyword::MATCHED)
            }
            sqlparser::keywords::Keyword::MATCHES => {
                transformer.transform(self, sqlparser::keywords::Keyword::MATCHES)
            }
            sqlparser::keywords::Keyword::MATCH_CONDITION => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::MATCH_CONDITION)
            }
            sqlparser::keywords::Keyword::MATCH_RECOGNIZE => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::MATCH_RECOGNIZE)
            }
            sqlparser::keywords::Keyword::MATERIALIZE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MATERIALIZE)
            }
            sqlparser::keywords::Keyword::MATERIALIZED => {
                transformer.transform(self, sqlparser::keywords::Keyword::MATERIALIZED)
            }
            sqlparser::keywords::Keyword::MAX => {
                transformer.transform(self, sqlparser::keywords::Keyword::MAX)
            }
            sqlparser::keywords::Keyword::MAXVALUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MAXVALUE)
            }
            sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS => {
                transformer
                    .transform(
                        self,
                        sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS,
                    )
            }
            sqlparser::keywords::Keyword::MEASURES => {
                transformer.transform(self, sqlparser::keywords::Keyword::MEASURES)
            }
            sqlparser::keywords::Keyword::MEDIUMINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::MEDIUMINT)
            }
            sqlparser::keywords::Keyword::MEMBER => {
                transformer.transform(self, sqlparser::keywords::Keyword::MEMBER)
            }
            sqlparser::keywords::Keyword::MERGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MERGE)
            }
            sqlparser::keywords::Keyword::METADATA => {
                transformer.transform(self, sqlparser::keywords::Keyword::METADATA)
            }
            sqlparser::keywords::Keyword::METHOD => {
                transformer.transform(self, sqlparser::keywords::Keyword::METHOD)
            }
            sqlparser::keywords::Keyword::MICROSECOND => {
                transformer.transform(self, sqlparser::keywords::Keyword::MICROSECOND)
            }
            sqlparser::keywords::Keyword::MICROSECONDS => {
                transformer.transform(self, sqlparser::keywords::Keyword::MICROSECONDS)
            }
            sqlparser::keywords::Keyword::MILLENIUM => {
                transformer.transform(self, sqlparser::keywords::Keyword::MILLENIUM)
            }
            sqlparser::keywords::Keyword::MILLENNIUM => {
                transformer.transform(self, sqlparser::keywords::Keyword::MILLENNIUM)
            }
            sqlparser::keywords::Keyword::MILLISECOND => {
                transformer.transform(self, sqlparser::keywords::Keyword::MILLISECOND)
            }
            sqlparser::keywords::Keyword::MILLISECONDS => {
                transformer.transform(self, sqlparser::keywords::Keyword::MILLISECONDS)
            }
            sqlparser::keywords::Keyword::MIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::MIN)
            }
            sqlparser::keywords::Keyword::MINUTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MINUTE)
            }
            sqlparser::keywords::Keyword::MINVALUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MINVALUE)
            }
            sqlparser::keywords::Keyword::MOD => {
                transformer.transform(self, sqlparser::keywords::Keyword::MOD)
            }
            sqlparser::keywords::Keyword::MODE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MODE)
            }
            sqlparser::keywords::Keyword::MODIFIES => {
                transformer.transform(self, sqlparser::keywords::Keyword::MODIFIES)
            }
            sqlparser::keywords::Keyword::MODIFY => {
                transformer.transform(self, sqlparser::keywords::Keyword::MODIFY)
            }
            sqlparser::keywords::Keyword::MODULE => {
                transformer.transform(self, sqlparser::keywords::Keyword::MODULE)
            }
            sqlparser::keywords::Keyword::MONTH => {
                transformer.transform(self, sqlparser::keywords::Keyword::MONTH)
            }
            sqlparser::keywords::Keyword::MSCK => {
                transformer.transform(self, sqlparser::keywords::Keyword::MSCK)
            }
            sqlparser::keywords::Keyword::MULTISET => {
                transformer.transform(self, sqlparser::keywords::Keyword::MULTISET)
            }
            sqlparser::keywords::Keyword::MUTATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::MUTATION)
            }
            sqlparser::keywords::Keyword::NAME => {
                transformer.transform(self, sqlparser::keywords::Keyword::NAME)
            }
            sqlparser::keywords::Keyword::NANOSECOND => {
                transformer.transform(self, sqlparser::keywords::Keyword::NANOSECOND)
            }
            sqlparser::keywords::Keyword::NANOSECONDS => {
                transformer.transform(self, sqlparser::keywords::Keyword::NANOSECONDS)
            }
            sqlparser::keywords::Keyword::NATIONAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::NATIONAL)
            }
            sqlparser::keywords::Keyword::NATURAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::NATURAL)
            }
            sqlparser::keywords::Keyword::NCHAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::NCHAR)
            }
            sqlparser::keywords::Keyword::NCLOB => {
                transformer.transform(self, sqlparser::keywords::Keyword::NCLOB)
            }
            sqlparser::keywords::Keyword::NESTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::NESTED)
            }
            sqlparser::keywords::Keyword::NEW => {
                transformer.transform(self, sqlparser::keywords::Keyword::NEW)
            }
            sqlparser::keywords::Keyword::NEXT => {
                transformer.transform(self, sqlparser::keywords::Keyword::NEXT)
            }
            sqlparser::keywords::Keyword::NO => {
                transformer.transform(self, sqlparser::keywords::Keyword::NO)
            }
            sqlparser::keywords::Keyword::NOBYPASSRLS => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOBYPASSRLS)
            }
            sqlparser::keywords::Keyword::NOCREATEDB => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOCREATEDB)
            }
            sqlparser::keywords::Keyword::NOCREATEROLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOCREATEROLE)
            }
            sqlparser::keywords::Keyword::NOINHERIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOINHERIT)
            }
            sqlparser::keywords::Keyword::NOLOGIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOLOGIN)
            }
            sqlparser::keywords::Keyword::NONE => {
                transformer.transform(self, sqlparser::keywords::Keyword::NONE)
            }
            sqlparser::keywords::Keyword::NOORDER => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOORDER)
            }
            sqlparser::keywords::Keyword::NOREPLICATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOREPLICATION)
            }
            sqlparser::keywords::Keyword::NORMALIZE => {
                transformer.transform(self, sqlparser::keywords::Keyword::NORMALIZE)
            }
            sqlparser::keywords::Keyword::NOSCAN => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOSCAN)
            }
            sqlparser::keywords::Keyword::NOSUPERUSER => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOSUPERUSER)
            }
            sqlparser::keywords::Keyword::NOT => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOT)
            }
            sqlparser::keywords::Keyword::NOTHING => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOTHING)
            }
            sqlparser::keywords::Keyword::NOTIFY => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOTIFY)
            }
            sqlparser::keywords::Keyword::NOWAIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::NOWAIT)
            }
            sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG)
            }
            sqlparser::keywords::Keyword::NTH_VALUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::NTH_VALUE)
            }
            sqlparser::keywords::Keyword::NTILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::NTILE)
            }
            sqlparser::keywords::Keyword::NULL => {
                transformer.transform(self, sqlparser::keywords::Keyword::NULL)
            }
            sqlparser::keywords::Keyword::NULLABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::NULLABLE)
            }
            sqlparser::keywords::Keyword::NULLIF => {
                transformer.transform(self, sqlparser::keywords::Keyword::NULLIF)
            }
            sqlparser::keywords::Keyword::NULLS => {
                transformer.transform(self, sqlparser::keywords::Keyword::NULLS)
            }
            sqlparser::keywords::Keyword::NUMERIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::NUMERIC)
            }
            sqlparser::keywords::Keyword::NVARCHAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::NVARCHAR)
            }
            sqlparser::keywords::Keyword::OBJECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::OBJECT)
            }
            sqlparser::keywords::Keyword::OCCURRENCES_REGEX => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::OCCURRENCES_REGEX)
            }
            sqlparser::keywords::Keyword::OCTETS => {
                transformer.transform(self, sqlparser::keywords::Keyword::OCTETS)
            }
            sqlparser::keywords::Keyword::OCTET_LENGTH => {
                transformer.transform(self, sqlparser::keywords::Keyword::OCTET_LENGTH)
            }
            sqlparser::keywords::Keyword::OF => {
                transformer.transform(self, sqlparser::keywords::Keyword::OF)
            }
            sqlparser::keywords::Keyword::OFFSET => {
                transformer.transform(self, sqlparser::keywords::Keyword::OFFSET)
            }
            sqlparser::keywords::Keyword::OLD => {
                transformer.transform(self, sqlparser::keywords::Keyword::OLD)
            }
            sqlparser::keywords::Keyword::OMIT => {
                transformer.transform(self, sqlparser::keywords::Keyword::OMIT)
            }
            sqlparser::keywords::Keyword::ON => {
                transformer.transform(self, sqlparser::keywords::Keyword::ON)
            }
            sqlparser::keywords::Keyword::ONE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ONE)
            }
            sqlparser::keywords::Keyword::ONLY => {
                transformer.transform(self, sqlparser::keywords::Keyword::ONLY)
            }
            sqlparser::keywords::Keyword::OPEN => {
                transformer.transform(self, sqlparser::keywords::Keyword::OPEN)
            }
            sqlparser::keywords::Keyword::OPERATOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::OPERATOR)
            }
            sqlparser::keywords::Keyword::OPTIMIZE => {
                transformer.transform(self, sqlparser::keywords::Keyword::OPTIMIZE)
            }
            sqlparser::keywords::Keyword::OPTIMIZER_COSTS => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::OPTIMIZER_COSTS)
            }
            sqlparser::keywords::Keyword::OPTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::OPTION)
            }
            sqlparser::keywords::Keyword::OPTIONS => {
                transformer.transform(self, sqlparser::keywords::Keyword::OPTIONS)
            }
            sqlparser::keywords::Keyword::OR => {
                transformer.transform(self, sqlparser::keywords::Keyword::OR)
            }
            sqlparser::keywords::Keyword::ORC => {
                transformer.transform(self, sqlparser::keywords::Keyword::ORC)
            }
            sqlparser::keywords::Keyword::ORDER => {
                transformer.transform(self, sqlparser::keywords::Keyword::ORDER)
            }
            sqlparser::keywords::Keyword::ORDINALITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::ORDINALITY)
            }
            sqlparser::keywords::Keyword::OUT => {
                transformer.transform(self, sqlparser::keywords::Keyword::OUT)
            }
            sqlparser::keywords::Keyword::OUTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::OUTER)
            }
            sqlparser::keywords::Keyword::OUTPUTFORMAT => {
                transformer.transform(self, sqlparser::keywords::Keyword::OUTPUTFORMAT)
            }
            sqlparser::keywords::Keyword::OVER => {
                transformer.transform(self, sqlparser::keywords::Keyword::OVER)
            }
            sqlparser::keywords::Keyword::OVERFLOW => {
                transformer.transform(self, sqlparser::keywords::Keyword::OVERFLOW)
            }
            sqlparser::keywords::Keyword::OVERLAPS => {
                transformer.transform(self, sqlparser::keywords::Keyword::OVERLAPS)
            }
            sqlparser::keywords::Keyword::OVERLAY => {
                transformer.transform(self, sqlparser::keywords::Keyword::OVERLAY)
            }
            sqlparser::keywords::Keyword::OVERWRITE => {
                transformer.transform(self, sqlparser::keywords::Keyword::OVERWRITE)
            }
            sqlparser::keywords::Keyword::OWNED => {
                transformer.transform(self, sqlparser::keywords::Keyword::OWNED)
            }
            sqlparser::keywords::Keyword::OWNER => {
                transformer.transform(self, sqlparser::keywords::Keyword::OWNER)
            }
            sqlparser::keywords::Keyword::PARALLEL => {
                transformer.transform(self, sqlparser::keywords::Keyword::PARALLEL)
            }
            sqlparser::keywords::Keyword::PARAMETER => {
                transformer.transform(self, sqlparser::keywords::Keyword::PARAMETER)
            }
            sqlparser::keywords::Keyword::PARQUET => {
                transformer.transform(self, sqlparser::keywords::Keyword::PARQUET)
            }
            sqlparser::keywords::Keyword::PART => {
                transformer.transform(self, sqlparser::keywords::Keyword::PART)
            }
            sqlparser::keywords::Keyword::PARTITION => {
                transformer.transform(self, sqlparser::keywords::Keyword::PARTITION)
            }
            sqlparser::keywords::Keyword::PARTITIONED => {
                transformer.transform(self, sqlparser::keywords::Keyword::PARTITIONED)
            }
            sqlparser::keywords::Keyword::PARTITIONS => {
                transformer.transform(self, sqlparser::keywords::Keyword::PARTITIONS)
            }
            sqlparser::keywords::Keyword::PASSWORD => {
                transformer.transform(self, sqlparser::keywords::Keyword::PASSWORD)
            }
            sqlparser::keywords::Keyword::PAST => {
                transformer.transform(self, sqlparser::keywords::Keyword::PAST)
            }
            sqlparser::keywords::Keyword::PATH => {
                transformer.transform(self, sqlparser::keywords::Keyword::PATH)
            }
            sqlparser::keywords::Keyword::PATTERN => {
                transformer.transform(self, sqlparser::keywords::Keyword::PATTERN)
            }
            sqlparser::keywords::Keyword::PER => {
                transformer.transform(self, sqlparser::keywords::Keyword::PER)
            }
            sqlparser::keywords::Keyword::PERCENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::PERCENT)
            }
            sqlparser::keywords::Keyword::PERCENTILE_CONT => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::PERCENTILE_CONT)
            }
            sqlparser::keywords::Keyword::PERCENTILE_DISC => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::PERCENTILE_DISC)
            }
            sqlparser::keywords::Keyword::PERCENT_RANK => {
                transformer.transform(self, sqlparser::keywords::Keyword::PERCENT_RANK)
            }
            sqlparser::keywords::Keyword::PERIOD => {
                transformer.transform(self, sqlparser::keywords::Keyword::PERIOD)
            }
            sqlparser::keywords::Keyword::PERMISSIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::PERMISSIVE)
            }
            sqlparser::keywords::Keyword::PERSISTENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::PERSISTENT)
            }
            sqlparser::keywords::Keyword::PIVOT => {
                transformer.transform(self, sqlparser::keywords::Keyword::PIVOT)
            }
            sqlparser::keywords::Keyword::PLACING => {
                transformer.transform(self, sqlparser::keywords::Keyword::PLACING)
            }
            sqlparser::keywords::Keyword::PLAN => {
                transformer.transform(self, sqlparser::keywords::Keyword::PLAN)
            }
            sqlparser::keywords::Keyword::PLANS => {
                transformer.transform(self, sqlparser::keywords::Keyword::PLANS)
            }
            sqlparser::keywords::Keyword::POLICY => {
                transformer.transform(self, sqlparser::keywords::Keyword::POLICY)
            }
            sqlparser::keywords::Keyword::PORTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::PORTION)
            }
            sqlparser::keywords::Keyword::POSITION => {
                transformer.transform(self, sqlparser::keywords::Keyword::POSITION)
            }
            sqlparser::keywords::Keyword::POSITION_REGEX => {
                transformer.transform(self, sqlparser::keywords::Keyword::POSITION_REGEX)
            }
            sqlparser::keywords::Keyword::POWER => {
                transformer.transform(self, sqlparser::keywords::Keyword::POWER)
            }
            sqlparser::keywords::Keyword::PRAGMA => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRAGMA)
            }
            sqlparser::keywords::Keyword::PRECEDES => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRECEDES)
            }
            sqlparser::keywords::Keyword::PRECEDING => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRECEDING)
            }
            sqlparser::keywords::Keyword::PRECISION => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRECISION)
            }
            sqlparser::keywords::Keyword::PREPARE => {
                transformer.transform(self, sqlparser::keywords::Keyword::PREPARE)
            }
            sqlparser::keywords::Keyword::PRESERVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRESERVE)
            }
            sqlparser::keywords::Keyword::PREWHERE => {
                transformer.transform(self, sqlparser::keywords::Keyword::PREWHERE)
            }
            sqlparser::keywords::Keyword::PRIMARY => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRIMARY)
            }
            sqlparser::keywords::Keyword::PRIOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRIOR)
            }
            sqlparser::keywords::Keyword::PRIVILEGES => {
                transformer.transform(self, sqlparser::keywords::Keyword::PRIVILEGES)
            }
            sqlparser::keywords::Keyword::PROCEDURE => {
                transformer.transform(self, sqlparser::keywords::Keyword::PROCEDURE)
            }
            sqlparser::keywords::Keyword::PROGRAM => {
                transformer.transform(self, sqlparser::keywords::Keyword::PROGRAM)
            }
            sqlparser::keywords::Keyword::PROJECTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::PROJECTION)
            }
            sqlparser::keywords::Keyword::PURGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::PURGE)
            }
            sqlparser::keywords::Keyword::QUALIFY => {
                transformer.transform(self, sqlparser::keywords::Keyword::QUALIFY)
            }
            sqlparser::keywords::Keyword::QUARTER => {
                transformer.transform(self, sqlparser::keywords::Keyword::QUARTER)
            }
            sqlparser::keywords::Keyword::QUERY => {
                transformer.transform(self, sqlparser::keywords::Keyword::QUERY)
            }
            sqlparser::keywords::Keyword::QUOTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::QUOTE)
            }
            sqlparser::keywords::Keyword::RANGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RANGE)
            }
            sqlparser::keywords::Keyword::RANK => {
                transformer.transform(self, sqlparser::keywords::Keyword::RANK)
            }
            sqlparser::keywords::Keyword::RAW => {
                transformer.transform(self, sqlparser::keywords::Keyword::RAW)
            }
            sqlparser::keywords::Keyword::RCFILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RCFILE)
            }
            sqlparser::keywords::Keyword::READ => {
                transformer.transform(self, sqlparser::keywords::Keyword::READ)
            }
            sqlparser::keywords::Keyword::READS => {
                transformer.transform(self, sqlparser::keywords::Keyword::READS)
            }
            sqlparser::keywords::Keyword::READ_ONLY => {
                transformer.transform(self, sqlparser::keywords::Keyword::READ_ONLY)
            }
            sqlparser::keywords::Keyword::REAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::REAL)
            }
            sqlparser::keywords::Keyword::RECURSIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RECURSIVE)
            }
            sqlparser::keywords::Keyword::REF => {
                transformer.transform(self, sqlparser::keywords::Keyword::REF)
            }
            sqlparser::keywords::Keyword::REFERENCES => {
                transformer.transform(self, sqlparser::keywords::Keyword::REFERENCES)
            }
            sqlparser::keywords::Keyword::REFERENCING => {
                transformer.transform(self, sqlparser::keywords::Keyword::REFERENCING)
            }
            sqlparser::keywords::Keyword::REGCLASS => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGCLASS)
            }
            sqlparser::keywords::Keyword::REGEXP => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGEXP)
            }
            sqlparser::keywords::Keyword::REGR_AVGX => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_AVGX)
            }
            sqlparser::keywords::Keyword::REGR_AVGY => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_AVGY)
            }
            sqlparser::keywords::Keyword::REGR_COUNT => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_COUNT)
            }
            sqlparser::keywords::Keyword::REGR_INTERCEPT => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_INTERCEPT)
            }
            sqlparser::keywords::Keyword::REGR_R2 => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_R2)
            }
            sqlparser::keywords::Keyword::REGR_SLOPE => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_SLOPE)
            }
            sqlparser::keywords::Keyword::REGR_SXX => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_SXX)
            }
            sqlparser::keywords::Keyword::REGR_SXY => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_SXY)
            }
            sqlparser::keywords::Keyword::REGR_SYY => {
                transformer.transform(self, sqlparser::keywords::Keyword::REGR_SYY)
            }
            sqlparser::keywords::Keyword::RELATIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RELATIVE)
            }
            sqlparser::keywords::Keyword::RELAY => {
                transformer.transform(self, sqlparser::keywords::Keyword::RELAY)
            }
            sqlparser::keywords::Keyword::RELEASE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RELEASE)
            }
            sqlparser::keywords::Keyword::REMOTE => {
                transformer.transform(self, sqlparser::keywords::Keyword::REMOTE)
            }
            sqlparser::keywords::Keyword::RENAME => {
                transformer.transform(self, sqlparser::keywords::Keyword::RENAME)
            }
            sqlparser::keywords::Keyword::REORG => {
                transformer.transform(self, sqlparser::keywords::Keyword::REORG)
            }
            sqlparser::keywords::Keyword::REPAIR => {
                transformer.transform(self, sqlparser::keywords::Keyword::REPAIR)
            }
            sqlparser::keywords::Keyword::REPEATABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::REPEATABLE)
            }
            sqlparser::keywords::Keyword::REPLACE => {
                transformer.transform(self, sqlparser::keywords::Keyword::REPLACE)
            }
            sqlparser::keywords::Keyword::REPLICA => {
                transformer.transform(self, sqlparser::keywords::Keyword::REPLICA)
            }
            sqlparser::keywords::Keyword::REPLICATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::REPLICATION)
            }
            sqlparser::keywords::Keyword::RESET => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESET)
            }
            sqlparser::keywords::Keyword::RESPECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESPECT)
            }
            sqlparser::keywords::Keyword::RESTART => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESTART)
            }
            sqlparser::keywords::Keyword::RESTRICT => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESTRICT)
            }
            sqlparser::keywords::Keyword::RESTRICTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESTRICTED)
            }
            sqlparser::keywords::Keyword::RESTRICTIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESTRICTIVE)
            }
            sqlparser::keywords::Keyword::RESULT => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESULT)
            }
            sqlparser::keywords::Keyword::RESULTSET => {
                transformer.transform(self, sqlparser::keywords::Keyword::RESULTSET)
            }
            sqlparser::keywords::Keyword::RETAIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::RETAIN)
            }
            sqlparser::keywords::Keyword::RETURN => {
                transformer.transform(self, sqlparser::keywords::Keyword::RETURN)
            }
            sqlparser::keywords::Keyword::RETURNING => {
                transformer.transform(self, sqlparser::keywords::Keyword::RETURNING)
            }
            sqlparser::keywords::Keyword::RETURNS => {
                transformer.transform(self, sqlparser::keywords::Keyword::RETURNS)
            }
            sqlparser::keywords::Keyword::REVOKE => {
                transformer.transform(self, sqlparser::keywords::Keyword::REVOKE)
            }
            sqlparser::keywords::Keyword::RIGHT => {
                transformer.transform(self, sqlparser::keywords::Keyword::RIGHT)
            }
            sqlparser::keywords::Keyword::RLIKE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RLIKE)
            }
            sqlparser::keywords::Keyword::ROLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROLE)
            }
            sqlparser::keywords::Keyword::ROLLBACK => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROLLBACK)
            }
            sqlparser::keywords::Keyword::ROLLUP => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROLLUP)
            }
            sqlparser::keywords::Keyword::ROOT => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROOT)
            }
            sqlparser::keywords::Keyword::ROW => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROW)
            }
            sqlparser::keywords::Keyword::ROWID => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROWID)
            }
            sqlparser::keywords::Keyword::ROWS => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROWS)
            }
            sqlparser::keywords::Keyword::ROW_NUMBER => {
                transformer.transform(self, sqlparser::keywords::Keyword::ROW_NUMBER)
            }
            sqlparser::keywords::Keyword::RULE => {
                transformer.transform(self, sqlparser::keywords::Keyword::RULE)
            }
            sqlparser::keywords::Keyword::RUN => {
                transformer.transform(self, sqlparser::keywords::Keyword::RUN)
            }
            sqlparser::keywords::Keyword::SAFE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SAFE)
            }
            sqlparser::keywords::Keyword::SAFE_CAST => {
                transformer.transform(self, sqlparser::keywords::Keyword::SAFE_CAST)
            }
            sqlparser::keywords::Keyword::SAVEPOINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::SAVEPOINT)
            }
            sqlparser::keywords::Keyword::SCHEMA => {
                transformer.transform(self, sqlparser::keywords::Keyword::SCHEMA)
            }
            sqlparser::keywords::Keyword::SCHEMAS => {
                transformer.transform(self, sqlparser::keywords::Keyword::SCHEMAS)
            }
            sqlparser::keywords::Keyword::SCOPE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SCOPE)
            }
            sqlparser::keywords::Keyword::SCROLL => {
                transformer.transform(self, sqlparser::keywords::Keyword::SCROLL)
            }
            sqlparser::keywords::Keyword::SEARCH => {
                transformer.transform(self, sqlparser::keywords::Keyword::SEARCH)
            }
            sqlparser::keywords::Keyword::SECOND => {
                transformer.transform(self, sqlparser::keywords::Keyword::SECOND)
            }
            sqlparser::keywords::Keyword::SECRET => {
                transformer.transform(self, sqlparser::keywords::Keyword::SECRET)
            }
            sqlparser::keywords::Keyword::SECURITY => {
                transformer.transform(self, sqlparser::keywords::Keyword::SECURITY)
            }
            sqlparser::keywords::Keyword::SELECT => {
                transformer.transform(self, sqlparser::keywords::Keyword::SELECT)
            }
            sqlparser::keywords::Keyword::SEMI => {
                transformer.transform(self, sqlparser::keywords::Keyword::SEMI)
            }
            sqlparser::keywords::Keyword::SENSITIVE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SENSITIVE)
            }
            sqlparser::keywords::Keyword::SEPARATOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::SEPARATOR)
            }
            sqlparser::keywords::Keyword::SEQUENCE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SEQUENCE)
            }
            sqlparser::keywords::Keyword::SEQUENCEFILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SEQUENCEFILE)
            }
            sqlparser::keywords::Keyword::SEQUENCES => {
                transformer.transform(self, sqlparser::keywords::Keyword::SEQUENCES)
            }
            sqlparser::keywords::Keyword::SERDE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SERDE)
            }
            sqlparser::keywords::Keyword::SERDEPROPERTIES => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::SERDEPROPERTIES)
            }
            sqlparser::keywords::Keyword::SERIALIZABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SERIALIZABLE)
            }
            sqlparser::keywords::Keyword::SESSION => {
                transformer.transform(self, sqlparser::keywords::Keyword::SESSION)
            }
            sqlparser::keywords::Keyword::SESSION_USER => {
                transformer.transform(self, sqlparser::keywords::Keyword::SESSION_USER)
            }
            sqlparser::keywords::Keyword::SET => {
                transformer.transform(self, sqlparser::keywords::Keyword::SET)
            }
            sqlparser::keywords::Keyword::SETS => {
                transformer.transform(self, sqlparser::keywords::Keyword::SETS)
            }
            sqlparser::keywords::Keyword::SETTINGS => {
                transformer.transform(self, sqlparser::keywords::Keyword::SETTINGS)
            }
            sqlparser::keywords::Keyword::SHARE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SHARE)
            }
            sqlparser::keywords::Keyword::SHOW => {
                transformer.transform(self, sqlparser::keywords::Keyword::SHOW)
            }
            sqlparser::keywords::Keyword::SIMILAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::SIMILAR)
            }
            sqlparser::keywords::Keyword::SKIP => {
                transformer.transform(self, sqlparser::keywords::Keyword::SKIP)
            }
            sqlparser::keywords::Keyword::SLOW => {
                transformer.transform(self, sqlparser::keywords::Keyword::SLOW)
            }
            sqlparser::keywords::Keyword::SMALLINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::SMALLINT)
            }
            sqlparser::keywords::Keyword::SNAPSHOT => {
                transformer.transform(self, sqlparser::keywords::Keyword::SNAPSHOT)
            }
            sqlparser::keywords::Keyword::SOME => {
                transformer.transform(self, sqlparser::keywords::Keyword::SOME)
            }
            sqlparser::keywords::Keyword::SORT => {
                transformer.transform(self, sqlparser::keywords::Keyword::SORT)
            }
            sqlparser::keywords::Keyword::SORTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::SORTED)
            }
            sqlparser::keywords::Keyword::SOURCE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SOURCE)
            }
            sqlparser::keywords::Keyword::SPATIAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::SPATIAL)
            }
            sqlparser::keywords::Keyword::SPECIFIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::SPECIFIC)
            }
            sqlparser::keywords::Keyword::SPECIFICTYPE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SPECIFICTYPE)
            }
            sqlparser::keywords::Keyword::SQL => {
                transformer.transform(self, sqlparser::keywords::Keyword::SQL)
            }
            sqlparser::keywords::Keyword::SQLEXCEPTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::SQLEXCEPTION)
            }
            sqlparser::keywords::Keyword::SQLSTATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::SQLSTATE)
            }
            sqlparser::keywords::Keyword::SQLWARNING => {
                transformer.transform(self, sqlparser::keywords::Keyword::SQLWARNING)
            }
            sqlparser::keywords::Keyword::SQRT => {
                transformer.transform(self, sqlparser::keywords::Keyword::SQRT)
            }
            sqlparser::keywords::Keyword::STABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::STABLE)
            }
            sqlparser::keywords::Keyword::STAGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::STAGE)
            }
            sqlparser::keywords::Keyword::START => {
                transformer.transform(self, sqlparser::keywords::Keyword::START)
            }
            sqlparser::keywords::Keyword::STATEMENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::STATEMENT)
            }
            sqlparser::keywords::Keyword::STATIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::STATIC)
            }
            sqlparser::keywords::Keyword::STATISTICS => {
                transformer.transform(self, sqlparser::keywords::Keyword::STATISTICS)
            }
            sqlparser::keywords::Keyword::STATUS => {
                transformer.transform(self, sqlparser::keywords::Keyword::STATUS)
            }
            sqlparser::keywords::Keyword::STDDEV_POP => {
                transformer.transform(self, sqlparser::keywords::Keyword::STDDEV_POP)
            }
            sqlparser::keywords::Keyword::STDDEV_SAMP => {
                transformer.transform(self, sqlparser::keywords::Keyword::STDDEV_SAMP)
            }
            sqlparser::keywords::Keyword::STDIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::STDIN)
            }
            sqlparser::keywords::Keyword::STDOUT => {
                transformer.transform(self, sqlparser::keywords::Keyword::STDOUT)
            }
            sqlparser::keywords::Keyword::STEP => {
                transformer.transform(self, sqlparser::keywords::Keyword::STEP)
            }
            sqlparser::keywords::Keyword::STORAGE_INTEGRATION => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::STORAGE_INTEGRATION)
            }
            sqlparser::keywords::Keyword::STORED => {
                transformer.transform(self, sqlparser::keywords::Keyword::STORED)
            }
            sqlparser::keywords::Keyword::STRICT => {
                transformer.transform(self, sqlparser::keywords::Keyword::STRICT)
            }
            sqlparser::keywords::Keyword::STRING => {
                transformer.transform(self, sqlparser::keywords::Keyword::STRING)
            }
            sqlparser::keywords::Keyword::STRUCT => {
                transformer.transform(self, sqlparser::keywords::Keyword::STRUCT)
            }
            sqlparser::keywords::Keyword::SUBMULTISET => {
                transformer.transform(self, sqlparser::keywords::Keyword::SUBMULTISET)
            }
            sqlparser::keywords::Keyword::SUBSTRING => {
                transformer.transform(self, sqlparser::keywords::Keyword::SUBSTRING)
            }
            sqlparser::keywords::Keyword::SUBSTRING_REGEX => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::SUBSTRING_REGEX)
            }
            sqlparser::keywords::Keyword::SUCCEEDS => {
                transformer.transform(self, sqlparser::keywords::Keyword::SUCCEEDS)
            }
            sqlparser::keywords::Keyword::SUM => {
                transformer.transform(self, sqlparser::keywords::Keyword::SUM)
            }
            sqlparser::keywords::Keyword::SUPER => {
                transformer.transform(self, sqlparser::keywords::Keyword::SUPER)
            }
            sqlparser::keywords::Keyword::SUPERUSER => {
                transformer.transform(self, sqlparser::keywords::Keyword::SUPERUSER)
            }
            sqlparser::keywords::Keyword::SWAP => {
                transformer.transform(self, sqlparser::keywords::Keyword::SWAP)
            }
            sqlparser::keywords::Keyword::SYMMETRIC => {
                transformer.transform(self, sqlparser::keywords::Keyword::SYMMETRIC)
            }
            sqlparser::keywords::Keyword::SYNC => {
                transformer.transform(self, sqlparser::keywords::Keyword::SYNC)
            }
            sqlparser::keywords::Keyword::SYSTEM => {
                transformer.transform(self, sqlparser::keywords::Keyword::SYSTEM)
            }
            sqlparser::keywords::Keyword::SYSTEM_TIME => {
                transformer.transform(self, sqlparser::keywords::Keyword::SYSTEM_TIME)
            }
            sqlparser::keywords::Keyword::SYSTEM_USER => {
                transformer.transform(self, sqlparser::keywords::Keyword::SYSTEM_USER)
            }
            sqlparser::keywords::Keyword::TABLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TABLE)
            }
            sqlparser::keywords::Keyword::TABLES => {
                transformer.transform(self, sqlparser::keywords::Keyword::TABLES)
            }
            sqlparser::keywords::Keyword::TABLESAMPLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TABLESAMPLE)
            }
            sqlparser::keywords::Keyword::TAG => {
                transformer.transform(self, sqlparser::keywords::Keyword::TAG)
            }
            sqlparser::keywords::Keyword::TARGET => {
                transformer.transform(self, sqlparser::keywords::Keyword::TARGET)
            }
            sqlparser::keywords::Keyword::TBLPROPERTIES => {
                transformer.transform(self, sqlparser::keywords::Keyword::TBLPROPERTIES)
            }
            sqlparser::keywords::Keyword::TEMP => {
                transformer.transform(self, sqlparser::keywords::Keyword::TEMP)
            }
            sqlparser::keywords::Keyword::TEMPORARY => {
                transformer.transform(self, sqlparser::keywords::Keyword::TEMPORARY)
            }
            sqlparser::keywords::Keyword::TERMINATED => {
                transformer.transform(self, sqlparser::keywords::Keyword::TERMINATED)
            }
            sqlparser::keywords::Keyword::TEXT => {
                transformer.transform(self, sqlparser::keywords::Keyword::TEXT)
            }
            sqlparser::keywords::Keyword::TEXTFILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TEXTFILE)
            }
            sqlparser::keywords::Keyword::THEN => {
                transformer.transform(self, sqlparser::keywords::Keyword::THEN)
            }
            sqlparser::keywords::Keyword::TIES => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIES)
            }
            sqlparser::keywords::Keyword::TIME => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIME)
            }
            sqlparser::keywords::Keyword::TIMESTAMP => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIMESTAMP)
            }
            sqlparser::keywords::Keyword::TIMESTAMPTZ => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIMESTAMPTZ)
            }
            sqlparser::keywords::Keyword::TIMETZ => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIMETZ)
            }
            sqlparser::keywords::Keyword::TIMEZONE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIMEZONE)
            }
            sqlparser::keywords::Keyword::TIMEZONE_ABBR => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIMEZONE_ABBR)
            }
            sqlparser::keywords::Keyword::TIMEZONE_HOUR => {
                transformer.transform(self, sqlparser::keywords::Keyword::TIMEZONE_HOUR)
            }
            sqlparser::keywords::Keyword::TIMEZONE_MINUTE => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::TIMEZONE_MINUTE)
            }
            sqlparser::keywords::Keyword::TIMEZONE_REGION => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::TIMEZONE_REGION)
            }
            sqlparser::keywords::Keyword::TINYINT => {
                transformer.transform(self, sqlparser::keywords::Keyword::TINYINT)
            }
            sqlparser::keywords::Keyword::TO => {
                transformer.transform(self, sqlparser::keywords::Keyword::TO)
            }
            sqlparser::keywords::Keyword::TOP => {
                transformer.transform(self, sqlparser::keywords::Keyword::TOP)
            }
            sqlparser::keywords::Keyword::TOTALS => {
                transformer.transform(self, sqlparser::keywords::Keyword::TOTALS)
            }
            sqlparser::keywords::Keyword::TRAILING => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRAILING)
            }
            sqlparser::keywords::Keyword::TRANSACTION => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRANSACTION)
            }
            sqlparser::keywords::Keyword::TRANSIENT => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRANSIENT)
            }
            sqlparser::keywords::Keyword::TRANSLATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRANSLATE)
            }
            sqlparser::keywords::Keyword::TRANSLATE_REGEX => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::TRANSLATE_REGEX)
            }
            sqlparser::keywords::Keyword::TRANSLATION => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRANSLATION)
            }
            sqlparser::keywords::Keyword::TREAT => {
                transformer.transform(self, sqlparser::keywords::Keyword::TREAT)
            }
            sqlparser::keywords::Keyword::TRIGGER => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRIGGER)
            }
            sqlparser::keywords::Keyword::TRIM => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRIM)
            }
            sqlparser::keywords::Keyword::TRIM_ARRAY => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRIM_ARRAY)
            }
            sqlparser::keywords::Keyword::TRUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRUE)
            }
            sqlparser::keywords::Keyword::TRUNCATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRUNCATE)
            }
            sqlparser::keywords::Keyword::TRY_CAST => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRY_CAST)
            }
            sqlparser::keywords::Keyword::TRY_CONVERT => {
                transformer.transform(self, sqlparser::keywords::Keyword::TRY_CONVERT)
            }
            sqlparser::keywords::Keyword::TUPLE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TUPLE)
            }
            sqlparser::keywords::Keyword::TYPE => {
                transformer.transform(self, sqlparser::keywords::Keyword::TYPE)
            }
            sqlparser::keywords::Keyword::UESCAPE => {
                transformer.transform(self, sqlparser::keywords::Keyword::UESCAPE)
            }
            sqlparser::keywords::Keyword::UINT128 => {
                transformer.transform(self, sqlparser::keywords::Keyword::UINT128)
            }
            sqlparser::keywords::Keyword::UINT16 => {
                transformer.transform(self, sqlparser::keywords::Keyword::UINT16)
            }
            sqlparser::keywords::Keyword::UINT256 => {
                transformer.transform(self, sqlparser::keywords::Keyword::UINT256)
            }
            sqlparser::keywords::Keyword::UINT32 => {
                transformer.transform(self, sqlparser::keywords::Keyword::UINT32)
            }
            sqlparser::keywords::Keyword::UINT64 => {
                transformer.transform(self, sqlparser::keywords::Keyword::UINT64)
            }
            sqlparser::keywords::Keyword::UINT8 => {
                transformer.transform(self, sqlparser::keywords::Keyword::UINT8)
            }
            sqlparser::keywords::Keyword::UNBOUNDED => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNBOUNDED)
            }
            sqlparser::keywords::Keyword::UNCACHE => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNCACHE)
            }
            sqlparser::keywords::Keyword::UNCOMMITTED => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNCOMMITTED)
            }
            sqlparser::keywords::Keyword::UNFREEZE => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNFREEZE)
            }
            sqlparser::keywords::Keyword::UNION => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNION)
            }
            sqlparser::keywords::Keyword::UNIQUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNIQUE)
            }
            sqlparser::keywords::Keyword::UNKNOWN => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNKNOWN)
            }
            sqlparser::keywords::Keyword::UNLOAD => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNLOAD)
            }
            sqlparser::keywords::Keyword::UNLOCK => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNLOCK)
            }
            sqlparser::keywords::Keyword::UNLOGGED => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNLOGGED)
            }
            sqlparser::keywords::Keyword::UNMATCHED => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNMATCHED)
            }
            sqlparser::keywords::Keyword::UNNEST => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNNEST)
            }
            sqlparser::keywords::Keyword::UNPIVOT => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNPIVOT)
            }
            sqlparser::keywords::Keyword::UNSAFE => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNSAFE)
            }
            sqlparser::keywords::Keyword::UNSIGNED => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNSIGNED)
            }
            sqlparser::keywords::Keyword::UNTIL => {
                transformer.transform(self, sqlparser::keywords::Keyword::UNTIL)
            }
            sqlparser::keywords::Keyword::UPDATE => {
                transformer.transform(self, sqlparser::keywords::Keyword::UPDATE)
            }
            sqlparser::keywords::Keyword::UPPER => {
                transformer.transform(self, sqlparser::keywords::Keyword::UPPER)
            }
            sqlparser::keywords::Keyword::URL => {
                transformer.transform(self, sqlparser::keywords::Keyword::URL)
            }
            sqlparser::keywords::Keyword::USAGE => {
                transformer.transform(self, sqlparser::keywords::Keyword::USAGE)
            }
            sqlparser::keywords::Keyword::USE => {
                transformer.transform(self, sqlparser::keywords::Keyword::USE)
            }
            sqlparser::keywords::Keyword::USER => {
                transformer.transform(self, sqlparser::keywords::Keyword::USER)
            }
            sqlparser::keywords::Keyword::USER_RESOURCES => {
                transformer.transform(self, sqlparser::keywords::Keyword::USER_RESOURCES)
            }
            sqlparser::keywords::Keyword::USING => {
                transformer.transform(self, sqlparser::keywords::Keyword::USING)
            }
            sqlparser::keywords::Keyword::UUID => {
                transformer.transform(self, sqlparser::keywords::Keyword::UUID)
            }
            sqlparser::keywords::Keyword::VACUUM => {
                transformer.transform(self, sqlparser::keywords::Keyword::VACUUM)
            }
            sqlparser::keywords::Keyword::VALID => {
                transformer.transform(self, sqlparser::keywords::Keyword::VALID)
            }
            sqlparser::keywords::Keyword::VALIDATION_MODE => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::VALIDATION_MODE)
            }
            sqlparser::keywords::Keyword::VALUE => {
                transformer.transform(self, sqlparser::keywords::Keyword::VALUE)
            }
            sqlparser::keywords::Keyword::VALUES => {
                transformer.transform(self, sqlparser::keywords::Keyword::VALUES)
            }
            sqlparser::keywords::Keyword::VALUE_OF => {
                transformer.transform(self, sqlparser::keywords::Keyword::VALUE_OF)
            }
            sqlparser::keywords::Keyword::VARBINARY => {
                transformer.transform(self, sqlparser::keywords::Keyword::VARBINARY)
            }
            sqlparser::keywords::Keyword::VARCHAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::VARCHAR)
            }
            sqlparser::keywords::Keyword::VARIABLES => {
                transformer.transform(self, sqlparser::keywords::Keyword::VARIABLES)
            }
            sqlparser::keywords::Keyword::VARYING => {
                transformer.transform(self, sqlparser::keywords::Keyword::VARYING)
            }
            sqlparser::keywords::Keyword::VAR_POP => {
                transformer.transform(self, sqlparser::keywords::Keyword::VAR_POP)
            }
            sqlparser::keywords::Keyword::VAR_SAMP => {
                transformer.transform(self, sqlparser::keywords::Keyword::VAR_SAMP)
            }
            sqlparser::keywords::Keyword::VERBOSE => {
                transformer.transform(self, sqlparser::keywords::Keyword::VERBOSE)
            }
            sqlparser::keywords::Keyword::VERSION => {
                transformer.transform(self, sqlparser::keywords::Keyword::VERSION)
            }
            sqlparser::keywords::Keyword::VERSIONING => {
                transformer.transform(self, sqlparser::keywords::Keyword::VERSIONING)
            }
            sqlparser::keywords::Keyword::VIEW => {
                transformer.transform(self, sqlparser::keywords::Keyword::VIEW)
            }
            sqlparser::keywords::Keyword::VIEWS => {
                transformer.transform(self, sqlparser::keywords::Keyword::VIEWS)
            }
            sqlparser::keywords::Keyword::VIRTUAL => {
                transformer.transform(self, sqlparser::keywords::Keyword::VIRTUAL)
            }
            sqlparser::keywords::Keyword::VOLATILE => {
                transformer.transform(self, sqlparser::keywords::Keyword::VOLATILE)
            }
            sqlparser::keywords::Keyword::WAREHOUSE => {
                transformer.transform(self, sqlparser::keywords::Keyword::WAREHOUSE)
            }
            sqlparser::keywords::Keyword::WEEK => {
                transformer.transform(self, sqlparser::keywords::Keyword::WEEK)
            }
            sqlparser::keywords::Keyword::WHEN => {
                transformer.transform(self, sqlparser::keywords::Keyword::WHEN)
            }
            sqlparser::keywords::Keyword::WHENEVER => {
                transformer.transform(self, sqlparser::keywords::Keyword::WHENEVER)
            }
            sqlparser::keywords::Keyword::WHERE => {
                transformer.transform(self, sqlparser::keywords::Keyword::WHERE)
            }
            sqlparser::keywords::Keyword::WIDTH_BUCKET => {
                transformer.transform(self, sqlparser::keywords::Keyword::WIDTH_BUCKET)
            }
            sqlparser::keywords::Keyword::WINDOW => {
                transformer.transform(self, sqlparser::keywords::Keyword::WINDOW)
            }
            sqlparser::keywords::Keyword::WITH => {
                transformer.transform(self, sqlparser::keywords::Keyword::WITH)
            }
            sqlparser::keywords::Keyword::WITHIN => {
                transformer.transform(self, sqlparser::keywords::Keyword::WITHIN)
            }
            sqlparser::keywords::Keyword::WITHOUT => {
                transformer.transform(self, sqlparser::keywords::Keyword::WITHOUT)
            }
            sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER => {
                transformer
                    .transform(self, sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER)
            }
            sqlparser::keywords::Keyword::WORK => {
                transformer.transform(self, sqlparser::keywords::Keyword::WORK)
            }
            sqlparser::keywords::Keyword::WRITE => {
                transformer.transform(self, sqlparser::keywords::Keyword::WRITE)
            }
            sqlparser::keywords::Keyword::XML => {
                transformer.transform(self, sqlparser::keywords::Keyword::XML)
            }
            sqlparser::keywords::Keyword::XOR => {
                transformer.transform(self, sqlparser::keywords::Keyword::XOR)
            }
            sqlparser::keywords::Keyword::YEAR => {
                transformer.transform(self, sqlparser::keywords::Keyword::YEAR)
            }
            sqlparser::keywords::Keyword::ZONE => {
                transformer.transform(self, sqlparser::keywords::Keyword::ZONE)
            }
            sqlparser::keywords::Keyword::ZORDER => {
                transformer.transform(self, sqlparser::keywords::Keyword::ZORDER)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Token {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::tokenizer::Token::EOF => {
                transformer.transform(self, sqlparser::tokenizer::Token::EOF)
            }
            sqlparser::tokenizer::Token::Word(field0) => {
                let transformed = sqlparser::tokenizer::Token::Word(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::Number(field0, field1) => {
                let transformed = sqlparser::tokenizer::Token::Number(
                    field0.apply_transform(transformer)?,
                    field1.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::Char(field0) => {
                let transformed = sqlparser::tokenizer::Token::Char(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::SingleQuotedString(field0) => {
                let transformed = sqlparser::tokenizer::Token::SingleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::DoubleQuotedString(field0) => {
                let transformed = sqlparser::tokenizer::Token::DoubleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::TripleSingleQuotedString(field0) => {
                let transformed = sqlparser::tokenizer::Token::TripleSingleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::TripleDoubleQuotedString(field0) => {
                let transformed = sqlparser::tokenizer::Token::TripleDoubleQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::DollarQuotedString(field0) => {
                let transformed = sqlparser::tokenizer::Token::DollarQuotedString(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::NationalStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::NationalStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::EscapedStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::EscapedStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::UnicodeStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::UnicodeStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::HexStringLiteral(field0) => {
                let transformed = sqlparser::tokenizer::Token::HexStringLiteral(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::Comma => {
                transformer.transform(self, sqlparser::tokenizer::Token::Comma)
            }
            sqlparser::tokenizer::Token::Whitespace(field0) => {
                let transformed = sqlparser::tokenizer::Token::Whitespace(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::DoubleEq => {
                transformer.transform(self, sqlparser::tokenizer::Token::DoubleEq)
            }
            sqlparser::tokenizer::Token::Eq => {
                transformer.transform(self, sqlparser::tokenizer::Token::Eq)
            }
            sqlparser::tokenizer::Token::Neq => {
                transformer.transform(self, sqlparser::tokenizer::Token::Neq)
            }
            sqlparser::tokenizer::Token::Lt => {
                transformer.transform(self, sqlparser::tokenizer::Token::Lt)
            }
            sqlparser::tokenizer::Token::Gt => {
                transformer.transform(self, sqlparser::tokenizer::Token::Gt)
            }
            sqlparser::tokenizer::Token::LtEq => {
                transformer.transform(self, sqlparser::tokenizer::Token::LtEq)
            }
            sqlparser::tokenizer::Token::GtEq => {
                transformer.transform(self, sqlparser::tokenizer::Token::GtEq)
            }
            sqlparser::tokenizer::Token::Spaceship => {
                transformer.transform(self, sqlparser::tokenizer::Token::Spaceship)
            }
            sqlparser::tokenizer::Token::Plus => {
                transformer.transform(self, sqlparser::tokenizer::Token::Plus)
            }
            sqlparser::tokenizer::Token::Minus => {
                transformer.transform(self, sqlparser::tokenizer::Token::Minus)
            }
            sqlparser::tokenizer::Token::Mul => {
                transformer.transform(self, sqlparser::tokenizer::Token::Mul)
            }
            sqlparser::tokenizer::Token::Div => {
                transformer.transform(self, sqlparser::tokenizer::Token::Div)
            }
            sqlparser::tokenizer::Token::DuckIntDiv => {
                transformer.transform(self, sqlparser::tokenizer::Token::DuckIntDiv)
            }
            sqlparser::tokenizer::Token::Mod => {
                transformer.transform(self, sqlparser::tokenizer::Token::Mod)
            }
            sqlparser::tokenizer::Token::StringConcat => {
                transformer.transform(self, sqlparser::tokenizer::Token::StringConcat)
            }
            sqlparser::tokenizer::Token::LParen => {
                transformer.transform(self, sqlparser::tokenizer::Token::LParen)
            }
            sqlparser::tokenizer::Token::RParen => {
                transformer.transform(self, sqlparser::tokenizer::Token::RParen)
            }
            sqlparser::tokenizer::Token::Period => {
                transformer.transform(self, sqlparser::tokenizer::Token::Period)
            }
            sqlparser::tokenizer::Token::Colon => {
                transformer.transform(self, sqlparser::tokenizer::Token::Colon)
            }
            sqlparser::tokenizer::Token::DoubleColon => {
                transformer.transform(self, sqlparser::tokenizer::Token::DoubleColon)
            }
            sqlparser::tokenizer::Token::Assignment => {
                transformer.transform(self, sqlparser::tokenizer::Token::Assignment)
            }
            sqlparser::tokenizer::Token::SemiColon => {
                transformer.transform(self, sqlparser::tokenizer::Token::SemiColon)
            }
            sqlparser::tokenizer::Token::Backslash => {
                transformer.transform(self, sqlparser::tokenizer::Token::Backslash)
            }
            sqlparser::tokenizer::Token::LBracket => {
                transformer.transform(self, sqlparser::tokenizer::Token::LBracket)
            }
            sqlparser::tokenizer::Token::RBracket => {
                transformer.transform(self, sqlparser::tokenizer::Token::RBracket)
            }
            sqlparser::tokenizer::Token::Ampersand => {
                transformer.transform(self, sqlparser::tokenizer::Token::Ampersand)
            }
            sqlparser::tokenizer::Token::Pipe => {
                transformer.transform(self, sqlparser::tokenizer::Token::Pipe)
            }
            sqlparser::tokenizer::Token::Caret => {
                transformer.transform(self, sqlparser::tokenizer::Token::Caret)
            }
            sqlparser::tokenizer::Token::LBrace => {
                transformer.transform(self, sqlparser::tokenizer::Token::LBrace)
            }
            sqlparser::tokenizer::Token::RBrace => {
                transformer.transform(self, sqlparser::tokenizer::Token::RBrace)
            }
            sqlparser::tokenizer::Token::RArrow => {
                transformer.transform(self, sqlparser::tokenizer::Token::RArrow)
            }
            sqlparser::tokenizer::Token::Sharp => {
                transformer.transform(self, sqlparser::tokenizer::Token::Sharp)
            }
            sqlparser::tokenizer::Token::Tilde => {
                transformer.transform(self, sqlparser::tokenizer::Token::Tilde)
            }
            sqlparser::tokenizer::Token::TildeAsterisk => {
                transformer.transform(self, sqlparser::tokenizer::Token::TildeAsterisk)
            }
            sqlparser::tokenizer::Token::ExclamationMarkTilde => {
                transformer
                    .transform(self, sqlparser::tokenizer::Token::ExclamationMarkTilde)
            }
            sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk => {
                transformer
                    .transform(
                        self,
                        sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk,
                    )
            }
            sqlparser::tokenizer::Token::DoubleTilde => {
                transformer.transform(self, sqlparser::tokenizer::Token::DoubleTilde)
            }
            sqlparser::tokenizer::Token::DoubleTildeAsterisk => {
                transformer
                    .transform(self, sqlparser::tokenizer::Token::DoubleTildeAsterisk)
            }
            sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde => {
                transformer
                    .transform(
                        self,
                        sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde,
                    )
            }
            sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk => {
                transformer
                    .transform(
                        self,
                        sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk,
                    )
            }
            sqlparser::tokenizer::Token::ShiftLeft => {
                transformer.transform(self, sqlparser::tokenizer::Token::ShiftLeft)
            }
            sqlparser::tokenizer::Token::ShiftRight => {
                transformer.transform(self, sqlparser::tokenizer::Token::ShiftRight)
            }
            sqlparser::tokenizer::Token::Overlap => {
                transformer.transform(self, sqlparser::tokenizer::Token::Overlap)
            }
            sqlparser::tokenizer::Token::ExclamationMark => {
                transformer.transform(self, sqlparser::tokenizer::Token::ExclamationMark)
            }
            sqlparser::tokenizer::Token::DoubleExclamationMark => {
                transformer
                    .transform(self, sqlparser::tokenizer::Token::DoubleExclamationMark)
            }
            sqlparser::tokenizer::Token::AtSign => {
                transformer.transform(self, sqlparser::tokenizer::Token::AtSign)
            }
            sqlparser::tokenizer::Token::CaretAt => {
                transformer.transform(self, sqlparser::tokenizer::Token::CaretAt)
            }
            sqlparser::tokenizer::Token::PGSquareRoot => {
                transformer.transform(self, sqlparser::tokenizer::Token::PGSquareRoot)
            }
            sqlparser::tokenizer::Token::PGCubeRoot => {
                transformer.transform(self, sqlparser::tokenizer::Token::PGCubeRoot)
            }
            sqlparser::tokenizer::Token::Placeholder(field0) => {
                let transformed = sqlparser::tokenizer::Token::Placeholder(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Token::Arrow => {
                transformer.transform(self, sqlparser::tokenizer::Token::Arrow)
            }
            sqlparser::tokenizer::Token::LongArrow => {
                transformer.transform(self, sqlparser::tokenizer::Token::LongArrow)
            }
            sqlparser::tokenizer::Token::HashArrow => {
                transformer.transform(self, sqlparser::tokenizer::Token::HashArrow)
            }
            sqlparser::tokenizer::Token::HashLongArrow => {
                transformer.transform(self, sqlparser::tokenizer::Token::HashLongArrow)
            }
            sqlparser::tokenizer::Token::AtArrow => {
                transformer.transform(self, sqlparser::tokenizer::Token::AtArrow)
            }
            sqlparser::tokenizer::Token::ArrowAt => {
                transformer.transform(self, sqlparser::tokenizer::Token::ArrowAt)
            }
            sqlparser::tokenizer::Token::HashMinus => {
                transformer.transform(self, sqlparser::tokenizer::Token::HashMinus)
            }
            sqlparser::tokenizer::Token::AtQuestion => {
                transformer.transform(self, sqlparser::tokenizer::Token::AtQuestion)
            }
            sqlparser::tokenizer::Token::AtAt => {
                transformer.transform(self, sqlparser::tokenizer::Token::AtAt)
            }
            sqlparser::tokenizer::Token::Question => {
                transformer.transform(self, sqlparser::tokenizer::Token::Question)
            }
            sqlparser::tokenizer::Token::QuestionAnd => {
                transformer.transform(self, sqlparser::tokenizer::Token::QuestionAnd)
            }
            sqlparser::tokenizer::Token::QuestionPipe => {
                transformer.transform(self, sqlparser::tokenizer::Token::QuestionPipe)
            }
            sqlparser::tokenizer::Token::CustomBinaryOperator(field0) => {
                let transformed = sqlparser::tokenizer::Token::CustomBinaryOperator(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Whitespace {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        match self {
            sqlparser::tokenizer::Whitespace::Space => {
                transformer.transform(self, sqlparser::tokenizer::Whitespace::Space)
            }
            sqlparser::tokenizer::Whitespace::Newline => {
                transformer.transform(self, sqlparser::tokenizer::Whitespace::Newline)
            }
            sqlparser::tokenizer::Whitespace::Tab => {
                transformer.transform(self, sqlparser::tokenizer::Whitespace::Tab)
            }
            sqlparser::tokenizer::Whitespace::SingleLineComment { comment, prefix } => {
                let transformed = sqlparser::tokenizer::Whitespace::SingleLineComment {
                    comment: comment.apply_transform(transformer)?,
                    prefix: prefix.apply_transform(transformer)?,
                };
                transformer.transform(self, transformed)
            }
            sqlparser::tokenizer::Whitespace::MultiLineComment(field0) => {
                let transformed = sqlparser::tokenizer::Whitespace::MultiLineComment(
                    field0.apply_transform(transformer)?,
                );
                transformer.transform(self, transformed)
            }
        }
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Word {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        let Self { value, quote_style, keyword } = self;
        let transformed = Self {
            value: value.apply_transform(transformer)?,
            quote_style: quote_style.apply_transform(transformer)?,
            keyword: keyword.apply_transform(transformer)?,
        };
        transformer.transform(self, transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for bigdecimal::BigDecimal {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, self.clone())
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for bool {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for char {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i16 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i32 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i64 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i8 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for String {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, self.clone())
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u16 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u32 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u64 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u8 {
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        transformer.transform(self, *self)
    }
}
