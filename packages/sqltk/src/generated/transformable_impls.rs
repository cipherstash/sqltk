#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Action {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Action::Connect => {
                    transformer.transform(node_path, sqlparser::ast::Action::Connect)?
                }
                sqlparser::ast::Action::Create => {
                    transformer.transform(node_path, sqlparser::ast::Action::Create)?
                }
                sqlparser::ast::Action::Delete => {
                    transformer.transform(node_path, sqlparser::ast::Action::Delete)?
                }
                sqlparser::ast::Action::Execute => {
                    transformer.transform(node_path, sqlparser::ast::Action::Execute)?
                }
                sqlparser::ast::Action::Insert { columns } => {
                    sqlparser::ast::Action::Insert {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Action::References { columns } => {
                    sqlparser::ast::Action::References {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Action::Select { columns } => {
                    sqlparser::ast::Action::Select {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Action::Temporary => {
                    transformer.transform(node_path, sqlparser::ast::Action::Temporary)?
                }
                sqlparser::ast::Action::Trigger => {
                    transformer.transform(node_path, sqlparser::ast::Action::Trigger)?
                }
                sqlparser::ast::Action::Truncate => {
                    transformer.transform(node_path, sqlparser::ast::Action::Truncate)?
                }
                sqlparser::ast::Action::Update { columns } => {
                    sqlparser::ast::Action::Update {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Action::Usage => {
                    transformer.transform(node_path, sqlparser::ast::Action::Usage)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AddDropSync {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AddDropSync::ADD => {
                    transformer.transform(node_path, sqlparser::ast::AddDropSync::ADD)?
                }
                sqlparser::ast::AddDropSync::DROP => {
                    transformer.transform(node_path, sqlparser::ast::AddDropSync::DROP)?
                }
                sqlparser::ast::AddDropSync::SYNC => {
                    transformer.transform(node_path, sqlparser::ast::AddDropSync::SYNC)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AfterMatchSkip {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AfterMatchSkip::PastLastRow => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AfterMatchSkip::PastLastRow,
                        )?
                }
                sqlparser::ast::AfterMatchSkip::ToNextRow => {
                    transformer
                        .transform(node_path, sqlparser::ast::AfterMatchSkip::ToNextRow)?
                }
                sqlparser::ast::AfterMatchSkip::ToFirst(field0) => {
                    sqlparser::ast::AfterMatchSkip::ToFirst(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::AfterMatchSkip::ToLast(field0) => {
                    sqlparser::ast::AfterMatchSkip::ToLast(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterColumnOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AlterColumnOperation::SetNotNull => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AlterColumnOperation::SetNotNull,
                        )?
                }
                sqlparser::ast::AlterColumnOperation::DropNotNull => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AlterColumnOperation::DropNotNull,
                        )?
                }
                sqlparser::ast::AlterColumnOperation::SetDefault { value } => {
                    sqlparser::ast::AlterColumnOperation::SetDefault {
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterColumnOperation::DropDefault => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AlterColumnOperation::DropDefault,
                        )?
                }
                sqlparser::ast::AlterColumnOperation::SetDataType {
                    data_type,
                    using,
                } => {
                    sqlparser::ast::AlterColumnOperation::SetDataType {
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterColumnOperation::AddGenerated {
                    generated_as,
                    sequence_options,
                } => {
                    sqlparser::ast::AlterColumnOperation::AddGenerated {
                        generated_as: generated_as
                            .apply_transform_with_path(transformer, node_path)?,
                        sequence_options: sequence_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterIndexOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AlterIndexOperation::RenameIndex { index_name } => {
                    sqlparser::ast::AlterIndexOperation::RenameIndex {
                        index_name: index_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterPolicyOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AlterPolicyOperation::Rename { new_name } => {
                    sqlparser::ast::AlterPolicyOperation::Rename {
                        new_name: new_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterPolicyOperation::Apply { to, using, with_check } => {
                    sqlparser::ast::AlterPolicyOperation::Apply {
                        to: to.apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                        with_check: with_check
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterRoleOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AlterRoleOperation::RenameRole { role_name } => {
                    sqlparser::ast::AlterRoleOperation::RenameRole {
                        role_name: role_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::AddMember { member_name } => {
                    sqlparser::ast::AlterRoleOperation::AddMember {
                        member_name: member_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::DropMember { member_name } => {
                    sqlparser::ast::AlterRoleOperation::DropMember {
                        member_name: member_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::WithOptions { options } => {
                    sqlparser::ast::AlterRoleOperation::WithOptions {
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::Set {
                    config_name,
                    config_value,
                    in_database,
                } => {
                    sqlparser::ast::AlterRoleOperation::Set {
                        config_name: config_name
                            .apply_transform_with_path(transformer, node_path)?,
                        config_value: config_value
                            .apply_transform_with_path(transformer, node_path)?,
                        in_database: in_database
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterRoleOperation::Reset {
                    config_name,
                    in_database,
                } => {
                    sqlparser::ast::AlterRoleOperation::Reset {
                        config_name: config_name
                            .apply_transform_with_path(transformer, node_path)?,
                        in_database: in_database
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AlterTableOperation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AlterTableOperation::AddConstraint(field0) => {
                    sqlparser::ast::AlterTableOperation::AddConstraint(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::AlterTableOperation::AddColumn {
                    column_keyword,
                    if_not_exists,
                    column_def,
                    column_position,
                } => {
                    sqlparser::ast::AlterTableOperation::AddColumn {
                        column_keyword: column_keyword
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        column_def: column_def
                            .apply_transform_with_path(transformer, node_path)?,
                        column_position: column_position
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::AddProjection {
                    if_not_exists,
                    name,
                    select,
                } => {
                    sqlparser::ast::AlterTableOperation::AddProjection {
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        select: select.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DropProjection {
                    if_exists,
                    name,
                } => {
                    sqlparser::ast::AlterTableOperation::DropProjection {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::MaterializeProjection {
                    if_exists,
                    name,
                    partition,
                } => {
                    sqlparser::ast::AlterTableOperation::MaterializeProjection {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::ClearProjection {
                    if_exists,
                    name,
                    partition,
                } => {
                    sqlparser::ast::AlterTableOperation::ClearProjection {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AlterTableOperation::DisableRowLevelSecurity,
                        )?
                }
                sqlparser::ast::AlterTableOperation::DisableRule { name } => {
                    sqlparser::ast::AlterTableOperation::DisableRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DisableTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::DisableTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DropConstraint {
                    if_exists,
                    name,
                    cascade,
                } => {
                    sqlparser::ast::AlterTableOperation::DropConstraint {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DropColumn {
                    column_name,
                    if_exists,
                    cascade,
                } => {
                    sqlparser::ast::AlterTableOperation::DropColumn {
                        column_name: column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::AttachPartition { partition } => {
                    sqlparser::ast::AlterTableOperation::AttachPartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DetachPartition { partition } => {
                    sqlparser::ast::AlterTableOperation::DetachPartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::FreezePartition {
                    partition,
                    with_name,
                } => {
                    sqlparser::ast::AlterTableOperation::FreezePartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                        with_name: with_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::UnfreezePartition {
                    partition,
                    with_name,
                } => {
                    sqlparser::ast::AlterTableOperation::UnfreezePartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                        with_name: with_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DropPrimaryKey => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AlterTableOperation::DropPrimaryKey,
                        )?
                }
                sqlparser::ast::AlterTableOperation::EnableAlwaysRule { name } => {
                    sqlparser::ast::AlterTableOperation::EnableAlwaysRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::EnableAlwaysTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableReplicaRule { name } => {
                    sqlparser::ast::AlterTableOperation::EnableReplicaRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableReplicaTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::EnableReplicaTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::AlterTableOperation::EnableRowLevelSecurity,
                        )?
                }
                sqlparser::ast::AlterTableOperation::EnableRule { name } => {
                    sqlparser::ast::AlterTableOperation::EnableRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::EnableTrigger { name } => {
                    sqlparser::ast::AlterTableOperation::EnableTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::RenamePartitions {
                    old_partitions,
                    new_partitions,
                } => {
                    sqlparser::ast::AlterTableOperation::RenamePartitions {
                        old_partitions: old_partitions
                            .apply_transform_with_path(transformer, node_path)?,
                        new_partitions: new_partitions
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::AddPartitions {
                    if_not_exists,
                    new_partitions,
                } => {
                    sqlparser::ast::AlterTableOperation::AddPartitions {
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        new_partitions: new_partitions
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::DropPartitions {
                    partitions,
                    if_exists,
                } => {
                    sqlparser::ast::AlterTableOperation::DropPartitions {
                        partitions: partitions
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::RenameColumn {
                    old_column_name,
                    new_column_name,
                } => {
                    sqlparser::ast::AlterTableOperation::RenameColumn {
                        old_column_name: old_column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        new_column_name: new_column_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::RenameTable { table_name } => {
                    sqlparser::ast::AlterTableOperation::RenameTable {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::ChangeColumn {
                    old_name,
                    new_name,
                    data_type,
                    options,
                    column_position,
                } => {
                    sqlparser::ast::AlterTableOperation::ChangeColumn {
                        old_name: old_name
                            .apply_transform_with_path(transformer, node_path)?,
                        new_name: new_name
                            .apply_transform_with_path(transformer, node_path)?,
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                        column_position: column_position
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::ModifyColumn {
                    col_name,
                    data_type,
                    options,
                    column_position,
                } => {
                    sqlparser::ast::AlterTableOperation::ModifyColumn {
                        col_name: col_name
                            .apply_transform_with_path(transformer, node_path)?,
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                        column_position: column_position
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::RenameConstraint {
                    old_name,
                    new_name,
                } => {
                    sqlparser::ast::AlterTableOperation::RenameConstraint {
                        old_name: old_name
                            .apply_transform_with_path(transformer, node_path)?,
                        new_name: new_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::AlterColumn { column_name, op } => {
                    sqlparser::ast::AlterTableOperation::AlterColumn {
                        column_name: column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        op: op.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::SwapWith { table_name } => {
                    sqlparser::ast::AlterTableOperation::SwapWith {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::SetTblProperties {
                    table_properties,
                } => {
                    sqlparser::ast::AlterTableOperation::SetTblProperties {
                        table_properties: table_properties
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::AlterTableOperation::OwnerTo { new_owner } => {
                    sqlparser::ast::AlterTableOperation::OwnerTo {
                        new_owner: new_owner
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AnalyzeFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AnalyzeFormat::TEXT => {
                    transformer
                        .transform(node_path, sqlparser::ast::AnalyzeFormat::TEXT)?
                }
                sqlparser::ast::AnalyzeFormat::GRAPHVIZ => {
                    transformer
                        .transform(node_path, sqlparser::ast::AnalyzeFormat::GRAPHVIZ)?
                }
                sqlparser::ast::AnalyzeFormat::JSON => {
                    transformer
                        .transform(node_path, sqlparser::ast::AnalyzeFormat::JSON)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ArgMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ArgMode::In => {
                    transformer.transform(node_path, sqlparser::ast::ArgMode::In)?
                }
                sqlparser::ast::ArgMode::Out => {
                    transformer.transform(node_path, sqlparser::ast::ArgMode::Out)?
                }
                sqlparser::ast::ArgMode::InOut => {
                    transformer.transform(node_path, sqlparser::ast::ArgMode::InOut)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Array {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { elem, named } = self;
            Self {
                elem: elem.apply_transform_with_path(transformer, node_path)?,
                named: named.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ArrayElemTypeDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ArrayElemTypeDef::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::ArrayElemTypeDef::None)?
                }
                sqlparser::ast::ArrayElemTypeDef::AngleBracket(field0) => {
                    sqlparser::ast::ArrayElemTypeDef::AngleBracket(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ArrayElemTypeDef::SquareBracket(field0, field1) => {
                    sqlparser::ast::ArrayElemTypeDef::SquareBracket(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ArrayElemTypeDef::Parenthesis(field0) => {
                    sqlparser::ast::ArrayElemTypeDef::Parenthesis(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Assignment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { target, value } = self;
            Self {
                target: target.apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AssignmentTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AssignmentTarget::ColumnName(field0) => {
                    sqlparser::ast::AssignmentTarget::ColumnName(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::AssignmentTarget::Tuple(field0) => {
                    sqlparser::ast::AssignmentTarget::Tuple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::AttachDuckDBDatabaseOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(field0) => {
                    sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::AttachDuckDBDatabaseOption::Type(field0) => {
                    sqlparser::ast::AttachDuckDBDatabaseOption::Type(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::BinaryOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::BinaryOperator::Plus => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Plus)?
                }
                sqlparser::ast::BinaryOperator::Minus => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Minus)?
                }
                sqlparser::ast::BinaryOperator::Multiply => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Multiply)?
                }
                sqlparser::ast::BinaryOperator::Divide => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Divide)?
                }
                sqlparser::ast::BinaryOperator::Modulo => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Modulo)?
                }
                sqlparser::ast::BinaryOperator::StringConcat => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::StringConcat,
                        )?
                }
                sqlparser::ast::BinaryOperator::Gt => {
                    transformer.transform(node_path, sqlparser::ast::BinaryOperator::Gt)?
                }
                sqlparser::ast::BinaryOperator::Lt => {
                    transformer.transform(node_path, sqlparser::ast::BinaryOperator::Lt)?
                }
                sqlparser::ast::BinaryOperator::GtEq => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::GtEq)?
                }
                sqlparser::ast::BinaryOperator::LtEq => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::LtEq)?
                }
                sqlparser::ast::BinaryOperator::Spaceship => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Spaceship)?
                }
                sqlparser::ast::BinaryOperator::Eq => {
                    transformer.transform(node_path, sqlparser::ast::BinaryOperator::Eq)?
                }
                sqlparser::ast::BinaryOperator::NotEq => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::NotEq)?
                }
                sqlparser::ast::BinaryOperator::And => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::And)?
                }
                sqlparser::ast::BinaryOperator::Or => {
                    transformer.transform(node_path, sqlparser::ast::BinaryOperator::Or)?
                }
                sqlparser::ast::BinaryOperator::Xor => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Xor)?
                }
                sqlparser::ast::BinaryOperator::BitwiseOr => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::BitwiseOr)?
                }
                sqlparser::ast::BinaryOperator::BitwiseAnd => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::BitwiseAnd,
                        )?
                }
                sqlparser::ast::BinaryOperator::BitwiseXor => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::BitwiseXor,
                        )?
                }
                sqlparser::ast::BinaryOperator::DuckIntegerDivide => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::DuckIntegerDivide,
                        )?
                }
                sqlparser::ast::BinaryOperator::MyIntegerDivide => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::MyIntegerDivide,
                        )?
                }
                sqlparser::ast::BinaryOperator::Custom(field0) => {
                    sqlparser::ast::BinaryOperator::Custom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::BinaryOperator::PGBitwiseXor => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGBitwiseXor,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGBitwiseShiftRight => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGBitwiseShiftRight,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGExp => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::PGExp)?
                }
                sqlparser::ast::BinaryOperator::PGOverlap => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::PGOverlap)?
                }
                sqlparser::ast::BinaryOperator::PGRegexMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGRegexMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGRegexIMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGRegexIMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGRegexNotMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGRegexNotMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGRegexNotIMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGRegexNotIMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGLikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGLikeMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGILikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGILikeMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGNotLikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGNotLikeMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGNotILikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGNotILikeMatch,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGStartsWith => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::PGStartsWith,
                        )?
                }
                sqlparser::ast::BinaryOperator::Arrow => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Arrow)?
                }
                sqlparser::ast::BinaryOperator::LongArrow => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::LongArrow)?
                }
                sqlparser::ast::BinaryOperator::HashArrow => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::HashArrow)?
                }
                sqlparser::ast::BinaryOperator::HashLongArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::HashLongArrow,
                        )?
                }
                sqlparser::ast::BinaryOperator::AtAt => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::AtAt)?
                }
                sqlparser::ast::BinaryOperator::AtArrow => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::AtArrow)?
                }
                sqlparser::ast::BinaryOperator::ArrowAt => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::ArrowAt)?
                }
                sqlparser::ast::BinaryOperator::HashMinus => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::HashMinus)?
                }
                sqlparser::ast::BinaryOperator::AtQuestion => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::AtQuestion,
                        )?
                }
                sqlparser::ast::BinaryOperator::Question => {
                    transformer
                        .transform(node_path, sqlparser::ast::BinaryOperator::Question)?
                }
                sqlparser::ast::BinaryOperator::QuestionAnd => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::QuestionAnd,
                        )?
                }
                sqlparser::ast::BinaryOperator::QuestionPipe => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::BinaryOperator::QuestionPipe,
                        )?
                }
                sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(field0) => {
                    sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CastFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CastFormat::Value(field0) => {
                    sqlparser::ast::CastFormat::Value(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CastFormat::ValueAtTimeZone(field0, field1) => {
                    sqlparser::ast::CastFormat::ValueAtTimeZone(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CastKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CastKind::Cast => {
                    transformer.transform(node_path, sqlparser::ast::CastKind::Cast)?
                }
                sqlparser::ast::CastKind::TryCast => {
                    transformer.transform(node_path, sqlparser::ast::CastKind::TryCast)?
                }
                sqlparser::ast::CastKind::SafeCast => {
                    transformer.transform(node_path, sqlparser::ast::CastKind::SafeCast)?
                }
                sqlparser::ast::CastKind::DoubleColon => {
                    transformer
                        .transform(node_path, sqlparser::ast::CastKind::DoubleColon)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CeilFloorKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CeilFloorKind::DateTimeField(field0) => {
                    sqlparser::ast::CeilFloorKind::DateTimeField(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CeilFloorKind::Scale(field0) => {
                    sqlparser::ast::CeilFloorKind::Scale(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CharLengthUnits {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CharLengthUnits::Characters => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CharLengthUnits::Characters,
                        )?
                }
                sqlparser::ast::CharLengthUnits::Octets => {
                    transformer
                        .transform(node_path, sqlparser::ast::CharLengthUnits::Octets)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CharacterLength {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CharacterLength::IntegerLength { length, unit } => {
                    sqlparser::ast::CharacterLength::IntegerLength {
                        length: length
                            .apply_transform_with_path(transformer, node_path)?,
                        unit: unit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::CharacterLength::Max => {
                    transformer
                        .transform(node_path, sqlparser::ast::CharacterLength::Max)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CloseCursor {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CloseCursor::All => {
                    transformer.transform(node_path, sqlparser::ast::CloseCursor::All)?
                }
                sqlparser::ast::CloseCursor::Specific { name } => {
                    sqlparser::ast::CloseCursor::Specific {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ClusteredBy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { columns, sorted_by, num_buckets } = self;
            Self {
                columns: columns.apply_transform_with_path(transformer, node_path)?,
                sorted_by: sorted_by.apply_transform_with_path(transformer, node_path)?,
                num_buckets: num_buckets
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ClusteredIndex {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, asc } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                asc: asc.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, data_type, collation, options } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                data_type: data_type.apply_transform_with_path(transformer, node_path)?,
                collation: collation.apply_transform_with_path(transformer, node_path)?,
                options: options.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ColumnOption::Null => {
                    transformer.transform(node_path, sqlparser::ast::ColumnOption::Null)?
                }
                sqlparser::ast::ColumnOption::NotNull => {
                    transformer
                        .transform(node_path, sqlparser::ast::ColumnOption::NotNull)?
                }
                sqlparser::ast::ColumnOption::Default(field0) => {
                    sqlparser::ast::ColumnOption::Default(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Materialized(field0) => {
                    sqlparser::ast::ColumnOption::Materialized(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Ephemeral(field0) => {
                    sqlparser::ast::ColumnOption::Ephemeral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Alias(field0) => {
                    sqlparser::ast::ColumnOption::Alias(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Unique { is_primary, characteristics } => {
                    sqlparser::ast::ColumnOption::Unique {
                        is_primary: is_primary
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::ColumnOption::ForeignKey {
                    foreign_table,
                    referred_columns,
                    on_delete,
                    on_update,
                    characteristics,
                } => {
                    sqlparser::ast::ColumnOption::ForeignKey {
                        foreign_table: foreign_table
                            .apply_transform_with_path(transformer, node_path)?,
                        referred_columns: referred_columns
                            .apply_transform_with_path(transformer, node_path)?,
                        on_delete: on_delete
                            .apply_transform_with_path(transformer, node_path)?,
                        on_update: on_update
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::ColumnOption::Check(field0) => {
                    sqlparser::ast::ColumnOption::Check(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::DialectSpecific(field0) => {
                    sqlparser::ast::ColumnOption::DialectSpecific(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::CharacterSet(field0) => {
                    sqlparser::ast::ColumnOption::CharacterSet(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Comment(field0) => {
                    sqlparser::ast::ColumnOption::Comment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::OnUpdate(field0) => {
                    sqlparser::ast::ColumnOption::OnUpdate(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Generated {
                    generated_as,
                    sequence_options,
                    generation_expr,
                    generation_expr_mode,
                    generated_keyword,
                } => {
                    sqlparser::ast::ColumnOption::Generated {
                        generated_as: generated_as
                            .apply_transform_with_path(transformer, node_path)?,
                        sequence_options: sequence_options
                            .apply_transform_with_path(transformer, node_path)?,
                        generation_expr: generation_expr
                            .apply_transform_with_path(transformer, node_path)?,
                        generation_expr_mode: generation_expr_mode
                            .apply_transform_with_path(transformer, node_path)?,
                        generated_keyword: generated_keyword
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::ColumnOption::Options(field0) => {
                    sqlparser::ast::ColumnOption::Options(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Identity(field0) => {
                    sqlparser::ast::ColumnOption::Identity(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::OnConflict(field0) => {
                    sqlparser::ast::ColumnOption::OnConflict(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Policy(field0) => {
                    sqlparser::ast::ColumnOption::Policy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnOption::Tags(field0) => {
                    sqlparser::ast::ColumnOption::Tags(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnOptionDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, option } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                option: option.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnPolicy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ColumnPolicy::MaskingPolicy(field0) => {
                    sqlparser::ast::ColumnPolicy::MaskingPolicy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ColumnPolicy::ProjectionPolicy(field0) => {
                    sqlparser::ast::ColumnPolicy::ProjectionPolicy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ColumnPolicyProperty {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { with, policy_name, using_columns } = self;
            Self {
                with: with.apply_transform_with_path(transformer, node_path)?,
                policy_name: policy_name
                    .apply_transform_with_path(transformer, node_path)?,
                using_columns: using_columns
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CommentDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CommentDef::WithEq(field0) => {
                    sqlparser::ast::CommentDef::WithEq(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CommentDef::WithoutEq(field0) => {
                    sqlparser::ast::CommentDef::WithoutEq(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(field0) => {
                    sqlparser::ast::CommentDef::AfterColumnDefsWithoutEq(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CommentObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CommentObject::Column => {
                    transformer
                        .transform(node_path, sqlparser::ast::CommentObject::Column)?
                }
                sqlparser::ast::CommentObject::Table => {
                    transformer
                        .transform(node_path, sqlparser::ast::CommentObject::Table)?
                }
                sqlparser::ast::CommentObject::Extension => {
                    transformer
                        .transform(node_path, sqlparser::ast::CommentObject::Extension)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConflictTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ConflictTarget::Columns(field0) => {
                    sqlparser::ast::ConflictTarget::Columns(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ConflictTarget::OnConstraint(field0) => {
                    sqlparser::ast::ConflictTarget::OnConstraint(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConnectBy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { condition, relationships } = self;
            Self {
                condition: condition.apply_transform_with_path(transformer, node_path)?,
                relationships: relationships
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ConstraintCharacteristics {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { deferrable, initially, enforced } = self;
            Self {
                deferrable: deferrable
                    .apply_transform_with_path(transformer, node_path)?,
                initially: initially.apply_transform_with_path(transformer, node_path)?,
                enforced: enforced.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ContextModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ContextModifier::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::ContextModifier::None)?
                }
                sqlparser::ast::ContextModifier::Local => {
                    transformer
                        .transform(node_path, sqlparser::ast::ContextModifier::Local)?
                }
                sqlparser::ast::ContextModifier::Session => {
                    transformer
                        .transform(node_path, sqlparser::ast::ContextModifier::Session)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyLegacyCsvOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CopyLegacyCsvOption::Header => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CopyLegacyCsvOption::Header,
                        )?
                }
                sqlparser::ast::CopyLegacyCsvOption::Quote(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::Quote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyLegacyCsvOption::Escape(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::Escape(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyLegacyCsvOption::ForceQuote(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::ForceQuote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(field0) => {
                    sqlparser::ast::CopyLegacyCsvOption::ForceNotNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyLegacyOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CopyLegacyOption::Binary => {
                    transformer
                        .transform(node_path, sqlparser::ast::CopyLegacyOption::Binary)?
                }
                sqlparser::ast::CopyLegacyOption::Delimiter(field0) => {
                    sqlparser::ast::CopyLegacyOption::Delimiter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyLegacyOption::Null(field0) => {
                    sqlparser::ast::CopyLegacyOption::Null(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyLegacyOption::Csv(field0) => {
                    sqlparser::ast::CopyLegacyOption::Csv(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CopyOption::Format(field0) => {
                    sqlparser::ast::CopyOption::Format(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Freeze(field0) => {
                    sqlparser::ast::CopyOption::Freeze(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Delimiter(field0) => {
                    sqlparser::ast::CopyOption::Delimiter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Null(field0) => {
                    sqlparser::ast::CopyOption::Null(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Header(field0) => {
                    sqlparser::ast::CopyOption::Header(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Quote(field0) => {
                    sqlparser::ast::CopyOption::Quote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Escape(field0) => {
                    sqlparser::ast::CopyOption::Escape(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::ForceQuote(field0) => {
                    sqlparser::ast::CopyOption::ForceQuote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::ForceNotNull(field0) => {
                    sqlparser::ast::CopyOption::ForceNotNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::ForceNull(field0) => {
                    sqlparser::ast::CopyOption::ForceNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CopyOption::Encoding(field0) => {
                    sqlparser::ast::CopyOption::Encoding(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopySource {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CopySource::Table { table_name, columns } => {
                    sqlparser::ast::CopySource::Table {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::CopySource::Query(field0) => {
                    sqlparser::ast::CopySource::Query(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CopyTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CopyTarget::Stdin => {
                    transformer.transform(node_path, sqlparser::ast::CopyTarget::Stdin)?
                }
                sqlparser::ast::CopyTarget::Stdout => {
                    transformer.transform(node_path, sqlparser::ast::CopyTarget::Stdout)?
                }
                sqlparser::ast::CopyTarget::File { filename } => {
                    sqlparser::ast::CopyTarget::File {
                        filename: filename
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::CopyTarget::Program { command } => {
                    sqlparser::ast::CopyTarget::Program {
                        command: command
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateFunctionBody {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CreateFunctionBody::AsBeforeOptions(field0) => {
                    sqlparser::ast::CreateFunctionBody::AsBeforeOptions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CreateFunctionBody::AsAfterOptions(field0) => {
                    sqlparser::ast::CreateFunctionBody::AsAfterOptions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CreateFunctionBody::Return(field0) => {
                    sqlparser::ast::CreateFunctionBody::Return(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateFunctionUsing {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CreateFunctionUsing::Jar(field0) => {
                    sqlparser::ast::CreateFunctionUsing::Jar(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CreateFunctionUsing::File(field0) => {
                    sqlparser::ast::CreateFunctionUsing::File(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CreateFunctionUsing::Archive(field0) => {
                    sqlparser::ast::CreateFunctionUsing::Archive(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateIndex {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                name: name.apply_transform_with_path(transformer, node_path)?,
                table_name: table_name
                    .apply_transform_with_path(transformer, node_path)?,
                using: using.apply_transform_with_path(transformer, node_path)?,
                columns: columns.apply_transform_with_path(transformer, node_path)?,
                unique: unique.apply_transform_with_path(transformer, node_path)?,
                concurrently: concurrently
                    .apply_transform_with_path(transformer, node_path)?,
                if_not_exists: if_not_exists
                    .apply_transform_with_path(transformer, node_path)?,
                include: include.apply_transform_with_path(transformer, node_path)?,
                nulls_distinct: nulls_distinct
                    .apply_transform_with_path(transformer, node_path)?,
                with: with.apply_transform_with_path(transformer, node_path)?,
                predicate: predicate.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreatePolicyCommand {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CreatePolicyCommand::All => {
                    transformer
                        .transform(node_path, sqlparser::ast::CreatePolicyCommand::All)?
                }
                sqlparser::ast::CreatePolicyCommand::Select => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CreatePolicyCommand::Select,
                        )?
                }
                sqlparser::ast::CreatePolicyCommand::Insert => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CreatePolicyCommand::Insert,
                        )?
                }
                sqlparser::ast::CreatePolicyCommand::Update => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CreatePolicyCommand::Update,
                        )?
                }
                sqlparser::ast::CreatePolicyCommand::Delete => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CreatePolicyCommand::Delete,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreatePolicyType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CreatePolicyType::Permissive => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CreatePolicyType::Permissive,
                        )?
                }
                sqlparser::ast::CreatePolicyType::Restrictive => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CreatePolicyType::Restrictive,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateTable {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                or_replace: or_replace
                    .apply_transform_with_path(transformer, node_path)?,
                temporary: temporary.apply_transform_with_path(transformer, node_path)?,
                external: external.apply_transform_with_path(transformer, node_path)?,
                global: global.apply_transform_with_path(transformer, node_path)?,
                if_not_exists: if_not_exists
                    .apply_transform_with_path(transformer, node_path)?,
                transient: transient.apply_transform_with_path(transformer, node_path)?,
                volatile: volatile.apply_transform_with_path(transformer, node_path)?,
                name: name.apply_transform_with_path(transformer, node_path)?,
                columns: columns.apply_transform_with_path(transformer, node_path)?,
                constraints: constraints
                    .apply_transform_with_path(transformer, node_path)?,
                hive_distribution: hive_distribution
                    .apply_transform_with_path(transformer, node_path)?,
                hive_formats: hive_formats
                    .apply_transform_with_path(transformer, node_path)?,
                table_properties: table_properties
                    .apply_transform_with_path(transformer, node_path)?,
                with_options: with_options
                    .apply_transform_with_path(transformer, node_path)?,
                file_format: file_format
                    .apply_transform_with_path(transformer, node_path)?,
                location: location.apply_transform_with_path(transformer, node_path)?,
                query: query.apply_transform_with_path(transformer, node_path)?,
                without_rowid: without_rowid
                    .apply_transform_with_path(transformer, node_path)?,
                like: like.apply_transform_with_path(transformer, node_path)?,
                clone: clone.apply_transform_with_path(transformer, node_path)?,
                engine: engine.apply_transform_with_path(transformer, node_path)?,
                comment: comment.apply_transform_with_path(transformer, node_path)?,
                auto_increment_offset: auto_increment_offset
                    .apply_transform_with_path(transformer, node_path)?,
                default_charset: default_charset
                    .apply_transform_with_path(transformer, node_path)?,
                collation: collation.apply_transform_with_path(transformer, node_path)?,
                on_commit: on_commit.apply_transform_with_path(transformer, node_path)?,
                on_cluster: on_cluster
                    .apply_transform_with_path(transformer, node_path)?,
                primary_key: primary_key
                    .apply_transform_with_path(transformer, node_path)?,
                order_by: order_by.apply_transform_with_path(transformer, node_path)?,
                partition_by: partition_by
                    .apply_transform_with_path(transformer, node_path)?,
                cluster_by: cluster_by
                    .apply_transform_with_path(transformer, node_path)?,
                clustered_by: clustered_by
                    .apply_transform_with_path(transformer, node_path)?,
                options: options.apply_transform_with_path(transformer, node_path)?,
                strict: strict.apply_transform_with_path(transformer, node_path)?,
                copy_grants: copy_grants
                    .apply_transform_with_path(transformer, node_path)?,
                enable_schema_evolution: enable_schema_evolution
                    .apply_transform_with_path(transformer, node_path)?,
                change_tracking: change_tracking
                    .apply_transform_with_path(transformer, node_path)?,
                data_retention_time_in_days: data_retention_time_in_days
                    .apply_transform_with_path(transformer, node_path)?,
                max_data_extension_time_in_days: max_data_extension_time_in_days
                    .apply_transform_with_path(transformer, node_path)?,
                default_ddl_collation: default_ddl_collation
                    .apply_transform_with_path(transformer, node_path)?,
                with_aggregation_policy: with_aggregation_policy
                    .apply_transform_with_path(transformer, node_path)?,
                with_row_access_policy: with_row_access_policy
                    .apply_transform_with_path(transformer, node_path)?,
                with_tags: with_tags.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CreateTableOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CreateTableOptions::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::CreateTableOptions::None)?
                }
                sqlparser::ast::CreateTableOptions::With(field0) => {
                    sqlparser::ast::CreateTableOptions::With(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::CreateTableOptions::Options(field0) => {
                    sqlparser::ast::CreateTableOptions::Options(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Cte {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { alias, query, from, materialized } = self;
            Self {
                alias: alias.apply_transform_with_path(transformer, node_path)?,
                query: query.apply_transform_with_path(transformer, node_path)?,
                from: from.apply_transform_with_path(transformer, node_path)?,
                materialized: materialized
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::CteAsMaterialized {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::CteAsMaterialized::Materialized => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CteAsMaterialized::Materialized,
                        )?
                }
                sqlparser::ast::CteAsMaterialized::NotMaterialized => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::CteAsMaterialized::NotMaterialized,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DataType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DataType::Character(field0) => {
                    sqlparser::ast::DataType::Character(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Char(field0) => {
                    sqlparser::ast::DataType::Char(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::CharacterVarying(field0) => {
                    sqlparser::ast::DataType::CharacterVarying(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::CharVarying(field0) => {
                    sqlparser::ast::DataType::CharVarying(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Varchar(field0) => {
                    sqlparser::ast::DataType::Varchar(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Nvarchar(field0) => {
                    sqlparser::ast::DataType::Nvarchar(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Uuid => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Uuid)?
                }
                sqlparser::ast::DataType::CharacterLargeObject(field0) => {
                    sqlparser::ast::DataType::CharacterLargeObject(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::CharLargeObject(field0) => {
                    sqlparser::ast::DataType::CharLargeObject(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Clob(field0) => {
                    sqlparser::ast::DataType::Clob(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Binary(field0) => {
                    sqlparser::ast::DataType::Binary(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Varbinary(field0) => {
                    sqlparser::ast::DataType::Varbinary(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Blob(field0) => {
                    sqlparser::ast::DataType::Blob(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Bytes(field0) => {
                    sqlparser::ast::DataType::Bytes(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Numeric(field0) => {
                    sqlparser::ast::DataType::Numeric(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Decimal(field0) => {
                    sqlparser::ast::DataType::Decimal(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::BigNumeric(field0) => {
                    sqlparser::ast::DataType::BigNumeric(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::BigDecimal(field0) => {
                    sqlparser::ast::DataType::BigDecimal(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Dec(field0) => {
                    sqlparser::ast::DataType::Dec(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Float(field0) => {
                    sqlparser::ast::DataType::Float(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::TinyInt(field0) => {
                    sqlparser::ast::DataType::TinyInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedTinyInt(field0) => {
                    sqlparser::ast::DataType::UnsignedTinyInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Int2(field0) => {
                    sqlparser::ast::DataType::Int2(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInt2(field0) => {
                    sqlparser::ast::DataType::UnsignedInt2(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::SmallInt(field0) => {
                    sqlparser::ast::DataType::SmallInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedSmallInt(field0) => {
                    sqlparser::ast::DataType::UnsignedSmallInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::MediumInt(field0) => {
                    sqlparser::ast::DataType::MediumInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedMediumInt(field0) => {
                    sqlparser::ast::DataType::UnsignedMediumInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Int(field0) => {
                    sqlparser::ast::DataType::Int(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Int4(field0) => {
                    sqlparser::ast::DataType::Int4(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Int8(field0) => {
                    sqlparser::ast::DataType::Int8(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Int16 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Int16)?
                }
                sqlparser::ast::DataType::Int32 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Int32)?
                }
                sqlparser::ast::DataType::Int64 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Int64)?
                }
                sqlparser::ast::DataType::Int128 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Int128)?
                }
                sqlparser::ast::DataType::Int256 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Int256)?
                }
                sqlparser::ast::DataType::Integer(field0) => {
                    sqlparser::ast::DataType::Integer(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInt(field0) => {
                    sqlparser::ast::DataType::UnsignedInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInt4(field0) => {
                    sqlparser::ast::DataType::UnsignedInt4(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInteger(field0) => {
                    sqlparser::ast::DataType::UnsignedInteger(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UInt8 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::UInt8)?
                }
                sqlparser::ast::DataType::UInt16 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::UInt16)?
                }
                sqlparser::ast::DataType::UInt32 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::UInt32)?
                }
                sqlparser::ast::DataType::UInt64 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::UInt64)?
                }
                sqlparser::ast::DataType::UInt128 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::UInt128)?
                }
                sqlparser::ast::DataType::UInt256 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::UInt256)?
                }
                sqlparser::ast::DataType::BigInt(field0) => {
                    sqlparser::ast::DataType::BigInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedBigInt(field0) => {
                    sqlparser::ast::DataType::UnsignedBigInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::UnsignedInt8(field0) => {
                    sqlparser::ast::DataType::UnsignedInt8(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Float4 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Float4)?
                }
                sqlparser::ast::DataType::Float32 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Float32)?
                }
                sqlparser::ast::DataType::Float64 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Float64)?
                }
                sqlparser::ast::DataType::Real => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Real)?
                }
                sqlparser::ast::DataType::Float8 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Float8)?
                }
                sqlparser::ast::DataType::Double => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Double)?
                }
                sqlparser::ast::DataType::DoublePrecision => {
                    transformer
                        .transform(node_path, sqlparser::ast::DataType::DoublePrecision)?
                }
                sqlparser::ast::DataType::Bool => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Bool)?
                }
                sqlparser::ast::DataType::Boolean => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Boolean)?
                }
                sqlparser::ast::DataType::Date => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Date)?
                }
                sqlparser::ast::DataType::Date32 => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Date32)?
                }
                sqlparser::ast::DataType::Time(field0, field1) => {
                    sqlparser::ast::DataType::Time(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Datetime(field0) => {
                    sqlparser::ast::DataType::Datetime(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Datetime64(field0, field1) => {
                    sqlparser::ast::DataType::Datetime64(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Timestamp(field0, field1) => {
                    sqlparser::ast::DataType::Timestamp(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Interval => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Interval)?
                }
                sqlparser::ast::DataType::JSON => {
                    transformer.transform(node_path, sqlparser::ast::DataType::JSON)?
                }
                sqlparser::ast::DataType::JSONB => {
                    transformer.transform(node_path, sqlparser::ast::DataType::JSONB)?
                }
                sqlparser::ast::DataType::Regclass => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Regclass)?
                }
                sqlparser::ast::DataType::Text => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Text)?
                }
                sqlparser::ast::DataType::String(field0) => {
                    sqlparser::ast::DataType::String(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::FixedString(field0) => {
                    sqlparser::ast::DataType::FixedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Bytea => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Bytea)?
                }
                sqlparser::ast::DataType::Custom(field0, field1) => {
                    sqlparser::ast::DataType::Custom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Array(field0) => {
                    sqlparser::ast::DataType::Array(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Map(field0, field1) => {
                    sqlparser::ast::DataType::Map(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Tuple(field0) => {
                    sqlparser::ast::DataType::Tuple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Nested(field0) => {
                    sqlparser::ast::DataType::Nested(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Enum(field0) => {
                    sqlparser::ast::DataType::Enum(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Set(field0) => {
                    sqlparser::ast::DataType::Set(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Struct(field0, field1) => {
                    sqlparser::ast::DataType::Struct(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Union(field0) => {
                    sqlparser::ast::DataType::Union(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Nullable(field0) => {
                    sqlparser::ast::DataType::Nullable(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::LowCardinality(field0) => {
                    sqlparser::ast::DataType::LowCardinality(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DataType::Unspecified => {
                    transformer
                        .transform(node_path, sqlparser::ast::DataType::Unspecified)?
                }
                sqlparser::ast::DataType::Trigger => {
                    transformer.transform(node_path, sqlparser::ast::DataType::Trigger)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DateTimeField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DateTimeField::Year => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Year)?
                }
                sqlparser::ast::DateTimeField::Month => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Month)?
                }
                sqlparser::ast::DateTimeField::Week(field0) => {
                    sqlparser::ast::DateTimeField::Week(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DateTimeField::Day => {
                    transformer.transform(node_path, sqlparser::ast::DateTimeField::Day)?
                }
                sqlparser::ast::DateTimeField::DayOfWeek => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::DayOfWeek)?
                }
                sqlparser::ast::DateTimeField::DayOfYear => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::DayOfYear)?
                }
                sqlparser::ast::DateTimeField::Date => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Date)?
                }
                sqlparser::ast::DateTimeField::Datetime => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Datetime)?
                }
                sqlparser::ast::DateTimeField::Hour => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Hour)?
                }
                sqlparser::ast::DateTimeField::Minute => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Minute)?
                }
                sqlparser::ast::DateTimeField::Second => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Second)?
                }
                sqlparser::ast::DateTimeField::Century => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Century)?
                }
                sqlparser::ast::DateTimeField::Decade => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Decade)?
                }
                sqlparser::ast::DateTimeField::Dow => {
                    transformer.transform(node_path, sqlparser::ast::DateTimeField::Dow)?
                }
                sqlparser::ast::DateTimeField::Doy => {
                    transformer.transform(node_path, sqlparser::ast::DateTimeField::Doy)?
                }
                sqlparser::ast::DateTimeField::Epoch => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Epoch)?
                }
                sqlparser::ast::DateTimeField::Isodow => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Isodow)?
                }
                sqlparser::ast::DateTimeField::IsoWeek => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::IsoWeek)?
                }
                sqlparser::ast::DateTimeField::Isoyear => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Isoyear)?
                }
                sqlparser::ast::DateTimeField::Julian => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Julian)?
                }
                sqlparser::ast::DateTimeField::Microsecond => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::Microsecond,
                        )?
                }
                sqlparser::ast::DateTimeField::Microseconds => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::Microseconds,
                        )?
                }
                sqlparser::ast::DateTimeField::Millenium => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Millenium)?
                }
                sqlparser::ast::DateTimeField::Millennium => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Millennium)?
                }
                sqlparser::ast::DateTimeField::Millisecond => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::Millisecond,
                        )?
                }
                sqlparser::ast::DateTimeField::Milliseconds => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::Milliseconds,
                        )?
                }
                sqlparser::ast::DateTimeField::Nanosecond => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Nanosecond)?
                }
                sqlparser::ast::DateTimeField::Nanoseconds => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::Nanoseconds,
                        )?
                }
                sqlparser::ast::DateTimeField::Quarter => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Quarter)?
                }
                sqlparser::ast::DateTimeField::Time => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Time)?
                }
                sqlparser::ast::DateTimeField::Timezone => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::Timezone)?
                }
                sqlparser::ast::DateTimeField::TimezoneAbbr => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::TimezoneAbbr,
                        )?
                }
                sqlparser::ast::DateTimeField::TimezoneHour => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::TimezoneHour,
                        )?
                }
                sqlparser::ast::DateTimeField::TimezoneMinute => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::TimezoneMinute,
                        )?
                }
                sqlparser::ast::DateTimeField::TimezoneRegion => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DateTimeField::TimezoneRegion,
                        )?
                }
                sqlparser::ast::DateTimeField::NoDateTime => {
                    transformer
                        .transform(node_path, sqlparser::ast::DateTimeField::NoDateTime)?
                }
                sqlparser::ast::DateTimeField::Custom(field0) => {
                    sqlparser::ast::DateTimeField::Custom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Declare {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                names: names.apply_transform_with_path(transformer, node_path)?,
                data_type: data_type.apply_transform_with_path(transformer, node_path)?,
                assignment: assignment
                    .apply_transform_with_path(transformer, node_path)?,
                declare_type: declare_type
                    .apply_transform_with_path(transformer, node_path)?,
                binary: binary.apply_transform_with_path(transformer, node_path)?,
                sensitive: sensitive.apply_transform_with_path(transformer, node_path)?,
                scroll: scroll.apply_transform_with_path(transformer, node_path)?,
                hold: hold.apply_transform_with_path(transformer, node_path)?,
                for_query: for_query.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeclareAssignment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DeclareAssignment::Expr(field0) => {
                    sqlparser::ast::DeclareAssignment::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::Default(field0) => {
                    sqlparser::ast::DeclareAssignment::Default(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::DuckAssignment(field0) => {
                    sqlparser::ast::DeclareAssignment::DuckAssignment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::For(field0) => {
                    sqlparser::ast::DeclareAssignment::For(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::DeclareAssignment::MsSqlAssignment(field0) => {
                    sqlparser::ast::DeclareAssignment::MsSqlAssignment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeclareType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DeclareType::Cursor => {
                    transformer
                        .transform(node_path, sqlparser::ast::DeclareType::Cursor)?
                }
                sqlparser::ast::DeclareType::ResultSet => {
                    transformer
                        .transform(node_path, sqlparser::ast::DeclareType::ResultSet)?
                }
                sqlparser::ast::DeclareType::Exception => {
                    transformer
                        .transform(node_path, sqlparser::ast::DeclareType::Exception)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Deduplicate {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Deduplicate::All => {
                    transformer.transform(node_path, sqlparser::ast::Deduplicate::All)?
                }
                sqlparser::ast::Deduplicate::ByExpression(field0) => {
                    sqlparser::ast::Deduplicate::ByExpression(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DeferrableInitial {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DeferrableInitial::Immediate => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DeferrableInitial::Immediate,
                        )?
                }
                sqlparser::ast::DeferrableInitial::Deferred => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DeferrableInitial::Deferred,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Delete {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { tables, from, using, selection, returning, order_by, limit } = self;
            Self {
                tables: tables.apply_transform_with_path(transformer, node_path)?,
                from: from.apply_transform_with_path(transformer, node_path)?,
                using: using.apply_transform_with_path(transformer, node_path)?,
                selection: selection.apply_transform_with_path(transformer, node_path)?,
                returning: returning.apply_transform_with_path(transformer, node_path)?,
                order_by: order_by.apply_transform_with_path(transformer, node_path)?,
                limit: limit.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DescribeAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DescribeAlias::Describe => {
                    transformer
                        .transform(node_path, sqlparser::ast::DescribeAlias::Describe)?
                }
                sqlparser::ast::DescribeAlias::Explain => {
                    transformer
                        .transform(node_path, sqlparser::ast::DescribeAlias::Explain)?
                }
                sqlparser::ast::DescribeAlias::Desc => {
                    transformer
                        .transform(node_path, sqlparser::ast::DescribeAlias::Desc)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DictionaryField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DiscardObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DiscardObject::ALL => {
                    transformer.transform(node_path, sqlparser::ast::DiscardObject::ALL)?
                }
                sqlparser::ast::DiscardObject::PLANS => {
                    transformer
                        .transform(node_path, sqlparser::ast::DiscardObject::PLANS)?
                }
                sqlparser::ast::DiscardObject::SEQUENCES => {
                    transformer
                        .transform(node_path, sqlparser::ast::DiscardObject::SEQUENCES)?
                }
                sqlparser::ast::DiscardObject::TEMP => {
                    transformer
                        .transform(node_path, sqlparser::ast::DiscardObject::TEMP)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Distinct {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Distinct::Distinct => {
                    transformer.transform(node_path, sqlparser::ast::Distinct::Distinct)?
                }
                sqlparser::ast::Distinct::On(field0) => {
                    sqlparser::ast::Distinct::On(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DoUpdate {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { assignments, selection } = self;
            Self {
                assignments: assignments
                    .apply_transform_with_path(transformer, node_path)?,
                selection: selection.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DollarQuotedString {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { value, tag } = self;
            Self {
                value: value.apply_transform_with_path(transformer, node_path)?,
                tag: tag.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::DuplicateTreatment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::DuplicateTreatment::Distinct => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::DuplicateTreatment::Distinct,
                        )?
                }
                sqlparser::ast::DuplicateTreatment::All => {
                    transformer
                        .transform(node_path, sqlparser::ast::DuplicateTreatment::All)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::EmptyMatchesMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::EmptyMatchesMode::Show => {
                    transformer
                        .transform(node_path, sqlparser::ast::EmptyMatchesMode::Show)?
                }
                sqlparser::ast::EmptyMatchesMode::Omit => {
                    transformer
                        .transform(node_path, sqlparser::ast::EmptyMatchesMode::Omit)?
                }
                sqlparser::ast::EmptyMatchesMode::WithUnmatched => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::EmptyMatchesMode::WithUnmatched,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExactNumberInfo {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ExactNumberInfo::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::ExactNumberInfo::None)?
                }
                sqlparser::ast::ExactNumberInfo::Precision(field0) => {
                    sqlparser::ast::ExactNumberInfo::Precision(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ExactNumberInfo::PrecisionAndScale(field0, field1) => {
                    sqlparser::ast::ExactNumberInfo::PrecisionAndScale(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExceptSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { first_element, additional_elements } = self;
            Self {
                first_element: first_element
                    .apply_transform_with_path(transformer, node_path)?,
                additional_elements: additional_elements
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExcludeSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ExcludeSelectItem::Single(field0) => {
                    sqlparser::ast::ExcludeSelectItem::Single(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ExcludeSelectItem::Multiple(field0) => {
                    sqlparser::ast::ExcludeSelectItem::Multiple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Expr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Expr::Identifier(field0) => {
                    sqlparser::ast::Expr::Identifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::CompoundIdentifier(field0) => {
                    sqlparser::ast::Expr::CompoundIdentifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::JsonAccess { value, path } => {
                    sqlparser::ast::Expr::JsonAccess {
                        value: value.apply_transform_with_path(transformer, node_path)?,
                        path: path.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::CompositeAccess { expr, key } => {
                    sqlparser::ast::Expr::CompositeAccess {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        key: key.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::IsFalse(field0) => {
                    sqlparser::ast::Expr::IsFalse(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsNotFalse(field0) => {
                    sqlparser::ast::Expr::IsNotFalse(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsTrue(field0) => {
                    sqlparser::ast::Expr::IsTrue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsNotTrue(field0) => {
                    sqlparser::ast::Expr::IsNotTrue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsNull(field0) => {
                    sqlparser::ast::Expr::IsNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsNotNull(field0) => {
                    sqlparser::ast::Expr::IsNotNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsUnknown(field0) => {
                    sqlparser::ast::Expr::IsUnknown(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsNotUnknown(field0) => {
                    sqlparser::ast::Expr::IsNotUnknown(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsDistinctFrom(field0, field1) => {
                    sqlparser::ast::Expr::IsDistinctFrom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IsNotDistinctFrom(field0, field1) => {
                    sqlparser::ast::Expr::IsNotDistinctFrom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::InList { expr, list, negated } => {
                    sqlparser::ast::Expr::InList {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        list: list.apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::InSubquery { expr, subquery, negated } => {
                    sqlparser::ast::Expr::InSubquery {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        subquery: subquery
                            .apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::InUnnest { expr, array_expr, negated } => {
                    sqlparser::ast::Expr::InUnnest {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        array_expr: array_expr
                            .apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Between { expr, negated, low, high } => {
                    sqlparser::ast::Expr::Between {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        low: low.apply_transform_with_path(transformer, node_path)?,
                        high: high.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::BinaryOp { left, op, right } => {
                    sqlparser::ast::Expr::BinaryOp {
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        op: op.apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Like {
                    negated,
                    any,
                    expr,
                    pattern,
                    escape_char,
                } => {
                    sqlparser::ast::Expr::Like {
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        any: any.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        escape_char: escape_char
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::ILike {
                    negated,
                    any,
                    expr,
                    pattern,
                    escape_char,
                } => {
                    sqlparser::ast::Expr::ILike {
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        any: any.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        escape_char: escape_char
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::SimilarTo {
                    negated,
                    expr,
                    pattern,
                    escape_char,
                } => {
                    sqlparser::ast::Expr::SimilarTo {
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        escape_char: escape_char
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::RLike { negated, expr, pattern, regexp } => {
                    sqlparser::ast::Expr::RLike {
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        regexp: regexp.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::AnyOp { left, compare_op, right, is_some } => {
                    sqlparser::ast::Expr::AnyOp {
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        compare_op: compare_op
                            .apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                        is_some: is_some
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::AllOp { left, compare_op, right } => {
                    sqlparser::ast::Expr::AllOp {
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        compare_op: compare_op
                            .apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::UnaryOp { op, expr } => {
                    sqlparser::ast::Expr::UnaryOp {
                        op: op.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Convert {
                    is_try,
                    expr,
                    data_type,
                    charset,
                    target_before_value,
                    styles,
                } => {
                    sqlparser::ast::Expr::Convert {
                        is_try: is_try
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        charset: charset
                            .apply_transform_with_path(transformer, node_path)?,
                        target_before_value: target_before_value
                            .apply_transform_with_path(transformer, node_path)?,
                        styles: styles.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Cast { kind, expr, data_type, format } => {
                    sqlparser::ast::Expr::Cast {
                        kind: kind.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        format: format.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::AtTimeZone { timestamp, time_zone } => {
                    sqlparser::ast::Expr::AtTimeZone {
                        timestamp: timestamp
                            .apply_transform_with_path(transformer, node_path)?,
                        time_zone: time_zone
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Extract { field, syntax, expr } => {
                    sqlparser::ast::Expr::Extract {
                        field: field.apply_transform_with_path(transformer, node_path)?,
                        syntax: syntax
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Ceil { expr, field } => {
                    sqlparser::ast::Expr::Ceil {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        field: field.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Floor { expr, field } => {
                    sqlparser::ast::Expr::Floor {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        field: field.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Position { expr, r#in } => {
                    sqlparser::ast::Expr::Position {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        r#in: r#in.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Substring {
                    expr,
                    substring_from,
                    substring_for,
                    special,
                } => {
                    sqlparser::ast::Expr::Substring {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        substring_from: substring_from
                            .apply_transform_with_path(transformer, node_path)?,
                        substring_for: substring_for
                            .apply_transform_with_path(transformer, node_path)?,
                        special: special
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Trim {
                    expr,
                    trim_where,
                    trim_what,
                    trim_characters,
                } => {
                    sqlparser::ast::Expr::Trim {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        trim_where: trim_where
                            .apply_transform_with_path(transformer, node_path)?,
                        trim_what: trim_what
                            .apply_transform_with_path(transformer, node_path)?,
                        trim_characters: trim_characters
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Overlay {
                    expr,
                    overlay_what,
                    overlay_from,
                    overlay_for,
                } => {
                    sqlparser::ast::Expr::Overlay {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        overlay_what: overlay_what
                            .apply_transform_with_path(transformer, node_path)?,
                        overlay_from: overlay_from
                            .apply_transform_with_path(transformer, node_path)?,
                        overlay_for: overlay_for
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Collate { expr, collation } => {
                    sqlparser::ast::Expr::Collate {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        collation: collation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Nested(field0) => {
                    sqlparser::ast::Expr::Nested(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Value(field0) => {
                    sqlparser::ast::Expr::Value(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::IntroducedString { introducer, value } => {
                    sqlparser::ast::Expr::IntroducedString {
                        introducer: introducer
                            .apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::TypedString { data_type, value } => {
                    sqlparser::ast::Expr::TypedString {
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::MapAccess { column, keys } => {
                    sqlparser::ast::Expr::MapAccess {
                        column: column
                            .apply_transform_with_path(transformer, node_path)?,
                        keys: keys.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Function(field0) => {
                    sqlparser::ast::Expr::Function(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Case {
                    operand,
                    conditions,
                    results,
                    else_result,
                } => {
                    sqlparser::ast::Expr::Case {
                        operand: operand
                            .apply_transform_with_path(transformer, node_path)?,
                        conditions: conditions
                            .apply_transform_with_path(transformer, node_path)?,
                        results: results
                            .apply_transform_with_path(transformer, node_path)?,
                        else_result: else_result
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Exists { subquery, negated } => {
                    sqlparser::ast::Expr::Exists {
                        subquery: subquery
                            .apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Subquery(field0) => {
                    sqlparser::ast::Expr::Subquery(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::GroupingSets(field0) => {
                    sqlparser::ast::Expr::GroupingSets(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Cube(field0) => {
                    sqlparser::ast::Expr::Cube(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Rollup(field0) => {
                    sqlparser::ast::Expr::Rollup(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Tuple(field0) => {
                    sqlparser::ast::Expr::Tuple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Struct { values, fields } => {
                    sqlparser::ast::Expr::Struct {
                        values: values
                            .apply_transform_with_path(transformer, node_path)?,
                        fields: fields.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Named { expr, name } => {
                    sqlparser::ast::Expr::Named {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Dictionary(field0) => {
                    sqlparser::ast::Expr::Dictionary(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Map(field0) => {
                    sqlparser::ast::Expr::Map(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Subscript { expr, subscript } => {
                    sqlparser::ast::Expr::Subscript {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        subscript: subscript
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Array(field0) => {
                    sqlparser::ast::Expr::Array(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Interval(field0) => {
                    sqlparser::ast::Expr::Interval(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::MatchAgainst {
                    columns,
                    match_value,
                    opt_search_modifier,
                } => {
                    sqlparser::ast::Expr::MatchAgainst {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        match_value: match_value
                            .apply_transform_with_path(transformer, node_path)?,
                        opt_search_modifier: opt_search_modifier
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Expr::Wildcard => {
                    transformer.transform(node_path, sqlparser::ast::Expr::Wildcard)?
                }
                sqlparser::ast::Expr::QualifiedWildcard(field0) => {
                    sqlparser::ast::Expr::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::OuterJoin(field0) => {
                    sqlparser::ast::Expr::OuterJoin(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Prior(field0) => {
                    sqlparser::ast::Expr::Prior(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Expr::Lambda(field0) => {
                    sqlparser::ast::Expr::Lambda(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExprWithAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { expr, alias } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, node_path)?,
                alias: alias.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ExtractSyntax {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ExtractSyntax::From => {
                    transformer
                        .transform(node_path, sqlparser::ast::ExtractSyntax::From)?
                }
                sqlparser::ast::ExtractSyntax::Comma => {
                    transformer
                        .transform(node_path, sqlparser::ast::ExtractSyntax::Comma)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Fetch {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { with_ties, percent, quantity } = self;
            Self {
                with_ties: with_ties.apply_transform_with_path(transformer, node_path)?,
                percent: percent.apply_transform_with_path(transformer, node_path)?,
                quantity: quantity.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FetchDirection {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FetchDirection::Count { limit } => {
                    sqlparser::ast::FetchDirection::Count {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::FetchDirection::Next => {
                    transformer
                        .transform(node_path, sqlparser::ast::FetchDirection::Next)?
                }
                sqlparser::ast::FetchDirection::Prior => {
                    transformer
                        .transform(node_path, sqlparser::ast::FetchDirection::Prior)?
                }
                sqlparser::ast::FetchDirection::First => {
                    transformer
                        .transform(node_path, sqlparser::ast::FetchDirection::First)?
                }
                sqlparser::ast::FetchDirection::Last => {
                    transformer
                        .transform(node_path, sqlparser::ast::FetchDirection::Last)?
                }
                sqlparser::ast::FetchDirection::Absolute { limit } => {
                    sqlparser::ast::FetchDirection::Absolute {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::FetchDirection::Relative { limit } => {
                    sqlparser::ast::FetchDirection::Relative {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::FetchDirection::All => {
                    transformer
                        .transform(node_path, sqlparser::ast::FetchDirection::All)?
                }
                sqlparser::ast::FetchDirection::Forward { limit } => {
                    sqlparser::ast::FetchDirection::Forward {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::FetchDirection::ForwardAll => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FetchDirection::ForwardAll,
                        )?
                }
                sqlparser::ast::FetchDirection::Backward { limit } => {
                    sqlparser::ast::FetchDirection::Backward {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::FetchDirection::BackwardAll => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FetchDirection::BackwardAll,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FileFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FileFormat::TEXTFILE => {
                    transformer
                        .transform(node_path, sqlparser::ast::FileFormat::TEXTFILE)?
                }
                sqlparser::ast::FileFormat::SEQUENCEFILE => {
                    transformer
                        .transform(node_path, sqlparser::ast::FileFormat::SEQUENCEFILE)?
                }
                sqlparser::ast::FileFormat::ORC => {
                    transformer.transform(node_path, sqlparser::ast::FileFormat::ORC)?
                }
                sqlparser::ast::FileFormat::PARQUET => {
                    transformer
                        .transform(node_path, sqlparser::ast::FileFormat::PARQUET)?
                }
                sqlparser::ast::FileFormat::AVRO => {
                    transformer.transform(node_path, sqlparser::ast::FileFormat::AVRO)?
                }
                sqlparser::ast::FileFormat::RCFILE => {
                    transformer.transform(node_path, sqlparser::ast::FileFormat::RCFILE)?
                }
                sqlparser::ast::FileFormat::JSONFILE => {
                    transformer
                        .transform(node_path, sqlparser::ast::FileFormat::JSONFILE)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FlushLocation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FlushLocation::NoWriteToBinlog => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FlushLocation::NoWriteToBinlog,
                        )?
                }
                sqlparser::ast::FlushLocation::Local => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushLocation::Local)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FlushType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FlushType::BinaryLogs => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::BinaryLogs)?
                }
                sqlparser::ast::FlushType::EngineLogs => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::EngineLogs)?
                }
                sqlparser::ast::FlushType::ErrorLogs => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::ErrorLogs)?
                }
                sqlparser::ast::FlushType::GeneralLogs => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::GeneralLogs)?
                }
                sqlparser::ast::FlushType::Hosts => {
                    transformer.transform(node_path, sqlparser::ast::FlushType::Hosts)?
                }
                sqlparser::ast::FlushType::Logs => {
                    transformer.transform(node_path, sqlparser::ast::FlushType::Logs)?
                }
                sqlparser::ast::FlushType::Privileges => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::Privileges)?
                }
                sqlparser::ast::FlushType::OptimizerCosts => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::OptimizerCosts)?
                }
                sqlparser::ast::FlushType::RelayLogs => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::RelayLogs)?
                }
                sqlparser::ast::FlushType::SlowLogs => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::SlowLogs)?
                }
                sqlparser::ast::FlushType::Status => {
                    transformer.transform(node_path, sqlparser::ast::FlushType::Status)?
                }
                sqlparser::ast::FlushType::UserResources => {
                    transformer
                        .transform(node_path, sqlparser::ast::FlushType::UserResources)?
                }
                sqlparser::ast::FlushType::Tables => {
                    transformer.transform(node_path, sqlparser::ast::FlushType::Tables)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ForClause::Browse => {
                    transformer.transform(node_path, sqlparser::ast::ForClause::Browse)?
                }
                sqlparser::ast::ForClause::Json {
                    for_json,
                    root,
                    include_null_values,
                    without_array_wrapper,
                } => {
                    sqlparser::ast::ForClause::Json {
                        for_json: for_json
                            .apply_transform_with_path(transformer, node_path)?,
                        root: root.apply_transform_with_path(transformer, node_path)?,
                        include_null_values: include_null_values
                            .apply_transform_with_path(transformer, node_path)?,
                        without_array_wrapper: without_array_wrapper
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::ForClause::Xml {
                    for_xml,
                    elements,
                    binary_base64,
                    root,
                    r#type,
                } => {
                    sqlparser::ast::ForClause::Xml {
                        for_xml: for_xml
                            .apply_transform_with_path(transformer, node_path)?,
                        elements: elements
                            .apply_transform_with_path(transformer, node_path)?,
                        binary_base64: binary_base64
                            .apply_transform_with_path(transformer, node_path)?,
                        root: root.apply_transform_with_path(transformer, node_path)?,
                        r#type: r#type.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForJson {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ForJson::Auto => {
                    transformer.transform(node_path, sqlparser::ast::ForJson::Auto)?
                }
                sqlparser::ast::ForJson::Path => {
                    transformer.transform(node_path, sqlparser::ast::ForJson::Path)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ForXml {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ForXml::Raw(field0) => {
                    sqlparser::ast::ForXml::Raw(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ForXml::Auto => {
                    transformer.transform(node_path, sqlparser::ast::ForXml::Auto)?
                }
                sqlparser::ast::ForXml::Explicit => {
                    transformer.transform(node_path, sqlparser::ast::ForXml::Explicit)?
                }
                sqlparser::ast::ForXml::Path(field0) => {
                    sqlparser::ast::ForXml::Path(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FormatClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FormatClause::Identifier(field0) => {
                    sqlparser::ast::FormatClause::Identifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FormatClause::Null => {
                    transformer.transform(node_path, sqlparser::ast::FormatClause::Null)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FromTable {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FromTable::WithFromKeyword(field0) => {
                    sqlparser::ast::FromTable::WithFromKeyword(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FromTable::WithoutKeyword(field0) => {
                    sqlparser::ast::FromTable::WithoutKeyword(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Function {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                name: name.apply_transform_with_path(transformer, node_path)?,
                parameters: parameters
                    .apply_transform_with_path(transformer, node_path)?,
                args: args.apply_transform_with_path(transformer, node_path)?,
                filter: filter.apply_transform_with_path(transformer, node_path)?,
                null_treatment: null_treatment
                    .apply_transform_with_path(transformer, node_path)?,
                over: over.apply_transform_with_path(transformer, node_path)?,
                within_group: within_group
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArg {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArg::Named { name, arg, operator } => {
                    sqlparser::ast::FunctionArg::Named {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        arg: arg.apply_transform_with_path(transformer, node_path)?,
                        operator: operator
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::FunctionArg::Unnamed(field0) => {
                    sqlparser::ast::FunctionArg::Unnamed(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArgExpr::Expr(field0) => {
                    sqlparser::ast::FunctionArgExpr::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgExpr::QualifiedWildcard(field0) => {
                    sqlparser::ast::FunctionArgExpr::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgExpr::Wildcard => {
                    transformer
                        .transform(node_path, sqlparser::ast::FunctionArgExpr::Wildcard)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArgOperator::Equals => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionArgOperator::Equals,
                        )?
                }
                sqlparser::ast::FunctionArgOperator::RightArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionArgOperator::RightArrow,
                        )?
                }
                sqlparser::ast::FunctionArgOperator::Assignment => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionArgOperator::Assignment,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgumentClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(field0) => {
                    sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::OrderBy(field0) => {
                    sqlparser::ast::FunctionArgumentClause::OrderBy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::Limit(field0) => {
                    sqlparser::ast::FunctionArgumentClause::Limit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::OnOverflow(field0) => {
                    sqlparser::ast::FunctionArgumentClause::OnOverflow(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::Having(field0) => {
                    sqlparser::ast::FunctionArgumentClause::Having(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArgumentClause::Separator(field0) => {
                    sqlparser::ast::FunctionArgumentClause::Separator(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArgumentList {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { duplicate_treatment, args, clauses } = self;
            Self {
                duplicate_treatment: duplicate_treatment
                    .apply_transform_with_path(transformer, node_path)?,
                args: args.apply_transform_with_path(transformer, node_path)?,
                clauses: clauses.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionArguments {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionArguments::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::FunctionArguments::None)?
                }
                sqlparser::ast::FunctionArguments::Subquery(field0) => {
                    sqlparser::ast::FunctionArguments::Subquery(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::FunctionArguments::List(field0) => {
                    sqlparser::ast::FunctionArguments::List(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionBehavior {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionBehavior::Immutable => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionBehavior::Immutable,
                        )?
                }
                sqlparser::ast::FunctionBehavior::Stable => {
                    transformer
                        .transform(node_path, sqlparser::ast::FunctionBehavior::Stable)?
                }
                sqlparser::ast::FunctionBehavior::Volatile => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionBehavior::Volatile,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionCalledOnNull {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput,
                        )?
                }
                sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput,
                        )?
                }
                sqlparser::ast::FunctionCalledOnNull::Strict => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionCalledOnNull::Strict,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionDesc {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, args } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                args: args.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionDeterminismSpecifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionDeterminismSpecifier::Deterministic => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionDeterminismSpecifier::Deterministic,
                        )?
                }
                sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionDeterminismSpecifier::NotDeterministic,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::FunctionParallel {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::FunctionParallel::Unsafe => {
                    transformer
                        .transform(node_path, sqlparser::ast::FunctionParallel::Unsafe)?
                }
                sqlparser::ast::FunctionParallel::Restricted => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::FunctionParallel::Restricted,
                        )?
                }
                sqlparser::ast::FunctionParallel::Safe => {
                    transformer
                        .transform(node_path, sqlparser::ast::FunctionParallel::Safe)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GeneratedAs {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::GeneratedAs::Always => {
                    transformer
                        .transform(node_path, sqlparser::ast::GeneratedAs::Always)?
                }
                sqlparser::ast::GeneratedAs::ByDefault => {
                    transformer
                        .transform(node_path, sqlparser::ast::GeneratedAs::ByDefault)?
                }
                sqlparser::ast::GeneratedAs::ExpStored => {
                    transformer
                        .transform(node_path, sqlparser::ast::GeneratedAs::ExpStored)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GeneratedExpressionMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::GeneratedExpressionMode::Virtual => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::GeneratedExpressionMode::Virtual,
                        )?
                }
                sqlparser::ast::GeneratedExpressionMode::Stored => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::GeneratedExpressionMode::Stored,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GrantObjects {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::GrantObjects::AllSequencesInSchema { schemas } => {
                    sqlparser::ast::GrantObjects::AllSequencesInSchema {
                        schemas: schemas
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::GrantObjects::AllTablesInSchema { schemas } => {
                    sqlparser::ast::GrantObjects::AllTablesInSchema {
                        schemas: schemas
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::GrantObjects::Schemas(field0) => {
                    sqlparser::ast::GrantObjects::Schemas(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::GrantObjects::Sequences(field0) => {
                    sqlparser::ast::GrantObjects::Sequences(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::GrantObjects::Tables(field0) => {
                    sqlparser::ast::GrantObjects::Tables(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GroupByExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::GroupByExpr::All(field0) => {
                    sqlparser::ast::GroupByExpr::All(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::GroupByExpr::Expressions(field0, field1) => {
                    sqlparser::ast::GroupByExpr::Expressions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::GroupByWithModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::GroupByWithModifier::Rollup => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::GroupByWithModifier::Rollup,
                        )?
                }
                sqlparser::ast::GroupByWithModifier::Cube => {
                    transformer
                        .transform(node_path, sqlparser::ast::GroupByWithModifier::Cube)?
                }
                sqlparser::ast::GroupByWithModifier::Totals => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::GroupByWithModifier::Totals,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HavingBound {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self(field0, field1) = self;
            Self(
                field0.apply_transform_with_path(transformer, node_path)?,
                field1.apply_transform_with_path(transformer, node_path)?,
            )
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HavingBoundKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::HavingBoundKind::Min => {
                    transformer
                        .transform(node_path, sqlparser::ast::HavingBoundKind::Min)?
                }
                sqlparser::ast::HavingBoundKind::Max => {
                    transformer
                        .transform(node_path, sqlparser::ast::HavingBoundKind::Max)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDelimiter {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::HiveDelimiter::FieldsTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDelimiter::FieldsTerminatedBy,
                        )?
                }
                sqlparser::ast::HiveDelimiter::FieldsEscapedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDelimiter::FieldsEscapedBy,
                        )?
                }
                sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy,
                        )?
                }
                sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy,
                        )?
                }
                sqlparser::ast::HiveDelimiter::LinesTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDelimiter::LinesTerminatedBy,
                        )?
                }
                sqlparser::ast::HiveDelimiter::NullDefinedAs => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDelimiter::NullDefinedAs,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDescribeFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::HiveDescribeFormat::Extended => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDescribeFormat::Extended,
                        )?
                }
                sqlparser::ast::HiveDescribeFormat::Formatted => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDescribeFormat::Formatted,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveDistributionStyle {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::HiveDistributionStyle::PARTITIONED { columns } => {
                    sqlparser::ast::HiveDistributionStyle::PARTITIONED {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::HiveDistributionStyle::SKEWED {
                    columns,
                    on,
                    stored_as_directories,
                } => {
                    sqlparser::ast::HiveDistributionStyle::SKEWED {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        on: on.apply_transform_with_path(transformer, node_path)?,
                        stored_as_directories: stored_as_directories
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::HiveDistributionStyle::NONE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::HiveDistributionStyle::NONE,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { row_format, serde_properties, storage, location } = self;
            Self {
                row_format: row_format
                    .apply_transform_with_path(transformer, node_path)?,
                serde_properties: serde_properties
                    .apply_transform_with_path(transformer, node_path)?,
                storage: storage.apply_transform_with_path(transformer, node_path)?,
                location: location.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveIOFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::HiveIOFormat::IOF { input_format, output_format } => {
                    sqlparser::ast::HiveIOFormat::IOF {
                        input_format: input_format
                            .apply_transform_with_path(transformer, node_path)?,
                        output_format: output_format
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::HiveIOFormat::FileFormat { format } => {
                    sqlparser::ast::HiveIOFormat::FileFormat {
                        format: format.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveRowDelimiter {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { delimiter, char } = self;
            Self {
                delimiter: delimiter.apply_transform_with_path(transformer, node_path)?,
                char: char.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveRowFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::HiveRowFormat::SERDE { class } => {
                    sqlparser::ast::HiveRowFormat::SERDE {
                        class: class.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::HiveRowFormat::DELIMITED { delimiters } => {
                    sqlparser::ast::HiveRowFormat::DELIMITED {
                        delimiters: delimiters
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::HiveSetLocation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { has_set, location } = self;
            Self {
                has_set: has_set.apply_transform_with_path(transformer, node_path)?,
                location: location.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Ident {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { value, quote_style } = self;
            Self {
                value: value.apply_transform_with_path(transformer, node_path)?,
                quote_style: quote_style
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentWithAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { ident, alias } = self;
            Self {
                ident: ident.apply_transform_with_path(transformer, node_path)?,
                alias: alias.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityParameters {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { seed, increment } = self;
            Self {
                seed: seed.apply_transform_with_path(transformer, node_path)?,
                increment: increment.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityProperty {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { parameters, order } = self;
            Self {
                parameters: parameters
                    .apply_transform_with_path(transformer, node_path)?,
                order: order.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyFormatKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(field0) => {
                    sqlparser::ast::IdentityPropertyFormatKind::FunctionCall(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(field0) => {
                    sqlparser::ast::IdentityPropertyFormatKind::StartAndIncrement(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::IdentityPropertyKind::Autoincrement(field0) => {
                    sqlparser::ast::IdentityPropertyKind::Autoincrement(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::IdentityPropertyKind::Identity(field0) => {
                    sqlparser::ast::IdentityPropertyKind::Identity(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IdentityPropertyOrder {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::IdentityPropertyOrder::Order => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::IdentityPropertyOrder::Order,
                        )?
                }
                sqlparser::ast::IdentityPropertyOrder::NoOrder => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::IdentityPropertyOrder::NoOrder,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IlikeSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { pattern } = self;
            Self {
                pattern: pattern.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IndexOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::IndexOption::Using(field0) => {
                    sqlparser::ast::IndexOption::Using(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::IndexOption::Comment(field0) => {
                    sqlparser::ast::IndexOption::Comment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::IndexType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::IndexType::BTree => {
                    transformer.transform(node_path, sqlparser::ast::IndexType::BTree)?
                }
                sqlparser::ast::IndexType::Hash => {
                    transformer.transform(node_path, sqlparser::ast::IndexType::Hash)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Insert {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                or: or.apply_transform_with_path(transformer, node_path)?,
                ignore: ignore.apply_transform_with_path(transformer, node_path)?,
                into: into.apply_transform_with_path(transformer, node_path)?,
                table_name: table_name
                    .apply_transform_with_path(transformer, node_path)?,
                table_alias: table_alias
                    .apply_transform_with_path(transformer, node_path)?,
                columns: columns.apply_transform_with_path(transformer, node_path)?,
                overwrite: overwrite.apply_transform_with_path(transformer, node_path)?,
                source: source.apply_transform_with_path(transformer, node_path)?,
                partitioned: partitioned
                    .apply_transform_with_path(transformer, node_path)?,
                after_columns: after_columns
                    .apply_transform_with_path(transformer, node_path)?,
                table: table.apply_transform_with_path(transformer, node_path)?,
                on: on.apply_transform_with_path(transformer, node_path)?,
                returning: returning.apply_transform_with_path(transformer, node_path)?,
                replace_into: replace_into
                    .apply_transform_with_path(transformer, node_path)?,
                priority: priority.apply_transform_with_path(transformer, node_path)?,
                insert_alias: insert_alias
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::InsertAliases {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { row_alias, col_aliases } = self;
            Self {
                row_alias: row_alias.apply_transform_with_path(transformer, node_path)?,
                col_aliases: col_aliases
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Interpolate {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { exprs } = self;
            Self {
                exprs: exprs.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::InterpolateExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { column, expr } = self;
            Self {
                column: column.apply_transform_with_path(transformer, node_path)?,
                expr: expr.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Interval {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self {
                value,
                leading_field,
                leading_precision,
                last_field,
                fractional_seconds_precision,
            } = self;
            Self {
                value: value.apply_transform_with_path(transformer, node_path)?,
                leading_field: leading_field
                    .apply_transform_with_path(transformer, node_path)?,
                leading_precision: leading_precision
                    .apply_transform_with_path(transformer, node_path)?,
                last_field: last_field
                    .apply_transform_with_path(transformer, node_path)?,
                fractional_seconds_precision: fractional_seconds_precision
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Join {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { relation, global, join_operator } = self;
            Self {
                relation: relation.apply_transform_with_path(transformer, node_path)?,
                global: global.apply_transform_with_path(transformer, node_path)?,
                join_operator: join_operator
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JoinConstraint {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::JoinConstraint::On(field0) => {
                    sqlparser::ast::JoinConstraint::On(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinConstraint::Using(field0) => {
                    sqlparser::ast::JoinConstraint::Using(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinConstraint::Natural => {
                    transformer
                        .transform(node_path, sqlparser::ast::JoinConstraint::Natural)?
                }
                sqlparser::ast::JoinConstraint::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::JoinConstraint::None)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JoinOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::JoinOperator::Inner(field0) => {
                    sqlparser::ast::JoinOperator::Inner(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::LeftOuter(field0) => {
                    sqlparser::ast::JoinOperator::LeftOuter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::RightOuter(field0) => {
                    sqlparser::ast::JoinOperator::RightOuter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::FullOuter(field0) => {
                    sqlparser::ast::JoinOperator::FullOuter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::CrossJoin => {
                    transformer
                        .transform(node_path, sqlparser::ast::JoinOperator::CrossJoin)?
                }
                sqlparser::ast::JoinOperator::LeftSemi(field0) => {
                    sqlparser::ast::JoinOperator::LeftSemi(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::RightSemi(field0) => {
                    sqlparser::ast::JoinOperator::RightSemi(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::LeftAnti(field0) => {
                    sqlparser::ast::JoinOperator::LeftAnti(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::RightAnti(field0) => {
                    sqlparser::ast::JoinOperator::RightAnti(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JoinOperator::CrossApply => {
                    transformer
                        .transform(node_path, sqlparser::ast::JoinOperator::CrossApply)?
                }
                sqlparser::ast::JoinOperator::OuterApply => {
                    transformer
                        .transform(node_path, sqlparser::ast::JoinOperator::OuterApply)?
                }
                sqlparser::ast::JoinOperator::AsOf { match_condition, constraint } => {
                    sqlparser::ast::JoinOperator::AsOf {
                        match_condition: match_condition
                            .apply_transform_with_path(transformer, node_path)?,
                        constraint: constraint
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonPath {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { path } = self;
            Self {
                path: path.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonPathElem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::JsonPathElem::Dot { key, quoted } => {
                    sqlparser::ast::JsonPathElem::Dot {
                        key: key.apply_transform_with_path(transformer, node_path)?,
                        quoted: quoted.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::JsonPathElem::Bracket { key } => {
                    sqlparser::ast::JsonPathElem::Bracket {
                        key: key.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::JsonTableColumn::Named(field0) => {
                    sqlparser::ast::JsonTableColumn::Named(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JsonTableColumn::ForOrdinality(field0) => {
                    sqlparser::ast::JsonTableColumn::ForOrdinality(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JsonTableColumn::Nested(field0) => {
                    sqlparser::ast::JsonTableColumn::Nested(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableColumnErrorHandling {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::JsonTableColumnErrorHandling::Null => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::JsonTableColumnErrorHandling::Null,
                        )?
                }
                sqlparser::ast::JsonTableColumnErrorHandling::Default(field0) => {
                    sqlparser::ast::JsonTableColumnErrorHandling::Default(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::JsonTableColumnErrorHandling::Error => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::JsonTableColumnErrorHandling::Error,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableNamedColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, r#type, path, exists, on_empty, on_error } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                r#type: r#type.apply_transform_with_path(transformer, node_path)?,
                path: path.apply_transform_with_path(transformer, node_path)?,
                exists: exists.apply_transform_with_path(transformer, node_path)?,
                on_empty: on_empty.apply_transform_with_path(transformer, node_path)?,
                on_error: on_error.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::JsonTableNestedColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { path, columns } = self;
            Self {
                path: path.apply_transform_with_path(transformer, node_path)?,
                columns: columns.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::KeyOrIndexDisplay {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::KeyOrIndexDisplay::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::KeyOrIndexDisplay::None)?
                }
                sqlparser::ast::KeyOrIndexDisplay::Key => {
                    transformer
                        .transform(node_path, sqlparser::ast::KeyOrIndexDisplay::Key)?
                }
                sqlparser::ast::KeyOrIndexDisplay::Index => {
                    transformer
                        .transform(node_path, sqlparser::ast::KeyOrIndexDisplay::Index)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::KillType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::KillType::Connection => {
                    transformer
                        .transform(node_path, sqlparser::ast::KillType::Connection)?
                }
                sqlparser::ast::KillType::Query => {
                    transformer.transform(node_path, sqlparser::ast::KillType::Query)?
                }
                sqlparser::ast::KillType::Mutation => {
                    transformer.transform(node_path, sqlparser::ast::KillType::Mutation)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LambdaFunction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { params, body } = self;
            Self {
                params: params.apply_transform_with_path(transformer, node_path)?,
                body: body.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LateralView {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { lateral_view, lateral_view_name, lateral_col_alias, outer } = self;
            Self {
                lateral_view: lateral_view
                    .apply_transform_with_path(transformer, node_path)?,
                lateral_view_name: lateral_view_name
                    .apply_transform_with_path(transformer, node_path)?,
                lateral_col_alias: lateral_col_alias
                    .apply_transform_with_path(transformer, node_path)?,
                outer: outer.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ListAggOnOverflow {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ListAggOnOverflow::Error => {
                    transformer
                        .transform(node_path, sqlparser::ast::ListAggOnOverflow::Error)?
                }
                sqlparser::ast::ListAggOnOverflow::Truncate { filler, with_count } => {
                    sqlparser::ast::ListAggOnOverflow::Truncate {
                        filler: filler
                            .apply_transform_with_path(transformer, node_path)?,
                        with_count: with_count
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { lock_type, of, nonblock } = self;
            Self {
                lock_type: lock_type.apply_transform_with_path(transformer, node_path)?,
                of: of.apply_transform_with_path(transformer, node_path)?,
                nonblock: nonblock.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockTable {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { table, alias, lock_type } = self;
            Self {
                table: table.apply_transform_with_path(transformer, node_path)?,
                alias: alias.apply_transform_with_path(transformer, node_path)?,
                lock_type: lock_type.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockTableType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::LockTableType::Read { local } => {
                    sqlparser::ast::LockTableType::Read {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::LockTableType::Write { low_priority } => {
                    sqlparser::ast::LockTableType::Write {
                        low_priority: low_priority
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::LockType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::LockType::Share => {
                    transformer.transform(node_path, sqlparser::ast::LockType::Share)?
                }
                sqlparser::ast::LockType::Update => {
                    transformer.transform(node_path, sqlparser::ast::LockType::Update)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MacroArg {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, default_expr } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                default_expr: default_expr
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MacroDefinition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MacroDefinition::Expr(field0) => {
                    sqlparser::ast::MacroDefinition::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MacroDefinition::Table(field0) => {
                    sqlparser::ast::MacroDefinition::Table(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Map {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { entries } = self;
            Self {
                entries: entries.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapAccessKey {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { key, syntax } = self;
            Self {
                key: key.apply_transform_with_path(transformer, node_path)?,
                syntax: syntax.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapAccessSyntax {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MapAccessSyntax::Bracket => {
                    transformer
                        .transform(node_path, sqlparser::ast::MapAccessSyntax::Bracket)?
                }
                sqlparser::ast::MapAccessSyntax::Period => {
                    transformer
                        .transform(node_path, sqlparser::ast::MapAccessSyntax::Period)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MapEntry {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MatchRecognizePattern {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MatchRecognizePattern::Symbol(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Symbol(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Exclude(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Exclude(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Permute(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Permute(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Concat(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Concat(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Group(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Group(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Alternation(field0) => {
                    sqlparser::ast::MatchRecognizePattern::Alternation(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizePattern::Repetition(field0, field1) => {
                    sqlparser::ast::MatchRecognizePattern::Repetition(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MatchRecognizeSymbol {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MatchRecognizeSymbol::Named(field0) => {
                    sqlparser::ast::MatchRecognizeSymbol::Named(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MatchRecognizeSymbol::Start => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MatchRecognizeSymbol::Start,
                        )?
                }
                sqlparser::ast::MatchRecognizeSymbol::End => {
                    transformer
                        .transform(node_path, sqlparser::ast::MatchRecognizeSymbol::End)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Measure {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { expr, alias } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, node_path)?,
                alias: alias.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeAction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MergeAction::Insert(field0) => {
                    sqlparser::ast::MergeAction::Insert(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MergeAction::Update { assignments } => {
                    sqlparser::ast::MergeAction::Update {
                        assignments: assignments
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::MergeAction::Delete => {
                    transformer
                        .transform(node_path, sqlparser::ast::MergeAction::Delete)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { clause_kind, predicate, action } = self;
            Self {
                clause_kind: clause_kind
                    .apply_transform_with_path(transformer, node_path)?,
                predicate: predicate.apply_transform_with_path(transformer, node_path)?,
                action: action.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeClauseKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MergeClauseKind::Matched => {
                    transformer
                        .transform(node_path, sqlparser::ast::MergeClauseKind::Matched)?
                }
                sqlparser::ast::MergeClauseKind::NotMatched => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MergeClauseKind::NotMatched,
                        )?
                }
                sqlparser::ast::MergeClauseKind::NotMatchedByTarget => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MergeClauseKind::NotMatchedByTarget,
                        )?
                }
                sqlparser::ast::MergeClauseKind::NotMatchedBySource => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MergeClauseKind::NotMatchedBySource,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeInsertExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { columns, kind } = self;
            Self {
                columns: columns.apply_transform_with_path(transformer, node_path)?,
                kind: kind.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MergeInsertKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MergeInsertKind::Values(field0) => {
                    sqlparser::ast::MergeInsertKind::Values(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::MergeInsertKind::Row => {
                    transformer
                        .transform(node_path, sqlparser::ast::MergeInsertKind::Row)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MinMaxValue {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MinMaxValue::Empty => {
                    transformer.transform(node_path, sqlparser::ast::MinMaxValue::Empty)?
                }
                sqlparser::ast::MinMaxValue::None => {
                    transformer.transform(node_path, sqlparser::ast::MinMaxValue::None)?
                }
                sqlparser::ast::MinMaxValue::Some(field0) => {
                    sqlparser::ast::MinMaxValue::Some(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MySQLColumnPosition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MySQLColumnPosition::First => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MySQLColumnPosition::First,
                        )?
                }
                sqlparser::ast::MySQLColumnPosition::After(field0) => {
                    sqlparser::ast::MySQLColumnPosition::After(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::MysqlInsertPriority {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::MysqlInsertPriority::LowPriority => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MysqlInsertPriority::LowPriority,
                        )?
                }
                sqlparser::ast::MysqlInsertPriority::Delayed => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MysqlInsertPriority::Delayed,
                        )?
                }
                sqlparser::ast::MysqlInsertPriority::HighPriority => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::MysqlInsertPriority::HighPriority,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NamedWindowDefinition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self(field0, field1) = self;
            Self(
                field0.apply_transform_with_path(transformer, node_path)?,
                field1.apply_transform_with_path(transformer, node_path)?,
            )
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NamedWindowExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::NamedWindowExpr::NamedWindow(field0) => {
                    sqlparser::ast::NamedWindowExpr::NamedWindow(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::NamedWindowExpr::WindowSpec(field0) => {
                    sqlparser::ast::NamedWindowExpr::WindowSpec(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NonBlock {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::NonBlock::Nowait => {
                    transformer.transform(node_path, sqlparser::ast::NonBlock::Nowait)?
                }
                sqlparser::ast::NonBlock::SkipLocked => {
                    transformer
                        .transform(node_path, sqlparser::ast::NonBlock::SkipLocked)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::NullTreatment {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::NullTreatment::IgnoreNulls => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::NullTreatment::IgnoreNulls,
                        )?
                }
                sqlparser::ast::NullTreatment::RespectNulls => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::NullTreatment::RespectNulls,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ObjectName {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self(field0) = self;
            Self(field0.apply_transform_with_path(transformer, node_path)?)
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ObjectType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ObjectType::Table => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::Table)?
                }
                sqlparser::ast::ObjectType::View => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::View)?
                }
                sqlparser::ast::ObjectType::Index => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::Index)?
                }
                sqlparser::ast::ObjectType::Schema => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::Schema)?
                }
                sqlparser::ast::ObjectType::Database => {
                    transformer
                        .transform(node_path, sqlparser::ast::ObjectType::Database)?
                }
                sqlparser::ast::ObjectType::Role => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::Role)?
                }
                sqlparser::ast::ObjectType::Sequence => {
                    transformer
                        .transform(node_path, sqlparser::ast::ObjectType::Sequence)?
                }
                sqlparser::ast::ObjectType::Stage => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::Stage)?
                }
                sqlparser::ast::ObjectType::Type => {
                    transformer.transform(node_path, sqlparser::ast::ObjectType::Type)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Offset {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { value, rows } = self;
            Self {
                value: value.apply_transform_with_path(transformer, node_path)?,
                rows: rows.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OffsetRows {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::OffsetRows::None => {
                    transformer.transform(node_path, sqlparser::ast::OffsetRows::None)?
                }
                sqlparser::ast::OffsetRows::Row => {
                    transformer.transform(node_path, sqlparser::ast::OffsetRows::Row)?
                }
                sqlparser::ast::OffsetRows::Rows => {
                    transformer.transform(node_path, sqlparser::ast::OffsetRows::Rows)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnCommit {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::OnCommit::DeleteRows => {
                    transformer
                        .transform(node_path, sqlparser::ast::OnCommit::DeleteRows)?
                }
                sqlparser::ast::OnCommit::PreserveRows => {
                    transformer
                        .transform(node_path, sqlparser::ast::OnCommit::PreserveRows)?
                }
                sqlparser::ast::OnCommit::Drop => {
                    transformer.transform(node_path, sqlparser::ast::OnCommit::Drop)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnConflict {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { conflict_target, action } = self;
            Self {
                conflict_target: conflict_target
                    .apply_transform_with_path(transformer, node_path)?,
                action: action.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnConflictAction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::OnConflictAction::DoNothing => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::OnConflictAction::DoNothing,
                        )?
                }
                sqlparser::ast::OnConflictAction::DoUpdate(field0) => {
                    sqlparser::ast::OnConflictAction::DoUpdate(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OnInsert {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::OnInsert::DuplicateKeyUpdate(field0) => {
                    sqlparser::ast::OnInsert::DuplicateKeyUpdate(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::OnInsert::OnConflict(field0) => {
                    sqlparser::ast::OnInsert::OnConflict(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                _ => unreachable!(),
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OperateFunctionArg {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { mode, name, data_type, default_expr } = self;
            Self {
                mode: mode.apply_transform_with_path(transformer, node_path)?,
                name: name.apply_transform_with_path(transformer, node_path)?,
                data_type: data_type.apply_transform_with_path(transformer, node_path)?,
                default_expr: default_expr
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OrderBy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { exprs, interpolate } = self;
            Self {
                exprs: exprs.apply_transform_with_path(transformer, node_path)?,
                interpolate: interpolate
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::OrderByExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { expr, asc, nulls_first, with_fill } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, node_path)?,
                asc: asc.apply_transform_with_path(transformer, node_path)?,
                nulls_first: nulls_first
                    .apply_transform_with_path(transformer, node_path)?,
                with_fill: with_fill.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Owner {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Owner::Ident(field0) => {
                    sqlparser::ast::Owner::Ident(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Owner::CurrentRole => {
                    transformer.transform(node_path, sqlparser::ast::Owner::CurrentRole)?
                }
                sqlparser::ast::Owner::CurrentUser => {
                    transformer.transform(node_path, sqlparser::ast::Owner::CurrentUser)?
                }
                sqlparser::ast::Owner::SessionUser => {
                    transformer.transform(node_path, sqlparser::ast::Owner::SessionUser)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Partition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Partition::Identifier(field0) => {
                    sqlparser::ast::Partition::Identifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Partition::Expr(field0) => {
                    sqlparser::ast::Partition::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Partition::Part(field0) => {
                    sqlparser::ast::Partition::Part(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Partition::Partitions(field0) => {
                    sqlparser::ast::Partition::Partitions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::PartitionRangeDirection {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::PartitionRangeDirection::Left => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::PartitionRangeDirection::Left,
                        )?
                }
                sqlparser::ast::PartitionRangeDirection::Right => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::PartitionRangeDirection::Right,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Password {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Password::Password(field0) => {
                    sqlparser::ast::Password::Password(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Password::NullPassword => {
                    transformer
                        .transform(node_path, sqlparser::ast::Password::NullPassword)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::PivotValueSource {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::PivotValueSource::List(field0) => {
                    sqlparser::ast::PivotValueSource::List(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::PivotValueSource::Any(field0) => {
                    sqlparser::ast::PivotValueSource::Any(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::PivotValueSource::Subquery(field0) => {
                    sqlparser::ast::PivotValueSource::Subquery(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Privileges {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Privileges::All { with_privileges_keyword } => {
                    sqlparser::ast::Privileges::All {
                        with_privileges_keyword: with_privileges_keyword
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Privileges::Actions(field0) => {
                    sqlparser::ast::Privileges::Actions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ProcedureParam {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, data_type } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                data_type: data_type.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ProjectionSelect {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { projection, order_by, group_by } = self;
            Self {
                projection: projection
                    .apply_transform_with_path(transformer, node_path)?,
                order_by: order_by.apply_transform_with_path(transformer, node_path)?,
                group_by: group_by.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Query {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                with: with.apply_transform_with_path(transformer, node_path)?,
                body: body.apply_transform_with_path(transformer, node_path)?,
                order_by: order_by.apply_transform_with_path(transformer, node_path)?,
                limit: limit.apply_transform_with_path(transformer, node_path)?,
                limit_by: limit_by.apply_transform_with_path(transformer, node_path)?,
                offset: offset.apply_transform_with_path(transformer, node_path)?,
                fetch: fetch.apply_transform_with_path(transformer, node_path)?,
                locks: locks.apply_transform_with_path(transformer, node_path)?,
                for_clause: for_clause
                    .apply_transform_with_path(transformer, node_path)?,
                settings: settings.apply_transform_with_path(transformer, node_path)?,
                format_clause: format_clause
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReferentialAction {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ReferentialAction::Restrict => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ReferentialAction::Restrict,
                        )?
                }
                sqlparser::ast::ReferentialAction::Cascade => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ReferentialAction::Cascade,
                        )?
                }
                sqlparser::ast::ReferentialAction::SetNull => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ReferentialAction::SetNull,
                        )?
                }
                sqlparser::ast::ReferentialAction::NoAction => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ReferentialAction::NoAction,
                        )?
                }
                sqlparser::ast::ReferentialAction::SetDefault => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ReferentialAction::SetDefault,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RenameSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::RenameSelectItem::Single(field0) => {
                    sqlparser::ast::RenameSelectItem::Single(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RenameSelectItem::Multiple(field0) => {
                    sqlparser::ast::RenameSelectItem::Multiple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RepetitionQuantifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::RepetitionQuantifier::ZeroOrMore => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::RepetitionQuantifier::ZeroOrMore,
                        )?
                }
                sqlparser::ast::RepetitionQuantifier::OneOrMore => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::RepetitionQuantifier::OneOrMore,
                        )?
                }
                sqlparser::ast::RepetitionQuantifier::AtMostOne => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::RepetitionQuantifier::AtMostOne,
                        )?
                }
                sqlparser::ast::RepetitionQuantifier::Exactly(field0) => {
                    sqlparser::ast::RepetitionQuantifier::Exactly(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RepetitionQuantifier::AtLeast(field0) => {
                    sqlparser::ast::RepetitionQuantifier::AtLeast(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RepetitionQuantifier::AtMost(field0) => {
                    sqlparser::ast::RepetitionQuantifier::AtMost(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RepetitionQuantifier::Range(field0, field1) => {
                    sqlparser::ast::RepetitionQuantifier::Range(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReplaceSelectElement {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { expr, column_name, as_keyword } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, node_path)?,
                column_name: column_name
                    .apply_transform_with_path(transformer, node_path)?,
                as_keyword: as_keyword.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ReplaceSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { items } = self;
            Self {
                items: items.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ResetConfig {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ResetConfig::ALL => {
                    transformer.transform(node_path, sqlparser::ast::ResetConfig::ALL)?
                }
                sqlparser::ast::ResetConfig::ConfigName(field0) => {
                    sqlparser::ast::ResetConfig::ConfigName(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RoleOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::RoleOption::BypassRLS(field0) => {
                    sqlparser::ast::RoleOption::BypassRLS(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::ConnectionLimit(field0) => {
                    sqlparser::ast::RoleOption::ConnectionLimit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::CreateDB(field0) => {
                    sqlparser::ast::RoleOption::CreateDB(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::CreateRole(field0) => {
                    sqlparser::ast::RoleOption::CreateRole(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::Inherit(field0) => {
                    sqlparser::ast::RoleOption::Inherit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::Login(field0) => {
                    sqlparser::ast::RoleOption::Login(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::Password(field0) => {
                    sqlparser::ast::RoleOption::Password(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::Replication(field0) => {
                    sqlparser::ast::RoleOption::Replication(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::SuperUser(field0) => {
                    sqlparser::ast::RoleOption::SuperUser(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::RoleOption::ValidUntil(field0) => {
                    sqlparser::ast::RoleOption::ValidUntil(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RowAccessPolicy {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { policy, on } = self;
            Self {
                policy: policy.apply_transform_with_path(transformer, node_path)?,
                on: on.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::RowsPerMatch {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::RowsPerMatch::OneRow => {
                    transformer
                        .transform(node_path, sqlparser::ast::RowsPerMatch::OneRow)?
                }
                sqlparser::ast::RowsPerMatch::AllRows(field0) => {
                    sqlparser::ast::RowsPerMatch::AllRows(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SchemaName {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SchemaName::Simple(field0) => {
                    sqlparser::ast::SchemaName::Simple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SchemaName::UnnamedAuthorization(field0) => {
                    sqlparser::ast::SchemaName::UnnamedAuthorization(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SchemaName::NamedAuthorization(field0, field1) => {
                    sqlparser::ast::SchemaName::NamedAuthorization(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SearchModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SearchModifier::InNaturalLanguageMode => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SearchModifier::InNaturalLanguageMode,
                        )?
                }
                sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion,
                        )?
                }
                sqlparser::ast::SearchModifier::InBooleanMode => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SearchModifier::InBooleanMode,
                        )?
                }
                sqlparser::ast::SearchModifier::WithQueryExpansion => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SearchModifier::WithQueryExpansion,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SecretOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Select {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                distinct: distinct.apply_transform_with_path(transformer, node_path)?,
                top: top.apply_transform_with_path(transformer, node_path)?,
                top_before_distinct: top_before_distinct
                    .apply_transform_with_path(transformer, node_path)?,
                projection: projection
                    .apply_transform_with_path(transformer, node_path)?,
                into: into.apply_transform_with_path(transformer, node_path)?,
                from: from.apply_transform_with_path(transformer, node_path)?,
                lateral_views: lateral_views
                    .apply_transform_with_path(transformer, node_path)?,
                prewhere: prewhere.apply_transform_with_path(transformer, node_path)?,
                selection: selection.apply_transform_with_path(transformer, node_path)?,
                group_by: group_by.apply_transform_with_path(transformer, node_path)?,
                cluster_by: cluster_by
                    .apply_transform_with_path(transformer, node_path)?,
                distribute_by: distribute_by
                    .apply_transform_with_path(transformer, node_path)?,
                sort_by: sort_by.apply_transform_with_path(transformer, node_path)?,
                having: having.apply_transform_with_path(transformer, node_path)?,
                named_window: named_window
                    .apply_transform_with_path(transformer, node_path)?,
                qualify: qualify.apply_transform_with_path(transformer, node_path)?,
                window_before_qualify: window_before_qualify
                    .apply_transform_with_path(transformer, node_path)?,
                value_table_mode: value_table_mode
                    .apply_transform_with_path(transformer, node_path)?,
                connect_by: connect_by.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SelectInto {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { temporary, unlogged, table, name } = self;
            Self {
                temporary: temporary.apply_transform_with_path(transformer, node_path)?,
                unlogged: unlogged.apply_transform_with_path(transformer, node_path)?,
                table: table.apply_transform_with_path(transformer, node_path)?,
                name: name.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SelectItem::UnnamedExpr(field0) => {
                    sqlparser::ast::SelectItem::UnnamedExpr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                    sqlparser::ast::SelectItem::ExprWithAlias {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::SelectItem::QualifiedWildcard(field0, field1) => {
                    sqlparser::ast::SelectItem::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SelectItem::Wildcard(field0) => {
                    sqlparser::ast::SelectItem::Wildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SequenceOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SequenceOptions::IncrementBy(field0, field1) => {
                    sqlparser::ast::SequenceOptions::IncrementBy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SequenceOptions::MinValue(field0) => {
                    sqlparser::ast::SequenceOptions::MinValue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SequenceOptions::MaxValue(field0) => {
                    sqlparser::ast::SequenceOptions::MaxValue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SequenceOptions::StartWith(field0, field1) => {
                    sqlparser::ast::SequenceOptions::StartWith(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SequenceOptions::Cache(field0) => {
                    sqlparser::ast::SequenceOptions::Cache(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SequenceOptions::Cycle(field0) => {
                    sqlparser::ast::SequenceOptions::Cycle(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetConfigValue {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SetConfigValue::Default => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetConfigValue::Default)?
                }
                sqlparser::ast::SetConfigValue::FromCurrent => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SetConfigValue::FromCurrent,
                        )?
                }
                sqlparser::ast::SetConfigValue::Value(field0) => {
                    sqlparser::ast::SetConfigValue::Value(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetExpr {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SetExpr::Select(field0) => {
                    sqlparser::ast::SetExpr::Select(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SetExpr::Query(field0) => {
                    sqlparser::ast::SetExpr::Query(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SetExpr::SetOperation {
                    op,
                    set_quantifier,
                    left,
                    right,
                } => {
                    sqlparser::ast::SetExpr::SetOperation {
                        op: op.apply_transform_with_path(transformer, node_path)?,
                        set_quantifier: set_quantifier
                            .apply_transform_with_path(transformer, node_path)?,
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::SetExpr::Values(field0) => {
                    sqlparser::ast::SetExpr::Values(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SetExpr::Insert(field0) => {
                    sqlparser::ast::SetExpr::Insert(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SetExpr::Update(field0) => {
                    sqlparser::ast::SetExpr::Update(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SetExpr::Table(field0) => {
                    sqlparser::ast::SetExpr::Table(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SetOperator::Union => {
                    transformer.transform(node_path, sqlparser::ast::SetOperator::Union)?
                }
                sqlparser::ast::SetOperator::Except => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetOperator::Except)?
                }
                sqlparser::ast::SetOperator::Intersect => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetOperator::Intersect)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SetQuantifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SetQuantifier::All => {
                    transformer.transform(node_path, sqlparser::ast::SetQuantifier::All)?
                }
                sqlparser::ast::SetQuantifier::Distinct => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetQuantifier::Distinct)?
                }
                sqlparser::ast::SetQuantifier::ByName => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetQuantifier::ByName)?
                }
                sqlparser::ast::SetQuantifier::AllByName => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetQuantifier::AllByName)?
                }
                sqlparser::ast::SetQuantifier::DistinctByName => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SetQuantifier::DistinctByName,
                        )?
                }
                sqlparser::ast::SetQuantifier::None => {
                    transformer
                        .transform(node_path, sqlparser::ast::SetQuantifier::None)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Setting {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowClause {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ShowClause::IN => {
                    transformer.transform(node_path, sqlparser::ast::ShowClause::IN)?
                }
                sqlparser::ast::ShowClause::FROM => {
                    transformer.transform(node_path, sqlparser::ast::ShowClause::FROM)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowCreateObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ShowCreateObject::Event => {
                    transformer
                        .transform(node_path, sqlparser::ast::ShowCreateObject::Event)?
                }
                sqlparser::ast::ShowCreateObject::Function => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ShowCreateObject::Function,
                        )?
                }
                sqlparser::ast::ShowCreateObject::Procedure => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::ShowCreateObject::Procedure,
                        )?
                }
                sqlparser::ast::ShowCreateObject::Table => {
                    transformer
                        .transform(node_path, sqlparser::ast::ShowCreateObject::Table)?
                }
                sqlparser::ast::ShowCreateObject::Trigger => {
                    transformer
                        .transform(node_path, sqlparser::ast::ShowCreateObject::Trigger)?
                }
                sqlparser::ast::ShowCreateObject::View => {
                    transformer
                        .transform(node_path, sqlparser::ast::ShowCreateObject::View)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ShowStatementFilter {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ShowStatementFilter::Like(field0) => {
                    sqlparser::ast::ShowStatementFilter::Like(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ShowStatementFilter::ILike(field0) => {
                    sqlparser::ast::ShowStatementFilter::ILike(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ShowStatementFilter::Where(field0) => {
                    sqlparser::ast::ShowStatementFilter::Where(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::ShowStatementFilter::NoKeyword(field0) => {
                    sqlparser::ast::ShowStatementFilter::NoKeyword(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SqlOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SqlOption::Clustered(field0) => {
                    sqlparser::ast::SqlOption::Clustered(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SqlOption::Ident(field0) => {
                    sqlparser::ast::SqlOption::Ident(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::SqlOption::KeyValue { key, value } => {
                    sqlparser::ast::SqlOption::KeyValue {
                        key: key.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::SqlOption::Partition {
                    column_name,
                    range_direction,
                    for_values,
                } => {
                    sqlparser::ast::SqlOption::Partition {
                        column_name: column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        range_direction: range_direction
                            .apply_transform_with_path(transformer, node_path)?,
                        for_values: for_values
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SqliteOnConflict {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::SqliteOnConflict::Rollback => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::SqliteOnConflict::Rollback,
                        )?
                }
                sqlparser::ast::SqliteOnConflict::Abort => {
                    transformer
                        .transform(node_path, sqlparser::ast::SqliteOnConflict::Abort)?
                }
                sqlparser::ast::SqliteOnConflict::Fail => {
                    transformer
                        .transform(node_path, sqlparser::ast::SqliteOnConflict::Fail)?
                }
                sqlparser::ast::SqliteOnConflict::Ignore => {
                    transformer
                        .transform(node_path, sqlparser::ast::SqliteOnConflict::Ignore)?
                }
                sqlparser::ast::SqliteOnConflict::Replace => {
                    transformer
                        .transform(node_path, sqlparser::ast::SqliteOnConflict::Replace)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Statement {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                } => {
                    sqlparser::ast::Statement::Analyze {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        partitions: partitions
                            .apply_transform_with_path(transformer, node_path)?,
                        for_columns: for_columns
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        cache_metadata: cache_metadata
                            .apply_transform_with_path(transformer, node_path)?,
                        noscan: noscan
                            .apply_transform_with_path(transformer, node_path)?,
                        compute_statistics: compute_statistics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::Truncate {
                        table_names: table_names
                            .apply_transform_with_path(transformer, node_path)?,
                        partitions: partitions
                            .apply_transform_with_path(transformer, node_path)?,
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        only: only.apply_transform_with_path(transformer, node_path)?,
                        identity: identity
                            .apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                        on_cluster: on_cluster
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Msck {
                    table_name,
                    repair,
                    partition_action,
                } => {
                    sqlparser::ast::Statement::Msck {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        repair: repair
                            .apply_transform_with_path(transformer, node_path)?,
                        partition_action: partition_action
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Query(field0) => {
                    sqlparser::ast::Statement::Query(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Statement::Insert(field0) => {
                    sqlparser::ast::Statement::Insert(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Statement::Install { extension_name } => {
                    sqlparser::ast::Statement::Install {
                        extension_name: extension_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Load { extension_name } => {
                    sqlparser::ast::Statement::Load {
                        extension_name: extension_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Directory {
                    overwrite,
                    local,
                    path,
                    file_format,
                    source,
                } => {
                    sqlparser::ast::Statement::Directory {
                        overwrite: overwrite
                            .apply_transform_with_path(transformer, node_path)?,
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        path: path.apply_transform_with_path(transformer, node_path)?,
                        file_format: file_format
                            .apply_transform_with_path(transformer, node_path)?,
                        source: source.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Call(field0) => {
                    sqlparser::ast::Statement::Call(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Statement::Copy {
                    source,
                    to,
                    target,
                    options,
                    legacy_options,
                    values,
                } => {
                    sqlparser::ast::Statement::Copy {
                        source: source
                            .apply_transform_with_path(transformer, node_path)?,
                        to: to.apply_transform_with_path(transformer, node_path)?,
                        target: target
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                        legacy_options: legacy_options
                            .apply_transform_with_path(transformer, node_path)?,
                        values: values.apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::CopyIntoSnowflake {
                        into: into.apply_transform_with_path(transformer, node_path)?,
                        from_stage: from_stage
                            .apply_transform_with_path(transformer, node_path)?,
                        from_stage_alias: from_stage_alias
                            .apply_transform_with_path(transformer, node_path)?,
                        stage_params: stage_params
                            .apply_transform_with_path(transformer, node_path)?,
                        from_transformations: from_transformations
                            .apply_transform_with_path(transformer, node_path)?,
                        files: files.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        file_format: file_format
                            .apply_transform_with_path(transformer, node_path)?,
                        copy_options: copy_options
                            .apply_transform_with_path(transformer, node_path)?,
                        validation_mode: validation_mode
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Close { cursor } => {
                    sqlparser::ast::Statement::Close {
                        cursor: cursor.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Update {
                    table,
                    assignments,
                    from,
                    selection,
                    returning,
                } => {
                    sqlparser::ast::Statement::Update {
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        assignments: assignments
                            .apply_transform_with_path(transformer, node_path)?,
                        from: from.apply_transform_with_path(transformer, node_path)?,
                        selection: selection
                            .apply_transform_with_path(transformer, node_path)?,
                        returning: returning
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Delete(field0) => {
                    sqlparser::ast::Statement::Delete(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
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
                    sqlparser::ast::Statement::CreateView {
                        or_replace: or_replace
                            .apply_transform_with_path(transformer, node_path)?,
                        materialized: materialized
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        query: query.apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                        cluster_by: cluster_by
                            .apply_transform_with_path(transformer, node_path)?,
                        comment: comment
                            .apply_transform_with_path(transformer, node_path)?,
                        with_no_schema_binding: with_no_schema_binding
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        to: to.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateTable(field0) => {
                    sqlparser::ast::Statement::CreateTable(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Statement::CreateVirtualTable {
                    name,
                    if_not_exists,
                    module_name,
                    module_args,
                } => {
                    sqlparser::ast::Statement::CreateVirtualTable {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        module_name: module_name
                            .apply_transform_with_path(transformer, node_path)?,
                        module_args: module_args
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateIndex(field0) => {
                    sqlparser::ast::Statement::CreateIndex(
                        field0.apply_transform_with_path(transformer, node_path)?,
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
                } => {
                    sqlparser::ast::Statement::CreateRole {
                        names: names.apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        login: login.apply_transform_with_path(transformer, node_path)?,
                        inherit: inherit
                            .apply_transform_with_path(transformer, node_path)?,
                        bypassrls: bypassrls
                            .apply_transform_with_path(transformer, node_path)?,
                        password: password
                            .apply_transform_with_path(transformer, node_path)?,
                        superuser: superuser
                            .apply_transform_with_path(transformer, node_path)?,
                        create_db: create_db
                            .apply_transform_with_path(transformer, node_path)?,
                        create_role: create_role
                            .apply_transform_with_path(transformer, node_path)?,
                        replication: replication
                            .apply_transform_with_path(transformer, node_path)?,
                        connection_limit: connection_limit
                            .apply_transform_with_path(transformer, node_path)?,
                        valid_until: valid_until
                            .apply_transform_with_path(transformer, node_path)?,
                        in_role: in_role
                            .apply_transform_with_path(transformer, node_path)?,
                        in_group: in_group
                            .apply_transform_with_path(transformer, node_path)?,
                        role: role.apply_transform_with_path(transformer, node_path)?,
                        user: user.apply_transform_with_path(transformer, node_path)?,
                        admin: admin.apply_transform_with_path(transformer, node_path)?,
                        authorization_owner: authorization_owner
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::CreateSecret {
                        or_replace: or_replace
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        storage_specifier: storage_specifier
                            .apply_transform_with_path(transformer, node_path)?,
                        secret_type: secret_type
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::CreatePolicy {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        policy_type: policy_type
                            .apply_transform_with_path(transformer, node_path)?,
                        command: command
                            .apply_transform_with_path(transformer, node_path)?,
                        to: to.apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                        with_check: with_check
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AlterTable {
                    name,
                    if_exists,
                    only,
                    operations,
                    location,
                    on_cluster,
                } => {
                    sqlparser::ast::Statement::AlterTable {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        only: only.apply_transform_with_path(transformer, node_path)?,
                        operations: operations
                            .apply_transform_with_path(transformer, node_path)?,
                        location: location
                            .apply_transform_with_path(transformer, node_path)?,
                        on_cluster: on_cluster
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AlterIndex { name, operation } => {
                    sqlparser::ast::Statement::AlterIndex {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        operation: operation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AlterView {
                    name,
                    columns,
                    query,
                    with_options,
                } => {
                    sqlparser::ast::Statement::AlterView {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        query: query.apply_transform_with_path(transformer, node_path)?,
                        with_options: with_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AlterRole { name, operation } => {
                    sqlparser::ast::Statement::AlterRole {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        operation: operation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AlterPolicy {
                    name,
                    table_name,
                    operation,
                } => {
                    sqlparser::ast::Statement::AlterPolicy {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        operation: operation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AttachDatabase {
                    schema_name,
                    database_file_name,
                    database,
                } => {
                    sqlparser::ast::Statement::AttachDatabase {
                        schema_name: schema_name
                            .apply_transform_with_path(transformer, node_path)?,
                        database_file_name: database_file_name
                            .apply_transform_with_path(transformer, node_path)?,
                        database: database
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::AttachDuckDBDatabase {
                    if_not_exists,
                    database,
                    database_path,
                    database_alias,
                    attach_options,
                } => {
                    sqlparser::ast::Statement::AttachDuckDBDatabase {
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        database: database
                            .apply_transform_with_path(transformer, node_path)?,
                        database_path: database_path
                            .apply_transform_with_path(transformer, node_path)?,
                        database_alias: database_alias
                            .apply_transform_with_path(transformer, node_path)?,
                        attach_options: attach_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::DetachDuckDBDatabase {
                    if_exists,
                    database,
                    database_alias,
                } => {
                    sqlparser::ast::Statement::DetachDuckDBDatabase {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        database: database
                            .apply_transform_with_path(transformer, node_path)?,
                        database_alias: database_alias
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::Drop {
                        object_type: object_type
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        names: names.apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                        restrict: restrict
                            .apply_transform_with_path(transformer, node_path)?,
                        purge: purge.apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::DropFunction {
                    if_exists,
                    func_desc,
                    option,
                } => {
                    sqlparser::ast::Statement::DropFunction {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        func_desc: func_desc
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::DropProcedure {
                    if_exists,
                    proc_desc,
                    option,
                } => {
                    sqlparser::ast::Statement::DropProcedure {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        proc_desc: proc_desc
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::DropSecret {
                    if_exists,
                    temporary,
                    name,
                    storage_specifier,
                } => {
                    sqlparser::ast::Statement::DropSecret {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        storage_specifier: storage_specifier
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::DropPolicy {
                    if_exists,
                    name,
                    table_name,
                    option,
                } => {
                    sqlparser::ast::Statement::DropPolicy {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Declare { stmts } => {
                    sqlparser::ast::Statement::Declare {
                        stmts: stmts.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateExtension {
                    name,
                    if_not_exists,
                    cascade,
                    schema,
                    version,
                } => {
                    sqlparser::ast::Statement::CreateExtension {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                        schema: schema
                            .apply_transform_with_path(transformer, node_path)?,
                        version: version
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Fetch { name, direction, into } => {
                    sqlparser::ast::Statement::Fetch {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        direction: direction
                            .apply_transform_with_path(transformer, node_path)?,
                        into: into.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Flush {
                    object_type,
                    location,
                    channel,
                    read_lock,
                    export,
                    tables,
                } => {
                    sqlparser::ast::Statement::Flush {
                        object_type: object_type
                            .apply_transform_with_path(transformer, node_path)?,
                        location: location
                            .apply_transform_with_path(transformer, node_path)?,
                        channel: channel
                            .apply_transform_with_path(transformer, node_path)?,
                        read_lock: read_lock
                            .apply_transform_with_path(transformer, node_path)?,
                        export: export
                            .apply_transform_with_path(transformer, node_path)?,
                        tables: tables.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Discard { object_type } => {
                    sqlparser::ast::Statement::Discard {
                        object_type: object_type
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::SetRole { context_modifier, role_name } => {
                    sqlparser::ast::Statement::SetRole {
                        context_modifier: context_modifier
                            .apply_transform_with_path(transformer, node_path)?,
                        role_name: role_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::SetVariable {
                    local,
                    hivevar,
                    variables,
                    value,
                } => {
                    sqlparser::ast::Statement::SetVariable {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        hivevar: hivevar
                            .apply_transform_with_path(transformer, node_path)?,
                        variables: variables
                            .apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::SetTimeZone { local, value } => {
                    sqlparser::ast::Statement::SetTimeZone {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::SetNames { charset_name, collation_name } => {
                    sqlparser::ast::Statement::SetNames {
                        charset_name: charset_name
                            .apply_transform_with_path(transformer, node_path)?,
                        collation_name: collation_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::SetNamesDefault {} => {
                    sqlparser::ast::Statement::SetNamesDefault {
                    }
                }
                sqlparser::ast::Statement::ShowFunctions { filter } => {
                    sqlparser::ast::Statement::ShowFunctions {
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowVariable { variable } => {
                    sqlparser::ast::Statement::ShowVariable {
                        variable: variable
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowStatus { filter, global, session } => {
                    sqlparser::ast::Statement::ShowStatus {
                        filter: filter
                            .apply_transform_with_path(transformer, node_path)?,
                        global: global
                            .apply_transform_with_path(transformer, node_path)?,
                        session: session
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowVariables { filter, global, session } => {
                    sqlparser::ast::Statement::ShowVariables {
                        filter: filter
                            .apply_transform_with_path(transformer, node_path)?,
                        global: global
                            .apply_transform_with_path(transformer, node_path)?,
                        session: session
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowCreate { obj_type, obj_name } => {
                    sqlparser::ast::Statement::ShowCreate {
                        obj_type: obj_type
                            .apply_transform_with_path(transformer, node_path)?,
                        obj_name: obj_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowColumns {
                    extended,
                    full,
                    table_name,
                    filter,
                } => {
                    sqlparser::ast::Statement::ShowColumns {
                        extended: extended
                            .apply_transform_with_path(transformer, node_path)?,
                        full: full.apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowDatabases { filter } => {
                    sqlparser::ast::Statement::ShowDatabases {
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowSchemas { filter } => {
                    sqlparser::ast::Statement::ShowSchemas {
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowTables {
                    extended,
                    full,
                    clause,
                    db_name,
                    filter,
                } => {
                    sqlparser::ast::Statement::ShowTables {
                        extended: extended
                            .apply_transform_with_path(transformer, node_path)?,
                        full: full.apply_transform_with_path(transformer, node_path)?,
                        clause: clause
                            .apply_transform_with_path(transformer, node_path)?,
                        db_name: db_name
                            .apply_transform_with_path(transformer, node_path)?,
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowViews {
                    materialized,
                    clause,
                    db_name,
                    filter,
                } => {
                    sqlparser::ast::Statement::ShowViews {
                        materialized: materialized
                            .apply_transform_with_path(transformer, node_path)?,
                        clause: clause
                            .apply_transform_with_path(transformer, node_path)?,
                        db_name: db_name
                            .apply_transform_with_path(transformer, node_path)?,
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ShowCollation { filter } => {
                    sqlparser::ast::Statement::ShowCollation {
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Use(field0) => {
                    sqlparser::ast::Statement::Use(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Statement::StartTransaction {
                    modes,
                    begin,
                    modifier,
                } => {
                    sqlparser::ast::Statement::StartTransaction {
                        modes: modes.apply_transform_with_path(transformer, node_path)?,
                        begin: begin.apply_transform_with_path(transformer, node_path)?,
                        modifier: modifier
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::SetTransaction {
                    modes,
                    snapshot,
                    session,
                } => {
                    sqlparser::ast::Statement::SetTransaction {
                        modes: modes.apply_transform_with_path(transformer, node_path)?,
                        snapshot: snapshot
                            .apply_transform_with_path(transformer, node_path)?,
                        session: session
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Comment {
                    object_type,
                    object_name,
                    comment,
                    if_exists,
                } => {
                    sqlparser::ast::Statement::Comment {
                        object_type: object_type
                            .apply_transform_with_path(transformer, node_path)?,
                        object_name: object_name
                            .apply_transform_with_path(transformer, node_path)?,
                        comment: comment
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Commit { chain } => {
                    sqlparser::ast::Statement::Commit {
                        chain: chain.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Rollback { chain, savepoint } => {
                    sqlparser::ast::Statement::Rollback {
                        chain: chain.apply_transform_with_path(transformer, node_path)?,
                        savepoint: savepoint
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateSchema {
                    schema_name,
                    if_not_exists,
                } => {
                    sqlparser::ast::Statement::CreateSchema {
                        schema_name: schema_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateDatabase {
                    db_name,
                    if_not_exists,
                    location,
                    managed_location,
                } => {
                    sqlparser::ast::Statement::CreateDatabase {
                        db_name: db_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        location: location
                            .apply_transform_with_path(transformer, node_path)?,
                        managed_location: managed_location
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::CreateFunction {
                        or_replace: or_replace
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        args: args.apply_transform_with_path(transformer, node_path)?,
                        return_type: return_type
                            .apply_transform_with_path(transformer, node_path)?,
                        function_body: function_body
                            .apply_transform_with_path(transformer, node_path)?,
                        behavior: behavior
                            .apply_transform_with_path(transformer, node_path)?,
                        called_on_null: called_on_null
                            .apply_transform_with_path(transformer, node_path)?,
                        parallel: parallel
                            .apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                        language: language
                            .apply_transform_with_path(transformer, node_path)?,
                        determinism_specifier: determinism_specifier
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                        remote_connection: remote_connection
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::CreateTrigger {
                        or_replace: or_replace
                            .apply_transform_with_path(transformer, node_path)?,
                        is_constraint: is_constraint
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        period: period
                            .apply_transform_with_path(transformer, node_path)?,
                        events: events
                            .apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        referenced_table_name: referenced_table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        referencing: referencing
                            .apply_transform_with_path(transformer, node_path)?,
                        trigger_object: trigger_object
                            .apply_transform_with_path(transformer, node_path)?,
                        include_each: include_each
                            .apply_transform_with_path(transformer, node_path)?,
                        condition: condition
                            .apply_transform_with_path(transformer, node_path)?,
                        exec_body: exec_body
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::DropTrigger {
                    if_exists,
                    trigger_name,
                    table_name,
                    option,
                } => {
                    sqlparser::ast::Statement::DropTrigger {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        trigger_name: trigger_name
                            .apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateProcedure {
                    or_alter,
                    name,
                    params,
                    body,
                } => {
                    sqlparser::ast::Statement::CreateProcedure {
                        or_alter: or_alter
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        params: params
                            .apply_transform_with_path(transformer, node_path)?,
                        body: body.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateMacro {
                    or_replace,
                    temporary,
                    name,
                    args,
                    definition,
                } => {
                    sqlparser::ast::Statement::CreateMacro {
                        or_replace: or_replace
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        args: args.apply_transform_with_path(transformer, node_path)?,
                        definition: definition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::CreateStage {
                        or_replace: or_replace
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        stage_params: stage_params
                            .apply_transform_with_path(transformer, node_path)?,
                        directory_table_params: directory_table_params
                            .apply_transform_with_path(transformer, node_path)?,
                        file_format: file_format
                            .apply_transform_with_path(transformer, node_path)?,
                        copy_options: copy_options
                            .apply_transform_with_path(transformer, node_path)?,
                        comment: comment
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Assert { condition, message } => {
                    sqlparser::ast::Statement::Assert {
                        condition: condition
                            .apply_transform_with_path(transformer, node_path)?,
                        message: message
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Grant {
                    privileges,
                    objects,
                    grantees,
                    with_grant_option,
                    granted_by,
                } => {
                    sqlparser::ast::Statement::Grant {
                        privileges: privileges
                            .apply_transform_with_path(transformer, node_path)?,
                        objects: objects
                            .apply_transform_with_path(transformer, node_path)?,
                        grantees: grantees
                            .apply_transform_with_path(transformer, node_path)?,
                        with_grant_option: with_grant_option
                            .apply_transform_with_path(transformer, node_path)?,
                        granted_by: granted_by
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Revoke {
                    privileges,
                    objects,
                    grantees,
                    granted_by,
                    cascade,
                } => {
                    sqlparser::ast::Statement::Revoke {
                        privileges: privileges
                            .apply_transform_with_path(transformer, node_path)?,
                        objects: objects
                            .apply_transform_with_path(transformer, node_path)?,
                        grantees: grantees
                            .apply_transform_with_path(transformer, node_path)?,
                        granted_by: granted_by
                            .apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Deallocate { name, prepare } => {
                    sqlparser::ast::Statement::Deallocate {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        prepare: prepare
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Execute {
                    name,
                    parameters,
                    has_parentheses,
                    using,
                } => {
                    sqlparser::ast::Statement::Execute {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        parameters: parameters
                            .apply_transform_with_path(transformer, node_path)?,
                        has_parentheses: has_parentheses
                            .apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Prepare { name, data_types, statement } => {
                    sqlparser::ast::Statement::Prepare {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        data_types: data_types
                            .apply_transform_with_path(transformer, node_path)?,
                        statement: statement
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Kill { modifier, id } => {
                    sqlparser::ast::Statement::Kill {
                        modifier: modifier
                            .apply_transform_with_path(transformer, node_path)?,
                        id: id.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ExplainTable {
                    describe_alias,
                    hive_format,
                    has_table_keyword,
                    table_name,
                } => {
                    sqlparser::ast::Statement::ExplainTable {
                        describe_alias: describe_alias
                            .apply_transform_with_path(transformer, node_path)?,
                        hive_format: hive_format
                            .apply_transform_with_path(transformer, node_path)?,
                        has_table_keyword: has_table_keyword
                            .apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::Statement::Explain {
                        describe_alias: describe_alias
                            .apply_transform_with_path(transformer, node_path)?,
                        analyze: analyze
                            .apply_transform_with_path(transformer, node_path)?,
                        verbose: verbose
                            .apply_transform_with_path(transformer, node_path)?,
                        query_plan: query_plan
                            .apply_transform_with_path(transformer, node_path)?,
                        statement: statement
                            .apply_transform_with_path(transformer, node_path)?,
                        format: format
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Savepoint { name } => {
                    sqlparser::ast::Statement::Savepoint {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::ReleaseSavepoint { name } => {
                    sqlparser::ast::Statement::ReleaseSavepoint {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Merge { into, table, source, on, clauses } => {
                    sqlparser::ast::Statement::Merge {
                        into: into.apply_transform_with_path(transformer, node_path)?,
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        source: source
                            .apply_transform_with_path(transformer, node_path)?,
                        on: on.apply_transform_with_path(transformer, node_path)?,
                        clauses: clauses
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Cache {
                    table_flag,
                    table_name,
                    has_as,
                    options,
                    query,
                } => {
                    sqlparser::ast::Statement::Cache {
                        table_flag: table_flag
                            .apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        has_as: has_as
                            .apply_transform_with_path(transformer, node_path)?,
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                        query: query.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::UNCache { table_name, if_exists } => {
                    sqlparser::ast::Statement::UNCache {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateSequence {
                    temporary,
                    if_not_exists,
                    name,
                    data_type,
                    sequence_options,
                    owned_by,
                } => {
                    sqlparser::ast::Statement::CreateSequence {
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        sequence_options: sequence_options
                            .apply_transform_with_path(transformer, node_path)?,
                        owned_by: owned_by
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::CreateType { name, representation } => {
                    sqlparser::ast::Statement::CreateType {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        representation: representation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::Pragma { name, value, is_eq } => {
                    sqlparser::ast::Statement::Pragma {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                        is_eq: is_eq.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::LockTables { tables } => {
                    sqlparser::ast::Statement::LockTables {
                        tables: tables.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::UnlockTables => {
                    transformer
                        .transform(node_path, sqlparser::ast::Statement::UnlockTables)?
                }
                sqlparser::ast::Statement::Unload { query, to, with } => {
                    sqlparser::ast::Statement::Unload {
                        query: query.apply_transform_with_path(transformer, node_path)?,
                        to: to.apply_transform_with_path(transformer, node_path)?,
                        with: with.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::OptimizeTable {
                    name,
                    on_cluster,
                    partition,
                    include_final,
                    deduplicate,
                } => {
                    sqlparser::ast::Statement::OptimizeTable {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        on_cluster: on_cluster
                            .apply_transform_with_path(transformer, node_path)?,
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                        include_final: include_final
                            .apply_transform_with_path(transformer, node_path)?,
                        deduplicate: deduplicate
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::LISTEN { channel } => {
                    sqlparser::ast::Statement::LISTEN {
                        channel: channel
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Statement::NOTIFY { channel, payload } => {
                    sqlparser::ast::Statement::NOTIFY {
                        channel: channel
                            .apply_transform_with_path(transformer, node_path)?,
                        payload: payload
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::StructBracketKind {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::StructBracketKind::Parentheses => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::StructBracketKind::Parentheses,
                        )?
                }
                sqlparser::ast::StructBracketKind::AngleBrackets => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::StructBracketKind::AngleBrackets,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::StructField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { field_name, field_type } = self;
            Self {
                field_name: field_name
                    .apply_transform_with_path(transformer, node_path)?,
                field_type: field_type.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Subscript {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Subscript::Index { index } => {
                    sqlparser::ast::Subscript::Index {
                        index: index.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::Subscript::Slice { lower_bound, upper_bound, stride } => {
                    sqlparser::ast::Subscript::Slice {
                        lower_bound: lower_bound
                            .apply_transform_with_path(transformer, node_path)?,
                        upper_bound: upper_bound
                            .apply_transform_with_path(transformer, node_path)?,
                        stride: stride.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::SymbolDefinition {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { symbol, definition } = self;
            Self {
                symbol: symbol.apply_transform_with_path(transformer, node_path)?,
                definition: definition.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Table {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { table_name, schema_name } = self;
            Self {
                table_name: table_name
                    .apply_transform_with_path(transformer, node_path)?,
                schema_name: schema_name
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableAlias {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, columns } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                columns: columns.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableConstraint {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                } => {
                    sqlparser::ast::TableConstraint::Unique {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        index_name: index_name
                            .apply_transform_with_path(transformer, node_path)?,
                        index_type_display: index_type_display
                            .apply_transform_with_path(transformer, node_path)?,
                        index_type: index_type
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        index_options: index_options
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableConstraint::PrimaryKey {
                    name,
                    index_name,
                    index_type,
                    columns,
                    index_options,
                    characteristics,
                } => {
                    sqlparser::ast::TableConstraint::PrimaryKey {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        index_name: index_name
                            .apply_transform_with_path(transformer, node_path)?,
                        index_type: index_type
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        index_options: index_options
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::TableConstraint::ForeignKey {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        foreign_table: foreign_table
                            .apply_transform_with_path(transformer, node_path)?,
                        referred_columns: referred_columns
                            .apply_transform_with_path(transformer, node_path)?,
                        on_delete: on_delete
                            .apply_transform_with_path(transformer, node_path)?,
                        on_update: on_update
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableConstraint::Check { name, expr } => {
                    sqlparser::ast::TableConstraint::Check {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableConstraint::Index {
                    display_as_key,
                    name,
                    index_type,
                    columns,
                } => {
                    sqlparser::ast::TableConstraint::Index {
                        display_as_key: display_as_key
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        index_type: index_type
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableConstraint::FulltextOrSpatial {
                    fulltext,
                    index_type_display,
                    opt_index_name,
                    columns,
                } => {
                    sqlparser::ast::TableConstraint::FulltextOrSpatial {
                        fulltext: fulltext
                            .apply_transform_with_path(transformer, node_path)?,
                        index_type_display: index_type_display
                            .apply_transform_with_path(transformer, node_path)?,
                        opt_index_name: opt_index_name
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableEngine {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, parameters } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                parameters: parameters.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableFactor {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
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
                } => {
                    sqlparser::ast::TableFactor::Table {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                        args: args.apply_transform_with_path(transformer, node_path)?,
                        with_hints: with_hints
                            .apply_transform_with_path(transformer, node_path)?,
                        version: version
                            .apply_transform_with_path(transformer, node_path)?,
                        with_ordinality: with_ordinality
                            .apply_transform_with_path(transformer, node_path)?,
                        partitions: partitions
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::Derived { lateral, subquery, alias } => {
                    sqlparser::ast::TableFactor::Derived {
                        lateral: lateral
                            .apply_transform_with_path(transformer, node_path)?,
                        subquery: subquery
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::TableFunction { expr, alias } => {
                    sqlparser::ast::TableFactor::TableFunction {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::Function { lateral, name, args, alias } => {
                    sqlparser::ast::TableFactor::Function {
                        lateral: lateral
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        args: args.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::UNNEST {
                    alias,
                    array_exprs,
                    with_offset,
                    with_offset_alias,
                    with_ordinality,
                } => {
                    sqlparser::ast::TableFactor::UNNEST {
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                        array_exprs: array_exprs
                            .apply_transform_with_path(transformer, node_path)?,
                        with_offset: with_offset
                            .apply_transform_with_path(transformer, node_path)?,
                        with_offset_alias: with_offset_alias
                            .apply_transform_with_path(transformer, node_path)?,
                        with_ordinality: with_ordinality
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::JsonTable {
                    json_expr,
                    json_path,
                    columns,
                    alias,
                } => {
                    sqlparser::ast::TableFactor::JsonTable {
                        json_expr: json_expr
                            .apply_transform_with_path(transformer, node_path)?,
                        json_path: json_path
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::NestedJoin { table_with_joins, alias } => {
                    sqlparser::ast::TableFactor::NestedJoin {
                        table_with_joins: table_with_joins
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::Pivot {
                    table,
                    aggregate_functions,
                    value_column,
                    value_source,
                    default_on_null,
                    alias,
                } => {
                    sqlparser::ast::TableFactor::Pivot {
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        aggregate_functions: aggregate_functions
                            .apply_transform_with_path(transformer, node_path)?,
                        value_column: value_column
                            .apply_transform_with_path(transformer, node_path)?,
                        value_source: value_source
                            .apply_transform_with_path(transformer, node_path)?,
                        default_on_null: default_on_null
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::TableFactor::Unpivot {
                    table,
                    value,
                    name,
                    columns,
                    alias,
                } => {
                    sqlparser::ast::TableFactor::Unpivot {
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
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
                    sqlparser::ast::TableFactor::MatchRecognize {
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        partition_by: partition_by
                            .apply_transform_with_path(transformer, node_path)?,
                        order_by: order_by
                            .apply_transform_with_path(transformer, node_path)?,
                        measures: measures
                            .apply_transform_with_path(transformer, node_path)?,
                        rows_per_match: rows_per_match
                            .apply_transform_with_path(transformer, node_path)?,
                        after_match_skip: after_match_skip
                            .apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        symbols: symbols
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableFunctionArgs {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { args, settings } = self;
            Self {
                args: args.apply_transform_with_path(transformer, node_path)?,
                settings: settings.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableOptionsClustered {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TableOptionsClustered::ColumnstoreIndex => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TableOptionsClustered::ColumnstoreIndex,
                        )?
                }
                sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(field0) => {
                    sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::TableOptionsClustered::Index(field0) => {
                    sqlparser::ast::TableOptionsClustered::Index(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableVersion {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TableVersion::ForSystemTimeAsOf(field0) => {
                    sqlparser::ast::TableVersion::ForSystemTimeAsOf(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TableWithJoins {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { relation, joins } = self;
            Self {
                relation: relation.apply_transform_with_path(transformer, node_path)?,
                joins: joins.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Tag {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { key, value } = self;
            Self {
                key: key.apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TagsColumnOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { with, tags } = self;
            Self {
                with: with.apply_transform_with_path(transformer, node_path)?,
                tags: tags.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TimezoneInfo {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TimezoneInfo::None => {
                    transformer.transform(node_path, sqlparser::ast::TimezoneInfo::None)?
                }
                sqlparser::ast::TimezoneInfo::WithTimeZone => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TimezoneInfo::WithTimeZone,
                        )?
                }
                sqlparser::ast::TimezoneInfo::WithoutTimeZone => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TimezoneInfo::WithoutTimeZone,
                        )?
                }
                sqlparser::ast::TimezoneInfo::Tz => {
                    transformer.transform(node_path, sqlparser::ast::TimezoneInfo::Tz)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Top {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { with_ties, percent, quantity } = self;
            Self {
                with_ties: with_ties.apply_transform_with_path(transformer, node_path)?,
                percent: percent.apply_transform_with_path(transformer, node_path)?,
                quantity: quantity.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TopQuantity {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TopQuantity::Expr(field0) => {
                    sqlparser::ast::TopQuantity::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::TopQuantity::Constant(field0) => {
                    sqlparser::ast::TopQuantity::Constant(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionAccessMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TransactionAccessMode::ReadOnly => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionAccessMode::ReadOnly,
                        )?
                }
                sqlparser::ast::TransactionAccessMode::ReadWrite => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionAccessMode::ReadWrite,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionIsolationLevel {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TransactionIsolationLevel::ReadUncommitted => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionIsolationLevel::ReadUncommitted,
                        )?
                }
                sqlparser::ast::TransactionIsolationLevel::ReadCommitted => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionIsolationLevel::ReadCommitted,
                        )?
                }
                sqlparser::ast::TransactionIsolationLevel::RepeatableRead => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionIsolationLevel::RepeatableRead,
                        )?
                }
                sqlparser::ast::TransactionIsolationLevel::Serializable => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionIsolationLevel::Serializable,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TransactionMode::AccessMode(field0) => {
                    sqlparser::ast::TransactionMode::AccessMode(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::TransactionMode::IsolationLevel(field0) => {
                    sqlparser::ast::TransactionMode::IsolationLevel(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TransactionModifier {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TransactionModifier::Deferred => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionModifier::Deferred,
                        )?
                }
                sqlparser::ast::TransactionModifier::Immediate => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionModifier::Immediate,
                        )?
                }
                sqlparser::ast::TransactionModifier::Exclusive => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TransactionModifier::Exclusive,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerEvent {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TriggerEvent::Insert => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerEvent::Insert)?
                }
                sqlparser::ast::TriggerEvent::Update(field0) => {
                    sqlparser::ast::TriggerEvent::Update(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::TriggerEvent::Delete => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerEvent::Delete)?
                }
                sqlparser::ast::TriggerEvent::Truncate => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerEvent::Truncate)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerExecBody {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { exec_type, func_desc } = self;
            Self {
                exec_type: exec_type.apply_transform_with_path(transformer, node_path)?,
                func_desc: func_desc.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerExecBodyType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TriggerExecBodyType::Function => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TriggerExecBodyType::Function,
                        )?
                }
                sqlparser::ast::TriggerExecBodyType::Procedure => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TriggerExecBodyType::Procedure,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TriggerObject::Row => {
                    transformer.transform(node_path, sqlparser::ast::TriggerObject::Row)?
                }
                sqlparser::ast::TriggerObject::Statement => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerObject::Statement)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerPeriod {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TriggerPeriod::After => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerPeriod::After)?
                }
                sqlparser::ast::TriggerPeriod::Before => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerPeriod::Before)?
                }
                sqlparser::ast::TriggerPeriod::InsteadOf => {
                    transformer
                        .transform(node_path, sqlparser::ast::TriggerPeriod::InsteadOf)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerReferencing {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { refer_type, is_as, transition_relation_name } = self;
            Self {
                refer_type: refer_type
                    .apply_transform_with_path(transformer, node_path)?,
                is_as: is_as.apply_transform_with_path(transformer, node_path)?,
                transition_relation_name: transition_relation_name
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TriggerReferencingType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TriggerReferencingType::OldTable => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TriggerReferencingType::OldTable,
                        )?
                }
                sqlparser::ast::TriggerReferencingType::NewTable => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TriggerReferencingType::NewTable,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TrimWhereField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TrimWhereField::Both => {
                    transformer
                        .transform(node_path, sqlparser::ast::TrimWhereField::Both)?
                }
                sqlparser::ast::TrimWhereField::Leading => {
                    transformer
                        .transform(node_path, sqlparser::ast::TrimWhereField::Leading)?
                }
                sqlparser::ast::TrimWhereField::Trailing => {
                    transformer
                        .transform(node_path, sqlparser::ast::TrimWhereField::Trailing)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateCascadeOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TruncateCascadeOption::Cascade => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TruncateCascadeOption::Cascade,
                        )?
                }
                sqlparser::ast::TruncateCascadeOption::Restrict => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TruncateCascadeOption::Restrict,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateIdentityOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::TruncateIdentityOption::Restart => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TruncateIdentityOption::Restart,
                        )?
                }
                sqlparser::ast::TruncateIdentityOption::Continue => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::TruncateIdentityOption::Continue,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::TruncateTableTarget {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UnaryOperator {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::UnaryOperator::Plus => {
                    transformer
                        .transform(node_path, sqlparser::ast::UnaryOperator::Plus)?
                }
                sqlparser::ast::UnaryOperator::Minus => {
                    transformer
                        .transform(node_path, sqlparser::ast::UnaryOperator::Minus)?
                }
                sqlparser::ast::UnaryOperator::Not => {
                    transformer.transform(node_path, sqlparser::ast::UnaryOperator::Not)?
                }
                sqlparser::ast::UnaryOperator::PGBitwiseNot => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::UnaryOperator::PGBitwiseNot,
                        )?
                }
                sqlparser::ast::UnaryOperator::PGSquareRoot => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::UnaryOperator::PGSquareRoot,
                        )?
                }
                sqlparser::ast::UnaryOperator::PGCubeRoot => {
                    transformer
                        .transform(node_path, sqlparser::ast::UnaryOperator::PGCubeRoot)?
                }
                sqlparser::ast::UnaryOperator::PGPostfixFactorial => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::UnaryOperator::PGPostfixFactorial,
                        )?
                }
                sqlparser::ast::UnaryOperator::PGPrefixFactorial => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::UnaryOperator::PGPrefixFactorial,
                        )?
                }
                sqlparser::ast::UnaryOperator::PGAbs => {
                    transformer
                        .transform(node_path, sqlparser::ast::UnaryOperator::PGAbs)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UnionField {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { field_name, field_type } = self;
            Self {
                field_name: field_name
                    .apply_transform_with_path(transformer, node_path)?,
                field_type: field_type.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Use {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Use::Catalog(field0) => {
                    sqlparser::ast::Use::Catalog(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Use::Schema(field0) => {
                    sqlparser::ast::Use::Schema(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Use::Database(field0) => {
                    sqlparser::ast::Use::Database(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Use::Warehouse(field0) => {
                    sqlparser::ast::Use::Warehouse(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Use::Object(field0) => {
                    sqlparser::ast::Use::Object(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Use::Default => {
                    transformer.transform(node_path, sqlparser::ast::Use::Default)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::UserDefinedTypeCompositeAttributeDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, data_type, collation } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                data_type: data_type.apply_transform_with_path(transformer, node_path)?,
                collation: collation.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UserDefinedTypeRepresentation {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::UserDefinedTypeRepresentation::Composite {
                    attributes,
                } => {
                    sqlparser::ast::UserDefinedTypeRepresentation::Composite {
                        attributes: attributes
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::ast::UserDefinedTypeRepresentation::Enum { labels } => {
                    sqlparser::ast::UserDefinedTypeRepresentation::Enum {
                        labels: labels.apply_transform_with_path(transformer, node_path)?,
                    }
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::UtilityOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, arg } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                arg: arg.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Value {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::Value::Number(field0, field1) => {
                    sqlparser::ast::Value::Number(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::SingleQuotedString(field0) => {
                    sqlparser::ast::Value::SingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::DollarQuotedString(field0) => {
                    sqlparser::ast::Value::DollarQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::TripleSingleQuotedString(field0) => {
                    sqlparser::ast::Value::TripleSingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::TripleDoubleQuotedString(field0) => {
                    sqlparser::ast::Value::TripleDoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::EscapedStringLiteral(field0) => {
                    sqlparser::ast::Value::EscapedStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::UnicodeStringLiteral(field0) => {
                    sqlparser::ast::Value::UnicodeStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::SingleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::SingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::DoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::DoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleSingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleDoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::SingleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::SingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::DoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::DoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleSingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::ast::Value::TripleDoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::NationalStringLiteral(field0) => {
                    sqlparser::ast::Value::NationalStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::HexStringLiteral(field0) => {
                    sqlparser::ast::Value::HexStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::DoubleQuotedString(field0) => {
                    sqlparser::ast::Value::DoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::Boolean(field0) => {
                    sqlparser::ast::Value::Boolean(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::Value::Null => {
                    transformer.transform(node_path, sqlparser::ast::Value::Null)?
                }
                sqlparser::ast::Value::Placeholder(field0) => {
                    sqlparser::ast::Value::Placeholder(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ValueTableMode {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::ValueTableMode::AsStruct => {
                    transformer
                        .transform(node_path, sqlparser::ast::ValueTableMode::AsStruct)?
                }
                sqlparser::ast::ValueTableMode::AsValue => {
                    transformer
                        .transform(node_path, sqlparser::ast::ValueTableMode::AsValue)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::Values {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { explicit_row, rows } = self;
            Self {
                explicit_row: explicit_row
                    .apply_transform_with_path(transformer, node_path)?,
                rows: rows.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::ViewColumnDef {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, data_type, options } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                data_type: data_type.apply_transform_with_path(transformer, node_path)?,
                options: options.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WildcardAdditionalOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { opt_ilike, opt_exclude, opt_except, opt_replace, opt_rename } = self;
            Self {
                opt_ilike: opt_ilike.apply_transform_with_path(transformer, node_path)?,
                opt_exclude: opt_exclude
                    .apply_transform_with_path(transformer, node_path)?,
                opt_except: opt_except
                    .apply_transform_with_path(transformer, node_path)?,
                opt_replace: opt_replace
                    .apply_transform_with_path(transformer, node_path)?,
                opt_rename: opt_rename.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrame {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { units, start_bound, end_bound } = self;
            Self {
                units: units.apply_transform_with_path(transformer, node_path)?,
                start_bound: start_bound
                    .apply_transform_with_path(transformer, node_path)?,
                end_bound: end_bound.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrameBound {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::WindowFrameBound::CurrentRow => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::WindowFrameBound::CurrentRow,
                        )?
                }
                sqlparser::ast::WindowFrameBound::Preceding(field0) => {
                    sqlparser::ast::WindowFrameBound::Preceding(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::WindowFrameBound::Following(field0) => {
                    sqlparser::ast::WindowFrameBound::Following(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowFrameUnits {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::WindowFrameUnits::Rows => {
                    transformer
                        .transform(node_path, sqlparser::ast::WindowFrameUnits::Rows)?
                }
                sqlparser::ast::WindowFrameUnits::Range => {
                    transformer
                        .transform(node_path, sqlparser::ast::WindowFrameUnits::Range)?
                }
                sqlparser::ast::WindowFrameUnits::Groups => {
                    transformer
                        .transform(node_path, sqlparser::ast::WindowFrameUnits::Groups)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowSpec {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { window_name, partition_by, order_by, window_frame } = self;
            Self {
                window_name: window_name
                    .apply_transform_with_path(transformer, node_path)?,
                partition_by: partition_by
                    .apply_transform_with_path(transformer, node_path)?,
                order_by: order_by.apply_transform_with_path(transformer, node_path)?,
                window_frame: window_frame
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WindowType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::WindowType::WindowSpec(field0) => {
                    sqlparser::ast::WindowType::WindowSpec(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::ast::WindowType::NamedWindow(field0) => {
                    sqlparser::ast::WindowType::NamedWindow(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::With {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { recursive, cte_tables } = self;
            Self {
                recursive: recursive.apply_transform_with_path(transformer, node_path)?,
                cte_tables: cte_tables.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::ast::WithFill {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { from, to, step } = self;
            Self {
                from: from.apply_transform_with_path(transformer, node_path)?,
                to: to.apply_transform_with_path(transformer, node_path)?,
                step: step.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOption {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { option_name, option_type, value } = self;
            Self {
                option_name: option_name
                    .apply_transform_with_path(transformer, node_path)?,
                option_type: option_type
                    .apply_transform_with_path(transformer, node_path)?,
                value: value.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING,
                        )?
                }
                sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN,
                        )?
                }
                sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM,
                        )?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::DataLoadingOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { options } = self;
            Self {
                options: options.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { alias, file_col_num, element, item_as } = self;
            Self {
                alias: alias.apply_transform_with_path(transformer, node_path)?,
                file_col_num: file_col_num
                    .apply_transform_with_path(transformer, node_path)?,
                element: element.apply_transform_with_path(transformer, node_path)?,
                item_as: item_as.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast>
for sqlparser::ast::helpers::stmt_data_loading::StageParamsObject {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { url, encryption, endpoint, storage_integration, credentials } = self;
            Self {
                url: url.apply_transform_with_path(transformer, node_path)?,
                encryption: encryption
                    .apply_transform_with_path(transformer, node_path)?,
                endpoint: endpoint.apply_transform_with_path(transformer, node_path)?,
                storage_integration: storage_integration
                    .apply_transform_with_path(transformer, node_path)?,
                credentials: credentials
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::keywords::Keyword {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::keywords::Keyword::NoKeyword => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NoKeyword)?
                }
                sqlparser::keywords::Keyword::ABORT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ABORT)?
                }
                sqlparser::keywords::Keyword::ABS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ABS)?
                }
                sqlparser::keywords::Keyword::ABSOLUTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ABSOLUTE)?
                }
                sqlparser::keywords::Keyword::ACCESS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ACCESS)?
                }
                sqlparser::keywords::Keyword::ACTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ACTION)?
                }
                sqlparser::keywords::Keyword::ADD => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ADD)?
                }
                sqlparser::keywords::Keyword::ADMIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ADMIN)?
                }
                sqlparser::keywords::Keyword::AFTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::AFTER)?
                }
                sqlparser::keywords::Keyword::AGAINST => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::AGAINST)?
                }
                sqlparser::keywords::Keyword::AGGREGATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::AGGREGATION)?
                }
                sqlparser::keywords::Keyword::ALIAS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ALIAS)?
                }
                sqlparser::keywords::Keyword::ALL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ALL)?
                }
                sqlparser::keywords::Keyword::ALLOCATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ALLOCATE)?
                }
                sqlparser::keywords::Keyword::ALTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ALTER)?
                }
                sqlparser::keywords::Keyword::ALWAYS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ALWAYS)?
                }
                sqlparser::keywords::Keyword::ANALYZE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ANALYZE)?
                }
                sqlparser::keywords::Keyword::AND => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::AND)?
                }
                sqlparser::keywords::Keyword::ANTI => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ANTI)?
                }
                sqlparser::keywords::Keyword::ANY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ANY)?
                }
                sqlparser::keywords::Keyword::APPLY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::APPLY)?
                }
                sqlparser::keywords::Keyword::ARCHIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ARCHIVE)?
                }
                sqlparser::keywords::Keyword::ARE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ARE)?
                }
                sqlparser::keywords::Keyword::ARRAY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ARRAY)?
                }
                sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::ARRAY_MAX_CARDINALITY,
                        )?
                }
                sqlparser::keywords::Keyword::AS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::AS)?
                }
                sqlparser::keywords::Keyword::ASC => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ASC)?
                }
                sqlparser::keywords::Keyword::ASENSITIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ASENSITIVE)?
                }
                sqlparser::keywords::Keyword::ASOF => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ASOF)?
                }
                sqlparser::keywords::Keyword::ASSERT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ASSERT)?
                }
                sqlparser::keywords::Keyword::ASYMMETRIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ASYMMETRIC)?
                }
                sqlparser::keywords::Keyword::AT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::AT)?
                }
                sqlparser::keywords::Keyword::ATOMIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ATOMIC)?
                }
                sqlparser::keywords::Keyword::ATTACH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ATTACH)?
                }
                sqlparser::keywords::Keyword::AUTHORIZATION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::AUTHORIZATION,
                        )?
                }
                sqlparser::keywords::Keyword::AUTO => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::AUTO)?
                }
                sqlparser::keywords::Keyword::AUTOINCREMENT => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::AUTOINCREMENT,
                        )?
                }
                sqlparser::keywords::Keyword::AUTO_INCREMENT => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::AUTO_INCREMENT,
                        )?
                }
                sqlparser::keywords::Keyword::AVG => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::AVG)?
                }
                sqlparser::keywords::Keyword::AVRO => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::AVRO)?
                }
                sqlparser::keywords::Keyword::BACKWARD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BACKWARD)?
                }
                sqlparser::keywords::Keyword::BASE64 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BASE64)?
                }
                sqlparser::keywords::Keyword::BEFORE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BEFORE)?
                }
                sqlparser::keywords::Keyword::BEGIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BEGIN)?
                }
                sqlparser::keywords::Keyword::BEGIN_FRAME => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BEGIN_FRAME)?
                }
                sqlparser::keywords::Keyword::BEGIN_PARTITION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::BEGIN_PARTITION,
                        )?
                }
                sqlparser::keywords::Keyword::BETWEEN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BETWEEN)?
                }
                sqlparser::keywords::Keyword::BIGDECIMAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BIGDECIMAL)?
                }
                sqlparser::keywords::Keyword::BIGINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BIGINT)?
                }
                sqlparser::keywords::Keyword::BIGNUMERIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BIGNUMERIC)?
                }
                sqlparser::keywords::Keyword::BINARY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BINARY)?
                }
                sqlparser::keywords::Keyword::BINDING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BINDING)?
                }
                sqlparser::keywords::Keyword::BLOB => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::BLOB)?
                }
                sqlparser::keywords::Keyword::BLOOMFILTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BLOOMFILTER)?
                }
                sqlparser::keywords::Keyword::BOOL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::BOOL)?
                }
                sqlparser::keywords::Keyword::BOOLEAN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BOOLEAN)?
                }
                sqlparser::keywords::Keyword::BOTH => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::BOTH)?
                }
                sqlparser::keywords::Keyword::BROWSE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BROWSE)?
                }
                sqlparser::keywords::Keyword::BTREE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BTREE)?
                }
                sqlparser::keywords::Keyword::BUCKETS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BUCKETS)?
                }
                sqlparser::keywords::Keyword::BY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::BY)?
                }
                sqlparser::keywords::Keyword::BYPASSRLS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BYPASSRLS)?
                }
                sqlparser::keywords::Keyword::BYTEA => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BYTEA)?
                }
                sqlparser::keywords::Keyword::BYTES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::BYTES)?
                }
                sqlparser::keywords::Keyword::CACHE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CACHE)?
                }
                sqlparser::keywords::Keyword::CALL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CALL)?
                }
                sqlparser::keywords::Keyword::CALLED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CALLED)?
                }
                sqlparser::keywords::Keyword::CARDINALITY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CARDINALITY)?
                }
                sqlparser::keywords::Keyword::CASCADE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CASCADE)?
                }
                sqlparser::keywords::Keyword::CASCADED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CASCADED)?
                }
                sqlparser::keywords::Keyword::CASE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CASE)?
                }
                sqlparser::keywords::Keyword::CAST => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CAST)?
                }
                sqlparser::keywords::Keyword::CATALOG => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CATALOG)?
                }
                sqlparser::keywords::Keyword::CEIL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CEIL)?
                }
                sqlparser::keywords::Keyword::CEILING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CEILING)?
                }
                sqlparser::keywords::Keyword::CENTURY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CENTURY)?
                }
                sqlparser::keywords::Keyword::CHAIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHAIN)?
                }
                sqlparser::keywords::Keyword::CHANGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHANGE)?
                }
                sqlparser::keywords::Keyword::CHANGE_TRACKING => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CHANGE_TRACKING,
                        )?
                }
                sqlparser::keywords::Keyword::CHANNEL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHANNEL)?
                }
                sqlparser::keywords::Keyword::CHAR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CHAR)?
                }
                sqlparser::keywords::Keyword::CHARACTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHARACTER)?
                }
                sqlparser::keywords::Keyword::CHARACTERS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHARACTERS)?
                }
                sqlparser::keywords::Keyword::CHARACTER_LENGTH => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CHARACTER_LENGTH,
                        )?
                }
                sqlparser::keywords::Keyword::CHARSET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHARSET)?
                }
                sqlparser::keywords::Keyword::CHAR_LENGTH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHAR_LENGTH)?
                }
                sqlparser::keywords::Keyword::CHECK => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CHECK)?
                }
                sqlparser::keywords::Keyword::CLEAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CLEAR)?
                }
                sqlparser::keywords::Keyword::CLOB => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CLOB)?
                }
                sqlparser::keywords::Keyword::CLONE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CLONE)?
                }
                sqlparser::keywords::Keyword::CLOSE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CLOSE)?
                }
                sqlparser::keywords::Keyword::CLUSTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CLUSTER)?
                }
                sqlparser::keywords::Keyword::CLUSTERED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CLUSTERED)?
                }
                sqlparser::keywords::Keyword::COALESCE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COALESCE)?
                }
                sqlparser::keywords::Keyword::COLLATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLLATE)?
                }
                sqlparser::keywords::Keyword::COLLATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLLATION)?
                }
                sqlparser::keywords::Keyword::COLLECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLLECT)?
                }
                sqlparser::keywords::Keyword::COLLECTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLLECTION)?
                }
                sqlparser::keywords::Keyword::COLUMN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLUMN)?
                }
                sqlparser::keywords::Keyword::COLUMNS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLUMNS)?
                }
                sqlparser::keywords::Keyword::COLUMNSTORE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COLUMNSTORE)?
                }
                sqlparser::keywords::Keyword::COMMENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COMMENT)?
                }
                sqlparser::keywords::Keyword::COMMIT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COMMIT)?
                }
                sqlparser::keywords::Keyword::COMMITTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COMMITTED)?
                }
                sqlparser::keywords::Keyword::COMPRESSION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COMPRESSION)?
                }
                sqlparser::keywords::Keyword::COMPUTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COMPUTE)?
                }
                sqlparser::keywords::Keyword::CONCURRENTLY => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CONCURRENTLY,
                        )?
                }
                sqlparser::keywords::Keyword::CONDITION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONDITION)?
                }
                sqlparser::keywords::Keyword::CONFLICT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONFLICT)?
                }
                sqlparser::keywords::Keyword::CONNECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONNECT)?
                }
                sqlparser::keywords::Keyword::CONNECTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONNECTION)?
                }
                sqlparser::keywords::Keyword::CONSTRAINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONSTRAINT)?
                }
                sqlparser::keywords::Keyword::CONTAINS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONTAINS)?
                }
                sqlparser::keywords::Keyword::CONTINUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONTINUE)?
                }
                sqlparser::keywords::Keyword::CONVERT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CONVERT)?
                }
                sqlparser::keywords::Keyword::COPY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::COPY)?
                }
                sqlparser::keywords::Keyword::COPY_OPTIONS => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::COPY_OPTIONS,
                        )?
                }
                sqlparser::keywords::Keyword::CORR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CORR)?
                }
                sqlparser::keywords::Keyword::CORRESPONDING => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CORRESPONDING,
                        )?
                }
                sqlparser::keywords::Keyword::COUNT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COUNT)?
                }
                sqlparser::keywords::Keyword::COVAR_POP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COVAR_POP)?
                }
                sqlparser::keywords::Keyword::COVAR_SAMP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::COVAR_SAMP)?
                }
                sqlparser::keywords::Keyword::CREATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CREATE)?
                }
                sqlparser::keywords::Keyword::CREATEDB => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CREATEDB)?
                }
                sqlparser::keywords::Keyword::CREATEROLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CREATEROLE)?
                }
                sqlparser::keywords::Keyword::CREDENTIALS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CREDENTIALS)?
                }
                sqlparser::keywords::Keyword::CROSS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CROSS)?
                }
                sqlparser::keywords::Keyword::CSV => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CSV)?
                }
                sqlparser::keywords::Keyword::CUBE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::CUBE)?
                }
                sqlparser::keywords::Keyword::CUME_DIST => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CUME_DIST)?
                }
                sqlparser::keywords::Keyword::CURRENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CURRENT)?
                }
                sqlparser::keywords::Keyword::CURRENT_CATALOG => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_CATALOG,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_DATE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_DATE,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_PATH => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_PATH,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_ROLE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_ROLE,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_ROW => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CURRENT_ROW)?
                }
                sqlparser::keywords::Keyword::CURRENT_SCHEMA => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_SCHEMA,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_TIME => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_TIME,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_TIMESTAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_TIMESTAMP,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE,
                        )?
                }
                sqlparser::keywords::Keyword::CURRENT_USER => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::CURRENT_USER,
                        )?
                }
                sqlparser::keywords::Keyword::CURSOR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CURSOR)?
                }
                sqlparser::keywords::Keyword::CYCLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::CYCLE)?
                }
                sqlparser::keywords::Keyword::DATA => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DATA)?
                }
                sqlparser::keywords::Keyword::DATABASE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DATABASE)?
                }
                sqlparser::keywords::Keyword::DATABASES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DATABASES)?
                }
                sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS,
                        )?
                }
                sqlparser::keywords::Keyword::DATE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DATE)?
                }
                sqlparser::keywords::Keyword::DATE32 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DATE32)?
                }
                sqlparser::keywords::Keyword::DATETIME => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DATETIME)?
                }
                sqlparser::keywords::Keyword::DATETIME64 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DATETIME64)?
                }
                sqlparser::keywords::Keyword::DAY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DAY)?
                }
                sqlparser::keywords::Keyword::DAYOFWEEK => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DAYOFWEEK)?
                }
                sqlparser::keywords::Keyword::DAYOFYEAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DAYOFYEAR)?
                }
                sqlparser::keywords::Keyword::DEALLOCATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEALLOCATE)?
                }
                sqlparser::keywords::Keyword::DEC => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DEC)?
                }
                sqlparser::keywords::Keyword::DECADE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DECADE)?
                }
                sqlparser::keywords::Keyword::DECIMAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DECIMAL)?
                }
                sqlparser::keywords::Keyword::DECLARE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DECLARE)?
                }
                sqlparser::keywords::Keyword::DEDUPLICATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEDUPLICATE)?
                }
                sqlparser::keywords::Keyword::DEFAULT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEFAULT)?
                }
                sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::DEFAULT_DDL_COLLATION,
                        )?
                }
                sqlparser::keywords::Keyword::DEFERRABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEFERRABLE)?
                }
                sqlparser::keywords::Keyword::DEFERRED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEFERRED)?
                }
                sqlparser::keywords::Keyword::DEFINE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEFINE)?
                }
                sqlparser::keywords::Keyword::DEFINED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEFINED)?
                }
                sqlparser::keywords::Keyword::DELAYED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DELAYED)?
                }
                sqlparser::keywords::Keyword::DELETE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DELETE)?
                }
                sqlparser::keywords::Keyword::DELIMITED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DELIMITED)?
                }
                sqlparser::keywords::Keyword::DELIMITER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DELIMITER)?
                }
                sqlparser::keywords::Keyword::DELTA => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DELTA)?
                }
                sqlparser::keywords::Keyword::DENSE_RANK => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DENSE_RANK)?
                }
                sqlparser::keywords::Keyword::DEREF => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DEREF)?
                }
                sqlparser::keywords::Keyword::DESC => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DESC)?
                }
                sqlparser::keywords::Keyword::DESCRIBE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DESCRIBE)?
                }
                sqlparser::keywords::Keyword::DETACH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DETACH)?
                }
                sqlparser::keywords::Keyword::DETAIL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DETAIL)?
                }
                sqlparser::keywords::Keyword::DETERMINISTIC => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::DETERMINISTIC,
                        )?
                }
                sqlparser::keywords::Keyword::DIRECTORY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DIRECTORY)?
                }
                sqlparser::keywords::Keyword::DISABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DISABLE)?
                }
                sqlparser::keywords::Keyword::DISCARD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DISCARD)?
                }
                sqlparser::keywords::Keyword::DISCONNECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DISCONNECT)?
                }
                sqlparser::keywords::Keyword::DISTINCT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DISTINCT)?
                }
                sqlparser::keywords::Keyword::DISTRIBUTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DISTRIBUTE)?
                }
                sqlparser::keywords::Keyword::DIV => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DIV)?
                }
                sqlparser::keywords::Keyword::DO => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DO)?
                }
                sqlparser::keywords::Keyword::DOUBLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DOUBLE)?
                }
                sqlparser::keywords::Keyword::DOW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DOW)?
                }
                sqlparser::keywords::Keyword::DOY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DOY)?
                }
                sqlparser::keywords::Keyword::DROP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DROP)?
                }
                sqlparser::keywords::Keyword::DRY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::DRY)?
                }
                sqlparser::keywords::Keyword::DUPLICATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DUPLICATE)?
                }
                sqlparser::keywords::Keyword::DYNAMIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::DYNAMIC)?
                }
                sqlparser::keywords::Keyword::EACH => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::EACH)?
                }
                sqlparser::keywords::Keyword::ELEMENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ELEMENT)?
                }
                sqlparser::keywords::Keyword::ELEMENTS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ELEMENTS)?
                }
                sqlparser::keywords::Keyword::ELSE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ELSE)?
                }
                sqlparser::keywords::Keyword::EMPTY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EMPTY)?
                }
                sqlparser::keywords::Keyword::ENABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ENABLE)?
                }
                sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION,
                        )?
                }
                sqlparser::keywords::Keyword::ENCODING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ENCODING)?
                }
                sqlparser::keywords::Keyword::ENCRYPTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ENCRYPTION)?
                }
                sqlparser::keywords::Keyword::END => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::END)?
                }
                sqlparser::keywords::Keyword::END_EXEC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::END_EXEC)?
                }
                sqlparser::keywords::Keyword::ENDPOINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ENDPOINT)?
                }
                sqlparser::keywords::Keyword::END_FRAME => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::END_FRAME)?
                }
                sqlparser::keywords::Keyword::END_PARTITION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::END_PARTITION,
                        )?
                }
                sqlparser::keywords::Keyword::ENFORCED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ENFORCED)?
                }
                sqlparser::keywords::Keyword::ENGINE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ENGINE)?
                }
                sqlparser::keywords::Keyword::ENUM => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ENUM)?
                }
                sqlparser::keywords::Keyword::EPHEMERAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EPHEMERAL)?
                }
                sqlparser::keywords::Keyword::EPOCH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EPOCH)?
                }
                sqlparser::keywords::Keyword::EQUALS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EQUALS)?
                }
                sqlparser::keywords::Keyword::ERROR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ERROR)?
                }
                sqlparser::keywords::Keyword::ESCAPE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ESCAPE)?
                }
                sqlparser::keywords::Keyword::ESCAPED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ESCAPED)?
                }
                sqlparser::keywords::Keyword::EVENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EVENT)?
                }
                sqlparser::keywords::Keyword::EVERY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EVERY)?
                }
                sqlparser::keywords::Keyword::EXCEPT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXCEPT)?
                }
                sqlparser::keywords::Keyword::EXCEPTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXCEPTION)?
                }
                sqlparser::keywords::Keyword::EXCLUDE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXCLUDE)?
                }
                sqlparser::keywords::Keyword::EXCLUSIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXCLUSIVE)?
                }
                sqlparser::keywords::Keyword::EXEC => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::EXEC)?
                }
                sqlparser::keywords::Keyword::EXECUTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXECUTE)?
                }
                sqlparser::keywords::Keyword::EXISTS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXISTS)?
                }
                sqlparser::keywords::Keyword::EXP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::EXP)?
                }
                sqlparser::keywords::Keyword::EXPANSION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXPANSION)?
                }
                sqlparser::keywords::Keyword::EXPLAIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXPLAIN)?
                }
                sqlparser::keywords::Keyword::EXPLICIT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXPLICIT)?
                }
                sqlparser::keywords::Keyword::EXPORT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXPORT)?
                }
                sqlparser::keywords::Keyword::EXTENDED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXTENDED)?
                }
                sqlparser::keywords::Keyword::EXTENSION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXTENSION)?
                }
                sqlparser::keywords::Keyword::EXTERNAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXTERNAL)?
                }
                sqlparser::keywords::Keyword::EXTRACT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::EXTRACT)?
                }
                sqlparser::keywords::Keyword::FAIL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FAIL)?
                }
                sqlparser::keywords::Keyword::FALSE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FALSE)?
                }
                sqlparser::keywords::Keyword::FETCH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FETCH)?
                }
                sqlparser::keywords::Keyword::FIELDS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FIELDS)?
                }
                sqlparser::keywords::Keyword::FILE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FILE)?
                }
                sqlparser::keywords::Keyword::FILES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FILES)?
                }
                sqlparser::keywords::Keyword::FILE_FORMAT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FILE_FORMAT)?
                }
                sqlparser::keywords::Keyword::FILL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FILL)?
                }
                sqlparser::keywords::Keyword::FILTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FILTER)?
                }
                sqlparser::keywords::Keyword::FINAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FINAL)?
                }
                sqlparser::keywords::Keyword::FIRST => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FIRST)?
                }
                sqlparser::keywords::Keyword::FIRST_VALUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FIRST_VALUE)?
                }
                sqlparser::keywords::Keyword::FIXEDSTRING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FIXEDSTRING)?
                }
                sqlparser::keywords::Keyword::FLOAT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLOAT)?
                }
                sqlparser::keywords::Keyword::FLOAT32 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLOAT32)?
                }
                sqlparser::keywords::Keyword::FLOAT4 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLOAT4)?
                }
                sqlparser::keywords::Keyword::FLOAT64 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLOAT64)?
                }
                sqlparser::keywords::Keyword::FLOAT8 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLOAT8)?
                }
                sqlparser::keywords::Keyword::FLOOR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLOOR)?
                }
                sqlparser::keywords::Keyword::FLUSH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FLUSH)?
                }
                sqlparser::keywords::Keyword::FOLLOWING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FOLLOWING)?
                }
                sqlparser::keywords::Keyword::FOR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FOR)?
                }
                sqlparser::keywords::Keyword::FORCE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FORCE)?
                }
                sqlparser::keywords::Keyword::FORCE_NOT_NULL => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::FORCE_NOT_NULL,
                        )?
                }
                sqlparser::keywords::Keyword::FORCE_NULL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FORCE_NULL)?
                }
                sqlparser::keywords::Keyword::FORCE_QUOTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FORCE_QUOTE)?
                }
                sqlparser::keywords::Keyword::FOREIGN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FOREIGN)?
                }
                sqlparser::keywords::Keyword::FORMAT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FORMAT)?
                }
                sqlparser::keywords::Keyword::FORMATTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FORMATTED)?
                }
                sqlparser::keywords::Keyword::FORWARD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FORWARD)?
                }
                sqlparser::keywords::Keyword::FRAME_ROW => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FRAME_ROW)?
                }
                sqlparser::keywords::Keyword::FREE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FREE)?
                }
                sqlparser::keywords::Keyword::FREEZE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FREEZE)?
                }
                sqlparser::keywords::Keyword::FROM => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FROM)?
                }
                sqlparser::keywords::Keyword::FSCK => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FSCK)?
                }
                sqlparser::keywords::Keyword::FULL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::FULL)?
                }
                sqlparser::keywords::Keyword::FULLTEXT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FULLTEXT)?
                }
                sqlparser::keywords::Keyword::FUNCTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FUNCTION)?
                }
                sqlparser::keywords::Keyword::FUNCTIONS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FUNCTIONS)?
                }
                sqlparser::keywords::Keyword::FUSION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::FUSION)?
                }
                sqlparser::keywords::Keyword::GENERAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GENERAL)?
                }
                sqlparser::keywords::Keyword::GENERATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GENERATE)?
                }
                sqlparser::keywords::Keyword::GENERATED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GENERATED)?
                }
                sqlparser::keywords::Keyword::GEOGRAPHY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GEOGRAPHY)?
                }
                sqlparser::keywords::Keyword::GET => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::GET)?
                }
                sqlparser::keywords::Keyword::GLOBAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GLOBAL)?
                }
                sqlparser::keywords::Keyword::GRANT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GRANT)?
                }
                sqlparser::keywords::Keyword::GRANTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GRANTED)?
                }
                sqlparser::keywords::Keyword::GRANTS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GRANTS)?
                }
                sqlparser::keywords::Keyword::GRAPHVIZ => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GRAPHVIZ)?
                }
                sqlparser::keywords::Keyword::GROUP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GROUP)?
                }
                sqlparser::keywords::Keyword::GROUPING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GROUPING)?
                }
                sqlparser::keywords::Keyword::GROUPS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::GROUPS)?
                }
                sqlparser::keywords::Keyword::HASH => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::HASH)?
                }
                sqlparser::keywords::Keyword::HAVING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::HAVING)?
                }
                sqlparser::keywords::Keyword::HEADER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::HEADER)?
                }
                sqlparser::keywords::Keyword::HEAP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::HEAP)?
                }
                sqlparser::keywords::Keyword::HIGH_PRIORITY => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::HIGH_PRIORITY,
                        )?
                }
                sqlparser::keywords::Keyword::HISTORY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::HISTORY)?
                }
                sqlparser::keywords::Keyword::HIVEVAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::HIVEVAR)?
                }
                sqlparser::keywords::Keyword::HOLD => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::HOLD)?
                }
                sqlparser::keywords::Keyword::HOSTS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::HOSTS)?
                }
                sqlparser::keywords::Keyword::HOUR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::HOUR)?
                }
                sqlparser::keywords::Keyword::HOURS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::HOURS)?
                }
                sqlparser::keywords::Keyword::ID => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ID)?
                }
                sqlparser::keywords::Keyword::IDENTITY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::IDENTITY)?
                }
                sqlparser::keywords::Keyword::IF => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::IF)?
                }
                sqlparser::keywords::Keyword::IGNORE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::IGNORE)?
                }
                sqlparser::keywords::Keyword::ILIKE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ILIKE)?
                }
                sqlparser::keywords::Keyword::IMMEDIATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::IMMEDIATE)?
                }
                sqlparser::keywords::Keyword::IMMUTABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::IMMUTABLE)?
                }
                sqlparser::keywords::Keyword::IN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::IN)?
                }
                sqlparser::keywords::Keyword::INCLUDE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INCLUDE)?
                }
                sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::INCLUDE_NULL_VALUES,
                        )?
                }
                sqlparser::keywords::Keyword::INCREMENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INCREMENT)?
                }
                sqlparser::keywords::Keyword::INDEX => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INDEX)?
                }
                sqlparser::keywords::Keyword::INDICATOR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INDICATOR)?
                }
                sqlparser::keywords::Keyword::INHERIT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INHERIT)?
                }
                sqlparser::keywords::Keyword::INITIALLY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INITIALLY)?
                }
                sqlparser::keywords::Keyword::INNER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INNER)?
                }
                sqlparser::keywords::Keyword::INOUT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INOUT)?
                }
                sqlparser::keywords::Keyword::INPUT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INPUT)?
                }
                sqlparser::keywords::Keyword::INPUTFORMAT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INPUTFORMAT)?
                }
                sqlparser::keywords::Keyword::INSENSITIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INSENSITIVE)?
                }
                sqlparser::keywords::Keyword::INSERT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INSERT)?
                }
                sqlparser::keywords::Keyword::INSTALL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INSTALL)?
                }
                sqlparser::keywords::Keyword::INSTEAD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INSTEAD)?
                }
                sqlparser::keywords::Keyword::INT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::INT)?
                }
                sqlparser::keywords::Keyword::INT128 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INT128)?
                }
                sqlparser::keywords::Keyword::INT16 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INT16)?
                }
                sqlparser::keywords::Keyword::INT2 => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::INT2)?
                }
                sqlparser::keywords::Keyword::INT256 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INT256)?
                }
                sqlparser::keywords::Keyword::INT32 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INT32)?
                }
                sqlparser::keywords::Keyword::INT4 => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::INT4)?
                }
                sqlparser::keywords::Keyword::INT64 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INT64)?
                }
                sqlparser::keywords::Keyword::INT8 => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::INT8)?
                }
                sqlparser::keywords::Keyword::INTEGER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INTEGER)?
                }
                sqlparser::keywords::Keyword::INTERPOLATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INTERPOLATE)?
                }
                sqlparser::keywords::Keyword::INTERSECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INTERSECT)?
                }
                sqlparser::keywords::Keyword::INTERSECTION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::INTERSECTION,
                        )?
                }
                sqlparser::keywords::Keyword::INTERVAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::INTERVAL)?
                }
                sqlparser::keywords::Keyword::INTO => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::INTO)?
                }
                sqlparser::keywords::Keyword::IS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::IS)?
                }
                sqlparser::keywords::Keyword::ISODOW => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ISODOW)?
                }
                sqlparser::keywords::Keyword::ISOLATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ISOLATION)?
                }
                sqlparser::keywords::Keyword::ISOWEEK => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ISOWEEK)?
                }
                sqlparser::keywords::Keyword::ISOYEAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ISOYEAR)?
                }
                sqlparser::keywords::Keyword::ITEMS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ITEMS)?
                }
                sqlparser::keywords::Keyword::JAR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::JAR)?
                }
                sqlparser::keywords::Keyword::JOIN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::JOIN)?
                }
                sqlparser::keywords::Keyword::JSON => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::JSON)?
                }
                sqlparser::keywords::Keyword::JSONB => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::JSONB)?
                }
                sqlparser::keywords::Keyword::JSONFILE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::JSONFILE)?
                }
                sqlparser::keywords::Keyword::JSON_TABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::JSON_TABLE)?
                }
                sqlparser::keywords::Keyword::JULIAN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::JULIAN)?
                }
                sqlparser::keywords::Keyword::KEY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::KEY)?
                }
                sqlparser::keywords::Keyword::KEYS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::KEYS)?
                }
                sqlparser::keywords::Keyword::KILL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::KILL)?
                }
                sqlparser::keywords::Keyword::LAG => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LAG)?
                }
                sqlparser::keywords::Keyword::LANGUAGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LANGUAGE)?
                }
                sqlparser::keywords::Keyword::LARGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LARGE)?
                }
                sqlparser::keywords::Keyword::LAST => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LAST)?
                }
                sqlparser::keywords::Keyword::LAST_VALUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LAST_VALUE)?
                }
                sqlparser::keywords::Keyword::LATERAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LATERAL)?
                }
                sqlparser::keywords::Keyword::LEAD => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LEAD)?
                }
                sqlparser::keywords::Keyword::LEADING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LEADING)?
                }
                sqlparser::keywords::Keyword::LEFT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LEFT)?
                }
                sqlparser::keywords::Keyword::LEVEL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LEVEL)?
                }
                sqlparser::keywords::Keyword::LIKE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LIKE)?
                }
                sqlparser::keywords::Keyword::LIKE_REGEX => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LIKE_REGEX)?
                }
                sqlparser::keywords::Keyword::LIMIT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LIMIT)?
                }
                sqlparser::keywords::Keyword::LINES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LINES)?
                }
                sqlparser::keywords::Keyword::LISTEN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LISTEN)?
                }
                sqlparser::keywords::Keyword::LN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LN)?
                }
                sqlparser::keywords::Keyword::LOAD => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LOAD)?
                }
                sqlparser::keywords::Keyword::LOCAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LOCAL)?
                }
                sqlparser::keywords::Keyword::LOCALTIME => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LOCALTIME)?
                }
                sqlparser::keywords::Keyword::LOCALTIMESTAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::LOCALTIMESTAMP,
                        )?
                }
                sqlparser::keywords::Keyword::LOCATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LOCATION)?
                }
                sqlparser::keywords::Keyword::LOCK => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LOCK)?
                }
                sqlparser::keywords::Keyword::LOCKED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LOCKED)?
                }
                sqlparser::keywords::Keyword::LOGIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LOGIN)?
                }
                sqlparser::keywords::Keyword::LOGS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::LOGS)?
                }
                sqlparser::keywords::Keyword::LOWCARDINALITY => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::LOWCARDINALITY,
                        )?
                }
                sqlparser::keywords::Keyword::LOWER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::LOWER)?
                }
                sqlparser::keywords::Keyword::LOW_PRIORITY => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::LOW_PRIORITY,
                        )?
                }
                sqlparser::keywords::Keyword::MACRO => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MACRO)?
                }
                sqlparser::keywords::Keyword::MANAGEDLOCATION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MANAGEDLOCATION,
                        )?
                }
                sqlparser::keywords::Keyword::MAP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::MAP)?
                }
                sqlparser::keywords::Keyword::MASKING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MASKING)?
                }
                sqlparser::keywords::Keyword::MATCH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MATCH)?
                }
                sqlparser::keywords::Keyword::MATCHED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MATCHED)?
                }
                sqlparser::keywords::Keyword::MATCHES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MATCHES)?
                }
                sqlparser::keywords::Keyword::MATCH_CONDITION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MATCH_CONDITION,
                        )?
                }
                sqlparser::keywords::Keyword::MATCH_RECOGNIZE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MATCH_RECOGNIZE,
                        )?
                }
                sqlparser::keywords::Keyword::MATERIALIZE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MATERIALIZE)?
                }
                sqlparser::keywords::Keyword::MATERIALIZED => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MATERIALIZED,
                        )?
                }
                sqlparser::keywords::Keyword::MAX => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::MAX)?
                }
                sqlparser::keywords::Keyword::MAXVALUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MAXVALUE)?
                }
                sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS,
                        )?
                }
                sqlparser::keywords::Keyword::MEASURES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MEASURES)?
                }
                sqlparser::keywords::Keyword::MEDIUMINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MEDIUMINT)?
                }
                sqlparser::keywords::Keyword::MEMBER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MEMBER)?
                }
                sqlparser::keywords::Keyword::MERGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MERGE)?
                }
                sqlparser::keywords::Keyword::METADATA => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::METADATA)?
                }
                sqlparser::keywords::Keyword::METHOD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::METHOD)?
                }
                sqlparser::keywords::Keyword::MICROSECOND => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MICROSECOND)?
                }
                sqlparser::keywords::Keyword::MICROSECONDS => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MICROSECONDS,
                        )?
                }
                sqlparser::keywords::Keyword::MILLENIUM => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MILLENIUM)?
                }
                sqlparser::keywords::Keyword::MILLENNIUM => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MILLENNIUM)?
                }
                sqlparser::keywords::Keyword::MILLISECOND => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MILLISECOND)?
                }
                sqlparser::keywords::Keyword::MILLISECONDS => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::MILLISECONDS,
                        )?
                }
                sqlparser::keywords::Keyword::MIN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::MIN)?
                }
                sqlparser::keywords::Keyword::MINUTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MINUTE)?
                }
                sqlparser::keywords::Keyword::MINVALUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MINVALUE)?
                }
                sqlparser::keywords::Keyword::MOD => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::MOD)?
                }
                sqlparser::keywords::Keyword::MODE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::MODE)?
                }
                sqlparser::keywords::Keyword::MODIFIES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MODIFIES)?
                }
                sqlparser::keywords::Keyword::MODIFY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MODIFY)?
                }
                sqlparser::keywords::Keyword::MODULE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MODULE)?
                }
                sqlparser::keywords::Keyword::MONTH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MONTH)?
                }
                sqlparser::keywords::Keyword::MSCK => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::MSCK)?
                }
                sqlparser::keywords::Keyword::MULTISET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MULTISET)?
                }
                sqlparser::keywords::Keyword::MUTATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::MUTATION)?
                }
                sqlparser::keywords::Keyword::NAME => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NAME)?
                }
                sqlparser::keywords::Keyword::NANOSECOND => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NANOSECOND)?
                }
                sqlparser::keywords::Keyword::NANOSECONDS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NANOSECONDS)?
                }
                sqlparser::keywords::Keyword::NATIONAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NATIONAL)?
                }
                sqlparser::keywords::Keyword::NATURAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NATURAL)?
                }
                sqlparser::keywords::Keyword::NCHAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NCHAR)?
                }
                sqlparser::keywords::Keyword::NCLOB => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NCLOB)?
                }
                sqlparser::keywords::Keyword::NESTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NESTED)?
                }
                sqlparser::keywords::Keyword::NEW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NEW)?
                }
                sqlparser::keywords::Keyword::NEXT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NEXT)?
                }
                sqlparser::keywords::Keyword::NO => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NO)?
                }
                sqlparser::keywords::Keyword::NOBYPASSRLS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOBYPASSRLS)?
                }
                sqlparser::keywords::Keyword::NOCREATEDB => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOCREATEDB)?
                }
                sqlparser::keywords::Keyword::NOCREATEROLE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::NOCREATEROLE,
                        )?
                }
                sqlparser::keywords::Keyword::NOINHERIT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOINHERIT)?
                }
                sqlparser::keywords::Keyword::NOLOGIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOLOGIN)?
                }
                sqlparser::keywords::Keyword::NONE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NONE)?
                }
                sqlparser::keywords::Keyword::NOORDER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOORDER)?
                }
                sqlparser::keywords::Keyword::NOREPLICATION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::NOREPLICATION,
                        )?
                }
                sqlparser::keywords::Keyword::NORMALIZE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NORMALIZE)?
                }
                sqlparser::keywords::Keyword::NOSCAN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOSCAN)?
                }
                sqlparser::keywords::Keyword::NOSUPERUSER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOSUPERUSER)?
                }
                sqlparser::keywords::Keyword::NOT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NOT)?
                }
                sqlparser::keywords::Keyword::NOTHING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOTHING)?
                }
                sqlparser::keywords::Keyword::NOTIFY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOTIFY)?
                }
                sqlparser::keywords::Keyword::NOWAIT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NOWAIT)?
                }
                sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::NO_WRITE_TO_BINLOG,
                        )?
                }
                sqlparser::keywords::Keyword::NTH_VALUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NTH_VALUE)?
                }
                sqlparser::keywords::Keyword::NTILE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NTILE)?
                }
                sqlparser::keywords::Keyword::NULL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::NULL)?
                }
                sqlparser::keywords::Keyword::NULLABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NULLABLE)?
                }
                sqlparser::keywords::Keyword::NULLIF => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NULLIF)?
                }
                sqlparser::keywords::Keyword::NULLS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NULLS)?
                }
                sqlparser::keywords::Keyword::NUMERIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NUMERIC)?
                }
                sqlparser::keywords::Keyword::NVARCHAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::NVARCHAR)?
                }
                sqlparser::keywords::Keyword::OBJECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OBJECT)?
                }
                sqlparser::keywords::Keyword::OCCURRENCES_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::OCCURRENCES_REGEX,
                        )?
                }
                sqlparser::keywords::Keyword::OCTETS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OCTETS)?
                }
                sqlparser::keywords::Keyword::OCTET_LENGTH => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::OCTET_LENGTH,
                        )?
                }
                sqlparser::keywords::Keyword::OF => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OF)?
                }
                sqlparser::keywords::Keyword::OFFSET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OFFSET)?
                }
                sqlparser::keywords::Keyword::OLD => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OLD)?
                }
                sqlparser::keywords::Keyword::OMIT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OMIT)?
                }
                sqlparser::keywords::Keyword::ON => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ON)?
                }
                sqlparser::keywords::Keyword::ONE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ONE)?
                }
                sqlparser::keywords::Keyword::ONLY => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ONLY)?
                }
                sqlparser::keywords::Keyword::OPEN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OPEN)?
                }
                sqlparser::keywords::Keyword::OPERATOR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OPERATOR)?
                }
                sqlparser::keywords::Keyword::OPTIMIZE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OPTIMIZE)?
                }
                sqlparser::keywords::Keyword::OPTIMIZER_COSTS => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::OPTIMIZER_COSTS,
                        )?
                }
                sqlparser::keywords::Keyword::OPTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OPTION)?
                }
                sqlparser::keywords::Keyword::OPTIONS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OPTIONS)?
                }
                sqlparser::keywords::Keyword::OR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OR)?
                }
                sqlparser::keywords::Keyword::ORC => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ORC)?
                }
                sqlparser::keywords::Keyword::ORDER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ORDER)?
                }
                sqlparser::keywords::Keyword::ORDINALITY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ORDINALITY)?
                }
                sqlparser::keywords::Keyword::OUT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OUT)?
                }
                sqlparser::keywords::Keyword::OUTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OUTER)?
                }
                sqlparser::keywords::Keyword::OUTPUTFORMAT => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::OUTPUTFORMAT,
                        )?
                }
                sqlparser::keywords::Keyword::OVER => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::OVER)?
                }
                sqlparser::keywords::Keyword::OVERFLOW => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OVERFLOW)?
                }
                sqlparser::keywords::Keyword::OVERLAPS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OVERLAPS)?
                }
                sqlparser::keywords::Keyword::OVERLAY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OVERLAY)?
                }
                sqlparser::keywords::Keyword::OVERWRITE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OVERWRITE)?
                }
                sqlparser::keywords::Keyword::OWNED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OWNED)?
                }
                sqlparser::keywords::Keyword::OWNER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::OWNER)?
                }
                sqlparser::keywords::Keyword::PARALLEL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PARALLEL)?
                }
                sqlparser::keywords::Keyword::PARAMETER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PARAMETER)?
                }
                sqlparser::keywords::Keyword::PARQUET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PARQUET)?
                }
                sqlparser::keywords::Keyword::PART => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::PART)?
                }
                sqlparser::keywords::Keyword::PARTITION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PARTITION)?
                }
                sqlparser::keywords::Keyword::PARTITIONED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PARTITIONED)?
                }
                sqlparser::keywords::Keyword::PARTITIONS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PARTITIONS)?
                }
                sqlparser::keywords::Keyword::PASSWORD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PASSWORD)?
                }
                sqlparser::keywords::Keyword::PAST => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::PAST)?
                }
                sqlparser::keywords::Keyword::PATH => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::PATH)?
                }
                sqlparser::keywords::Keyword::PATTERN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PATTERN)?
                }
                sqlparser::keywords::Keyword::PER => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::PER)?
                }
                sqlparser::keywords::Keyword::PERCENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PERCENT)?
                }
                sqlparser::keywords::Keyword::PERCENTILE_CONT => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::PERCENTILE_CONT,
                        )?
                }
                sqlparser::keywords::Keyword::PERCENTILE_DISC => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::PERCENTILE_DISC,
                        )?
                }
                sqlparser::keywords::Keyword::PERCENT_RANK => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::PERCENT_RANK,
                        )?
                }
                sqlparser::keywords::Keyword::PERIOD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PERIOD)?
                }
                sqlparser::keywords::Keyword::PERMISSIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PERMISSIVE)?
                }
                sqlparser::keywords::Keyword::PERSISTENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PERSISTENT)?
                }
                sqlparser::keywords::Keyword::PIVOT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PIVOT)?
                }
                sqlparser::keywords::Keyword::PLACING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PLACING)?
                }
                sqlparser::keywords::Keyword::PLAN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::PLAN)?
                }
                sqlparser::keywords::Keyword::PLANS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PLANS)?
                }
                sqlparser::keywords::Keyword::POLICY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::POLICY)?
                }
                sqlparser::keywords::Keyword::PORTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PORTION)?
                }
                sqlparser::keywords::Keyword::POSITION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::POSITION)?
                }
                sqlparser::keywords::Keyword::POSITION_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::POSITION_REGEX,
                        )?
                }
                sqlparser::keywords::Keyword::POWER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::POWER)?
                }
                sqlparser::keywords::Keyword::PRAGMA => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRAGMA)?
                }
                sqlparser::keywords::Keyword::PRECEDES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRECEDES)?
                }
                sqlparser::keywords::Keyword::PRECEDING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRECEDING)?
                }
                sqlparser::keywords::Keyword::PRECISION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRECISION)?
                }
                sqlparser::keywords::Keyword::PREPARE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PREPARE)?
                }
                sqlparser::keywords::Keyword::PRESERVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRESERVE)?
                }
                sqlparser::keywords::Keyword::PREWHERE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PREWHERE)?
                }
                sqlparser::keywords::Keyword::PRIMARY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRIMARY)?
                }
                sqlparser::keywords::Keyword::PRIOR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRIOR)?
                }
                sqlparser::keywords::Keyword::PRIVILEGES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PRIVILEGES)?
                }
                sqlparser::keywords::Keyword::PROCEDURE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PROCEDURE)?
                }
                sqlparser::keywords::Keyword::PROGRAM => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PROGRAM)?
                }
                sqlparser::keywords::Keyword::PROJECTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PROJECTION)?
                }
                sqlparser::keywords::Keyword::PURGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::PURGE)?
                }
                sqlparser::keywords::Keyword::QUALIFY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::QUALIFY)?
                }
                sqlparser::keywords::Keyword::QUARTER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::QUARTER)?
                }
                sqlparser::keywords::Keyword::QUERY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::QUERY)?
                }
                sqlparser::keywords::Keyword::QUOTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::QUOTE)?
                }
                sqlparser::keywords::Keyword::RANGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RANGE)?
                }
                sqlparser::keywords::Keyword::RANK => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::RANK)?
                }
                sqlparser::keywords::Keyword::RAW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::RAW)?
                }
                sqlparser::keywords::Keyword::RCFILE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RCFILE)?
                }
                sqlparser::keywords::Keyword::READ => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::READ)?
                }
                sqlparser::keywords::Keyword::READS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::READS)?
                }
                sqlparser::keywords::Keyword::READ_ONLY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::READ_ONLY)?
                }
                sqlparser::keywords::Keyword::REAL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::REAL)?
                }
                sqlparser::keywords::Keyword::RECURSIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RECURSIVE)?
                }
                sqlparser::keywords::Keyword::REF => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::REF)?
                }
                sqlparser::keywords::Keyword::REFERENCES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REFERENCES)?
                }
                sqlparser::keywords::Keyword::REFERENCING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REFERENCING)?
                }
                sqlparser::keywords::Keyword::REGCLASS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGCLASS)?
                }
                sqlparser::keywords::Keyword::REGEXP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGEXP)?
                }
                sqlparser::keywords::Keyword::REGR_AVGX => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_AVGX)?
                }
                sqlparser::keywords::Keyword::REGR_AVGY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_AVGY)?
                }
                sqlparser::keywords::Keyword::REGR_COUNT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_COUNT)?
                }
                sqlparser::keywords::Keyword::REGR_INTERCEPT => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::REGR_INTERCEPT,
                        )?
                }
                sqlparser::keywords::Keyword::REGR_R2 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_R2)?
                }
                sqlparser::keywords::Keyword::REGR_SLOPE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_SLOPE)?
                }
                sqlparser::keywords::Keyword::REGR_SXX => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_SXX)?
                }
                sqlparser::keywords::Keyword::REGR_SXY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_SXY)?
                }
                sqlparser::keywords::Keyword::REGR_SYY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REGR_SYY)?
                }
                sqlparser::keywords::Keyword::RELATIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RELATIVE)?
                }
                sqlparser::keywords::Keyword::RELAY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RELAY)?
                }
                sqlparser::keywords::Keyword::RELEASE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RELEASE)?
                }
                sqlparser::keywords::Keyword::REMOTE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REMOTE)?
                }
                sqlparser::keywords::Keyword::RENAME => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RENAME)?
                }
                sqlparser::keywords::Keyword::REORG => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REORG)?
                }
                sqlparser::keywords::Keyword::REPAIR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REPAIR)?
                }
                sqlparser::keywords::Keyword::REPEATABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REPEATABLE)?
                }
                sqlparser::keywords::Keyword::REPLACE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REPLACE)?
                }
                sqlparser::keywords::Keyword::REPLICA => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REPLICA)?
                }
                sqlparser::keywords::Keyword::REPLICATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REPLICATION)?
                }
                sqlparser::keywords::Keyword::RESET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESET)?
                }
                sqlparser::keywords::Keyword::RESPECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESPECT)?
                }
                sqlparser::keywords::Keyword::RESTART => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESTART)?
                }
                sqlparser::keywords::Keyword::RESTRICT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESTRICT)?
                }
                sqlparser::keywords::Keyword::RESTRICTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESTRICTED)?
                }
                sqlparser::keywords::Keyword::RESTRICTIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESTRICTIVE)?
                }
                sqlparser::keywords::Keyword::RESULT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESULT)?
                }
                sqlparser::keywords::Keyword::RESULTSET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RESULTSET)?
                }
                sqlparser::keywords::Keyword::RETAIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RETAIN)?
                }
                sqlparser::keywords::Keyword::RETURN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RETURN)?
                }
                sqlparser::keywords::Keyword::RETURNING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RETURNING)?
                }
                sqlparser::keywords::Keyword::RETURNS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RETURNS)?
                }
                sqlparser::keywords::Keyword::REVOKE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::REVOKE)?
                }
                sqlparser::keywords::Keyword::RIGHT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RIGHT)?
                }
                sqlparser::keywords::Keyword::RLIKE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::RLIKE)?
                }
                sqlparser::keywords::Keyword::ROLE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ROLE)?
                }
                sqlparser::keywords::Keyword::ROLLBACK => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ROLLBACK)?
                }
                sqlparser::keywords::Keyword::ROLLUP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ROLLUP)?
                }
                sqlparser::keywords::Keyword::ROOT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ROOT)?
                }
                sqlparser::keywords::Keyword::ROW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ROW)?
                }
                sqlparser::keywords::Keyword::ROWID => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ROWID)?
                }
                sqlparser::keywords::Keyword::ROWS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ROWS)?
                }
                sqlparser::keywords::Keyword::ROW_NUMBER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ROW_NUMBER)?
                }
                sqlparser::keywords::Keyword::RULE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::RULE)?
                }
                sqlparser::keywords::Keyword::RUN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::RUN)?
                }
                sqlparser::keywords::Keyword::SAFE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SAFE)?
                }
                sqlparser::keywords::Keyword::SAFE_CAST => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SAFE_CAST)?
                }
                sqlparser::keywords::Keyword::SAVEPOINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SAVEPOINT)?
                }
                sqlparser::keywords::Keyword::SCHEMA => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SCHEMA)?
                }
                sqlparser::keywords::Keyword::SCHEMAS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SCHEMAS)?
                }
                sqlparser::keywords::Keyword::SCOPE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SCOPE)?
                }
                sqlparser::keywords::Keyword::SCROLL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SCROLL)?
                }
                sqlparser::keywords::Keyword::SEARCH => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SEARCH)?
                }
                sqlparser::keywords::Keyword::SECOND => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SECOND)?
                }
                sqlparser::keywords::Keyword::SECRET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SECRET)?
                }
                sqlparser::keywords::Keyword::SECURITY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SECURITY)?
                }
                sqlparser::keywords::Keyword::SELECT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SELECT)?
                }
                sqlparser::keywords::Keyword::SEMI => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SEMI)?
                }
                sqlparser::keywords::Keyword::SENSITIVE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SENSITIVE)?
                }
                sqlparser::keywords::Keyword::SEPARATOR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SEPARATOR)?
                }
                sqlparser::keywords::Keyword::SEQUENCE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SEQUENCE)?
                }
                sqlparser::keywords::Keyword::SEQUENCEFILE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SEQUENCEFILE,
                        )?
                }
                sqlparser::keywords::Keyword::SEQUENCES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SEQUENCES)?
                }
                sqlparser::keywords::Keyword::SERDE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SERDE)?
                }
                sqlparser::keywords::Keyword::SERDEPROPERTIES => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SERDEPROPERTIES,
                        )?
                }
                sqlparser::keywords::Keyword::SERIALIZABLE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SERIALIZABLE,
                        )?
                }
                sqlparser::keywords::Keyword::SESSION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SESSION)?
                }
                sqlparser::keywords::Keyword::SESSION_USER => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SESSION_USER,
                        )?
                }
                sqlparser::keywords::Keyword::SET => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SET)?
                }
                sqlparser::keywords::Keyword::SETS => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SETS)?
                }
                sqlparser::keywords::Keyword::SETTINGS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SETTINGS)?
                }
                sqlparser::keywords::Keyword::SHARE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SHARE)?
                }
                sqlparser::keywords::Keyword::SHOW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SHOW)?
                }
                sqlparser::keywords::Keyword::SIMILAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SIMILAR)?
                }
                sqlparser::keywords::Keyword::SKIP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SKIP)?
                }
                sqlparser::keywords::Keyword::SLOW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SLOW)?
                }
                sqlparser::keywords::Keyword::SMALLINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SMALLINT)?
                }
                sqlparser::keywords::Keyword::SNAPSHOT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SNAPSHOT)?
                }
                sqlparser::keywords::Keyword::SOME => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SOME)?
                }
                sqlparser::keywords::Keyword::SORT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SORT)?
                }
                sqlparser::keywords::Keyword::SORTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SORTED)?
                }
                sqlparser::keywords::Keyword::SOURCE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SOURCE)?
                }
                sqlparser::keywords::Keyword::SPATIAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SPATIAL)?
                }
                sqlparser::keywords::Keyword::SPECIFIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SPECIFIC)?
                }
                sqlparser::keywords::Keyword::SPECIFICTYPE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SPECIFICTYPE,
                        )?
                }
                sqlparser::keywords::Keyword::SQL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SQL)?
                }
                sqlparser::keywords::Keyword::SQLEXCEPTION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SQLEXCEPTION,
                        )?
                }
                sqlparser::keywords::Keyword::SQLSTATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SQLSTATE)?
                }
                sqlparser::keywords::Keyword::SQLWARNING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SQLWARNING)?
                }
                sqlparser::keywords::Keyword::SQRT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SQRT)?
                }
                sqlparser::keywords::Keyword::STABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STABLE)?
                }
                sqlparser::keywords::Keyword::STAGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STAGE)?
                }
                sqlparser::keywords::Keyword::START => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::START)?
                }
                sqlparser::keywords::Keyword::STATEMENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STATEMENT)?
                }
                sqlparser::keywords::Keyword::STATIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STATIC)?
                }
                sqlparser::keywords::Keyword::STATISTICS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STATISTICS)?
                }
                sqlparser::keywords::Keyword::STATUS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STATUS)?
                }
                sqlparser::keywords::Keyword::STDDEV_POP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STDDEV_POP)?
                }
                sqlparser::keywords::Keyword::STDDEV_SAMP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STDDEV_SAMP)?
                }
                sqlparser::keywords::Keyword::STDIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STDIN)?
                }
                sqlparser::keywords::Keyword::STDOUT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STDOUT)?
                }
                sqlparser::keywords::Keyword::STEP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::STEP)?
                }
                sqlparser::keywords::Keyword::STORAGE_INTEGRATION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::STORAGE_INTEGRATION,
                        )?
                }
                sqlparser::keywords::Keyword::STORED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STORED)?
                }
                sqlparser::keywords::Keyword::STRICT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STRICT)?
                }
                sqlparser::keywords::Keyword::STRING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STRING)?
                }
                sqlparser::keywords::Keyword::STRUCT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::STRUCT)?
                }
                sqlparser::keywords::Keyword::SUBMULTISET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SUBMULTISET)?
                }
                sqlparser::keywords::Keyword::SUBSTRING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SUBSTRING)?
                }
                sqlparser::keywords::Keyword::SUBSTRING_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::SUBSTRING_REGEX,
                        )?
                }
                sqlparser::keywords::Keyword::SUCCEEDS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SUCCEEDS)?
                }
                sqlparser::keywords::Keyword::SUM => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SUM)?
                }
                sqlparser::keywords::Keyword::SUPER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SUPER)?
                }
                sqlparser::keywords::Keyword::SUPERUSER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SUPERUSER)?
                }
                sqlparser::keywords::Keyword::SWAP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SWAP)?
                }
                sqlparser::keywords::Keyword::SYMMETRIC => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SYMMETRIC)?
                }
                sqlparser::keywords::Keyword::SYNC => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::SYNC)?
                }
                sqlparser::keywords::Keyword::SYSTEM => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SYSTEM)?
                }
                sqlparser::keywords::Keyword::SYSTEM_TIME => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SYSTEM_TIME)?
                }
                sqlparser::keywords::Keyword::SYSTEM_USER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::SYSTEM_USER)?
                }
                sqlparser::keywords::Keyword::TABLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TABLE)?
                }
                sqlparser::keywords::Keyword::TABLES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TABLES)?
                }
                sqlparser::keywords::Keyword::TABLESAMPLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TABLESAMPLE)?
                }
                sqlparser::keywords::Keyword::TAG => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TAG)?
                }
                sqlparser::keywords::Keyword::TARGET => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TARGET)?
                }
                sqlparser::keywords::Keyword::TBLPROPERTIES => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::TBLPROPERTIES,
                        )?
                }
                sqlparser::keywords::Keyword::TEMP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TEMP)?
                }
                sqlparser::keywords::Keyword::TEMPORARY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TEMPORARY)?
                }
                sqlparser::keywords::Keyword::TERMINATED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TERMINATED)?
                }
                sqlparser::keywords::Keyword::TEXT => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TEXT)?
                }
                sqlparser::keywords::Keyword::TEXTFILE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TEXTFILE)?
                }
                sqlparser::keywords::Keyword::THEN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::THEN)?
                }
                sqlparser::keywords::Keyword::TIES => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TIES)?
                }
                sqlparser::keywords::Keyword::TIME => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TIME)?
                }
                sqlparser::keywords::Keyword::TIMESTAMP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TIMESTAMP)?
                }
                sqlparser::keywords::Keyword::TIMESTAMPTZ => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TIMESTAMPTZ)?
                }
                sqlparser::keywords::Keyword::TIMETZ => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TIMETZ)?
                }
                sqlparser::keywords::Keyword::TIMEZONE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TIMEZONE)?
                }
                sqlparser::keywords::Keyword::TIMEZONE_ABBR => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::TIMEZONE_ABBR,
                        )?
                }
                sqlparser::keywords::Keyword::TIMEZONE_HOUR => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::TIMEZONE_HOUR,
                        )?
                }
                sqlparser::keywords::Keyword::TIMEZONE_MINUTE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::TIMEZONE_MINUTE,
                        )?
                }
                sqlparser::keywords::Keyword::TIMEZONE_REGION => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::TIMEZONE_REGION,
                        )?
                }
                sqlparser::keywords::Keyword::TINYINT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TINYINT)?
                }
                sqlparser::keywords::Keyword::TO => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TO)?
                }
                sqlparser::keywords::Keyword::TOP => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TOP)?
                }
                sqlparser::keywords::Keyword::TOTALS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TOTALS)?
                }
                sqlparser::keywords::Keyword::TRAILING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRAILING)?
                }
                sqlparser::keywords::Keyword::TRANSACTION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRANSACTION)?
                }
                sqlparser::keywords::Keyword::TRANSIENT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRANSIENT)?
                }
                sqlparser::keywords::Keyword::TRANSLATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRANSLATE)?
                }
                sqlparser::keywords::Keyword::TRANSLATE_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::TRANSLATE_REGEX,
                        )?
                }
                sqlparser::keywords::Keyword::TRANSLATION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRANSLATION)?
                }
                sqlparser::keywords::Keyword::TREAT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TREAT)?
                }
                sqlparser::keywords::Keyword::TRIGGER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRIGGER)?
                }
                sqlparser::keywords::Keyword::TRIM => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TRIM)?
                }
                sqlparser::keywords::Keyword::TRIM_ARRAY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRIM_ARRAY)?
                }
                sqlparser::keywords::Keyword::TRUE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TRUE)?
                }
                sqlparser::keywords::Keyword::TRUNCATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRUNCATE)?
                }
                sqlparser::keywords::Keyword::TRY_CAST => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRY_CAST)?
                }
                sqlparser::keywords::Keyword::TRY_CONVERT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TRY_CONVERT)?
                }
                sqlparser::keywords::Keyword::TUPLE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::TUPLE)?
                }
                sqlparser::keywords::Keyword::TYPE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::TYPE)?
                }
                sqlparser::keywords::Keyword::UESCAPE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UESCAPE)?
                }
                sqlparser::keywords::Keyword::UINT128 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UINT128)?
                }
                sqlparser::keywords::Keyword::UINT16 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UINT16)?
                }
                sqlparser::keywords::Keyword::UINT256 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UINT256)?
                }
                sqlparser::keywords::Keyword::UINT32 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UINT32)?
                }
                sqlparser::keywords::Keyword::UINT64 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UINT64)?
                }
                sqlparser::keywords::Keyword::UINT8 => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UINT8)?
                }
                sqlparser::keywords::Keyword::UNBOUNDED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNBOUNDED)?
                }
                sqlparser::keywords::Keyword::UNCACHE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNCACHE)?
                }
                sqlparser::keywords::Keyword::UNCOMMITTED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNCOMMITTED)?
                }
                sqlparser::keywords::Keyword::UNFREEZE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNFREEZE)?
                }
                sqlparser::keywords::Keyword::UNION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNION)?
                }
                sqlparser::keywords::Keyword::UNIQUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNIQUE)?
                }
                sqlparser::keywords::Keyword::UNKNOWN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNKNOWN)?
                }
                sqlparser::keywords::Keyword::UNLOAD => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNLOAD)?
                }
                sqlparser::keywords::Keyword::UNLOCK => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNLOCK)?
                }
                sqlparser::keywords::Keyword::UNLOGGED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNLOGGED)?
                }
                sqlparser::keywords::Keyword::UNMATCHED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNMATCHED)?
                }
                sqlparser::keywords::Keyword::UNNEST => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNNEST)?
                }
                sqlparser::keywords::Keyword::UNPIVOT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNPIVOT)?
                }
                sqlparser::keywords::Keyword::UNSAFE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNSAFE)?
                }
                sqlparser::keywords::Keyword::UNSIGNED => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNSIGNED)?
                }
                sqlparser::keywords::Keyword::UNTIL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UNTIL)?
                }
                sqlparser::keywords::Keyword::UPDATE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UPDATE)?
                }
                sqlparser::keywords::Keyword::UPPER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::UPPER)?
                }
                sqlparser::keywords::Keyword::URL => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::URL)?
                }
                sqlparser::keywords::Keyword::USAGE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::USAGE)?
                }
                sqlparser::keywords::Keyword::USE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::USE)?
                }
                sqlparser::keywords::Keyword::USER => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::USER)?
                }
                sqlparser::keywords::Keyword::USER_RESOURCES => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::USER_RESOURCES,
                        )?
                }
                sqlparser::keywords::Keyword::USING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::USING)?
                }
                sqlparser::keywords::Keyword::UUID => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::UUID)?
                }
                sqlparser::keywords::Keyword::VACUUM => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VACUUM)?
                }
                sqlparser::keywords::Keyword::VALID => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VALID)?
                }
                sqlparser::keywords::Keyword::VALIDATION_MODE => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::VALIDATION_MODE,
                        )?
                }
                sqlparser::keywords::Keyword::VALUE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VALUE)?
                }
                sqlparser::keywords::Keyword::VALUES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VALUES)?
                }
                sqlparser::keywords::Keyword::VALUE_OF => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VALUE_OF)?
                }
                sqlparser::keywords::Keyword::VARBINARY => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VARBINARY)?
                }
                sqlparser::keywords::Keyword::VARCHAR => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VARCHAR)?
                }
                sqlparser::keywords::Keyword::VARIABLES => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VARIABLES)?
                }
                sqlparser::keywords::Keyword::VARYING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VARYING)?
                }
                sqlparser::keywords::Keyword::VAR_POP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VAR_POP)?
                }
                sqlparser::keywords::Keyword::VAR_SAMP => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VAR_SAMP)?
                }
                sqlparser::keywords::Keyword::VERBOSE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VERBOSE)?
                }
                sqlparser::keywords::Keyword::VERSION => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VERSION)?
                }
                sqlparser::keywords::Keyword::VERSIONING => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VERSIONING)?
                }
                sqlparser::keywords::Keyword::VIEW => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::VIEW)?
                }
                sqlparser::keywords::Keyword::VIEWS => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VIEWS)?
                }
                sqlparser::keywords::Keyword::VIRTUAL => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VIRTUAL)?
                }
                sqlparser::keywords::Keyword::VOLATILE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::VOLATILE)?
                }
                sqlparser::keywords::Keyword::WAREHOUSE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WAREHOUSE)?
                }
                sqlparser::keywords::Keyword::WEEK => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::WEEK)?
                }
                sqlparser::keywords::Keyword::WHEN => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::WHEN)?
                }
                sqlparser::keywords::Keyword::WHENEVER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WHENEVER)?
                }
                sqlparser::keywords::Keyword::WHERE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WHERE)?
                }
                sqlparser::keywords::Keyword::WIDTH_BUCKET => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::WIDTH_BUCKET,
                        )?
                }
                sqlparser::keywords::Keyword::WINDOW => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WINDOW)?
                }
                sqlparser::keywords::Keyword::WITH => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::WITH)?
                }
                sqlparser::keywords::Keyword::WITHIN => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WITHIN)?
                }
                sqlparser::keywords::Keyword::WITHOUT => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WITHOUT)?
                }
                sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER,
                        )?
                }
                sqlparser::keywords::Keyword::WORK => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::WORK)?
                }
                sqlparser::keywords::Keyword::WRITE => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::WRITE)?
                }
                sqlparser::keywords::Keyword::XML => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::XML)?
                }
                sqlparser::keywords::Keyword::XOR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::XOR)?
                }
                sqlparser::keywords::Keyword::YEAR => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::YEAR)?
                }
                sqlparser::keywords::Keyword::ZONE => {
                    transformer.transform(node_path, sqlparser::keywords::Keyword::ZONE)?
                }
                sqlparser::keywords::Keyword::ZORDER => {
                    transformer
                        .transform(node_path, sqlparser::keywords::Keyword::ZORDER)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Token {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::tokenizer::Token::EOF => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::EOF)?
                }
                sqlparser::tokenizer::Token::Word(field0) => {
                    sqlparser::tokenizer::Token::Word(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::Number(field0, field1) => {
                    sqlparser::tokenizer::Token::Number(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::Char(field0) => {
                    sqlparser::tokenizer::Token::Char(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::SingleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::SingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::DoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleSingleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::TripleSingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleDoubleQuotedString(field0) => {
                    sqlparser::tokenizer::Token::TripleDoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::DollarQuotedString(field0) => {
                    sqlparser::tokenizer::Token::DollarQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                    field0,
                ) => {
                    sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                    field0,
                ) => {
                    sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                    field0,
                ) => {
                    sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                    field0,
                ) => {
                    sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::NationalStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::NationalStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::EscapedStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::EscapedStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::UnicodeStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::UnicodeStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::HexStringLiteral(field0) => {
                    sqlparser::tokenizer::Token::HexStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::Comma => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Comma)?
                }
                sqlparser::tokenizer::Token::Whitespace(field0) => {
                    sqlparser::tokenizer::Token::Whitespace(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::DoubleEq => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::DoubleEq)?
                }
                sqlparser::tokenizer::Token::Eq => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Eq)?
                }
                sqlparser::tokenizer::Token::Neq => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Neq)?
                }
                sqlparser::tokenizer::Token::Lt => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Lt)?
                }
                sqlparser::tokenizer::Token::Gt => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Gt)?
                }
                sqlparser::tokenizer::Token::LtEq => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::LtEq)?
                }
                sqlparser::tokenizer::Token::GtEq => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::GtEq)?
                }
                sqlparser::tokenizer::Token::Spaceship => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Spaceship)?
                }
                sqlparser::tokenizer::Token::Plus => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Plus)?
                }
                sqlparser::tokenizer::Token::Minus => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Minus)?
                }
                sqlparser::tokenizer::Token::Mul => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Mul)?
                }
                sqlparser::tokenizer::Token::Div => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Div)?
                }
                sqlparser::tokenizer::Token::DuckIntDiv => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::DuckIntDiv)?
                }
                sqlparser::tokenizer::Token::Mod => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Mod)?
                }
                sqlparser::tokenizer::Token::StringConcat => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::StringConcat)?
                }
                sqlparser::tokenizer::Token::LParen => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::LParen)?
                }
                sqlparser::tokenizer::Token::RParen => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::RParen)?
                }
                sqlparser::tokenizer::Token::Period => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Period)?
                }
                sqlparser::tokenizer::Token::Colon => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Colon)?
                }
                sqlparser::tokenizer::Token::DoubleColon => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::DoubleColon)?
                }
                sqlparser::tokenizer::Token::Assignment => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Assignment)?
                }
                sqlparser::tokenizer::Token::SemiColon => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::SemiColon)?
                }
                sqlparser::tokenizer::Token::Backslash => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Backslash)?
                }
                sqlparser::tokenizer::Token::LBracket => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::LBracket)?
                }
                sqlparser::tokenizer::Token::RBracket => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::RBracket)?
                }
                sqlparser::tokenizer::Token::Ampersand => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Ampersand)?
                }
                sqlparser::tokenizer::Token::Pipe => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Pipe)?
                }
                sqlparser::tokenizer::Token::Caret => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Caret)?
                }
                sqlparser::tokenizer::Token::LBrace => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::LBrace)?
                }
                sqlparser::tokenizer::Token::RBrace => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::RBrace)?
                }
                sqlparser::tokenizer::Token::RArrow => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::RArrow)?
                }
                sqlparser::tokenizer::Token::Sharp => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Sharp)?
                }
                sqlparser::tokenizer::Token::Tilde => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Tilde)?
                }
                sqlparser::tokenizer::Token::TildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::TildeAsterisk,
                        )?
                }
                sqlparser::tokenizer::Token::ExclamationMarkTilde => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::ExclamationMarkTilde,
                        )?
                }
                sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk,
                        )?
                }
                sqlparser::tokenizer::Token::DoubleTilde => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::DoubleTilde)?
                }
                sqlparser::tokenizer::Token::DoubleTildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::DoubleTildeAsterisk,
                        )?
                }
                sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde,
                        )?
                }
                sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk,
                        )?
                }
                sqlparser::tokenizer::Token::ShiftLeft => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::ShiftLeft)?
                }
                sqlparser::tokenizer::Token::ShiftRight => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::ShiftRight)?
                }
                sqlparser::tokenizer::Token::Overlap => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Overlap)?
                }
                sqlparser::tokenizer::Token::ExclamationMark => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::ExclamationMark,
                        )?
                }
                sqlparser::tokenizer::Token::DoubleExclamationMark => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::DoubleExclamationMark,
                        )?
                }
                sqlparser::tokenizer::Token::AtSign => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::AtSign)?
                }
                sqlparser::tokenizer::Token::CaretAt => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::CaretAt)?
                }
                sqlparser::tokenizer::Token::PGSquareRoot => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::PGSquareRoot)?
                }
                sqlparser::tokenizer::Token::PGCubeRoot => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::PGCubeRoot)?
                }
                sqlparser::tokenizer::Token::Placeholder(field0) => {
                    sqlparser::tokenizer::Token::Placeholder(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqlparser::tokenizer::Token::Arrow => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::Arrow)?
                }
                sqlparser::tokenizer::Token::LongArrow => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::LongArrow)?
                }
                sqlparser::tokenizer::Token::HashArrow => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::HashArrow)?
                }
                sqlparser::tokenizer::Token::HashLongArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqlparser::tokenizer::Token::HashLongArrow,
                        )?
                }
                sqlparser::tokenizer::Token::AtArrow => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::AtArrow)?
                }
                sqlparser::tokenizer::Token::ArrowAt => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::ArrowAt)?
                }
                sqlparser::tokenizer::Token::HashMinus => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::HashMinus)?
                }
                sqlparser::tokenizer::Token::AtQuestion => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::AtQuestion)?
                }
                sqlparser::tokenizer::Token::AtAt => {
                    transformer.transform(node_path, sqlparser::tokenizer::Token::AtAt)?
                }
                sqlparser::tokenizer::Token::Question => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::Question)?
                }
                sqlparser::tokenizer::Token::QuestionAnd => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::QuestionAnd)?
                }
                sqlparser::tokenizer::Token::QuestionPipe => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Token::QuestionPipe)?
                }
                sqlparser::tokenizer::Token::CustomBinaryOperator(field0) => {
                    sqlparser::tokenizer::Token::CustomBinaryOperator(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Whitespace {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            match self {
                sqlparser::tokenizer::Whitespace::Space => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Whitespace::Space)?
                }
                sqlparser::tokenizer::Whitespace::Newline => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Whitespace::Newline)?
                }
                sqlparser::tokenizer::Whitespace::Tab => {
                    transformer
                        .transform(node_path, sqlparser::tokenizer::Whitespace::Tab)?
                }
                sqlparser::tokenizer::Whitespace::SingleLineComment {
                    comment,
                    prefix,
                } => {
                    sqlparser::tokenizer::Whitespace::SingleLineComment {
                        comment: comment
                            .apply_transform_with_path(transformer, node_path)?,
                        prefix: prefix.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqlparser::tokenizer::Whitespace::MultiLineComment(field0) => {
                    sqlparser::tokenizer::Whitespace::MultiLineComment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqlparser::tokenizer::Word {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { value, quote_style, keyword } = self;
            Self {
                value: value.apply_transform_with_path(transformer, node_path)?,
                quote_style: quote_style
                    .apply_transform_with_path(transformer, node_path)?,
                keyword: keyword.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for bigdecimal::BigDecimal {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, self.clone())? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for bool {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for char {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i16 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i32 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i64 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for i8 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for String {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, self.clone())? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u16 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u32 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u64 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for u8 {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = { transformer.transform(node_path, *self)? };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
