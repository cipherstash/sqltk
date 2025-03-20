#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Action {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Action::Connect => {
                    transformer.transform(sqlparser::ast::Action::Connect, self, context)?
                }
                sqlparser::ast::Action::Create => {
                    transformer.transform(sqlparser::ast::Action::Create, self, context)?
                }
                sqlparser::ast::Action::Delete => {
                    transformer.transform(sqlparser::ast::Action::Delete, self, context)?
                }
                sqlparser::ast::Action::Execute => {
                    transformer.transform(sqlparser::ast::Action::Execute, self, context)?
                }
                sqlparser::ast::Action::Insert { columns } => sqlparser::ast::Action::Insert {
                    columns: columns.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Action::References { columns } => {
                    sqlparser::ast::Action::References {
                        columns: columns.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Action::Select { columns } => sqlparser::ast::Action::Select {
                    columns: columns.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Action::Temporary => {
                    transformer.transform(sqlparser::ast::Action::Temporary, self, context)?
                }
                sqlparser::ast::Action::Trigger => {
                    transformer.transform(sqlparser::ast::Action::Trigger, self, context)?
                }
                sqlparser::ast::Action::Truncate => {
                    transformer.transform(sqlparser::ast::Action::Truncate, self, context)?
                }
                sqlparser::ast::Action::Update { columns } => sqlparser::ast::Action::Update {
                    columns: columns.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Action::Usage => {
                    transformer.transform(sqlparser::ast::Action::Usage, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AddDropSync {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AddDropSync::ADD => {
                    transformer.transform(sqlparser::ast::AddDropSync::ADD, self, context)?
                }
                sqlparser::ast::AddDropSync::DROP => {
                    transformer.transform(sqlparser::ast::AddDropSync::DROP, self, context)?
                }
                sqlparser::ast::AddDropSync::SYNC => {
                    transformer.transform(sqlparser::ast::AddDropSync::SYNC, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AfterMatchSkip {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AfterMatchSkip::PastLastRow => transformer.transform(
                    sqlparser::ast::AfterMatchSkip::PastLastRow,
                    self,
                    context,
                )?,
                sqlparser::ast::AfterMatchSkip::ToNextRow => transformer.transform(
                    sqlparser::ast::AfterMatchSkip::ToNextRow,
                    self,
                    context,
                )?,
                sqlparser::ast::AfterMatchSkip::ToFirst(field0) => {
                    sqlparser::ast::AfterMatchSkip::ToFirst(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::AfterMatchSkip::ToLast(field0) => {
                    sqlparser::ast::AfterMatchSkip::ToLast(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterColumnOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AlterColumnOperation::SetNotNull => transformer.transform(
                    sqlparser::ast::AlterColumnOperation::SetNotNull,
                    self,
                    context,
                )?,
                sqlparser::ast::AlterColumnOperation::DropNotNull => transformer.transform(
                    sqlparser::ast::AlterColumnOperation::DropNotNull,
                    self,
                    context,
                )?,
                sqlparser::ast::AlterColumnOperation::SetDefault { value } => {
                    sqlparser::ast::AlterColumnOperation::SetDefault {
                        value: value.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterColumnOperation::DropDefault => transformer.transform(
                    sqlparser::ast::AlterColumnOperation::DropDefault,
                    self,
                    context,
                )?,
                sqlparser::ast::AlterColumnOperation::SetDataType { data_type, using } => {
                    sqlparser::ast::AlterColumnOperation::SetDataType {
                        data_type: data_type.apply_transform_with_path(transformer, context)?,
                        using: using.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterColumnOperation::AddGenerated {
                    generated_as,
                    sequence_options,
                } => sqlparser::ast::AlterColumnOperation::AddGenerated {
                    generated_as: generated_as.apply_transform_with_path(transformer, context)?,
                    sequence_options: sequence_options
                        .apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterIndexOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AlterIndexOperation::RenameIndex { index_name } => {
                    sqlparser::ast::AlterIndexOperation::RenameIndex {
                        index_name: index_name.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterPolicyOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AlterPolicyOperation::Rename { new_name } => {
                    sqlparser::ast::AlterPolicyOperation::Rename {
                        new_name: new_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterPolicyOperation::Apply {
                    to,
                    using,
                    with_check,
                } => sqlparser::ast::AlterPolicyOperation::Apply {
                    to: to.apply_transform_with_path(transformer, context)?,
                    using: using.apply_transform_with_path(transformer, context)?,
                    with_check: with_check.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterRoleOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AlterRoleOperation::RenameRole { role_name } => {
                    sqlparser::ast::AlterRoleOperation::RenameRole {
                        role_name: role_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::AddMember { member_name } => {
                    sqlparser::ast::AlterRoleOperation::AddMember {
                        member_name: member_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::DropMember { member_name } => {
                    sqlparser::ast::AlterRoleOperation::DropMember {
                        member_name: member_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::WithOptions { options } => {
                    sqlparser::ast::AlterRoleOperation::WithOptions {
                        options: options.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::Set {
                    config_name,
                    config_value,
                    in_database,
                } => sqlparser::ast::AlterRoleOperation::Set {
                    config_name: config_name.apply_transform_with_path(transformer, context)?,
                    config_value: config_value.apply_transform_with_path(transformer, context)?,
                    in_database: in_database.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterRoleOperation::Reset {
                    config_name,
                    in_database,
                } => sqlparser::ast::AlterRoleOperation::Reset {
                    config_name: config_name.apply_transform_with_path(transformer, context)?,
                    in_database: in_database.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterTableOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AlterTableOperation::AddConstraint(field0) => {
                    sqlparser::ast::AlterTableOperation::AddConstraint(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::AlterTableOperation::AddColumn {
                    column_keyword,
                    if_not_exists,
                    column_def,
                    column_position,
                } => sqlparser::ast::AlterTableOperation::AddColumn {
                    column_keyword: column_keyword
                        .apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    column_def: column_def.apply_transform_with_path(transformer, context)?,
                    column_position: column_position
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::AddProjection {
                    if_not_exists,
                    name,
                    select,
                } => sqlparser::ast::AlterTableOperation::AddProjection {
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    select: select.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::DropProjection { if_exists, name } => {
                    sqlparser::ast::AlterTableOperation::DropProjection {
                        if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::MaterializeProjection {
                    if_exists,
                    name,
                    partition,
                } => sqlparser::ast::AlterTableOperation::MaterializeProjection {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    partition: partition.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::ClearProjection {
                    if_exists,
                    name,
                    partition,
                } => sqlparser::ast::AlterTableOperation::ClearProjection {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    partition: partition.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity => transformer
                    .transform(
                        sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity,
                        self,
                        context,
                    )?,
                sqlparser::ast::AlterTableOperation::DisableRule { name } => {
                    sqlparser::ast::AlterTableOperation::DisableRule {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DisableTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::DisableTrigger {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DropConstraint {
                    if_exists,
                    name,
                    cascade,
                } => sqlparser::ast::AlterTableOperation::DropConstraint {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    cascade: cascade.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::DropColumn {
                    column_name,
                    if_exists,
                    cascade,
                } => sqlparser::ast::AlterTableOperation::DropColumn {
                    column_name: column_name.apply_transform_with_path(transformer, context)?,
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    cascade: cascade.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::AttachPartition { partition } => {
                    sqlparser::ast::AlterTableOperation::AttachPartition {
                        partition: partition.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DetachPartition { partition } => {
                    sqlparser::ast::AlterTableOperation::DetachPartition {
                        partition: partition.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::FreezePartition {
                    partition,
                    with_name,
                } => sqlparser::ast::AlterTableOperation::FreezePartition {
                    partition: partition.apply_transform_with_path(transformer, context)?,
                    with_name: with_name.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::UnfreezePartition {
                    partition,
                    with_name,
                } => sqlparser::ast::AlterTableOperation::UnfreezePartition {
                    partition: partition.apply_transform_with_path(transformer, context)?,
                    with_name: with_name.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::DropPrimaryKey => transformer.transform(
                    sqlparser::ast::AlterTableOperation::DropPrimaryKey,
                    self,
                    context,
                )?,
                sqlparser::ast::AlterTableOperation::EnableAlwaysRule { name } => {
                    sqlparser::ast::AlterTableOperation::EnableAlwaysRule {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableReplicaRule { name } => {
                    sqlparser::ast::AlterTableOperation::EnableReplicaRule {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableReplicaTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::EnableReplicaTrigger {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity => transformer
                    .transform(
                        sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity,
                        self,
                        context,
                    )?,
                sqlparser::ast::AlterTableOperation::EnableRule { name } => {
                    sqlparser::ast::AlterTableOperation::EnableRule {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::EnableTrigger {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::RenamePartitions {
                    old_partitions,
                    new_partitions,
                } => sqlparser::ast::AlterTableOperation::RenamePartitions {
                    old_partitions: old_partitions
                        .apply_transform_with_path(transformer, context)?,
                    new_partitions: new_partitions
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::AddPartitions {
                    if_not_exists,
                    new_partitions,
                } => sqlparser::ast::AlterTableOperation::AddPartitions {
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    new_partitions: new_partitions
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::DropPartitions {
                    partitions,
                    if_exists,
                } => sqlparser::ast::AlterTableOperation::DropPartitions {
                    partitions: partitions.apply_transform_with_path(transformer, context)?,
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::RenameColumn {
                    old_column_name,
                    new_column_name,
                } => sqlparser::ast::AlterTableOperation::RenameColumn {
                    old_column_name: old_column_name
                        .apply_transform_with_path(transformer, context)?,
                    new_column_name: new_column_name
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::RenameTable { table_name } => {
                    sqlparser::ast::AlterTableOperation::RenameTable {
                        table_name: table_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::ChangeColumn {
                    old_name,
                    new_name,
                    data_type,
                    options,
                    column_position,
                } => sqlparser::ast::AlterTableOperation::ChangeColumn {
                    old_name: old_name.apply_transform_with_path(transformer, context)?,
                    new_name: new_name.apply_transform_with_path(transformer, context)?,
                    data_type: data_type.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                    column_position: column_position
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::ModifyColumn {
                    col_name,
                    data_type,
                    options,
                    column_position,
                } => sqlparser::ast::AlterTableOperation::ModifyColumn {
                    col_name: col_name.apply_transform_with_path(transformer, context)?,
                    data_type: data_type.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                    column_position: column_position
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::AlterTableOperation::RenameConstraint { old_name, new_name } => {
                    sqlparser::ast::AlterTableOperation::RenameConstraint {
                        old_name: old_name.apply_transform_with_path(transformer, context)?,
                        new_name: new_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::AlterColumn { column_name, op } => {
                    sqlparser::ast::AlterTableOperation::AlterColumn {
                        column_name: column_name.apply_transform_with_path(transformer, context)?,
                        op: op.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::SwapWith { table_name } => {
                    sqlparser::ast::AlterTableOperation::SwapWith {
                        table_name: table_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::SetTblProperties { table_properties } => {
                    sqlparser::ast::AlterTableOperation::SetTblProperties {
                        table_properties: table_properties
                            .apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::OwnerTo { new_owner } => {
                    sqlparser::ast::AlterTableOperation::OwnerTo {
                        new_owner: new_owner.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AnalyzeFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AnalyzeFormat::TEXT => {
                    transformer.transform(sqlparser::ast::AnalyzeFormat::TEXT, self, context)?
                }
                sqlparser::ast::AnalyzeFormat::GRAPHVIZ => {
                    transformer.transform(sqlparser::ast::AnalyzeFormat::GRAPHVIZ, self, context)?
                }
                sqlparser::ast::AnalyzeFormat::JSON => {
                    transformer.transform(sqlparser::ast::AnalyzeFormat::JSON, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ArgMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ArgMode::In => {
                    transformer.transform(sqlparser::ast::ArgMode::In, self, context)?
                }
                sqlparser::ast::ArgMode::Out => {
                    transformer.transform(sqlparser::ast::ArgMode::Out, self, context)?
                }
                sqlparser::ast::ArgMode::InOut => {
                    transformer.transform(sqlparser::ast::ArgMode::InOut, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Array {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { elem, named } = self;
            Self {
                elem: elem.apply_transform_with_path(transformer, context)?,
                named: named.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ArrayElemTypeDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ArrayElemTypeDef::None => {
                    transformer.transform(sqlparser::ast::ArrayElemTypeDef::None, self, context)?
                }
                sqlparser::ast::ArrayElemTypeDef::AngleBracket(field0) => {
                    sqlparser::ast::ArrayElemTypeDef::AngleBracket(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ArrayElemTypeDef::SquareBracket(field0, field1) => {
                    sqlparser::ast::ArrayElemTypeDef::SquareBracket(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ArrayElemTypeDef::Parenthesis(field0) => {
                    sqlparser::ast::ArrayElemTypeDef::Parenthesis(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Assignment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { target, value } = self;
            Self {
                target: target.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AssignmentTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AssignmentTarget::ColumnName(field0) => {
                    sqlparser::ast::AssignmentTarget::ColumnName(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::AssignmentTarget::Tuple(field0) => {
                    sqlparser::ast::AssignmentTarget::Tuple(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AttachDuckDBDatabaseOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(field0) => {
                    sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::AttachDuckDBDatabaseOption::Type(field0) => {
                    sqlparser::ast::AttachDuckDBDatabaseOption::Type(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::BinaryOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::BinaryOperator::Plus => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Plus, self, context)?
                }
                sqlparser::ast::BinaryOperator::Minus => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Minus, self, context)?
                }
                sqlparser::ast::BinaryOperator::Multiply => transformer.transform(
                    sqlparser::ast::BinaryOperator::Multiply,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::Divide => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Divide, self, context)?
                }
                sqlparser::ast::BinaryOperator::Modulo => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Modulo, self, context)?
                }
                sqlparser::ast::BinaryOperator::StringConcat => transformer.transform(
                    sqlparser::ast::BinaryOperator::StringConcat,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::Gt => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Gt, self, context)?
                }
                sqlparser::ast::BinaryOperator::Lt => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Lt, self, context)?
                }
                sqlparser::ast::BinaryOperator::GtEq => {
                    transformer.transform(sqlparser::ast::BinaryOperator::GtEq, self, context)?
                }
                sqlparser::ast::BinaryOperator::LtEq => {
                    transformer.transform(sqlparser::ast::BinaryOperator::LtEq, self, context)?
                }
                sqlparser::ast::BinaryOperator::Spaceship => transformer.transform(
                    sqlparser::ast::BinaryOperator::Spaceship,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::Eq => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Eq, self, context)?
                }
                sqlparser::ast::BinaryOperator::NotEq => {
                    transformer.transform(sqlparser::ast::BinaryOperator::NotEq, self, context)?
                }
                sqlparser::ast::BinaryOperator::And => {
                    transformer.transform(sqlparser::ast::BinaryOperator::And, self, context)?
                }
                sqlparser::ast::BinaryOperator::Or => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Or, self, context)?
                }
                sqlparser::ast::BinaryOperator::Xor => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Xor, self, context)?
                }
                sqlparser::ast::BinaryOperator::BitwiseOr => transformer.transform(
                    sqlparser::ast::BinaryOperator::BitwiseOr,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::BitwiseAnd => transformer.transform(
                    sqlparser::ast::BinaryOperator::BitwiseAnd,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::BitwiseXor => transformer.transform(
                    sqlparser::ast::BinaryOperator::BitwiseXor,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::DuckIntegerDivide => transformer.transform(
                    sqlparser::ast::BinaryOperator::DuckIntegerDivide,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::MyIntegerDivide => transformer.transform(
                    sqlparser::ast::BinaryOperator::MyIntegerDivide,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::Custom(field0) => {
                    sqlparser::ast::BinaryOperator::Custom(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::BinaryOperator::PGBitwiseXor => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGBitwiseXor,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGBitwiseShiftRight => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGBitwiseShiftRight,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGExp => {
                    transformer.transform(sqlparser::ast::BinaryOperator::PGExp, self, context)?
                }
                sqlparser::ast::BinaryOperator::PGOverlap => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGOverlap,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGRegexMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGRegexMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGRegexIMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGRegexIMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGRegexNotMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGRegexNotMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGRegexNotIMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGRegexNotIMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGLikeMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGLikeMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGILikeMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGILikeMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGNotLikeMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGNotLikeMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGNotILikeMatch => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGNotILikeMatch,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGStartsWith => transformer.transform(
                    sqlparser::ast::BinaryOperator::PGStartsWith,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::Arrow => {
                    transformer.transform(sqlparser::ast::BinaryOperator::Arrow, self, context)?
                }
                sqlparser::ast::BinaryOperator::LongArrow => transformer.transform(
                    sqlparser::ast::BinaryOperator::LongArrow,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::HashArrow => transformer.transform(
                    sqlparser::ast::BinaryOperator::HashArrow,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::HashLongArrow => transformer.transform(
                    sqlparser::ast::BinaryOperator::HashLongArrow,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::AtAt => {
                    transformer.transform(sqlparser::ast::BinaryOperator::AtAt, self, context)?
                }
                sqlparser::ast::BinaryOperator::AtArrow => {
                    transformer.transform(sqlparser::ast::BinaryOperator::AtArrow, self, context)?
                }
                sqlparser::ast::BinaryOperator::ArrowAt => {
                    transformer.transform(sqlparser::ast::BinaryOperator::ArrowAt, self, context)?
                }
                sqlparser::ast::BinaryOperator::HashMinus => transformer.transform(
                    sqlparser::ast::BinaryOperator::HashMinus,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::AtQuestion => transformer.transform(
                    sqlparser::ast::BinaryOperator::AtQuestion,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::Question => transformer.transform(
                    sqlparser::ast::BinaryOperator::Question,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::QuestionAnd => transformer.transform(
                    sqlparser::ast::BinaryOperator::QuestionAnd,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::QuestionPipe => transformer.transform(
                    sqlparser::ast::BinaryOperator::QuestionPipe,
                    self,
                    context,
                )?,
                sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(field0) => {
                    sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CastFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CastFormat::Value(field0) => sqlparser::ast::CastFormat::Value(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CastFormat::ValueAtTimeZone(field0, field1) => {
                    sqlparser::ast::CastFormat::ValueAtTimeZone(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CastKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CastKind::Cast => {
                    transformer.transform(sqlparser::ast::CastKind::Cast, self, context)?
                }
                sqlparser::ast::CastKind::TryCast => {
                    transformer.transform(sqlparser::ast::CastKind::TryCast, self, context)?
                }
                sqlparser::ast::CastKind::SafeCast => {
                    transformer.transform(sqlparser::ast::CastKind::SafeCast, self, context)?
                }
                sqlparser::ast::CastKind::DoubleColon => {
                    transformer.transform(sqlparser::ast::CastKind::DoubleColon, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CeilFloorKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CeilFloorKind::DateTimeField(field0) => {
                    sqlparser::ast::CeilFloorKind::DateTimeField(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CeilFloorKind::Scale(field0) => {
                    sqlparser::ast::CeilFloorKind::Scale(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CharLengthUnits {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CharLengthUnits::Characters => transformer.transform(
                    sqlparser::ast::CharLengthUnits::Characters,
                    self,
                    context,
                )?,
                sqlparser::ast::CharLengthUnits::Octets => {
                    transformer.transform(sqlparser::ast::CharLengthUnits::Octets, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CharacterLength {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CharacterLength::IntegerLength { length, unit } => {
                    sqlparser::ast::CharacterLength::IntegerLength {
                        length: length.apply_transform_with_path(transformer, context)?,
                        unit: unit.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::CharacterLength::Max => {
                    transformer.transform(sqlparser::ast::CharacterLength::Max, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CloseCursor {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CloseCursor::All => {
                    transformer.transform(sqlparser::ast::CloseCursor::All, self, context)?
                }
                sqlparser::ast::CloseCursor::Specific { name } => {
                    sqlparser::ast::CloseCursor::Specific {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ClusteredBy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                columns,
                sorted_by,
                num_buckets,
            } = self;
            Self {
                columns: columns.apply_transform_with_path(transformer, context)?,
                sorted_by: sorted_by.apply_transform_with_path(transformer, context)?,
                num_buckets: num_buckets.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ClusteredIndex {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, asc } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                asc: asc.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                name,
                data_type,
                collation,
                options,
            } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                data_type: data_type.apply_transform_with_path(transformer, context)?,
                collation: collation.apply_transform_with_path(transformer, context)?,
                options: options.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ColumnOption::Null => {
                    transformer.transform(sqlparser::ast::ColumnOption::Null, self, context)?
                }
                sqlparser::ast::ColumnOption::NotNull => {
                    transformer.transform(sqlparser::ast::ColumnOption::NotNull, self, context)?
                }
                sqlparser::ast::ColumnOption::Default(field0) => {
                    sqlparser::ast::ColumnOption::Default(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Materialized(field0) => {
                    sqlparser::ast::ColumnOption::Materialized(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Ephemeral(field0) => {
                    sqlparser::ast::ColumnOption::Ephemeral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Alias(field0) => sqlparser::ast::ColumnOption::Alias(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::ColumnOption::Unique {
                    is_primary,
                    characteristics,
                } => sqlparser::ast::ColumnOption::Unique {
                    is_primary: is_primary.apply_transform_with_path(transformer, context)?,
                    characteristics: characteristics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::ColumnOption::ForeignKey {
                    foreign_table,
                    referred_columns,
                    on_delete,
                    on_update,
                    characteristics,
                } => sqlparser::ast::ColumnOption::ForeignKey {
                    foreign_table: foreign_table.apply_transform_with_path(transformer, context)?,
                    referred_columns: referred_columns
                        .apply_transform_with_path(transformer, context)?,
                    on_delete: on_delete.apply_transform_with_path(transformer, context)?,
                    on_update: on_update.apply_transform_with_path(transformer, context)?,
                    characteristics: characteristics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::ColumnOption::Check(field0) => sqlparser::ast::ColumnOption::Check(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::ColumnOption::DialectSpecific(field0) => {
                    sqlparser::ast::ColumnOption::DialectSpecific(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::CharacterSet(field0) => {
                    sqlparser::ast::ColumnOption::CharacterSet(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Comment(field0) => {
                    sqlparser::ast::ColumnOption::Comment(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::OnUpdate(field0) => {
                    sqlparser::ast::ColumnOption::OnUpdate(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Generated {
                    generated_as,
                    sequence_options,
                    generation_expr,
                    generation_expr_mode,
                    generated_keyword,
                } => sqlparser::ast::ColumnOption::Generated {
                    generated_as: generated_as.apply_transform_with_path(transformer, context)?,
                    sequence_options: sequence_options
                        .apply_transform_with_path(transformer, context)?,
                    generation_expr: generation_expr
                        .apply_transform_with_path(transformer, context)?,
                    generation_expr_mode: generation_expr_mode
                        .apply_transform_with_path(transformer, context)?,
                    generated_keyword: generated_keyword
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::ColumnOption::Options(field0) => {
                    sqlparser::ast::ColumnOption::Options(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Identity(field0) => {
                    sqlparser::ast::ColumnOption::Identity(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::OnConflict(field0) => {
                    sqlparser::ast::ColumnOption::OnConflict(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Policy(field0) => {
                    sqlparser::ast::ColumnOption::Policy(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnOption::Tags(field0) => sqlparser::ast::ColumnOption::Tags(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnOptionDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, option } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                option: option.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnPolicy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ColumnPolicy::MaskingPolicy(field0) => {
                    sqlparser::ast::ColumnPolicy::MaskingPolicy(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ColumnPolicy::ProjectionPolicy(field0) => {
                    sqlparser::ast::ColumnPolicy::ProjectionPolicy(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnPolicyProperty {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                with,
                policy_name,
                using_columns,
            } = self;
            Self {
                with: with.apply_transform_with_path(transformer, context)?,
                policy_name: policy_name.apply_transform_with_path(transformer, context)?,
                using_columns: using_columns.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CommentDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CommentDef::WithEq(field0) => sqlparser::ast::CommentDef::WithEq(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CommentDef::WithoutEq(field0) => {
                    sqlparser::ast::CommentDef::WithoutEq(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(field0) => {
                    sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CommentObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CommentObject::Column => {
                    transformer.transform(sqlparser::ast::CommentObject::Column, self, context)?
                }
                sqlparser::ast::CommentObject::Table => {
                    transformer.transform(sqlparser::ast::CommentObject::Table, self, context)?
                }
                sqlparser::ast::CommentObject::Extension => transformer.transform(
                    sqlparser::ast::CommentObject::Extension,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConflictTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ConflictTarget::Columns(field0) => {
                    sqlparser::ast::ConflictTarget::Columns(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ConflictTarget::OnConstraint(field0) => {
                    sqlparser::ast::ConflictTarget::OnConstraint(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConnectBy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                condition,
                relationships,
            } = self;
            Self {
                condition: condition.apply_transform_with_path(transformer, context)?,
                relationships: relationships.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConstraintCharacteristics {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                deferrable,
                initially,
                enforced,
            } = self;
            Self {
                deferrable: deferrable.apply_transform_with_path(transformer, context)?,
                initially: initially.apply_transform_with_path(transformer, context)?,
                enforced: enforced.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ContextModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ContextModifier::None => {
                    transformer.transform(sqlparser::ast::ContextModifier::None, self, context)?
                }
                sqlparser::ast::ContextModifier::Local => {
                    transformer.transform(sqlparser::ast::ContextModifier::Local, self, context)?
                }
                sqlparser::ast::ContextModifier::Session => transformer.transform(
                    sqlparser::ast::ContextModifier::Session,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyLegacyCsvOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CopyLegacyCsvOption::Header => transformer.transform(
                    sqlparser::ast::CopyLegacyCsvOption::Header,
                    self,
                    context,
                )?,
                sqlparser::ast::CopyLegacyCsvOption::Quote(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::Quote(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyLegacyCsvOption::Escape(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::Escape(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyLegacyCsvOption::ForceQuote(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::ForceQuote(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyLegacyOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CopyLegacyOption::Binary => transformer.transform(
                    sqlparser::ast::CopyLegacyOption::Binary,
                    self,
                    context,
                )?,
                sqlparser::ast::CopyLegacyOption::Delimiter(field0) => {
                    sqlparser::ast::CopyLegacyOption::Delimiter(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyLegacyOption::Null(field0) => {
                    sqlparser::ast::CopyLegacyOption::Null(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyLegacyOption::Csv(field0) => {
                    sqlparser::ast::CopyLegacyOption::Csv(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CopyOption::Format(field0) => sqlparser::ast::CopyOption::Format(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CopyOption::Freeze(field0) => sqlparser::ast::CopyOption::Freeze(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CopyOption::Delimiter(field0) => {
                    sqlparser::ast::CopyOption::Delimiter(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyOption::Null(field0) => sqlparser::ast::CopyOption::Null(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CopyOption::Header(field0) => sqlparser::ast::CopyOption::Header(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CopyOption::Quote(field0) => sqlparser::ast::CopyOption::Quote(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CopyOption::Escape(field0) => sqlparser::ast::CopyOption::Escape(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::CopyOption::ForceQuote(field0) => {
                    sqlparser::ast::CopyOption::ForceQuote(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyOption::ForceNotNull(field0) => {
                    sqlparser::ast::CopyOption::ForceNotNull(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyOption::ForceNull(field0) => {
                    sqlparser::ast::CopyOption::ForceNull(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CopyOption::Encoding(field0) => {
                    sqlparser::ast::CopyOption::Encoding(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopySource {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CopySource::Table {
                    table_name,
                    columns,
                } => sqlparser::ast::CopySource::Table {
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::CopySource::Query(field0) => sqlparser::ast::CopySource::Query(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CopyTarget::Stdin => {
                    transformer.transform(sqlparser::ast::CopyTarget::Stdin, self, context)?
                }
                sqlparser::ast::CopyTarget::Stdout => {
                    transformer.transform(sqlparser::ast::CopyTarget::Stdout, self, context)?
                }
                sqlparser::ast::CopyTarget::File { filename } => sqlparser::ast::CopyTarget::File {
                    filename: filename.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::CopyTarget::Program { command } => {
                    sqlparser::ast::CopyTarget::Program {
                        command: command.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateFunctionBody {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CreateFunctionBody::AsBeforeOptions(field0) => {
                    sqlparser::ast::CreateFunctionBody::AsBeforeOptions(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CreateFunctionBody::AsAfterOptions(field0) => {
                    sqlparser::ast::CreateFunctionBody::AsAfterOptions(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CreateFunctionBody::Return(field0) => {
                    sqlparser::ast::CreateFunctionBody::Return(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateFunctionUsing {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CreateFunctionUsing::Jar(field0) => {
                    sqlparser::ast::CreateFunctionUsing::Jar(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CreateFunctionUsing::File(field0) => {
                    sqlparser::ast::CreateFunctionUsing::File(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CreateFunctionUsing::Archive(field0) => {
                    sqlparser::ast::CreateFunctionUsing::Archive(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateIndex {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
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
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                table_name: table_name.apply_transform_with_path(transformer, context)?,
                using: using.apply_transform_with_path(transformer, context)?,
                columns: columns.apply_transform_with_path(transformer, context)?,
                unique: unique.apply_transform_with_path(transformer, context)?,
                concurrently: concurrently.apply_transform_with_path(transformer, context)?,
                if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                include: include.apply_transform_with_path(transformer, context)?,
                nulls_distinct: nulls_distinct.apply_transform_with_path(transformer, context)?,
                with: with.apply_transform_with_path(transformer, context)?,
                predicate: predicate.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreatePolicyCommand {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CreatePolicyCommand::All => transformer.transform(
                    sqlparser::ast::CreatePolicyCommand::All,
                    self,
                    context,
                )?,
                sqlparser::ast::CreatePolicyCommand::Select => transformer.transform(
                    sqlparser::ast::CreatePolicyCommand::Select,
                    self,
                    context,
                )?,
                sqlparser::ast::CreatePolicyCommand::Insert => transformer.transform(
                    sqlparser::ast::CreatePolicyCommand::Insert,
                    self,
                    context,
                )?,
                sqlparser::ast::CreatePolicyCommand::Update => transformer.transform(
                    sqlparser::ast::CreatePolicyCommand::Update,
                    self,
                    context,
                )?,
                sqlparser::ast::CreatePolicyCommand::Delete => transformer.transform(
                    sqlparser::ast::CreatePolicyCommand::Delete,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreatePolicyType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CreatePolicyType::Permissive => transformer.transform(
                    sqlparser::ast::CreatePolicyType::Permissive,
                    self,
                    context,
                )?,
                sqlparser::ast::CreatePolicyType::Restrictive => transformer.transform(
                    sqlparser::ast::CreatePolicyType::Restrictive,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateTable {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
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
            Self {
                or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                temporary: temporary.apply_transform_with_path(transformer, context)?,
                external: external.apply_transform_with_path(transformer, context)?,
                global: global.apply_transform_with_path(transformer, context)?,
                if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                transient: transient.apply_transform_with_path(transformer, context)?,
                volatile: volatile.apply_transform_with_path(transformer, context)?,
                name: name.apply_transform_with_path(transformer, context)?,
                columns: columns.apply_transform_with_path(transformer, context)?,
                constraints: constraints.apply_transform_with_path(transformer, context)?,
                hive_distribution: hive_distribution
                    .apply_transform_with_path(transformer, context)?,
                hive_formats: hive_formats.apply_transform_with_path(transformer, context)?,
                table_properties: table_properties
                    .apply_transform_with_path(transformer, context)?,
                with_options: with_options.apply_transform_with_path(transformer, context)?,
                file_format: file_format.apply_transform_with_path(transformer, context)?,
                location: location.apply_transform_with_path(transformer, context)?,
                query: query.apply_transform_with_path(transformer, context)?,
                without_rowid: without_rowid.apply_transform_with_path(transformer, context)?,
                like: like.apply_transform_with_path(transformer, context)?,
                clone: clone.apply_transform_with_path(transformer, context)?,
                engine: engine.apply_transform_with_path(transformer, context)?,
                comment: comment.apply_transform_with_path(transformer, context)?,
                auto_increment_offset: auto_increment_offset
                    .apply_transform_with_path(transformer, context)?,
                default_charset: default_charset.apply_transform_with_path(transformer, context)?,
                collation: collation.apply_transform_with_path(transformer, context)?,
                on_commit: on_commit.apply_transform_with_path(transformer, context)?,
                on_cluster: on_cluster.apply_transform_with_path(transformer, context)?,
                primary_key: primary_key.apply_transform_with_path(transformer, context)?,
                order_by: order_by.apply_transform_with_path(transformer, context)?,
                partition_by: partition_by.apply_transform_with_path(transformer, context)?,
                cluster_by: cluster_by.apply_transform_with_path(transformer, context)?,
                clustered_by: clustered_by.apply_transform_with_path(transformer, context)?,
                options: options.apply_transform_with_path(transformer, context)?,
                strict: strict.apply_transform_with_path(transformer, context)?,
                copy_grants: copy_grants.apply_transform_with_path(transformer, context)?,
                enable_schema_evolution: enable_schema_evolution
                    .apply_transform_with_path(transformer, context)?,
                change_tracking: change_tracking.apply_transform_with_path(transformer, context)?,
                data_retention_time_in_days: data_retention_time_in_days
                    .apply_transform_with_path(transformer, context)?,
                max_data_extension_time_in_days: max_data_extension_time_in_days
                    .apply_transform_with_path(transformer, context)?,
                default_ddl_collation: default_ddl_collation
                    .apply_transform_with_path(transformer, context)?,
                with_aggregation_policy: with_aggregation_policy
                    .apply_transform_with_path(transformer, context)?,
                with_row_access_policy: with_row_access_policy
                    .apply_transform_with_path(transformer, context)?,
                with_tags: with_tags.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateTableOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CreateTableOptions::None => transformer.transform(
                    sqlparser::ast::CreateTableOptions::None,
                    self,
                    context,
                )?,
                sqlparser::ast::CreateTableOptions::With(field0) => {
                    sqlparser::ast::CreateTableOptions::With(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::CreateTableOptions::Options(field0) => {
                    sqlparser::ast::CreateTableOptions::Options(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Cte {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                alias,
                query,
                from,
                materialized,
            } = self;
            Self {
                alias: alias.apply_transform_with_path(transformer, context)?,
                query: query.apply_transform_with_path(transformer, context)?,
                from: from.apply_transform_with_path(transformer, context)?,
                materialized: materialized.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CteAsMaterialized {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::CteAsMaterialized::Materialized => transformer.transform(
                    sqlparser::ast::CteAsMaterialized::Materialized,
                    self,
                    context,
                )?,
                sqlparser::ast::CteAsMaterialized::NotMaterialized => transformer.transform(
                    sqlparser::ast::CteAsMaterialized::NotMaterialized,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DataType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DataType::Character(field0) => sqlparser::ast::DataType::Character(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Char(field0) => sqlparser::ast::DataType::Char(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::CharacterVarying(field0) => {
                    sqlparser::ast::DataType::CharacterVarying(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::CharVarying(field0) => {
                    sqlparser::ast::DataType::CharVarying(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Varchar(field0) => sqlparser::ast::DataType::Varchar(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Nvarchar(field0) => sqlparser::ast::DataType::Nvarchar(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Uuid => {
                    transformer.transform(sqlparser::ast::DataType::Uuid, self, context)?
                }
                sqlparser::ast::DataType::CharacterLargeObject(field0) => {
                    sqlparser::ast::DataType::CharacterLargeObject(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::CharLargeObject(field0) => {
                    sqlparser::ast::DataType::CharLargeObject(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Clob(field0) => sqlparser::ast::DataType::Clob(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Binary(field0) => sqlparser::ast::DataType::Binary(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Varbinary(field0) => sqlparser::ast::DataType::Varbinary(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Blob(field0) => sqlparser::ast::DataType::Blob(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Bytes(field0) => sqlparser::ast::DataType::Bytes(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Numeric(field0) => sqlparser::ast::DataType::Numeric(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Decimal(field0) => sqlparser::ast::DataType::Decimal(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::BigNumeric(field0) => {
                    sqlparser::ast::DataType::BigNumeric(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::BigDecimal(field0) => {
                    sqlparser::ast::DataType::BigDecimal(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Dec(field0) => sqlparser::ast::DataType::Dec(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Float(field0) => sqlparser::ast::DataType::Float(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::TinyInt(field0) => sqlparser::ast::DataType::TinyInt(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::UnsignedTinyInt(field0) => {
                    sqlparser::ast::DataType::UnsignedTinyInt(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Int2(field0) => sqlparser::ast::DataType::Int2(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::UnsignedInt2(field0) => {
                    sqlparser::ast::DataType::UnsignedInt2(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::SmallInt(field0) => sqlparser::ast::DataType::SmallInt(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::UnsignedSmallInt(field0) => {
                    sqlparser::ast::DataType::UnsignedSmallInt(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::MediumInt(field0) => sqlparser::ast::DataType::MediumInt(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::UnsignedMediumInt(field0) => {
                    sqlparser::ast::DataType::UnsignedMediumInt(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Int(field0) => sqlparser::ast::DataType::Int(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Int4(field0) => sqlparser::ast::DataType::Int4(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Int8(field0) => sqlparser::ast::DataType::Int8(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Int16 => {
                    transformer.transform(sqlparser::ast::DataType::Int16, self, context)?
                }
                sqlparser::ast::DataType::Int32 => {
                    transformer.transform(sqlparser::ast::DataType::Int32, self, context)?
                }
                sqlparser::ast::DataType::Int64 => {
                    transformer.transform(sqlparser::ast::DataType::Int64, self, context)?
                }
                sqlparser::ast::DataType::Int128 => {
                    transformer.transform(sqlparser::ast::DataType::Int128, self, context)?
                }
                sqlparser::ast::DataType::Int256 => {
                    transformer.transform(sqlparser::ast::DataType::Int256, self, context)?
                }
                sqlparser::ast::DataType::Integer(field0) => sqlparser::ast::DataType::Integer(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::UnsignedInt(field0) => {
                    sqlparser::ast::DataType::UnsignedInt(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInt4(field0) => {
                    sqlparser::ast::DataType::UnsignedInt4(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInteger(field0) => {
                    sqlparser::ast::DataType::UnsignedInteger(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::UInt8 => {
                    transformer.transform(sqlparser::ast::DataType::UInt8, self, context)?
                }
                sqlparser::ast::DataType::UInt16 => {
                    transformer.transform(sqlparser::ast::DataType::UInt16, self, context)?
                }
                sqlparser::ast::DataType::UInt32 => {
                    transformer.transform(sqlparser::ast::DataType::UInt32, self, context)?
                }
                sqlparser::ast::DataType::UInt64 => {
                    transformer.transform(sqlparser::ast::DataType::UInt64, self, context)?
                }
                sqlparser::ast::DataType::UInt128 => {
                    transformer.transform(sqlparser::ast::DataType::UInt128, self, context)?
                }
                sqlparser::ast::DataType::UInt256 => {
                    transformer.transform(sqlparser::ast::DataType::UInt256, self, context)?
                }
                sqlparser::ast::DataType::BigInt(field0) => sqlparser::ast::DataType::BigInt(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::UnsignedBigInt(field0) => {
                    sqlparser::ast::DataType::UnsignedBigInt(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInt8(field0) => {
                    sqlparser::ast::DataType::UnsignedInt8(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Float4 => {
                    transformer.transform(sqlparser::ast::DataType::Float4, self, context)?
                }
                sqlparser::ast::DataType::Float32 => {
                    transformer.transform(sqlparser::ast::DataType::Float32, self, context)?
                }
                sqlparser::ast::DataType::Float64 => {
                    transformer.transform(sqlparser::ast::DataType::Float64, self, context)?
                }
                sqlparser::ast::DataType::Real => {
                    transformer.transform(sqlparser::ast::DataType::Real, self, context)?
                }
                sqlparser::ast::DataType::Float8 => {
                    transformer.transform(sqlparser::ast::DataType::Float8, self, context)?
                }
                sqlparser::ast::DataType::Double => {
                    transformer.transform(sqlparser::ast::DataType::Double, self, context)?
                }
                sqlparser::ast::DataType::DoublePrecision => transformer.transform(
                    sqlparser::ast::DataType::DoublePrecision,
                    self,
                    context,
                )?,
                sqlparser::ast::DataType::Bool => {
                    transformer.transform(sqlparser::ast::DataType::Bool, self, context)?
                }
                sqlparser::ast::DataType::Boolean => {
                    transformer.transform(sqlparser::ast::DataType::Boolean, self, context)?
                }
                sqlparser::ast::DataType::Date => {
                    transformer.transform(sqlparser::ast::DataType::Date, self, context)?
                }
                sqlparser::ast::DataType::Date32 => {
                    transformer.transform(sqlparser::ast::DataType::Date32, self, context)?
                }
                sqlparser::ast::DataType::Time(field0, field1) => sqlparser::ast::DataType::Time(
                    field0.apply_transform_with_path(transformer, context)?,
                    field1.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Datetime(field0) => sqlparser::ast::DataType::Datetime(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Datetime64(field0, field1) => {
                    sqlparser::ast::DataType::Datetime64(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Timestamp(field0, field1) => {
                    sqlparser::ast::DataType::Timestamp(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Interval => {
                    transformer.transform(sqlparser::ast::DataType::Interval, self, context)?
                }
                sqlparser::ast::DataType::JSON => {
                    transformer.transform(sqlparser::ast::DataType::JSON, self, context)?
                }
                sqlparser::ast::DataType::JSONB => {
                    transformer.transform(sqlparser::ast::DataType::JSONB, self, context)?
                }
                sqlparser::ast::DataType::Regclass => {
                    transformer.transform(sqlparser::ast::DataType::Regclass, self, context)?
                }
                sqlparser::ast::DataType::Text => {
                    transformer.transform(sqlparser::ast::DataType::Text, self, context)?
                }
                sqlparser::ast::DataType::String(field0) => sqlparser::ast::DataType::String(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::FixedString(field0) => {
                    sqlparser::ast::DataType::FixedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Bytea => {
                    transformer.transform(sqlparser::ast::DataType::Bytea, self, context)?
                }
                sqlparser::ast::DataType::Custom(field0, field1) => {
                    sqlparser::ast::DataType::Custom(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Array(field0) => sqlparser::ast::DataType::Array(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Map(field0, field1) => sqlparser::ast::DataType::Map(
                    field0.apply_transform_with_path(transformer, context)?,
                    field1.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Tuple(field0) => sqlparser::ast::DataType::Tuple(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Nested(field0) => sqlparser::ast::DataType::Nested(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Enum(field0) => sqlparser::ast::DataType::Enum(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Set(field0) => sqlparser::ast::DataType::Set(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Struct(field0, field1) => {
                    sqlparser::ast::DataType::Struct(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Union(field0) => sqlparser::ast::DataType::Union(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::Nullable(field0) => sqlparser::ast::DataType::Nullable(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DataType::LowCardinality(field0) => {
                    sqlparser::ast::DataType::LowCardinality(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DataType::Unspecified => {
                    transformer.transform(sqlparser::ast::DataType::Unspecified, self, context)?
                }
                sqlparser::ast::DataType::Trigger => {
                    transformer.transform(sqlparser::ast::DataType::Trigger, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DateTimeField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DateTimeField::Year => {
                    transformer.transform(sqlparser::ast::DateTimeField::Year, self, context)?
                }
                sqlparser::ast::DateTimeField::Month => {
                    transformer.transform(sqlparser::ast::DateTimeField::Month, self, context)?
                }
                sqlparser::ast::DateTimeField::Week(field0) => sqlparser::ast::DateTimeField::Week(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::DateTimeField::Day => {
                    transformer.transform(sqlparser::ast::DateTimeField::Day, self, context)?
                }
                sqlparser::ast::DateTimeField::DayOfWeek => transformer.transform(
                    sqlparser::ast::DateTimeField::DayOfWeek,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::DayOfYear => transformer.transform(
                    sqlparser::ast::DateTimeField::DayOfYear,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Date => {
                    transformer.transform(sqlparser::ast::DateTimeField::Date, self, context)?
                }
                sqlparser::ast::DateTimeField::Datetime => {
                    transformer.transform(sqlparser::ast::DateTimeField::Datetime, self, context)?
                }
                sqlparser::ast::DateTimeField::Hour => {
                    transformer.transform(sqlparser::ast::DateTimeField::Hour, self, context)?
                }
                sqlparser::ast::DateTimeField::Minute => {
                    transformer.transform(sqlparser::ast::DateTimeField::Minute, self, context)?
                }
                sqlparser::ast::DateTimeField::Second => {
                    transformer.transform(sqlparser::ast::DateTimeField::Second, self, context)?
                }
                sqlparser::ast::DateTimeField::Century => {
                    transformer.transform(sqlparser::ast::DateTimeField::Century, self, context)?
                }
                sqlparser::ast::DateTimeField::Decade => {
                    transformer.transform(sqlparser::ast::DateTimeField::Decade, self, context)?
                }
                sqlparser::ast::DateTimeField::Dow => {
                    transformer.transform(sqlparser::ast::DateTimeField::Dow, self, context)?
                }
                sqlparser::ast::DateTimeField::Doy => {
                    transformer.transform(sqlparser::ast::DateTimeField::Doy, self, context)?
                }
                sqlparser::ast::DateTimeField::Epoch => {
                    transformer.transform(sqlparser::ast::DateTimeField::Epoch, self, context)?
                }
                sqlparser::ast::DateTimeField::Isodow => {
                    transformer.transform(sqlparser::ast::DateTimeField::Isodow, self, context)?
                }
                sqlparser::ast::DateTimeField::IsoWeek => {
                    transformer.transform(sqlparser::ast::DateTimeField::IsoWeek, self, context)?
                }
                sqlparser::ast::DateTimeField::Isoyear => {
                    transformer.transform(sqlparser::ast::DateTimeField::Isoyear, self, context)?
                }
                sqlparser::ast::DateTimeField::Julian => {
                    transformer.transform(sqlparser::ast::DateTimeField::Julian, self, context)?
                }
                sqlparser::ast::DateTimeField::Microsecond => transformer.transform(
                    sqlparser::ast::DateTimeField::Microsecond,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Microseconds => transformer.transform(
                    sqlparser::ast::DateTimeField::Microseconds,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Millenium => transformer.transform(
                    sqlparser::ast::DateTimeField::Millenium,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Millennium => transformer.transform(
                    sqlparser::ast::DateTimeField::Millennium,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Millisecond => transformer.transform(
                    sqlparser::ast::DateTimeField::Millisecond,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Milliseconds => transformer.transform(
                    sqlparser::ast::DateTimeField::Milliseconds,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Nanosecond => transformer.transform(
                    sqlparser::ast::DateTimeField::Nanosecond,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Nanoseconds => transformer.transform(
                    sqlparser::ast::DateTimeField::Nanoseconds,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Quarter => {
                    transformer.transform(sqlparser::ast::DateTimeField::Quarter, self, context)?
                }
                sqlparser::ast::DateTimeField::Time => {
                    transformer.transform(sqlparser::ast::DateTimeField::Time, self, context)?
                }
                sqlparser::ast::DateTimeField::Timezone => {
                    transformer.transform(sqlparser::ast::DateTimeField::Timezone, self, context)?
                }
                sqlparser::ast::DateTimeField::TimezoneAbbr => transformer.transform(
                    sqlparser::ast::DateTimeField::TimezoneAbbr,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::TimezoneHour => transformer.transform(
                    sqlparser::ast::DateTimeField::TimezoneHour,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::TimezoneMinute => transformer.transform(
                    sqlparser::ast::DateTimeField::TimezoneMinute,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::TimezoneRegion => transformer.transform(
                    sqlparser::ast::DateTimeField::TimezoneRegion,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::NoDateTime => transformer.transform(
                    sqlparser::ast::DateTimeField::NoDateTime,
                    self,
                    context,
                )?,
                sqlparser::ast::DateTimeField::Custom(field0) => {
                    sqlparser::ast::DateTimeField::Custom(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Declare {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
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
            Self {
                names: names.apply_transform_with_path(transformer, context)?,
                data_type: data_type.apply_transform_with_path(transformer, context)?,
                assignment: assignment.apply_transform_with_path(transformer, context)?,
                declare_type: declare_type.apply_transform_with_path(transformer, context)?,
                binary: binary.apply_transform_with_path(transformer, context)?,
                sensitive: sensitive.apply_transform_with_path(transformer, context)?,
                scroll: scroll.apply_transform_with_path(transformer, context)?,
                hold: hold.apply_transform_with_path(transformer, context)?,
                for_query: for_query.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeclareAssignment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DeclareAssignment::Expr(field0) => {
                    sqlparser::ast::DeclareAssignment::Expr(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::Default(field0) => {
                    sqlparser::ast::DeclareAssignment::Default(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::DuckAssignment(field0) => {
                    sqlparser::ast::DeclareAssignment::DuckAssignment(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::For(field0) => {
                    sqlparser::ast::DeclareAssignment::For(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::MsSqlAssignment(field0) => {
                    sqlparser::ast::DeclareAssignment::MsSqlAssignment(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeclareType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DeclareType::Cursor => {
                    transformer.transform(sqlparser::ast::DeclareType::Cursor, self, context)?
                }
                sqlparser::ast::DeclareType::ResultSet => {
                    transformer.transform(sqlparser::ast::DeclareType::ResultSet, self, context)?
                }
                sqlparser::ast::DeclareType::Exception => {
                    transformer.transform(sqlparser::ast::DeclareType::Exception, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Deduplicate {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Deduplicate::All => {
                    transformer.transform(sqlparser::ast::Deduplicate::All, self, context)?
                }
                sqlparser::ast::Deduplicate::ByExpression(field0) => {
                    sqlparser::ast::Deduplicate::ByExpression(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeferrableInitial {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DeferrableInitial::Immediate => transformer.transform(
                    sqlparser::ast::DeferrableInitial::Immediate,
                    self,
                    context,
                )?,
                sqlparser::ast::DeferrableInitial::Deferred => transformer.transform(
                    sqlparser::ast::DeferrableInitial::Deferred,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Delete {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                tables,
                from,
                using,
                selection,
                returning,
                order_by,
                limit,
            } = self;
            Self {
                tables: tables.apply_transform_with_path(transformer, context)?,
                from: from.apply_transform_with_path(transformer, context)?,
                using: using.apply_transform_with_path(transformer, context)?,
                selection: selection.apply_transform_with_path(transformer, context)?,
                returning: returning.apply_transform_with_path(transformer, context)?,
                order_by: order_by.apply_transform_with_path(transformer, context)?,
                limit: limit.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DescribeAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DescribeAlias::Describe => {
                    transformer.transform(sqlparser::ast::DescribeAlias::Describe, self, context)?
                }
                sqlparser::ast::DescribeAlias::Explain => {
                    transformer.transform(sqlparser::ast::DescribeAlias::Explain, self, context)?
                }
                sqlparser::ast::DescribeAlias::Desc => {
                    transformer.transform(sqlparser::ast::DescribeAlias::Desc, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DictionaryField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DiscardObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DiscardObject::ALL => {
                    transformer.transform(sqlparser::ast::DiscardObject::ALL, self, context)?
                }
                sqlparser::ast::DiscardObject::PLANS => {
                    transformer.transform(sqlparser::ast::DiscardObject::PLANS, self, context)?
                }
                sqlparser::ast::DiscardObject::SEQUENCES => transformer.transform(
                    sqlparser::ast::DiscardObject::SEQUENCES,
                    self,
                    context,
                )?,
                sqlparser::ast::DiscardObject::TEMP => {
                    transformer.transform(sqlparser::ast::DiscardObject::TEMP, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Distinct {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Distinct::Distinct => {
                    transformer.transform(sqlparser::ast::Distinct::Distinct, self, context)?
                }
                sqlparser::ast::Distinct::On(field0) => sqlparser::ast::Distinct::On(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DoUpdate {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                assignments,
                selection,
            } = self;
            Self {
                assignments: assignments.apply_transform_with_path(transformer, context)?,
                selection: selection.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DollarQuotedString {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { value, tag } = self;
            Self {
                value: value.apply_transform_with_path(transformer, context)?,
                tag: tag.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DuplicateTreatment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::DuplicateTreatment::Distinct => transformer.transform(
                    sqlparser::ast::DuplicateTreatment::Distinct,
                    self,
                    context,
                )?,
                sqlparser::ast::DuplicateTreatment::All => {
                    transformer.transform(sqlparser::ast::DuplicateTreatment::All, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::EmptyMatchesMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::EmptyMatchesMode::Show => {
                    transformer.transform(sqlparser::ast::EmptyMatchesMode::Show, self, context)?
                }
                sqlparser::ast::EmptyMatchesMode::Omit => {
                    transformer.transform(sqlparser::ast::EmptyMatchesMode::Omit, self, context)?
                }
                sqlparser::ast::EmptyMatchesMode::WithUnmatched => transformer.transform(
                    sqlparser::ast::EmptyMatchesMode::WithUnmatched,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExactNumberInfo {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ExactNumberInfo::None => {
                    transformer.transform(sqlparser::ast::ExactNumberInfo::None, self, context)?
                }
                sqlparser::ast::ExactNumberInfo::Precision(field0) => {
                    sqlparser::ast::ExactNumberInfo::Precision(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ExactNumberInfo::PrecisionAndScale(field0, field1) => {
                    sqlparser::ast::ExactNumberInfo::PrecisionAndScale(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExceptSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                first_element,
                additional_elements,
            } = self;
            Self {
                first_element: first_element.apply_transform_with_path(transformer, context)?,
                additional_elements: additional_elements
                    .apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExcludeSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ExcludeSelectItem::Single(field0) => {
                    sqlparser::ast::ExcludeSelectItem::Single(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ExcludeSelectItem::Multiple(field0) => {
                    sqlparser::ast::ExcludeSelectItem::Multiple(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Expr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Expr::Identifier(field0) => sqlparser::ast::Expr::Identifier(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::CompoundIdentifier(field0) => {
                    sqlparser::ast::Expr::CompoundIdentifier(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Expr::JsonAccess { value, path } => {
                    sqlparser::ast::Expr::JsonAccess {
                        value: value.apply_transform_with_path(transformer, context)?,
                        path: path.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::CompositeAccess { expr, key } => {
                    sqlparser::ast::Expr::CompositeAccess {
                        expr: expr.apply_transform_with_path(transformer, context)?,
                        key: key.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::IsFalse(field0) => sqlparser::ast::Expr::IsFalse(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsNotFalse(field0) => sqlparser::ast::Expr::IsNotFalse(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsTrue(field0) => sqlparser::ast::Expr::IsTrue(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsNotTrue(field0) => sqlparser::ast::Expr::IsNotTrue(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsNull(field0) => sqlparser::ast::Expr::IsNull(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsNotNull(field0) => sqlparser::ast::Expr::IsNotNull(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsUnknown(field0) => sqlparser::ast::Expr::IsUnknown(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsNotUnknown(field0) => sqlparser::ast::Expr::IsNotUnknown(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IsDistinctFrom(field0, field1) => {
                    sqlparser::ast::Expr::IsDistinctFrom(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Expr::IsNotDistinctFrom(field0, field1) => {
                    sqlparser::ast::Expr::IsNotDistinctFrom(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Expr::InList {
                    expr,
                    list,
                    negated,
                } => sqlparser::ast::Expr::InList {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    list: list.apply_transform_with_path(transformer, context)?,
                    negated: negated.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::InSubquery {
                    expr,
                    subquery,
                    negated,
                } => sqlparser::ast::Expr::InSubquery {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    subquery: subquery.apply_transform_with_path(transformer, context)?,
                    negated: negated.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::InUnnest {
                    expr,
                    array_expr,
                    negated,
                } => sqlparser::ast::Expr::InUnnest {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    array_expr: array_expr.apply_transform_with_path(transformer, context)?,
                    negated: negated.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Between {
                    expr,
                    negated,
                    low,
                    high,
                } => sqlparser::ast::Expr::Between {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    negated: negated.apply_transform_with_path(transformer, context)?,
                    low: low.apply_transform_with_path(transformer, context)?,
                    high: high.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::BinaryOp { left, op, right } => {
                    sqlparser::ast::Expr::BinaryOp {
                        left: left.apply_transform_with_path(transformer, context)?,
                        op: op.apply_transform_with_path(transformer, context)?,
                        right: right.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::Like {
                    negated,
                    any,
                    expr,
                    pattern,
                    escape_char,
                } => sqlparser::ast::Expr::Like {
                    negated: negated.apply_transform_with_path(transformer, context)?,
                    any: any.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    pattern: pattern.apply_transform_with_path(transformer, context)?,
                    escape_char: escape_char.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::ILike {
                    negated,
                    any,
                    expr,
                    pattern,
                    escape_char,
                } => sqlparser::ast::Expr::ILike {
                    negated: negated.apply_transform_with_path(transformer, context)?,
                    any: any.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    pattern: pattern.apply_transform_with_path(transformer, context)?,
                    escape_char: escape_char.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::SimilarTo {
                    negated,
                    expr,
                    pattern,
                    escape_char,
                } => sqlparser::ast::Expr::SimilarTo {
                    negated: negated.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    pattern: pattern.apply_transform_with_path(transformer, context)?,
                    escape_char: escape_char.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::RLike {
                    negated,
                    expr,
                    pattern,
                    regexp,
                } => sqlparser::ast::Expr::RLike {
                    negated: negated.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    pattern: pattern.apply_transform_with_path(transformer, context)?,
                    regexp: regexp.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::AnyOp {
                    left,
                    compare_op,
                    right,
                    is_some,
                } => sqlparser::ast::Expr::AnyOp {
                    left: left.apply_transform_with_path(transformer, context)?,
                    compare_op: compare_op.apply_transform_with_path(transformer, context)?,
                    right: right.apply_transform_with_path(transformer, context)?,
                    is_some: is_some.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::AllOp {
                    left,
                    compare_op,
                    right,
                } => sqlparser::ast::Expr::AllOp {
                    left: left.apply_transform_with_path(transformer, context)?,
                    compare_op: compare_op.apply_transform_with_path(transformer, context)?,
                    right: right.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::UnaryOp { op, expr } => sqlparser::ast::Expr::UnaryOp {
                    op: op.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Convert {
                    is_try,
                    expr,
                    data_type,
                    charset,
                    target_before_value,
                    styles,
                } => sqlparser::ast::Expr::Convert {
                    is_try: is_try.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    data_type: data_type.apply_transform_with_path(transformer, context)?,
                    charset: charset.apply_transform_with_path(transformer, context)?,
                    target_before_value: target_before_value
                        .apply_transform_with_path(transformer, context)?,
                    styles: styles.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Cast {
                    kind,
                    expr,
                    data_type,
                    format,
                } => sqlparser::ast::Expr::Cast {
                    kind: kind.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    data_type: data_type.apply_transform_with_path(transformer, context)?,
                    format: format.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::AtTimeZone {
                    timestamp,
                    time_zone,
                } => sqlparser::ast::Expr::AtTimeZone {
                    timestamp: timestamp.apply_transform_with_path(transformer, context)?,
                    time_zone: time_zone.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Extract {
                    field,
                    syntax,
                    expr,
                } => sqlparser::ast::Expr::Extract {
                    field: field.apply_transform_with_path(transformer, context)?,
                    syntax: syntax.apply_transform_with_path(transformer, context)?,
                    expr: expr.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Ceil { expr, field } => sqlparser::ast::Expr::Ceil {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    field: field.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Floor { expr, field } => sqlparser::ast::Expr::Floor {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    field: field.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Position { expr, r#in } => sqlparser::ast::Expr::Position {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    r#in: r#in.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Substring {
                    expr,
                    substring_from,
                    substring_for,
                    special,
                } => sqlparser::ast::Expr::Substring {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    substring_from: substring_from
                        .apply_transform_with_path(transformer, context)?,
                    substring_for: substring_for.apply_transform_with_path(transformer, context)?,
                    special: special.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Trim {
                    expr,
                    trim_where,
                    trim_what,
                    trim_characters,
                } => sqlparser::ast::Expr::Trim {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    trim_where: trim_where.apply_transform_with_path(transformer, context)?,
                    trim_what: trim_what.apply_transform_with_path(transformer, context)?,
                    trim_characters: trim_characters
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Overlay {
                    expr,
                    overlay_what,
                    overlay_from,
                    overlay_for,
                } => sqlparser::ast::Expr::Overlay {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    overlay_what: overlay_what.apply_transform_with_path(transformer, context)?,
                    overlay_from: overlay_from.apply_transform_with_path(transformer, context)?,
                    overlay_for: overlay_for.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Collate { expr, collation } => {
                    sqlparser::ast::Expr::Collate {
                        expr: expr.apply_transform_with_path(transformer, context)?,
                        collation: collation.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::Nested(field0) => sqlparser::ast::Expr::Nested(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Value(field0) => sqlparser::ast::Expr::Value(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::IntroducedString { introducer, value } => {
                    sqlparser::ast::Expr::IntroducedString {
                        introducer: introducer.apply_transform_with_path(transformer, context)?,
                        value: value.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::TypedString { data_type, value } => {
                    sqlparser::ast::Expr::TypedString {
                        data_type: data_type.apply_transform_with_path(transformer, context)?,
                        value: value.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::MapAccess { column, keys } => {
                    sqlparser::ast::Expr::MapAccess {
                        column: column.apply_transform_with_path(transformer, context)?,
                        keys: keys.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::Function(field0) => sqlparser::ast::Expr::Function(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Case {
                    operand,
                    conditions,
                    results,
                    else_result,
                } => sqlparser::ast::Expr::Case {
                    operand: operand.apply_transform_with_path(transformer, context)?,
                    conditions: conditions.apply_transform_with_path(transformer, context)?,
                    results: results.apply_transform_with_path(transformer, context)?,
                    else_result: else_result.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Exists { subquery, negated } => {
                    sqlparser::ast::Expr::Exists {
                        subquery: subquery.apply_transform_with_path(transformer, context)?,
                        negated: negated.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::Subquery(field0) => sqlparser::ast::Expr::Subquery(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::GroupingSets(field0) => sqlparser::ast::Expr::GroupingSets(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Cube(field0) => sqlparser::ast::Expr::Cube(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Rollup(field0) => sqlparser::ast::Expr::Rollup(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Tuple(field0) => sqlparser::ast::Expr::Tuple(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Struct { values, fields } => sqlparser::ast::Expr::Struct {
                    values: values.apply_transform_with_path(transformer, context)?,
                    fields: fields.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Named { expr, name } => sqlparser::ast::Expr::Named {
                    expr: expr.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Dictionary(field0) => sqlparser::ast::Expr::Dictionary(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Map(field0) => sqlparser::ast::Expr::Map(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Subscript { expr, subscript } => {
                    sqlparser::ast::Expr::Subscript {
                        expr: expr.apply_transform_with_path(transformer, context)?,
                        subscript: subscript.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Expr::Array(field0) => sqlparser::ast::Expr::Array(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Interval(field0) => sqlparser::ast::Expr::Interval(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::MatchAgainst {
                    columns,
                    match_value,
                    opt_search_modifier,
                } => sqlparser::ast::Expr::MatchAgainst {
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    match_value: match_value.apply_transform_with_path(transformer, context)?,
                    opt_search_modifier: opt_search_modifier
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Expr::Wildcard => {
                    transformer.transform(sqlparser::ast::Expr::Wildcard, self, context)?
                }
                sqlparser::ast::Expr::QualifiedWildcard(field0) => {
                    sqlparser::ast::Expr::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Expr::OuterJoin(field0) => sqlparser::ast::Expr::OuterJoin(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Prior(field0) => sqlparser::ast::Expr::Prior(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Expr::Lambda(field0) => sqlparser::ast::Expr::Lambda(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExprWithAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { expr, alias } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, context)?,
                alias: alias.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExtractSyntax {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ExtractSyntax::From => {
                    transformer.transform(sqlparser::ast::ExtractSyntax::From, self, context)?
                }
                sqlparser::ast::ExtractSyntax::Comma => {
                    transformer.transform(sqlparser::ast::ExtractSyntax::Comma, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Fetch {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                with_ties,
                percent,
                quantity,
            } = self;
            Self {
                with_ties: with_ties.apply_transform_with_path(transformer, context)?,
                percent: percent.apply_transform_with_path(transformer, context)?,
                quantity: quantity.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FetchDirection {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FetchDirection::Count { limit } => {
                    sqlparser::ast::FetchDirection::Count {
                        limit: limit.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::FetchDirection::Next => {
                    transformer.transform(sqlparser::ast::FetchDirection::Next, self, context)?
                }
                sqlparser::ast::FetchDirection::Prior => {
                    transformer.transform(sqlparser::ast::FetchDirection::Prior, self, context)?
                }
                sqlparser::ast::FetchDirection::First => {
                    transformer.transform(sqlparser::ast::FetchDirection::First, self, context)?
                }
                sqlparser::ast::FetchDirection::Last => {
                    transformer.transform(sqlparser::ast::FetchDirection::Last, self, context)?
                }
                sqlparser::ast::FetchDirection::Absolute { limit } => {
                    sqlparser::ast::FetchDirection::Absolute {
                        limit: limit.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::FetchDirection::Relative { limit } => {
                    sqlparser::ast::FetchDirection::Relative {
                        limit: limit.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::FetchDirection::All => {
                    transformer.transform(sqlparser::ast::FetchDirection::All, self, context)?
                }
                sqlparser::ast::FetchDirection::Forward { limit } => {
                    sqlparser::ast::FetchDirection::Forward {
                        limit: limit.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::FetchDirection::ForwardAll => transformer.transform(
                    sqlparser::ast::FetchDirection::ForwardAll,
                    self,
                    context,
                )?,
                sqlparser::ast::FetchDirection::Backward { limit } => {
                    sqlparser::ast::FetchDirection::Backward {
                        limit: limit.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::FetchDirection::BackwardAll => transformer.transform(
                    sqlparser::ast::FetchDirection::BackwardAll,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FileFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FileFormat::TEXTFILE => {
                    transformer.transform(sqlparser::ast::FileFormat::TEXTFILE, self, context)?
                }
                sqlparser::ast::FileFormat::SEQUENCEFILE => transformer.transform(
                    sqlparser::ast::FileFormat::SEQUENCEFILE,
                    self,
                    context,
                )?,
                sqlparser::ast::FileFormat::ORC => {
                    transformer.transform(sqlparser::ast::FileFormat::ORC, self, context)?
                }
                sqlparser::ast::FileFormat::PARQUET => {
                    transformer.transform(sqlparser::ast::FileFormat::PARQUET, self, context)?
                }
                sqlparser::ast::FileFormat::AVRO => {
                    transformer.transform(sqlparser::ast::FileFormat::AVRO, self, context)?
                }
                sqlparser::ast::FileFormat::RCFILE => {
                    transformer.transform(sqlparser::ast::FileFormat::RCFILE, self, context)?
                }
                sqlparser::ast::FileFormat::JSONFILE => {
                    transformer.transform(sqlparser::ast::FileFormat::JSONFILE, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FlushLocation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FlushLocation::NoWriteToBinlog => transformer.transform(
                    sqlparser::ast::FlushLocation::NoWriteToBinlog,
                    self,
                    context,
                )?,
                sqlparser::ast::FlushLocation::Local => {
                    transformer.transform(sqlparser::ast::FlushLocation::Local, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FlushType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FlushType::BinaryLogs => {
                    transformer.transform(sqlparser::ast::FlushType::BinaryLogs, self, context)?
                }
                sqlparser::ast::FlushType::EngineLogs => {
                    transformer.transform(sqlparser::ast::FlushType::EngineLogs, self, context)?
                }
                sqlparser::ast::FlushType::ErrorLogs => {
                    transformer.transform(sqlparser::ast::FlushType::ErrorLogs, self, context)?
                }
                sqlparser::ast::FlushType::GeneralLogs => {
                    transformer.transform(sqlparser::ast::FlushType::GeneralLogs, self, context)?
                }
                sqlparser::ast::FlushType::Hosts => {
                    transformer.transform(sqlparser::ast::FlushType::Hosts, self, context)?
                }
                sqlparser::ast::FlushType::Logs => {
                    transformer.transform(sqlparser::ast::FlushType::Logs, self, context)?
                }
                sqlparser::ast::FlushType::Privileges => {
                    transformer.transform(sqlparser::ast::FlushType::Privileges, self, context)?
                }
                sqlparser::ast::FlushType::OptimizerCosts => transformer.transform(
                    sqlparser::ast::FlushType::OptimizerCosts,
                    self,
                    context,
                )?,
                sqlparser::ast::FlushType::RelayLogs => {
                    transformer.transform(sqlparser::ast::FlushType::RelayLogs, self, context)?
                }
                sqlparser::ast::FlushType::SlowLogs => {
                    transformer.transform(sqlparser::ast::FlushType::SlowLogs, self, context)?
                }
                sqlparser::ast::FlushType::Status => {
                    transformer.transform(sqlparser::ast::FlushType::Status, self, context)?
                }
                sqlparser::ast::FlushType::UserResources => transformer.transform(
                    sqlparser::ast::FlushType::UserResources,
                    self,
                    context,
                )?,
                sqlparser::ast::FlushType::Tables => {
                    transformer.transform(sqlparser::ast::FlushType::Tables, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ForClause::Browse => {
                    transformer.transform(sqlparser::ast::ForClause::Browse, self, context)?
                }
                sqlparser::ast::ForClause::Json {
                    for_json,
                    root,
                    include_null_values,
                    without_array_wrapper,
                } => sqlparser::ast::ForClause::Json {
                    for_json: for_json.apply_transform_with_path(transformer, context)?,
                    root: root.apply_transform_with_path(transformer, context)?,
                    include_null_values: include_null_values
                        .apply_transform_with_path(transformer, context)?,
                    without_array_wrapper: without_array_wrapper
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::ForClause::Xml {
                    for_xml,
                    elements,
                    binary_base64,
                    root,
                    r#type,
                } => sqlparser::ast::ForClause::Xml {
                    for_xml: for_xml.apply_transform_with_path(transformer, context)?,
                    elements: elements.apply_transform_with_path(transformer, context)?,
                    binary_base64: binary_base64.apply_transform_with_path(transformer, context)?,
                    root: root.apply_transform_with_path(transformer, context)?,
                    r#type: r#type.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForJson {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ForJson::Auto => {
                    transformer.transform(sqlparser::ast::ForJson::Auto, self, context)?
                }
                sqlparser::ast::ForJson::Path => {
                    transformer.transform(sqlparser::ast::ForJson::Path, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForXml {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ForXml::Raw(field0) => sqlparser::ast::ForXml::Raw(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::ForXml::Auto => {
                    transformer.transform(sqlparser::ast::ForXml::Auto, self, context)?
                }
                sqlparser::ast::ForXml::Explicit => {
                    transformer.transform(sqlparser::ast::ForXml::Explicit, self, context)?
                }
                sqlparser::ast::ForXml::Path(field0) => sqlparser::ast::ForXml::Path(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FormatClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FormatClause::Identifier(field0) => {
                    sqlparser::ast::FormatClause::Identifier(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FormatClause::Null => {
                    transformer.transform(sqlparser::ast::FormatClause::Null, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FromTable {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FromTable::WithFromKeyword(field0) => {
                    sqlparser::ast::FromTable::WithFromKeyword(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FromTable::WithoutKeyword(field0) => {
                    sqlparser::ast::FromTable::WithoutKeyword(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Function {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                name,
                parameters,
                args,
                filter,
                null_treatment,
                over,
                within_group,
            } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                parameters: parameters.apply_transform_with_path(transformer, context)?,
                args: args.apply_transform_with_path(transformer, context)?,
                filter: filter.apply_transform_with_path(transformer, context)?,
                null_treatment: null_treatment.apply_transform_with_path(transformer, context)?,
                over: over.apply_transform_with_path(transformer, context)?,
                within_group: within_group.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArg {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArg::Named {
                    name,
                    arg,
                    operator,
                } => sqlparser::ast::FunctionArg::Named {
                    name: name.apply_transform_with_path(transformer, context)?,
                    arg: arg.apply_transform_with_path(transformer, context)?,
                    operator: operator.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::FunctionArg::Unnamed(field0) => {
                    sqlparser::ast::FunctionArg::Unnamed(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArgExpr::Expr(field0) => {
                    sqlparser::ast::FunctionArgExpr::Expr(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgExpr::QualifiedWildcard(field0) => {
                    sqlparser::ast::FunctionArgExpr::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgExpr::Wildcard => transformer.transform(
                    sqlparser::ast::FunctionArgExpr::Wildcard,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArgOperator::Equals => transformer.transform(
                    sqlparser::ast::FunctionArgOperator::Equals,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionArgOperator::RightArrow => transformer.transform(
                    sqlparser::ast::FunctionArgOperator::RightArrow,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionArgOperator::Assignment => transformer.transform(
                    sqlparser::ast::FunctionArgOperator::Assignment,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgumentClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(field0) => {
                    sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::OrderBy(field0) => {
                    sqlparser::ast::FunctionArgumentClause::OrderBy(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::Limit(field0) => {
                    sqlparser::ast::FunctionArgumentClause::Limit(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::OnOverflow(field0) => {
                    sqlparser::ast::FunctionArgumentClause::OnOverflow(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::Having(field0) => {
                    sqlparser::ast::FunctionArgumentClause::Having(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::Separator(field0) => {
                    sqlparser::ast::FunctionArgumentClause::Separator(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgumentList {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                duplicate_treatment,
                args,
                clauses,
            } = self;
            Self {
                duplicate_treatment: duplicate_treatment
                    .apply_transform_with_path(transformer, context)?,
                args: args.apply_transform_with_path(transformer, context)?,
                clauses: clauses.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArguments {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArguments::None => {
                    transformer.transform(sqlparser::ast::FunctionArguments::None, self, context)?
                }
                sqlparser::ast::FunctionArguments::Subquery(field0) => {
                    sqlparser::ast::FunctionArguments::Subquery(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::FunctionArguments::List(field0) => {
                    sqlparser::ast::FunctionArguments::List(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionBehavior {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionBehavior::Immutable => transformer.transform(
                    sqlparser::ast::FunctionBehavior::Immutable,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionBehavior::Stable => transformer.transform(
                    sqlparser::ast::FunctionBehavior::Stable,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionBehavior::Volatile => transformer.transform(
                    sqlparser::ast::FunctionBehavior::Volatile,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionCalledOnNull {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput => transformer.transform(
                    sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput => transformer
                    .transform(
                        sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput,
                        self,
                        context,
                    )?,
                sqlparser::ast::FunctionCalledOnNull::Strict => transformer.transform(
                    sqlparser::ast::FunctionCalledOnNull::Strict,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionDesc {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, args } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                args: args.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionDeterminismSpecifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionDeterminismSpecifier::Deterministic => transformer
                    .transform(
                        sqlparser::ast::FunctionDeterminismSpecifier::Deterministic,
                        self,
                        context,
                    )?,
                sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic => transformer
                    .transform(
                        sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic,
                        self,
                        context,
                    )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionParallel {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::FunctionParallel::Unsafe => transformer.transform(
                    sqlparser::ast::FunctionParallel::Unsafe,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionParallel::Restricted => transformer.transform(
                    sqlparser::ast::FunctionParallel::Restricted,
                    self,
                    context,
                )?,
                sqlparser::ast::FunctionParallel::Safe => {
                    transformer.transform(sqlparser::ast::FunctionParallel::Safe, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GeneratedAs {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::GeneratedAs::Always => {
                    transformer.transform(sqlparser::ast::GeneratedAs::Always, self, context)?
                }
                sqlparser::ast::GeneratedAs::ByDefault => {
                    transformer.transform(sqlparser::ast::GeneratedAs::ByDefault, self, context)?
                }
                sqlparser::ast::GeneratedAs::ExpStored => {
                    transformer.transform(sqlparser::ast::GeneratedAs::ExpStored, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GeneratedExpressionMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::GeneratedExpressionMode::Virtual => transformer.transform(
                    sqlparser::ast::GeneratedExpressionMode::Virtual,
                    self,
                    context,
                )?,
                sqlparser::ast::GeneratedExpressionMode::Stored => transformer.transform(
                    sqlparser::ast::GeneratedExpressionMode::Stored,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GrantObjects {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::GrantObjects::AllSequencesInSchema { schemas } => {
                    sqlparser::ast::GrantObjects::AllSequencesInSchema {
                        schemas: schemas.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::GrantObjects::AllTablesInSchema { schemas } => {
                    sqlparser::ast::GrantObjects::AllTablesInSchema {
                        schemas: schemas.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::GrantObjects::Schemas(field0) => {
                    sqlparser::ast::GrantObjects::Schemas(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::GrantObjects::Sequences(field0) => {
                    sqlparser::ast::GrantObjects::Sequences(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::GrantObjects::Tables(field0) => {
                    sqlparser::ast::GrantObjects::Tables(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GroupByExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::GroupByExpr::All(field0) => sqlparser::ast::GroupByExpr::All(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::GroupByExpr::Expressions(field0, field1) => {
                    sqlparser::ast::GroupByExpr::Expressions(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GroupByWithModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::GroupByWithModifier::Rollup => transformer.transform(
                    sqlparser::ast::GroupByWithModifier::Rollup,
                    self,
                    context,
                )?,
                sqlparser::ast::GroupByWithModifier::Cube => transformer.transform(
                    sqlparser::ast::GroupByWithModifier::Cube,
                    self,
                    context,
                )?,
                sqlparser::ast::GroupByWithModifier::Totals => transformer.transform(
                    sqlparser::ast::GroupByWithModifier::Totals,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HavingBound {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self(field0, field1) = self;
            Self(
                field0.apply_transform_with_path(transformer, context)?,
                field1.apply_transform_with_path(transformer, context)?,
            )
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HavingBoundKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::HavingBoundKind::Min => {
                    transformer.transform(sqlparser::ast::HavingBoundKind::Min, self, context)?
                }
                sqlparser::ast::HavingBoundKind::Max => {
                    transformer.transform(sqlparser::ast::HavingBoundKind::Max, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDelimiter {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::HiveDelimiter::FieldsTerminatedBy => transformer.transform(
                    sqlparser::ast::HiveDelimiter::FieldsTerminatedBy,
                    self,
                    context,
                )?,
                sqlparser::ast::HiveDelimiter::FieldsEscapedBy => transformer.transform(
                    sqlparser::ast::HiveDelimiter::FieldsEscapedBy,
                    self,
                    context,
                )?,
                sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy => transformer
                    .transform(
                        sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy,
                        self,
                        context,
                    )?,
                sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy => transformer.transform(
                    sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy,
                    self,
                    context,
                )?,
                sqlparser::ast::HiveDelimiter::LinesTerminatedBy => transformer.transform(
                    sqlparser::ast::HiveDelimiter::LinesTerminatedBy,
                    self,
                    context,
                )?,
                sqlparser::ast::HiveDelimiter::NullDefinedAs => transformer.transform(
                    sqlparser::ast::HiveDelimiter::NullDefinedAs,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDescribeFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::HiveDescribeFormat::Extended => transformer.transform(
                    sqlparser::ast::HiveDescribeFormat::Extended,
                    self,
                    context,
                )?,
                sqlparser::ast::HiveDescribeFormat::Formatted => transformer.transform(
                    sqlparser::ast::HiveDescribeFormat::Formatted,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDistributionStyle {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::HiveDistributionStyle::PARTITIONED { columns } => {
                    sqlparser::ast::HiveDistributionStyle::PARTITIONED {
                        columns: columns.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::HiveDistributionStyle::SKEWED {
                    columns,
                    on,
                    stored_as_directories,
                } => sqlparser::ast::HiveDistributionStyle::SKEWED {
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    on: on.apply_transform_with_path(transformer, context)?,
                    stored_as_directories: stored_as_directories
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::HiveDistributionStyle::NONE => transformer.transform(
                    sqlparser::ast::HiveDistributionStyle::NONE,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                row_format,
                serde_properties,
                storage,
                location,
            } = self;
            Self {
                row_format: row_format.apply_transform_with_path(transformer, context)?,
                serde_properties: serde_properties
                    .apply_transform_with_path(transformer, context)?,
                storage: storage.apply_transform_with_path(transformer, context)?,
                location: location.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveIOFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::HiveIOFormat::IOF {
                    input_format,
                    output_format,
                } => sqlparser::ast::HiveIOFormat::IOF {
                    input_format: input_format.apply_transform_with_path(transformer, context)?,
                    output_format: output_format.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::HiveIOFormat::FileFormat { format } => {
                    sqlparser::ast::HiveIOFormat::FileFormat {
                        format: format.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveRowDelimiter {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { delimiter, char } = self;
            Self {
                delimiter: delimiter.apply_transform_with_path(transformer, context)?,
                char: char.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveRowFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::HiveRowFormat::SERDE { class } => {
                    sqlparser::ast::HiveRowFormat::SERDE {
                        class: class.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::HiveRowFormat::DELIMITED { delimiters } => {
                    sqlparser::ast::HiveRowFormat::DELIMITED {
                        delimiters: delimiters.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveSetLocation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { has_set, location } = self;
            Self {
                has_set: has_set.apply_transform_with_path(transformer, context)?,
                location: location.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Ident {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { value, quote_style } = self;
            Self {
                value: value.apply_transform_with_path(transformer, context)?,
                quote_style: quote_style.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentWithAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { ident, alias } = self;
            Self {
                ident: ident.apply_transform_with_path(transformer, context)?,
                alias: alias.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityParameters {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { seed, increment } = self;
            Self {
                seed: seed.apply_transform_with_path(transformer, context)?,
                increment: increment.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityProperty {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { parameters, order } = self;
            Self {
                parameters: parameters.apply_transform_with_path(transformer, context)?,
                order: order.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyFormatKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(field0) => {
                    sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(field0) => {
                    sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::IdentityPropertyKind::Autoincrement(field0) => {
                    sqlparser::ast::IdentityPropertyKind::Autoincrement(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::IdentityPropertyKind::Identity(field0) => {
                    sqlparser::ast::IdentityPropertyKind::Identity(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyOrder {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::IdentityPropertyOrder::Order => transformer.transform(
                    sqlparser::ast::IdentityPropertyOrder::Order,
                    self,
                    context,
                )?,
                sqlparser::ast::IdentityPropertyOrder::NoOrder => transformer.transform(
                    sqlparser::ast::IdentityPropertyOrder::NoOrder,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IlikeSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { pattern } = self;
            Self {
                pattern: pattern.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IndexOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::IndexOption::Using(field0) => sqlparser::ast::IndexOption::Using(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::IndexOption::Comment(field0) => {
                    sqlparser::ast::IndexOption::Comment(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IndexType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::IndexType::BTree => {
                    transformer.transform(sqlparser::ast::IndexType::BTree, self, context)?
                }
                sqlparser::ast::IndexType::Hash => {
                    transformer.transform(sqlparser::ast::IndexType::Hash, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Insert {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
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
            Self {
                or: or.apply_transform_with_path(transformer, context)?,
                ignore: ignore.apply_transform_with_path(transformer, context)?,
                into: into.apply_transform_with_path(transformer, context)?,
                table_name: table_name.apply_transform_with_path(transformer, context)?,
                table_alias: table_alias.apply_transform_with_path(transformer, context)?,
                columns: columns.apply_transform_with_path(transformer, context)?,
                overwrite: overwrite.apply_transform_with_path(transformer, context)?,
                source: source.apply_transform_with_path(transformer, context)?,
                partitioned: partitioned.apply_transform_with_path(transformer, context)?,
                after_columns: after_columns.apply_transform_with_path(transformer, context)?,
                table: table.apply_transform_with_path(transformer, context)?,
                on: on.apply_transform_with_path(transformer, context)?,
                returning: returning.apply_transform_with_path(transformer, context)?,
                replace_into: replace_into.apply_transform_with_path(transformer, context)?,
                priority: priority.apply_transform_with_path(transformer, context)?,
                insert_alias: insert_alias.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::InsertAliases {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                row_alias,
                col_aliases,
            } = self;
            Self {
                row_alias: row_alias.apply_transform_with_path(transformer, context)?,
                col_aliases: col_aliases.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Interpolate {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { exprs } = self;
            Self {
                exprs: exprs.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::InterpolateExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { column, expr } = self;
            Self {
                column: column.apply_transform_with_path(transformer, context)?,
                expr: expr.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Interval {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                value,
                leading_field,
                leading_precision,
                last_field,
                fractional_seconds_precision,
            } = self;
            Self {
                value: value.apply_transform_with_path(transformer, context)?,
                leading_field: leading_field.apply_transform_with_path(transformer, context)?,
                leading_precision: leading_precision
                    .apply_transform_with_path(transformer, context)?,
                last_field: last_field.apply_transform_with_path(transformer, context)?,
                fractional_seconds_precision: fractional_seconds_precision
                    .apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Join {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                relation,
                global,
                join_operator,
            } = self;
            Self {
                relation: relation.apply_transform_with_path(transformer, context)?,
                global: global.apply_transform_with_path(transformer, context)?,
                join_operator: join_operator.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JoinConstraint {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::JoinConstraint::On(field0) => sqlparser::ast::JoinConstraint::On(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::JoinConstraint::Using(field0) => {
                    sqlparser::ast::JoinConstraint::Using(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinConstraint::Natural => {
                    transformer.transform(sqlparser::ast::JoinConstraint::Natural, self, context)?
                }
                sqlparser::ast::JoinConstraint::None => {
                    transformer.transform(sqlparser::ast::JoinConstraint::None, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JoinOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::JoinOperator::Inner(field0) => sqlparser::ast::JoinOperator::Inner(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::JoinOperator::LeftOuter(field0) => {
                    sqlparser::ast::JoinOperator::LeftOuter(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::RightOuter(field0) => {
                    sqlparser::ast::JoinOperator::RightOuter(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::FullOuter(field0) => {
                    sqlparser::ast::JoinOperator::FullOuter(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::CrossJoin => {
                    transformer.transform(sqlparser::ast::JoinOperator::CrossJoin, self, context)?
                }
                sqlparser::ast::JoinOperator::LeftSemi(field0) => {
                    sqlparser::ast::JoinOperator::LeftSemi(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::RightSemi(field0) => {
                    sqlparser::ast::JoinOperator::RightSemi(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::LeftAnti(field0) => {
                    sqlparser::ast::JoinOperator::LeftAnti(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::RightAnti(field0) => {
                    sqlparser::ast::JoinOperator::RightAnti(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JoinOperator::CrossApply => transformer.transform(
                    sqlparser::ast::JoinOperator::CrossApply,
                    self,
                    context,
                )?,
                sqlparser::ast::JoinOperator::OuterApply => transformer.transform(
                    sqlparser::ast::JoinOperator::OuterApply,
                    self,
                    context,
                )?,
                sqlparser::ast::JoinOperator::AsOf {
                    match_condition,
                    constraint,
                } => sqlparser::ast::JoinOperator::AsOf {
                    match_condition: match_condition
                        .apply_transform_with_path(transformer, context)?,
                    constraint: constraint.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonPath {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { path } = self;
            Self {
                path: path.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonPathElem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::JsonPathElem::Dot { key, quoted } => {
                    sqlparser::ast::JsonPathElem::Dot {
                        key: key.apply_transform_with_path(transformer, context)?,
                        quoted: quoted.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::JsonPathElem::Bracket { key } => {
                    sqlparser::ast::JsonPathElem::Bracket {
                        key: key.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::JsonTableColumn::Named(field0) => {
                    sqlparser::ast::JsonTableColumn::Named(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JsonTableColumn::ForOrdinality(field0) => {
                    sqlparser::ast::JsonTableColumn::ForOrdinality(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JsonTableColumn::Nested(field0) => {
                    sqlparser::ast::JsonTableColumn::Nested(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableColumnErrorHandling {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::JsonTableColumnErrorHandling::Null => transformer.transform(
                    sqlparser::ast::JsonTableColumnErrorHandling::Null,
                    self,
                    context,
                )?,
                sqlparser::ast::JsonTableColumnErrorHandling::Default(field0) => {
                    sqlparser::ast::JsonTableColumnErrorHandling::Default(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::JsonTableColumnErrorHandling::Error => transformer.transform(
                    sqlparser::ast::JsonTableColumnErrorHandling::Error,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableNamedColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                name,
                r#type,
                path,
                exists,
                on_empty,
                on_error,
            } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                r#type: r#type.apply_transform_with_path(transformer, context)?,
                path: path.apply_transform_with_path(transformer, context)?,
                exists: exists.apply_transform_with_path(transformer, context)?,
                on_empty: on_empty.apply_transform_with_path(transformer, context)?,
                on_error: on_error.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableNestedColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { path, columns } = self;
            Self {
                path: path.apply_transform_with_path(transformer, context)?,
                columns: columns.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::KeyOrIndexDisplay {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::KeyOrIndexDisplay::None => {
                    transformer.transform(sqlparser::ast::KeyOrIndexDisplay::None, self, context)?
                }
                sqlparser::ast::KeyOrIndexDisplay::Key => {
                    transformer.transform(sqlparser::ast::KeyOrIndexDisplay::Key, self, context)?
                }
                sqlparser::ast::KeyOrIndexDisplay::Index => transformer.transform(
                    sqlparser::ast::KeyOrIndexDisplay::Index,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::KillType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::KillType::Connection => {
                    transformer.transform(sqlparser::ast::KillType::Connection, self, context)?
                }
                sqlparser::ast::KillType::Query => {
                    transformer.transform(sqlparser::ast::KillType::Query, self, context)?
                }
                sqlparser::ast::KillType::Mutation => {
                    transformer.transform(sqlparser::ast::KillType::Mutation, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LambdaFunction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { params, body } = self;
            Self {
                params: params.apply_transform_with_path(transformer, context)?,
                body: body.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LateralView {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                lateral_view,
                lateral_view_name,
                lateral_col_alias,
                outer,
            } = self;
            Self {
                lateral_view: lateral_view.apply_transform_with_path(transformer, context)?,
                lateral_view_name: lateral_view_name
                    .apply_transform_with_path(transformer, context)?,
                lateral_col_alias: lateral_col_alias
                    .apply_transform_with_path(transformer, context)?,
                outer: outer.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ListAggOnOverflow {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ListAggOnOverflow::Error => transformer.transform(
                    sqlparser::ast::ListAggOnOverflow::Error,
                    self,
                    context,
                )?,
                sqlparser::ast::ListAggOnOverflow::Truncate { filler, with_count } => {
                    sqlparser::ast::ListAggOnOverflow::Truncate {
                        filler: filler.apply_transform_with_path(transformer, context)?,
                        with_count: with_count.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                lock_type,
                of,
                nonblock,
            } = self;
            Self {
                lock_type: lock_type.apply_transform_with_path(transformer, context)?,
                of: of.apply_transform_with_path(transformer, context)?,
                nonblock: nonblock.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockTable {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                table,
                alias,
                lock_type,
            } = self;
            Self {
                table: table.apply_transform_with_path(transformer, context)?,
                alias: alias.apply_transform_with_path(transformer, context)?,
                lock_type: lock_type.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockTableType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::LockTableType::Read { local } => {
                    sqlparser::ast::LockTableType::Read {
                        local: local.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::LockTableType::Write { low_priority } => {
                    sqlparser::ast::LockTableType::Write {
                        low_priority: low_priority
                            .apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::LockType::Share => {
                    transformer.transform(sqlparser::ast::LockType::Share, self, context)?
                }
                sqlparser::ast::LockType::Update => {
                    transformer.transform(sqlparser::ast::LockType::Update, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MacroArg {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, default_expr } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                default_expr: default_expr.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MacroDefinition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MacroDefinition::Expr(field0) => {
                    sqlparser::ast::MacroDefinition::Expr(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MacroDefinition::Table(field0) => {
                    sqlparser::ast::MacroDefinition::Table(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Map {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { entries } = self;
            Self {
                entries: entries.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapAccessKey {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { key, syntax } = self;
            Self {
                key: key.apply_transform_with_path(transformer, context)?,
                syntax: syntax.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapAccessSyntax {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MapAccessSyntax::Bracket => transformer.transform(
                    sqlparser::ast::MapAccessSyntax::Bracket,
                    self,
                    context,
                )?,
                sqlparser::ast::MapAccessSyntax::Period => {
                    transformer.transform(sqlparser::ast::MapAccessSyntax::Period, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapEntry {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MatchRecognizePattern {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MatchRecognizePattern::Symbol(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Symbol(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Exclude(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Exclude(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Permute(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Permute(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Concat(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Concat(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Group(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Group(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Alternation(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Alternation(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Repetition(field0, field1) => {
                    sqlparser::ast::MatchRecognizePattern::Repetition(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MatchRecognizeSymbol {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MatchRecognizeSymbol::Named(field0) => {
                    sqlparser::ast::MatchRecognizeSymbol::Named(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MatchRecognizeSymbol::Start => transformer.transform(
                    sqlparser::ast::MatchRecognizeSymbol::Start,
                    self,
                    context,
                )?,
                sqlparser::ast::MatchRecognizeSymbol::End => transformer.transform(
                    sqlparser::ast::MatchRecognizeSymbol::End,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Measure {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { expr, alias } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, context)?,
                alias: alias.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeAction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MergeAction::Insert(field0) => sqlparser::ast::MergeAction::Insert(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::MergeAction::Update { assignments } => {
                    sqlparser::ast::MergeAction::Update {
                        assignments: assignments.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::MergeAction::Delete => {
                    transformer.transform(sqlparser::ast::MergeAction::Delete, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                clause_kind,
                predicate,
                action,
            } = self;
            Self {
                clause_kind: clause_kind.apply_transform_with_path(transformer, context)?,
                predicate: predicate.apply_transform_with_path(transformer, context)?,
                action: action.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeClauseKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MergeClauseKind::Matched => transformer.transform(
                    sqlparser::ast::MergeClauseKind::Matched,
                    self,
                    context,
                )?,
                sqlparser::ast::MergeClauseKind::NotMatched => transformer.transform(
                    sqlparser::ast::MergeClauseKind::NotMatched,
                    self,
                    context,
                )?,
                sqlparser::ast::MergeClauseKind::NotMatchedByTarget => transformer.transform(
                    sqlparser::ast::MergeClauseKind::NotMatchedByTarget,
                    self,
                    context,
                )?,
                sqlparser::ast::MergeClauseKind::NotMatchedBySource => transformer.transform(
                    sqlparser::ast::MergeClauseKind::NotMatchedBySource,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeInsertExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { columns, kind } = self;
            Self {
                columns: columns.apply_transform_with_path(transformer, context)?,
                kind: kind.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeInsertKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MergeInsertKind::Values(field0) => {
                    sqlparser::ast::MergeInsertKind::Values(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::MergeInsertKind::Row => {
                    transformer.transform(sqlparser::ast::MergeInsertKind::Row, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MinMaxValue {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MinMaxValue::Empty => {
                    transformer.transform(sqlparser::ast::MinMaxValue::Empty, self, context)?
                }
                sqlparser::ast::MinMaxValue::None => {
                    transformer.transform(sqlparser::ast::MinMaxValue::None, self, context)?
                }
                sqlparser::ast::MinMaxValue::Some(field0) => sqlparser::ast::MinMaxValue::Some(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MySQLColumnPosition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MySQLColumnPosition::First => transformer.transform(
                    sqlparser::ast::MySQLColumnPosition::First,
                    self,
                    context,
                )?,
                sqlparser::ast::MySQLColumnPosition::After(field0) => {
                    sqlparser::ast::MySQLColumnPosition::After(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MysqlInsertPriority {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::MysqlInsertPriority::LowPriority => transformer.transform(
                    sqlparser::ast::MysqlInsertPriority::LowPriority,
                    self,
                    context,
                )?,
                sqlparser::ast::MysqlInsertPriority::Delayed => transformer.transform(
                    sqlparser::ast::MysqlInsertPriority::Delayed,
                    self,
                    context,
                )?,
                sqlparser::ast::MysqlInsertPriority::HighPriority => transformer.transform(
                    sqlparser::ast::MysqlInsertPriority::HighPriority,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NamedWindowDefinition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self(field0, field1) = self;
            Self(
                field0.apply_transform_with_path(transformer, context)?,
                field1.apply_transform_with_path(transformer, context)?,
            )
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NamedWindowExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::NamedWindowExpr::NamedWindow(field0) => {
                    sqlparser::ast::NamedWindowExpr::NamedWindow(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::NamedWindowExpr::WindowSpec(field0) => {
                    sqlparser::ast::NamedWindowExpr::WindowSpec(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NonBlock {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::NonBlock::Nowait => {
                    transformer.transform(sqlparser::ast::NonBlock::Nowait, self, context)?
                }
                sqlparser::ast::NonBlock::SkipLocked => {
                    transformer.transform(sqlparser::ast::NonBlock::SkipLocked, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NullTreatment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::NullTreatment::IgnoreNulls => transformer.transform(
                    sqlparser::ast::NullTreatment::IgnoreNulls,
                    self,
                    context,
                )?,
                sqlparser::ast::NullTreatment::RespectNulls => transformer.transform(
                    sqlparser::ast::NullTreatment::RespectNulls,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ObjectName {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self(field0) = self;
            Self(field0.apply_transform_with_path(transformer, context)?)
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ObjectType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ObjectType::Table => {
                    transformer.transform(sqlparser::ast::ObjectType::Table, self, context)?
                }
                sqlparser::ast::ObjectType::View => {
                    transformer.transform(sqlparser::ast::ObjectType::View, self, context)?
                }
                sqlparser::ast::ObjectType::Index => {
                    transformer.transform(sqlparser::ast::ObjectType::Index, self, context)?
                }
                sqlparser::ast::ObjectType::Schema => {
                    transformer.transform(sqlparser::ast::ObjectType::Schema, self, context)?
                }
                sqlparser::ast::ObjectType::Database => {
                    transformer.transform(sqlparser::ast::ObjectType::Database, self, context)?
                }
                sqlparser::ast::ObjectType::Role => {
                    transformer.transform(sqlparser::ast::ObjectType::Role, self, context)?
                }
                sqlparser::ast::ObjectType::Sequence => {
                    transformer.transform(sqlparser::ast::ObjectType::Sequence, self, context)?
                }
                sqlparser::ast::ObjectType::Stage => {
                    transformer.transform(sqlparser::ast::ObjectType::Stage, self, context)?
                }
                sqlparser::ast::ObjectType::Type => {
                    transformer.transform(sqlparser::ast::ObjectType::Type, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Offset {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { value, rows } = self;
            Self {
                value: value.apply_transform_with_path(transformer, context)?,
                rows: rows.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OffsetRows {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::OffsetRows::None => {
                    transformer.transform(sqlparser::ast::OffsetRows::None, self, context)?
                }
                sqlparser::ast::OffsetRows::Row => {
                    transformer.transform(sqlparser::ast::OffsetRows::Row, self, context)?
                }
                sqlparser::ast::OffsetRows::Rows => {
                    transformer.transform(sqlparser::ast::OffsetRows::Rows, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnCommit {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::OnCommit::DeleteRows => {
                    transformer.transform(sqlparser::ast::OnCommit::DeleteRows, self, context)?
                }
                sqlparser::ast::OnCommit::PreserveRows => {
                    transformer.transform(sqlparser::ast::OnCommit::PreserveRows, self, context)?
                }
                sqlparser::ast::OnCommit::Drop => {
                    transformer.transform(sqlparser::ast::OnCommit::Drop, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnConflict {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                conflict_target,
                action,
            } = self;
            Self {
                conflict_target: conflict_target.apply_transform_with_path(transformer, context)?,
                action: action.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnConflictAction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::OnConflictAction::DoNothing => transformer.transform(
                    sqlparser::ast::OnConflictAction::DoNothing,
                    self,
                    context,
                )?,
                sqlparser::ast::OnConflictAction::DoUpdate(field0) => {
                    sqlparser::ast::OnConflictAction::DoUpdate(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnInsert {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::OnInsert::DuplicateKeyUpdate(field0) => {
                    sqlparser::ast::OnInsert::DuplicateKeyUpdate(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::OnInsert::OnConflict(field0) => {
                    sqlparser::ast::OnInsert::OnConflict(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                _ => unreachable!(),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OperateFunctionArg {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                mode,
                name,
                data_type,
                default_expr,
            } = self;
            Self {
                mode: mode.apply_transform_with_path(transformer, context)?,
                name: name.apply_transform_with_path(transformer, context)?,
                data_type: data_type.apply_transform_with_path(transformer, context)?,
                default_expr: default_expr.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OrderBy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { exprs, interpolate } = self;
            Self {
                exprs: exprs.apply_transform_with_path(transformer, context)?,
                interpolate: interpolate.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OrderByExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                expr,
                asc,
                nulls_first,
                with_fill,
            } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, context)?,
                asc: asc.apply_transform_with_path(transformer, context)?,
                nulls_first: nulls_first.apply_transform_with_path(transformer, context)?,
                with_fill: with_fill.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Owner {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Owner::Ident(field0) => sqlparser::ast::Owner::Ident(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Owner::CurrentRole => {
                    transformer.transform(sqlparser::ast::Owner::CurrentRole, self, context)?
                }
                sqlparser::ast::Owner::CurrentUser => {
                    transformer.transform(sqlparser::ast::Owner::CurrentUser, self, context)?
                }
                sqlparser::ast::Owner::SessionUser => {
                    transformer.transform(sqlparser::ast::Owner::SessionUser, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Partition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Partition::Identifier(field0) => {
                    sqlparser::ast::Partition::Identifier(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Partition::Expr(field0) => sqlparser::ast::Partition::Expr(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Partition::Part(field0) => sqlparser::ast::Partition::Part(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Partition::Partitions(field0) => {
                    sqlparser::ast::Partition::Partitions(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::PartitionRangeDirection {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::PartitionRangeDirection::Left => transformer.transform(
                    sqlparser::ast::PartitionRangeDirection::Left,
                    self,
                    context,
                )?,
                sqlparser::ast::PartitionRangeDirection::Right => transformer.transform(
                    sqlparser::ast::PartitionRangeDirection::Right,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Password {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Password::Password(field0) => sqlparser::ast::Password::Password(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Password::NullPassword => {
                    transformer.transform(sqlparser::ast::Password::NullPassword, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::PivotValueSource {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::PivotValueSource::List(field0) => {
                    sqlparser::ast::PivotValueSource::List(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::PivotValueSource::Any(field0) => {
                    sqlparser::ast::PivotValueSource::Any(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::PivotValueSource::Subquery(field0) => {
                    sqlparser::ast::PivotValueSource::Subquery(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Privileges {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Privileges::All {
                    with_privileges_keyword,
                } => sqlparser::ast::Privileges::All {
                    with_privileges_keyword: with_privileges_keyword
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Privileges::Actions(field0) => sqlparser::ast::Privileges::Actions(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ProcedureParam {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, data_type } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                data_type: data_type.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ProjectionSelect {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                projection,
                order_by,
                group_by,
            } = self;
            Self {
                projection: projection.apply_transform_with_path(transformer, context)?,
                order_by: order_by.apply_transform_with_path(transformer, context)?,
                group_by: group_by.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Query {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
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
            Self {
                with: with.apply_transform_with_path(transformer, context)?,
                body: body.apply_transform_with_path(transformer, context)?,
                order_by: order_by.apply_transform_with_path(transformer, context)?,
                limit: limit.apply_transform_with_path(transformer, context)?,
                limit_by: limit_by.apply_transform_with_path(transformer, context)?,
                offset: offset.apply_transform_with_path(transformer, context)?,
                fetch: fetch.apply_transform_with_path(transformer, context)?,
                locks: locks.apply_transform_with_path(transformer, context)?,
                for_clause: for_clause.apply_transform_with_path(transformer, context)?,
                settings: settings.apply_transform_with_path(transformer, context)?,
                format_clause: format_clause.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReferentialAction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ReferentialAction::Restrict => transformer.transform(
                    sqlparser::ast::ReferentialAction::Restrict,
                    self,
                    context,
                )?,
                sqlparser::ast::ReferentialAction::Cascade => transformer.transform(
                    sqlparser::ast::ReferentialAction::Cascade,
                    self,
                    context,
                )?,
                sqlparser::ast::ReferentialAction::SetNull => transformer.transform(
                    sqlparser::ast::ReferentialAction::SetNull,
                    self,
                    context,
                )?,
                sqlparser::ast::ReferentialAction::NoAction => transformer.transform(
                    sqlparser::ast::ReferentialAction::NoAction,
                    self,
                    context,
                )?,
                sqlparser::ast::ReferentialAction::SetDefault => transformer.transform(
                    sqlparser::ast::ReferentialAction::SetDefault,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RenameSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::RenameSelectItem::Single(field0) => {
                    sqlparser::ast::RenameSelectItem::Single(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RenameSelectItem::Multiple(field0) => {
                    sqlparser::ast::RenameSelectItem::Multiple(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RepetitionQuantifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::RepetitionQuantifier::ZeroOrMore => transformer.transform(
                    sqlparser::ast::RepetitionQuantifier::ZeroOrMore,
                    self,
                    context,
                )?,
                sqlparser::ast::RepetitionQuantifier::OneOrMore => transformer.transform(
                    sqlparser::ast::RepetitionQuantifier::OneOrMore,
                    self,
                    context,
                )?,
                sqlparser::ast::RepetitionQuantifier::AtMostOne => transformer.transform(
                    sqlparser::ast::RepetitionQuantifier::AtMostOne,
                    self,
                    context,
                )?,
                sqlparser::ast::RepetitionQuantifier::Exactly(field0) => {
                    sqlparser::ast::RepetitionQuantifier::Exactly(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RepetitionQuantifier::AtLeast(field0) => {
                    sqlparser::ast::RepetitionQuantifier::AtLeast(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RepetitionQuantifier::AtMost(field0) => {
                    sqlparser::ast::RepetitionQuantifier::AtMost(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RepetitionQuantifier::Range(field0, field1) => {
                    sqlparser::ast::RepetitionQuantifier::Range(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReplaceSelectElement {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                expr,
                column_name,
                as_keyword,
            } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, context)?,
                column_name: column_name.apply_transform_with_path(transformer, context)?,
                as_keyword: as_keyword.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReplaceSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { items } = self;
            Self {
                items: items.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ResetConfig {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ResetConfig::ALL => {
                    transformer.transform(sqlparser::ast::ResetConfig::ALL, self, context)?
                }
                sqlparser::ast::ResetConfig::ConfigName(field0) => {
                    sqlparser::ast::ResetConfig::ConfigName(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RoleOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::RoleOption::BypassRLS(field0) => {
                    sqlparser::ast::RoleOption::BypassRLS(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::ConnectionLimit(field0) => {
                    sqlparser::ast::RoleOption::ConnectionLimit(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::CreateDB(field0) => {
                    sqlparser::ast::RoleOption::CreateDB(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::CreateRole(field0) => {
                    sqlparser::ast::RoleOption::CreateRole(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::Inherit(field0) => sqlparser::ast::RoleOption::Inherit(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::RoleOption::Login(field0) => sqlparser::ast::RoleOption::Login(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::RoleOption::Password(field0) => {
                    sqlparser::ast::RoleOption::Password(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::Replication(field0) => {
                    sqlparser::ast::RoleOption::Replication(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::SuperUser(field0) => {
                    sqlparser::ast::RoleOption::SuperUser(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::RoleOption::ValidUntil(field0) => {
                    sqlparser::ast::RoleOption::ValidUntil(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RowAccessPolicy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { policy, on } = self;
            Self {
                policy: policy.apply_transform_with_path(transformer, context)?,
                on: on.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RowsPerMatch {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::RowsPerMatch::OneRow => {
                    transformer.transform(sqlparser::ast::RowsPerMatch::OneRow, self, context)?
                }
                sqlparser::ast::RowsPerMatch::AllRows(field0) => {
                    sqlparser::ast::RowsPerMatch::AllRows(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SchemaName {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SchemaName::Simple(field0) => sqlparser::ast::SchemaName::Simple(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SchemaName::UnnamedAuthorization(field0) => {
                    sqlparser::ast::SchemaName::UnnamedAuthorization(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SchemaName::NamedAuthorization(field0, field1) => {
                    sqlparser::ast::SchemaName::NamedAuthorization(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SearchModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SearchModifier::InNaturalLanguageMode => transformer.transform(
                    sqlparser::ast::SearchModifier::InNaturalLanguageMode,
                    self,
                    context,
                )?,
                sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion => {
                    transformer.transform(
                        sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion,
                        self,
                        context,
                    )?
                }
                sqlparser::ast::SearchModifier::InBooleanMode => transformer.transform(
                    sqlparser::ast::SearchModifier::InBooleanMode,
                    self,
                    context,
                )?,
                sqlparser::ast::SearchModifier::WithQueryExpansion => transformer.transform(
                    sqlparser::ast::SearchModifier::WithQueryExpansion,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SecretOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Select {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
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
            Self {
                distinct: distinct.apply_transform_with_path(transformer, context)?,
                top: top.apply_transform_with_path(transformer, context)?,
                top_before_distinct: top_before_distinct
                    .apply_transform_with_path(transformer, context)?,
                projection: projection.apply_transform_with_path(transformer, context)?,
                into: into.apply_transform_with_path(transformer, context)?,
                from: from.apply_transform_with_path(transformer, context)?,
                lateral_views: lateral_views.apply_transform_with_path(transformer, context)?,
                prewhere: prewhere.apply_transform_with_path(transformer, context)?,
                selection: selection.apply_transform_with_path(transformer, context)?,
                group_by: group_by.apply_transform_with_path(transformer, context)?,
                cluster_by: cluster_by.apply_transform_with_path(transformer, context)?,
                distribute_by: distribute_by.apply_transform_with_path(transformer, context)?,
                sort_by: sort_by.apply_transform_with_path(transformer, context)?,
                having: having.apply_transform_with_path(transformer, context)?,
                named_window: named_window.apply_transform_with_path(transformer, context)?,
                qualify: qualify.apply_transform_with_path(transformer, context)?,
                window_before_qualify: window_before_qualify
                    .apply_transform_with_path(transformer, context)?,
                value_table_mode: value_table_mode
                    .apply_transform_with_path(transformer, context)?,
                connect_by: connect_by.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SelectInto {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                temporary,
                unlogged,
                table,
                name,
            } = self;
            Self {
                temporary: temporary.apply_transform_with_path(transformer, context)?,
                unlogged: unlogged.apply_transform_with_path(transformer, context)?,
                table: table.apply_transform_with_path(transformer, context)?,
                name: name.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SelectItem::UnnamedExpr(field0) => {
                    sqlparser::ast::SelectItem::UnnamedExpr(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                    sqlparser::ast::SelectItem::ExprWithAlias {
                        expr: expr.apply_transform_with_path(transformer, context)?,
                        alias: alias.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::SelectItem::QualifiedWildcard(field0, field1) => {
                    sqlparser::ast::SelectItem::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SelectItem::Wildcard(field0) => {
                    sqlparser::ast::SelectItem::Wildcard(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SequenceOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SequenceOptions::IncrementBy(field0, field1) => {
                    sqlparser::ast::SequenceOptions::IncrementBy(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SequenceOptions::MinValue(field0) => {
                    sqlparser::ast::SequenceOptions::MinValue(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SequenceOptions::MaxValue(field0) => {
                    sqlparser::ast::SequenceOptions::MaxValue(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SequenceOptions::StartWith(field0, field1) => {
                    sqlparser::ast::SequenceOptions::StartWith(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SequenceOptions::Cache(field0) => {
                    sqlparser::ast::SequenceOptions::Cache(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SequenceOptions::Cycle(field0) => {
                    sqlparser::ast::SequenceOptions::Cycle(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetConfigValue {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SetConfigValue::Default => {
                    transformer.transform(sqlparser::ast::SetConfigValue::Default, self, context)?
                }
                sqlparser::ast::SetConfigValue::FromCurrent => transformer.transform(
                    sqlparser::ast::SetConfigValue::FromCurrent,
                    self,
                    context,
                )?,
                sqlparser::ast::SetConfigValue::Value(field0) => {
                    sqlparser::ast::SetConfigValue::Value(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SetExpr::Select(field0) => sqlparser::ast::SetExpr::Select(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SetExpr::Query(field0) => sqlparser::ast::SetExpr::Query(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SetExpr::SetOperation {
                    op,
                    set_quantifier,
                    left,
                    right,
                } => sqlparser::ast::SetExpr::SetOperation {
                    op: op.apply_transform_with_path(transformer, context)?,
                    set_quantifier: set_quantifier
                        .apply_transform_with_path(transformer, context)?,
                    left: left.apply_transform_with_path(transformer, context)?,
                    right: right.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::SetExpr::Values(field0) => sqlparser::ast::SetExpr::Values(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SetExpr::Insert(field0) => sqlparser::ast::SetExpr::Insert(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SetExpr::Update(field0) => sqlparser::ast::SetExpr::Update(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SetExpr::Table(field0) => sqlparser::ast::SetExpr::Table(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SetOperator::Union => {
                    transformer.transform(sqlparser::ast::SetOperator::Union, self, context)?
                }
                sqlparser::ast::SetOperator::Except => {
                    transformer.transform(sqlparser::ast::SetOperator::Except, self, context)?
                }
                sqlparser::ast::SetOperator::Intersect => {
                    transformer.transform(sqlparser::ast::SetOperator::Intersect, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetQuantifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SetQuantifier::All => {
                    transformer.transform(sqlparser::ast::SetQuantifier::All, self, context)?
                }
                sqlparser::ast::SetQuantifier::Distinct => {
                    transformer.transform(sqlparser::ast::SetQuantifier::Distinct, self, context)?
                }
                sqlparser::ast::SetQuantifier::ByName => {
                    transformer.transform(sqlparser::ast::SetQuantifier::ByName, self, context)?
                }
                sqlparser::ast::SetQuantifier::AllByName => transformer.transform(
                    sqlparser::ast::SetQuantifier::AllByName,
                    self,
                    context,
                )?,
                sqlparser::ast::SetQuantifier::DistinctByName => transformer.transform(
                    sqlparser::ast::SetQuantifier::DistinctByName,
                    self,
                    context,
                )?,
                sqlparser::ast::SetQuantifier::None => {
                    transformer.transform(sqlparser::ast::SetQuantifier::None, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Setting {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ShowClause::IN => {
                    transformer.transform(sqlparser::ast::ShowClause::IN, self, context)?
                }
                sqlparser::ast::ShowClause::FROM => {
                    transformer.transform(sqlparser::ast::ShowClause::FROM, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowCreateObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ShowCreateObject::Event => {
                    transformer.transform(sqlparser::ast::ShowCreateObject::Event, self, context)?
                }
                sqlparser::ast::ShowCreateObject::Function => transformer.transform(
                    sqlparser::ast::ShowCreateObject::Function,
                    self,
                    context,
                )?,
                sqlparser::ast::ShowCreateObject::Procedure => transformer.transform(
                    sqlparser::ast::ShowCreateObject::Procedure,
                    self,
                    context,
                )?,
                sqlparser::ast::ShowCreateObject::Table => {
                    transformer.transform(sqlparser::ast::ShowCreateObject::Table, self, context)?
                }
                sqlparser::ast::ShowCreateObject::Trigger => transformer.transform(
                    sqlparser::ast::ShowCreateObject::Trigger,
                    self,
                    context,
                )?,
                sqlparser::ast::ShowCreateObject::View => {
                    transformer.transform(sqlparser::ast::ShowCreateObject::View, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowStatementFilter {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ShowStatementFilter::Like(field0) => {
                    sqlparser::ast::ShowStatementFilter::Like(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ShowStatementFilter::ILike(field0) => {
                    sqlparser::ast::ShowStatementFilter::ILike(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ShowStatementFilter::Where(field0) => {
                    sqlparser::ast::ShowStatementFilter::Where(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::ShowStatementFilter::NoKeyword(field0) => {
                    sqlparser::ast::ShowStatementFilter::NoKeyword(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SqlOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SqlOption::Clustered(field0) => {
                    sqlparser::ast::SqlOption::Clustered(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::SqlOption::Ident(field0) => sqlparser::ast::SqlOption::Ident(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::SqlOption::KeyValue { key, value } => {
                    sqlparser::ast::SqlOption::KeyValue {
                        key: key.apply_transform_with_path(transformer, context)?,
                        value: value.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::SqlOption::Partition {
                    column_name,
                    range_direction,
                    for_values,
                } => sqlparser::ast::SqlOption::Partition {
                    column_name: column_name.apply_transform_with_path(transformer, context)?,
                    range_direction: range_direction
                        .apply_transform_with_path(transformer, context)?,
                    for_values: for_values.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SqliteOnConflict {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::SqliteOnConflict::Rollback => transformer.transform(
                    sqlparser::ast::SqliteOnConflict::Rollback,
                    self,
                    context,
                )?,
                sqlparser::ast::SqliteOnConflict::Abort => {
                    transformer.transform(sqlparser::ast::SqliteOnConflict::Abort, self, context)?
                }
                sqlparser::ast::SqliteOnConflict::Fail => {
                    transformer.transform(sqlparser::ast::SqliteOnConflict::Fail, self, context)?
                }
                sqlparser::ast::SqliteOnConflict::Ignore => transformer.transform(
                    sqlparser::ast::SqliteOnConflict::Ignore,
                    self,
                    context,
                )?,
                sqlparser::ast::SqliteOnConflict::Replace => transformer.transform(
                    sqlparser::ast::SqliteOnConflict::Replace,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Statement {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Statement::Analyze {
                    table_name,
                    partitions,
                    for_columns,
                    columns,
                    cache_metadata,
                    noscan,
                    compute_statistics,
                } => sqlparser::ast::Statement::Analyze {
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    partitions: partitions.apply_transform_with_path(transformer, context)?,
                    for_columns: for_columns.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    cache_metadata: cache_metadata
                        .apply_transform_with_path(transformer, context)?,
                    noscan: noscan.apply_transform_with_path(transformer, context)?,
                    compute_statistics: compute_statistics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Truncate {
                    table_names,
                    partitions,
                    table,
                    only,
                    identity,
                    cascade,
                    on_cluster,
                } => sqlparser::ast::Statement::Truncate {
                    table_names: table_names.apply_transform_with_path(transformer, context)?,
                    partitions: partitions.apply_transform_with_path(transformer, context)?,
                    table: table.apply_transform_with_path(transformer, context)?,
                    only: only.apply_transform_with_path(transformer, context)?,
                    identity: identity.apply_transform_with_path(transformer, context)?,
                    cascade: cascade.apply_transform_with_path(transformer, context)?,
                    on_cluster: on_cluster.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Msck {
                    table_name,
                    repair,
                    partition_action,
                } => sqlparser::ast::Statement::Msck {
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    repair: repair.apply_transform_with_path(transformer, context)?,
                    partition_action: partition_action
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Query(field0) => sqlparser::ast::Statement::Query(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Statement::Insert(field0) => sqlparser::ast::Statement::Insert(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Statement::Install { extension_name } => {
                    sqlparser::ast::Statement::Install {
                        extension_name: extension_name
                            .apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::Load { extension_name } => {
                    sqlparser::ast::Statement::Load {
                        extension_name: extension_name
                            .apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::Directory {
                    overwrite,
                    local,
                    path,
                    file_format,
                    source,
                } => sqlparser::ast::Statement::Directory {
                    overwrite: overwrite.apply_transform_with_path(transformer, context)?,
                    local: local.apply_transform_with_path(transformer, context)?,
                    path: path.apply_transform_with_path(transformer, context)?,
                    file_format: file_format.apply_transform_with_path(transformer, context)?,
                    source: source.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Call(field0) => sqlparser::ast::Statement::Call(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Statement::Copy {
                    source,
                    to,
                    target,
                    options,
                    legacy_options,
                    values,
                } => sqlparser::ast::Statement::Copy {
                    source: source.apply_transform_with_path(transformer, context)?,
                    to: to.apply_transform_with_path(transformer, context)?,
                    target: target.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                    legacy_options: legacy_options
                        .apply_transform_with_path(transformer, context)?,
                    values: values.apply_transform_with_path(transformer, context)?,
                },
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
                } => sqlparser::ast::Statement::CopyIntoSnowflake {
                    into: into.apply_transform_with_path(transformer, context)?,
                    from_stage: from_stage.apply_transform_with_path(transformer, context)?,
                    from_stage_alias: from_stage_alias
                        .apply_transform_with_path(transformer, context)?,
                    stage_params: stage_params.apply_transform_with_path(transformer, context)?,
                    from_transformations: from_transformations
                        .apply_transform_with_path(transformer, context)?,
                    files: files.apply_transform_with_path(transformer, context)?,
                    pattern: pattern.apply_transform_with_path(transformer, context)?,
                    file_format: file_format.apply_transform_with_path(transformer, context)?,
                    copy_options: copy_options.apply_transform_with_path(transformer, context)?,
                    validation_mode: validation_mode
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Close { cursor } => sqlparser::ast::Statement::Close {
                    cursor: cursor.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Update {
                    table,
                    assignments,
                    from,
                    selection,
                    returning,
                } => sqlparser::ast::Statement::Update {
                    table: table.apply_transform_with_path(transformer, context)?,
                    assignments: assignments.apply_transform_with_path(transformer, context)?,
                    from: from.apply_transform_with_path(transformer, context)?,
                    selection: selection.apply_transform_with_path(transformer, context)?,
                    returning: returning.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Delete(field0) => sqlparser::ast::Statement::Delete(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
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
                } => sqlparser::ast::Statement::CreateView {
                    or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                    materialized: materialized.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    query: query.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                    cluster_by: cluster_by.apply_transform_with_path(transformer, context)?,
                    comment: comment.apply_transform_with_path(transformer, context)?,
                    with_no_schema_binding: with_no_schema_binding
                        .apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    to: to.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateTable(field0) => {
                    sqlparser::ast::Statement::CreateTable(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Statement::CreateVirtualTable {
                    name,
                    if_not_exists,
                    module_name,
                    module_args,
                } => sqlparser::ast::Statement::CreateVirtualTable {
                    name: name.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    module_name: module_name.apply_transform_with_path(transformer, context)?,
                    module_args: module_args.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateIndex(field0) => {
                    sqlparser::ast::Statement::CreateIndex(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
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
                } => sqlparser::ast::Statement::CreateRole {
                    names: names.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    login: login.apply_transform_with_path(transformer, context)?,
                    inherit: inherit.apply_transform_with_path(transformer, context)?,
                    bypassrls: bypassrls.apply_transform_with_path(transformer, context)?,
                    password: password.apply_transform_with_path(transformer, context)?,
                    superuser: superuser.apply_transform_with_path(transformer, context)?,
                    create_db: create_db.apply_transform_with_path(transformer, context)?,
                    create_role: create_role.apply_transform_with_path(transformer, context)?,
                    replication: replication.apply_transform_with_path(transformer, context)?,
                    connection_limit: connection_limit
                        .apply_transform_with_path(transformer, context)?,
                    valid_until: valid_until.apply_transform_with_path(transformer, context)?,
                    in_role: in_role.apply_transform_with_path(transformer, context)?,
                    in_group: in_group.apply_transform_with_path(transformer, context)?,
                    role: role.apply_transform_with_path(transformer, context)?,
                    user: user.apply_transform_with_path(transformer, context)?,
                    admin: admin.apply_transform_with_path(transformer, context)?,
                    authorization_owner: authorization_owner
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateSecret {
                    or_replace,
                    temporary,
                    if_not_exists,
                    name,
                    storage_specifier,
                    secret_type,
                    options,
                } => sqlparser::ast::Statement::CreateSecret {
                    or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    storage_specifier: storage_specifier
                        .apply_transform_with_path(transformer, context)?,
                    secret_type: secret_type.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreatePolicy {
                    name,
                    table_name,
                    policy_type,
                    command,
                    to,
                    using,
                    with_check,
                } => sqlparser::ast::Statement::CreatePolicy {
                    name: name.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    policy_type: policy_type.apply_transform_with_path(transformer, context)?,
                    command: command.apply_transform_with_path(transformer, context)?,
                    to: to.apply_transform_with_path(transformer, context)?,
                    using: using.apply_transform_with_path(transformer, context)?,
                    with_check: with_check.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::AlterTable {
                    name,
                    if_exists,
                    only,
                    operations,
                    location,
                    on_cluster,
                } => sqlparser::ast::Statement::AlterTable {
                    name: name.apply_transform_with_path(transformer, context)?,
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    only: only.apply_transform_with_path(transformer, context)?,
                    operations: operations.apply_transform_with_path(transformer, context)?,
                    location: location.apply_transform_with_path(transformer, context)?,
                    on_cluster: on_cluster.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::AlterIndex { name, operation } => {
                    sqlparser::ast::Statement::AlterIndex {
                        name: name.apply_transform_with_path(transformer, context)?,
                        operation: operation.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::AlterView {
                    name,
                    columns,
                    query,
                    with_options,
                } => sqlparser::ast::Statement::AlterView {
                    name: name.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    query: query.apply_transform_with_path(transformer, context)?,
                    with_options: with_options.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::AlterRole { name, operation } => {
                    sqlparser::ast::Statement::AlterRole {
                        name: name.apply_transform_with_path(transformer, context)?,
                        operation: operation.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::AlterPolicy {
                    name,
                    table_name,
                    operation,
                } => sqlparser::ast::Statement::AlterPolicy {
                    name: name.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    operation: operation.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::AttachDatabase {
                    schema_name,
                    database_file_name,
                    database,
                } => sqlparser::ast::Statement::AttachDatabase {
                    schema_name: schema_name.apply_transform_with_path(transformer, context)?,
                    database_file_name: database_file_name
                        .apply_transform_with_path(transformer, context)?,
                    database: database.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::AttachDuckDBDatabase {
                    if_not_exists,
                    database,
                    database_path,
                    database_alias,
                    attach_options,
                } => sqlparser::ast::Statement::AttachDuckDBDatabase {
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    database: database.apply_transform_with_path(transformer, context)?,
                    database_path: database_path.apply_transform_with_path(transformer, context)?,
                    database_alias: database_alias
                        .apply_transform_with_path(transformer, context)?,
                    attach_options: attach_options
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::DetachDuckDBDatabase {
                    if_exists,
                    database,
                    database_alias,
                } => sqlparser::ast::Statement::DetachDuckDBDatabase {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    database: database.apply_transform_with_path(transformer, context)?,
                    database_alias: database_alias
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Drop {
                    object_type,
                    if_exists,
                    names,
                    cascade,
                    restrict,
                    purge,
                    temporary,
                } => sqlparser::ast::Statement::Drop {
                    object_type: object_type.apply_transform_with_path(transformer, context)?,
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    names: names.apply_transform_with_path(transformer, context)?,
                    cascade: cascade.apply_transform_with_path(transformer, context)?,
                    restrict: restrict.apply_transform_with_path(transformer, context)?,
                    purge: purge.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::DropFunction {
                    if_exists,
                    func_desc,
                    option,
                } => sqlparser::ast::Statement::DropFunction {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    func_desc: func_desc.apply_transform_with_path(transformer, context)?,
                    option: option.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::DropProcedure {
                    if_exists,
                    proc_desc,
                    option,
                } => sqlparser::ast::Statement::DropProcedure {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    proc_desc: proc_desc.apply_transform_with_path(transformer, context)?,
                    option: option.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::DropSecret {
                    if_exists,
                    temporary,
                    name,
                    storage_specifier,
                } => sqlparser::ast::Statement::DropSecret {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    storage_specifier: storage_specifier
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::DropPolicy {
                    if_exists,
                    name,
                    table_name,
                    option,
                } => sqlparser::ast::Statement::DropPolicy {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    option: option.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Declare { stmts } => {
                    sqlparser::ast::Statement::Declare {
                        stmts: stmts.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::CreateExtension {
                    name,
                    if_not_exists,
                    cascade,
                    schema,
                    version,
                } => sqlparser::ast::Statement::CreateExtension {
                    name: name.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    cascade: cascade.apply_transform_with_path(transformer, context)?,
                    schema: schema.apply_transform_with_path(transformer, context)?,
                    version: version.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Fetch {
                    name,
                    direction,
                    into,
                } => sqlparser::ast::Statement::Fetch {
                    name: name.apply_transform_with_path(transformer, context)?,
                    direction: direction.apply_transform_with_path(transformer, context)?,
                    into: into.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Flush {
                    object_type,
                    location,
                    channel,
                    read_lock,
                    export,
                    tables,
                } => sqlparser::ast::Statement::Flush {
                    object_type: object_type.apply_transform_with_path(transformer, context)?,
                    location: location.apply_transform_with_path(transformer, context)?,
                    channel: channel.apply_transform_with_path(transformer, context)?,
                    read_lock: read_lock.apply_transform_with_path(transformer, context)?,
                    export: export.apply_transform_with_path(transformer, context)?,
                    tables: tables.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Discard { object_type } => {
                    sqlparser::ast::Statement::Discard {
                        object_type: object_type.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::SetRole {
                    context_modifier,
                    role_name,
                } => sqlparser::ast::Statement::SetRole {
                    context_modifier: context_modifier
                        .apply_transform_with_path(transformer, context)?,
                    role_name: role_name.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::SetVariable {
                    local,
                    hivevar,
                    variables,
                    value,
                } => sqlparser::ast::Statement::SetVariable {
                    local: local.apply_transform_with_path(transformer, context)?,
                    hivevar: hivevar.apply_transform_with_path(transformer, context)?,
                    variables: variables.apply_transform_with_path(transformer, context)?,
                    value: value.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::SetTimeZone { local, value } => {
                    sqlparser::ast::Statement::SetTimeZone {
                        local: local.apply_transform_with_path(transformer, context)?,
                        value: value.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::SetNames {
                    charset_name,
                    collation_name,
                } => sqlparser::ast::Statement::SetNames {
                    charset_name: charset_name.apply_transform_with_path(transformer, context)?,
                    collation_name: collation_name
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::SetNamesDefault {} => {
                    sqlparser::ast::Statement::SetNamesDefault {}
                }
                sqlparser::ast::Statement::ShowFunctions { filter } => {
                    sqlparser::ast::Statement::ShowFunctions {
                        filter: filter.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ShowVariable { variable } => {
                    sqlparser::ast::Statement::ShowVariable {
                        variable: variable.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ShowStatus {
                    filter,
                    global,
                    session,
                } => sqlparser::ast::Statement::ShowStatus {
                    filter: filter.apply_transform_with_path(transformer, context)?,
                    global: global.apply_transform_with_path(transformer, context)?,
                    session: session.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::ShowVariables {
                    filter,
                    global,
                    session,
                } => sqlparser::ast::Statement::ShowVariables {
                    filter: filter.apply_transform_with_path(transformer, context)?,
                    global: global.apply_transform_with_path(transformer, context)?,
                    session: session.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::ShowCreate { obj_type, obj_name } => {
                    sqlparser::ast::Statement::ShowCreate {
                        obj_type: obj_type.apply_transform_with_path(transformer, context)?,
                        obj_name: obj_name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ShowColumns {
                    extended,
                    full,
                    table_name,
                    filter,
                } => sqlparser::ast::Statement::ShowColumns {
                    extended: extended.apply_transform_with_path(transformer, context)?,
                    full: full.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    filter: filter.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::ShowDatabases { filter } => {
                    sqlparser::ast::Statement::ShowDatabases {
                        filter: filter.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ShowSchemas { filter } => {
                    sqlparser::ast::Statement::ShowSchemas {
                        filter: filter.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ShowTables {
                    extended,
                    full,
                    clause,
                    db_name,
                    filter,
                } => sqlparser::ast::Statement::ShowTables {
                    extended: extended.apply_transform_with_path(transformer, context)?,
                    full: full.apply_transform_with_path(transformer, context)?,
                    clause: clause.apply_transform_with_path(transformer, context)?,
                    db_name: db_name.apply_transform_with_path(transformer, context)?,
                    filter: filter.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::ShowViews {
                    materialized,
                    clause,
                    db_name,
                    filter,
                } => sqlparser::ast::Statement::ShowViews {
                    materialized: materialized.apply_transform_with_path(transformer, context)?,
                    clause: clause.apply_transform_with_path(transformer, context)?,
                    db_name: db_name.apply_transform_with_path(transformer, context)?,
                    filter: filter.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::ShowCollation { filter } => {
                    sqlparser::ast::Statement::ShowCollation {
                        filter: filter.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::Use(field0) => sqlparser::ast::Statement::Use(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Statement::StartTransaction {
                    modes,
                    begin,
                    modifier,
                } => sqlparser::ast::Statement::StartTransaction {
                    modes: modes.apply_transform_with_path(transformer, context)?,
                    begin: begin.apply_transform_with_path(transformer, context)?,
                    modifier: modifier.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::SetTransaction {
                    modes,
                    snapshot,
                    session,
                } => sqlparser::ast::Statement::SetTransaction {
                    modes: modes.apply_transform_with_path(transformer, context)?,
                    snapshot: snapshot.apply_transform_with_path(transformer, context)?,
                    session: session.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Comment {
                    object_type,
                    object_name,
                    comment,
                    if_exists,
                } => sqlparser::ast::Statement::Comment {
                    object_type: object_type.apply_transform_with_path(transformer, context)?,
                    object_name: object_name.apply_transform_with_path(transformer, context)?,
                    comment: comment.apply_transform_with_path(transformer, context)?,
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Commit { chain } => sqlparser::ast::Statement::Commit {
                    chain: chain.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Rollback { chain, savepoint } => {
                    sqlparser::ast::Statement::Rollback {
                        chain: chain.apply_transform_with_path(transformer, context)?,
                        savepoint: savepoint.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::CreateSchema {
                    schema_name,
                    if_not_exists,
                } => sqlparser::ast::Statement::CreateSchema {
                    schema_name: schema_name.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateDatabase {
                    db_name,
                    if_not_exists,
                    location,
                    managed_location,
                } => sqlparser::ast::Statement::CreateDatabase {
                    db_name: db_name.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    location: location.apply_transform_with_path(transformer, context)?,
                    managed_location: managed_location
                        .apply_transform_with_path(transformer, context)?,
                },
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
                } => sqlparser::ast::Statement::CreateFunction {
                    or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    args: args.apply_transform_with_path(transformer, context)?,
                    return_type: return_type.apply_transform_with_path(transformer, context)?,
                    function_body: function_body.apply_transform_with_path(transformer, context)?,
                    behavior: behavior.apply_transform_with_path(transformer, context)?,
                    called_on_null: called_on_null
                        .apply_transform_with_path(transformer, context)?,
                    parallel: parallel.apply_transform_with_path(transformer, context)?,
                    using: using.apply_transform_with_path(transformer, context)?,
                    language: language.apply_transform_with_path(transformer, context)?,
                    determinism_specifier: determinism_specifier
                        .apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                    remote_connection: remote_connection
                        .apply_transform_with_path(transformer, context)?,
                },
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
                } => sqlparser::ast::Statement::CreateTrigger {
                    or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                    is_constraint: is_constraint.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    period: period.apply_transform_with_path(transformer, context)?,
                    events: events.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    referenced_table_name: referenced_table_name
                        .apply_transform_with_path(transformer, context)?,
                    referencing: referencing.apply_transform_with_path(transformer, context)?,
                    trigger_object: trigger_object
                        .apply_transform_with_path(transformer, context)?,
                    include_each: include_each.apply_transform_with_path(transformer, context)?,
                    condition: condition.apply_transform_with_path(transformer, context)?,
                    exec_body: exec_body.apply_transform_with_path(transformer, context)?,
                    characteristics: characteristics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::DropTrigger {
                    if_exists,
                    trigger_name,
                    table_name,
                    option,
                } => sqlparser::ast::Statement::DropTrigger {
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                    trigger_name: trigger_name.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    option: option.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateProcedure {
                    or_alter,
                    name,
                    params,
                    body,
                } => sqlparser::ast::Statement::CreateProcedure {
                    or_alter: or_alter.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    params: params.apply_transform_with_path(transformer, context)?,
                    body: body.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateMacro {
                    or_replace,
                    temporary,
                    name,
                    args,
                    definition,
                } => sqlparser::ast::Statement::CreateMacro {
                    or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    args: args.apply_transform_with_path(transformer, context)?,
                    definition: definition.apply_transform_with_path(transformer, context)?,
                },
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
                } => sqlparser::ast::Statement::CreateStage {
                    or_replace: or_replace.apply_transform_with_path(transformer, context)?,
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    stage_params: stage_params.apply_transform_with_path(transformer, context)?,
                    directory_table_params: directory_table_params
                        .apply_transform_with_path(transformer, context)?,
                    file_format: file_format.apply_transform_with_path(transformer, context)?,
                    copy_options: copy_options.apply_transform_with_path(transformer, context)?,
                    comment: comment.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Assert { condition, message } => {
                    sqlparser::ast::Statement::Assert {
                        condition: condition.apply_transform_with_path(transformer, context)?,
                        message: message.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::Grant {
                    privileges,
                    objects,
                    grantees,
                    with_grant_option,
                    granted_by,
                } => sqlparser::ast::Statement::Grant {
                    privileges: privileges.apply_transform_with_path(transformer, context)?,
                    objects: objects.apply_transform_with_path(transformer, context)?,
                    grantees: grantees.apply_transform_with_path(transformer, context)?,
                    with_grant_option: with_grant_option
                        .apply_transform_with_path(transformer, context)?,
                    granted_by: granted_by.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Revoke {
                    privileges,
                    objects,
                    grantees,
                    granted_by,
                    cascade,
                } => sqlparser::ast::Statement::Revoke {
                    privileges: privileges.apply_transform_with_path(transformer, context)?,
                    objects: objects.apply_transform_with_path(transformer, context)?,
                    grantees: grantees.apply_transform_with_path(transformer, context)?,
                    granted_by: granted_by.apply_transform_with_path(transformer, context)?,
                    cascade: cascade.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Deallocate { name, prepare } => {
                    sqlparser::ast::Statement::Deallocate {
                        name: name.apply_transform_with_path(transformer, context)?,
                        prepare: prepare.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::Execute {
                    name,
                    parameters,
                    has_parentheses,
                    using,
                } => sqlparser::ast::Statement::Execute {
                    name: name.apply_transform_with_path(transformer, context)?,
                    parameters: parameters.apply_transform_with_path(transformer, context)?,
                    has_parentheses: has_parentheses
                        .apply_transform_with_path(transformer, context)?,
                    using: using.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Prepare {
                    name,
                    data_types,
                    statement,
                } => sqlparser::ast::Statement::Prepare {
                    name: name.apply_transform_with_path(transformer, context)?,
                    data_types: data_types.apply_transform_with_path(transformer, context)?,
                    statement: statement.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Kill { modifier, id } => {
                    sqlparser::ast::Statement::Kill {
                        modifier: modifier.apply_transform_with_path(transformer, context)?,
                        id: id.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ExplainTable {
                    describe_alias,
                    hive_format,
                    has_table_keyword,
                    table_name,
                } => sqlparser::ast::Statement::ExplainTable {
                    describe_alias: describe_alias
                        .apply_transform_with_path(transformer, context)?,
                    hive_format: hive_format.apply_transform_with_path(transformer, context)?,
                    has_table_keyword: has_table_keyword
                        .apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Explain {
                    describe_alias,
                    analyze,
                    verbose,
                    query_plan,
                    statement,
                    format,
                    options,
                } => sqlparser::ast::Statement::Explain {
                    describe_alias: describe_alias
                        .apply_transform_with_path(transformer, context)?,
                    analyze: analyze.apply_transform_with_path(transformer, context)?,
                    verbose: verbose.apply_transform_with_path(transformer, context)?,
                    query_plan: query_plan.apply_transform_with_path(transformer, context)?,
                    statement: statement.apply_transform_with_path(transformer, context)?,
                    format: format.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Savepoint { name } => {
                    sqlparser::ast::Statement::Savepoint {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::ReleaseSavepoint { name } => {
                    sqlparser::ast::Statement::ReleaseSavepoint {
                        name: name.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::Merge {
                    into,
                    table,
                    source,
                    on,
                    clauses,
                } => sqlparser::ast::Statement::Merge {
                    into: into.apply_transform_with_path(transformer, context)?,
                    table: table.apply_transform_with_path(transformer, context)?,
                    source: source.apply_transform_with_path(transformer, context)?,
                    on: on.apply_transform_with_path(transformer, context)?,
                    clauses: clauses.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Cache {
                    table_flag,
                    table_name,
                    has_as,
                    options,
                    query,
                } => sqlparser::ast::Statement::Cache {
                    table_flag: table_flag.apply_transform_with_path(transformer, context)?,
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    has_as: has_as.apply_transform_with_path(transformer, context)?,
                    options: options.apply_transform_with_path(transformer, context)?,
                    query: query.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::UNCache {
                    table_name,
                    if_exists,
                } => sqlparser::ast::Statement::UNCache {
                    table_name: table_name.apply_transform_with_path(transformer, context)?,
                    if_exists: if_exists.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateSequence {
                    temporary,
                    if_not_exists,
                    name,
                    data_type,
                    sequence_options,
                    owned_by,
                } => sqlparser::ast::Statement::CreateSequence {
                    temporary: temporary.apply_transform_with_path(transformer, context)?,
                    if_not_exists: if_not_exists.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    data_type: data_type.apply_transform_with_path(transformer, context)?,
                    sequence_options: sequence_options
                        .apply_transform_with_path(transformer, context)?,
                    owned_by: owned_by.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::CreateType {
                    name,
                    representation,
                } => sqlparser::ast::Statement::CreateType {
                    name: name.apply_transform_with_path(transformer, context)?,
                    representation: representation
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::Pragma { name, value, is_eq } => {
                    sqlparser::ast::Statement::Pragma {
                        name: name.apply_transform_with_path(transformer, context)?,
                        value: value.apply_transform_with_path(transformer, context)?,
                        is_eq: is_eq.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::LockTables { tables } => {
                    sqlparser::ast::Statement::LockTables {
                        tables: tables.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::UnlockTables => {
                    transformer.transform(sqlparser::ast::Statement::UnlockTables, self, context)?
                }
                sqlparser::ast::Statement::Unload { query, to, with } => {
                    sqlparser::ast::Statement::Unload {
                        query: query.apply_transform_with_path(transformer, context)?,
                        to: to.apply_transform_with_path(transformer, context)?,
                        with: with.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::OptimizeTable {
                    name,
                    on_cluster,
                    partition,
                    include_final,
                    deduplicate,
                } => sqlparser::ast::Statement::OptimizeTable {
                    name: name.apply_transform_with_path(transformer, context)?,
                    on_cluster: on_cluster.apply_transform_with_path(transformer, context)?,
                    partition: partition.apply_transform_with_path(transformer, context)?,
                    include_final: include_final.apply_transform_with_path(transformer, context)?,
                    deduplicate: deduplicate.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Statement::LISTEN { channel } => {
                    sqlparser::ast::Statement::LISTEN {
                        channel: channel.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::Statement::NOTIFY { channel, payload } => {
                    sqlparser::ast::Statement::NOTIFY {
                        channel: channel.apply_transform_with_path(transformer, context)?,
                        payload: payload.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::StructBracketKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::StructBracketKind::Parentheses => transformer.transform(
                    sqlparser::ast::StructBracketKind::Parentheses,
                    self,
                    context,
                )?,
                sqlparser::ast::StructBracketKind::AngleBrackets => transformer.transform(
                    sqlparser::ast::StructBracketKind::AngleBrackets,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::StructField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                field_name,
                field_type,
            } = self;
            Self {
                field_name: field_name.apply_transform_with_path(transformer, context)?,
                field_type: field_type.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Subscript {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Subscript::Index { index } => sqlparser::ast::Subscript::Index {
                    index: index.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::Subscript::Slice {
                    lower_bound,
                    upper_bound,
                    stride,
                } => sqlparser::ast::Subscript::Slice {
                    lower_bound: lower_bound.apply_transform_with_path(transformer, context)?,
                    upper_bound: upper_bound.apply_transform_with_path(transformer, context)?,
                    stride: stride.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SymbolDefinition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { symbol, definition } = self;
            Self {
                symbol: symbol.apply_transform_with_path(transformer, context)?,
                definition: definition.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Table {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                table_name,
                schema_name,
            } = self;
            Self {
                table_name: table_name.apply_transform_with_path(transformer, context)?,
                schema_name: schema_name.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, columns } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                columns: columns.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableConstraint {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TableConstraint::Unique {
                    name,
                    index_name,
                    index_type_display,
                    index_type,
                    columns,
                    index_options,
                    characteristics,
                } => sqlparser::ast::TableConstraint::Unique {
                    name: name.apply_transform_with_path(transformer, context)?,
                    index_name: index_name.apply_transform_with_path(transformer, context)?,
                    index_type_display: index_type_display
                        .apply_transform_with_path(transformer, context)?,
                    index_type: index_type.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    index_options: index_options.apply_transform_with_path(transformer, context)?,
                    characteristics: characteristics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableConstraint::PrimaryKey {
                    name,
                    index_name,
                    index_type,
                    columns,
                    index_options,
                    characteristics,
                } => sqlparser::ast::TableConstraint::PrimaryKey {
                    name: name.apply_transform_with_path(transformer, context)?,
                    index_name: index_name.apply_transform_with_path(transformer, context)?,
                    index_type: index_type.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    index_options: index_options.apply_transform_with_path(transformer, context)?,
                    characteristics: characteristics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableConstraint::ForeignKey {
                    name,
                    columns,
                    foreign_table,
                    referred_columns,
                    on_delete,
                    on_update,
                    characteristics,
                } => sqlparser::ast::TableConstraint::ForeignKey {
                    name: name.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    foreign_table: foreign_table.apply_transform_with_path(transformer, context)?,
                    referred_columns: referred_columns
                        .apply_transform_with_path(transformer, context)?,
                    on_delete: on_delete.apply_transform_with_path(transformer, context)?,
                    on_update: on_update.apply_transform_with_path(transformer, context)?,
                    characteristics: characteristics
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableConstraint::Check { name, expr } => {
                    sqlparser::ast::TableConstraint::Check {
                        name: name.apply_transform_with_path(transformer, context)?,
                        expr: expr.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::TableConstraint::Index {
                    display_as_key,
                    name,
                    index_type,
                    columns,
                } => sqlparser::ast::TableConstraint::Index {
                    display_as_key: display_as_key
                        .apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    index_type: index_type.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableConstraint::FulltextOrSpatial {
                    fulltext,
                    index_type_display,
                    opt_index_name,
                    columns,
                } => sqlparser::ast::TableConstraint::FulltextOrSpatial {
                    fulltext: fulltext.apply_transform_with_path(transformer, context)?,
                    index_type_display: index_type_display
                        .apply_transform_with_path(transformer, context)?,
                    opt_index_name: opt_index_name
                        .apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableEngine {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, parameters } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                parameters: parameters.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableFactor {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TableFactor::Table {
                    name,
                    alias,
                    args,
                    with_hints,
                    version,
                    with_ordinality,
                    partitions,
                } => sqlparser::ast::TableFactor::Table {
                    name: name.apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                    args: args.apply_transform_with_path(transformer, context)?,
                    with_hints: with_hints.apply_transform_with_path(transformer, context)?,
                    version: version.apply_transform_with_path(transformer, context)?,
                    with_ordinality: with_ordinality
                        .apply_transform_with_path(transformer, context)?,
                    partitions: partitions.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::Derived {
                    lateral,
                    subquery,
                    alias,
                } => sqlparser::ast::TableFactor::Derived {
                    lateral: lateral.apply_transform_with_path(transformer, context)?,
                    subquery: subquery.apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::TableFunction { expr, alias } => {
                    sqlparser::ast::TableFactor::TableFunction {
                        expr: expr.apply_transform_with_path(transformer, context)?,
                        alias: alias.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::TableFactor::Function {
                    lateral,
                    name,
                    args,
                    alias,
                } => sqlparser::ast::TableFactor::Function {
                    lateral: lateral.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    args: args.apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::UNNEST {
                    alias,
                    array_exprs,
                    with_offset,
                    with_offset_alias,
                    with_ordinality,
                } => sqlparser::ast::TableFactor::UNNEST {
                    alias: alias.apply_transform_with_path(transformer, context)?,
                    array_exprs: array_exprs.apply_transform_with_path(transformer, context)?,
                    with_offset: with_offset.apply_transform_with_path(transformer, context)?,
                    with_offset_alias: with_offset_alias
                        .apply_transform_with_path(transformer, context)?,
                    with_ordinality: with_ordinality
                        .apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::JsonTable {
                    json_expr,
                    json_path,
                    columns,
                    alias,
                } => sqlparser::ast::TableFactor::JsonTable {
                    json_expr: json_expr.apply_transform_with_path(transformer, context)?,
                    json_path: json_path.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::NestedJoin {
                    table_with_joins,
                    alias,
                } => sqlparser::ast::TableFactor::NestedJoin {
                    table_with_joins: table_with_joins
                        .apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::Pivot {
                    table,
                    aggregate_functions,
                    value_column,
                    value_source,
                    default_on_null,
                    alias,
                } => sqlparser::ast::TableFactor::Pivot {
                    table: table.apply_transform_with_path(transformer, context)?,
                    aggregate_functions: aggregate_functions
                        .apply_transform_with_path(transformer, context)?,
                    value_column: value_column.apply_transform_with_path(transformer, context)?,
                    value_source: value_source.apply_transform_with_path(transformer, context)?,
                    default_on_null: default_on_null
                        .apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
                sqlparser::ast::TableFactor::Unpivot {
                    table,
                    value,
                    name,
                    columns,
                    alias,
                } => sqlparser::ast::TableFactor::Unpivot {
                    table: table.apply_transform_with_path(transformer, context)?,
                    value: value.apply_transform_with_path(transformer, context)?,
                    name: name.apply_transform_with_path(transformer, context)?,
                    columns: columns.apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
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
                } => sqlparser::ast::TableFactor::MatchRecognize {
                    table: table.apply_transform_with_path(transformer, context)?,
                    partition_by: partition_by.apply_transform_with_path(transformer, context)?,
                    order_by: order_by.apply_transform_with_path(transformer, context)?,
                    measures: measures.apply_transform_with_path(transformer, context)?,
                    rows_per_match: rows_per_match
                        .apply_transform_with_path(transformer, context)?,
                    after_match_skip: after_match_skip
                        .apply_transform_with_path(transformer, context)?,
                    pattern: pattern.apply_transform_with_path(transformer, context)?,
                    symbols: symbols.apply_transform_with_path(transformer, context)?,
                    alias: alias.apply_transform_with_path(transformer, context)?,
                },
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableFunctionArgs {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { args, settings } = self;
            Self {
                args: args.apply_transform_with_path(transformer, context)?,
                settings: settings.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableOptionsClustered {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TableOptionsClustered::ColumnstoreIndex => transformer.transform(
                    sqlparser::ast::TableOptionsClustered::ColumnstoreIndex,
                    self,
                    context,
                )?,
                sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(field0) => {
                    sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::TableOptionsClustered::Index(field0) => {
                    sqlparser::ast::TableOptionsClustered::Index(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableVersion {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TableVersion::ForSystemTimeAsOf(field0) => {
                    sqlparser::ast::TableVersion::ForSystemTimeAsOf(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableWithJoins {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { relation, joins } = self;
            Self {
                relation: relation.apply_transform_with_path(transformer, context)?,
                joins: joins.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Tag {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TagsColumnOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { with, tags } = self;
            Self {
                with: with.apply_transform_with_path(transformer, context)?,
                tags: tags.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TimezoneInfo {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TimezoneInfo::None => {
                    transformer.transform(sqlparser::ast::TimezoneInfo::None, self, context)?
                }
                sqlparser::ast::TimezoneInfo::WithTimeZone => transformer.transform(
                    sqlparser::ast::TimezoneInfo::WithTimeZone,
                    self,
                    context,
                )?,
                sqlparser::ast::TimezoneInfo::WithoutTimeZone => transformer.transform(
                    sqlparser::ast::TimezoneInfo::WithoutTimeZone,
                    self,
                    context,
                )?,
                sqlparser::ast::TimezoneInfo::Tz => {
                    transformer.transform(sqlparser::ast::TimezoneInfo::Tz, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Top {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                with_ties,
                percent,
                quantity,
            } = self;
            Self {
                with_ties: with_ties.apply_transform_with_path(transformer, context)?,
                percent: percent.apply_transform_with_path(transformer, context)?,
                quantity: quantity.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TopQuantity {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TopQuantity::Expr(field0) => sqlparser::ast::TopQuantity::Expr(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::TopQuantity::Constant(field0) => {
                    sqlparser::ast::TopQuantity::Constant(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionAccessMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TransactionAccessMode::ReadOnly => transformer.transform(
                    sqlparser::ast::TransactionAccessMode::ReadOnly,
                    self,
                    context,
                )?,
                sqlparser::ast::TransactionAccessMode::ReadWrite => transformer.transform(
                    sqlparser::ast::TransactionAccessMode::ReadWrite,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionIsolationLevel {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TransactionIsolationLevel::ReadUncommitted => transformer
                    .transform(
                        sqlparser::ast::TransactionIsolationLevel::ReadUncommitted,
                        self,
                        context,
                    )?,
                sqlparser::ast::TransactionIsolationLevel::ReadCommitted => transformer.transform(
                    sqlparser::ast::TransactionIsolationLevel::ReadCommitted,
                    self,
                    context,
                )?,
                sqlparser::ast::TransactionIsolationLevel::RepeatableRead => transformer
                    .transform(
                        sqlparser::ast::TransactionIsolationLevel::RepeatableRead,
                        self,
                        context,
                    )?,
                sqlparser::ast::TransactionIsolationLevel::Serializable => transformer.transform(
                    sqlparser::ast::TransactionIsolationLevel::Serializable,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TransactionMode::AccessMode(field0) => {
                    sqlparser::ast::TransactionMode::AccessMode(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::TransactionMode::IsolationLevel(field0) => {
                    sqlparser::ast::TransactionMode::IsolationLevel(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TransactionModifier::Deferred => transformer.transform(
                    sqlparser::ast::TransactionModifier::Deferred,
                    self,
                    context,
                )?,
                sqlparser::ast::TransactionModifier::Immediate => transformer.transform(
                    sqlparser::ast::TransactionModifier::Immediate,
                    self,
                    context,
                )?,
                sqlparser::ast::TransactionModifier::Exclusive => transformer.transform(
                    sqlparser::ast::TransactionModifier::Exclusive,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerEvent {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TriggerEvent::Insert => {
                    transformer.transform(sqlparser::ast::TriggerEvent::Insert, self, context)?
                }
                sqlparser::ast::TriggerEvent::Update(field0) => {
                    sqlparser::ast::TriggerEvent::Update(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::TriggerEvent::Delete => {
                    transformer.transform(sqlparser::ast::TriggerEvent::Delete, self, context)?
                }
                sqlparser::ast::TriggerEvent::Truncate => {
                    transformer.transform(sqlparser::ast::TriggerEvent::Truncate, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerExecBody {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                exec_type,
                func_desc,
            } = self;
            Self {
                exec_type: exec_type.apply_transform_with_path(transformer, context)?,
                func_desc: func_desc.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerExecBodyType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TriggerExecBodyType::Function => transformer.transform(
                    sqlparser::ast::TriggerExecBodyType::Function,
                    self,
                    context,
                )?,
                sqlparser::ast::TriggerExecBodyType::Procedure => transformer.transform(
                    sqlparser::ast::TriggerExecBodyType::Procedure,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TriggerObject::Row => {
                    transformer.transform(sqlparser::ast::TriggerObject::Row, self, context)?
                }
                sqlparser::ast::TriggerObject::Statement => transformer.transform(
                    sqlparser::ast::TriggerObject::Statement,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerPeriod {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TriggerPeriod::After => {
                    transformer.transform(sqlparser::ast::TriggerPeriod::After, self, context)?
                }
                sqlparser::ast::TriggerPeriod::Before => {
                    transformer.transform(sqlparser::ast::TriggerPeriod::Before, self, context)?
                }
                sqlparser::ast::TriggerPeriod::InsteadOf => transformer.transform(
                    sqlparser::ast::TriggerPeriod::InsteadOf,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerReferencing {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                refer_type,
                is_as,
                transition_relation_name,
            } = self;
            Self {
                refer_type: refer_type.apply_transform_with_path(transformer, context)?,
                is_as: is_as.apply_transform_with_path(transformer, context)?,
                transition_relation_name: transition_relation_name
                    .apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerReferencingType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TriggerReferencingType::OldTable => transformer.transform(
                    sqlparser::ast::TriggerReferencingType::OldTable,
                    self,
                    context,
                )?,
                sqlparser::ast::TriggerReferencingType::NewTable => transformer.transform(
                    sqlparser::ast::TriggerReferencingType::NewTable,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TrimWhereField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TrimWhereField::Both => {
                    transformer.transform(sqlparser::ast::TrimWhereField::Both, self, context)?
                }
                sqlparser::ast::TrimWhereField::Leading => {
                    transformer.transform(sqlparser::ast::TrimWhereField::Leading, self, context)?
                }
                sqlparser::ast::TrimWhereField::Trailing => transformer.transform(
                    sqlparser::ast::TrimWhereField::Trailing,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateCascadeOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TruncateCascadeOption::Cascade => transformer.transform(
                    sqlparser::ast::TruncateCascadeOption::Cascade,
                    self,
                    context,
                )?,
                sqlparser::ast::TruncateCascadeOption::Restrict => transformer.transform(
                    sqlparser::ast::TruncateCascadeOption::Restrict,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateIdentityOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::TruncateIdentityOption::Restart => transformer.transform(
                    sqlparser::ast::TruncateIdentityOption::Restart,
                    self,
                    context,
                )?,
                sqlparser::ast::TruncateIdentityOption::Continue => transformer.transform(
                    sqlparser::ast::TruncateIdentityOption::Continue,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateTableTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UnaryOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::UnaryOperator::Plus => {
                    transformer.transform(sqlparser::ast::UnaryOperator::Plus, self, context)?
                }
                sqlparser::ast::UnaryOperator::Minus => {
                    transformer.transform(sqlparser::ast::UnaryOperator::Minus, self, context)?
                }
                sqlparser::ast::UnaryOperator::Not => {
                    transformer.transform(sqlparser::ast::UnaryOperator::Not, self, context)?
                }
                sqlparser::ast::UnaryOperator::PGBitwiseNot => transformer.transform(
                    sqlparser::ast::UnaryOperator::PGBitwiseNot,
                    self,
                    context,
                )?,
                sqlparser::ast::UnaryOperator::PGSquareRoot => transformer.transform(
                    sqlparser::ast::UnaryOperator::PGSquareRoot,
                    self,
                    context,
                )?,
                sqlparser::ast::UnaryOperator::PGCubeRoot => transformer.transform(
                    sqlparser::ast::UnaryOperator::PGCubeRoot,
                    self,
                    context,
                )?,
                sqlparser::ast::UnaryOperator::PGPostfixFactorial => transformer.transform(
                    sqlparser::ast::UnaryOperator::PGPostfixFactorial,
                    self,
                    context,
                )?,
                sqlparser::ast::UnaryOperator::PGPrefixFactorial => transformer.transform(
                    sqlparser::ast::UnaryOperator::PGPrefixFactorial,
                    self,
                    context,
                )?,
                sqlparser::ast::UnaryOperator::PGAbs => {
                    transformer.transform(sqlparser::ast::UnaryOperator::PGAbs, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UnionField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                field_name,
                field_type,
            } = self;
            Self {
                field_name: field_name.apply_transform_with_path(transformer, context)?,
                field_type: field_type.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Use {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Use::Catalog(field0) => sqlparser::ast::Use::Catalog(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Use::Schema(field0) => sqlparser::ast::Use::Schema(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Use::Database(field0) => sqlparser::ast::Use::Database(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Use::Warehouse(field0) => sqlparser::ast::Use::Warehouse(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Use::Object(field0) => sqlparser::ast::Use::Object(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Use::Default => {
                    transformer.transform(sqlparser::ast::Use::Default, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UserDefinedTypeCompositeAttributeDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                name,
                data_type,
                collation,
            } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                data_type: data_type.apply_transform_with_path(transformer, context)?,
                collation: collation.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UserDefinedTypeRepresentation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::UserDefinedTypeRepresentation::Composite { attributes } => {
                    sqlparser::ast::UserDefinedTypeRepresentation::Composite {
                        attributes: attributes.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::ast::UserDefinedTypeRepresentation::Enum { labels } => {
                    sqlparser::ast::UserDefinedTypeRepresentation::Enum {
                        labels: labels.apply_transform_with_path(transformer, context)?,
                    }
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UtilityOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { name, arg } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                arg: arg.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Value {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::Value::Number(field0, field1) => sqlparser::ast::Value::Number(
                    field0.apply_transform_with_path(transformer, context)?,
                    field1.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Value::SingleQuotedString(field0) => {
                    sqlparser::ast::Value::SingleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::DollarQuotedString(field0) => {
                    sqlparser::ast::Value::DollarQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::TripleSingleQuotedString(field0) => {
                    sqlparser::ast::Value::TripleSingleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::TripleDoubleQuotedString(field0) => {
                    sqlparser::ast::Value::TripleDoubleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::EscapedStringLiteral(field0) => {
                    sqlparser::ast::Value::EscapedStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::UnicodeStringLiteral(field0) => {
                    sqlparser::ast::Value::UnicodeStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::SingleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::SingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::DoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::DoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::SingleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::SingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::DoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::DoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::NationalStringLiteral(field0) => {
                    sqlparser::ast::Value::NationalStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::HexStringLiteral(field0) => {
                    sqlparser::ast::Value::HexStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::DoubleQuotedString(field0) => {
                    sqlparser::ast::Value::DoubleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::Value::Boolean(field0) => sqlparser::ast::Value::Boolean(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::ast::Value::Null => {
                    transformer.transform(sqlparser::ast::Value::Null, self, context)?
                }
                sqlparser::ast::Value::Placeholder(field0) => sqlparser::ast::Value::Placeholder(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ValueTableMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::ValueTableMode::AsStruct => transformer.transform(
                    sqlparser::ast::ValueTableMode::AsStruct,
                    self,
                    context,
                )?,
                sqlparser::ast::ValueTableMode::AsValue => {
                    transformer.transform(sqlparser::ast::ValueTableMode::AsValue, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Values {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { explicit_row, rows } = self;
            Self {
                explicit_row: explicit_row.apply_transform_with_path(transformer, context)?,
                rows: rows.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ViewColumnDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                name,
                data_type,
                options,
            } = self;
            Self {
                name: name.apply_transform_with_path(transformer, context)?,
                data_type: data_type.apply_transform_with_path(transformer, context)?,
                options: options.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WildcardAdditionalOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                opt_ilike,
                opt_exclude,
                opt_except,
                opt_replace,
                opt_rename,
            } = self;
            Self {
                opt_ilike: opt_ilike.apply_transform_with_path(transformer, context)?,
                opt_exclude: opt_exclude.apply_transform_with_path(transformer, context)?,
                opt_except: opt_except.apply_transform_with_path(transformer, context)?,
                opt_replace: opt_replace.apply_transform_with_path(transformer, context)?,
                opt_rename: opt_rename.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrame {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                units,
                start_bound,
                end_bound,
            } = self;
            Self {
                units: units.apply_transform_with_path(transformer, context)?,
                start_bound: start_bound.apply_transform_with_path(transformer, context)?,
                end_bound: end_bound.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrameBound {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::WindowFrameBound::CurrentRow => transformer.transform(
                    sqlparser::ast::WindowFrameBound::CurrentRow,
                    self,
                    context,
                )?,
                sqlparser::ast::WindowFrameBound::Preceding(field0) => {
                    sqlparser::ast::WindowFrameBound::Preceding(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::WindowFrameBound::Following(field0) => {
                    sqlparser::ast::WindowFrameBound::Following(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrameUnits {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::WindowFrameUnits::Rows => {
                    transformer.transform(sqlparser::ast::WindowFrameUnits::Rows, self, context)?
                }
                sqlparser::ast::WindowFrameUnits::Range => {
                    transformer.transform(sqlparser::ast::WindowFrameUnits::Range, self, context)?
                }
                sqlparser::ast::WindowFrameUnits::Groups => transformer.transform(
                    sqlparser::ast::WindowFrameUnits::Groups,
                    self,
                    context,
                )?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowSpec {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                window_name,
                partition_by,
                order_by,
                window_frame,
            } = self;
            Self {
                window_name: window_name.apply_transform_with_path(transformer, context)?,
                partition_by: partition_by.apply_transform_with_path(transformer, context)?,
                order_by: order_by.apply_transform_with_path(transformer, context)?,
                window_frame: window_frame.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::WindowType::WindowSpec(field0) => {
                    sqlparser::ast::WindowType::WindowSpec(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::ast::WindowType::NamedWindow(field0) => {
                    sqlparser::ast::WindowType::NamedWindow(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::With {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                recursive,
                cte_tables,
            } = self;
            Self {
                recursive: recursive.apply_transform_with_path(transformer, context)?,
                cte_tables: cte_tables.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WithFill {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { from, to, step } = self;
            Self {
                from: from.apply_transform_with_path(transformer, context)?,
                to: to.apply_transform_with_path(transformer, context)?,
                step: step.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
    for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOption
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                option_name,
                option_type,
                value,
            } = self;
            Self {
                option_name: option_name.apply_transform_with_path(transformer, context)?,
                option_type: option_type.apply_transform_with_path(transformer, context)?,
                value: value.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
    for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING => {
                    transformer.transform(
                        sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING,
                        self,
                        context,
                    )?
                }
                sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN => {
                    transformer.transform(
                        sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN,
                        self,
                        context,
                    )?
                }
                sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM => {
                    transformer.transform(
                        sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM,
                        self,
                        context,
                    )?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
    for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptions
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self { options } = self;
            Self {
                options: options.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
    for sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                alias,
                file_col_num,
                element,
                item_as,
            } = self;
            Self {
                alias: alias.apply_transform_with_path(transformer, context)?,
                file_col_num: file_col_num.apply_transform_with_path(transformer, context)?,
                element: element.apply_transform_with_path(transformer, context)?,
                item_as: item_as.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
    for sqlparser::ast::helpers::stmt_data_loading::StageParamsObject
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                url,
                encryption,
                endpoint,
                storage_integration,
                credentials,
            } = self;
            Self {
                url: url.apply_transform_with_path(transformer, context)?,
                encryption: encryption.apply_transform_with_path(transformer, context)?,
                endpoint: endpoint.apply_transform_with_path(transformer, context)?,
                storage_integration: storage_integration
                    .apply_transform_with_path(transformer, context)?,
                credentials: credentials.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::keywords::Keyword {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::keywords::Keyword::NoKeyword => {
                    transformer.transform(sqlparser::keywords::Keyword::NoKeyword, self, context)?
                }
                sqlparser::keywords::Keyword::ABORT => {
                    transformer.transform(sqlparser::keywords::Keyword::ABORT, self, context)?
                }
                sqlparser::keywords::Keyword::ABS => {
                    transformer.transform(sqlparser::keywords::Keyword::ABS, self, context)?
                }
                sqlparser::keywords::Keyword::ABSOLUTE => {
                    transformer.transform(sqlparser::keywords::Keyword::ABSOLUTE, self, context)?
                }
                sqlparser::keywords::Keyword::ACCESS => {
                    transformer.transform(sqlparser::keywords::Keyword::ACCESS, self, context)?
                }
                sqlparser::keywords::Keyword::ACTION => {
                    transformer.transform(sqlparser::keywords::Keyword::ACTION, self, context)?
                }
                sqlparser::keywords::Keyword::ADD => {
                    transformer.transform(sqlparser::keywords::Keyword::ADD, self, context)?
                }
                sqlparser::keywords::Keyword::ADMIN => {
                    transformer.transform(sqlparser::keywords::Keyword::ADMIN, self, context)?
                }
                sqlparser::keywords::Keyword::AFTER => {
                    transformer.transform(sqlparser::keywords::Keyword::AFTER, self, context)?
                }
                sqlparser::keywords::Keyword::AGAINST => {
                    transformer.transform(sqlparser::keywords::Keyword::AGAINST, self, context)?
                }
                sqlparser::keywords::Keyword::AGGREGATION => transformer.transform(
                    sqlparser::keywords::Keyword::AGGREGATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::ALIAS => {
                    transformer.transform(sqlparser::keywords::Keyword::ALIAS, self, context)?
                }
                sqlparser::keywords::Keyword::ALL => {
                    transformer.transform(sqlparser::keywords::Keyword::ALL, self, context)?
                }
                sqlparser::keywords::Keyword::ALLOCATE => {
                    transformer.transform(sqlparser::keywords::Keyword::ALLOCATE, self, context)?
                }
                sqlparser::keywords::Keyword::ALTER => {
                    transformer.transform(sqlparser::keywords::Keyword::ALTER, self, context)?
                }
                sqlparser::keywords::Keyword::ALWAYS => {
                    transformer.transform(sqlparser::keywords::Keyword::ALWAYS, self, context)?
                }
                sqlparser::keywords::Keyword::ANALYZE => {
                    transformer.transform(sqlparser::keywords::Keyword::ANALYZE, self, context)?
                }
                sqlparser::keywords::Keyword::AND => {
                    transformer.transform(sqlparser::keywords::Keyword::AND, self, context)?
                }
                sqlparser::keywords::Keyword::ANTI => {
                    transformer.transform(sqlparser::keywords::Keyword::ANTI, self, context)?
                }
                sqlparser::keywords::Keyword::ANY => {
                    transformer.transform(sqlparser::keywords::Keyword::ANY, self, context)?
                }
                sqlparser::keywords::Keyword::APPLY => {
                    transformer.transform(sqlparser::keywords::Keyword::APPLY, self, context)?
                }
                sqlparser::keywords::Keyword::ARCHIVE => {
                    transformer.transform(sqlparser::keywords::Keyword::ARCHIVE, self, context)?
                }
                sqlparser::keywords::Keyword::ARE => {
                    transformer.transform(sqlparser::keywords::Keyword::ARE, self, context)?
                }
                sqlparser::keywords::Keyword::ARRAY => {
                    transformer.transform(sqlparser::keywords::Keyword::ARRAY, self, context)?
                }
                sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY => transformer.transform(
                    sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::AS => {
                    transformer.transform(sqlparser::keywords::Keyword::AS, self, context)?
                }
                sqlparser::keywords::Keyword::ASC => {
                    transformer.transform(sqlparser::keywords::Keyword::ASC, self, context)?
                }
                sqlparser::keywords::Keyword::ASENSITIVE => transformer.transform(
                    sqlparser::keywords::Keyword::ASENSITIVE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::ASOF => {
                    transformer.transform(sqlparser::keywords::Keyword::ASOF, self, context)?
                }
                sqlparser::keywords::Keyword::ASSERT => {
                    transformer.transform(sqlparser::keywords::Keyword::ASSERT, self, context)?
                }
                sqlparser::keywords::Keyword::ASYMMETRIC => transformer.transform(
                    sqlparser::keywords::Keyword::ASYMMETRIC,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::AT => {
                    transformer.transform(sqlparser::keywords::Keyword::AT, self, context)?
                }
                sqlparser::keywords::Keyword::ATOMIC => {
                    transformer.transform(sqlparser::keywords::Keyword::ATOMIC, self, context)?
                }
                sqlparser::keywords::Keyword::ATTACH => {
                    transformer.transform(sqlparser::keywords::Keyword::ATTACH, self, context)?
                }
                sqlparser::keywords::Keyword::AUTHORIZATION => transformer.transform(
                    sqlparser::keywords::Keyword::AUTHORIZATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::AUTO => {
                    transformer.transform(sqlparser::keywords::Keyword::AUTO, self, context)?
                }
                sqlparser::keywords::Keyword::AUTOINCREMENT => transformer.transform(
                    sqlparser::keywords::Keyword::AUTOINCREMENT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::AUTO_INCREMENT => transformer.transform(
                    sqlparser::keywords::Keyword::AUTO_INCREMENT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::AVG => {
                    transformer.transform(sqlparser::keywords::Keyword::AVG, self, context)?
                }
                sqlparser::keywords::Keyword::AVRO => {
                    transformer.transform(sqlparser::keywords::Keyword::AVRO, self, context)?
                }
                sqlparser::keywords::Keyword::BACKWARD => {
                    transformer.transform(sqlparser::keywords::Keyword::BACKWARD, self, context)?
                }
                sqlparser::keywords::Keyword::BASE64 => {
                    transformer.transform(sqlparser::keywords::Keyword::BASE64, self, context)?
                }
                sqlparser::keywords::Keyword::BEFORE => {
                    transformer.transform(sqlparser::keywords::Keyword::BEFORE, self, context)?
                }
                sqlparser::keywords::Keyword::BEGIN => {
                    transformer.transform(sqlparser::keywords::Keyword::BEGIN, self, context)?
                }
                sqlparser::keywords::Keyword::BEGIN_FRAME => transformer.transform(
                    sqlparser::keywords::Keyword::BEGIN_FRAME,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::BEGIN_PARTITION => transformer.transform(
                    sqlparser::keywords::Keyword::BEGIN_PARTITION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::BETWEEN => {
                    transformer.transform(sqlparser::keywords::Keyword::BETWEEN, self, context)?
                }
                sqlparser::keywords::Keyword::BIGDECIMAL => transformer.transform(
                    sqlparser::keywords::Keyword::BIGDECIMAL,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::BIGINT => {
                    transformer.transform(sqlparser::keywords::Keyword::BIGINT, self, context)?
                }
                sqlparser::keywords::Keyword::BIGNUMERIC => transformer.transform(
                    sqlparser::keywords::Keyword::BIGNUMERIC,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::BINARY => {
                    transformer.transform(sqlparser::keywords::Keyword::BINARY, self, context)?
                }
                sqlparser::keywords::Keyword::BINDING => {
                    transformer.transform(sqlparser::keywords::Keyword::BINDING, self, context)?
                }
                sqlparser::keywords::Keyword::BLOB => {
                    transformer.transform(sqlparser::keywords::Keyword::BLOB, self, context)?
                }
                sqlparser::keywords::Keyword::BLOOMFILTER => transformer.transform(
                    sqlparser::keywords::Keyword::BLOOMFILTER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::BOOL => {
                    transformer.transform(sqlparser::keywords::Keyword::BOOL, self, context)?
                }
                sqlparser::keywords::Keyword::BOOLEAN => {
                    transformer.transform(sqlparser::keywords::Keyword::BOOLEAN, self, context)?
                }
                sqlparser::keywords::Keyword::BOTH => {
                    transformer.transform(sqlparser::keywords::Keyword::BOTH, self, context)?
                }
                sqlparser::keywords::Keyword::BROWSE => {
                    transformer.transform(sqlparser::keywords::Keyword::BROWSE, self, context)?
                }
                sqlparser::keywords::Keyword::BTREE => {
                    transformer.transform(sqlparser::keywords::Keyword::BTREE, self, context)?
                }
                sqlparser::keywords::Keyword::BUCKETS => {
                    transformer.transform(sqlparser::keywords::Keyword::BUCKETS, self, context)?
                }
                sqlparser::keywords::Keyword::BY => {
                    transformer.transform(sqlparser::keywords::Keyword::BY, self, context)?
                }
                sqlparser::keywords::Keyword::BYPASSRLS => {
                    transformer.transform(sqlparser::keywords::Keyword::BYPASSRLS, self, context)?
                }
                sqlparser::keywords::Keyword::BYTEA => {
                    transformer.transform(sqlparser::keywords::Keyword::BYTEA, self, context)?
                }
                sqlparser::keywords::Keyword::BYTES => {
                    transformer.transform(sqlparser::keywords::Keyword::BYTES, self, context)?
                }
                sqlparser::keywords::Keyword::CACHE => {
                    transformer.transform(sqlparser::keywords::Keyword::CACHE, self, context)?
                }
                sqlparser::keywords::Keyword::CALL => {
                    transformer.transform(sqlparser::keywords::Keyword::CALL, self, context)?
                }
                sqlparser::keywords::Keyword::CALLED => {
                    transformer.transform(sqlparser::keywords::Keyword::CALLED, self, context)?
                }
                sqlparser::keywords::Keyword::CARDINALITY => transformer.transform(
                    sqlparser::keywords::Keyword::CARDINALITY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CASCADE => {
                    transformer.transform(sqlparser::keywords::Keyword::CASCADE, self, context)?
                }
                sqlparser::keywords::Keyword::CASCADED => {
                    transformer.transform(sqlparser::keywords::Keyword::CASCADED, self, context)?
                }
                sqlparser::keywords::Keyword::CASE => {
                    transformer.transform(sqlparser::keywords::Keyword::CASE, self, context)?
                }
                sqlparser::keywords::Keyword::CAST => {
                    transformer.transform(sqlparser::keywords::Keyword::CAST, self, context)?
                }
                sqlparser::keywords::Keyword::CATALOG => {
                    transformer.transform(sqlparser::keywords::Keyword::CATALOG, self, context)?
                }
                sqlparser::keywords::Keyword::CEIL => {
                    transformer.transform(sqlparser::keywords::Keyword::CEIL, self, context)?
                }
                sqlparser::keywords::Keyword::CEILING => {
                    transformer.transform(sqlparser::keywords::Keyword::CEILING, self, context)?
                }
                sqlparser::keywords::Keyword::CENTURY => {
                    transformer.transform(sqlparser::keywords::Keyword::CENTURY, self, context)?
                }
                sqlparser::keywords::Keyword::CHAIN => {
                    transformer.transform(sqlparser::keywords::Keyword::CHAIN, self, context)?
                }
                sqlparser::keywords::Keyword::CHANGE => {
                    transformer.transform(sqlparser::keywords::Keyword::CHANGE, self, context)?
                }
                sqlparser::keywords::Keyword::CHANGE_TRACKING => transformer.transform(
                    sqlparser::keywords::Keyword::CHANGE_TRACKING,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CHANNEL => {
                    transformer.transform(sqlparser::keywords::Keyword::CHANNEL, self, context)?
                }
                sqlparser::keywords::Keyword::CHAR => {
                    transformer.transform(sqlparser::keywords::Keyword::CHAR, self, context)?
                }
                sqlparser::keywords::Keyword::CHARACTER => {
                    transformer.transform(sqlparser::keywords::Keyword::CHARACTER, self, context)?
                }
                sqlparser::keywords::Keyword::CHARACTERS => transformer.transform(
                    sqlparser::keywords::Keyword::CHARACTERS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CHARACTER_LENGTH => transformer.transform(
                    sqlparser::keywords::Keyword::CHARACTER_LENGTH,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CHARSET => {
                    transformer.transform(sqlparser::keywords::Keyword::CHARSET, self, context)?
                }
                sqlparser::keywords::Keyword::CHAR_LENGTH => transformer.transform(
                    sqlparser::keywords::Keyword::CHAR_LENGTH,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CHECK => {
                    transformer.transform(sqlparser::keywords::Keyword::CHECK, self, context)?
                }
                sqlparser::keywords::Keyword::CLEAR => {
                    transformer.transform(sqlparser::keywords::Keyword::CLEAR, self, context)?
                }
                sqlparser::keywords::Keyword::CLOB => {
                    transformer.transform(sqlparser::keywords::Keyword::CLOB, self, context)?
                }
                sqlparser::keywords::Keyword::CLONE => {
                    transformer.transform(sqlparser::keywords::Keyword::CLONE, self, context)?
                }
                sqlparser::keywords::Keyword::CLOSE => {
                    transformer.transform(sqlparser::keywords::Keyword::CLOSE, self, context)?
                }
                sqlparser::keywords::Keyword::CLUSTER => {
                    transformer.transform(sqlparser::keywords::Keyword::CLUSTER, self, context)?
                }
                sqlparser::keywords::Keyword::CLUSTERED => {
                    transformer.transform(sqlparser::keywords::Keyword::CLUSTERED, self, context)?
                }
                sqlparser::keywords::Keyword::COALESCE => {
                    transformer.transform(sqlparser::keywords::Keyword::COALESCE, self, context)?
                }
                sqlparser::keywords::Keyword::COLLATE => {
                    transformer.transform(sqlparser::keywords::Keyword::COLLATE, self, context)?
                }
                sqlparser::keywords::Keyword::COLLATION => {
                    transformer.transform(sqlparser::keywords::Keyword::COLLATION, self, context)?
                }
                sqlparser::keywords::Keyword::COLLECT => {
                    transformer.transform(sqlparser::keywords::Keyword::COLLECT, self, context)?
                }
                sqlparser::keywords::Keyword::COLLECTION => transformer.transform(
                    sqlparser::keywords::Keyword::COLLECTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::COLUMN => {
                    transformer.transform(sqlparser::keywords::Keyword::COLUMN, self, context)?
                }
                sqlparser::keywords::Keyword::COLUMNS => {
                    transformer.transform(sqlparser::keywords::Keyword::COLUMNS, self, context)?
                }
                sqlparser::keywords::Keyword::COLUMNSTORE => transformer.transform(
                    sqlparser::keywords::Keyword::COLUMNSTORE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::COMMENT => {
                    transformer.transform(sqlparser::keywords::Keyword::COMMENT, self, context)?
                }
                sqlparser::keywords::Keyword::COMMIT => {
                    transformer.transform(sqlparser::keywords::Keyword::COMMIT, self, context)?
                }
                sqlparser::keywords::Keyword::COMMITTED => {
                    transformer.transform(sqlparser::keywords::Keyword::COMMITTED, self, context)?
                }
                sqlparser::keywords::Keyword::COMPRESSION => transformer.transform(
                    sqlparser::keywords::Keyword::COMPRESSION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::COMPUTE => {
                    transformer.transform(sqlparser::keywords::Keyword::COMPUTE, self, context)?
                }
                sqlparser::keywords::Keyword::CONCURRENTLY => transformer.transform(
                    sqlparser::keywords::Keyword::CONCURRENTLY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CONDITION => {
                    transformer.transform(sqlparser::keywords::Keyword::CONDITION, self, context)?
                }
                sqlparser::keywords::Keyword::CONFLICT => {
                    transformer.transform(sqlparser::keywords::Keyword::CONFLICT, self, context)?
                }
                sqlparser::keywords::Keyword::CONNECT => {
                    transformer.transform(sqlparser::keywords::Keyword::CONNECT, self, context)?
                }
                sqlparser::keywords::Keyword::CONNECTION => transformer.transform(
                    sqlparser::keywords::Keyword::CONNECTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CONSTRAINT => transformer.transform(
                    sqlparser::keywords::Keyword::CONSTRAINT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CONTAINS => {
                    transformer.transform(sqlparser::keywords::Keyword::CONTAINS, self, context)?
                }
                sqlparser::keywords::Keyword::CONTINUE => {
                    transformer.transform(sqlparser::keywords::Keyword::CONTINUE, self, context)?
                }
                sqlparser::keywords::Keyword::CONVERT => {
                    transformer.transform(sqlparser::keywords::Keyword::CONVERT, self, context)?
                }
                sqlparser::keywords::Keyword::COPY => {
                    transformer.transform(sqlparser::keywords::Keyword::COPY, self, context)?
                }
                sqlparser::keywords::Keyword::COPY_OPTIONS => transformer.transform(
                    sqlparser::keywords::Keyword::COPY_OPTIONS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CORR => {
                    transformer.transform(sqlparser::keywords::Keyword::CORR, self, context)?
                }
                sqlparser::keywords::Keyword::CORRESPONDING => transformer.transform(
                    sqlparser::keywords::Keyword::CORRESPONDING,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::COUNT => {
                    transformer.transform(sqlparser::keywords::Keyword::COUNT, self, context)?
                }
                sqlparser::keywords::Keyword::COVAR_POP => {
                    transformer.transform(sqlparser::keywords::Keyword::COVAR_POP, self, context)?
                }
                sqlparser::keywords::Keyword::COVAR_SAMP => transformer.transform(
                    sqlparser::keywords::Keyword::COVAR_SAMP,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CREATE => {
                    transformer.transform(sqlparser::keywords::Keyword::CREATE, self, context)?
                }
                sqlparser::keywords::Keyword::CREATEDB => {
                    transformer.transform(sqlparser::keywords::Keyword::CREATEDB, self, context)?
                }
                sqlparser::keywords::Keyword::CREATEROLE => transformer.transform(
                    sqlparser::keywords::Keyword::CREATEROLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CREDENTIALS => transformer.transform(
                    sqlparser::keywords::Keyword::CREDENTIALS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CROSS => {
                    transformer.transform(sqlparser::keywords::Keyword::CROSS, self, context)?
                }
                sqlparser::keywords::Keyword::CSV => {
                    transformer.transform(sqlparser::keywords::Keyword::CSV, self, context)?
                }
                sqlparser::keywords::Keyword::CUBE => {
                    transformer.transform(sqlparser::keywords::Keyword::CUBE, self, context)?
                }
                sqlparser::keywords::Keyword::CUME_DIST => {
                    transformer.transform(sqlparser::keywords::Keyword::CUME_DIST, self, context)?
                }
                sqlparser::keywords::Keyword::CURRENT => {
                    transformer.transform(sqlparser::keywords::Keyword::CURRENT, self, context)?
                }
                sqlparser::keywords::Keyword::CURRENT_CATALOG => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_CATALOG,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_DATE => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_DATE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP => transformer
                    .transform(
                        sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP,
                        self,
                        context,
                    )?,
                sqlparser::keywords::Keyword::CURRENT_PATH => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_PATH,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_ROLE => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_ROLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_ROW => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_ROW,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_SCHEMA => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_SCHEMA,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_TIME => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_TIME,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_TIMESTAMP => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_TIMESTAMP,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE => transformer
                    .transform(
                        sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE,
                        self,
                        context,
                    )?,
                sqlparser::keywords::Keyword::CURRENT_USER => transformer.transform(
                    sqlparser::keywords::Keyword::CURRENT_USER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::CURSOR => {
                    transformer.transform(sqlparser::keywords::Keyword::CURSOR, self, context)?
                }
                sqlparser::keywords::Keyword::CYCLE => {
                    transformer.transform(sqlparser::keywords::Keyword::CYCLE, self, context)?
                }
                sqlparser::keywords::Keyword::DATA => {
                    transformer.transform(sqlparser::keywords::Keyword::DATA, self, context)?
                }
                sqlparser::keywords::Keyword::DATABASE => {
                    transformer.transform(sqlparser::keywords::Keyword::DATABASE, self, context)?
                }
                sqlparser::keywords::Keyword::DATABASES => {
                    transformer.transform(sqlparser::keywords::Keyword::DATABASES, self, context)?
                }
                sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS => transformer
                    .transform(
                        sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS,
                        self,
                        context,
                    )?,
                sqlparser::keywords::Keyword::DATE => {
                    transformer.transform(sqlparser::keywords::Keyword::DATE, self, context)?
                }
                sqlparser::keywords::Keyword::DATE32 => {
                    transformer.transform(sqlparser::keywords::Keyword::DATE32, self, context)?
                }
                sqlparser::keywords::Keyword::DATETIME => {
                    transformer.transform(sqlparser::keywords::Keyword::DATETIME, self, context)?
                }
                sqlparser::keywords::Keyword::DATETIME64 => transformer.transform(
                    sqlparser::keywords::Keyword::DATETIME64,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DAY => {
                    transformer.transform(sqlparser::keywords::Keyword::DAY, self, context)?
                }
                sqlparser::keywords::Keyword::DAYOFWEEK => {
                    transformer.transform(sqlparser::keywords::Keyword::DAYOFWEEK, self, context)?
                }
                sqlparser::keywords::Keyword::DAYOFYEAR => {
                    transformer.transform(sqlparser::keywords::Keyword::DAYOFYEAR, self, context)?
                }
                sqlparser::keywords::Keyword::DEALLOCATE => transformer.transform(
                    sqlparser::keywords::Keyword::DEALLOCATE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DEC => {
                    transformer.transform(sqlparser::keywords::Keyword::DEC, self, context)?
                }
                sqlparser::keywords::Keyword::DECADE => {
                    transformer.transform(sqlparser::keywords::Keyword::DECADE, self, context)?
                }
                sqlparser::keywords::Keyword::DECIMAL => {
                    transformer.transform(sqlparser::keywords::Keyword::DECIMAL, self, context)?
                }
                sqlparser::keywords::Keyword::DECLARE => {
                    transformer.transform(sqlparser::keywords::Keyword::DECLARE, self, context)?
                }
                sqlparser::keywords::Keyword::DEDUPLICATE => transformer.transform(
                    sqlparser::keywords::Keyword::DEDUPLICATE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DEFAULT => {
                    transformer.transform(sqlparser::keywords::Keyword::DEFAULT, self, context)?
                }
                sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION => transformer.transform(
                    sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DEFERRABLE => transformer.transform(
                    sqlparser::keywords::Keyword::DEFERRABLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DEFERRED => {
                    transformer.transform(sqlparser::keywords::Keyword::DEFERRED, self, context)?
                }
                sqlparser::keywords::Keyword::DEFINE => {
                    transformer.transform(sqlparser::keywords::Keyword::DEFINE, self, context)?
                }
                sqlparser::keywords::Keyword::DEFINED => {
                    transformer.transform(sqlparser::keywords::Keyword::DEFINED, self, context)?
                }
                sqlparser::keywords::Keyword::DELAYED => {
                    transformer.transform(sqlparser::keywords::Keyword::DELAYED, self, context)?
                }
                sqlparser::keywords::Keyword::DELETE => {
                    transformer.transform(sqlparser::keywords::Keyword::DELETE, self, context)?
                }
                sqlparser::keywords::Keyword::DELIMITED => {
                    transformer.transform(sqlparser::keywords::Keyword::DELIMITED, self, context)?
                }
                sqlparser::keywords::Keyword::DELIMITER => {
                    transformer.transform(sqlparser::keywords::Keyword::DELIMITER, self, context)?
                }
                sqlparser::keywords::Keyword::DELTA => {
                    transformer.transform(sqlparser::keywords::Keyword::DELTA, self, context)?
                }
                sqlparser::keywords::Keyword::DENSE_RANK => transformer.transform(
                    sqlparser::keywords::Keyword::DENSE_RANK,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DEREF => {
                    transformer.transform(sqlparser::keywords::Keyword::DEREF, self, context)?
                }
                sqlparser::keywords::Keyword::DESC => {
                    transformer.transform(sqlparser::keywords::Keyword::DESC, self, context)?
                }
                sqlparser::keywords::Keyword::DESCRIBE => {
                    transformer.transform(sqlparser::keywords::Keyword::DESCRIBE, self, context)?
                }
                sqlparser::keywords::Keyword::DETACH => {
                    transformer.transform(sqlparser::keywords::Keyword::DETACH, self, context)?
                }
                sqlparser::keywords::Keyword::DETAIL => {
                    transformer.transform(sqlparser::keywords::Keyword::DETAIL, self, context)?
                }
                sqlparser::keywords::Keyword::DETERMINISTIC => transformer.transform(
                    sqlparser::keywords::Keyword::DETERMINISTIC,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DIRECTORY => {
                    transformer.transform(sqlparser::keywords::Keyword::DIRECTORY, self, context)?
                }
                sqlparser::keywords::Keyword::DISABLE => {
                    transformer.transform(sqlparser::keywords::Keyword::DISABLE, self, context)?
                }
                sqlparser::keywords::Keyword::DISCARD => {
                    transformer.transform(sqlparser::keywords::Keyword::DISCARD, self, context)?
                }
                sqlparser::keywords::Keyword::DISCONNECT => transformer.transform(
                    sqlparser::keywords::Keyword::DISCONNECT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DISTINCT => {
                    transformer.transform(sqlparser::keywords::Keyword::DISTINCT, self, context)?
                }
                sqlparser::keywords::Keyword::DISTRIBUTE => transformer.transform(
                    sqlparser::keywords::Keyword::DISTRIBUTE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::DIV => {
                    transformer.transform(sqlparser::keywords::Keyword::DIV, self, context)?
                }
                sqlparser::keywords::Keyword::DO => {
                    transformer.transform(sqlparser::keywords::Keyword::DO, self, context)?
                }
                sqlparser::keywords::Keyword::DOUBLE => {
                    transformer.transform(sqlparser::keywords::Keyword::DOUBLE, self, context)?
                }
                sqlparser::keywords::Keyword::DOW => {
                    transformer.transform(sqlparser::keywords::Keyword::DOW, self, context)?
                }
                sqlparser::keywords::Keyword::DOY => {
                    transformer.transform(sqlparser::keywords::Keyword::DOY, self, context)?
                }
                sqlparser::keywords::Keyword::DROP => {
                    transformer.transform(sqlparser::keywords::Keyword::DROP, self, context)?
                }
                sqlparser::keywords::Keyword::DRY => {
                    transformer.transform(sqlparser::keywords::Keyword::DRY, self, context)?
                }
                sqlparser::keywords::Keyword::DUPLICATE => {
                    transformer.transform(sqlparser::keywords::Keyword::DUPLICATE, self, context)?
                }
                sqlparser::keywords::Keyword::DYNAMIC => {
                    transformer.transform(sqlparser::keywords::Keyword::DYNAMIC, self, context)?
                }
                sqlparser::keywords::Keyword::EACH => {
                    transformer.transform(sqlparser::keywords::Keyword::EACH, self, context)?
                }
                sqlparser::keywords::Keyword::ELEMENT => {
                    transformer.transform(sqlparser::keywords::Keyword::ELEMENT, self, context)?
                }
                sqlparser::keywords::Keyword::ELEMENTS => {
                    transformer.transform(sqlparser::keywords::Keyword::ELEMENTS, self, context)?
                }
                sqlparser::keywords::Keyword::ELSE => {
                    transformer.transform(sqlparser::keywords::Keyword::ELSE, self, context)?
                }
                sqlparser::keywords::Keyword::EMPTY => {
                    transformer.transform(sqlparser::keywords::Keyword::EMPTY, self, context)?
                }
                sqlparser::keywords::Keyword::ENABLE => {
                    transformer.transform(sqlparser::keywords::Keyword::ENABLE, self, context)?
                }
                sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION => transformer.transform(
                    sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::ENCODING => {
                    transformer.transform(sqlparser::keywords::Keyword::ENCODING, self, context)?
                }
                sqlparser::keywords::Keyword::ENCRYPTION => transformer.transform(
                    sqlparser::keywords::Keyword::ENCRYPTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::END => {
                    transformer.transform(sqlparser::keywords::Keyword::END, self, context)?
                }
                sqlparser::keywords::Keyword::END_EXEC => {
                    transformer.transform(sqlparser::keywords::Keyword::END_EXEC, self, context)?
                }
                sqlparser::keywords::Keyword::ENDPOINT => {
                    transformer.transform(sqlparser::keywords::Keyword::ENDPOINT, self, context)?
                }
                sqlparser::keywords::Keyword::END_FRAME => {
                    transformer.transform(sqlparser::keywords::Keyword::END_FRAME, self, context)?
                }
                sqlparser::keywords::Keyword::END_PARTITION => transformer.transform(
                    sqlparser::keywords::Keyword::END_PARTITION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::ENFORCED => {
                    transformer.transform(sqlparser::keywords::Keyword::ENFORCED, self, context)?
                }
                sqlparser::keywords::Keyword::ENGINE => {
                    transformer.transform(sqlparser::keywords::Keyword::ENGINE, self, context)?
                }
                sqlparser::keywords::Keyword::ENUM => {
                    transformer.transform(sqlparser::keywords::Keyword::ENUM, self, context)?
                }
                sqlparser::keywords::Keyword::EPHEMERAL => {
                    transformer.transform(sqlparser::keywords::Keyword::EPHEMERAL, self, context)?
                }
                sqlparser::keywords::Keyword::EPOCH => {
                    transformer.transform(sqlparser::keywords::Keyword::EPOCH, self, context)?
                }
                sqlparser::keywords::Keyword::EQUALS => {
                    transformer.transform(sqlparser::keywords::Keyword::EQUALS, self, context)?
                }
                sqlparser::keywords::Keyword::ERROR => {
                    transformer.transform(sqlparser::keywords::Keyword::ERROR, self, context)?
                }
                sqlparser::keywords::Keyword::ESCAPE => {
                    transformer.transform(sqlparser::keywords::Keyword::ESCAPE, self, context)?
                }
                sqlparser::keywords::Keyword::ESCAPED => {
                    transformer.transform(sqlparser::keywords::Keyword::ESCAPED, self, context)?
                }
                sqlparser::keywords::Keyword::EVENT => {
                    transformer.transform(sqlparser::keywords::Keyword::EVENT, self, context)?
                }
                sqlparser::keywords::Keyword::EVERY => {
                    transformer.transform(sqlparser::keywords::Keyword::EVERY, self, context)?
                }
                sqlparser::keywords::Keyword::EXCEPT => {
                    transformer.transform(sqlparser::keywords::Keyword::EXCEPT, self, context)?
                }
                sqlparser::keywords::Keyword::EXCEPTION => {
                    transformer.transform(sqlparser::keywords::Keyword::EXCEPTION, self, context)?
                }
                sqlparser::keywords::Keyword::EXCLUDE => {
                    transformer.transform(sqlparser::keywords::Keyword::EXCLUDE, self, context)?
                }
                sqlparser::keywords::Keyword::EXCLUSIVE => {
                    transformer.transform(sqlparser::keywords::Keyword::EXCLUSIVE, self, context)?
                }
                sqlparser::keywords::Keyword::EXEC => {
                    transformer.transform(sqlparser::keywords::Keyword::EXEC, self, context)?
                }
                sqlparser::keywords::Keyword::EXECUTE => {
                    transformer.transform(sqlparser::keywords::Keyword::EXECUTE, self, context)?
                }
                sqlparser::keywords::Keyword::EXISTS => {
                    transformer.transform(sqlparser::keywords::Keyword::EXISTS, self, context)?
                }
                sqlparser::keywords::Keyword::EXP => {
                    transformer.transform(sqlparser::keywords::Keyword::EXP, self, context)?
                }
                sqlparser::keywords::Keyword::EXPANSION => {
                    transformer.transform(sqlparser::keywords::Keyword::EXPANSION, self, context)?
                }
                sqlparser::keywords::Keyword::EXPLAIN => {
                    transformer.transform(sqlparser::keywords::Keyword::EXPLAIN, self, context)?
                }
                sqlparser::keywords::Keyword::EXPLICIT => {
                    transformer.transform(sqlparser::keywords::Keyword::EXPLICIT, self, context)?
                }
                sqlparser::keywords::Keyword::EXPORT => {
                    transformer.transform(sqlparser::keywords::Keyword::EXPORT, self, context)?
                }
                sqlparser::keywords::Keyword::EXTENDED => {
                    transformer.transform(sqlparser::keywords::Keyword::EXTENDED, self, context)?
                }
                sqlparser::keywords::Keyword::EXTENSION => {
                    transformer.transform(sqlparser::keywords::Keyword::EXTENSION, self, context)?
                }
                sqlparser::keywords::Keyword::EXTERNAL => {
                    transformer.transform(sqlparser::keywords::Keyword::EXTERNAL, self, context)?
                }
                sqlparser::keywords::Keyword::EXTRACT => {
                    transformer.transform(sqlparser::keywords::Keyword::EXTRACT, self, context)?
                }
                sqlparser::keywords::Keyword::FAIL => {
                    transformer.transform(sqlparser::keywords::Keyword::FAIL, self, context)?
                }
                sqlparser::keywords::Keyword::FALSE => {
                    transformer.transform(sqlparser::keywords::Keyword::FALSE, self, context)?
                }
                sqlparser::keywords::Keyword::FETCH => {
                    transformer.transform(sqlparser::keywords::Keyword::FETCH, self, context)?
                }
                sqlparser::keywords::Keyword::FIELDS => {
                    transformer.transform(sqlparser::keywords::Keyword::FIELDS, self, context)?
                }
                sqlparser::keywords::Keyword::FILE => {
                    transformer.transform(sqlparser::keywords::Keyword::FILE, self, context)?
                }
                sqlparser::keywords::Keyword::FILES => {
                    transformer.transform(sqlparser::keywords::Keyword::FILES, self, context)?
                }
                sqlparser::keywords::Keyword::FILE_FORMAT => transformer.transform(
                    sqlparser::keywords::Keyword::FILE_FORMAT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::FILL => {
                    transformer.transform(sqlparser::keywords::Keyword::FILL, self, context)?
                }
                sqlparser::keywords::Keyword::FILTER => {
                    transformer.transform(sqlparser::keywords::Keyword::FILTER, self, context)?
                }
                sqlparser::keywords::Keyword::FINAL => {
                    transformer.transform(sqlparser::keywords::Keyword::FINAL, self, context)?
                }
                sqlparser::keywords::Keyword::FIRST => {
                    transformer.transform(sqlparser::keywords::Keyword::FIRST, self, context)?
                }
                sqlparser::keywords::Keyword::FIRST_VALUE => transformer.transform(
                    sqlparser::keywords::Keyword::FIRST_VALUE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::FIXEDSTRING => transformer.transform(
                    sqlparser::keywords::Keyword::FIXEDSTRING,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::FLOAT => {
                    transformer.transform(sqlparser::keywords::Keyword::FLOAT, self, context)?
                }
                sqlparser::keywords::Keyword::FLOAT32 => {
                    transformer.transform(sqlparser::keywords::Keyword::FLOAT32, self, context)?
                }
                sqlparser::keywords::Keyword::FLOAT4 => {
                    transformer.transform(sqlparser::keywords::Keyword::FLOAT4, self, context)?
                }
                sqlparser::keywords::Keyword::FLOAT64 => {
                    transformer.transform(sqlparser::keywords::Keyword::FLOAT64, self, context)?
                }
                sqlparser::keywords::Keyword::FLOAT8 => {
                    transformer.transform(sqlparser::keywords::Keyword::FLOAT8, self, context)?
                }
                sqlparser::keywords::Keyword::FLOOR => {
                    transformer.transform(sqlparser::keywords::Keyword::FLOOR, self, context)?
                }
                sqlparser::keywords::Keyword::FLUSH => {
                    transformer.transform(sqlparser::keywords::Keyword::FLUSH, self, context)?
                }
                sqlparser::keywords::Keyword::FOLLOWING => {
                    transformer.transform(sqlparser::keywords::Keyword::FOLLOWING, self, context)?
                }
                sqlparser::keywords::Keyword::FOR => {
                    transformer.transform(sqlparser::keywords::Keyword::FOR, self, context)?
                }
                sqlparser::keywords::Keyword::FORCE => {
                    transformer.transform(sqlparser::keywords::Keyword::FORCE, self, context)?
                }
                sqlparser::keywords::Keyword::FORCE_NOT_NULL => transformer.transform(
                    sqlparser::keywords::Keyword::FORCE_NOT_NULL,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::FORCE_NULL => transformer.transform(
                    sqlparser::keywords::Keyword::FORCE_NULL,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::FORCE_QUOTE => transformer.transform(
                    sqlparser::keywords::Keyword::FORCE_QUOTE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::FOREIGN => {
                    transformer.transform(sqlparser::keywords::Keyword::FOREIGN, self, context)?
                }
                sqlparser::keywords::Keyword::FORMAT => {
                    transformer.transform(sqlparser::keywords::Keyword::FORMAT, self, context)?
                }
                sqlparser::keywords::Keyword::FORMATTED => {
                    transformer.transform(sqlparser::keywords::Keyword::FORMATTED, self, context)?
                }
                sqlparser::keywords::Keyword::FORWARD => {
                    transformer.transform(sqlparser::keywords::Keyword::FORWARD, self, context)?
                }
                sqlparser::keywords::Keyword::FRAME_ROW => {
                    transformer.transform(sqlparser::keywords::Keyword::FRAME_ROW, self, context)?
                }
                sqlparser::keywords::Keyword::FREE => {
                    transformer.transform(sqlparser::keywords::Keyword::FREE, self, context)?
                }
                sqlparser::keywords::Keyword::FREEZE => {
                    transformer.transform(sqlparser::keywords::Keyword::FREEZE, self, context)?
                }
                sqlparser::keywords::Keyword::FROM => {
                    transformer.transform(sqlparser::keywords::Keyword::FROM, self, context)?
                }
                sqlparser::keywords::Keyword::FSCK => {
                    transformer.transform(sqlparser::keywords::Keyword::FSCK, self, context)?
                }
                sqlparser::keywords::Keyword::FULL => {
                    transformer.transform(sqlparser::keywords::Keyword::FULL, self, context)?
                }
                sqlparser::keywords::Keyword::FULLTEXT => {
                    transformer.transform(sqlparser::keywords::Keyword::FULLTEXT, self, context)?
                }
                sqlparser::keywords::Keyword::FUNCTION => {
                    transformer.transform(sqlparser::keywords::Keyword::FUNCTION, self, context)?
                }
                sqlparser::keywords::Keyword::FUNCTIONS => {
                    transformer.transform(sqlparser::keywords::Keyword::FUNCTIONS, self, context)?
                }
                sqlparser::keywords::Keyword::FUSION => {
                    transformer.transform(sqlparser::keywords::Keyword::FUSION, self, context)?
                }
                sqlparser::keywords::Keyword::GENERAL => {
                    transformer.transform(sqlparser::keywords::Keyword::GENERAL, self, context)?
                }
                sqlparser::keywords::Keyword::GENERATE => {
                    transformer.transform(sqlparser::keywords::Keyword::GENERATE, self, context)?
                }
                sqlparser::keywords::Keyword::GENERATED => {
                    transformer.transform(sqlparser::keywords::Keyword::GENERATED, self, context)?
                }
                sqlparser::keywords::Keyword::GEOGRAPHY => {
                    transformer.transform(sqlparser::keywords::Keyword::GEOGRAPHY, self, context)?
                }
                sqlparser::keywords::Keyword::GET => {
                    transformer.transform(sqlparser::keywords::Keyword::GET, self, context)?
                }
                sqlparser::keywords::Keyword::GLOBAL => {
                    transformer.transform(sqlparser::keywords::Keyword::GLOBAL, self, context)?
                }
                sqlparser::keywords::Keyword::GRANT => {
                    transformer.transform(sqlparser::keywords::Keyword::GRANT, self, context)?
                }
                sqlparser::keywords::Keyword::GRANTED => {
                    transformer.transform(sqlparser::keywords::Keyword::GRANTED, self, context)?
                }
                sqlparser::keywords::Keyword::GRANTS => {
                    transformer.transform(sqlparser::keywords::Keyword::GRANTS, self, context)?
                }
                sqlparser::keywords::Keyword::GRAPHVIZ => {
                    transformer.transform(sqlparser::keywords::Keyword::GRAPHVIZ, self, context)?
                }
                sqlparser::keywords::Keyword::GROUP => {
                    transformer.transform(sqlparser::keywords::Keyword::GROUP, self, context)?
                }
                sqlparser::keywords::Keyword::GROUPING => {
                    transformer.transform(sqlparser::keywords::Keyword::GROUPING, self, context)?
                }
                sqlparser::keywords::Keyword::GROUPS => {
                    transformer.transform(sqlparser::keywords::Keyword::GROUPS, self, context)?
                }
                sqlparser::keywords::Keyword::HASH => {
                    transformer.transform(sqlparser::keywords::Keyword::HASH, self, context)?
                }
                sqlparser::keywords::Keyword::HAVING => {
                    transformer.transform(sqlparser::keywords::Keyword::HAVING, self, context)?
                }
                sqlparser::keywords::Keyword::HEADER => {
                    transformer.transform(sqlparser::keywords::Keyword::HEADER, self, context)?
                }
                sqlparser::keywords::Keyword::HEAP => {
                    transformer.transform(sqlparser::keywords::Keyword::HEAP, self, context)?
                }
                sqlparser::keywords::Keyword::HIGH_PRIORITY => transformer.transform(
                    sqlparser::keywords::Keyword::HIGH_PRIORITY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::HISTORY => {
                    transformer.transform(sqlparser::keywords::Keyword::HISTORY, self, context)?
                }
                sqlparser::keywords::Keyword::HIVEVAR => {
                    transformer.transform(sqlparser::keywords::Keyword::HIVEVAR, self, context)?
                }
                sqlparser::keywords::Keyword::HOLD => {
                    transformer.transform(sqlparser::keywords::Keyword::HOLD, self, context)?
                }
                sqlparser::keywords::Keyword::HOSTS => {
                    transformer.transform(sqlparser::keywords::Keyword::HOSTS, self, context)?
                }
                sqlparser::keywords::Keyword::HOUR => {
                    transformer.transform(sqlparser::keywords::Keyword::HOUR, self, context)?
                }
                sqlparser::keywords::Keyword::HOURS => {
                    transformer.transform(sqlparser::keywords::Keyword::HOURS, self, context)?
                }
                sqlparser::keywords::Keyword::ID => {
                    transformer.transform(sqlparser::keywords::Keyword::ID, self, context)?
                }
                sqlparser::keywords::Keyword::IDENTITY => {
                    transformer.transform(sqlparser::keywords::Keyword::IDENTITY, self, context)?
                }
                sqlparser::keywords::Keyword::IF => {
                    transformer.transform(sqlparser::keywords::Keyword::IF, self, context)?
                }
                sqlparser::keywords::Keyword::IGNORE => {
                    transformer.transform(sqlparser::keywords::Keyword::IGNORE, self, context)?
                }
                sqlparser::keywords::Keyword::ILIKE => {
                    transformer.transform(sqlparser::keywords::Keyword::ILIKE, self, context)?
                }
                sqlparser::keywords::Keyword::IMMEDIATE => {
                    transformer.transform(sqlparser::keywords::Keyword::IMMEDIATE, self, context)?
                }
                sqlparser::keywords::Keyword::IMMUTABLE => {
                    transformer.transform(sqlparser::keywords::Keyword::IMMUTABLE, self, context)?
                }
                sqlparser::keywords::Keyword::IN => {
                    transformer.transform(sqlparser::keywords::Keyword::IN, self, context)?
                }
                sqlparser::keywords::Keyword::INCLUDE => {
                    transformer.transform(sqlparser::keywords::Keyword::INCLUDE, self, context)?
                }
                sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES => transformer.transform(
                    sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::INCREMENT => {
                    transformer.transform(sqlparser::keywords::Keyword::INCREMENT, self, context)?
                }
                sqlparser::keywords::Keyword::INDEX => {
                    transformer.transform(sqlparser::keywords::Keyword::INDEX, self, context)?
                }
                sqlparser::keywords::Keyword::INDICATOR => {
                    transformer.transform(sqlparser::keywords::Keyword::INDICATOR, self, context)?
                }
                sqlparser::keywords::Keyword::INHERIT => {
                    transformer.transform(sqlparser::keywords::Keyword::INHERIT, self, context)?
                }
                sqlparser::keywords::Keyword::INITIALLY => {
                    transformer.transform(sqlparser::keywords::Keyword::INITIALLY, self, context)?
                }
                sqlparser::keywords::Keyword::INNER => {
                    transformer.transform(sqlparser::keywords::Keyword::INNER, self, context)?
                }
                sqlparser::keywords::Keyword::INOUT => {
                    transformer.transform(sqlparser::keywords::Keyword::INOUT, self, context)?
                }
                sqlparser::keywords::Keyword::INPUT => {
                    transformer.transform(sqlparser::keywords::Keyword::INPUT, self, context)?
                }
                sqlparser::keywords::Keyword::INPUTFORMAT => transformer.transform(
                    sqlparser::keywords::Keyword::INPUTFORMAT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::INSENSITIVE => transformer.transform(
                    sqlparser::keywords::Keyword::INSENSITIVE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::INSERT => {
                    transformer.transform(sqlparser::keywords::Keyword::INSERT, self, context)?
                }
                sqlparser::keywords::Keyword::INSTALL => {
                    transformer.transform(sqlparser::keywords::Keyword::INSTALL, self, context)?
                }
                sqlparser::keywords::Keyword::INSTEAD => {
                    transformer.transform(sqlparser::keywords::Keyword::INSTEAD, self, context)?
                }
                sqlparser::keywords::Keyword::INT => {
                    transformer.transform(sqlparser::keywords::Keyword::INT, self, context)?
                }
                sqlparser::keywords::Keyword::INT128 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT128, self, context)?
                }
                sqlparser::keywords::Keyword::INT16 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT16, self, context)?
                }
                sqlparser::keywords::Keyword::INT2 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT2, self, context)?
                }
                sqlparser::keywords::Keyword::INT256 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT256, self, context)?
                }
                sqlparser::keywords::Keyword::INT32 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT32, self, context)?
                }
                sqlparser::keywords::Keyword::INT4 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT4, self, context)?
                }
                sqlparser::keywords::Keyword::INT64 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT64, self, context)?
                }
                sqlparser::keywords::Keyword::INT8 => {
                    transformer.transform(sqlparser::keywords::Keyword::INT8, self, context)?
                }
                sqlparser::keywords::Keyword::INTEGER => {
                    transformer.transform(sqlparser::keywords::Keyword::INTEGER, self, context)?
                }
                sqlparser::keywords::Keyword::INTERPOLATE => transformer.transform(
                    sqlparser::keywords::Keyword::INTERPOLATE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::INTERSECT => {
                    transformer.transform(sqlparser::keywords::Keyword::INTERSECT, self, context)?
                }
                sqlparser::keywords::Keyword::INTERSECTION => transformer.transform(
                    sqlparser::keywords::Keyword::INTERSECTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::INTERVAL => {
                    transformer.transform(sqlparser::keywords::Keyword::INTERVAL, self, context)?
                }
                sqlparser::keywords::Keyword::INTO => {
                    transformer.transform(sqlparser::keywords::Keyword::INTO, self, context)?
                }
                sqlparser::keywords::Keyword::IS => {
                    transformer.transform(sqlparser::keywords::Keyword::IS, self, context)?
                }
                sqlparser::keywords::Keyword::ISODOW => {
                    transformer.transform(sqlparser::keywords::Keyword::ISODOW, self, context)?
                }
                sqlparser::keywords::Keyword::ISOLATION => {
                    transformer.transform(sqlparser::keywords::Keyword::ISOLATION, self, context)?
                }
                sqlparser::keywords::Keyword::ISOWEEK => {
                    transformer.transform(sqlparser::keywords::Keyword::ISOWEEK, self, context)?
                }
                sqlparser::keywords::Keyword::ISOYEAR => {
                    transformer.transform(sqlparser::keywords::Keyword::ISOYEAR, self, context)?
                }
                sqlparser::keywords::Keyword::ITEMS => {
                    transformer.transform(sqlparser::keywords::Keyword::ITEMS, self, context)?
                }
                sqlparser::keywords::Keyword::JAR => {
                    transformer.transform(sqlparser::keywords::Keyword::JAR, self, context)?
                }
                sqlparser::keywords::Keyword::JOIN => {
                    transformer.transform(sqlparser::keywords::Keyword::JOIN, self, context)?
                }
                sqlparser::keywords::Keyword::JSON => {
                    transformer.transform(sqlparser::keywords::Keyword::JSON, self, context)?
                }
                sqlparser::keywords::Keyword::JSONB => {
                    transformer.transform(sqlparser::keywords::Keyword::JSONB, self, context)?
                }
                sqlparser::keywords::Keyword::JSONFILE => {
                    transformer.transform(sqlparser::keywords::Keyword::JSONFILE, self, context)?
                }
                sqlparser::keywords::Keyword::JSON_TABLE => transformer.transform(
                    sqlparser::keywords::Keyword::JSON_TABLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::JULIAN => {
                    transformer.transform(sqlparser::keywords::Keyword::JULIAN, self, context)?
                }
                sqlparser::keywords::Keyword::KEY => {
                    transformer.transform(sqlparser::keywords::Keyword::KEY, self, context)?
                }
                sqlparser::keywords::Keyword::KEYS => {
                    transformer.transform(sqlparser::keywords::Keyword::KEYS, self, context)?
                }
                sqlparser::keywords::Keyword::KILL => {
                    transformer.transform(sqlparser::keywords::Keyword::KILL, self, context)?
                }
                sqlparser::keywords::Keyword::LAG => {
                    transformer.transform(sqlparser::keywords::Keyword::LAG, self, context)?
                }
                sqlparser::keywords::Keyword::LANGUAGE => {
                    transformer.transform(sqlparser::keywords::Keyword::LANGUAGE, self, context)?
                }
                sqlparser::keywords::Keyword::LARGE => {
                    transformer.transform(sqlparser::keywords::Keyword::LARGE, self, context)?
                }
                sqlparser::keywords::Keyword::LAST => {
                    transformer.transform(sqlparser::keywords::Keyword::LAST, self, context)?
                }
                sqlparser::keywords::Keyword::LAST_VALUE => transformer.transform(
                    sqlparser::keywords::Keyword::LAST_VALUE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::LATERAL => {
                    transformer.transform(sqlparser::keywords::Keyword::LATERAL, self, context)?
                }
                sqlparser::keywords::Keyword::LEAD => {
                    transformer.transform(sqlparser::keywords::Keyword::LEAD, self, context)?
                }
                sqlparser::keywords::Keyword::LEADING => {
                    transformer.transform(sqlparser::keywords::Keyword::LEADING, self, context)?
                }
                sqlparser::keywords::Keyword::LEFT => {
                    transformer.transform(sqlparser::keywords::Keyword::LEFT, self, context)?
                }
                sqlparser::keywords::Keyword::LEVEL => {
                    transformer.transform(sqlparser::keywords::Keyword::LEVEL, self, context)?
                }
                sqlparser::keywords::Keyword::LIKE => {
                    transformer.transform(sqlparser::keywords::Keyword::LIKE, self, context)?
                }
                sqlparser::keywords::Keyword::LIKE_REGEX => transformer.transform(
                    sqlparser::keywords::Keyword::LIKE_REGEX,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::LIMIT => {
                    transformer.transform(sqlparser::keywords::Keyword::LIMIT, self, context)?
                }
                sqlparser::keywords::Keyword::LINES => {
                    transformer.transform(sqlparser::keywords::Keyword::LINES, self, context)?
                }
                sqlparser::keywords::Keyword::LISTEN => {
                    transformer.transform(sqlparser::keywords::Keyword::LISTEN, self, context)?
                }
                sqlparser::keywords::Keyword::LN => {
                    transformer.transform(sqlparser::keywords::Keyword::LN, self, context)?
                }
                sqlparser::keywords::Keyword::LOAD => {
                    transformer.transform(sqlparser::keywords::Keyword::LOAD, self, context)?
                }
                sqlparser::keywords::Keyword::LOCAL => {
                    transformer.transform(sqlparser::keywords::Keyword::LOCAL, self, context)?
                }
                sqlparser::keywords::Keyword::LOCALTIME => {
                    transformer.transform(sqlparser::keywords::Keyword::LOCALTIME, self, context)?
                }
                sqlparser::keywords::Keyword::LOCALTIMESTAMP => transformer.transform(
                    sqlparser::keywords::Keyword::LOCALTIMESTAMP,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::LOCATION => {
                    transformer.transform(sqlparser::keywords::Keyword::LOCATION, self, context)?
                }
                sqlparser::keywords::Keyword::LOCK => {
                    transformer.transform(sqlparser::keywords::Keyword::LOCK, self, context)?
                }
                sqlparser::keywords::Keyword::LOCKED => {
                    transformer.transform(sqlparser::keywords::Keyword::LOCKED, self, context)?
                }
                sqlparser::keywords::Keyword::LOGIN => {
                    transformer.transform(sqlparser::keywords::Keyword::LOGIN, self, context)?
                }
                sqlparser::keywords::Keyword::LOGS => {
                    transformer.transform(sqlparser::keywords::Keyword::LOGS, self, context)?
                }
                sqlparser::keywords::Keyword::LOWCARDINALITY => transformer.transform(
                    sqlparser::keywords::Keyword::LOWCARDINALITY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::LOWER => {
                    transformer.transform(sqlparser::keywords::Keyword::LOWER, self, context)?
                }
                sqlparser::keywords::Keyword::LOW_PRIORITY => transformer.transform(
                    sqlparser::keywords::Keyword::LOW_PRIORITY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MACRO => {
                    transformer.transform(sqlparser::keywords::Keyword::MACRO, self, context)?
                }
                sqlparser::keywords::Keyword::MANAGEDLOCATION => transformer.transform(
                    sqlparser::keywords::Keyword::MANAGEDLOCATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MAP => {
                    transformer.transform(sqlparser::keywords::Keyword::MAP, self, context)?
                }
                sqlparser::keywords::Keyword::MASKING => {
                    transformer.transform(sqlparser::keywords::Keyword::MASKING, self, context)?
                }
                sqlparser::keywords::Keyword::MATCH => {
                    transformer.transform(sqlparser::keywords::Keyword::MATCH, self, context)?
                }
                sqlparser::keywords::Keyword::MATCHED => {
                    transformer.transform(sqlparser::keywords::Keyword::MATCHED, self, context)?
                }
                sqlparser::keywords::Keyword::MATCHES => {
                    transformer.transform(sqlparser::keywords::Keyword::MATCHES, self, context)?
                }
                sqlparser::keywords::Keyword::MATCH_CONDITION => transformer.transform(
                    sqlparser::keywords::Keyword::MATCH_CONDITION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MATCH_RECOGNIZE => transformer.transform(
                    sqlparser::keywords::Keyword::MATCH_RECOGNIZE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MATERIALIZE => transformer.transform(
                    sqlparser::keywords::Keyword::MATERIALIZE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MATERIALIZED => transformer.transform(
                    sqlparser::keywords::Keyword::MATERIALIZED,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MAX => {
                    transformer.transform(sqlparser::keywords::Keyword::MAX, self, context)?
                }
                sqlparser::keywords::Keyword::MAXVALUE => {
                    transformer.transform(sqlparser::keywords::Keyword::MAXVALUE, self, context)?
                }
                sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS => transformer
                    .transform(
                        sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS,
                        self,
                        context,
                    )?,
                sqlparser::keywords::Keyword::MEASURES => {
                    transformer.transform(sqlparser::keywords::Keyword::MEASURES, self, context)?
                }
                sqlparser::keywords::Keyword::MEDIUMINT => {
                    transformer.transform(sqlparser::keywords::Keyword::MEDIUMINT, self, context)?
                }
                sqlparser::keywords::Keyword::MEMBER => {
                    transformer.transform(sqlparser::keywords::Keyword::MEMBER, self, context)?
                }
                sqlparser::keywords::Keyword::MERGE => {
                    transformer.transform(sqlparser::keywords::Keyword::MERGE, self, context)?
                }
                sqlparser::keywords::Keyword::METADATA => {
                    transformer.transform(sqlparser::keywords::Keyword::METADATA, self, context)?
                }
                sqlparser::keywords::Keyword::METHOD => {
                    transformer.transform(sqlparser::keywords::Keyword::METHOD, self, context)?
                }
                sqlparser::keywords::Keyword::MICROSECOND => transformer.transform(
                    sqlparser::keywords::Keyword::MICROSECOND,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MICROSECONDS => transformer.transform(
                    sqlparser::keywords::Keyword::MICROSECONDS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MILLENIUM => {
                    transformer.transform(sqlparser::keywords::Keyword::MILLENIUM, self, context)?
                }
                sqlparser::keywords::Keyword::MILLENNIUM => transformer.transform(
                    sqlparser::keywords::Keyword::MILLENNIUM,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MILLISECOND => transformer.transform(
                    sqlparser::keywords::Keyword::MILLISECOND,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MILLISECONDS => transformer.transform(
                    sqlparser::keywords::Keyword::MILLISECONDS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::MIN => {
                    transformer.transform(sqlparser::keywords::Keyword::MIN, self, context)?
                }
                sqlparser::keywords::Keyword::MINUTE => {
                    transformer.transform(sqlparser::keywords::Keyword::MINUTE, self, context)?
                }
                sqlparser::keywords::Keyword::MINVALUE => {
                    transformer.transform(sqlparser::keywords::Keyword::MINVALUE, self, context)?
                }
                sqlparser::keywords::Keyword::MOD => {
                    transformer.transform(sqlparser::keywords::Keyword::MOD, self, context)?
                }
                sqlparser::keywords::Keyword::MODE => {
                    transformer.transform(sqlparser::keywords::Keyword::MODE, self, context)?
                }
                sqlparser::keywords::Keyword::MODIFIES => {
                    transformer.transform(sqlparser::keywords::Keyword::MODIFIES, self, context)?
                }
                sqlparser::keywords::Keyword::MODIFY => {
                    transformer.transform(sqlparser::keywords::Keyword::MODIFY, self, context)?
                }
                sqlparser::keywords::Keyword::MODULE => {
                    transformer.transform(sqlparser::keywords::Keyword::MODULE, self, context)?
                }
                sqlparser::keywords::Keyword::MONTH => {
                    transformer.transform(sqlparser::keywords::Keyword::MONTH, self, context)?
                }
                sqlparser::keywords::Keyword::MSCK => {
                    transformer.transform(sqlparser::keywords::Keyword::MSCK, self, context)?
                }
                sqlparser::keywords::Keyword::MULTISET => {
                    transformer.transform(sqlparser::keywords::Keyword::MULTISET, self, context)?
                }
                sqlparser::keywords::Keyword::MUTATION => {
                    transformer.transform(sqlparser::keywords::Keyword::MUTATION, self, context)?
                }
                sqlparser::keywords::Keyword::NAME => {
                    transformer.transform(sqlparser::keywords::Keyword::NAME, self, context)?
                }
                sqlparser::keywords::Keyword::NANOSECOND => transformer.transform(
                    sqlparser::keywords::Keyword::NANOSECOND,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NANOSECONDS => transformer.transform(
                    sqlparser::keywords::Keyword::NANOSECONDS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NATIONAL => {
                    transformer.transform(sqlparser::keywords::Keyword::NATIONAL, self, context)?
                }
                sqlparser::keywords::Keyword::NATURAL => {
                    transformer.transform(sqlparser::keywords::Keyword::NATURAL, self, context)?
                }
                sqlparser::keywords::Keyword::NCHAR => {
                    transformer.transform(sqlparser::keywords::Keyword::NCHAR, self, context)?
                }
                sqlparser::keywords::Keyword::NCLOB => {
                    transformer.transform(sqlparser::keywords::Keyword::NCLOB, self, context)?
                }
                sqlparser::keywords::Keyword::NESTED => {
                    transformer.transform(sqlparser::keywords::Keyword::NESTED, self, context)?
                }
                sqlparser::keywords::Keyword::NEW => {
                    transformer.transform(sqlparser::keywords::Keyword::NEW, self, context)?
                }
                sqlparser::keywords::Keyword::NEXT => {
                    transformer.transform(sqlparser::keywords::Keyword::NEXT, self, context)?
                }
                sqlparser::keywords::Keyword::NO => {
                    transformer.transform(sqlparser::keywords::Keyword::NO, self, context)?
                }
                sqlparser::keywords::Keyword::NOBYPASSRLS => transformer.transform(
                    sqlparser::keywords::Keyword::NOBYPASSRLS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NOCREATEDB => transformer.transform(
                    sqlparser::keywords::Keyword::NOCREATEDB,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NOCREATEROLE => transformer.transform(
                    sqlparser::keywords::Keyword::NOCREATEROLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NOINHERIT => {
                    transformer.transform(sqlparser::keywords::Keyword::NOINHERIT, self, context)?
                }
                sqlparser::keywords::Keyword::NOLOGIN => {
                    transformer.transform(sqlparser::keywords::Keyword::NOLOGIN, self, context)?
                }
                sqlparser::keywords::Keyword::NONE => {
                    transformer.transform(sqlparser::keywords::Keyword::NONE, self, context)?
                }
                sqlparser::keywords::Keyword::NOORDER => {
                    transformer.transform(sqlparser::keywords::Keyword::NOORDER, self, context)?
                }
                sqlparser::keywords::Keyword::NOREPLICATION => transformer.transform(
                    sqlparser::keywords::Keyword::NOREPLICATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NORMALIZE => {
                    transformer.transform(sqlparser::keywords::Keyword::NORMALIZE, self, context)?
                }
                sqlparser::keywords::Keyword::NOSCAN => {
                    transformer.transform(sqlparser::keywords::Keyword::NOSCAN, self, context)?
                }
                sqlparser::keywords::Keyword::NOSUPERUSER => transformer.transform(
                    sqlparser::keywords::Keyword::NOSUPERUSER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NOT => {
                    transformer.transform(sqlparser::keywords::Keyword::NOT, self, context)?
                }
                sqlparser::keywords::Keyword::NOTHING => {
                    transformer.transform(sqlparser::keywords::Keyword::NOTHING, self, context)?
                }
                sqlparser::keywords::Keyword::NOTIFY => {
                    transformer.transform(sqlparser::keywords::Keyword::NOTIFY, self, context)?
                }
                sqlparser::keywords::Keyword::NOWAIT => {
                    transformer.transform(sqlparser::keywords::Keyword::NOWAIT, self, context)?
                }
                sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG => transformer.transform(
                    sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::NTH_VALUE => {
                    transformer.transform(sqlparser::keywords::Keyword::NTH_VALUE, self, context)?
                }
                sqlparser::keywords::Keyword::NTILE => {
                    transformer.transform(sqlparser::keywords::Keyword::NTILE, self, context)?
                }
                sqlparser::keywords::Keyword::NULL => {
                    transformer.transform(sqlparser::keywords::Keyword::NULL, self, context)?
                }
                sqlparser::keywords::Keyword::NULLABLE => {
                    transformer.transform(sqlparser::keywords::Keyword::NULLABLE, self, context)?
                }
                sqlparser::keywords::Keyword::NULLIF => {
                    transformer.transform(sqlparser::keywords::Keyword::NULLIF, self, context)?
                }
                sqlparser::keywords::Keyword::NULLS => {
                    transformer.transform(sqlparser::keywords::Keyword::NULLS, self, context)?
                }
                sqlparser::keywords::Keyword::NUMERIC => {
                    transformer.transform(sqlparser::keywords::Keyword::NUMERIC, self, context)?
                }
                sqlparser::keywords::Keyword::NVARCHAR => {
                    transformer.transform(sqlparser::keywords::Keyword::NVARCHAR, self, context)?
                }
                sqlparser::keywords::Keyword::OBJECT => {
                    transformer.transform(sqlparser::keywords::Keyword::OBJECT, self, context)?
                }
                sqlparser::keywords::Keyword::OCCURRENCES_REGEX => transformer.transform(
                    sqlparser::keywords::Keyword::OCCURRENCES_REGEX,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::OCTETS => {
                    transformer.transform(sqlparser::keywords::Keyword::OCTETS, self, context)?
                }
                sqlparser::keywords::Keyword::OCTET_LENGTH => transformer.transform(
                    sqlparser::keywords::Keyword::OCTET_LENGTH,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::OF => {
                    transformer.transform(sqlparser::keywords::Keyword::OF, self, context)?
                }
                sqlparser::keywords::Keyword::OFFSET => {
                    transformer.transform(sqlparser::keywords::Keyword::OFFSET, self, context)?
                }
                sqlparser::keywords::Keyword::OLD => {
                    transformer.transform(sqlparser::keywords::Keyword::OLD, self, context)?
                }
                sqlparser::keywords::Keyword::OMIT => {
                    transformer.transform(sqlparser::keywords::Keyword::OMIT, self, context)?
                }
                sqlparser::keywords::Keyword::ON => {
                    transformer.transform(sqlparser::keywords::Keyword::ON, self, context)?
                }
                sqlparser::keywords::Keyword::ONE => {
                    transformer.transform(sqlparser::keywords::Keyword::ONE, self, context)?
                }
                sqlparser::keywords::Keyword::ONLY => {
                    transformer.transform(sqlparser::keywords::Keyword::ONLY, self, context)?
                }
                sqlparser::keywords::Keyword::OPEN => {
                    transformer.transform(sqlparser::keywords::Keyword::OPEN, self, context)?
                }
                sqlparser::keywords::Keyword::OPERATOR => {
                    transformer.transform(sqlparser::keywords::Keyword::OPERATOR, self, context)?
                }
                sqlparser::keywords::Keyword::OPTIMIZE => {
                    transformer.transform(sqlparser::keywords::Keyword::OPTIMIZE, self, context)?
                }
                sqlparser::keywords::Keyword::OPTIMIZER_COSTS => transformer.transform(
                    sqlparser::keywords::Keyword::OPTIMIZER_COSTS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::OPTION => {
                    transformer.transform(sqlparser::keywords::Keyword::OPTION, self, context)?
                }
                sqlparser::keywords::Keyword::OPTIONS => {
                    transformer.transform(sqlparser::keywords::Keyword::OPTIONS, self, context)?
                }
                sqlparser::keywords::Keyword::OR => {
                    transformer.transform(sqlparser::keywords::Keyword::OR, self, context)?
                }
                sqlparser::keywords::Keyword::ORC => {
                    transformer.transform(sqlparser::keywords::Keyword::ORC, self, context)?
                }
                sqlparser::keywords::Keyword::ORDER => {
                    transformer.transform(sqlparser::keywords::Keyword::ORDER, self, context)?
                }
                sqlparser::keywords::Keyword::ORDINALITY => transformer.transform(
                    sqlparser::keywords::Keyword::ORDINALITY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::OUT => {
                    transformer.transform(sqlparser::keywords::Keyword::OUT, self, context)?
                }
                sqlparser::keywords::Keyword::OUTER => {
                    transformer.transform(sqlparser::keywords::Keyword::OUTER, self, context)?
                }
                sqlparser::keywords::Keyword::OUTPUTFORMAT => transformer.transform(
                    sqlparser::keywords::Keyword::OUTPUTFORMAT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::OVER => {
                    transformer.transform(sqlparser::keywords::Keyword::OVER, self, context)?
                }
                sqlparser::keywords::Keyword::OVERFLOW => {
                    transformer.transform(sqlparser::keywords::Keyword::OVERFLOW, self, context)?
                }
                sqlparser::keywords::Keyword::OVERLAPS => {
                    transformer.transform(sqlparser::keywords::Keyword::OVERLAPS, self, context)?
                }
                sqlparser::keywords::Keyword::OVERLAY => {
                    transformer.transform(sqlparser::keywords::Keyword::OVERLAY, self, context)?
                }
                sqlparser::keywords::Keyword::OVERWRITE => {
                    transformer.transform(sqlparser::keywords::Keyword::OVERWRITE, self, context)?
                }
                sqlparser::keywords::Keyword::OWNED => {
                    transformer.transform(sqlparser::keywords::Keyword::OWNED, self, context)?
                }
                sqlparser::keywords::Keyword::OWNER => {
                    transformer.transform(sqlparser::keywords::Keyword::OWNER, self, context)?
                }
                sqlparser::keywords::Keyword::PARALLEL => {
                    transformer.transform(sqlparser::keywords::Keyword::PARALLEL, self, context)?
                }
                sqlparser::keywords::Keyword::PARAMETER => {
                    transformer.transform(sqlparser::keywords::Keyword::PARAMETER, self, context)?
                }
                sqlparser::keywords::Keyword::PARQUET => {
                    transformer.transform(sqlparser::keywords::Keyword::PARQUET, self, context)?
                }
                sqlparser::keywords::Keyword::PART => {
                    transformer.transform(sqlparser::keywords::Keyword::PART, self, context)?
                }
                sqlparser::keywords::Keyword::PARTITION => {
                    transformer.transform(sqlparser::keywords::Keyword::PARTITION, self, context)?
                }
                sqlparser::keywords::Keyword::PARTITIONED => transformer.transform(
                    sqlparser::keywords::Keyword::PARTITIONED,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PARTITIONS => transformer.transform(
                    sqlparser::keywords::Keyword::PARTITIONS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PASSWORD => {
                    transformer.transform(sqlparser::keywords::Keyword::PASSWORD, self, context)?
                }
                sqlparser::keywords::Keyword::PAST => {
                    transformer.transform(sqlparser::keywords::Keyword::PAST, self, context)?
                }
                sqlparser::keywords::Keyword::PATH => {
                    transformer.transform(sqlparser::keywords::Keyword::PATH, self, context)?
                }
                sqlparser::keywords::Keyword::PATTERN => {
                    transformer.transform(sqlparser::keywords::Keyword::PATTERN, self, context)?
                }
                sqlparser::keywords::Keyword::PER => {
                    transformer.transform(sqlparser::keywords::Keyword::PER, self, context)?
                }
                sqlparser::keywords::Keyword::PERCENT => {
                    transformer.transform(sqlparser::keywords::Keyword::PERCENT, self, context)?
                }
                sqlparser::keywords::Keyword::PERCENTILE_CONT => transformer.transform(
                    sqlparser::keywords::Keyword::PERCENTILE_CONT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PERCENTILE_DISC => transformer.transform(
                    sqlparser::keywords::Keyword::PERCENTILE_DISC,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PERCENT_RANK => transformer.transform(
                    sqlparser::keywords::Keyword::PERCENT_RANK,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PERIOD => {
                    transformer.transform(sqlparser::keywords::Keyword::PERIOD, self, context)?
                }
                sqlparser::keywords::Keyword::PERMISSIVE => transformer.transform(
                    sqlparser::keywords::Keyword::PERMISSIVE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PERSISTENT => transformer.transform(
                    sqlparser::keywords::Keyword::PERSISTENT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PIVOT => {
                    transformer.transform(sqlparser::keywords::Keyword::PIVOT, self, context)?
                }
                sqlparser::keywords::Keyword::PLACING => {
                    transformer.transform(sqlparser::keywords::Keyword::PLACING, self, context)?
                }
                sqlparser::keywords::Keyword::PLAN => {
                    transformer.transform(sqlparser::keywords::Keyword::PLAN, self, context)?
                }
                sqlparser::keywords::Keyword::PLANS => {
                    transformer.transform(sqlparser::keywords::Keyword::PLANS, self, context)?
                }
                sqlparser::keywords::Keyword::POLICY => {
                    transformer.transform(sqlparser::keywords::Keyword::POLICY, self, context)?
                }
                sqlparser::keywords::Keyword::PORTION => {
                    transformer.transform(sqlparser::keywords::Keyword::PORTION, self, context)?
                }
                sqlparser::keywords::Keyword::POSITION => {
                    transformer.transform(sqlparser::keywords::Keyword::POSITION, self, context)?
                }
                sqlparser::keywords::Keyword::POSITION_REGEX => transformer.transform(
                    sqlparser::keywords::Keyword::POSITION_REGEX,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::POWER => {
                    transformer.transform(sqlparser::keywords::Keyword::POWER, self, context)?
                }
                sqlparser::keywords::Keyword::PRAGMA => {
                    transformer.transform(sqlparser::keywords::Keyword::PRAGMA, self, context)?
                }
                sqlparser::keywords::Keyword::PRECEDES => {
                    transformer.transform(sqlparser::keywords::Keyword::PRECEDES, self, context)?
                }
                sqlparser::keywords::Keyword::PRECEDING => {
                    transformer.transform(sqlparser::keywords::Keyword::PRECEDING, self, context)?
                }
                sqlparser::keywords::Keyword::PRECISION => {
                    transformer.transform(sqlparser::keywords::Keyword::PRECISION, self, context)?
                }
                sqlparser::keywords::Keyword::PREPARE => {
                    transformer.transform(sqlparser::keywords::Keyword::PREPARE, self, context)?
                }
                sqlparser::keywords::Keyword::PRESERVE => {
                    transformer.transform(sqlparser::keywords::Keyword::PRESERVE, self, context)?
                }
                sqlparser::keywords::Keyword::PREWHERE => {
                    transformer.transform(sqlparser::keywords::Keyword::PREWHERE, self, context)?
                }
                sqlparser::keywords::Keyword::PRIMARY => {
                    transformer.transform(sqlparser::keywords::Keyword::PRIMARY, self, context)?
                }
                sqlparser::keywords::Keyword::PRIOR => {
                    transformer.transform(sqlparser::keywords::Keyword::PRIOR, self, context)?
                }
                sqlparser::keywords::Keyword::PRIVILEGES => transformer.transform(
                    sqlparser::keywords::Keyword::PRIVILEGES,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PROCEDURE => {
                    transformer.transform(sqlparser::keywords::Keyword::PROCEDURE, self, context)?
                }
                sqlparser::keywords::Keyword::PROGRAM => {
                    transformer.transform(sqlparser::keywords::Keyword::PROGRAM, self, context)?
                }
                sqlparser::keywords::Keyword::PROJECTION => transformer.transform(
                    sqlparser::keywords::Keyword::PROJECTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::PURGE => {
                    transformer.transform(sqlparser::keywords::Keyword::PURGE, self, context)?
                }
                sqlparser::keywords::Keyword::QUALIFY => {
                    transformer.transform(sqlparser::keywords::Keyword::QUALIFY, self, context)?
                }
                sqlparser::keywords::Keyword::QUARTER => {
                    transformer.transform(sqlparser::keywords::Keyword::QUARTER, self, context)?
                }
                sqlparser::keywords::Keyword::QUERY => {
                    transformer.transform(sqlparser::keywords::Keyword::QUERY, self, context)?
                }
                sqlparser::keywords::Keyword::QUOTE => {
                    transformer.transform(sqlparser::keywords::Keyword::QUOTE, self, context)?
                }
                sqlparser::keywords::Keyword::RANGE => {
                    transformer.transform(sqlparser::keywords::Keyword::RANGE, self, context)?
                }
                sqlparser::keywords::Keyword::RANK => {
                    transformer.transform(sqlparser::keywords::Keyword::RANK, self, context)?
                }
                sqlparser::keywords::Keyword::RAW => {
                    transformer.transform(sqlparser::keywords::Keyword::RAW, self, context)?
                }
                sqlparser::keywords::Keyword::RCFILE => {
                    transformer.transform(sqlparser::keywords::Keyword::RCFILE, self, context)?
                }
                sqlparser::keywords::Keyword::READ => {
                    transformer.transform(sqlparser::keywords::Keyword::READ, self, context)?
                }
                sqlparser::keywords::Keyword::READS => {
                    transformer.transform(sqlparser::keywords::Keyword::READS, self, context)?
                }
                sqlparser::keywords::Keyword::READ_ONLY => {
                    transformer.transform(sqlparser::keywords::Keyword::READ_ONLY, self, context)?
                }
                sqlparser::keywords::Keyword::REAL => {
                    transformer.transform(sqlparser::keywords::Keyword::REAL, self, context)?
                }
                sqlparser::keywords::Keyword::RECURSIVE => {
                    transformer.transform(sqlparser::keywords::Keyword::RECURSIVE, self, context)?
                }
                sqlparser::keywords::Keyword::REF => {
                    transformer.transform(sqlparser::keywords::Keyword::REF, self, context)?
                }
                sqlparser::keywords::Keyword::REFERENCES => transformer.transform(
                    sqlparser::keywords::Keyword::REFERENCES,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::REFERENCING => transformer.transform(
                    sqlparser::keywords::Keyword::REFERENCING,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::REGCLASS => {
                    transformer.transform(sqlparser::keywords::Keyword::REGCLASS, self, context)?
                }
                sqlparser::keywords::Keyword::REGEXP => {
                    transformer.transform(sqlparser::keywords::Keyword::REGEXP, self, context)?
                }
                sqlparser::keywords::Keyword::REGR_AVGX => {
                    transformer.transform(sqlparser::keywords::Keyword::REGR_AVGX, self, context)?
                }
                sqlparser::keywords::Keyword::REGR_AVGY => {
                    transformer.transform(sqlparser::keywords::Keyword::REGR_AVGY, self, context)?
                }
                sqlparser::keywords::Keyword::REGR_COUNT => transformer.transform(
                    sqlparser::keywords::Keyword::REGR_COUNT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::REGR_INTERCEPT => transformer.transform(
                    sqlparser::keywords::Keyword::REGR_INTERCEPT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::REGR_R2 => {
                    transformer.transform(sqlparser::keywords::Keyword::REGR_R2, self, context)?
                }
                sqlparser::keywords::Keyword::REGR_SLOPE => transformer.transform(
                    sqlparser::keywords::Keyword::REGR_SLOPE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::REGR_SXX => {
                    transformer.transform(sqlparser::keywords::Keyword::REGR_SXX, self, context)?
                }
                sqlparser::keywords::Keyword::REGR_SXY => {
                    transformer.transform(sqlparser::keywords::Keyword::REGR_SXY, self, context)?
                }
                sqlparser::keywords::Keyword::REGR_SYY => {
                    transformer.transform(sqlparser::keywords::Keyword::REGR_SYY, self, context)?
                }
                sqlparser::keywords::Keyword::RELATIVE => {
                    transformer.transform(sqlparser::keywords::Keyword::RELATIVE, self, context)?
                }
                sqlparser::keywords::Keyword::RELAY => {
                    transformer.transform(sqlparser::keywords::Keyword::RELAY, self, context)?
                }
                sqlparser::keywords::Keyword::RELEASE => {
                    transformer.transform(sqlparser::keywords::Keyword::RELEASE, self, context)?
                }
                sqlparser::keywords::Keyword::REMOTE => {
                    transformer.transform(sqlparser::keywords::Keyword::REMOTE, self, context)?
                }
                sqlparser::keywords::Keyword::RENAME => {
                    transformer.transform(sqlparser::keywords::Keyword::RENAME, self, context)?
                }
                sqlparser::keywords::Keyword::REORG => {
                    transformer.transform(sqlparser::keywords::Keyword::REORG, self, context)?
                }
                sqlparser::keywords::Keyword::REPAIR => {
                    transformer.transform(sqlparser::keywords::Keyword::REPAIR, self, context)?
                }
                sqlparser::keywords::Keyword::REPEATABLE => transformer.transform(
                    sqlparser::keywords::Keyword::REPEATABLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::REPLACE => {
                    transformer.transform(sqlparser::keywords::Keyword::REPLACE, self, context)?
                }
                sqlparser::keywords::Keyword::REPLICA => {
                    transformer.transform(sqlparser::keywords::Keyword::REPLICA, self, context)?
                }
                sqlparser::keywords::Keyword::REPLICATION => transformer.transform(
                    sqlparser::keywords::Keyword::REPLICATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::RESET => {
                    transformer.transform(sqlparser::keywords::Keyword::RESET, self, context)?
                }
                sqlparser::keywords::Keyword::RESPECT => {
                    transformer.transform(sqlparser::keywords::Keyword::RESPECT, self, context)?
                }
                sqlparser::keywords::Keyword::RESTART => {
                    transformer.transform(sqlparser::keywords::Keyword::RESTART, self, context)?
                }
                sqlparser::keywords::Keyword::RESTRICT => {
                    transformer.transform(sqlparser::keywords::Keyword::RESTRICT, self, context)?
                }
                sqlparser::keywords::Keyword::RESTRICTED => transformer.transform(
                    sqlparser::keywords::Keyword::RESTRICTED,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::RESTRICTIVE => transformer.transform(
                    sqlparser::keywords::Keyword::RESTRICTIVE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::RESULT => {
                    transformer.transform(sqlparser::keywords::Keyword::RESULT, self, context)?
                }
                sqlparser::keywords::Keyword::RESULTSET => {
                    transformer.transform(sqlparser::keywords::Keyword::RESULTSET, self, context)?
                }
                sqlparser::keywords::Keyword::RETAIN => {
                    transformer.transform(sqlparser::keywords::Keyword::RETAIN, self, context)?
                }
                sqlparser::keywords::Keyword::RETURN => {
                    transformer.transform(sqlparser::keywords::Keyword::RETURN, self, context)?
                }
                sqlparser::keywords::Keyword::RETURNING => {
                    transformer.transform(sqlparser::keywords::Keyword::RETURNING, self, context)?
                }
                sqlparser::keywords::Keyword::RETURNS => {
                    transformer.transform(sqlparser::keywords::Keyword::RETURNS, self, context)?
                }
                sqlparser::keywords::Keyword::REVOKE => {
                    transformer.transform(sqlparser::keywords::Keyword::REVOKE, self, context)?
                }
                sqlparser::keywords::Keyword::RIGHT => {
                    transformer.transform(sqlparser::keywords::Keyword::RIGHT, self, context)?
                }
                sqlparser::keywords::Keyword::RLIKE => {
                    transformer.transform(sqlparser::keywords::Keyword::RLIKE, self, context)?
                }
                sqlparser::keywords::Keyword::ROLE => {
                    transformer.transform(sqlparser::keywords::Keyword::ROLE, self, context)?
                }
                sqlparser::keywords::Keyword::ROLLBACK => {
                    transformer.transform(sqlparser::keywords::Keyword::ROLLBACK, self, context)?
                }
                sqlparser::keywords::Keyword::ROLLUP => {
                    transformer.transform(sqlparser::keywords::Keyword::ROLLUP, self, context)?
                }
                sqlparser::keywords::Keyword::ROOT => {
                    transformer.transform(sqlparser::keywords::Keyword::ROOT, self, context)?
                }
                sqlparser::keywords::Keyword::ROW => {
                    transformer.transform(sqlparser::keywords::Keyword::ROW, self, context)?
                }
                sqlparser::keywords::Keyword::ROWID => {
                    transformer.transform(sqlparser::keywords::Keyword::ROWID, self, context)?
                }
                sqlparser::keywords::Keyword::ROWS => {
                    transformer.transform(sqlparser::keywords::Keyword::ROWS, self, context)?
                }
                sqlparser::keywords::Keyword::ROW_NUMBER => transformer.transform(
                    sqlparser::keywords::Keyword::ROW_NUMBER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::RULE => {
                    transformer.transform(sqlparser::keywords::Keyword::RULE, self, context)?
                }
                sqlparser::keywords::Keyword::RUN => {
                    transformer.transform(sqlparser::keywords::Keyword::RUN, self, context)?
                }
                sqlparser::keywords::Keyword::SAFE => {
                    transformer.transform(sqlparser::keywords::Keyword::SAFE, self, context)?
                }
                sqlparser::keywords::Keyword::SAFE_CAST => {
                    transformer.transform(sqlparser::keywords::Keyword::SAFE_CAST, self, context)?
                }
                sqlparser::keywords::Keyword::SAVEPOINT => {
                    transformer.transform(sqlparser::keywords::Keyword::SAVEPOINT, self, context)?
                }
                sqlparser::keywords::Keyword::SCHEMA => {
                    transformer.transform(sqlparser::keywords::Keyword::SCHEMA, self, context)?
                }
                sqlparser::keywords::Keyword::SCHEMAS => {
                    transformer.transform(sqlparser::keywords::Keyword::SCHEMAS, self, context)?
                }
                sqlparser::keywords::Keyword::SCOPE => {
                    transformer.transform(sqlparser::keywords::Keyword::SCOPE, self, context)?
                }
                sqlparser::keywords::Keyword::SCROLL => {
                    transformer.transform(sqlparser::keywords::Keyword::SCROLL, self, context)?
                }
                sqlparser::keywords::Keyword::SEARCH => {
                    transformer.transform(sqlparser::keywords::Keyword::SEARCH, self, context)?
                }
                sqlparser::keywords::Keyword::SECOND => {
                    transformer.transform(sqlparser::keywords::Keyword::SECOND, self, context)?
                }
                sqlparser::keywords::Keyword::SECRET => {
                    transformer.transform(sqlparser::keywords::Keyword::SECRET, self, context)?
                }
                sqlparser::keywords::Keyword::SECURITY => {
                    transformer.transform(sqlparser::keywords::Keyword::SECURITY, self, context)?
                }
                sqlparser::keywords::Keyword::SELECT => {
                    transformer.transform(sqlparser::keywords::Keyword::SELECT, self, context)?
                }
                sqlparser::keywords::Keyword::SEMI => {
                    transformer.transform(sqlparser::keywords::Keyword::SEMI, self, context)?
                }
                sqlparser::keywords::Keyword::SENSITIVE => {
                    transformer.transform(sqlparser::keywords::Keyword::SENSITIVE, self, context)?
                }
                sqlparser::keywords::Keyword::SEPARATOR => {
                    transformer.transform(sqlparser::keywords::Keyword::SEPARATOR, self, context)?
                }
                sqlparser::keywords::Keyword::SEQUENCE => {
                    transformer.transform(sqlparser::keywords::Keyword::SEQUENCE, self, context)?
                }
                sqlparser::keywords::Keyword::SEQUENCEFILE => transformer.transform(
                    sqlparser::keywords::Keyword::SEQUENCEFILE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SEQUENCES => {
                    transformer.transform(sqlparser::keywords::Keyword::SEQUENCES, self, context)?
                }
                sqlparser::keywords::Keyword::SERDE => {
                    transformer.transform(sqlparser::keywords::Keyword::SERDE, self, context)?
                }
                sqlparser::keywords::Keyword::SERDEPROPERTIES => transformer.transform(
                    sqlparser::keywords::Keyword::SERDEPROPERTIES,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SERIALIZABLE => transformer.transform(
                    sqlparser::keywords::Keyword::SERIALIZABLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SESSION => {
                    transformer.transform(sqlparser::keywords::Keyword::SESSION, self, context)?
                }
                sqlparser::keywords::Keyword::SESSION_USER => transformer.transform(
                    sqlparser::keywords::Keyword::SESSION_USER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SET => {
                    transformer.transform(sqlparser::keywords::Keyword::SET, self, context)?
                }
                sqlparser::keywords::Keyword::SETS => {
                    transformer.transform(sqlparser::keywords::Keyword::SETS, self, context)?
                }
                sqlparser::keywords::Keyword::SETTINGS => {
                    transformer.transform(sqlparser::keywords::Keyword::SETTINGS, self, context)?
                }
                sqlparser::keywords::Keyword::SHARE => {
                    transformer.transform(sqlparser::keywords::Keyword::SHARE, self, context)?
                }
                sqlparser::keywords::Keyword::SHOW => {
                    transformer.transform(sqlparser::keywords::Keyword::SHOW, self, context)?
                }
                sqlparser::keywords::Keyword::SIMILAR => {
                    transformer.transform(sqlparser::keywords::Keyword::SIMILAR, self, context)?
                }
                sqlparser::keywords::Keyword::SKIP => {
                    transformer.transform(sqlparser::keywords::Keyword::SKIP, self, context)?
                }
                sqlparser::keywords::Keyword::SLOW => {
                    transformer.transform(sqlparser::keywords::Keyword::SLOW, self, context)?
                }
                sqlparser::keywords::Keyword::SMALLINT => {
                    transformer.transform(sqlparser::keywords::Keyword::SMALLINT, self, context)?
                }
                sqlparser::keywords::Keyword::SNAPSHOT => {
                    transformer.transform(sqlparser::keywords::Keyword::SNAPSHOT, self, context)?
                }
                sqlparser::keywords::Keyword::SOME => {
                    transformer.transform(sqlparser::keywords::Keyword::SOME, self, context)?
                }
                sqlparser::keywords::Keyword::SORT => {
                    transformer.transform(sqlparser::keywords::Keyword::SORT, self, context)?
                }
                sqlparser::keywords::Keyword::SORTED => {
                    transformer.transform(sqlparser::keywords::Keyword::SORTED, self, context)?
                }
                sqlparser::keywords::Keyword::SOURCE => {
                    transformer.transform(sqlparser::keywords::Keyword::SOURCE, self, context)?
                }
                sqlparser::keywords::Keyword::SPATIAL => {
                    transformer.transform(sqlparser::keywords::Keyword::SPATIAL, self, context)?
                }
                sqlparser::keywords::Keyword::SPECIFIC => {
                    transformer.transform(sqlparser::keywords::Keyword::SPECIFIC, self, context)?
                }
                sqlparser::keywords::Keyword::SPECIFICTYPE => transformer.transform(
                    sqlparser::keywords::Keyword::SPECIFICTYPE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SQL => {
                    transformer.transform(sqlparser::keywords::Keyword::SQL, self, context)?
                }
                sqlparser::keywords::Keyword::SQLEXCEPTION => transformer.transform(
                    sqlparser::keywords::Keyword::SQLEXCEPTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SQLSTATE => {
                    transformer.transform(sqlparser::keywords::Keyword::SQLSTATE, self, context)?
                }
                sqlparser::keywords::Keyword::SQLWARNING => transformer.transform(
                    sqlparser::keywords::Keyword::SQLWARNING,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SQRT => {
                    transformer.transform(sqlparser::keywords::Keyword::SQRT, self, context)?
                }
                sqlparser::keywords::Keyword::STABLE => {
                    transformer.transform(sqlparser::keywords::Keyword::STABLE, self, context)?
                }
                sqlparser::keywords::Keyword::STAGE => {
                    transformer.transform(sqlparser::keywords::Keyword::STAGE, self, context)?
                }
                sqlparser::keywords::Keyword::START => {
                    transformer.transform(sqlparser::keywords::Keyword::START, self, context)?
                }
                sqlparser::keywords::Keyword::STATEMENT => {
                    transformer.transform(sqlparser::keywords::Keyword::STATEMENT, self, context)?
                }
                sqlparser::keywords::Keyword::STATIC => {
                    transformer.transform(sqlparser::keywords::Keyword::STATIC, self, context)?
                }
                sqlparser::keywords::Keyword::STATISTICS => transformer.transform(
                    sqlparser::keywords::Keyword::STATISTICS,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::STATUS => {
                    transformer.transform(sqlparser::keywords::Keyword::STATUS, self, context)?
                }
                sqlparser::keywords::Keyword::STDDEV_POP => transformer.transform(
                    sqlparser::keywords::Keyword::STDDEV_POP,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::STDDEV_SAMP => transformer.transform(
                    sqlparser::keywords::Keyword::STDDEV_SAMP,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::STDIN => {
                    transformer.transform(sqlparser::keywords::Keyword::STDIN, self, context)?
                }
                sqlparser::keywords::Keyword::STDOUT => {
                    transformer.transform(sqlparser::keywords::Keyword::STDOUT, self, context)?
                }
                sqlparser::keywords::Keyword::STEP => {
                    transformer.transform(sqlparser::keywords::Keyword::STEP, self, context)?
                }
                sqlparser::keywords::Keyword::STORAGE_INTEGRATION => transformer.transform(
                    sqlparser::keywords::Keyword::STORAGE_INTEGRATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::STORED => {
                    transformer.transform(sqlparser::keywords::Keyword::STORED, self, context)?
                }
                sqlparser::keywords::Keyword::STRICT => {
                    transformer.transform(sqlparser::keywords::Keyword::STRICT, self, context)?
                }
                sqlparser::keywords::Keyword::STRING => {
                    transformer.transform(sqlparser::keywords::Keyword::STRING, self, context)?
                }
                sqlparser::keywords::Keyword::STRUCT => {
                    transformer.transform(sqlparser::keywords::Keyword::STRUCT, self, context)?
                }
                sqlparser::keywords::Keyword::SUBMULTISET => transformer.transform(
                    sqlparser::keywords::Keyword::SUBMULTISET,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SUBSTRING => {
                    transformer.transform(sqlparser::keywords::Keyword::SUBSTRING, self, context)?
                }
                sqlparser::keywords::Keyword::SUBSTRING_REGEX => transformer.transform(
                    sqlparser::keywords::Keyword::SUBSTRING_REGEX,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SUCCEEDS => {
                    transformer.transform(sqlparser::keywords::Keyword::SUCCEEDS, self, context)?
                }
                sqlparser::keywords::Keyword::SUM => {
                    transformer.transform(sqlparser::keywords::Keyword::SUM, self, context)?
                }
                sqlparser::keywords::Keyword::SUPER => {
                    transformer.transform(sqlparser::keywords::Keyword::SUPER, self, context)?
                }
                sqlparser::keywords::Keyword::SUPERUSER => {
                    transformer.transform(sqlparser::keywords::Keyword::SUPERUSER, self, context)?
                }
                sqlparser::keywords::Keyword::SWAP => {
                    transformer.transform(sqlparser::keywords::Keyword::SWAP, self, context)?
                }
                sqlparser::keywords::Keyword::SYMMETRIC => {
                    transformer.transform(sqlparser::keywords::Keyword::SYMMETRIC, self, context)?
                }
                sqlparser::keywords::Keyword::SYNC => {
                    transformer.transform(sqlparser::keywords::Keyword::SYNC, self, context)?
                }
                sqlparser::keywords::Keyword::SYSTEM => {
                    transformer.transform(sqlparser::keywords::Keyword::SYSTEM, self, context)?
                }
                sqlparser::keywords::Keyword::SYSTEM_TIME => transformer.transform(
                    sqlparser::keywords::Keyword::SYSTEM_TIME,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::SYSTEM_USER => transformer.transform(
                    sqlparser::keywords::Keyword::SYSTEM_USER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TABLE => {
                    transformer.transform(sqlparser::keywords::Keyword::TABLE, self, context)?
                }
                sqlparser::keywords::Keyword::TABLES => {
                    transformer.transform(sqlparser::keywords::Keyword::TABLES, self, context)?
                }
                sqlparser::keywords::Keyword::TABLESAMPLE => transformer.transform(
                    sqlparser::keywords::Keyword::TABLESAMPLE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TAG => {
                    transformer.transform(sqlparser::keywords::Keyword::TAG, self, context)?
                }
                sqlparser::keywords::Keyword::TARGET => {
                    transformer.transform(sqlparser::keywords::Keyword::TARGET, self, context)?
                }
                sqlparser::keywords::Keyword::TBLPROPERTIES => transformer.transform(
                    sqlparser::keywords::Keyword::TBLPROPERTIES,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TEMP => {
                    transformer.transform(sqlparser::keywords::Keyword::TEMP, self, context)?
                }
                sqlparser::keywords::Keyword::TEMPORARY => {
                    transformer.transform(sqlparser::keywords::Keyword::TEMPORARY, self, context)?
                }
                sqlparser::keywords::Keyword::TERMINATED => transformer.transform(
                    sqlparser::keywords::Keyword::TERMINATED,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TEXT => {
                    transformer.transform(sqlparser::keywords::Keyword::TEXT, self, context)?
                }
                sqlparser::keywords::Keyword::TEXTFILE => {
                    transformer.transform(sqlparser::keywords::Keyword::TEXTFILE, self, context)?
                }
                sqlparser::keywords::Keyword::THEN => {
                    transformer.transform(sqlparser::keywords::Keyword::THEN, self, context)?
                }
                sqlparser::keywords::Keyword::TIES => {
                    transformer.transform(sqlparser::keywords::Keyword::TIES, self, context)?
                }
                sqlparser::keywords::Keyword::TIME => {
                    transformer.transform(sqlparser::keywords::Keyword::TIME, self, context)?
                }
                sqlparser::keywords::Keyword::TIMESTAMP => {
                    transformer.transform(sqlparser::keywords::Keyword::TIMESTAMP, self, context)?
                }
                sqlparser::keywords::Keyword::TIMESTAMPTZ => transformer.transform(
                    sqlparser::keywords::Keyword::TIMESTAMPTZ,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TIMETZ => {
                    transformer.transform(sqlparser::keywords::Keyword::TIMETZ, self, context)?
                }
                sqlparser::keywords::Keyword::TIMEZONE => {
                    transformer.transform(sqlparser::keywords::Keyword::TIMEZONE, self, context)?
                }
                sqlparser::keywords::Keyword::TIMEZONE_ABBR => transformer.transform(
                    sqlparser::keywords::Keyword::TIMEZONE_ABBR,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TIMEZONE_HOUR => transformer.transform(
                    sqlparser::keywords::Keyword::TIMEZONE_HOUR,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TIMEZONE_MINUTE => transformer.transform(
                    sqlparser::keywords::Keyword::TIMEZONE_MINUTE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TIMEZONE_REGION => transformer.transform(
                    sqlparser::keywords::Keyword::TIMEZONE_REGION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TINYINT => {
                    transformer.transform(sqlparser::keywords::Keyword::TINYINT, self, context)?
                }
                sqlparser::keywords::Keyword::TO => {
                    transformer.transform(sqlparser::keywords::Keyword::TO, self, context)?
                }
                sqlparser::keywords::Keyword::TOP => {
                    transformer.transform(sqlparser::keywords::Keyword::TOP, self, context)?
                }
                sqlparser::keywords::Keyword::TOTALS => {
                    transformer.transform(sqlparser::keywords::Keyword::TOTALS, self, context)?
                }
                sqlparser::keywords::Keyword::TRAILING => {
                    transformer.transform(sqlparser::keywords::Keyword::TRAILING, self, context)?
                }
                sqlparser::keywords::Keyword::TRANSACTION => transformer.transform(
                    sqlparser::keywords::Keyword::TRANSACTION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TRANSIENT => {
                    transformer.transform(sqlparser::keywords::Keyword::TRANSIENT, self, context)?
                }
                sqlparser::keywords::Keyword::TRANSLATE => {
                    transformer.transform(sqlparser::keywords::Keyword::TRANSLATE, self, context)?
                }
                sqlparser::keywords::Keyword::TRANSLATE_REGEX => transformer.transform(
                    sqlparser::keywords::Keyword::TRANSLATE_REGEX,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TRANSLATION => transformer.transform(
                    sqlparser::keywords::Keyword::TRANSLATION,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TREAT => {
                    transformer.transform(sqlparser::keywords::Keyword::TREAT, self, context)?
                }
                sqlparser::keywords::Keyword::TRIGGER => {
                    transformer.transform(sqlparser::keywords::Keyword::TRIGGER, self, context)?
                }
                sqlparser::keywords::Keyword::TRIM => {
                    transformer.transform(sqlparser::keywords::Keyword::TRIM, self, context)?
                }
                sqlparser::keywords::Keyword::TRIM_ARRAY => transformer.transform(
                    sqlparser::keywords::Keyword::TRIM_ARRAY,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TRUE => {
                    transformer.transform(sqlparser::keywords::Keyword::TRUE, self, context)?
                }
                sqlparser::keywords::Keyword::TRUNCATE => {
                    transformer.transform(sqlparser::keywords::Keyword::TRUNCATE, self, context)?
                }
                sqlparser::keywords::Keyword::TRY_CAST => {
                    transformer.transform(sqlparser::keywords::Keyword::TRY_CAST, self, context)?
                }
                sqlparser::keywords::Keyword::TRY_CONVERT => transformer.transform(
                    sqlparser::keywords::Keyword::TRY_CONVERT,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::TUPLE => {
                    transformer.transform(sqlparser::keywords::Keyword::TUPLE, self, context)?
                }
                sqlparser::keywords::Keyword::TYPE => {
                    transformer.transform(sqlparser::keywords::Keyword::TYPE, self, context)?
                }
                sqlparser::keywords::Keyword::UESCAPE => {
                    transformer.transform(sqlparser::keywords::Keyword::UESCAPE, self, context)?
                }
                sqlparser::keywords::Keyword::UINT128 => {
                    transformer.transform(sqlparser::keywords::Keyword::UINT128, self, context)?
                }
                sqlparser::keywords::Keyword::UINT16 => {
                    transformer.transform(sqlparser::keywords::Keyword::UINT16, self, context)?
                }
                sqlparser::keywords::Keyword::UINT256 => {
                    transformer.transform(sqlparser::keywords::Keyword::UINT256, self, context)?
                }
                sqlparser::keywords::Keyword::UINT32 => {
                    transformer.transform(sqlparser::keywords::Keyword::UINT32, self, context)?
                }
                sqlparser::keywords::Keyword::UINT64 => {
                    transformer.transform(sqlparser::keywords::Keyword::UINT64, self, context)?
                }
                sqlparser::keywords::Keyword::UINT8 => {
                    transformer.transform(sqlparser::keywords::Keyword::UINT8, self, context)?
                }
                sqlparser::keywords::Keyword::UNBOUNDED => {
                    transformer.transform(sqlparser::keywords::Keyword::UNBOUNDED, self, context)?
                }
                sqlparser::keywords::Keyword::UNCACHE => {
                    transformer.transform(sqlparser::keywords::Keyword::UNCACHE, self, context)?
                }
                sqlparser::keywords::Keyword::UNCOMMITTED => transformer.transform(
                    sqlparser::keywords::Keyword::UNCOMMITTED,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::UNFREEZE => {
                    transformer.transform(sqlparser::keywords::Keyword::UNFREEZE, self, context)?
                }
                sqlparser::keywords::Keyword::UNION => {
                    transformer.transform(sqlparser::keywords::Keyword::UNION, self, context)?
                }
                sqlparser::keywords::Keyword::UNIQUE => {
                    transformer.transform(sqlparser::keywords::Keyword::UNIQUE, self, context)?
                }
                sqlparser::keywords::Keyword::UNKNOWN => {
                    transformer.transform(sqlparser::keywords::Keyword::UNKNOWN, self, context)?
                }
                sqlparser::keywords::Keyword::UNLOAD => {
                    transformer.transform(sqlparser::keywords::Keyword::UNLOAD, self, context)?
                }
                sqlparser::keywords::Keyword::UNLOCK => {
                    transformer.transform(sqlparser::keywords::Keyword::UNLOCK, self, context)?
                }
                sqlparser::keywords::Keyword::UNLOGGED => {
                    transformer.transform(sqlparser::keywords::Keyword::UNLOGGED, self, context)?
                }
                sqlparser::keywords::Keyword::UNMATCHED => {
                    transformer.transform(sqlparser::keywords::Keyword::UNMATCHED, self, context)?
                }
                sqlparser::keywords::Keyword::UNNEST => {
                    transformer.transform(sqlparser::keywords::Keyword::UNNEST, self, context)?
                }
                sqlparser::keywords::Keyword::UNPIVOT => {
                    transformer.transform(sqlparser::keywords::Keyword::UNPIVOT, self, context)?
                }
                sqlparser::keywords::Keyword::UNSAFE => {
                    transformer.transform(sqlparser::keywords::Keyword::UNSAFE, self, context)?
                }
                sqlparser::keywords::Keyword::UNSIGNED => {
                    transformer.transform(sqlparser::keywords::Keyword::UNSIGNED, self, context)?
                }
                sqlparser::keywords::Keyword::UNTIL => {
                    transformer.transform(sqlparser::keywords::Keyword::UNTIL, self, context)?
                }
                sqlparser::keywords::Keyword::UPDATE => {
                    transformer.transform(sqlparser::keywords::Keyword::UPDATE, self, context)?
                }
                sqlparser::keywords::Keyword::UPPER => {
                    transformer.transform(sqlparser::keywords::Keyword::UPPER, self, context)?
                }
                sqlparser::keywords::Keyword::URL => {
                    transformer.transform(sqlparser::keywords::Keyword::URL, self, context)?
                }
                sqlparser::keywords::Keyword::USAGE => {
                    transformer.transform(sqlparser::keywords::Keyword::USAGE, self, context)?
                }
                sqlparser::keywords::Keyword::USE => {
                    transformer.transform(sqlparser::keywords::Keyword::USE, self, context)?
                }
                sqlparser::keywords::Keyword::USER => {
                    transformer.transform(sqlparser::keywords::Keyword::USER, self, context)?
                }
                sqlparser::keywords::Keyword::USER_RESOURCES => transformer.transform(
                    sqlparser::keywords::Keyword::USER_RESOURCES,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::USING => {
                    transformer.transform(sqlparser::keywords::Keyword::USING, self, context)?
                }
                sqlparser::keywords::Keyword::UUID => {
                    transformer.transform(sqlparser::keywords::Keyword::UUID, self, context)?
                }
                sqlparser::keywords::Keyword::VACUUM => {
                    transformer.transform(sqlparser::keywords::Keyword::VACUUM, self, context)?
                }
                sqlparser::keywords::Keyword::VALID => {
                    transformer.transform(sqlparser::keywords::Keyword::VALID, self, context)?
                }
                sqlparser::keywords::Keyword::VALIDATION_MODE => transformer.transform(
                    sqlparser::keywords::Keyword::VALIDATION_MODE,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::VALUE => {
                    transformer.transform(sqlparser::keywords::Keyword::VALUE, self, context)?
                }
                sqlparser::keywords::Keyword::VALUES => {
                    transformer.transform(sqlparser::keywords::Keyword::VALUES, self, context)?
                }
                sqlparser::keywords::Keyword::VALUE_OF => {
                    transformer.transform(sqlparser::keywords::Keyword::VALUE_OF, self, context)?
                }
                sqlparser::keywords::Keyword::VARBINARY => {
                    transformer.transform(sqlparser::keywords::Keyword::VARBINARY, self, context)?
                }
                sqlparser::keywords::Keyword::VARCHAR => {
                    transformer.transform(sqlparser::keywords::Keyword::VARCHAR, self, context)?
                }
                sqlparser::keywords::Keyword::VARIABLES => {
                    transformer.transform(sqlparser::keywords::Keyword::VARIABLES, self, context)?
                }
                sqlparser::keywords::Keyword::VARYING => {
                    transformer.transform(sqlparser::keywords::Keyword::VARYING, self, context)?
                }
                sqlparser::keywords::Keyword::VAR_POP => {
                    transformer.transform(sqlparser::keywords::Keyword::VAR_POP, self, context)?
                }
                sqlparser::keywords::Keyword::VAR_SAMP => {
                    transformer.transform(sqlparser::keywords::Keyword::VAR_SAMP, self, context)?
                }
                sqlparser::keywords::Keyword::VERBOSE => {
                    transformer.transform(sqlparser::keywords::Keyword::VERBOSE, self, context)?
                }
                sqlparser::keywords::Keyword::VERSION => {
                    transformer.transform(sqlparser::keywords::Keyword::VERSION, self, context)?
                }
                sqlparser::keywords::Keyword::VERSIONING => transformer.transform(
                    sqlparser::keywords::Keyword::VERSIONING,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::VIEW => {
                    transformer.transform(sqlparser::keywords::Keyword::VIEW, self, context)?
                }
                sqlparser::keywords::Keyword::VIEWS => {
                    transformer.transform(sqlparser::keywords::Keyword::VIEWS, self, context)?
                }
                sqlparser::keywords::Keyword::VIRTUAL => {
                    transformer.transform(sqlparser::keywords::Keyword::VIRTUAL, self, context)?
                }
                sqlparser::keywords::Keyword::VOLATILE => {
                    transformer.transform(sqlparser::keywords::Keyword::VOLATILE, self, context)?
                }
                sqlparser::keywords::Keyword::WAREHOUSE => {
                    transformer.transform(sqlparser::keywords::Keyword::WAREHOUSE, self, context)?
                }
                sqlparser::keywords::Keyword::WEEK => {
                    transformer.transform(sqlparser::keywords::Keyword::WEEK, self, context)?
                }
                sqlparser::keywords::Keyword::WHEN => {
                    transformer.transform(sqlparser::keywords::Keyword::WHEN, self, context)?
                }
                sqlparser::keywords::Keyword::WHENEVER => {
                    transformer.transform(sqlparser::keywords::Keyword::WHENEVER, self, context)?
                }
                sqlparser::keywords::Keyword::WHERE => {
                    transformer.transform(sqlparser::keywords::Keyword::WHERE, self, context)?
                }
                sqlparser::keywords::Keyword::WIDTH_BUCKET => transformer.transform(
                    sqlparser::keywords::Keyword::WIDTH_BUCKET,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::WINDOW => {
                    transformer.transform(sqlparser::keywords::Keyword::WINDOW, self, context)?
                }
                sqlparser::keywords::Keyword::WITH => {
                    transformer.transform(sqlparser::keywords::Keyword::WITH, self, context)?
                }
                sqlparser::keywords::Keyword::WITHIN => {
                    transformer.transform(sqlparser::keywords::Keyword::WITHIN, self, context)?
                }
                sqlparser::keywords::Keyword::WITHOUT => {
                    transformer.transform(sqlparser::keywords::Keyword::WITHOUT, self, context)?
                }
                sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER => transformer.transform(
                    sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER,
                    self,
                    context,
                )?,
                sqlparser::keywords::Keyword::WORK => {
                    transformer.transform(sqlparser::keywords::Keyword::WORK, self, context)?
                }
                sqlparser::keywords::Keyword::WRITE => {
                    transformer.transform(sqlparser::keywords::Keyword::WRITE, self, context)?
                }
                sqlparser::keywords::Keyword::XML => {
                    transformer.transform(sqlparser::keywords::Keyword::XML, self, context)?
                }
                sqlparser::keywords::Keyword::XOR => {
                    transformer.transform(sqlparser::keywords::Keyword::XOR, self, context)?
                }
                sqlparser::keywords::Keyword::YEAR => {
                    transformer.transform(sqlparser::keywords::Keyword::YEAR, self, context)?
                }
                sqlparser::keywords::Keyword::ZONE => {
                    transformer.transform(sqlparser::keywords::Keyword::ZONE, self, context)?
                }
                sqlparser::keywords::Keyword::ZORDER => {
                    transformer.transform(sqlparser::keywords::Keyword::ZORDER, self, context)?
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Token {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::tokenizer::Token::EOF => {
                    transformer.transform(sqlparser::tokenizer::Token::EOF, self, context)?
                }
                sqlparser::tokenizer::Token::Word(field0) => sqlparser::tokenizer::Token::Word(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::tokenizer::Token::Number(field0, field1) => {
                    sqlparser::tokenizer::Token::Number(
                        field0.apply_transform_with_path(transformer, context)?,
                        field1.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::Char(field0) => sqlparser::tokenizer::Token::Char(
                    field0.apply_transform_with_path(transformer, context)?,
                ),
                sqlparser::tokenizer::Token::SingleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::SingleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::DoubleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleSingleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::TripleSingleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleDoubleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::TripleDoubleQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::DollarQuotedString(field0) => {
                    sqlparser::tokenizer::Token::DollarQuotedString(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::NationalStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::NationalStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::EscapedStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::EscapedStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::UnicodeStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::UnicodeStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::HexStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::HexStringLiteral(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::Comma => {
                    transformer.transform(sqlparser::tokenizer::Token::Comma, self, context)?
                }
                sqlparser::tokenizer::Token::Whitespace(field0) => {
                    sqlparser::tokenizer::Token::Whitespace(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleEq => {
                    transformer.transform(sqlparser::tokenizer::Token::DoubleEq, self, context)?
                }
                sqlparser::tokenizer::Token::Eq => {
                    transformer.transform(sqlparser::tokenizer::Token::Eq, self, context)?
                }
                sqlparser::tokenizer::Token::Neq => {
                    transformer.transform(sqlparser::tokenizer::Token::Neq, self, context)?
                }
                sqlparser::tokenizer::Token::Lt => {
                    transformer.transform(sqlparser::tokenizer::Token::Lt, self, context)?
                }
                sqlparser::tokenizer::Token::Gt => {
                    transformer.transform(sqlparser::tokenizer::Token::Gt, self, context)?
                }
                sqlparser::tokenizer::Token::LtEq => {
                    transformer.transform(sqlparser::tokenizer::Token::LtEq, self, context)?
                }
                sqlparser::tokenizer::Token::GtEq => {
                    transformer.transform(sqlparser::tokenizer::Token::GtEq, self, context)?
                }
                sqlparser::tokenizer::Token::Spaceship => {
                    transformer.transform(sqlparser::tokenizer::Token::Spaceship, self, context)?
                }
                sqlparser::tokenizer::Token::Plus => {
                    transformer.transform(sqlparser::tokenizer::Token::Plus, self, context)?
                }
                sqlparser::tokenizer::Token::Minus => {
                    transformer.transform(sqlparser::tokenizer::Token::Minus, self, context)?
                }
                sqlparser::tokenizer::Token::Mul => {
                    transformer.transform(sqlparser::tokenizer::Token::Mul, self, context)?
                }
                sqlparser::tokenizer::Token::Div => {
                    transformer.transform(sqlparser::tokenizer::Token::Div, self, context)?
                }
                sqlparser::tokenizer::Token::DuckIntDiv => {
                    transformer.transform(sqlparser::tokenizer::Token::DuckIntDiv, self, context)?
                }
                sqlparser::tokenizer::Token::Mod => {
                    transformer.transform(sqlparser::tokenizer::Token::Mod, self, context)?
                }
                sqlparser::tokenizer::Token::StringConcat => transformer.transform(
                    sqlparser::tokenizer::Token::StringConcat,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::LParen => {
                    transformer.transform(sqlparser::tokenizer::Token::LParen, self, context)?
                }
                sqlparser::tokenizer::Token::RParen => {
                    transformer.transform(sqlparser::tokenizer::Token::RParen, self, context)?
                }
                sqlparser::tokenizer::Token::Period => {
                    transformer.transform(sqlparser::tokenizer::Token::Period, self, context)?
                }
                sqlparser::tokenizer::Token::Colon => {
                    transformer.transform(sqlparser::tokenizer::Token::Colon, self, context)?
                }
                sqlparser::tokenizer::Token::DoubleColon => transformer.transform(
                    sqlparser::tokenizer::Token::DoubleColon,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::Assignment => {
                    transformer.transform(sqlparser::tokenizer::Token::Assignment, self, context)?
                }
                sqlparser::tokenizer::Token::SemiColon => {
                    transformer.transform(sqlparser::tokenizer::Token::SemiColon, self, context)?
                }
                sqlparser::tokenizer::Token::Backslash => {
                    transformer.transform(sqlparser::tokenizer::Token::Backslash, self, context)?
                }
                sqlparser::tokenizer::Token::LBracket => {
                    transformer.transform(sqlparser::tokenizer::Token::LBracket, self, context)?
                }
                sqlparser::tokenizer::Token::RBracket => {
                    transformer.transform(sqlparser::tokenizer::Token::RBracket, self, context)?
                }
                sqlparser::tokenizer::Token::Ampersand => {
                    transformer.transform(sqlparser::tokenizer::Token::Ampersand, self, context)?
                }
                sqlparser::tokenizer::Token::Pipe => {
                    transformer.transform(sqlparser::tokenizer::Token::Pipe, self, context)?
                }
                sqlparser::tokenizer::Token::Caret => {
                    transformer.transform(sqlparser::tokenizer::Token::Caret, self, context)?
                }
                sqlparser::tokenizer::Token::LBrace => {
                    transformer.transform(sqlparser::tokenizer::Token::LBrace, self, context)?
                }
                sqlparser::tokenizer::Token::RBrace => {
                    transformer.transform(sqlparser::tokenizer::Token::RBrace, self, context)?
                }
                sqlparser::tokenizer::Token::RArrow => {
                    transformer.transform(sqlparser::tokenizer::Token::RArrow, self, context)?
                }
                sqlparser::tokenizer::Token::Sharp => {
                    transformer.transform(sqlparser::tokenizer::Token::Sharp, self, context)?
                }
                sqlparser::tokenizer::Token::Tilde => {
                    transformer.transform(sqlparser::tokenizer::Token::Tilde, self, context)?
                }
                sqlparser::tokenizer::Token::TildeAsterisk => transformer.transform(
                    sqlparser::tokenizer::Token::TildeAsterisk,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::ExclamationMarkTilde => transformer.transform(
                    sqlparser::tokenizer::Token::ExclamationMarkTilde,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk => transformer
                    .transform(
                        sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk,
                        self,
                        context,
                    )?,
                sqlparser::tokenizer::Token::DoubleTilde => transformer.transform(
                    sqlparser::tokenizer::Token::DoubleTilde,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::DoubleTildeAsterisk => transformer.transform(
                    sqlparser::tokenizer::Token::DoubleTildeAsterisk,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde => transformer.transform(
                    sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk => transformer
                    .transform(
                        sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk,
                        self,
                        context,
                    )?,
                sqlparser::tokenizer::Token::ShiftLeft => {
                    transformer.transform(sqlparser::tokenizer::Token::ShiftLeft, self, context)?
                }
                sqlparser::tokenizer::Token::ShiftRight => {
                    transformer.transform(sqlparser::tokenizer::Token::ShiftRight, self, context)?
                }
                sqlparser::tokenizer::Token::Overlap => {
                    transformer.transform(sqlparser::tokenizer::Token::Overlap, self, context)?
                }
                sqlparser::tokenizer::Token::ExclamationMark => transformer.transform(
                    sqlparser::tokenizer::Token::ExclamationMark,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::DoubleExclamationMark => transformer.transform(
                    sqlparser::tokenizer::Token::DoubleExclamationMark,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::AtSign => {
                    transformer.transform(sqlparser::tokenizer::Token::AtSign, self, context)?
                }
                sqlparser::tokenizer::Token::CaretAt => {
                    transformer.transform(sqlparser::tokenizer::Token::CaretAt, self, context)?
                }
                sqlparser::tokenizer::Token::PGSquareRoot => transformer.transform(
                    sqlparser::tokenizer::Token::PGSquareRoot,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::PGCubeRoot => {
                    transformer.transform(sqlparser::tokenizer::Token::PGCubeRoot, self, context)?
                }
                sqlparser::tokenizer::Token::Placeholder(field0) => {
                    sqlparser::tokenizer::Token::Placeholder(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
                sqlparser::tokenizer::Token::Arrow => {
                    transformer.transform(sqlparser::tokenizer::Token::Arrow, self, context)?
                }
                sqlparser::tokenizer::Token::LongArrow => {
                    transformer.transform(sqlparser::tokenizer::Token::LongArrow, self, context)?
                }
                sqlparser::tokenizer::Token::HashArrow => {
                    transformer.transform(sqlparser::tokenizer::Token::HashArrow, self, context)?
                }
                sqlparser::tokenizer::Token::HashLongArrow => transformer.transform(
                    sqlparser::tokenizer::Token::HashLongArrow,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::AtArrow => {
                    transformer.transform(sqlparser::tokenizer::Token::AtArrow, self, context)?
                }
                sqlparser::tokenizer::Token::ArrowAt => {
                    transformer.transform(sqlparser::tokenizer::Token::ArrowAt, self, context)?
                }
                sqlparser::tokenizer::Token::HashMinus => {
                    transformer.transform(sqlparser::tokenizer::Token::HashMinus, self, context)?
                }
                sqlparser::tokenizer::Token::AtQuestion => {
                    transformer.transform(sqlparser::tokenizer::Token::AtQuestion, self, context)?
                }
                sqlparser::tokenizer::Token::AtAt => {
                    transformer.transform(sqlparser::tokenizer::Token::AtAt, self, context)?
                }
                sqlparser::tokenizer::Token::Question => {
                    transformer.transform(sqlparser::tokenizer::Token::Question, self, context)?
                }
                sqlparser::tokenizer::Token::QuestionAnd => transformer.transform(
                    sqlparser::tokenizer::Token::QuestionAnd,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::QuestionPipe => transformer.transform(
                    sqlparser::tokenizer::Token::QuestionPipe,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Token::CustomBinaryOperator(field0) => {
                    sqlparser::tokenizer::Token::CustomBinaryOperator(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Whitespace {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            match self {
                sqlparser::tokenizer::Whitespace::Space => {
                    transformer.transform(sqlparser::tokenizer::Whitespace::Space, self, context)?
                }
                sqlparser::tokenizer::Whitespace::Newline => transformer.transform(
                    sqlparser::tokenizer::Whitespace::Newline,
                    self,
                    context,
                )?,
                sqlparser::tokenizer::Whitespace::Tab => {
                    transformer.transform(sqlparser::tokenizer::Whitespace::Tab, self, context)?
                }
                sqlparser::tokenizer::Whitespace::SingleLineComment { comment, prefix } => {
                    sqlparser::tokenizer::Whitespace::SingleLineComment {
                        comment: comment.apply_transform_with_path(transformer, context)?,
                        prefix: prefix.apply_transform_with_path(transformer, context)?,
                    }
                }
                sqlparser::tokenizer::Whitespace::MultiLineComment(field0) => {
                    sqlparser::tokenizer::Whitespace::MultiLineComment(
                        field0.apply_transform_with_path(transformer, context)?,
                    )
                }
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Word {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = {
            let Self {
                value,
                quote_style,
                keyword,
            } = self;
            Self {
                value: value.apply_transform_with_path(transformer, context)?,
                quote_style: quote_style.apply_transform_with_path(transformer, context)?,
                keyword: keyword.apply_transform_with_path(transformer, context)?,
            }
        };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for bigdecimal::BigDecimal {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(self.clone(), self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for bool {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for char {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i16 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i32 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i64 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i8 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for String {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(self.clone(), self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u16 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u32 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u64 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u8 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        context: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        context.push(self as &'ast dyn std::any::Any);
        let transformed = { transformer.transform(*self, self, context)? };
        context.pop();
        transformer.transform(transformed, self, context)
    }
}
