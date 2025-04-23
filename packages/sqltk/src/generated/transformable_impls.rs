#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Action {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Action::Connect => {
                    transformer.transform(node_path, sqltk_parser::ast::Action::Connect)?
                }
                sqltk_parser::ast::Action::Create => {
                    transformer.transform(node_path, sqltk_parser::ast::Action::Create)?
                }
                sqltk_parser::ast::Action::Delete => {
                    transformer.transform(node_path, sqltk_parser::ast::Action::Delete)?
                }
                sqltk_parser::ast::Action::Execute => {
                    transformer.transform(node_path, sqltk_parser::ast::Action::Execute)?
                }
                sqltk_parser::ast::Action::Insert { columns } => {
                    sqltk_parser::ast::Action::Insert {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Action::References { columns } => {
                    sqltk_parser::ast::Action::References {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Action::Select { columns } => {
                    sqltk_parser::ast::Action::Select {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Action::Temporary => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Action::Temporary)?
                }
                sqltk_parser::ast::Action::Trigger => {
                    transformer.transform(node_path, sqltk_parser::ast::Action::Trigger)?
                }
                sqltk_parser::ast::Action::Truncate => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Action::Truncate)?
                }
                sqltk_parser::ast::Action::Update { columns } => {
                    sqltk_parser::ast::Action::Update {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Action::Usage => {
                    transformer.transform(node_path, sqltk_parser::ast::Action::Usage)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AddDropSync {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AddDropSync::ADD => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::AddDropSync::ADD)?
                }
                sqltk_parser::ast::AddDropSync::DROP => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::AddDropSync::DROP)?
                }
                sqltk_parser::ast::AddDropSync::SYNC => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::AddDropSync::SYNC)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AfterMatchSkip {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AfterMatchSkip::PastLastRow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AfterMatchSkip::PastLastRow,
                        )?
                }
                sqltk_parser::ast::AfterMatchSkip::ToNextRow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AfterMatchSkip::ToNextRow,
                        )?
                }
                sqltk_parser::ast::AfterMatchSkip::ToFirst(field0) => {
                    sqltk_parser::ast::AfterMatchSkip::ToFirst(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::AfterMatchSkip::ToLast(field0) => {
                    sqltk_parser::ast::AfterMatchSkip::ToLast(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AlterColumnOperation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AlterColumnOperation::SetNotNull => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterColumnOperation::SetNotNull,
                        )?
                }
                sqltk_parser::ast::AlterColumnOperation::DropNotNull => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterColumnOperation::DropNotNull,
                        )?
                }
                sqltk_parser::ast::AlterColumnOperation::SetDefault { value } => {
                    sqltk_parser::ast::AlterColumnOperation::SetDefault {
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterColumnOperation::DropDefault => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterColumnOperation::DropDefault,
                        )?
                }
                sqltk_parser::ast::AlterColumnOperation::SetDataType {
                    data_type,
                    using,
                } => {
                    sqltk_parser::ast::AlterColumnOperation::SetDataType {
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterColumnOperation::AddGenerated {
                    generated_as,
                    sequence_options,
                } => {
                    sqltk_parser::ast::AlterColumnOperation::AddGenerated {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AlterIndexOperation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AlterIndexOperation::RenameIndex { index_name } => {
                    sqltk_parser::ast::AlterIndexOperation::RenameIndex {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AlterPolicyOperation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AlterPolicyOperation::Rename { new_name } => {
                    sqltk_parser::ast::AlterPolicyOperation::Rename {
                        new_name: new_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterPolicyOperation::Apply {
                    to,
                    using,
                    with_check,
                } => {
                    sqltk_parser::ast::AlterPolicyOperation::Apply {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AlterRoleOperation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AlterRoleOperation::RenameRole { role_name } => {
                    sqltk_parser::ast::AlterRoleOperation::RenameRole {
                        role_name: role_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterRoleOperation::AddMember { member_name } => {
                    sqltk_parser::ast::AlterRoleOperation::AddMember {
                        member_name: member_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterRoleOperation::DropMember { member_name } => {
                    sqltk_parser::ast::AlterRoleOperation::DropMember {
                        member_name: member_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterRoleOperation::WithOptions { options } => {
                    sqltk_parser::ast::AlterRoleOperation::WithOptions {
                        options: options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterRoleOperation::Set {
                    config_name,
                    config_value,
                    in_database,
                } => {
                    sqltk_parser::ast::AlterRoleOperation::Set {
                        config_name: config_name
                            .apply_transform_with_path(transformer, node_path)?,
                        config_value: config_value
                            .apply_transform_with_path(transformer, node_path)?,
                        in_database: in_database
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterRoleOperation::Reset {
                    config_name,
                    in_database,
                } => {
                    sqltk_parser::ast::AlterRoleOperation::Reset {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AlterTableOperation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AlterTableOperation::AddConstraint(field0) => {
                    sqltk_parser::ast::AlterTableOperation::AddConstraint(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::AlterTableOperation::AddColumn {
                    column_keyword,
                    if_not_exists,
                    column_def,
                    column_position,
                } => {
                    sqltk_parser::ast::AlterTableOperation::AddColumn {
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
                sqltk_parser::ast::AlterTableOperation::AddProjection {
                    if_not_exists,
                    name,
                    select,
                } => {
                    sqltk_parser::ast::AlterTableOperation::AddProjection {
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        select: select.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DropProjection {
                    if_exists,
                    name,
                } => {
                    sqltk_parser::ast::AlterTableOperation::DropProjection {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::MaterializeProjection {
                    if_exists,
                    name,
                    partition,
                } => {
                    sqltk_parser::ast::AlterTableOperation::MaterializeProjection {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::ClearProjection {
                    if_exists,
                    name,
                    partition,
                } => {
                    sqltk_parser::ast::AlterTableOperation::ClearProjection {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DisableRowLevelSecurity => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterTableOperation::DisableRowLevelSecurity,
                        )?
                }
                sqltk_parser::ast::AlterTableOperation::DisableRule { name } => {
                    sqltk_parser::ast::AlterTableOperation::DisableRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DisableTrigger { name } => {
                    sqltk_parser::ast::AlterTableOperation::DisableTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DropConstraint {
                    if_exists,
                    name,
                    cascade,
                } => {
                    sqltk_parser::ast::AlterTableOperation::DropConstraint {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DropColumn {
                    column_name,
                    if_exists,
                    cascade,
                } => {
                    sqltk_parser::ast::AlterTableOperation::DropColumn {
                        column_name: column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        cascade: cascade
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::AttachPartition { partition } => {
                    sqltk_parser::ast::AlterTableOperation::AttachPartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DetachPartition { partition } => {
                    sqltk_parser::ast::AlterTableOperation::DetachPartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::FreezePartition {
                    partition,
                    with_name,
                } => {
                    sqltk_parser::ast::AlterTableOperation::FreezePartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                        with_name: with_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::UnfreezePartition {
                    partition,
                    with_name,
                } => {
                    sqltk_parser::ast::AlterTableOperation::UnfreezePartition {
                        partition: partition
                            .apply_transform_with_path(transformer, node_path)?,
                        with_name: with_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DropPrimaryKey => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterTableOperation::DropPrimaryKey,
                        )?
                }
                sqltk_parser::ast::AlterTableOperation::EnableAlwaysRule { name } => {
                    sqltk_parser::ast::AlterTableOperation::EnableAlwaysRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::EnableAlwaysTrigger { name } => {
                    sqltk_parser::ast::AlterTableOperation::EnableAlwaysTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::EnableReplicaRule { name } => {
                    sqltk_parser::ast::AlterTableOperation::EnableReplicaRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::EnableReplicaTrigger { name } => {
                    sqltk_parser::ast::AlterTableOperation::EnableReplicaTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::EnableRowLevelSecurity => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterTableOperation::EnableRowLevelSecurity,
                        )?
                }
                sqltk_parser::ast::AlterTableOperation::EnableRule { name } => {
                    sqltk_parser::ast::AlterTableOperation::EnableRule {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::EnableTrigger { name } => {
                    sqltk_parser::ast::AlterTableOperation::EnableTrigger {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::RenamePartitions {
                    old_partitions,
                    new_partitions,
                } => {
                    sqltk_parser::ast::AlterTableOperation::RenamePartitions {
                        old_partitions: old_partitions
                            .apply_transform_with_path(transformer, node_path)?,
                        new_partitions: new_partitions
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::AddPartitions {
                    if_not_exists,
                    new_partitions,
                } => {
                    sqltk_parser::ast::AlterTableOperation::AddPartitions {
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        new_partitions: new_partitions
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DropPartitions {
                    partitions,
                    if_exists,
                } => {
                    sqltk_parser::ast::AlterTableOperation::DropPartitions {
                        partitions: partitions
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::RenameColumn {
                    old_column_name,
                    new_column_name,
                } => {
                    sqltk_parser::ast::AlterTableOperation::RenameColumn {
                        old_column_name: old_column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        new_column_name: new_column_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::RenameTable { table_name } => {
                    sqltk_parser::ast::AlterTableOperation::RenameTable {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::ChangeColumn {
                    old_name,
                    new_name,
                    data_type,
                    options,
                    column_position,
                } => {
                    sqltk_parser::ast::AlterTableOperation::ChangeColumn {
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
                sqltk_parser::ast::AlterTableOperation::ModifyColumn {
                    col_name,
                    data_type,
                    options,
                    column_position,
                } => {
                    sqltk_parser::ast::AlterTableOperation::ModifyColumn {
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
                sqltk_parser::ast::AlterTableOperation::RenameConstraint {
                    old_name,
                    new_name,
                } => {
                    sqltk_parser::ast::AlterTableOperation::RenameConstraint {
                        old_name: old_name
                            .apply_transform_with_path(transformer, node_path)?,
                        new_name: new_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::AlterColumn {
                    column_name,
                    op,
                } => {
                    sqltk_parser::ast::AlterTableOperation::AlterColumn {
                        column_name: column_name
                            .apply_transform_with_path(transformer, node_path)?,
                        op: op.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::SwapWith { table_name } => {
                    sqltk_parser::ast::AlterTableOperation::SwapWith {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::SetTblProperties {
                    table_properties,
                } => {
                    sqltk_parser::ast::AlterTableOperation::SetTblProperties {
                        table_properties: table_properties
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::OwnerTo { new_owner } => {
                    sqltk_parser::ast::AlterTableOperation::OwnerTo {
                        new_owner: new_owner
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::ClusterBy { exprs } => {
                    sqltk_parser::ast::AlterTableOperation::ClusterBy {
                        exprs: exprs.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::AlterTableOperation::DropClusteringKey => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterTableOperation::DropClusteringKey,
                        )?
                }
                sqltk_parser::ast::AlterTableOperation::SuspendRecluster => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterTableOperation::SuspendRecluster,
                        )?
                }
                sqltk_parser::ast::AlterTableOperation::ResumeRecluster => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AlterTableOperation::ResumeRecluster,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AnalyzeFormat {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AnalyzeFormat::TEXT => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::AnalyzeFormat::TEXT)?
                }
                sqltk_parser::ast::AnalyzeFormat::GRAPHVIZ => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::AnalyzeFormat::GRAPHVIZ,
                        )?
                }
                sqltk_parser::ast::AnalyzeFormat::JSON => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::AnalyzeFormat::JSON)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ArgMode {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ArgMode::In => {
                    transformer.transform(node_path, sqltk_parser::ast::ArgMode::In)?
                }
                sqltk_parser::ast::ArgMode::Out => {
                    transformer.transform(node_path, sqltk_parser::ast::ArgMode::Out)?
                }
                sqltk_parser::ast::ArgMode::InOut => {
                    transformer.transform(node_path, sqltk_parser::ast::ArgMode::InOut)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Array {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ArrayElemTypeDef {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ArrayElemTypeDef::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ArrayElemTypeDef::None)?
                }
                sqltk_parser::ast::ArrayElemTypeDef::AngleBracket(field0) => {
                    sqltk_parser::ast::ArrayElemTypeDef::AngleBracket(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ArrayElemTypeDef::SquareBracket(field0, field1) => {
                    sqltk_parser::ast::ArrayElemTypeDef::SquareBracket(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ArrayElemTypeDef::Parenthesis(field0) => {
                    sqltk_parser::ast::ArrayElemTypeDef::Parenthesis(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Assignment {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AssignmentTarget {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AssignmentTarget::ColumnName(field0) => {
                    sqltk_parser::ast::AssignmentTarget::ColumnName(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::AssignmentTarget::Tuple(field0) => {
                    sqltk_parser::ast::AssignmentTarget::Tuple(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::AttachDuckDBDatabaseOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::AttachDuckDBDatabaseOption::ReadOnly(field0) => {
                    sqltk_parser::ast::AttachDuckDBDatabaseOption::ReadOnly(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::AttachDuckDBDatabaseOption::Type(field0) => {
                    sqltk_parser::ast::AttachDuckDBDatabaseOption::Type(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::BeginTransactionKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::BeginTransactionKind::Transaction => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BeginTransactionKind::Transaction,
                        )?
                }
                sqltk_parser::ast::BeginTransactionKind::Work => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BeginTransactionKind::Work,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::BinaryOperator {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::BinaryOperator::Plus => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Plus)?
                }
                sqltk_parser::ast::BinaryOperator::Minus => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Minus)?
                }
                sqltk_parser::ast::BinaryOperator::Multiply => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::Multiply,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::Divide => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Divide)?
                }
                sqltk_parser::ast::BinaryOperator::Modulo => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Modulo)?
                }
                sqltk_parser::ast::BinaryOperator::StringConcat => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::StringConcat,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::Gt => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Gt)?
                }
                sqltk_parser::ast::BinaryOperator::Lt => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Lt)?
                }
                sqltk_parser::ast::BinaryOperator::GtEq => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::GtEq)?
                }
                sqltk_parser::ast::BinaryOperator::LtEq => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::LtEq)?
                }
                sqltk_parser::ast::BinaryOperator::Spaceship => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::Spaceship,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::Eq => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Eq)?
                }
                sqltk_parser::ast::BinaryOperator::NotEq => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::NotEq)?
                }
                sqltk_parser::ast::BinaryOperator::And => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::And)?
                }
                sqltk_parser::ast::BinaryOperator::Or => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Or)?
                }
                sqltk_parser::ast::BinaryOperator::Xor => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Xor)?
                }
                sqltk_parser::ast::BinaryOperator::BitwiseOr => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::BitwiseOr,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::BitwiseAnd => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::BitwiseAnd,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::BitwiseXor => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::BitwiseXor,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::DuckIntegerDivide => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::DuckIntegerDivide,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::MyIntegerDivide => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::MyIntegerDivide,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::Custom(field0) => {
                    sqltk_parser::ast::BinaryOperator::Custom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::BinaryOperator::PGBitwiseXor => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGBitwiseXor,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGBitwiseShiftLeft => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGBitwiseShiftLeft,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGBitwiseShiftRight => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGBitwiseShiftRight,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGExp => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::PGExp)?
                }
                sqltk_parser::ast::BinaryOperator::PGOverlap => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGOverlap,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGRegexMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGRegexMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGRegexIMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGRegexIMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGRegexNotMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGRegexNotMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGRegexNotIMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGRegexNotIMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGLikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGLikeMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGILikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGILikeMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGNotLikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGNotLikeMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGNotILikeMatch => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGNotILikeMatch,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGStartsWith => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::PGStartsWith,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::Arrow => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::Arrow)?
                }
                sqltk_parser::ast::BinaryOperator::LongArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::LongArrow,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::HashArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::HashArrow,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::HashLongArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::HashLongArrow,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::AtAt => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::BinaryOperator::AtAt)?
                }
                sqltk_parser::ast::BinaryOperator::AtArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::AtArrow,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::ArrowAt => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::ArrowAt,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::HashMinus => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::HashMinus,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::AtQuestion => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::AtQuestion,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::Question => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::Question,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::QuestionAnd => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::QuestionAnd,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::QuestionPipe => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::BinaryOperator::QuestionPipe,
                        )?
                }
                sqltk_parser::ast::BinaryOperator::PGCustomBinaryOperator(field0) => {
                    sqltk_parser::ast::BinaryOperator::PGCustomBinaryOperator(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CastFormat {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CastFormat::Value(field0) => {
                    sqltk_parser::ast::CastFormat::Value(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CastFormat::ValueAtTimeZone(field0, field1) => {
                    sqltk_parser::ast::CastFormat::ValueAtTimeZone(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CastKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CastKind::Cast => {
                    transformer.transform(node_path, sqltk_parser::ast::CastKind::Cast)?
                }
                sqltk_parser::ast::CastKind::TryCast => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CastKind::TryCast)?
                }
                sqltk_parser::ast::CastKind::SafeCast => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CastKind::SafeCast)?
                }
                sqltk_parser::ast::CastKind::DoubleColon => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CastKind::DoubleColon)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CeilFloorKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CeilFloorKind::DateTimeField(field0) => {
                    sqltk_parser::ast::CeilFloorKind::DateTimeField(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CeilFloorKind::Scale(field0) => {
                    sqltk_parser::ast::CeilFloorKind::Scale(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CharLengthUnits {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CharLengthUnits::Characters => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CharLengthUnits::Characters,
                        )?
                }
                sqltk_parser::ast::CharLengthUnits::Octets => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CharLengthUnits::Octets,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CharacterLength {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CharacterLength::IntegerLength { length, unit } => {
                    sqltk_parser::ast::CharacterLength::IntegerLength {
                        length: length
                            .apply_transform_with_path(transformer, node_path)?,
                        unit: unit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::CharacterLength::Max => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CharacterLength::Max)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CloseCursor {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CloseCursor::All => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CloseCursor::All)?
                }
                sqltk_parser::ast::CloseCursor::Specific { name } => {
                    sqltk_parser::ast::CloseCursor::Specific {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ClusteredBy {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ClusteredIndex {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ColumnDef {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ColumnOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ColumnOption::Null => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ColumnOption::Null)?
                }
                sqltk_parser::ast::ColumnOption::NotNull => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ColumnOption::NotNull)?
                }
                sqltk_parser::ast::ColumnOption::Default(field0) => {
                    sqltk_parser::ast::ColumnOption::Default(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Materialized(field0) => {
                    sqltk_parser::ast::ColumnOption::Materialized(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Ephemeral(field0) => {
                    sqltk_parser::ast::ColumnOption::Ephemeral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Alias(field0) => {
                    sqltk_parser::ast::ColumnOption::Alias(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Unique {
                    is_primary,
                    characteristics,
                } => {
                    sqltk_parser::ast::ColumnOption::Unique {
                        is_primary: is_primary
                            .apply_transform_with_path(transformer, node_path)?,
                        characteristics: characteristics
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::ColumnOption::ForeignKey {
                    foreign_table,
                    referred_columns,
                    on_delete,
                    on_update,
                    characteristics,
                } => {
                    sqltk_parser::ast::ColumnOption::ForeignKey {
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
                sqltk_parser::ast::ColumnOption::Check(field0) => {
                    sqltk_parser::ast::ColumnOption::Check(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::DialectSpecific(field0) => {
                    sqltk_parser::ast::ColumnOption::DialectSpecific(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::CharacterSet(field0) => {
                    sqltk_parser::ast::ColumnOption::CharacterSet(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Comment(field0) => {
                    sqltk_parser::ast::ColumnOption::Comment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::OnUpdate(field0) => {
                    sqltk_parser::ast::ColumnOption::OnUpdate(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Generated {
                    generated_as,
                    sequence_options,
                    generation_expr,
                    generation_expr_mode,
                    generated_keyword,
                } => {
                    sqltk_parser::ast::ColumnOption::Generated {
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
                sqltk_parser::ast::ColumnOption::Options(field0) => {
                    sqltk_parser::ast::ColumnOption::Options(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Identity(field0) => {
                    sqltk_parser::ast::ColumnOption::Identity(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::OnConflict(field0) => {
                    sqltk_parser::ast::ColumnOption::OnConflict(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Policy(field0) => {
                    sqltk_parser::ast::ColumnOption::Policy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnOption::Tags(field0) => {
                    sqltk_parser::ast::ColumnOption::Tags(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ColumnOptionDef {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ColumnPolicy {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ColumnPolicy::MaskingPolicy(field0) => {
                    sqltk_parser::ast::ColumnPolicy::MaskingPolicy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ColumnPolicy::ProjectionPolicy(field0) => {
                    sqltk_parser::ast::ColumnPolicy::ProjectionPolicy(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ColumnPolicyProperty {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CommentDef {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CommentDef::WithEq(field0) => {
                    sqltk_parser::ast::CommentDef::WithEq(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CommentDef::WithoutEq(field0) => {
                    sqltk_parser::ast::CommentDef::WithoutEq(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CommentDef::AfterColumnDefsWithoutEq(field0) => {
                    sqltk_parser::ast::CommentDef::AfterColumnDefsWithoutEq(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CommentObject {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CommentObject::Column => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CommentObject::Column)?
                }
                sqltk_parser::ast::CommentObject::Table => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CommentObject::Table)?
                }
                sqltk_parser::ast::CommentObject::Extension => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CommentObject::Extension,
                        )?
                }
                sqltk_parser::ast::CommentObject::Schema => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CommentObject::Schema)?
                }
                sqltk_parser::ast::CommentObject::Database => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CommentObject::Database,
                        )?
                }
                sqltk_parser::ast::CommentObject::User => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CommentObject::User)?
                }
                sqltk_parser::ast::CommentObject::Role => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CommentObject::Role)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ConflictTarget {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ConflictTarget::Columns(field0) => {
                    sqltk_parser::ast::ConflictTarget::Columns(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ConflictTarget::OnConstraint(field0) => {
                    sqltk_parser::ast::ConflictTarget::OnConstraint(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ConnectBy {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ConstraintCharacteristics {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ContextModifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ContextModifier::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ContextModifier::None)?
                }
                sqltk_parser::ast::ContextModifier::Local => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ContextModifier::Local)?
                }
                sqltk_parser::ast::ContextModifier::Session => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ContextModifier::Session,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CopyLegacyCsvOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CopyLegacyCsvOption::Header => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CopyLegacyCsvOption::Header,
                        )?
                }
                sqltk_parser::ast::CopyLegacyCsvOption::Quote(field0) => {
                    sqltk_parser::ast::CopyLegacyCsvOption::Quote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyLegacyCsvOption::Escape(field0) => {
                    sqltk_parser::ast::CopyLegacyCsvOption::Escape(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyLegacyCsvOption::ForceQuote(field0) => {
                    sqltk_parser::ast::CopyLegacyCsvOption::ForceQuote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyLegacyCsvOption::ForceNotNull(field0) => {
                    sqltk_parser::ast::CopyLegacyCsvOption::ForceNotNull(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CopyLegacyOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CopyLegacyOption::Binary => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CopyLegacyOption::Binary,
                        )?
                }
                sqltk_parser::ast::CopyLegacyOption::Delimiter(field0) => {
                    sqltk_parser::ast::CopyLegacyOption::Delimiter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyLegacyOption::Null(field0) => {
                    sqltk_parser::ast::CopyLegacyOption::Null(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyLegacyOption::Csv(field0) => {
                    sqltk_parser::ast::CopyLegacyOption::Csv(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CopyOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CopyOption::Format(field0) => {
                    sqltk_parser::ast::CopyOption::Format(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Freeze(field0) => {
                    sqltk_parser::ast::CopyOption::Freeze(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Delimiter(field0) => {
                    sqltk_parser::ast::CopyOption::Delimiter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Null(field0) => {
                    sqltk_parser::ast::CopyOption::Null(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Header(field0) => {
                    sqltk_parser::ast::CopyOption::Header(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Quote(field0) => {
                    sqltk_parser::ast::CopyOption::Quote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Escape(field0) => {
                    sqltk_parser::ast::CopyOption::Escape(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::ForceQuote(field0) => {
                    sqltk_parser::ast::CopyOption::ForceQuote(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::ForceNotNull(field0) => {
                    sqltk_parser::ast::CopyOption::ForceNotNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::ForceNull(field0) => {
                    sqltk_parser::ast::CopyOption::ForceNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CopyOption::Encoding(field0) => {
                    sqltk_parser::ast::CopyOption::Encoding(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CopySource {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CopySource::Table { table_name, columns } => {
                    sqltk_parser::ast::CopySource::Table {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::CopySource::Query(field0) => {
                    sqltk_parser::ast::CopySource::Query(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CopyTarget {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CopyTarget::Stdin => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CopyTarget::Stdin)?
                }
                sqltk_parser::ast::CopyTarget::Stdout => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::CopyTarget::Stdout)?
                }
                sqltk_parser::ast::CopyTarget::File { filename } => {
                    sqltk_parser::ast::CopyTarget::File {
                        filename: filename
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::CopyTarget::Program { command } => {
                    sqltk_parser::ast::CopyTarget::Program {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreateFunction {
    fn apply_transform_with_path<T>(
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
            } = self;
            Self {
                or_replace: or_replace
                    .apply_transform_with_path(transformer, node_path)?,
                temporary: temporary.apply_transform_with_path(transformer, node_path)?,
                if_not_exists: if_not_exists
                    .apply_transform_with_path(transformer, node_path)?,
                name: name.apply_transform_with_path(transformer, node_path)?,
                args: args.apply_transform_with_path(transformer, node_path)?,
                return_type: return_type
                    .apply_transform_with_path(transformer, node_path)?,
                function_body: function_body
                    .apply_transform_with_path(transformer, node_path)?,
                behavior: behavior.apply_transform_with_path(transformer, node_path)?,
                called_on_null: called_on_null
                    .apply_transform_with_path(transformer, node_path)?,
                parallel: parallel.apply_transform_with_path(transformer, node_path)?,
                using: using.apply_transform_with_path(transformer, node_path)?,
                language: language.apply_transform_with_path(transformer, node_path)?,
                determinism_specifier: determinism_specifier
                    .apply_transform_with_path(transformer, node_path)?,
                options: options.apply_transform_with_path(transformer, node_path)?,
                remote_connection: remote_connection
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreateFunctionBody {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CreateFunctionBody::AsBeforeOptions(field0) => {
                    sqltk_parser::ast::CreateFunctionBody::AsBeforeOptions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CreateFunctionBody::AsAfterOptions(field0) => {
                    sqltk_parser::ast::CreateFunctionBody::AsAfterOptions(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CreateFunctionBody::Return(field0) => {
                    sqltk_parser::ast::CreateFunctionBody::Return(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreateFunctionUsing {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CreateFunctionUsing::Jar(field0) => {
                    sqltk_parser::ast::CreateFunctionUsing::Jar(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CreateFunctionUsing::File(field0) => {
                    sqltk_parser::ast::CreateFunctionUsing::File(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CreateFunctionUsing::Archive(field0) => {
                    sqltk_parser::ast::CreateFunctionUsing::Archive(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreateIndex {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreatePolicyCommand {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CreatePolicyCommand::All => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyCommand::All,
                        )?
                }
                sqltk_parser::ast::CreatePolicyCommand::Select => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyCommand::Select,
                        )?
                }
                sqltk_parser::ast::CreatePolicyCommand::Insert => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyCommand::Insert,
                        )?
                }
                sqltk_parser::ast::CreatePolicyCommand::Update => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyCommand::Update,
                        )?
                }
                sqltk_parser::ast::CreatePolicyCommand::Delete => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyCommand::Delete,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreatePolicyType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CreatePolicyType::Permissive => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyType::Permissive,
                        )?
                }
                sqltk_parser::ast::CreatePolicyType::Restrictive => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreatePolicyType::Restrictive,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreateTable {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CreateTableOptions {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CreateTableOptions::None => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CreateTableOptions::None,
                        )?
                }
                sqltk_parser::ast::CreateTableOptions::With(field0) => {
                    sqltk_parser::ast::CreateTableOptions::With(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::CreateTableOptions::Options(field0) => {
                    sqltk_parser::ast::CreateTableOptions::Options(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Cte {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { alias, query, from, materialized, closing_paren_token } = self;
            Self {
                alias: alias.apply_transform_with_path(transformer, node_path)?,
                query: query.apply_transform_with_path(transformer, node_path)?,
                from: from.apply_transform_with_path(transformer, node_path)?,
                materialized: materialized
                    .apply_transform_with_path(transformer, node_path)?,
                closing_paren_token: closing_paren_token
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::CteAsMaterialized {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::CteAsMaterialized::Materialized => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CteAsMaterialized::Materialized,
                        )?
                }
                sqltk_parser::ast::CteAsMaterialized::NotMaterialized => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::CteAsMaterialized::NotMaterialized,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DataType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DataType::Character(field0) => {
                    sqltk_parser::ast::DataType::Character(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Char(field0) => {
                    sqltk_parser::ast::DataType::Char(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::CharacterVarying(field0) => {
                    sqltk_parser::ast::DataType::CharacterVarying(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::CharVarying(field0) => {
                    sqltk_parser::ast::DataType::CharVarying(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Varchar(field0) => {
                    sqltk_parser::ast::DataType::Varchar(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Nvarchar(field0) => {
                    sqltk_parser::ast::DataType::Nvarchar(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Uuid => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Uuid)?
                }
                sqltk_parser::ast::DataType::CharacterLargeObject(field0) => {
                    sqltk_parser::ast::DataType::CharacterLargeObject(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::CharLargeObject(field0) => {
                    sqltk_parser::ast::DataType::CharLargeObject(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Clob(field0) => {
                    sqltk_parser::ast::DataType::Clob(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Binary(field0) => {
                    sqltk_parser::ast::DataType::Binary(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Varbinary(field0) => {
                    sqltk_parser::ast::DataType::Varbinary(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Blob(field0) => {
                    sqltk_parser::ast::DataType::Blob(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::TinyBlob => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::TinyBlob)?
                }
                sqltk_parser::ast::DataType::MediumBlob => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::MediumBlob)?
                }
                sqltk_parser::ast::DataType::LongBlob => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::LongBlob)?
                }
                sqltk_parser::ast::DataType::Bytes(field0) => {
                    sqltk_parser::ast::DataType::Bytes(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Numeric(field0) => {
                    sqltk_parser::ast::DataType::Numeric(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Decimal(field0) => {
                    sqltk_parser::ast::DataType::Decimal(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::BigNumeric(field0) => {
                    sqltk_parser::ast::DataType::BigNumeric(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::BigDecimal(field0) => {
                    sqltk_parser::ast::DataType::BigDecimal(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Dec(field0) => {
                    sqltk_parser::ast::DataType::Dec(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Float(field0) => {
                    sqltk_parser::ast::DataType::Float(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::TinyInt(field0) => {
                    sqltk_parser::ast::DataType::TinyInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedTinyInt(field0) => {
                    sqltk_parser::ast::DataType::UnsignedTinyInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Int2(field0) => {
                    sqltk_parser::ast::DataType::Int2(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedInt2(field0) => {
                    sqltk_parser::ast::DataType::UnsignedInt2(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::SmallInt(field0) => {
                    sqltk_parser::ast::DataType::SmallInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedSmallInt(field0) => {
                    sqltk_parser::ast::DataType::UnsignedSmallInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::MediumInt(field0) => {
                    sqltk_parser::ast::DataType::MediumInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedMediumInt(field0) => {
                    sqltk_parser::ast::DataType::UnsignedMediumInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Int(field0) => {
                    sqltk_parser::ast::DataType::Int(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Int4(field0) => {
                    sqltk_parser::ast::DataType::Int4(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Int8(field0) => {
                    sqltk_parser::ast::DataType::Int8(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Int16 => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Int16)?
                }
                sqltk_parser::ast::DataType::Int32 => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Int32)?
                }
                sqltk_parser::ast::DataType::Int64 => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Int64)?
                }
                sqltk_parser::ast::DataType::Int128 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Int128)?
                }
                sqltk_parser::ast::DataType::Int256 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Int256)?
                }
                sqltk_parser::ast::DataType::Integer(field0) => {
                    sqltk_parser::ast::DataType::Integer(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedInt(field0) => {
                    sqltk_parser::ast::DataType::UnsignedInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedInt4(field0) => {
                    sqltk_parser::ast::DataType::UnsignedInt4(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedInteger(field0) => {
                    sqltk_parser::ast::DataType::UnsignedInteger(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UInt8 => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::UInt8)?
                }
                sqltk_parser::ast::DataType::UInt16 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::UInt16)?
                }
                sqltk_parser::ast::DataType::UInt32 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::UInt32)?
                }
                sqltk_parser::ast::DataType::UInt64 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::UInt64)?
                }
                sqltk_parser::ast::DataType::UInt128 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::UInt128)?
                }
                sqltk_parser::ast::DataType::UInt256 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::UInt256)?
                }
                sqltk_parser::ast::DataType::BigInt(field0) => {
                    sqltk_parser::ast::DataType::BigInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedBigInt(field0) => {
                    sqltk_parser::ast::DataType::UnsignedBigInt(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::UnsignedInt8(field0) => {
                    sqltk_parser::ast::DataType::UnsignedInt8(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Float4 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Float4)?
                }
                sqltk_parser::ast::DataType::Float32 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Float32)?
                }
                sqltk_parser::ast::DataType::Float64 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Float64)?
                }
                sqltk_parser::ast::DataType::Real => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Real)?
                }
                sqltk_parser::ast::DataType::Float8 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Float8)?
                }
                sqltk_parser::ast::DataType::Double => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Double)?
                }
                sqltk_parser::ast::DataType::DoublePrecision => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DataType::DoublePrecision,
                        )?
                }
                sqltk_parser::ast::DataType::Bool => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Bool)?
                }
                sqltk_parser::ast::DataType::Boolean => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Boolean)?
                }
                sqltk_parser::ast::DataType::Date => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Date)?
                }
                sqltk_parser::ast::DataType::Date32 => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Date32)?
                }
                sqltk_parser::ast::DataType::Time(field0, field1) => {
                    sqltk_parser::ast::DataType::Time(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Datetime(field0) => {
                    sqltk_parser::ast::DataType::Datetime(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Datetime64(field0, field1) => {
                    sqltk_parser::ast::DataType::Datetime64(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Timestamp(field0, field1) => {
                    sqltk_parser::ast::DataType::Timestamp(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Interval => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Interval)?
                }
                sqltk_parser::ast::DataType::JSON => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::JSON)?
                }
                sqltk_parser::ast::DataType::JSONB => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::JSONB)?
                }
                sqltk_parser::ast::DataType::Regclass => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Regclass)?
                }
                sqltk_parser::ast::DataType::Text => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Text)?
                }
                sqltk_parser::ast::DataType::TinyText => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::TinyText)?
                }
                sqltk_parser::ast::DataType::MediumText => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::MediumText)?
                }
                sqltk_parser::ast::DataType::LongText => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::LongText)?
                }
                sqltk_parser::ast::DataType::String(field0) => {
                    sqltk_parser::ast::DataType::String(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::FixedString(field0) => {
                    sqltk_parser::ast::DataType::FixedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Bytea => {
                    transformer.transform(node_path, sqltk_parser::ast::DataType::Bytea)?
                }
                sqltk_parser::ast::DataType::Bit(field0) => {
                    sqltk_parser::ast::DataType::Bit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::BitVarying(field0) => {
                    sqltk_parser::ast::DataType::BitVarying(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Custom(field0, field1) => {
                    sqltk_parser::ast::DataType::Custom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Array(field0) => {
                    sqltk_parser::ast::DataType::Array(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Map(field0, field1) => {
                    sqltk_parser::ast::DataType::Map(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Tuple(field0) => {
                    sqltk_parser::ast::DataType::Tuple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Nested(field0) => {
                    sqltk_parser::ast::DataType::Nested(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Enum(field0, field1) => {
                    sqltk_parser::ast::DataType::Enum(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Set(field0) => {
                    sqltk_parser::ast::DataType::Set(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Struct(field0, field1) => {
                    sqltk_parser::ast::DataType::Struct(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Union(field0) => {
                    sqltk_parser::ast::DataType::Union(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Nullable(field0) => {
                    sqltk_parser::ast::DataType::Nullable(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::LowCardinality(field0) => {
                    sqltk_parser::ast::DataType::LowCardinality(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DataType::Unspecified => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Unspecified)?
                }
                sqltk_parser::ast::DataType::Trigger => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DataType::Trigger)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DateTimeField {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DateTimeField::Year => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Year)?
                }
                sqltk_parser::ast::DateTimeField::Month => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Month)?
                }
                sqltk_parser::ast::DateTimeField::Week(field0) => {
                    sqltk_parser::ast::DateTimeField::Week(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DateTimeField::Day => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Day)?
                }
                sqltk_parser::ast::DateTimeField::DayOfWeek => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::DayOfWeek,
                        )?
                }
                sqltk_parser::ast::DateTimeField::DayOfYear => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::DayOfYear,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Date => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Date)?
                }
                sqltk_parser::ast::DateTimeField::Datetime => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Datetime,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Hour => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Hour)?
                }
                sqltk_parser::ast::DateTimeField::Minute => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Minute)?
                }
                sqltk_parser::ast::DateTimeField::Second => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Second)?
                }
                sqltk_parser::ast::DateTimeField::Century => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Century)?
                }
                sqltk_parser::ast::DateTimeField::Decade => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Decade)?
                }
                sqltk_parser::ast::DateTimeField::Dow => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Dow)?
                }
                sqltk_parser::ast::DateTimeField::Doy => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Doy)?
                }
                sqltk_parser::ast::DateTimeField::Epoch => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Epoch)?
                }
                sqltk_parser::ast::DateTimeField::Isodow => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Isodow)?
                }
                sqltk_parser::ast::DateTimeField::IsoWeek => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::IsoWeek)?
                }
                sqltk_parser::ast::DateTimeField::Isoyear => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Isoyear)?
                }
                sqltk_parser::ast::DateTimeField::Julian => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Julian)?
                }
                sqltk_parser::ast::DateTimeField::Microsecond => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Microsecond,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Microseconds => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Microseconds,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Millenium => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Millenium,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Millennium => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Millennium,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Millisecond => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Millisecond,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Milliseconds => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Milliseconds,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Nanosecond => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Nanosecond,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Nanoseconds => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Nanoseconds,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Quarter => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Quarter)?
                }
                sqltk_parser::ast::DateTimeField::Time => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DateTimeField::Time)?
                }
                sqltk_parser::ast::DateTimeField::Timezone => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::Timezone,
                        )?
                }
                sqltk_parser::ast::DateTimeField::TimezoneAbbr => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::TimezoneAbbr,
                        )?
                }
                sqltk_parser::ast::DateTimeField::TimezoneHour => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::TimezoneHour,
                        )?
                }
                sqltk_parser::ast::DateTimeField::TimezoneMinute => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::TimezoneMinute,
                        )?
                }
                sqltk_parser::ast::DateTimeField::TimezoneRegion => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::TimezoneRegion,
                        )?
                }
                sqltk_parser::ast::DateTimeField::NoDateTime => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DateTimeField::NoDateTime,
                        )?
                }
                sqltk_parser::ast::DateTimeField::Custom(field0) => {
                    sqltk_parser::ast::DateTimeField::Custom(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Declare {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DeclareAssignment {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DeclareAssignment::Expr(field0) => {
                    sqltk_parser::ast::DeclareAssignment::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DeclareAssignment::Default(field0) => {
                    sqltk_parser::ast::DeclareAssignment::Default(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DeclareAssignment::DuckAssignment(field0) => {
                    sqltk_parser::ast::DeclareAssignment::DuckAssignment(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DeclareAssignment::For(field0) => {
                    sqltk_parser::ast::DeclareAssignment::For(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::DeclareAssignment::MsSqlAssignment(field0) => {
                    sqltk_parser::ast::DeclareAssignment::MsSqlAssignment(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DeclareType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DeclareType::Cursor => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DeclareType::Cursor)?
                }
                sqltk_parser::ast::DeclareType::ResultSet => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DeclareType::ResultSet)?
                }
                sqltk_parser::ast::DeclareType::Exception => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DeclareType::Exception)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Deduplicate {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Deduplicate::All => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Deduplicate::All)?
                }
                sqltk_parser::ast::Deduplicate::ByExpression(field0) => {
                    sqltk_parser::ast::Deduplicate::ByExpression(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DeferrableInitial {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DeferrableInitial::Immediate => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DeferrableInitial::Immediate,
                        )?
                }
                sqltk_parser::ast::DeferrableInitial::Deferred => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DeferrableInitial::Deferred,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Delete {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DescribeAlias {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DescribeAlias::Describe => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DescribeAlias::Describe,
                        )?
                }
                sqltk_parser::ast::DescribeAlias::Explain => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DescribeAlias::Explain)?
                }
                sqltk_parser::ast::DescribeAlias::Desc => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DescribeAlias::Desc)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DictionaryField {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DiscardObject {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DiscardObject::ALL => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DiscardObject::ALL)?
                }
                sqltk_parser::ast::DiscardObject::PLANS => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DiscardObject::PLANS)?
                }
                sqltk_parser::ast::DiscardObject::SEQUENCES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DiscardObject::SEQUENCES,
                        )?
                }
                sqltk_parser::ast::DiscardObject::TEMP => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::DiscardObject::TEMP)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Distinct {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Distinct::Distinct => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Distinct::Distinct)?
                }
                sqltk_parser::ast::Distinct::On(field0) => {
                    sqltk_parser::ast::Distinct::On(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DoUpdate {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DollarQuotedString {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::DuplicateTreatment {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::DuplicateTreatment::Distinct => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DuplicateTreatment::Distinct,
                        )?
                }
                sqltk_parser::ast::DuplicateTreatment::All => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::DuplicateTreatment::All,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::EmptyMatchesMode {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::EmptyMatchesMode::Show => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::EmptyMatchesMode::Show)?
                }
                sqltk_parser::ast::EmptyMatchesMode::Omit => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::EmptyMatchesMode::Omit)?
                }
                sqltk_parser::ast::EmptyMatchesMode::WithUnmatched => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::EmptyMatchesMode::WithUnmatched,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::EnumMember {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::EnumMember::Name(field0) => {
                    sqltk_parser::ast::EnumMember::Name(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::EnumMember::NamedValue(field0, field1) => {
                    sqltk_parser::ast::EnumMember::NamedValue(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ExactNumberInfo {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ExactNumberInfo::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ExactNumberInfo::None)?
                }
                sqltk_parser::ast::ExactNumberInfo::Precision(field0) => {
                    sqltk_parser::ast::ExactNumberInfo::Precision(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ExactNumberInfo::PrecisionAndScale(field0, field1) => {
                    sqltk_parser::ast::ExactNumberInfo::PrecisionAndScale(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ExceptSelectItem {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ExcludeSelectItem {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ExcludeSelectItem::Single(field0) => {
                    sqltk_parser::ast::ExcludeSelectItem::Single(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ExcludeSelectItem::Multiple(field0) => {
                    sqltk_parser::ast::ExcludeSelectItem::Multiple(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Expr {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Expr::Identifier(field0) => {
                    sqltk_parser::ast::Expr::Identifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::CompoundIdentifier(field0) => {
                    sqltk_parser::ast::Expr::CompoundIdentifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::JsonAccess { value, path } => {
                    sqltk_parser::ast::Expr::JsonAccess {
                        value: value.apply_transform_with_path(transformer, node_path)?,
                        path: path.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::CompositeAccess { expr, key } => {
                    sqltk_parser::ast::Expr::CompositeAccess {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        key: key.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::IsFalse(field0) => {
                    sqltk_parser::ast::Expr::IsFalse(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsNotFalse(field0) => {
                    sqltk_parser::ast::Expr::IsNotFalse(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsTrue(field0) => {
                    sqltk_parser::ast::Expr::IsTrue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsNotTrue(field0) => {
                    sqltk_parser::ast::Expr::IsNotTrue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsNull(field0) => {
                    sqltk_parser::ast::Expr::IsNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsNotNull(field0) => {
                    sqltk_parser::ast::Expr::IsNotNull(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsUnknown(field0) => {
                    sqltk_parser::ast::Expr::IsUnknown(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsNotUnknown(field0) => {
                    sqltk_parser::ast::Expr::IsNotUnknown(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsDistinctFrom(field0, field1) => {
                    sqltk_parser::ast::Expr::IsDistinctFrom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IsNotDistinctFrom(field0, field1) => {
                    sqltk_parser::ast::Expr::IsNotDistinctFrom(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::InList { expr, list, negated } => {
                    sqltk_parser::ast::Expr::InList {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        list: list.apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::InSubquery { expr, subquery, negated } => {
                    sqltk_parser::ast::Expr::InSubquery {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        subquery: subquery
                            .apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::InUnnest { expr, array_expr, negated } => {
                    sqltk_parser::ast::Expr::InUnnest {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        array_expr: array_expr
                            .apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Between { expr, negated, low, high } => {
                    sqltk_parser::ast::Expr::Between {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        low: low.apply_transform_with_path(transformer, node_path)?,
                        high: high.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::BinaryOp { left, op, right } => {
                    sqltk_parser::ast::Expr::BinaryOp {
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        op: op.apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Like {
                    negated,
                    any,
                    expr,
                    pattern,
                    escape_char,
                } => {
                    sqltk_parser::ast::Expr::Like {
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
                sqltk_parser::ast::Expr::ILike {
                    negated,
                    any,
                    expr,
                    pattern,
                    escape_char,
                } => {
                    sqltk_parser::ast::Expr::ILike {
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
                sqltk_parser::ast::Expr::SimilarTo {
                    negated,
                    expr,
                    pattern,
                    escape_char,
                } => {
                    sqltk_parser::ast::Expr::SimilarTo {
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        escape_char: escape_char
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::RLike { negated, expr, pattern, regexp } => {
                    sqltk_parser::ast::Expr::RLike {
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        pattern: pattern
                            .apply_transform_with_path(transformer, node_path)?,
                        regexp: regexp.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::AnyOp { left, compare_op, right, is_some } => {
                    sqltk_parser::ast::Expr::AnyOp {
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        compare_op: compare_op
                            .apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                        is_some: is_some
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::AllOp { left, compare_op, right } => {
                    sqltk_parser::ast::Expr::AllOp {
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        compare_op: compare_op
                            .apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::UnaryOp { op, expr } => {
                    sqltk_parser::ast::Expr::UnaryOp {
                        op: op.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Convert {
                    is_try,
                    expr,
                    data_type,
                    charset,
                    target_before_value,
                    styles,
                } => {
                    sqltk_parser::ast::Expr::Convert {
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
                sqltk_parser::ast::Expr::Cast { kind, expr, data_type, format } => {
                    sqltk_parser::ast::Expr::Cast {
                        kind: kind.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        format: format.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::AtTimeZone { timestamp, time_zone } => {
                    sqltk_parser::ast::Expr::AtTimeZone {
                        timestamp: timestamp
                            .apply_transform_with_path(transformer, node_path)?,
                        time_zone: time_zone
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Extract { field, syntax, expr } => {
                    sqltk_parser::ast::Expr::Extract {
                        field: field.apply_transform_with_path(transformer, node_path)?,
                        syntax: syntax
                            .apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Ceil { expr, field } => {
                    sqltk_parser::ast::Expr::Ceil {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        field: field.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Floor { expr, field } => {
                    sqltk_parser::ast::Expr::Floor {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        field: field.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Position { expr, r#in } => {
                    sqltk_parser::ast::Expr::Position {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        r#in: r#in.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Substring {
                    expr,
                    substring_from,
                    substring_for,
                    special,
                } => {
                    sqltk_parser::ast::Expr::Substring {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        substring_from: substring_from
                            .apply_transform_with_path(transformer, node_path)?,
                        substring_for: substring_for
                            .apply_transform_with_path(transformer, node_path)?,
                        special: special
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Trim {
                    expr,
                    trim_where,
                    trim_what,
                    trim_characters,
                } => {
                    sqltk_parser::ast::Expr::Trim {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        trim_where: trim_where
                            .apply_transform_with_path(transformer, node_path)?,
                        trim_what: trim_what
                            .apply_transform_with_path(transformer, node_path)?,
                        trim_characters: trim_characters
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Overlay {
                    expr,
                    overlay_what,
                    overlay_from,
                    overlay_for,
                } => {
                    sqltk_parser::ast::Expr::Overlay {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        overlay_what: overlay_what
                            .apply_transform_with_path(transformer, node_path)?,
                        overlay_from: overlay_from
                            .apply_transform_with_path(transformer, node_path)?,
                        overlay_for: overlay_for
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Collate { expr, collation } => {
                    sqltk_parser::ast::Expr::Collate {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        collation: collation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Nested(field0) => {
                    sqltk_parser::ast::Expr::Nested(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Value(field0) => {
                    sqltk_parser::ast::Expr::Value(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::IntroducedString { introducer, value } => {
                    sqltk_parser::ast::Expr::IntroducedString {
                        introducer: introducer
                            .apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::TypedString { data_type, value } => {
                    sqltk_parser::ast::Expr::TypedString {
                        data_type: data_type
                            .apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::MapAccess { column, keys } => {
                    sqltk_parser::ast::Expr::MapAccess {
                        column: column
                            .apply_transform_with_path(transformer, node_path)?,
                        keys: keys.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Function(field0) => {
                    sqltk_parser::ast::Expr::Function(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Method(field0) => {
                    sqltk_parser::ast::Expr::Method(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Case {
                    operand,
                    conditions,
                    results,
                    else_result,
                } => {
                    sqltk_parser::ast::Expr::Case {
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
                sqltk_parser::ast::Expr::Exists { subquery, negated } => {
                    sqltk_parser::ast::Expr::Exists {
                        subquery: subquery
                            .apply_transform_with_path(transformer, node_path)?,
                        negated: negated
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Subquery(field0) => {
                    sqltk_parser::ast::Expr::Subquery(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::GroupingSets(field0) => {
                    sqltk_parser::ast::Expr::GroupingSets(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Cube(field0) => {
                    sqltk_parser::ast::Expr::Cube(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Rollup(field0) => {
                    sqltk_parser::ast::Expr::Rollup(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Tuple(field0) => {
                    sqltk_parser::ast::Expr::Tuple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Struct { values, fields } => {
                    sqltk_parser::ast::Expr::Struct {
                        values: values
                            .apply_transform_with_path(transformer, node_path)?,
                        fields: fields.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Named { expr, name } => {
                    sqltk_parser::ast::Expr::Named {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Dictionary(field0) => {
                    sqltk_parser::ast::Expr::Dictionary(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Map(field0) => {
                    sqltk_parser::ast::Expr::Map(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Subscript { expr, subscript } => {
                    sqltk_parser::ast::Expr::Subscript {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        subscript: subscript
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Array(field0) => {
                    sqltk_parser::ast::Expr::Array(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Interval(field0) => {
                    sqltk_parser::ast::Expr::Interval(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::MatchAgainst {
                    columns,
                    match_value,
                    opt_search_modifier,
                } => {
                    sqltk_parser::ast::Expr::MatchAgainst {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        match_value: match_value
                            .apply_transform_with_path(transformer, node_path)?,
                        opt_search_modifier: opt_search_modifier
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Expr::Wildcard(field0) => {
                    sqltk_parser::ast::Expr::Wildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::QualifiedWildcard(field0, field1) => {
                    sqltk_parser::ast::Expr::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::OuterJoin(field0) => {
                    sqltk_parser::ast::Expr::OuterJoin(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Prior(field0) => {
                    sqltk_parser::ast::Expr::Prior(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Expr::Lambda(field0) => {
                    sqltk_parser::ast::Expr::Lambda(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ExprWithAlias {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ExtractSyntax {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ExtractSyntax::From => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ExtractSyntax::From)?
                }
                sqltk_parser::ast::ExtractSyntax::Comma => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ExtractSyntax::Comma)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Fetch {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FetchDirection {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FetchDirection::Count { limit } => {
                    sqltk_parser::ast::FetchDirection::Count {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FetchDirection::Next => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FetchDirection::Next)?
                }
                sqltk_parser::ast::FetchDirection::Prior => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FetchDirection::Prior)?
                }
                sqltk_parser::ast::FetchDirection::First => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FetchDirection::First)?
                }
                sqltk_parser::ast::FetchDirection::Last => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FetchDirection::Last)?
                }
                sqltk_parser::ast::FetchDirection::Absolute { limit } => {
                    sqltk_parser::ast::FetchDirection::Absolute {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FetchDirection::Relative { limit } => {
                    sqltk_parser::ast::FetchDirection::Relative {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FetchDirection::All => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FetchDirection::All)?
                }
                sqltk_parser::ast::FetchDirection::Forward { limit } => {
                    sqltk_parser::ast::FetchDirection::Forward {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FetchDirection::ForwardAll => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FetchDirection::ForwardAll,
                        )?
                }
                sqltk_parser::ast::FetchDirection::Backward { limit } => {
                    sqltk_parser::ast::FetchDirection::Backward {
                        limit: limit.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FetchDirection::BackwardAll => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FetchDirection::BackwardAll,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FileFormat {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FileFormat::TEXTFILE => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FileFormat::TEXTFILE)?
                }
                sqltk_parser::ast::FileFormat::SEQUENCEFILE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FileFormat::SEQUENCEFILE,
                        )?
                }
                sqltk_parser::ast::FileFormat::ORC => {
                    transformer.transform(node_path, sqltk_parser::ast::FileFormat::ORC)?
                }
                sqltk_parser::ast::FileFormat::PARQUET => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FileFormat::PARQUET)?
                }
                sqltk_parser::ast::FileFormat::AVRO => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FileFormat::AVRO)?
                }
                sqltk_parser::ast::FileFormat::RCFILE => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FileFormat::RCFILE)?
                }
                sqltk_parser::ast::FileFormat::JSONFILE => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FileFormat::JSONFILE)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FlushLocation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FlushLocation::NoWriteToBinlog => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FlushLocation::NoWriteToBinlog,
                        )?
                }
                sqltk_parser::ast::FlushLocation::Local => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushLocation::Local)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FlushType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FlushType::BinaryLogs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::BinaryLogs)?
                }
                sqltk_parser::ast::FlushType::EngineLogs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::EngineLogs)?
                }
                sqltk_parser::ast::FlushType::ErrorLogs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::ErrorLogs)?
                }
                sqltk_parser::ast::FlushType::GeneralLogs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::GeneralLogs)?
                }
                sqltk_parser::ast::FlushType::Hosts => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::Hosts)?
                }
                sqltk_parser::ast::FlushType::Logs => {
                    transformer.transform(node_path, sqltk_parser::ast::FlushType::Logs)?
                }
                sqltk_parser::ast::FlushType::Privileges => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::Privileges)?
                }
                sqltk_parser::ast::FlushType::OptimizerCosts => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FlushType::OptimizerCosts,
                        )?
                }
                sqltk_parser::ast::FlushType::RelayLogs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::RelayLogs)?
                }
                sqltk_parser::ast::FlushType::SlowLogs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::SlowLogs)?
                }
                sqltk_parser::ast::FlushType::Status => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::Status)?
                }
                sqltk_parser::ast::FlushType::UserResources => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FlushType::UserResources,
                        )?
                }
                sqltk_parser::ast::FlushType::Tables => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FlushType::Tables)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ForClause {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ForClause::Browse => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ForClause::Browse)?
                }
                sqltk_parser::ast::ForClause::Json {
                    for_json,
                    root,
                    include_null_values,
                    without_array_wrapper,
                } => {
                    sqltk_parser::ast::ForClause::Json {
                        for_json: for_json
                            .apply_transform_with_path(transformer, node_path)?,
                        root: root.apply_transform_with_path(transformer, node_path)?,
                        include_null_values: include_null_values
                            .apply_transform_with_path(transformer, node_path)?,
                        without_array_wrapper: without_array_wrapper
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::ForClause::Xml {
                    for_xml,
                    elements,
                    binary_base64,
                    root,
                    r#type,
                } => {
                    sqltk_parser::ast::ForClause::Xml {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ForJson {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ForJson::Auto => {
                    transformer.transform(node_path, sqltk_parser::ast::ForJson::Auto)?
                }
                sqltk_parser::ast::ForJson::Path => {
                    transformer.transform(node_path, sqltk_parser::ast::ForJson::Path)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ForXml {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ForXml::Raw(field0) => {
                    sqltk_parser::ast::ForXml::Raw(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ForXml::Auto => {
                    transformer.transform(node_path, sqltk_parser::ast::ForXml::Auto)?
                }
                sqltk_parser::ast::ForXml::Explicit => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ForXml::Explicit)?
                }
                sqltk_parser::ast::ForXml::Path(field0) => {
                    sqltk_parser::ast::ForXml::Path(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FormatClause {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FormatClause::Identifier(field0) => {
                    sqltk_parser::ast::FormatClause::Identifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FormatClause::Null => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FormatClause::Null)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FromTable {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FromTable::WithFromKeyword(field0) => {
                    sqltk_parser::ast::FromTable::WithFromKeyword(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FromTable::WithoutKeyword(field0) => {
                    sqltk_parser::ast::FromTable::WithoutKeyword(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Function {
    fn apply_transform_with_path<T>(
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
                uses_odbc_syntax,
                parameters,
                args,
                filter,
                null_treatment,
                over,
                within_group,
            } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                uses_odbc_syntax: uses_odbc_syntax
                    .apply_transform_with_path(transformer, node_path)?,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionArg {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionArg::Named { name, arg, operator } => {
                    sqltk_parser::ast::FunctionArg::Named {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        arg: arg.apply_transform_with_path(transformer, node_path)?,
                        operator: operator
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FunctionArg::ExprNamed { name, arg, operator } => {
                    sqltk_parser::ast::FunctionArg::ExprNamed {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        arg: arg.apply_transform_with_path(transformer, node_path)?,
                        operator: operator
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::FunctionArg::Unnamed(field0) => {
                    sqltk_parser::ast::FunctionArg::Unnamed(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionArgExpr {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionArgExpr::Expr(field0) => {
                    sqltk_parser::ast::FunctionArgExpr::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgExpr::QualifiedWildcard(field0) => {
                    sqltk_parser::ast::FunctionArgExpr::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgExpr::Wildcard => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArgExpr::Wildcard,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionArgOperator {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionArgOperator::Equals => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArgOperator::Equals,
                        )?
                }
                sqltk_parser::ast::FunctionArgOperator::RightArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArgOperator::RightArrow,
                        )?
                }
                sqltk_parser::ast::FunctionArgOperator::Assignment => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArgOperator::Assignment,
                        )?
                }
                sqltk_parser::ast::FunctionArgOperator::Colon => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArgOperator::Colon,
                        )?
                }
                sqltk_parser::ast::FunctionArgOperator::Value => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArgOperator::Value,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionArgumentClause {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                    field0,
                ) => {
                    sqltk_parser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgumentClause::OrderBy(field0) => {
                    sqltk_parser::ast::FunctionArgumentClause::OrderBy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgumentClause::Limit(field0) => {
                    sqltk_parser::ast::FunctionArgumentClause::Limit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgumentClause::OnOverflow(field0) => {
                    sqltk_parser::ast::FunctionArgumentClause::OnOverflow(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgumentClause::Having(field0) => {
                    sqltk_parser::ast::FunctionArgumentClause::Having(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgumentClause::Separator(field0) => {
                    sqltk_parser::ast::FunctionArgumentClause::Separator(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArgumentClause::JsonNullClause(field0) => {
                    sqltk_parser::ast::FunctionArgumentClause::JsonNullClause(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionArgumentList {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionArguments {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionArguments::None => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionArguments::None,
                        )?
                }
                sqltk_parser::ast::FunctionArguments::Subquery(field0) => {
                    sqltk_parser::ast::FunctionArguments::Subquery(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::FunctionArguments::List(field0) => {
                    sqltk_parser::ast::FunctionArguments::List(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionBehavior {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionBehavior::Immutable => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionBehavior::Immutable,
                        )?
                }
                sqltk_parser::ast::FunctionBehavior::Stable => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionBehavior::Stable,
                        )?
                }
                sqltk_parser::ast::FunctionBehavior::Volatile => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionBehavior::Volatile,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionCalledOnNull {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionCalledOnNull::CalledOnNullInput => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionCalledOnNull::CalledOnNullInput,
                        )?
                }
                sqltk_parser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput,
                        )?
                }
                sqltk_parser::ast::FunctionCalledOnNull::Strict => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionCalledOnNull::Strict,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionDesc {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast>
for sqltk_parser::ast::FunctionDeterminismSpecifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionDeterminismSpecifier::Deterministic => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionDeterminismSpecifier::Deterministic,
                        )?
                }
                sqltk_parser::ast::FunctionDeterminismSpecifier::NotDeterministic => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionDeterminismSpecifier::NotDeterministic,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::FunctionParallel {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::FunctionParallel::Unsafe => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionParallel::Unsafe,
                        )?
                }
                sqltk_parser::ast::FunctionParallel::Restricted => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::FunctionParallel::Restricted,
                        )?
                }
                sqltk_parser::ast::FunctionParallel::Safe => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::FunctionParallel::Safe)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::GeneratedAs {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::GeneratedAs::Always => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::GeneratedAs::Always)?
                }
                sqltk_parser::ast::GeneratedAs::ByDefault => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::GeneratedAs::ByDefault)?
                }
                sqltk_parser::ast::GeneratedAs::ExpStored => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::GeneratedAs::ExpStored)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::GeneratedExpressionMode {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::GeneratedExpressionMode::Virtual => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::GeneratedExpressionMode::Virtual,
                        )?
                }
                sqltk_parser::ast::GeneratedExpressionMode::Stored => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::GeneratedExpressionMode::Stored,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::GrantObjects {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::GrantObjects::AllSequencesInSchema { schemas } => {
                    sqltk_parser::ast::GrantObjects::AllSequencesInSchema {
                        schemas: schemas
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::GrantObjects::AllTablesInSchema { schemas } => {
                    sqltk_parser::ast::GrantObjects::AllTablesInSchema {
                        schemas: schemas
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::GrantObjects::Schemas(field0) => {
                    sqltk_parser::ast::GrantObjects::Schemas(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::GrantObjects::Sequences(field0) => {
                    sqltk_parser::ast::GrantObjects::Sequences(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::GrantObjects::Tables(field0) => {
                    sqltk_parser::ast::GrantObjects::Tables(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::GroupByExpr {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::GroupByExpr::All(field0) => {
                    sqltk_parser::ast::GroupByExpr::All(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::GroupByExpr::Expressions(field0, field1) => {
                    sqltk_parser::ast::GroupByExpr::Expressions(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::GroupByWithModifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::GroupByWithModifier::Rollup => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::GroupByWithModifier::Rollup,
                        )?
                }
                sqltk_parser::ast::GroupByWithModifier::Cube => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::GroupByWithModifier::Cube,
                        )?
                }
                sqltk_parser::ast::GroupByWithModifier::Totals => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::GroupByWithModifier::Totals,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HavingBound {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HavingBoundKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::HavingBoundKind::Min => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::HavingBoundKind::Min)?
                }
                sqltk_parser::ast::HavingBoundKind::Max => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::HavingBoundKind::Max)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveDelimiter {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::HiveDelimiter::FieldsTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDelimiter::FieldsTerminatedBy,
                        )?
                }
                sqltk_parser::ast::HiveDelimiter::FieldsEscapedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDelimiter::FieldsEscapedBy,
                        )?
                }
                sqltk_parser::ast::HiveDelimiter::CollectionItemsTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDelimiter::CollectionItemsTerminatedBy,
                        )?
                }
                sqltk_parser::ast::HiveDelimiter::MapKeysTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDelimiter::MapKeysTerminatedBy,
                        )?
                }
                sqltk_parser::ast::HiveDelimiter::LinesTerminatedBy => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDelimiter::LinesTerminatedBy,
                        )?
                }
                sqltk_parser::ast::HiveDelimiter::NullDefinedAs => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDelimiter::NullDefinedAs,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveDescribeFormat {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::HiveDescribeFormat::Extended => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDescribeFormat::Extended,
                        )?
                }
                sqltk_parser::ast::HiveDescribeFormat::Formatted => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDescribeFormat::Formatted,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveDistributionStyle {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::HiveDistributionStyle::PARTITIONED { columns } => {
                    sqltk_parser::ast::HiveDistributionStyle::PARTITIONED {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::HiveDistributionStyle::SKEWED {
                    columns,
                    on,
                    stored_as_directories,
                } => {
                    sqltk_parser::ast::HiveDistributionStyle::SKEWED {
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        on: on.apply_transform_with_path(transformer, node_path)?,
                        stored_as_directories: stored_as_directories
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::HiveDistributionStyle::NONE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::HiveDistributionStyle::NONE,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveFormat {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveIOFormat {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::HiveIOFormat::IOF { input_format, output_format } => {
                    sqltk_parser::ast::HiveIOFormat::IOF {
                        input_format: input_format
                            .apply_transform_with_path(transformer, node_path)?,
                        output_format: output_format
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::HiveIOFormat::FileFormat { format } => {
                    sqltk_parser::ast::HiveIOFormat::FileFormat {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveLoadDataFormat {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { serde, input_format } = self;
            Self {
                serde: serde.apply_transform_with_path(transformer, node_path)?,
                input_format: input_format
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveRowDelimiter {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveRowFormat {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::HiveRowFormat::SERDE { class } => {
                    sqltk_parser::ast::HiveRowFormat::SERDE {
                        class: class.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::HiveRowFormat::DELIMITED { delimiters } => {
                    sqltk_parser::ast::HiveRowFormat::DELIMITED {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::HiveSetLocation {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Ident {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { value, quote_style, span } = self;
            Self {
                value: value.apply_transform_with_path(transformer, node_path)?,
                quote_style: quote_style
                    .apply_transform_with_path(transformer, node_path)?,
                span: span.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IdentWithAlias {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IdentityParameters {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IdentityProperty {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IdentityPropertyFormatKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::IdentityPropertyFormatKind::FunctionCall(field0) => {
                    sqltk_parser::ast::IdentityPropertyFormatKind::FunctionCall(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::IdentityPropertyFormatKind::StartAndIncrement(
                    field0,
                ) => {
                    sqltk_parser::ast::IdentityPropertyFormatKind::StartAndIncrement(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IdentityPropertyKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::IdentityPropertyKind::Autoincrement(field0) => {
                    sqltk_parser::ast::IdentityPropertyKind::Autoincrement(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::IdentityPropertyKind::Identity(field0) => {
                    sqltk_parser::ast::IdentityPropertyKind::Identity(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IdentityPropertyOrder {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::IdentityPropertyOrder::Order => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::IdentityPropertyOrder::Order,
                        )?
                }
                sqltk_parser::ast::IdentityPropertyOrder::NoOrder => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::IdentityPropertyOrder::NoOrder,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IlikeSelectItem {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IndexOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::IndexOption::Using(field0) => {
                    sqltk_parser::ast::IndexOption::Using(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::IndexOption::Comment(field0) => {
                    sqltk_parser::ast::IndexOption::Comment(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::IndexType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::IndexType::BTree => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::IndexType::BTree)?
                }
                sqltk_parser::ast::IndexType::Hash => {
                    transformer.transform(node_path, sqltk_parser::ast::IndexType::Hash)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Insert {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::InsertAliases {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Interpolate {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::InterpolateExpr {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Interval {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Join {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JoinConstraint {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::JoinConstraint::On(field0) => {
                    sqltk_parser::ast::JoinConstraint::On(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinConstraint::Using(field0) => {
                    sqltk_parser::ast::JoinConstraint::Using(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinConstraint::Natural => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JoinConstraint::Natural,
                        )?
                }
                sqltk_parser::ast::JoinConstraint::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::JoinConstraint::None)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JoinOperator {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::JoinOperator::Inner(field0) => {
                    sqltk_parser::ast::JoinOperator::Inner(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::LeftOuter(field0) => {
                    sqltk_parser::ast::JoinOperator::LeftOuter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::RightOuter(field0) => {
                    sqltk_parser::ast::JoinOperator::RightOuter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::FullOuter(field0) => {
                    sqltk_parser::ast::JoinOperator::FullOuter(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::CrossJoin => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JoinOperator::CrossJoin,
                        )?
                }
                sqltk_parser::ast::JoinOperator::Semi(field0) => {
                    sqltk_parser::ast::JoinOperator::Semi(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::LeftSemi(field0) => {
                    sqltk_parser::ast::JoinOperator::LeftSemi(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::RightSemi(field0) => {
                    sqltk_parser::ast::JoinOperator::RightSemi(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::Anti(field0) => {
                    sqltk_parser::ast::JoinOperator::Anti(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::LeftAnti(field0) => {
                    sqltk_parser::ast::JoinOperator::LeftAnti(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::RightAnti(field0) => {
                    sqltk_parser::ast::JoinOperator::RightAnti(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JoinOperator::CrossApply => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JoinOperator::CrossApply,
                        )?
                }
                sqltk_parser::ast::JoinOperator::OuterApply => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JoinOperator::OuterApply,
                        )?
                }
                sqltk_parser::ast::JoinOperator::AsOf { match_condition, constraint } => {
                    sqltk_parser::ast::JoinOperator::AsOf {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JsonNullClause {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::JsonNullClause::NullOnNull => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JsonNullClause::NullOnNull,
                        )?
                }
                sqltk_parser::ast::JsonNullClause::AbsentOnNull => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JsonNullClause::AbsentOnNull,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JsonPath {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JsonPathElem {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::JsonPathElem::Dot { key, quoted } => {
                    sqltk_parser::ast::JsonPathElem::Dot {
                        key: key.apply_transform_with_path(transformer, node_path)?,
                        quoted: quoted.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::JsonPathElem::Bracket { key } => {
                    sqltk_parser::ast::JsonPathElem::Bracket {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JsonTableColumn {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::JsonTableColumn::Named(field0) => {
                    sqltk_parser::ast::JsonTableColumn::Named(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JsonTableColumn::ForOrdinality(field0) => {
                    sqltk_parser::ast::JsonTableColumn::ForOrdinality(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JsonTableColumn::Nested(field0) => {
                    sqltk_parser::ast::JsonTableColumn::Nested(
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
impl<'ast> crate::Transformable<'ast>
for sqltk_parser::ast::JsonTableColumnErrorHandling {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::JsonTableColumnErrorHandling::Null => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JsonTableColumnErrorHandling::Null,
                        )?
                }
                sqltk_parser::ast::JsonTableColumnErrorHandling::Default(field0) => {
                    sqltk_parser::ast::JsonTableColumnErrorHandling::Default(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::JsonTableColumnErrorHandling::Error => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::JsonTableColumnErrorHandling::Error,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JsonTableNamedColumn {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::JsonTableNestedColumn {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::KeyOrIndexDisplay {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::KeyOrIndexDisplay::None => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::KeyOrIndexDisplay::None,
                        )?
                }
                sqltk_parser::ast::KeyOrIndexDisplay::Key => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::KeyOrIndexDisplay::Key)?
                }
                sqltk_parser::ast::KeyOrIndexDisplay::Index => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::KeyOrIndexDisplay::Index,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::KillType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::KillType::Connection => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::KillType::Connection)?
                }
                sqltk_parser::ast::KillType::Query => {
                    transformer.transform(node_path, sqltk_parser::ast::KillType::Query)?
                }
                sqltk_parser::ast::KillType::Mutation => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::KillType::Mutation)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::LambdaFunction {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::LateralView {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ListAggOnOverflow {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ListAggOnOverflow::Error => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ListAggOnOverflow::Error,
                        )?
                }
                sqltk_parser::ast::ListAggOnOverflow::Truncate { filler, with_count } => {
                    sqltk_parser::ast::ListAggOnOverflow::Truncate {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::LockClause {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::LockTable {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::LockTableType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::LockTableType::Read { local } => {
                    sqltk_parser::ast::LockTableType::Read {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::LockTableType::Write { low_priority } => {
                    sqltk_parser::ast::LockTableType::Write {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::LockType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::LockType::Share => {
                    transformer.transform(node_path, sqltk_parser::ast::LockType::Share)?
                }
                sqltk_parser::ast::LockType::Update => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::LockType::Update)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MacroArg {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MacroDefinition {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MacroDefinition::Expr(field0) => {
                    sqltk_parser::ast::MacroDefinition::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MacroDefinition::Table(field0) => {
                    sqltk_parser::ast::MacroDefinition::Table(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Map {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MapAccessKey {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MapAccessSyntax {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MapAccessSyntax::Bracket => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MapAccessSyntax::Bracket,
                        )?
                }
                sqltk_parser::ast::MapAccessSyntax::Period => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MapAccessSyntax::Period,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MapEntry {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MatchRecognizePattern {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MatchRecognizePattern::Symbol(field0) => {
                    sqltk_parser::ast::MatchRecognizePattern::Symbol(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizePattern::Exclude(field0) => {
                    sqltk_parser::ast::MatchRecognizePattern::Exclude(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizePattern::Permute(field0) => {
                    sqltk_parser::ast::MatchRecognizePattern::Permute(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizePattern::Concat(field0) => {
                    sqltk_parser::ast::MatchRecognizePattern::Concat(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizePattern::Group(field0) => {
                    sqltk_parser::ast::MatchRecognizePattern::Group(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizePattern::Alternation(field0) => {
                    sqltk_parser::ast::MatchRecognizePattern::Alternation(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizePattern::Repetition(field0, field1) => {
                    sqltk_parser::ast::MatchRecognizePattern::Repetition(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MatchRecognizeSymbol {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MatchRecognizeSymbol::Named(field0) => {
                    sqltk_parser::ast::MatchRecognizeSymbol::Named(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MatchRecognizeSymbol::Start => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MatchRecognizeSymbol::Start,
                        )?
                }
                sqltk_parser::ast::MatchRecognizeSymbol::End => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MatchRecognizeSymbol::End,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Measure {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MergeAction {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MergeAction::Insert(field0) => {
                    sqltk_parser::ast::MergeAction::Insert(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MergeAction::Update { assignments } => {
                    sqltk_parser::ast::MergeAction::Update {
                        assignments: assignments
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::MergeAction::Delete => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::MergeAction::Delete)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MergeClause {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MergeClauseKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MergeClauseKind::Matched => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MergeClauseKind::Matched,
                        )?
                }
                sqltk_parser::ast::MergeClauseKind::NotMatched => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MergeClauseKind::NotMatched,
                        )?
                }
                sqltk_parser::ast::MergeClauseKind::NotMatchedByTarget => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MergeClauseKind::NotMatchedByTarget,
                        )?
                }
                sqltk_parser::ast::MergeClauseKind::NotMatchedBySource => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MergeClauseKind::NotMatchedBySource,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MergeInsertExpr {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MergeInsertKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MergeInsertKind::Values(field0) => {
                    sqltk_parser::ast::MergeInsertKind::Values(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::MergeInsertKind::Row => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::MergeInsertKind::Row)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Method {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { expr, method_chain } = self;
            Self {
                expr: expr.apply_transform_with_path(transformer, node_path)?,
                method_chain: method_chain
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MinMaxValue {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MinMaxValue::Empty => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::MinMaxValue::Empty)?
                }
                sqltk_parser::ast::MinMaxValue::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::MinMaxValue::None)?
                }
                sqltk_parser::ast::MinMaxValue::Some(field0) => {
                    sqltk_parser::ast::MinMaxValue::Some(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MySQLColumnPosition {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MySQLColumnPosition::First => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MySQLColumnPosition::First,
                        )?
                }
                sqltk_parser::ast::MySQLColumnPosition::After(field0) => {
                    sqltk_parser::ast::MySQLColumnPosition::After(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::MysqlInsertPriority {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::MysqlInsertPriority::LowPriority => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MysqlInsertPriority::LowPriority,
                        )?
                }
                sqltk_parser::ast::MysqlInsertPriority::Delayed => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MysqlInsertPriority::Delayed,
                        )?
                }
                sqltk_parser::ast::MysqlInsertPriority::HighPriority => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::MysqlInsertPriority::HighPriority,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::NamedWindowDefinition {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::NamedWindowExpr {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::NamedWindowExpr::NamedWindow(field0) => {
                    sqltk_parser::ast::NamedWindowExpr::NamedWindow(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::NamedWindowExpr::WindowSpec(field0) => {
                    sqltk_parser::ast::NamedWindowExpr::WindowSpec(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::NonBlock {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::NonBlock::Nowait => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::NonBlock::Nowait)?
                }
                sqltk_parser::ast::NonBlock::SkipLocked => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::NonBlock::SkipLocked)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::NullTreatment {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::NullTreatment::IgnoreNulls => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::NullTreatment::IgnoreNulls,
                        )?
                }
                sqltk_parser::ast::NullTreatment::RespectNulls => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::NullTreatment::RespectNulls,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::NullsDistinctOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::NullsDistinctOption::None => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::NullsDistinctOption::None,
                        )?
                }
                sqltk_parser::ast::NullsDistinctOption::Distinct => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::NullsDistinctOption::Distinct,
                        )?
                }
                sqltk_parser::ast::NullsDistinctOption::NotDistinct => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::NullsDistinctOption::NotDistinct,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ObjectName {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ObjectType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ObjectType::Table => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Table)?
                }
                sqltk_parser::ast::ObjectType::View => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::View)?
                }
                sqltk_parser::ast::ObjectType::Index => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Index)?
                }
                sqltk_parser::ast::ObjectType::Schema => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Schema)?
                }
                sqltk_parser::ast::ObjectType::Database => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Database)?
                }
                sqltk_parser::ast::ObjectType::Role => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Role)?
                }
                sqltk_parser::ast::ObjectType::Sequence => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Sequence)?
                }
                sqltk_parser::ast::ObjectType::Stage => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Stage)?
                }
                sqltk_parser::ast::ObjectType::Type => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ObjectType::Type)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Offset {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OffsetRows {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::OffsetRows::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::OffsetRows::None)?
                }
                sqltk_parser::ast::OffsetRows::Row => {
                    transformer.transform(node_path, sqltk_parser::ast::OffsetRows::Row)?
                }
                sqltk_parser::ast::OffsetRows::Rows => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::OffsetRows::Rows)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OnCommit {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::OnCommit::DeleteRows => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::OnCommit::DeleteRows)?
                }
                sqltk_parser::ast::OnCommit::PreserveRows => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::OnCommit::PreserveRows)?
                }
                sqltk_parser::ast::OnCommit::Drop => {
                    transformer.transform(node_path, sqltk_parser::ast::OnCommit::Drop)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OnConflict {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OnConflictAction {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::OnConflictAction::DoNothing => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::OnConflictAction::DoNothing,
                        )?
                }
                sqltk_parser::ast::OnConflictAction::DoUpdate(field0) => {
                    sqltk_parser::ast::OnConflictAction::DoUpdate(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OnInsert {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::OnInsert::DuplicateKeyUpdate(field0) => {
                    sqltk_parser::ast::OnInsert::DuplicateKeyUpdate(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::OnInsert::OnConflict(field0) => {
                    sqltk_parser::ast::OnInsert::OnConflict(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OpenJsonTableColumn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { name, r#type, path, as_json } = self;
            Self {
                name: name.apply_transform_with_path(transformer, node_path)?,
                r#type: r#type.apply_transform_with_path(transformer, node_path)?,
                path: path.apply_transform_with_path(transformer, node_path)?,
                as_json: as_json.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OperateFunctionArg {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OrderBy {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::OrderByExpr {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Owner {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Owner::Ident(field0) => {
                    sqltk_parser::ast::Owner::Ident(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Owner::CurrentRole => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Owner::CurrentRole)?
                }
                sqltk_parser::ast::Owner::CurrentUser => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Owner::CurrentUser)?
                }
                sqltk_parser::ast::Owner::SessionUser => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Owner::SessionUser)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Partition {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Partition::Identifier(field0) => {
                    sqltk_parser::ast::Partition::Identifier(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Partition::Expr(field0) => {
                    sqltk_parser::ast::Partition::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Partition::Part(field0) => {
                    sqltk_parser::ast::Partition::Part(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Partition::Partitions(field0) => {
                    sqltk_parser::ast::Partition::Partitions(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::PartitionRangeDirection {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::PartitionRangeDirection::Left => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::PartitionRangeDirection::Left,
                        )?
                }
                sqltk_parser::ast::PartitionRangeDirection::Right => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::PartitionRangeDirection::Right,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Password {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Password::Password(field0) => {
                    sqltk_parser::ast::Password::Password(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Password::NullPassword => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::Password::NullPassword)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::PivotValueSource {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::PivotValueSource::List(field0) => {
                    sqltk_parser::ast::PivotValueSource::List(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::PivotValueSource::Any(field0) => {
                    sqltk_parser::ast::PivotValueSource::Any(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::PivotValueSource::Subquery(field0) => {
                    sqltk_parser::ast::PivotValueSource::Subquery(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Privileges {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Privileges::All { with_privileges_keyword } => {
                    sqltk_parser::ast::Privileges::All {
                        with_privileges_keyword: with_privileges_keyword
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Privileges::Actions(field0) => {
                    sqltk_parser::ast::Privileges::Actions(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ProcedureParam {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ProjectionSelect {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Query {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ReferentialAction {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ReferentialAction::Restrict => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ReferentialAction::Restrict,
                        )?
                }
                sqltk_parser::ast::ReferentialAction::Cascade => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ReferentialAction::Cascade,
                        )?
                }
                sqltk_parser::ast::ReferentialAction::SetNull => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ReferentialAction::SetNull,
                        )?
                }
                sqltk_parser::ast::ReferentialAction::NoAction => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ReferentialAction::NoAction,
                        )?
                }
                sqltk_parser::ast::ReferentialAction::SetDefault => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ReferentialAction::SetDefault,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::RenameSelectItem {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::RenameSelectItem::Single(field0) => {
                    sqltk_parser::ast::RenameSelectItem::Single(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RenameSelectItem::Multiple(field0) => {
                    sqltk_parser::ast::RenameSelectItem::Multiple(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::RepetitionQuantifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::RepetitionQuantifier::ZeroOrMore => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::RepetitionQuantifier::ZeroOrMore,
                        )?
                }
                sqltk_parser::ast::RepetitionQuantifier::OneOrMore => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::RepetitionQuantifier::OneOrMore,
                        )?
                }
                sqltk_parser::ast::RepetitionQuantifier::AtMostOne => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::RepetitionQuantifier::AtMostOne,
                        )?
                }
                sqltk_parser::ast::RepetitionQuantifier::Exactly(field0) => {
                    sqltk_parser::ast::RepetitionQuantifier::Exactly(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RepetitionQuantifier::AtLeast(field0) => {
                    sqltk_parser::ast::RepetitionQuantifier::AtLeast(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RepetitionQuantifier::AtMost(field0) => {
                    sqltk_parser::ast::RepetitionQuantifier::AtMost(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RepetitionQuantifier::Range(field0, field1) => {
                    sqltk_parser::ast::RepetitionQuantifier::Range(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ReplaceSelectElement {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ReplaceSelectItem {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ResetConfig {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ResetConfig::ALL => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ResetConfig::ALL)?
                }
                sqltk_parser::ast::ResetConfig::ConfigName(field0) => {
                    sqltk_parser::ast::ResetConfig::ConfigName(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::RoleOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::RoleOption::BypassRLS(field0) => {
                    sqltk_parser::ast::RoleOption::BypassRLS(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::ConnectionLimit(field0) => {
                    sqltk_parser::ast::RoleOption::ConnectionLimit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::CreateDB(field0) => {
                    sqltk_parser::ast::RoleOption::CreateDB(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::CreateRole(field0) => {
                    sqltk_parser::ast::RoleOption::CreateRole(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::Inherit(field0) => {
                    sqltk_parser::ast::RoleOption::Inherit(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::Login(field0) => {
                    sqltk_parser::ast::RoleOption::Login(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::Password(field0) => {
                    sqltk_parser::ast::RoleOption::Password(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::Replication(field0) => {
                    sqltk_parser::ast::RoleOption::Replication(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::SuperUser(field0) => {
                    sqltk_parser::ast::RoleOption::SuperUser(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::RoleOption::ValidUntil(field0) => {
                    sqltk_parser::ast::RoleOption::ValidUntil(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::RowAccessPolicy {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::RowsPerMatch {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::RowsPerMatch::OneRow => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::RowsPerMatch::OneRow)?
                }
                sqltk_parser::ast::RowsPerMatch::AllRows(field0) => {
                    sqltk_parser::ast::RowsPerMatch::AllRows(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SchemaName {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SchemaName::Simple(field0) => {
                    sqltk_parser::ast::SchemaName::Simple(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SchemaName::UnnamedAuthorization(field0) => {
                    sqltk_parser::ast::SchemaName::UnnamedAuthorization(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SchemaName::NamedAuthorization(field0, field1) => {
                    sqltk_parser::ast::SchemaName::NamedAuthorization(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SearchModifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SearchModifier::InNaturalLanguageMode => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SearchModifier::InNaturalLanguageMode,
                        )?
                }
                sqltk_parser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion,
                        )?
                }
                sqltk_parser::ast::SearchModifier::InBooleanMode => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SearchModifier::InBooleanMode,
                        )?
                }
                sqltk_parser::ast::SearchModifier::WithQueryExpansion => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SearchModifier::WithQueryExpansion,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SecondaryRoles {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SecondaryRoles::All => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SecondaryRoles::All)?
                }
                sqltk_parser::ast::SecondaryRoles::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SecondaryRoles::None)?
                }
                sqltk_parser::ast::SecondaryRoles::List(field0) => {
                    sqltk_parser::ast::SecondaryRoles::List(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SecretOption {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Select {
    fn apply_transform_with_path<T>(
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
                select_token,
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
                select_token: select_token
                    .apply_transform_with_path(transformer, node_path)?,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SelectInto {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SelectItem {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SelectItem::UnnamedExpr(field0) => {
                    sqltk_parser::ast::SelectItem::UnnamedExpr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                    sqltk_parser::ast::SelectItem::ExprWithAlias {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::SelectItem::QualifiedWildcard(field0, field1) => {
                    sqltk_parser::ast::SelectItem::QualifiedWildcard(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SelectItem::Wildcard(field0) => {
                    sqltk_parser::ast::SelectItem::Wildcard(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SequenceOptions {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SequenceOptions::IncrementBy(field0, field1) => {
                    sqltk_parser::ast::SequenceOptions::IncrementBy(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SequenceOptions::MinValue(field0) => {
                    sqltk_parser::ast::SequenceOptions::MinValue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SequenceOptions::MaxValue(field0) => {
                    sqltk_parser::ast::SequenceOptions::MaxValue(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SequenceOptions::StartWith(field0, field1) => {
                    sqltk_parser::ast::SequenceOptions::StartWith(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SequenceOptions::Cache(field0) => {
                    sqltk_parser::ast::SequenceOptions::Cache(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SequenceOptions::Cycle(field0) => {
                    sqltk_parser::ast::SequenceOptions::Cycle(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SetConfigValue {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SetConfigValue::Default => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SetConfigValue::Default,
                        )?
                }
                sqltk_parser::ast::SetConfigValue::FromCurrent => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SetConfigValue::FromCurrent,
                        )?
                }
                sqltk_parser::ast::SetConfigValue::Value(field0) => {
                    sqltk_parser::ast::SetConfigValue::Value(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SetExpr {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SetExpr::Select(field0) => {
                    sqltk_parser::ast::SetExpr::Select(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SetExpr::Query(field0) => {
                    sqltk_parser::ast::SetExpr::Query(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SetExpr::SetOperation {
                    op,
                    set_quantifier,
                    left,
                    right,
                } => {
                    sqltk_parser::ast::SetExpr::SetOperation {
                        op: op.apply_transform_with_path(transformer, node_path)?,
                        set_quantifier: set_quantifier
                            .apply_transform_with_path(transformer, node_path)?,
                        left: left.apply_transform_with_path(transformer, node_path)?,
                        right: right.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::SetExpr::Values(field0) => {
                    sqltk_parser::ast::SetExpr::Values(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SetExpr::Insert(field0) => {
                    sqltk_parser::ast::SetExpr::Insert(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SetExpr::Update(field0) => {
                    sqltk_parser::ast::SetExpr::Update(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SetExpr::Table(field0) => {
                    sqltk_parser::ast::SetExpr::Table(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SetOperator {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SetOperator::Union => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SetOperator::Union)?
                }
                sqltk_parser::ast::SetOperator::Except => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SetOperator::Except)?
                }
                sqltk_parser::ast::SetOperator::Intersect => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SetOperator::Intersect)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SetQuantifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SetQuantifier::All => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SetQuantifier::All)?
                }
                sqltk_parser::ast::SetQuantifier::Distinct => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SetQuantifier::Distinct,
                        )?
                }
                sqltk_parser::ast::SetQuantifier::ByName => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SetQuantifier::ByName)?
                }
                sqltk_parser::ast::SetQuantifier::AllByName => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SetQuantifier::AllByName,
                        )?
                }
                sqltk_parser::ast::SetQuantifier::DistinctByName => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SetQuantifier::DistinctByName,
                        )?
                }
                sqltk_parser::ast::SetQuantifier::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SetQuantifier::None)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Setting {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ShowCreateObject {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ShowCreateObject::Event => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowCreateObject::Event,
                        )?
                }
                sqltk_parser::ast::ShowCreateObject::Function => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowCreateObject::Function,
                        )?
                }
                sqltk_parser::ast::ShowCreateObject::Procedure => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowCreateObject::Procedure,
                        )?
                }
                sqltk_parser::ast::ShowCreateObject::Table => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowCreateObject::Table,
                        )?
                }
                sqltk_parser::ast::ShowCreateObject::Trigger => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowCreateObject::Trigger,
                        )?
                }
                sqltk_parser::ast::ShowCreateObject::View => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::ShowCreateObject::View)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ShowStatementFilter {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ShowStatementFilter::Like(field0) => {
                    sqltk_parser::ast::ShowStatementFilter::Like(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ShowStatementFilter::ILike(field0) => {
                    sqltk_parser::ast::ShowStatementFilter::ILike(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ShowStatementFilter::Where(field0) => {
                    sqltk_parser::ast::ShowStatementFilter::Where(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ShowStatementFilter::NoKeyword(field0) => {
                    sqltk_parser::ast::ShowStatementFilter::NoKeyword(
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
impl<'ast> crate::Transformable<'ast>
for sqltk_parser::ast::ShowStatementFilterPosition {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ShowStatementFilterPosition::Infix(field0) => {
                    sqltk_parser::ast::ShowStatementFilterPosition::Infix(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::ShowStatementFilterPosition::Suffix(field0) => {
                    sqltk_parser::ast::ShowStatementFilterPosition::Suffix(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ShowStatementIn {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { clause, parent_type, parent_name } = self;
            Self {
                clause: clause.apply_transform_with_path(transformer, node_path)?,
                parent_type: parent_type
                    .apply_transform_with_path(transformer, node_path)?,
                parent_name: parent_name
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ShowStatementInClause {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ShowStatementInClause::IN => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInClause::IN,
                        )?
                }
                sqltk_parser::ast::ShowStatementInClause::FROM => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInClause::FROM,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ShowStatementInParentType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ShowStatementInParentType::Account => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInParentType::Account,
                        )?
                }
                sqltk_parser::ast::ShowStatementInParentType::Database => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInParentType::Database,
                        )?
                }
                sqltk_parser::ast::ShowStatementInParentType::Schema => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInParentType::Schema,
                        )?
                }
                sqltk_parser::ast::ShowStatementInParentType::Table => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInParentType::Table,
                        )?
                }
                sqltk_parser::ast::ShowStatementInParentType::View => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ShowStatementInParentType::View,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ShowStatementOptions {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { show_in, starts_with, limit, limit_from, filter_position } = self;
            Self {
                show_in: show_in.apply_transform_with_path(transformer, node_path)?,
                starts_with: starts_with
                    .apply_transform_with_path(transformer, node_path)?,
                limit: limit.apply_transform_with_path(transformer, node_path)?,
                limit_from: limit_from
                    .apply_transform_with_path(transformer, node_path)?,
                filter_position: filter_position
                    .apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SqlOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SqlOption::Clustered(field0) => {
                    sqltk_parser::ast::SqlOption::Clustered(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SqlOption::Ident(field0) => {
                    sqltk_parser::ast::SqlOption::Ident(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::SqlOption::KeyValue { key, value } => {
                    sqltk_parser::ast::SqlOption::KeyValue {
                        key: key.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::SqlOption::Partition {
                    column_name,
                    range_direction,
                    for_values,
                } => {
                    sqltk_parser::ast::SqlOption::Partition {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SqliteOnConflict {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::SqliteOnConflict::Rollback => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SqliteOnConflict::Rollback,
                        )?
                }
                sqltk_parser::ast::SqliteOnConflict::Abort => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SqliteOnConflict::Abort,
                        )?
                }
                sqltk_parser::ast::SqliteOnConflict::Fail => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::SqliteOnConflict::Fail)?
                }
                sqltk_parser::ast::SqliteOnConflict::Ignore => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SqliteOnConflict::Ignore,
                        )?
                }
                sqltk_parser::ast::SqliteOnConflict::Replace => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::SqliteOnConflict::Replace,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Statement {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Statement::Analyze {
                    table_name,
                    partitions,
                    for_columns,
                    columns,
                    cache_metadata,
                    noscan,
                    compute_statistics,
                } => {
                    sqltk_parser::ast::Statement::Analyze {
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
                sqltk_parser::ast::Statement::Truncate {
                    table_names,
                    partitions,
                    table,
                    only,
                    identity,
                    cascade,
                    on_cluster,
                } => {
                    sqltk_parser::ast::Statement::Truncate {
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
                sqltk_parser::ast::Statement::Msck {
                    table_name,
                    repair,
                    partition_action,
                } => {
                    sqltk_parser::ast::Statement::Msck {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        repair: repair
                            .apply_transform_with_path(transformer, node_path)?,
                        partition_action: partition_action
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Query(field0) => {
                    sqltk_parser::ast::Statement::Query(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::Insert(field0) => {
                    sqltk_parser::ast::Statement::Insert(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::Install { extension_name } => {
                    sqltk_parser::ast::Statement::Install {
                        extension_name: extension_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Load { extension_name } => {
                    sqltk_parser::ast::Statement::Load {
                        extension_name: extension_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Directory {
                    overwrite,
                    local,
                    path,
                    file_format,
                    source,
                } => {
                    sqltk_parser::ast::Statement::Directory {
                        overwrite: overwrite
                            .apply_transform_with_path(transformer, node_path)?,
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        path: path.apply_transform_with_path(transformer, node_path)?,
                        file_format: file_format
                            .apply_transform_with_path(transformer, node_path)?,
                        source: source.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Call(field0) => {
                    sqltk_parser::ast::Statement::Call(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::Copy {
                    source,
                    to,
                    target,
                    options,
                    legacy_options,
                    values,
                } => {
                    sqltk_parser::ast::Statement::Copy {
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
                sqltk_parser::ast::Statement::CopyIntoSnowflake {
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
                    sqltk_parser::ast::Statement::CopyIntoSnowflake {
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
                sqltk_parser::ast::Statement::Close { cursor } => {
                    sqltk_parser::ast::Statement::Close {
                        cursor: cursor.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Update {
                    table,
                    assignments,
                    from,
                    selection,
                    returning,
                    or,
                } => {
                    sqltk_parser::ast::Statement::Update {
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        assignments: assignments
                            .apply_transform_with_path(transformer, node_path)?,
                        from: from.apply_transform_with_path(transformer, node_path)?,
                        selection: selection
                            .apply_transform_with_path(transformer, node_path)?,
                        returning: returning
                            .apply_transform_with_path(transformer, node_path)?,
                        or: or.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Delete(field0) => {
                    sqltk_parser::ast::Statement::Delete(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::CreateView {
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
                    sqltk_parser::ast::Statement::CreateView {
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
                sqltk_parser::ast::Statement::CreateTable(field0) => {
                    sqltk_parser::ast::Statement::CreateTable(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::CreateVirtualTable {
                    name,
                    if_not_exists,
                    module_name,
                    module_args,
                } => {
                    sqltk_parser::ast::Statement::CreateVirtualTable {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        module_name: module_name
                            .apply_transform_with_path(transformer, node_path)?,
                        module_args: module_args
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateIndex(field0) => {
                    sqltk_parser::ast::Statement::CreateIndex(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::CreateRole {
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
                    sqltk_parser::ast::Statement::CreateRole {
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
                sqltk_parser::ast::Statement::CreateSecret {
                    or_replace,
                    temporary,
                    if_not_exists,
                    name,
                    storage_specifier,
                    secret_type,
                    options,
                } => {
                    sqltk_parser::ast::Statement::CreateSecret {
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
                sqltk_parser::ast::Statement::CreatePolicy {
                    name,
                    table_name,
                    policy_type,
                    command,
                    to,
                    using,
                    with_check,
                } => {
                    sqltk_parser::ast::Statement::CreatePolicy {
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
                sqltk_parser::ast::Statement::AlterTable {
                    name,
                    if_exists,
                    only,
                    operations,
                    location,
                    on_cluster,
                } => {
                    sqltk_parser::ast::Statement::AlterTable {
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
                sqltk_parser::ast::Statement::AlterIndex { name, operation } => {
                    sqltk_parser::ast::Statement::AlterIndex {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        operation: operation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::AlterView {
                    name,
                    columns,
                    query,
                    with_options,
                } => {
                    sqltk_parser::ast::Statement::AlterView {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        query: query.apply_transform_with_path(transformer, node_path)?,
                        with_options: with_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::AlterRole { name, operation } => {
                    sqltk_parser::ast::Statement::AlterRole {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        operation: operation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::AlterPolicy {
                    name,
                    table_name,
                    operation,
                } => {
                    sqltk_parser::ast::Statement::AlterPolicy {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        operation: operation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::AttachDatabase {
                    schema_name,
                    database_file_name,
                    database,
                } => {
                    sqltk_parser::ast::Statement::AttachDatabase {
                        schema_name: schema_name
                            .apply_transform_with_path(transformer, node_path)?,
                        database_file_name: database_file_name
                            .apply_transform_with_path(transformer, node_path)?,
                        database: database
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::AttachDuckDBDatabase {
                    if_not_exists,
                    database,
                    database_path,
                    database_alias,
                    attach_options,
                } => {
                    sqltk_parser::ast::Statement::AttachDuckDBDatabase {
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
                sqltk_parser::ast::Statement::DetachDuckDBDatabase {
                    if_exists,
                    database,
                    database_alias,
                } => {
                    sqltk_parser::ast::Statement::DetachDuckDBDatabase {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        database: database
                            .apply_transform_with_path(transformer, node_path)?,
                        database_alias: database_alias
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Drop {
                    object_type,
                    if_exists,
                    names,
                    cascade,
                    restrict,
                    purge,
                    temporary,
                } => {
                    sqltk_parser::ast::Statement::Drop {
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
                sqltk_parser::ast::Statement::DropFunction {
                    if_exists,
                    func_desc,
                    option,
                } => {
                    sqltk_parser::ast::Statement::DropFunction {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        func_desc: func_desc
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::DropProcedure {
                    if_exists,
                    proc_desc,
                    option,
                } => {
                    sqltk_parser::ast::Statement::DropProcedure {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        proc_desc: proc_desc
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::DropSecret {
                    if_exists,
                    temporary,
                    name,
                    storage_specifier,
                } => {
                    sqltk_parser::ast::Statement::DropSecret {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        temporary: temporary
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        storage_specifier: storage_specifier
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::DropPolicy {
                    if_exists,
                    name,
                    table_name,
                    option,
                } => {
                    sqltk_parser::ast::Statement::DropPolicy {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Declare { stmts } => {
                    sqltk_parser::ast::Statement::Declare {
                        stmts: stmts.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateExtension {
                    name,
                    if_not_exists,
                    cascade,
                    schema,
                    version,
                } => {
                    sqltk_parser::ast::Statement::CreateExtension {
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
                sqltk_parser::ast::Statement::Fetch { name, direction, into } => {
                    sqltk_parser::ast::Statement::Fetch {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        direction: direction
                            .apply_transform_with_path(transformer, node_path)?,
                        into: into.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Flush {
                    object_type,
                    location,
                    channel,
                    read_lock,
                    export,
                    tables,
                } => {
                    sqltk_parser::ast::Statement::Flush {
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
                sqltk_parser::ast::Statement::Discard { object_type } => {
                    sqltk_parser::ast::Statement::Discard {
                        object_type: object_type
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::SetRole { context_modifier, role_name } => {
                    sqltk_parser::ast::Statement::SetRole {
                        context_modifier: context_modifier
                            .apply_transform_with_path(transformer, node_path)?,
                        role_name: role_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::SetVariable {
                    local,
                    hivevar,
                    variables,
                    value,
                } => {
                    sqltk_parser::ast::Statement::SetVariable {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        hivevar: hivevar
                            .apply_transform_with_path(transformer, node_path)?,
                        variables: variables
                            .apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::SetTimeZone { local, value } => {
                    sqltk_parser::ast::Statement::SetTimeZone {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::SetNames {
                    charset_name,
                    collation_name,
                } => {
                    sqltk_parser::ast::Statement::SetNames {
                        charset_name: charset_name
                            .apply_transform_with_path(transformer, node_path)?,
                        collation_name: collation_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::SetNamesDefault {} => {
                    sqltk_parser::ast::Statement::SetNamesDefault {
                    }
                }
                sqltk_parser::ast::Statement::ShowFunctions { filter } => {
                    sqltk_parser::ast::Statement::ShowFunctions {
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowVariable { variable } => {
                    sqltk_parser::ast::Statement::ShowVariable {
                        variable: variable
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowStatus { filter, global, session } => {
                    sqltk_parser::ast::Statement::ShowStatus {
                        filter: filter
                            .apply_transform_with_path(transformer, node_path)?,
                        global: global
                            .apply_transform_with_path(transformer, node_path)?,
                        session: session
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowVariables {
                    filter,
                    global,
                    session,
                } => {
                    sqltk_parser::ast::Statement::ShowVariables {
                        filter: filter
                            .apply_transform_with_path(transformer, node_path)?,
                        global: global
                            .apply_transform_with_path(transformer, node_path)?,
                        session: session
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowCreate { obj_type, obj_name } => {
                    sqltk_parser::ast::Statement::ShowCreate {
                        obj_type: obj_type
                            .apply_transform_with_path(transformer, node_path)?,
                        obj_name: obj_name
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowColumns {
                    extended,
                    full,
                    show_options,
                } => {
                    sqltk_parser::ast::Statement::ShowColumns {
                        extended: extended
                            .apply_transform_with_path(transformer, node_path)?,
                        full: full.apply_transform_with_path(transformer, node_path)?,
                        show_options: show_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowDatabases {
                    terse,
                    history,
                    show_options,
                } => {
                    sqltk_parser::ast::Statement::ShowDatabases {
                        terse: terse.apply_transform_with_path(transformer, node_path)?,
                        history: history
                            .apply_transform_with_path(transformer, node_path)?,
                        show_options: show_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowSchemas {
                    terse,
                    history,
                    show_options,
                } => {
                    sqltk_parser::ast::Statement::ShowSchemas {
                        terse: terse.apply_transform_with_path(transformer, node_path)?,
                        history: history
                            .apply_transform_with_path(transformer, node_path)?,
                        show_options: show_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowTables {
                    terse,
                    history,
                    extended,
                    full,
                    external,
                    show_options,
                } => {
                    sqltk_parser::ast::Statement::ShowTables {
                        terse: terse.apply_transform_with_path(transformer, node_path)?,
                        history: history
                            .apply_transform_with_path(transformer, node_path)?,
                        extended: extended
                            .apply_transform_with_path(transformer, node_path)?,
                        full: full.apply_transform_with_path(transformer, node_path)?,
                        external: external
                            .apply_transform_with_path(transformer, node_path)?,
                        show_options: show_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowViews {
                    terse,
                    materialized,
                    show_options,
                } => {
                    sqltk_parser::ast::Statement::ShowViews {
                        terse: terse.apply_transform_with_path(transformer, node_path)?,
                        materialized: materialized
                            .apply_transform_with_path(transformer, node_path)?,
                        show_options: show_options
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ShowCollation { filter } => {
                    sqltk_parser::ast::Statement::ShowCollation {
                        filter: filter.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Use(field0) => {
                    sqltk_parser::ast::Statement::Use(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::StartTransaction {
                    modes,
                    begin,
                    transaction,
                    modifier,
                } => {
                    sqltk_parser::ast::Statement::StartTransaction {
                        modes: modes.apply_transform_with_path(transformer, node_path)?,
                        begin: begin.apply_transform_with_path(transformer, node_path)?,
                        transaction: transaction
                            .apply_transform_with_path(transformer, node_path)?,
                        modifier: modifier
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::SetTransaction {
                    modes,
                    snapshot,
                    session,
                } => {
                    sqltk_parser::ast::Statement::SetTransaction {
                        modes: modes.apply_transform_with_path(transformer, node_path)?,
                        snapshot: snapshot
                            .apply_transform_with_path(transformer, node_path)?,
                        session: session
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Comment {
                    object_type,
                    object_name,
                    comment,
                    if_exists,
                } => {
                    sqltk_parser::ast::Statement::Comment {
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
                sqltk_parser::ast::Statement::Commit { chain } => {
                    sqltk_parser::ast::Statement::Commit {
                        chain: chain.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Rollback { chain, savepoint } => {
                    sqltk_parser::ast::Statement::Rollback {
                        chain: chain.apply_transform_with_path(transformer, node_path)?,
                        savepoint: savepoint
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateSchema {
                    schema_name,
                    if_not_exists,
                } => {
                    sqltk_parser::ast::Statement::CreateSchema {
                        schema_name: schema_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_not_exists: if_not_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateDatabase {
                    db_name,
                    if_not_exists,
                    location,
                    managed_location,
                } => {
                    sqltk_parser::ast::Statement::CreateDatabase {
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
                sqltk_parser::ast::Statement::CreateFunction(field0) => {
                    sqltk_parser::ast::Statement::CreateFunction(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Statement::CreateTrigger {
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
                    sqltk_parser::ast::Statement::CreateTrigger {
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
                sqltk_parser::ast::Statement::DropTrigger {
                    if_exists,
                    trigger_name,
                    table_name,
                    option,
                } => {
                    sqltk_parser::ast::Statement::DropTrigger {
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                        trigger_name: trigger_name
                            .apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        option: option.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateProcedure {
                    or_alter,
                    name,
                    params,
                    body,
                } => {
                    sqltk_parser::ast::Statement::CreateProcedure {
                        or_alter: or_alter
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        params: params
                            .apply_transform_with_path(transformer, node_path)?,
                        body: body.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateMacro {
                    or_replace,
                    temporary,
                    name,
                    args,
                    definition,
                } => {
                    sqltk_parser::ast::Statement::CreateMacro {
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
                sqltk_parser::ast::Statement::CreateStage {
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
                    sqltk_parser::ast::Statement::CreateStage {
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
                sqltk_parser::ast::Statement::Assert { condition, message } => {
                    sqltk_parser::ast::Statement::Assert {
                        condition: condition
                            .apply_transform_with_path(transformer, node_path)?,
                        message: message
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Grant {
                    privileges,
                    objects,
                    grantees,
                    with_grant_option,
                    granted_by,
                } => {
                    sqltk_parser::ast::Statement::Grant {
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
                sqltk_parser::ast::Statement::Revoke {
                    privileges,
                    objects,
                    grantees,
                    granted_by,
                    cascade,
                } => {
                    sqltk_parser::ast::Statement::Revoke {
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
                sqltk_parser::ast::Statement::Deallocate { name, prepare } => {
                    sqltk_parser::ast::Statement::Deallocate {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        prepare: prepare
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Execute {
                    name,
                    parameters,
                    has_parentheses,
                    using,
                } => {
                    sqltk_parser::ast::Statement::Execute {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        parameters: parameters
                            .apply_transform_with_path(transformer, node_path)?,
                        has_parentheses: has_parentheses
                            .apply_transform_with_path(transformer, node_path)?,
                        using: using.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Prepare { name, data_types, statement } => {
                    sqltk_parser::ast::Statement::Prepare {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        data_types: data_types
                            .apply_transform_with_path(transformer, node_path)?,
                        statement: statement
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Kill { modifier, id } => {
                    sqltk_parser::ast::Statement::Kill {
                        modifier: modifier
                            .apply_transform_with_path(transformer, node_path)?,
                        id: id.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ExplainTable {
                    describe_alias,
                    hive_format,
                    has_table_keyword,
                    table_name,
                } => {
                    sqltk_parser::ast::Statement::ExplainTable {
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
                sqltk_parser::ast::Statement::Explain {
                    describe_alias,
                    analyze,
                    verbose,
                    query_plan,
                    statement,
                    format,
                    options,
                } => {
                    sqltk_parser::ast::Statement::Explain {
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
                sqltk_parser::ast::Statement::Savepoint { name } => {
                    sqltk_parser::ast::Statement::Savepoint {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::ReleaseSavepoint { name } => {
                    sqltk_parser::ast::Statement::ReleaseSavepoint {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Merge {
                    into,
                    table,
                    source,
                    on,
                    clauses,
                } => {
                    sqltk_parser::ast::Statement::Merge {
                        into: into.apply_transform_with_path(transformer, node_path)?,
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        source: source
                            .apply_transform_with_path(transformer, node_path)?,
                        on: on.apply_transform_with_path(transformer, node_path)?,
                        clauses: clauses
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Cache {
                    table_flag,
                    table_name,
                    has_as,
                    options,
                    query,
                } => {
                    sqltk_parser::ast::Statement::Cache {
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
                sqltk_parser::ast::Statement::UNCache { table_name, if_exists } => {
                    sqltk_parser::ast::Statement::UNCache {
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        if_exists: if_exists
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::CreateSequence {
                    temporary,
                    if_not_exists,
                    name,
                    data_type,
                    sequence_options,
                    owned_by,
                } => {
                    sqltk_parser::ast::Statement::CreateSequence {
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
                sqltk_parser::ast::Statement::CreateType { name, representation } => {
                    sqltk_parser::ast::Statement::CreateType {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        representation: representation
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::Pragma { name, value, is_eq } => {
                    sqltk_parser::ast::Statement::Pragma {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                        is_eq: is_eq.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::LockTables { tables } => {
                    sqltk_parser::ast::Statement::LockTables {
                        tables: tables.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::UnlockTables => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::Statement::UnlockTables,
                        )?
                }
                sqltk_parser::ast::Statement::Unload { query, to, with } => {
                    sqltk_parser::ast::Statement::Unload {
                        query: query.apply_transform_with_path(transformer, node_path)?,
                        to: to.apply_transform_with_path(transformer, node_path)?,
                        with: with.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::OptimizeTable {
                    name,
                    on_cluster,
                    partition,
                    include_final,
                    deduplicate,
                } => {
                    sqltk_parser::ast::Statement::OptimizeTable {
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
                sqltk_parser::ast::Statement::LISTEN { channel } => {
                    sqltk_parser::ast::Statement::LISTEN {
                        channel: channel
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::UNLISTEN { channel } => {
                    sqltk_parser::ast::Statement::UNLISTEN {
                        channel: channel
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::NOTIFY { channel, payload } => {
                    sqltk_parser::ast::Statement::NOTIFY {
                        channel: channel
                            .apply_transform_with_path(transformer, node_path)?,
                        payload: payload
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Statement::LoadData {
                    local,
                    inpath,
                    overwrite,
                    table_name,
                    partitioned,
                    table_format,
                } => {
                    sqltk_parser::ast::Statement::LoadData {
                        local: local.apply_transform_with_path(transformer, node_path)?,
                        inpath: inpath
                            .apply_transform_with_path(transformer, node_path)?,
                        overwrite: overwrite
                            .apply_transform_with_path(transformer, node_path)?,
                        table_name: table_name
                            .apply_transform_with_path(transformer, node_path)?,
                        partitioned: partitioned
                            .apply_transform_with_path(transformer, node_path)?,
                        table_format: table_format
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::StructBracketKind {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::StructBracketKind::Parentheses => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::StructBracketKind::Parentheses,
                        )?
                }
                sqltk_parser::ast::StructBracketKind::AngleBrackets => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::StructBracketKind::AngleBrackets,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::StructField {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Subscript {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Subscript::Index { index } => {
                    sqltk_parser::ast::Subscript::Index {
                        index: index.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::Subscript::Slice {
                    lower_bound,
                    upper_bound,
                    stride,
                } => {
                    sqltk_parser::ast::Subscript::Slice {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::SymbolDefinition {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Table {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableAlias {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableAliasColumnDef {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableConstraint {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TableConstraint::Unique {
                    name,
                    index_name,
                    index_type_display,
                    index_type,
                    columns,
                    index_options,
                    characteristics,
                    nulls_distinct,
                } => {
                    sqltk_parser::ast::TableConstraint::Unique {
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
                        nulls_distinct: nulls_distinct
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableConstraint::PrimaryKey {
                    name,
                    index_name,
                    index_type,
                    columns,
                    index_options,
                    characteristics,
                } => {
                    sqltk_parser::ast::TableConstraint::PrimaryKey {
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
                sqltk_parser::ast::TableConstraint::ForeignKey {
                    name,
                    columns,
                    foreign_table,
                    referred_columns,
                    on_delete,
                    on_update,
                    characteristics,
                } => {
                    sqltk_parser::ast::TableConstraint::ForeignKey {
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
                sqltk_parser::ast::TableConstraint::Check { name, expr } => {
                    sqltk_parser::ast::TableConstraint::Check {
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableConstraint::Index {
                    display_as_key,
                    name,
                    index_type,
                    columns,
                } => {
                    sqltk_parser::ast::TableConstraint::Index {
                        display_as_key: display_as_key
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        index_type: index_type
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableConstraint::FulltextOrSpatial {
                    fulltext,
                    index_type_display,
                    opt_index_name,
                    columns,
                } => {
                    sqltk_parser::ast::TableConstraint::FulltextOrSpatial {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableEngine {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableFactor {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TableFactor::Table {
                    name,
                    alias,
                    args,
                    with_hints,
                    version,
                    with_ordinality,
                    partitions,
                    json_path,
                } => {
                    sqltk_parser::ast::TableFactor::Table {
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
                        json_path: json_path
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::Derived { lateral, subquery, alias } => {
                    sqltk_parser::ast::TableFactor::Derived {
                        lateral: lateral
                            .apply_transform_with_path(transformer, node_path)?,
                        subquery: subquery
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::TableFunction { expr, alias } => {
                    sqltk_parser::ast::TableFactor::TableFunction {
                        expr: expr.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::Function {
                    lateral,
                    name,
                    args,
                    alias,
                } => {
                    sqltk_parser::ast::TableFactor::Function {
                        lateral: lateral
                            .apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        args: args.apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::UNNEST {
                    alias,
                    array_exprs,
                    with_offset,
                    with_offset_alias,
                    with_ordinality,
                } => {
                    sqltk_parser::ast::TableFactor::UNNEST {
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
                sqltk_parser::ast::TableFactor::JsonTable {
                    json_expr,
                    json_path,
                    columns,
                    alias,
                } => {
                    sqltk_parser::ast::TableFactor::JsonTable {
                        json_expr: json_expr
                            .apply_transform_with_path(transformer, node_path)?,
                        json_path: json_path
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::OpenJsonTable {
                    json_expr,
                    json_path,
                    columns,
                    alias,
                } => {
                    sqltk_parser::ast::TableFactor::OpenJsonTable {
                        json_expr: json_expr
                            .apply_transform_with_path(transformer, node_path)?,
                        json_path: json_path
                            .apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::NestedJoin {
                    table_with_joins,
                    alias,
                } => {
                    sqltk_parser::ast::TableFactor::NestedJoin {
                        table_with_joins: table_with_joins
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::Pivot {
                    table,
                    aggregate_functions,
                    value_column,
                    value_source,
                    default_on_null,
                    alias,
                } => {
                    sqltk_parser::ast::TableFactor::Pivot {
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
                sqltk_parser::ast::TableFactor::Unpivot {
                    table,
                    value,
                    name,
                    columns,
                    alias,
                } => {
                    sqltk_parser::ast::TableFactor::Unpivot {
                        table: table.apply_transform_with_path(transformer, node_path)?,
                        value: value.apply_transform_with_path(transformer, node_path)?,
                        name: name.apply_transform_with_path(transformer, node_path)?,
                        columns: columns
                            .apply_transform_with_path(transformer, node_path)?,
                        alias: alias.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::TableFactor::MatchRecognize {
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
                    sqltk_parser::ast::TableFactor::MatchRecognize {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableFunctionArgs {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableOptionsClustered {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TableOptionsClustered::ColumnstoreIndex => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TableOptionsClustered::ColumnstoreIndex,
                        )?
                }
                sqltk_parser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                    field0,
                ) => {
                    sqltk_parser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::TableOptionsClustered::Index(field0) => {
                    sqltk_parser::ast::TableOptionsClustered::Index(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableVersion {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TableVersion::ForSystemTimeAsOf(field0) => {
                    sqltk_parser::ast::TableVersion::ForSystemTimeAsOf(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TableWithJoins {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Tag {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TagsColumnOption {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TimezoneInfo {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TimezoneInfo::None => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TimezoneInfo::None)?
                }
                sqltk_parser::ast::TimezoneInfo::WithTimeZone => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TimezoneInfo::WithTimeZone,
                        )?
                }
                sqltk_parser::ast::TimezoneInfo::WithoutTimeZone => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TimezoneInfo::WithoutTimeZone,
                        )?
                }
                sqltk_parser::ast::TimezoneInfo::Tz => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TimezoneInfo::Tz)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Top {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TopQuantity {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TopQuantity::Expr(field0) => {
                    sqltk_parser::ast::TopQuantity::Expr(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::TopQuantity::Constant(field0) => {
                    sqltk_parser::ast::TopQuantity::Constant(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TransactionAccessMode {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TransactionAccessMode::ReadOnly => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionAccessMode::ReadOnly,
                        )?
                }
                sqltk_parser::ast::TransactionAccessMode::ReadWrite => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionAccessMode::ReadWrite,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TransactionIsolationLevel {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TransactionIsolationLevel::ReadUncommitted => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionIsolationLevel::ReadUncommitted,
                        )?
                }
                sqltk_parser::ast::TransactionIsolationLevel::ReadCommitted => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionIsolationLevel::ReadCommitted,
                        )?
                }
                sqltk_parser::ast::TransactionIsolationLevel::RepeatableRead => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionIsolationLevel::RepeatableRead,
                        )?
                }
                sqltk_parser::ast::TransactionIsolationLevel::Serializable => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionIsolationLevel::Serializable,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TransactionMode {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TransactionMode::AccessMode(field0) => {
                    sqltk_parser::ast::TransactionMode::AccessMode(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::TransactionMode::IsolationLevel(field0) => {
                    sqltk_parser::ast::TransactionMode::IsolationLevel(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TransactionModifier {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TransactionModifier::Deferred => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionModifier::Deferred,
                        )?
                }
                sqltk_parser::ast::TransactionModifier::Immediate => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionModifier::Immediate,
                        )?
                }
                sqltk_parser::ast::TransactionModifier::Exclusive => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TransactionModifier::Exclusive,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerEvent {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TriggerEvent::Insert => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TriggerEvent::Insert)?
                }
                sqltk_parser::ast::TriggerEvent::Update(field0) => {
                    sqltk_parser::ast::TriggerEvent::Update(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::TriggerEvent::Delete => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TriggerEvent::Delete)?
                }
                sqltk_parser::ast::TriggerEvent::Truncate => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TriggerEvent::Truncate)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerExecBody {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerExecBodyType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TriggerExecBodyType::Function => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TriggerExecBodyType::Function,
                        )?
                }
                sqltk_parser::ast::TriggerExecBodyType::Procedure => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TriggerExecBodyType::Procedure,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerObject {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TriggerObject::Row => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TriggerObject::Row)?
                }
                sqltk_parser::ast::TriggerObject::Statement => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TriggerObject::Statement,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerPeriod {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TriggerPeriod::After => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TriggerPeriod::After)?
                }
                sqltk_parser::ast::TriggerPeriod::Before => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TriggerPeriod::Before)?
                }
                sqltk_parser::ast::TriggerPeriod::InsteadOf => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TriggerPeriod::InsteadOf,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerReferencing {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TriggerReferencingType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TriggerReferencingType::OldTable => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TriggerReferencingType::OldTable,
                        )?
                }
                sqltk_parser::ast::TriggerReferencingType::NewTable => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TriggerReferencingType::NewTable,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TrimWhereField {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TrimWhereField::Both => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::TrimWhereField::Both)?
                }
                sqltk_parser::ast::TrimWhereField::Leading => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TrimWhereField::Leading,
                        )?
                }
                sqltk_parser::ast::TrimWhereField::Trailing => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TrimWhereField::Trailing,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TruncateCascadeOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TruncateCascadeOption::Cascade => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TruncateCascadeOption::Cascade,
                        )?
                }
                sqltk_parser::ast::TruncateCascadeOption::Restrict => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TruncateCascadeOption::Restrict,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TruncateIdentityOption {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::TruncateIdentityOption::Restart => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TruncateIdentityOption::Restart,
                        )?
                }
                sqltk_parser::ast::TruncateIdentityOption::Continue => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::TruncateIdentityOption::Continue,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::TruncateTableTarget {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::UnaryOperator {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::UnaryOperator::Plus => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::UnaryOperator::Plus)?
                }
                sqltk_parser::ast::UnaryOperator::Minus => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::UnaryOperator::Minus)?
                }
                sqltk_parser::ast::UnaryOperator::Not => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::UnaryOperator::Not)?
                }
                sqltk_parser::ast::UnaryOperator::PGBitwiseNot => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::UnaryOperator::PGBitwiseNot,
                        )?
                }
                sqltk_parser::ast::UnaryOperator::PGSquareRoot => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::UnaryOperator::PGSquareRoot,
                        )?
                }
                sqltk_parser::ast::UnaryOperator::PGCubeRoot => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::UnaryOperator::PGCubeRoot,
                        )?
                }
                sqltk_parser::ast::UnaryOperator::PGPostfixFactorial => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::UnaryOperator::PGPostfixFactorial,
                        )?
                }
                sqltk_parser::ast::UnaryOperator::PGPrefixFactorial => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::UnaryOperator::PGPrefixFactorial,
                        )?
                }
                sqltk_parser::ast::UnaryOperator::PGAbs => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::UnaryOperator::PGAbs)?
                }
                sqltk_parser::ast::UnaryOperator::BangNot => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::UnaryOperator::BangNot)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::UnionField {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Use {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Use::Catalog(field0) => {
                    sqltk_parser::ast::Use::Catalog(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::Schema(field0) => {
                    sqltk_parser::ast::Use::Schema(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::Database(field0) => {
                    sqltk_parser::ast::Use::Database(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::Warehouse(field0) => {
                    sqltk_parser::ast::Use::Warehouse(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::Role(field0) => {
                    sqltk_parser::ast::Use::Role(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::SecondaryRoles(field0) => {
                    sqltk_parser::ast::Use::SecondaryRoles(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::Object(field0) => {
                    sqltk_parser::ast::Use::Object(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Use::Default => {
                    transformer.transform(node_path, sqltk_parser::ast::Use::Default)?
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
for sqltk_parser::ast::UserDefinedTypeCompositeAttributeDef {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast>
for sqltk_parser::ast::UserDefinedTypeRepresentation {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::UserDefinedTypeRepresentation::Composite {
                    attributes,
                } => {
                    sqltk_parser::ast::UserDefinedTypeRepresentation::Composite {
                        attributes: attributes
                            .apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::ast::UserDefinedTypeRepresentation::Enum { labels } => {
                    sqltk_parser::ast::UserDefinedTypeRepresentation::Enum {
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::UtilityOption {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Value {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::Value::Number(field0, field1) => {
                    sqltk_parser::ast::Value::Number(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::SingleQuotedString(field0) => {
                    sqltk_parser::ast::Value::SingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::DollarQuotedString(field0) => {
                    sqltk_parser::ast::Value::DollarQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::TripleSingleQuotedString(field0) => {
                    sqltk_parser::ast::Value::TripleSingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::TripleDoubleQuotedString(field0) => {
                    sqltk_parser::ast::Value::TripleDoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::EscapedStringLiteral(field0) => {
                    sqltk_parser::ast::Value::EscapedStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::UnicodeStringLiteral(field0) => {
                    sqltk_parser::ast::Value::UnicodeStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::SingleQuotedByteStringLiteral(field0) => {
                    sqltk_parser::ast::Value::SingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::DoubleQuotedByteStringLiteral(field0) => {
                    sqltk_parser::ast::Value::DoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::TripleSingleQuotedByteStringLiteral(field0) => {
                    sqltk_parser::ast::Value::TripleSingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::TripleDoubleQuotedByteStringLiteral(field0) => {
                    sqltk_parser::ast::Value::TripleDoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::SingleQuotedRawStringLiteral(field0) => {
                    sqltk_parser::ast::Value::SingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::DoubleQuotedRawStringLiteral(field0) => {
                    sqltk_parser::ast::Value::DoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::TripleSingleQuotedRawStringLiteral(field0) => {
                    sqltk_parser::ast::Value::TripleSingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::TripleDoubleQuotedRawStringLiteral(field0) => {
                    sqltk_parser::ast::Value::TripleDoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::NationalStringLiteral(field0) => {
                    sqltk_parser::ast::Value::NationalStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::HexStringLiteral(field0) => {
                    sqltk_parser::ast::Value::HexStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::DoubleQuotedString(field0) => {
                    sqltk_parser::ast::Value::DoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::Boolean(field0) => {
                    sqltk_parser::ast::Value::Boolean(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::Value::Null => {
                    transformer.transform(node_path, sqltk_parser::ast::Value::Null)?
                }
                sqltk_parser::ast::Value::Placeholder(field0) => {
                    sqltk_parser::ast::Value::Placeholder(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ValueTableMode {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::ValueTableMode::AsStruct => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ValueTableMode::AsStruct,
                        )?
                }
                sqltk_parser::ast::ValueTableMode::AsValue => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::ValueTableMode::AsValue,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::Values {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::ViewColumnDef {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WildcardAdditionalOptions {
    fn apply_transform_with_path<T>(
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
                wildcard_token,
                opt_ilike,
                opt_exclude,
                opt_except,
                opt_replace,
                opt_rename,
            } = self;
            Self {
                wildcard_token: wildcard_token
                    .apply_transform_with_path(transformer, node_path)?,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WindowFrame {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WindowFrameBound {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::WindowFrameBound::CurrentRow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::WindowFrameBound::CurrentRow,
                        )?
                }
                sqltk_parser::ast::WindowFrameBound::Preceding(field0) => {
                    sqltk_parser::ast::WindowFrameBound::Preceding(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::WindowFrameBound::Following(field0) => {
                    sqltk_parser::ast::WindowFrameBound::Following(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WindowFrameUnits {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::WindowFrameUnits::Rows => {
                    transformer
                        .transform(node_path, sqltk_parser::ast::WindowFrameUnits::Rows)?
                }
                sqltk_parser::ast::WindowFrameUnits::Range => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::WindowFrameUnits::Range,
                        )?
                }
                sqltk_parser::ast::WindowFrameUnits::Groups => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::WindowFrameUnits::Groups,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WindowSpec {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WindowType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::WindowType::WindowSpec(field0) => {
                    sqltk_parser::ast::WindowType::WindowSpec(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::ast::WindowType::NamedWindow(field0) => {
                    sqltk_parser::ast::WindowType::NamedWindow(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::With {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { with_token, recursive, cte_tables } = self;
            Self {
                with_token: with_token
                    .apply_transform_with_path(transformer, node_path)?,
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::ast::WithFill {
    fn apply_transform_with_path<T>(
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
for sqltk_parser::ast::helpers::attached_token::AttachedToken {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast>
for sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOption {
    fn apply_transform_with_path<T>(
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
for sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType::STRING,
                        )?
                }
                sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType::BOOLEAN,
                        )?
                }
                sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptionType::ENUM,
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
for sqltk_parser::ast::helpers::stmt_data_loading::DataLoadingOptions {
    fn apply_transform_with_path<T>(
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
for sqltk_parser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
    fn apply_transform_with_path<T>(
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
for sqltk_parser::ast::helpers::stmt_data_loading::StageParamsObject {
    fn apply_transform_with_path<T>(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::keywords::Keyword {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::keywords::Keyword::NoKeyword => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NoKeyword,
                        )?
                }
                sqltk_parser::keywords::Keyword::ABORT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ABORT)?
                }
                sqltk_parser::keywords::Keyword::ABS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ABS)?
                }
                sqltk_parser::keywords::Keyword::ABSENT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ABSENT)?
                }
                sqltk_parser::keywords::Keyword::ABSOLUTE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ABSOLUTE)?
                }
                sqltk_parser::keywords::Keyword::ACCESS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ACCESS)?
                }
                sqltk_parser::keywords::Keyword::ACCOUNT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ACCOUNT)?
                }
                sqltk_parser::keywords::Keyword::ACTION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ACTION)?
                }
                sqltk_parser::keywords::Keyword::ADD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ADD)?
                }
                sqltk_parser::keywords::Keyword::ADMIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ADMIN)?
                }
                sqltk_parser::keywords::Keyword::AFTER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AFTER)?
                }
                sqltk_parser::keywords::Keyword::AGAINST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AGAINST)?
                }
                sqltk_parser::keywords::Keyword::AGGREGATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::AGGREGATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::ALIAS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ALIAS)?
                }
                sqltk_parser::keywords::Keyword::ALL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ALL)?
                }
                sqltk_parser::keywords::Keyword::ALLOCATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ALLOCATE)?
                }
                sqltk_parser::keywords::Keyword::ALTER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ALTER)?
                }
                sqltk_parser::keywords::Keyword::ALWAYS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ALWAYS)?
                }
                sqltk_parser::keywords::Keyword::ANALYZE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ANALYZE)?
                }
                sqltk_parser::keywords::Keyword::AND => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AND)?
                }
                sqltk_parser::keywords::Keyword::ANTI => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ANTI)?
                }
                sqltk_parser::keywords::Keyword::ANY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ANY)?
                }
                sqltk_parser::keywords::Keyword::APPLICATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::APPLICATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::APPLY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::APPLY)?
                }
                sqltk_parser::keywords::Keyword::ARCHIVE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ARCHIVE)?
                }
                sqltk_parser::keywords::Keyword::ARE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ARE)?
                }
                sqltk_parser::keywords::Keyword::ARRAY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ARRAY)?
                }
                sqltk_parser::keywords::Keyword::ARRAY_MAX_CARDINALITY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ARRAY_MAX_CARDINALITY,
                        )?
                }
                sqltk_parser::keywords::Keyword::AS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AS)?
                }
                sqltk_parser::keywords::Keyword::ASC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ASC)?
                }
                sqltk_parser::keywords::Keyword::ASENSITIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ASENSITIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::ASOF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ASOF)?
                }
                sqltk_parser::keywords::Keyword::ASSERT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ASSERT)?
                }
                sqltk_parser::keywords::Keyword::ASYMMETRIC => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ASYMMETRIC,
                        )?
                }
                sqltk_parser::keywords::Keyword::AT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AT)?
                }
                sqltk_parser::keywords::Keyword::ATOMIC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ATOMIC)?
                }
                sqltk_parser::keywords::Keyword::ATTACH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ATTACH)?
                }
                sqltk_parser::keywords::Keyword::AUTHORIZATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::AUTHORIZATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::AUTO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AUTO)?
                }
                sqltk_parser::keywords::Keyword::AUTOINCREMENT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::AUTOINCREMENT,
                        )?
                }
                sqltk_parser::keywords::Keyword::AUTO_INCREMENT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::AUTO_INCREMENT,
                        )?
                }
                sqltk_parser::keywords::Keyword::AVG => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AVG)?
                }
                sqltk_parser::keywords::Keyword::AVRO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::AVRO)?
                }
                sqltk_parser::keywords::Keyword::BACKWARD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BACKWARD)?
                }
                sqltk_parser::keywords::Keyword::BASE64 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BASE64)?
                }
                sqltk_parser::keywords::Keyword::BEFORE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BEFORE)?
                }
                sqltk_parser::keywords::Keyword::BEGIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BEGIN)?
                }
                sqltk_parser::keywords::Keyword::BEGIN_FRAME => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::BEGIN_FRAME,
                        )?
                }
                sqltk_parser::keywords::Keyword::BEGIN_PARTITION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::BEGIN_PARTITION,
                        )?
                }
                sqltk_parser::keywords::Keyword::BETWEEN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BETWEEN)?
                }
                sqltk_parser::keywords::Keyword::BIGDECIMAL => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::BIGDECIMAL,
                        )?
                }
                sqltk_parser::keywords::Keyword::BIGINT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BIGINT)?
                }
                sqltk_parser::keywords::Keyword::BIGNUMERIC => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::BIGNUMERIC,
                        )?
                }
                sqltk_parser::keywords::Keyword::BINARY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BINARY)?
                }
                sqltk_parser::keywords::Keyword::BINDING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BINDING)?
                }
                sqltk_parser::keywords::Keyword::BIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BIT)?
                }
                sqltk_parser::keywords::Keyword::BLOB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BLOB)?
                }
                sqltk_parser::keywords::Keyword::BLOOMFILTER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::BLOOMFILTER,
                        )?
                }
                sqltk_parser::keywords::Keyword::BOOL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BOOL)?
                }
                sqltk_parser::keywords::Keyword::BOOLEAN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BOOLEAN)?
                }
                sqltk_parser::keywords::Keyword::BOTH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BOTH)?
                }
                sqltk_parser::keywords::Keyword::BROWSE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BROWSE)?
                }
                sqltk_parser::keywords::Keyword::BTREE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BTREE)?
                }
                sqltk_parser::keywords::Keyword::BUCKETS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BUCKETS)?
                }
                sqltk_parser::keywords::Keyword::BY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BY)?
                }
                sqltk_parser::keywords::Keyword::BYPASSRLS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::BYPASSRLS,
                        )?
                }
                sqltk_parser::keywords::Keyword::BYTEA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BYTEA)?
                }
                sqltk_parser::keywords::Keyword::BYTES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::BYTES)?
                }
                sqltk_parser::keywords::Keyword::CACHE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CACHE)?
                }
                sqltk_parser::keywords::Keyword::CALL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CALL)?
                }
                sqltk_parser::keywords::Keyword::CALLED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CALLED)?
                }
                sqltk_parser::keywords::Keyword::CARDINALITY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CARDINALITY,
                        )?
                }
                sqltk_parser::keywords::Keyword::CASCADE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CASCADE)?
                }
                sqltk_parser::keywords::Keyword::CASCADED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CASCADED)?
                }
                sqltk_parser::keywords::Keyword::CASE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CASE)?
                }
                sqltk_parser::keywords::Keyword::CAST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CAST)?
                }
                sqltk_parser::keywords::Keyword::CATALOG => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CATALOG)?
                }
                sqltk_parser::keywords::Keyword::CEIL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CEIL)?
                }
                sqltk_parser::keywords::Keyword::CEILING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CEILING)?
                }
                sqltk_parser::keywords::Keyword::CENTURY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CENTURY)?
                }
                sqltk_parser::keywords::Keyword::CHAIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CHAIN)?
                }
                sqltk_parser::keywords::Keyword::CHANGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CHANGE)?
                }
                sqltk_parser::keywords::Keyword::CHANGE_TRACKING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CHANGE_TRACKING,
                        )?
                }
                sqltk_parser::keywords::Keyword::CHANNEL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CHANNEL)?
                }
                sqltk_parser::keywords::Keyword::CHAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CHAR)?
                }
                sqltk_parser::keywords::Keyword::CHARACTER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CHARACTER,
                        )?
                }
                sqltk_parser::keywords::Keyword::CHARACTERS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CHARACTERS,
                        )?
                }
                sqltk_parser::keywords::Keyword::CHARACTER_LENGTH => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CHARACTER_LENGTH,
                        )?
                }
                sqltk_parser::keywords::Keyword::CHARSET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CHARSET)?
                }
                sqltk_parser::keywords::Keyword::CHAR_LENGTH => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CHAR_LENGTH,
                        )?
                }
                sqltk_parser::keywords::Keyword::CHECK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CHECK)?
                }
                sqltk_parser::keywords::Keyword::CLEAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CLEAR)?
                }
                sqltk_parser::keywords::Keyword::CLOB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CLOB)?
                }
                sqltk_parser::keywords::Keyword::CLONE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CLONE)?
                }
                sqltk_parser::keywords::Keyword::CLOSE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CLOSE)?
                }
                sqltk_parser::keywords::Keyword::CLUSTER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CLUSTER)?
                }
                sqltk_parser::keywords::Keyword::CLUSTERED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CLUSTERED,
                        )?
                }
                sqltk_parser::keywords::Keyword::CLUSTERING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CLUSTERING,
                        )?
                }
                sqltk_parser::keywords::Keyword::COALESCE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COALESCE)?
                }
                sqltk_parser::keywords::Keyword::COLLATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COLLATE)?
                }
                sqltk_parser::keywords::Keyword::COLLATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COLLATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::COLLECT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COLLECT)?
                }
                sqltk_parser::keywords::Keyword::COLLECTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COLLECTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::COLUMN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COLUMN)?
                }
                sqltk_parser::keywords::Keyword::COLUMNS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COLUMNS)?
                }
                sqltk_parser::keywords::Keyword::COLUMNSTORE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COLUMNSTORE,
                        )?
                }
                sqltk_parser::keywords::Keyword::COMMENT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COMMENT)?
                }
                sqltk_parser::keywords::Keyword::COMMIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COMMIT)?
                }
                sqltk_parser::keywords::Keyword::COMMITTED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COMMITTED,
                        )?
                }
                sqltk_parser::keywords::Keyword::COMPRESSION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COMPRESSION,
                        )?
                }
                sqltk_parser::keywords::Keyword::COMPUTE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COMPUTE)?
                }
                sqltk_parser::keywords::Keyword::CONCURRENTLY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CONCURRENTLY,
                        )?
                }
                sqltk_parser::keywords::Keyword::CONDITION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CONDITION,
                        )?
                }
                sqltk_parser::keywords::Keyword::CONFLICT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CONFLICT)?
                }
                sqltk_parser::keywords::Keyword::CONNECT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CONNECT)?
                }
                sqltk_parser::keywords::Keyword::CONNECTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CONNECTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::CONSTRAINT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CONSTRAINT,
                        )?
                }
                sqltk_parser::keywords::Keyword::CONTAINS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CONTAINS)?
                }
                sqltk_parser::keywords::Keyword::CONTINUE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CONTINUE)?
                }
                sqltk_parser::keywords::Keyword::CONVERT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CONVERT)?
                }
                sqltk_parser::keywords::Keyword::COPY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COPY)?
                }
                sqltk_parser::keywords::Keyword::COPY_OPTIONS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COPY_OPTIONS,
                        )?
                }
                sqltk_parser::keywords::Keyword::CORR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CORR)?
                }
                sqltk_parser::keywords::Keyword::CORRESPONDING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CORRESPONDING,
                        )?
                }
                sqltk_parser::keywords::Keyword::COUNT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::COUNT)?
                }
                sqltk_parser::keywords::Keyword::COVAR_POP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COVAR_POP,
                        )?
                }
                sqltk_parser::keywords::Keyword::COVAR_SAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::COVAR_SAMP,
                        )?
                }
                sqltk_parser::keywords::Keyword::CREATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CREATE)?
                }
                sqltk_parser::keywords::Keyword::CREATEDB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CREATEDB)?
                }
                sqltk_parser::keywords::Keyword::CREATEROLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CREATEROLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::CREDENTIALS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CREDENTIALS,
                        )?
                }
                sqltk_parser::keywords::Keyword::CROSS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CROSS)?
                }
                sqltk_parser::keywords::Keyword::CSV => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CSV)?
                }
                sqltk_parser::keywords::Keyword::CUBE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CUBE)?
                }
                sqltk_parser::keywords::Keyword::CUME_DIST => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CUME_DIST,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CURRENT)?
                }
                sqltk_parser::keywords::Keyword::CURRENT_CATALOG => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_CATALOG,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_DATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_DATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_PATH => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_PATH,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_ROLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_ROLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_ROW => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_ROW,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_SCHEMA => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_SCHEMA,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_TIME => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_TIME,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_TIMESTAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_TIMESTAMP,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURRENT_USER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::CURRENT_USER,
                        )?
                }
                sqltk_parser::keywords::Keyword::CURSOR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CURSOR)?
                }
                sqltk_parser::keywords::Keyword::CYCLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::CYCLE)?
                }
                sqltk_parser::keywords::Keyword::DATA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DATA)?
                }
                sqltk_parser::keywords::Keyword::DATABASE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DATABASE)?
                }
                sqltk_parser::keywords::Keyword::DATABASES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DATABASES,
                        )?
                }
                sqltk_parser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS,
                        )?
                }
                sqltk_parser::keywords::Keyword::DATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DATE)?
                }
                sqltk_parser::keywords::Keyword::DATE32 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DATE32)?
                }
                sqltk_parser::keywords::Keyword::DATETIME => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DATETIME)?
                }
                sqltk_parser::keywords::Keyword::DATETIME64 => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DATETIME64,
                        )?
                }
                sqltk_parser::keywords::Keyword::DAY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DAY)?
                }
                sqltk_parser::keywords::Keyword::DAYOFWEEK => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DAYOFWEEK,
                        )?
                }
                sqltk_parser::keywords::Keyword::DAYOFYEAR => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DAYOFYEAR,
                        )?
                }
                sqltk_parser::keywords::Keyword::DEALLOCATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DEALLOCATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::DEC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DEC)?
                }
                sqltk_parser::keywords::Keyword::DECADE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DECADE)?
                }
                sqltk_parser::keywords::Keyword::DECIMAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DECIMAL)?
                }
                sqltk_parser::keywords::Keyword::DECLARE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DECLARE)?
                }
                sqltk_parser::keywords::Keyword::DEDUPLICATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DEDUPLICATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::DEFAULT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DEFAULT)?
                }
                sqltk_parser::keywords::Keyword::DEFAULT_DDL_COLLATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DEFAULT_DDL_COLLATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::DEFERRABLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DEFERRABLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::DEFERRED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DEFERRED)?
                }
                sqltk_parser::keywords::Keyword::DEFINE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DEFINE)?
                }
                sqltk_parser::keywords::Keyword::DEFINED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DEFINED)?
                }
                sqltk_parser::keywords::Keyword::DELAYED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DELAYED)?
                }
                sqltk_parser::keywords::Keyword::DELETE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DELETE)?
                }
                sqltk_parser::keywords::Keyword::DELIMITED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DELIMITED,
                        )?
                }
                sqltk_parser::keywords::Keyword::DELIMITER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DELIMITER,
                        )?
                }
                sqltk_parser::keywords::Keyword::DELTA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DELTA)?
                }
                sqltk_parser::keywords::Keyword::DENSE_RANK => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DENSE_RANK,
                        )?
                }
                sqltk_parser::keywords::Keyword::DEREF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DEREF)?
                }
                sqltk_parser::keywords::Keyword::DESC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DESC)?
                }
                sqltk_parser::keywords::Keyword::DESCRIBE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DESCRIBE)?
                }
                sqltk_parser::keywords::Keyword::DETACH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DETACH)?
                }
                sqltk_parser::keywords::Keyword::DETAIL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DETAIL)?
                }
                sqltk_parser::keywords::Keyword::DETERMINISTIC => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DETERMINISTIC,
                        )?
                }
                sqltk_parser::keywords::Keyword::DIRECTORY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DIRECTORY,
                        )?
                }
                sqltk_parser::keywords::Keyword::DISABLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DISABLE)?
                }
                sqltk_parser::keywords::Keyword::DISCARD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DISCARD)?
                }
                sqltk_parser::keywords::Keyword::DISCONNECT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DISCONNECT,
                        )?
                }
                sqltk_parser::keywords::Keyword::DISTINCT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DISTINCT)?
                }
                sqltk_parser::keywords::Keyword::DISTRIBUTE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DISTRIBUTE,
                        )?
                }
                sqltk_parser::keywords::Keyword::DIV => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DIV)?
                }
                sqltk_parser::keywords::Keyword::DO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DO)?
                }
                sqltk_parser::keywords::Keyword::DOUBLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DOUBLE)?
                }
                sqltk_parser::keywords::Keyword::DOW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DOW)?
                }
                sqltk_parser::keywords::Keyword::DOY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DOY)?
                }
                sqltk_parser::keywords::Keyword::DROP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DROP)?
                }
                sqltk_parser::keywords::Keyword::DRY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DRY)?
                }
                sqltk_parser::keywords::Keyword::DUPLICATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::DUPLICATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::DYNAMIC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::DYNAMIC)?
                }
                sqltk_parser::keywords::Keyword::EACH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EACH)?
                }
                sqltk_parser::keywords::Keyword::ELEMENT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ELEMENT)?
                }
                sqltk_parser::keywords::Keyword::ELEMENTS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ELEMENTS)?
                }
                sqltk_parser::keywords::Keyword::ELSE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ELSE)?
                }
                sqltk_parser::keywords::Keyword::EMPTY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EMPTY)?
                }
                sqltk_parser::keywords::Keyword::ENABLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENABLE)?
                }
                sqltk_parser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::ENCODING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENCODING)?
                }
                sqltk_parser::keywords::Keyword::ENCRYPTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ENCRYPTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::END => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::END)?
                }
                sqltk_parser::keywords::Keyword::END_EXEC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::END_EXEC)?
                }
                sqltk_parser::keywords::Keyword::ENDPOINT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENDPOINT)?
                }
                sqltk_parser::keywords::Keyword::END_FRAME => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::END_FRAME,
                        )?
                }
                sqltk_parser::keywords::Keyword::END_PARTITION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::END_PARTITION,
                        )?
                }
                sqltk_parser::keywords::Keyword::ENFORCED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENFORCED)?
                }
                sqltk_parser::keywords::Keyword::ENGINE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENGINE)?
                }
                sqltk_parser::keywords::Keyword::ENUM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENUM)?
                }
                sqltk_parser::keywords::Keyword::ENUM16 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENUM16)?
                }
                sqltk_parser::keywords::Keyword::ENUM8 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ENUM8)?
                }
                sqltk_parser::keywords::Keyword::EPHEMERAL => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::EPHEMERAL,
                        )?
                }
                sqltk_parser::keywords::Keyword::EPOCH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EPOCH)?
                }
                sqltk_parser::keywords::Keyword::EQUALS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EQUALS)?
                }
                sqltk_parser::keywords::Keyword::ERROR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ERROR)?
                }
                sqltk_parser::keywords::Keyword::ESCAPE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ESCAPE)?
                }
                sqltk_parser::keywords::Keyword::ESCAPED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ESCAPED)?
                }
                sqltk_parser::keywords::Keyword::EVENT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EVENT)?
                }
                sqltk_parser::keywords::Keyword::EVERY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EVERY)?
                }
                sqltk_parser::keywords::Keyword::EXCEPT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXCEPT)?
                }
                sqltk_parser::keywords::Keyword::EXCEPTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::EXCEPTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::EXCLUDE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXCLUDE)?
                }
                sqltk_parser::keywords::Keyword::EXCLUSIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::EXCLUSIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::EXEC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXEC)?
                }
                sqltk_parser::keywords::Keyword::EXECUTE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXECUTE)?
                }
                sqltk_parser::keywords::Keyword::EXISTS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXISTS)?
                }
                sqltk_parser::keywords::Keyword::EXP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXP)?
                }
                sqltk_parser::keywords::Keyword::EXPANSION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::EXPANSION,
                        )?
                }
                sqltk_parser::keywords::Keyword::EXPLAIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXPLAIN)?
                }
                sqltk_parser::keywords::Keyword::EXPLICIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXPLICIT)?
                }
                sqltk_parser::keywords::Keyword::EXPORT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXPORT)?
                }
                sqltk_parser::keywords::Keyword::EXTENDED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXTENDED)?
                }
                sqltk_parser::keywords::Keyword::EXTENSION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::EXTENSION,
                        )?
                }
                sqltk_parser::keywords::Keyword::EXTERNAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXTERNAL)?
                }
                sqltk_parser::keywords::Keyword::EXTRACT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::EXTRACT)?
                }
                sqltk_parser::keywords::Keyword::FAIL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FAIL)?
                }
                sqltk_parser::keywords::Keyword::FALSE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FALSE)?
                }
                sqltk_parser::keywords::Keyword::FETCH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FETCH)?
                }
                sqltk_parser::keywords::Keyword::FIELDS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FIELDS)?
                }
                sqltk_parser::keywords::Keyword::FILE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FILE)?
                }
                sqltk_parser::keywords::Keyword::FILES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FILES)?
                }
                sqltk_parser::keywords::Keyword::FILE_FORMAT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FILE_FORMAT,
                        )?
                }
                sqltk_parser::keywords::Keyword::FILL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FILL)?
                }
                sqltk_parser::keywords::Keyword::FILTER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FILTER)?
                }
                sqltk_parser::keywords::Keyword::FINAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FINAL)?
                }
                sqltk_parser::keywords::Keyword::FIRST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FIRST)?
                }
                sqltk_parser::keywords::Keyword::FIRST_VALUE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FIRST_VALUE,
                        )?
                }
                sqltk_parser::keywords::Keyword::FIXEDSTRING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FIXEDSTRING,
                        )?
                }
                sqltk_parser::keywords::Keyword::FLOAT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLOAT)?
                }
                sqltk_parser::keywords::Keyword::FLOAT32 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLOAT32)?
                }
                sqltk_parser::keywords::Keyword::FLOAT4 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLOAT4)?
                }
                sqltk_parser::keywords::Keyword::FLOAT64 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLOAT64)?
                }
                sqltk_parser::keywords::Keyword::FLOAT8 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLOAT8)?
                }
                sqltk_parser::keywords::Keyword::FLOOR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLOOR)?
                }
                sqltk_parser::keywords::Keyword::FLUSH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FLUSH)?
                }
                sqltk_parser::keywords::Keyword::FN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FN)?
                }
                sqltk_parser::keywords::Keyword::FOLLOWING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FOLLOWING,
                        )?
                }
                sqltk_parser::keywords::Keyword::FOR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FOR)?
                }
                sqltk_parser::keywords::Keyword::FORCE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FORCE)?
                }
                sqltk_parser::keywords::Keyword::FORCE_NOT_NULL => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FORCE_NOT_NULL,
                        )?
                }
                sqltk_parser::keywords::Keyword::FORCE_NULL => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FORCE_NULL,
                        )?
                }
                sqltk_parser::keywords::Keyword::FORCE_QUOTE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FORCE_QUOTE,
                        )?
                }
                sqltk_parser::keywords::Keyword::FOREIGN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FOREIGN)?
                }
                sqltk_parser::keywords::Keyword::FORMAT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FORMAT)?
                }
                sqltk_parser::keywords::Keyword::FORMATTED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FORMATTED,
                        )?
                }
                sqltk_parser::keywords::Keyword::FORWARD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FORWARD)?
                }
                sqltk_parser::keywords::Keyword::FRAME_ROW => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FRAME_ROW,
                        )?
                }
                sqltk_parser::keywords::Keyword::FREE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FREE)?
                }
                sqltk_parser::keywords::Keyword::FREEZE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FREEZE)?
                }
                sqltk_parser::keywords::Keyword::FROM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FROM)?
                }
                sqltk_parser::keywords::Keyword::FSCK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FSCK)?
                }
                sqltk_parser::keywords::Keyword::FULL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FULL)?
                }
                sqltk_parser::keywords::Keyword::FULLTEXT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FULLTEXT)?
                }
                sqltk_parser::keywords::Keyword::FUNCTION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FUNCTION)?
                }
                sqltk_parser::keywords::Keyword::FUNCTIONS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::FUNCTIONS,
                        )?
                }
                sqltk_parser::keywords::Keyword::FUSION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::FUSION)?
                }
                sqltk_parser::keywords::Keyword::GENERAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GENERAL)?
                }
                sqltk_parser::keywords::Keyword::GENERATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GENERATE)?
                }
                sqltk_parser::keywords::Keyword::GENERATED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::GENERATED,
                        )?
                }
                sqltk_parser::keywords::Keyword::GEOGRAPHY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::GEOGRAPHY,
                        )?
                }
                sqltk_parser::keywords::Keyword::GET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GET)?
                }
                sqltk_parser::keywords::Keyword::GLOBAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GLOBAL)?
                }
                sqltk_parser::keywords::Keyword::GRANT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GRANT)?
                }
                sqltk_parser::keywords::Keyword::GRANTED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GRANTED)?
                }
                sqltk_parser::keywords::Keyword::GRANTS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GRANTS)?
                }
                sqltk_parser::keywords::Keyword::GRAPHVIZ => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GRAPHVIZ)?
                }
                sqltk_parser::keywords::Keyword::GROUP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GROUP)?
                }
                sqltk_parser::keywords::Keyword::GROUPING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GROUPING)?
                }
                sqltk_parser::keywords::Keyword::GROUPS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::GROUPS)?
                }
                sqltk_parser::keywords::Keyword::HASH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HASH)?
                }
                sqltk_parser::keywords::Keyword::HAVING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HAVING)?
                }
                sqltk_parser::keywords::Keyword::HEADER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HEADER)?
                }
                sqltk_parser::keywords::Keyword::HEAP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HEAP)?
                }
                sqltk_parser::keywords::Keyword::HIGH_PRIORITY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::HIGH_PRIORITY,
                        )?
                }
                sqltk_parser::keywords::Keyword::HISTORY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HISTORY)?
                }
                sqltk_parser::keywords::Keyword::HIVEVAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HIVEVAR)?
                }
                sqltk_parser::keywords::Keyword::HOLD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HOLD)?
                }
                sqltk_parser::keywords::Keyword::HOSTS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HOSTS)?
                }
                sqltk_parser::keywords::Keyword::HOUR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HOUR)?
                }
                sqltk_parser::keywords::Keyword::HOURS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::HOURS)?
                }
                sqltk_parser::keywords::Keyword::ID => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ID)?
                }
                sqltk_parser::keywords::Keyword::IDENTITY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::IDENTITY)?
                }
                sqltk_parser::keywords::Keyword::IF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::IF)?
                }
                sqltk_parser::keywords::Keyword::IGNORE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::IGNORE)?
                }
                sqltk_parser::keywords::Keyword::ILIKE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ILIKE)?
                }
                sqltk_parser::keywords::Keyword::IMMEDIATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::IMMEDIATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::IMMUTABLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::IMMUTABLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::IN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::IN)?
                }
                sqltk_parser::keywords::Keyword::INCLUDE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INCLUDE)?
                }
                sqltk_parser::keywords::Keyword::INCLUDE_NULL_VALUES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INCLUDE_NULL_VALUES,
                        )?
                }
                sqltk_parser::keywords::Keyword::INCREMENT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INCREMENT,
                        )?
                }
                sqltk_parser::keywords::Keyword::INDEX => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INDEX)?
                }
                sqltk_parser::keywords::Keyword::INDICATOR => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INDICATOR,
                        )?
                }
                sqltk_parser::keywords::Keyword::INHERIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INHERIT)?
                }
                sqltk_parser::keywords::Keyword::INITIALLY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INITIALLY,
                        )?
                }
                sqltk_parser::keywords::Keyword::INNER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INNER)?
                }
                sqltk_parser::keywords::Keyword::INOUT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INOUT)?
                }
                sqltk_parser::keywords::Keyword::INPATH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INPATH)?
                }
                sqltk_parser::keywords::Keyword::INPUT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INPUT)?
                }
                sqltk_parser::keywords::Keyword::INPUTFORMAT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INPUTFORMAT,
                        )?
                }
                sqltk_parser::keywords::Keyword::INSENSITIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INSENSITIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::INSERT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INSERT)?
                }
                sqltk_parser::keywords::Keyword::INSTALL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INSTALL)?
                }
                sqltk_parser::keywords::Keyword::INSTEAD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INSTEAD)?
                }
                sqltk_parser::keywords::Keyword::INT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT)?
                }
                sqltk_parser::keywords::Keyword::INT128 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT128)?
                }
                sqltk_parser::keywords::Keyword::INT16 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT16)?
                }
                sqltk_parser::keywords::Keyword::INT2 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT2)?
                }
                sqltk_parser::keywords::Keyword::INT256 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT256)?
                }
                sqltk_parser::keywords::Keyword::INT32 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT32)?
                }
                sqltk_parser::keywords::Keyword::INT4 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT4)?
                }
                sqltk_parser::keywords::Keyword::INT64 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT64)?
                }
                sqltk_parser::keywords::Keyword::INT8 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INT8)?
                }
                sqltk_parser::keywords::Keyword::INTEGER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INTEGER)?
                }
                sqltk_parser::keywords::Keyword::INTERPOLATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INTERPOLATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::INTERSECT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INTERSECT,
                        )?
                }
                sqltk_parser::keywords::Keyword::INTERSECTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::INTERSECTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::INTERVAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INTERVAL)?
                }
                sqltk_parser::keywords::Keyword::INTO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::INTO)?
                }
                sqltk_parser::keywords::Keyword::IS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::IS)?
                }
                sqltk_parser::keywords::Keyword::ISODOW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ISODOW)?
                }
                sqltk_parser::keywords::Keyword::ISOLATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ISOLATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::ISOWEEK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ISOWEEK)?
                }
                sqltk_parser::keywords::Keyword::ISOYEAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ISOYEAR)?
                }
                sqltk_parser::keywords::Keyword::ITEMS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ITEMS)?
                }
                sqltk_parser::keywords::Keyword::JAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::JAR)?
                }
                sqltk_parser::keywords::Keyword::JOIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::JOIN)?
                }
                sqltk_parser::keywords::Keyword::JSON => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::JSON)?
                }
                sqltk_parser::keywords::Keyword::JSONB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::JSONB)?
                }
                sqltk_parser::keywords::Keyword::JSONFILE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::JSONFILE)?
                }
                sqltk_parser::keywords::Keyword::JSON_TABLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::JSON_TABLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::JULIAN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::JULIAN)?
                }
                sqltk_parser::keywords::Keyword::KEY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::KEY)?
                }
                sqltk_parser::keywords::Keyword::KEYS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::KEYS)?
                }
                sqltk_parser::keywords::Keyword::KILL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::KILL)?
                }
                sqltk_parser::keywords::Keyword::LAG => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LAG)?
                }
                sqltk_parser::keywords::Keyword::LANGUAGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LANGUAGE)?
                }
                sqltk_parser::keywords::Keyword::LARGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LARGE)?
                }
                sqltk_parser::keywords::Keyword::LAST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LAST)?
                }
                sqltk_parser::keywords::Keyword::LAST_VALUE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::LAST_VALUE,
                        )?
                }
                sqltk_parser::keywords::Keyword::LATERAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LATERAL)?
                }
                sqltk_parser::keywords::Keyword::LEAD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LEAD)?
                }
                sqltk_parser::keywords::Keyword::LEADING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LEADING)?
                }
                sqltk_parser::keywords::Keyword::LEFT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LEFT)?
                }
                sqltk_parser::keywords::Keyword::LEVEL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LEVEL)?
                }
                sqltk_parser::keywords::Keyword::LIKE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LIKE)?
                }
                sqltk_parser::keywords::Keyword::LIKE_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::LIKE_REGEX,
                        )?
                }
                sqltk_parser::keywords::Keyword::LIMIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LIMIT)?
                }
                sqltk_parser::keywords::Keyword::LINES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LINES)?
                }
                sqltk_parser::keywords::Keyword::LISTEN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LISTEN)?
                }
                sqltk_parser::keywords::Keyword::LN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LN)?
                }
                sqltk_parser::keywords::Keyword::LOAD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOAD)?
                }
                sqltk_parser::keywords::Keyword::LOCAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOCAL)?
                }
                sqltk_parser::keywords::Keyword::LOCALTIME => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::LOCALTIME,
                        )?
                }
                sqltk_parser::keywords::Keyword::LOCALTIMESTAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::LOCALTIMESTAMP,
                        )?
                }
                sqltk_parser::keywords::Keyword::LOCATION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOCATION)?
                }
                sqltk_parser::keywords::Keyword::LOCK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOCK)?
                }
                sqltk_parser::keywords::Keyword::LOCKED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOCKED)?
                }
                sqltk_parser::keywords::Keyword::LOGIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOGIN)?
                }
                sqltk_parser::keywords::Keyword::LOGS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOGS)?
                }
                sqltk_parser::keywords::Keyword::LONGBLOB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LONGBLOB)?
                }
                sqltk_parser::keywords::Keyword::LONGTEXT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LONGTEXT)?
                }
                sqltk_parser::keywords::Keyword::LOWCARDINALITY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::LOWCARDINALITY,
                        )?
                }
                sqltk_parser::keywords::Keyword::LOWER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::LOWER)?
                }
                sqltk_parser::keywords::Keyword::LOW_PRIORITY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::LOW_PRIORITY,
                        )?
                }
                sqltk_parser::keywords::Keyword::MACRO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MACRO)?
                }
                sqltk_parser::keywords::Keyword::MANAGEDLOCATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MANAGEDLOCATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::MAP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MAP)?
                }
                sqltk_parser::keywords::Keyword::MASKING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MASKING)?
                }
                sqltk_parser::keywords::Keyword::MATCH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MATCH)?
                }
                sqltk_parser::keywords::Keyword::MATCHED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MATCHED)?
                }
                sqltk_parser::keywords::Keyword::MATCHES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MATCHES)?
                }
                sqltk_parser::keywords::Keyword::MATCH_CONDITION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MATCH_CONDITION,
                        )?
                }
                sqltk_parser::keywords::Keyword::MATCH_RECOGNIZE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MATCH_RECOGNIZE,
                        )?
                }
                sqltk_parser::keywords::Keyword::MATERIALIZE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MATERIALIZE,
                        )?
                }
                sqltk_parser::keywords::Keyword::MATERIALIZED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MATERIALIZED,
                        )?
                }
                sqltk_parser::keywords::Keyword::MAX => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MAX)?
                }
                sqltk_parser::keywords::Keyword::MAXVALUE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MAXVALUE)?
                }
                sqltk_parser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS,
                        )?
                }
                sqltk_parser::keywords::Keyword::MEASURES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MEASURES)?
                }
                sqltk_parser::keywords::Keyword::MEDIUMBLOB => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MEDIUMBLOB,
                        )?
                }
                sqltk_parser::keywords::Keyword::MEDIUMINT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MEDIUMINT,
                        )?
                }
                sqltk_parser::keywords::Keyword::MEDIUMTEXT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MEDIUMTEXT,
                        )?
                }
                sqltk_parser::keywords::Keyword::MEMBER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MEMBER)?
                }
                sqltk_parser::keywords::Keyword::MERGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MERGE)?
                }
                sqltk_parser::keywords::Keyword::METADATA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::METADATA)?
                }
                sqltk_parser::keywords::Keyword::METHOD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::METHOD)?
                }
                sqltk_parser::keywords::Keyword::MICROSECOND => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MICROSECOND,
                        )?
                }
                sqltk_parser::keywords::Keyword::MICROSECONDS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MICROSECONDS,
                        )?
                }
                sqltk_parser::keywords::Keyword::MILLENIUM => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MILLENIUM,
                        )?
                }
                sqltk_parser::keywords::Keyword::MILLENNIUM => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MILLENNIUM,
                        )?
                }
                sqltk_parser::keywords::Keyword::MILLISECOND => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MILLISECOND,
                        )?
                }
                sqltk_parser::keywords::Keyword::MILLISECONDS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::MILLISECONDS,
                        )?
                }
                sqltk_parser::keywords::Keyword::MIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MIN)?
                }
                sqltk_parser::keywords::Keyword::MINUTE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MINUTE)?
                }
                sqltk_parser::keywords::Keyword::MINVALUE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MINVALUE)?
                }
                sqltk_parser::keywords::Keyword::MOD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MOD)?
                }
                sqltk_parser::keywords::Keyword::MODE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MODE)?
                }
                sqltk_parser::keywords::Keyword::MODIFIES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MODIFIES)?
                }
                sqltk_parser::keywords::Keyword::MODIFY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MODIFY)?
                }
                sqltk_parser::keywords::Keyword::MODULE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MODULE)?
                }
                sqltk_parser::keywords::Keyword::MONTH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MONTH)?
                }
                sqltk_parser::keywords::Keyword::MSCK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MSCK)?
                }
                sqltk_parser::keywords::Keyword::MULTISET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MULTISET)?
                }
                sqltk_parser::keywords::Keyword::MUTATION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::MUTATION)?
                }
                sqltk_parser::keywords::Keyword::NAME => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NAME)?
                }
                sqltk_parser::keywords::Keyword::NANOSECOND => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NANOSECOND,
                        )?
                }
                sqltk_parser::keywords::Keyword::NANOSECONDS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NANOSECONDS,
                        )?
                }
                sqltk_parser::keywords::Keyword::NATIONAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NATIONAL)?
                }
                sqltk_parser::keywords::Keyword::NATURAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NATURAL)?
                }
                sqltk_parser::keywords::Keyword::NCHAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NCHAR)?
                }
                sqltk_parser::keywords::Keyword::NCLOB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NCLOB)?
                }
                sqltk_parser::keywords::Keyword::NESTED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NESTED)?
                }
                sqltk_parser::keywords::Keyword::NEW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NEW)?
                }
                sqltk_parser::keywords::Keyword::NEXT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NEXT)?
                }
                sqltk_parser::keywords::Keyword::NO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NO)?
                }
                sqltk_parser::keywords::Keyword::NOBYPASSRLS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NOBYPASSRLS,
                        )?
                }
                sqltk_parser::keywords::Keyword::NOCREATEDB => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NOCREATEDB,
                        )?
                }
                sqltk_parser::keywords::Keyword::NOCREATEROLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NOCREATEROLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::NOINHERIT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NOINHERIT,
                        )?
                }
                sqltk_parser::keywords::Keyword::NOLOGIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOLOGIN)?
                }
                sqltk_parser::keywords::Keyword::NONE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NONE)?
                }
                sqltk_parser::keywords::Keyword::NOORDER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOORDER)?
                }
                sqltk_parser::keywords::Keyword::NOREPLICATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NOREPLICATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::NORMALIZE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NORMALIZE,
                        )?
                }
                sqltk_parser::keywords::Keyword::NOSCAN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOSCAN)?
                }
                sqltk_parser::keywords::Keyword::NOSUPERUSER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NOSUPERUSER,
                        )?
                }
                sqltk_parser::keywords::Keyword::NOT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOT)?
                }
                sqltk_parser::keywords::Keyword::NOTHING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOTHING)?
                }
                sqltk_parser::keywords::Keyword::NOTIFY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOTIFY)?
                }
                sqltk_parser::keywords::Keyword::NOWAIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NOWAIT)?
                }
                sqltk_parser::keywords::Keyword::NO_WRITE_TO_BINLOG => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NO_WRITE_TO_BINLOG,
                        )?
                }
                sqltk_parser::keywords::Keyword::NTH_VALUE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::NTH_VALUE,
                        )?
                }
                sqltk_parser::keywords::Keyword::NTILE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NTILE)?
                }
                sqltk_parser::keywords::Keyword::NULL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NULL)?
                }
                sqltk_parser::keywords::Keyword::NULLABLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NULLABLE)?
                }
                sqltk_parser::keywords::Keyword::NULLIF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NULLIF)?
                }
                sqltk_parser::keywords::Keyword::NULLS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NULLS)?
                }
                sqltk_parser::keywords::Keyword::NUMERIC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NUMERIC)?
                }
                sqltk_parser::keywords::Keyword::NVARCHAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::NVARCHAR)?
                }
                sqltk_parser::keywords::Keyword::OBJECT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OBJECT)?
                }
                sqltk_parser::keywords::Keyword::OCCURRENCES_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::OCCURRENCES_REGEX,
                        )?
                }
                sqltk_parser::keywords::Keyword::OCTETS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OCTETS)?
                }
                sqltk_parser::keywords::Keyword::OCTET_LENGTH => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::OCTET_LENGTH,
                        )?
                }
                sqltk_parser::keywords::Keyword::OF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OF)?
                }
                sqltk_parser::keywords::Keyword::OFFSET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OFFSET)?
                }
                sqltk_parser::keywords::Keyword::OLD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OLD)?
                }
                sqltk_parser::keywords::Keyword::OMIT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OMIT)?
                }
                sqltk_parser::keywords::Keyword::ON => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ON)?
                }
                sqltk_parser::keywords::Keyword::ONE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ONE)?
                }
                sqltk_parser::keywords::Keyword::ONLY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ONLY)?
                }
                sqltk_parser::keywords::Keyword::OPEN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OPEN)?
                }
                sqltk_parser::keywords::Keyword::OPENJSON => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OPENJSON)?
                }
                sqltk_parser::keywords::Keyword::OPERATOR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OPERATOR)?
                }
                sqltk_parser::keywords::Keyword::OPTIMIZE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OPTIMIZE)?
                }
                sqltk_parser::keywords::Keyword::OPTIMIZER_COSTS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::OPTIMIZER_COSTS,
                        )?
                }
                sqltk_parser::keywords::Keyword::OPTION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OPTION)?
                }
                sqltk_parser::keywords::Keyword::OPTIONS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OPTIONS)?
                }
                sqltk_parser::keywords::Keyword::OR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OR)?
                }
                sqltk_parser::keywords::Keyword::ORC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ORC)?
                }
                sqltk_parser::keywords::Keyword::ORDER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ORDER)?
                }
                sqltk_parser::keywords::Keyword::ORDINALITY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ORDINALITY,
                        )?
                }
                sqltk_parser::keywords::Keyword::OUT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OUT)?
                }
                sqltk_parser::keywords::Keyword::OUTER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OUTER)?
                }
                sqltk_parser::keywords::Keyword::OUTPUTFORMAT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::OUTPUTFORMAT,
                        )?
                }
                sqltk_parser::keywords::Keyword::OVER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OVER)?
                }
                sqltk_parser::keywords::Keyword::OVERFLOW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OVERFLOW)?
                }
                sqltk_parser::keywords::Keyword::OVERLAPS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OVERLAPS)?
                }
                sqltk_parser::keywords::Keyword::OVERLAY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OVERLAY)?
                }
                sqltk_parser::keywords::Keyword::OVERWRITE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::OVERWRITE,
                        )?
                }
                sqltk_parser::keywords::Keyword::OWNED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OWNED)?
                }
                sqltk_parser::keywords::Keyword::OWNER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::OWNER)?
                }
                sqltk_parser::keywords::Keyword::PARALLEL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PARALLEL)?
                }
                sqltk_parser::keywords::Keyword::PARAMETER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PARAMETER,
                        )?
                }
                sqltk_parser::keywords::Keyword::PARQUET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PARQUET)?
                }
                sqltk_parser::keywords::Keyword::PART => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PART)?
                }
                sqltk_parser::keywords::Keyword::PARTITION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PARTITION,
                        )?
                }
                sqltk_parser::keywords::Keyword::PARTITIONED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PARTITIONED,
                        )?
                }
                sqltk_parser::keywords::Keyword::PARTITIONS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PARTITIONS,
                        )?
                }
                sqltk_parser::keywords::Keyword::PASSWORD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PASSWORD)?
                }
                sqltk_parser::keywords::Keyword::PAST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PAST)?
                }
                sqltk_parser::keywords::Keyword::PATH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PATH)?
                }
                sqltk_parser::keywords::Keyword::PATTERN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PATTERN)?
                }
                sqltk_parser::keywords::Keyword::PER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PER)?
                }
                sqltk_parser::keywords::Keyword::PERCENT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PERCENT)?
                }
                sqltk_parser::keywords::Keyword::PERCENTILE_CONT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PERCENTILE_CONT,
                        )?
                }
                sqltk_parser::keywords::Keyword::PERCENTILE_DISC => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PERCENTILE_DISC,
                        )?
                }
                sqltk_parser::keywords::Keyword::PERCENT_RANK => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PERCENT_RANK,
                        )?
                }
                sqltk_parser::keywords::Keyword::PERIOD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PERIOD)?
                }
                sqltk_parser::keywords::Keyword::PERMISSIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PERMISSIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::PERSISTENT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PERSISTENT,
                        )?
                }
                sqltk_parser::keywords::Keyword::PIVOT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PIVOT)?
                }
                sqltk_parser::keywords::Keyword::PLACING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PLACING)?
                }
                sqltk_parser::keywords::Keyword::PLAN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PLAN)?
                }
                sqltk_parser::keywords::Keyword::PLANS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PLANS)?
                }
                sqltk_parser::keywords::Keyword::POLICY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::POLICY)?
                }
                sqltk_parser::keywords::Keyword::PORTION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PORTION)?
                }
                sqltk_parser::keywords::Keyword::POSITION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::POSITION)?
                }
                sqltk_parser::keywords::Keyword::POSITION_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::POSITION_REGEX,
                        )?
                }
                sqltk_parser::keywords::Keyword::POWER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::POWER)?
                }
                sqltk_parser::keywords::Keyword::PRAGMA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PRAGMA)?
                }
                sqltk_parser::keywords::Keyword::PRECEDES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PRECEDES)?
                }
                sqltk_parser::keywords::Keyword::PRECEDING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PRECEDING,
                        )?
                }
                sqltk_parser::keywords::Keyword::PRECISION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PRECISION,
                        )?
                }
                sqltk_parser::keywords::Keyword::PREPARE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PREPARE)?
                }
                sqltk_parser::keywords::Keyword::PRESERVE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PRESERVE)?
                }
                sqltk_parser::keywords::Keyword::PREWHERE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PREWHERE)?
                }
                sqltk_parser::keywords::Keyword::PRIMARY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PRIMARY)?
                }
                sqltk_parser::keywords::Keyword::PRIOR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PRIOR)?
                }
                sqltk_parser::keywords::Keyword::PRIVILEGES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PRIVILEGES,
                        )?
                }
                sqltk_parser::keywords::Keyword::PROCEDURE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PROCEDURE,
                        )?
                }
                sqltk_parser::keywords::Keyword::PROGRAM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PROGRAM)?
                }
                sqltk_parser::keywords::Keyword::PROJECTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::PROJECTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::PURGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::PURGE)?
                }
                sqltk_parser::keywords::Keyword::QUALIFY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::QUALIFY)?
                }
                sqltk_parser::keywords::Keyword::QUARTER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::QUARTER)?
                }
                sqltk_parser::keywords::Keyword::QUERY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::QUERY)?
                }
                sqltk_parser::keywords::Keyword::QUOTE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::QUOTE)?
                }
                sqltk_parser::keywords::Keyword::RANGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RANGE)?
                }
                sqltk_parser::keywords::Keyword::RANK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RANK)?
                }
                sqltk_parser::keywords::Keyword::RAW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RAW)?
                }
                sqltk_parser::keywords::Keyword::RCFILE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RCFILE)?
                }
                sqltk_parser::keywords::Keyword::READ => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::READ)?
                }
                sqltk_parser::keywords::Keyword::READS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::READS)?
                }
                sqltk_parser::keywords::Keyword::READ_ONLY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::READ_ONLY,
                        )?
                }
                sqltk_parser::keywords::Keyword::REAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REAL)?
                }
                sqltk_parser::keywords::Keyword::RECLUSTER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::RECLUSTER,
                        )?
                }
                sqltk_parser::keywords::Keyword::RECURSIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::RECURSIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::REF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REF)?
                }
                sqltk_parser::keywords::Keyword::REFERENCES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REFERENCES,
                        )?
                }
                sqltk_parser::keywords::Keyword::REFERENCING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REFERENCING,
                        )?
                }
                sqltk_parser::keywords::Keyword::REGCLASS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REGCLASS)?
                }
                sqltk_parser::keywords::Keyword::REGEXP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REGEXP)?
                }
                sqltk_parser::keywords::Keyword::REGR_AVGX => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REGR_AVGX,
                        )?
                }
                sqltk_parser::keywords::Keyword::REGR_AVGY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REGR_AVGY,
                        )?
                }
                sqltk_parser::keywords::Keyword::REGR_COUNT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REGR_COUNT,
                        )?
                }
                sqltk_parser::keywords::Keyword::REGR_INTERCEPT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REGR_INTERCEPT,
                        )?
                }
                sqltk_parser::keywords::Keyword::REGR_R2 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REGR_R2)?
                }
                sqltk_parser::keywords::Keyword::REGR_SLOPE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REGR_SLOPE,
                        )?
                }
                sqltk_parser::keywords::Keyword::REGR_SXX => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REGR_SXX)?
                }
                sqltk_parser::keywords::Keyword::REGR_SXY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REGR_SXY)?
                }
                sqltk_parser::keywords::Keyword::REGR_SYY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REGR_SYY)?
                }
                sqltk_parser::keywords::Keyword::RELATIVE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RELATIVE)?
                }
                sqltk_parser::keywords::Keyword::RELAY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RELAY)?
                }
                sqltk_parser::keywords::Keyword::RELEASE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RELEASE)?
                }
                sqltk_parser::keywords::Keyword::REMOTE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REMOTE)?
                }
                sqltk_parser::keywords::Keyword::RENAME => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RENAME)?
                }
                sqltk_parser::keywords::Keyword::REORG => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REORG)?
                }
                sqltk_parser::keywords::Keyword::REPAIR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REPAIR)?
                }
                sqltk_parser::keywords::Keyword::REPEATABLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REPEATABLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::REPLACE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REPLACE)?
                }
                sqltk_parser::keywords::Keyword::REPLICA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REPLICA)?
                }
                sqltk_parser::keywords::Keyword::REPLICATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::REPLICATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::RESET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RESET)?
                }
                sqltk_parser::keywords::Keyword::RESPECT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RESPECT)?
                }
                sqltk_parser::keywords::Keyword::RESTART => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RESTART)?
                }
                sqltk_parser::keywords::Keyword::RESTRICT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RESTRICT)?
                }
                sqltk_parser::keywords::Keyword::RESTRICTED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::RESTRICTED,
                        )?
                }
                sqltk_parser::keywords::Keyword::RESTRICTIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::RESTRICTIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::RESULT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RESULT)?
                }
                sqltk_parser::keywords::Keyword::RESULTSET => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::RESULTSET,
                        )?
                }
                sqltk_parser::keywords::Keyword::RESUME => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RESUME)?
                }
                sqltk_parser::keywords::Keyword::RETAIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RETAIN)?
                }
                sqltk_parser::keywords::Keyword::RETURN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RETURN)?
                }
                sqltk_parser::keywords::Keyword::RETURNING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::RETURNING,
                        )?
                }
                sqltk_parser::keywords::Keyword::RETURNS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RETURNS)?
                }
                sqltk_parser::keywords::Keyword::REVOKE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::REVOKE)?
                }
                sqltk_parser::keywords::Keyword::RIGHT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RIGHT)?
                }
                sqltk_parser::keywords::Keyword::RLIKE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RLIKE)?
                }
                sqltk_parser::keywords::Keyword::ROLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROLE)?
                }
                sqltk_parser::keywords::Keyword::ROLES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROLES)?
                }
                sqltk_parser::keywords::Keyword::ROLLBACK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROLLBACK)?
                }
                sqltk_parser::keywords::Keyword::ROLLUP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROLLUP)?
                }
                sqltk_parser::keywords::Keyword::ROOT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROOT)?
                }
                sqltk_parser::keywords::Keyword::ROW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROW)?
                }
                sqltk_parser::keywords::Keyword::ROWID => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROWID)?
                }
                sqltk_parser::keywords::Keyword::ROWS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ROWS)?
                }
                sqltk_parser::keywords::Keyword::ROW_NUMBER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::ROW_NUMBER,
                        )?
                }
                sqltk_parser::keywords::Keyword::RULE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RULE)?
                }
                sqltk_parser::keywords::Keyword::RUN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::RUN)?
                }
                sqltk_parser::keywords::Keyword::SAFE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SAFE)?
                }
                sqltk_parser::keywords::Keyword::SAFE_CAST => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SAFE_CAST,
                        )?
                }
                sqltk_parser::keywords::Keyword::SAVEPOINT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SAVEPOINT,
                        )?
                }
                sqltk_parser::keywords::Keyword::SCHEMA => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SCHEMA)?
                }
                sqltk_parser::keywords::Keyword::SCHEMAS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SCHEMAS)?
                }
                sqltk_parser::keywords::Keyword::SCOPE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SCOPE)?
                }
                sqltk_parser::keywords::Keyword::SCROLL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SCROLL)?
                }
                sqltk_parser::keywords::Keyword::SEARCH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SEARCH)?
                }
                sqltk_parser::keywords::Keyword::SECOND => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SECOND)?
                }
                sqltk_parser::keywords::Keyword::SECONDARY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SECONDARY,
                        )?
                }
                sqltk_parser::keywords::Keyword::SECRET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SECRET)?
                }
                sqltk_parser::keywords::Keyword::SECURITY => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SECURITY)?
                }
                sqltk_parser::keywords::Keyword::SELECT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SELECT)?
                }
                sqltk_parser::keywords::Keyword::SEMI => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SEMI)?
                }
                sqltk_parser::keywords::Keyword::SENSITIVE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SENSITIVE,
                        )?
                }
                sqltk_parser::keywords::Keyword::SEPARATOR => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SEPARATOR,
                        )?
                }
                sqltk_parser::keywords::Keyword::SEQUENCE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SEQUENCE)?
                }
                sqltk_parser::keywords::Keyword::SEQUENCEFILE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SEQUENCEFILE,
                        )?
                }
                sqltk_parser::keywords::Keyword::SEQUENCES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SEQUENCES,
                        )?
                }
                sqltk_parser::keywords::Keyword::SERDE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SERDE)?
                }
                sqltk_parser::keywords::Keyword::SERDEPROPERTIES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SERDEPROPERTIES,
                        )?
                }
                sqltk_parser::keywords::Keyword::SERIALIZABLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SERIALIZABLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::SESSION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SESSION)?
                }
                sqltk_parser::keywords::Keyword::SESSION_USER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SESSION_USER,
                        )?
                }
                sqltk_parser::keywords::Keyword::SET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SET)?
                }
                sqltk_parser::keywords::Keyword::SETS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SETS)?
                }
                sqltk_parser::keywords::Keyword::SETTINGS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SETTINGS)?
                }
                sqltk_parser::keywords::Keyword::SHARE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SHARE)?
                }
                sqltk_parser::keywords::Keyword::SHOW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SHOW)?
                }
                sqltk_parser::keywords::Keyword::SIMILAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SIMILAR)?
                }
                sqltk_parser::keywords::Keyword::SKIP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SKIP)?
                }
                sqltk_parser::keywords::Keyword::SLOW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SLOW)?
                }
                sqltk_parser::keywords::Keyword::SMALLINT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SMALLINT)?
                }
                sqltk_parser::keywords::Keyword::SNAPSHOT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SNAPSHOT)?
                }
                sqltk_parser::keywords::Keyword::SOME => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SOME)?
                }
                sqltk_parser::keywords::Keyword::SORT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SORT)?
                }
                sqltk_parser::keywords::Keyword::SORTED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SORTED)?
                }
                sqltk_parser::keywords::Keyword::SOURCE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SOURCE)?
                }
                sqltk_parser::keywords::Keyword::SPATIAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SPATIAL)?
                }
                sqltk_parser::keywords::Keyword::SPECIFIC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SPECIFIC)?
                }
                sqltk_parser::keywords::Keyword::SPECIFICTYPE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SPECIFICTYPE,
                        )?
                }
                sqltk_parser::keywords::Keyword::SQL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SQL)?
                }
                sqltk_parser::keywords::Keyword::SQLEXCEPTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SQLEXCEPTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::SQLSTATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SQLSTATE)?
                }
                sqltk_parser::keywords::Keyword::SQLWARNING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SQLWARNING,
                        )?
                }
                sqltk_parser::keywords::Keyword::SQRT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SQRT)?
                }
                sqltk_parser::keywords::Keyword::STABLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STABLE)?
                }
                sqltk_parser::keywords::Keyword::STAGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STAGE)?
                }
                sqltk_parser::keywords::Keyword::START => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::START)?
                }
                sqltk_parser::keywords::Keyword::STARTS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STARTS)?
                }
                sqltk_parser::keywords::Keyword::STATEMENT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::STATEMENT,
                        )?
                }
                sqltk_parser::keywords::Keyword::STATIC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STATIC)?
                }
                sqltk_parser::keywords::Keyword::STATISTICS => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::STATISTICS,
                        )?
                }
                sqltk_parser::keywords::Keyword::STATUS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STATUS)?
                }
                sqltk_parser::keywords::Keyword::STDDEV_POP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::STDDEV_POP,
                        )?
                }
                sqltk_parser::keywords::Keyword::STDDEV_SAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::STDDEV_SAMP,
                        )?
                }
                sqltk_parser::keywords::Keyword::STDIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STDIN)?
                }
                sqltk_parser::keywords::Keyword::STDOUT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STDOUT)?
                }
                sqltk_parser::keywords::Keyword::STEP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STEP)?
                }
                sqltk_parser::keywords::Keyword::STORAGE_INTEGRATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::STORAGE_INTEGRATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::STORED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STORED)?
                }
                sqltk_parser::keywords::Keyword::STRICT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STRICT)?
                }
                sqltk_parser::keywords::Keyword::STRING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STRING)?
                }
                sqltk_parser::keywords::Keyword::STRUCT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::STRUCT)?
                }
                sqltk_parser::keywords::Keyword::SUBMULTISET => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SUBMULTISET,
                        )?
                }
                sqltk_parser::keywords::Keyword::SUBSTRING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SUBSTRING,
                        )?
                }
                sqltk_parser::keywords::Keyword::SUBSTRING_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SUBSTRING_REGEX,
                        )?
                }
                sqltk_parser::keywords::Keyword::SUCCEEDS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SUCCEEDS)?
                }
                sqltk_parser::keywords::Keyword::SUM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SUM)?
                }
                sqltk_parser::keywords::Keyword::SUPER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SUPER)?
                }
                sqltk_parser::keywords::Keyword::SUPERUSER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SUPERUSER,
                        )?
                }
                sqltk_parser::keywords::Keyword::SUSPEND => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SUSPEND)?
                }
                sqltk_parser::keywords::Keyword::SWAP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SWAP)?
                }
                sqltk_parser::keywords::Keyword::SYMMETRIC => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SYMMETRIC,
                        )?
                }
                sqltk_parser::keywords::Keyword::SYNC => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SYNC)?
                }
                sqltk_parser::keywords::Keyword::SYSTEM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::SYSTEM)?
                }
                sqltk_parser::keywords::Keyword::SYSTEM_TIME => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SYSTEM_TIME,
                        )?
                }
                sqltk_parser::keywords::Keyword::SYSTEM_USER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::SYSTEM_USER,
                        )?
                }
                sqltk_parser::keywords::Keyword::TABLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TABLE)?
                }
                sqltk_parser::keywords::Keyword::TABLES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TABLES)?
                }
                sqltk_parser::keywords::Keyword::TABLESAMPLE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TABLESAMPLE,
                        )?
                }
                sqltk_parser::keywords::Keyword::TAG => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TAG)?
                }
                sqltk_parser::keywords::Keyword::TARGET => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TARGET)?
                }
                sqltk_parser::keywords::Keyword::TBLPROPERTIES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TBLPROPERTIES,
                        )?
                }
                sqltk_parser::keywords::Keyword::TEMP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TEMP)?
                }
                sqltk_parser::keywords::Keyword::TEMPORARY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TEMPORARY,
                        )?
                }
                sqltk_parser::keywords::Keyword::TERMINATED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TERMINATED,
                        )?
                }
                sqltk_parser::keywords::Keyword::TERSE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TERSE)?
                }
                sqltk_parser::keywords::Keyword::TEXT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TEXT)?
                }
                sqltk_parser::keywords::Keyword::TEXTFILE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TEXTFILE)?
                }
                sqltk_parser::keywords::Keyword::THEN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::THEN)?
                }
                sqltk_parser::keywords::Keyword::TIES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TIES)?
                }
                sqltk_parser::keywords::Keyword::TIME => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TIME)?
                }
                sqltk_parser::keywords::Keyword::TIMESTAMP => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TIMESTAMP,
                        )?
                }
                sqltk_parser::keywords::Keyword::TIMESTAMPTZ => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TIMESTAMPTZ,
                        )?
                }
                sqltk_parser::keywords::Keyword::TIMETZ => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TIMETZ)?
                }
                sqltk_parser::keywords::Keyword::TIMEZONE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TIMEZONE)?
                }
                sqltk_parser::keywords::Keyword::TIMEZONE_ABBR => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TIMEZONE_ABBR,
                        )?
                }
                sqltk_parser::keywords::Keyword::TIMEZONE_HOUR => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TIMEZONE_HOUR,
                        )?
                }
                sqltk_parser::keywords::Keyword::TIMEZONE_MINUTE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TIMEZONE_MINUTE,
                        )?
                }
                sqltk_parser::keywords::Keyword::TIMEZONE_REGION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TIMEZONE_REGION,
                        )?
                }
                sqltk_parser::keywords::Keyword::TINYBLOB => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TINYBLOB)?
                }
                sqltk_parser::keywords::Keyword::TINYINT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TINYINT)?
                }
                sqltk_parser::keywords::Keyword::TINYTEXT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TINYTEXT)?
                }
                sqltk_parser::keywords::Keyword::TO => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TO)?
                }
                sqltk_parser::keywords::Keyword::TOP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TOP)?
                }
                sqltk_parser::keywords::Keyword::TOTALS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TOTALS)?
                }
                sqltk_parser::keywords::Keyword::TRAILING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TRAILING)?
                }
                sqltk_parser::keywords::Keyword::TRANSACTION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRANSACTION,
                        )?
                }
                sqltk_parser::keywords::Keyword::TRANSIENT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRANSIENT,
                        )?
                }
                sqltk_parser::keywords::Keyword::TRANSLATE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRANSLATE,
                        )?
                }
                sqltk_parser::keywords::Keyword::TRANSLATE_REGEX => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRANSLATE_REGEX,
                        )?
                }
                sqltk_parser::keywords::Keyword::TRANSLATION => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRANSLATION,
                        )?
                }
                sqltk_parser::keywords::Keyword::TREAT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TREAT)?
                }
                sqltk_parser::keywords::Keyword::TRIGGER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TRIGGER)?
                }
                sqltk_parser::keywords::Keyword::TRIM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TRIM)?
                }
                sqltk_parser::keywords::Keyword::TRIM_ARRAY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRIM_ARRAY,
                        )?
                }
                sqltk_parser::keywords::Keyword::TRUE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TRUE)?
                }
                sqltk_parser::keywords::Keyword::TRUNCATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TRUNCATE)?
                }
                sqltk_parser::keywords::Keyword::TRY_CAST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TRY_CAST)?
                }
                sqltk_parser::keywords::Keyword::TRY_CONVERT => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::TRY_CONVERT,
                        )?
                }
                sqltk_parser::keywords::Keyword::TUPLE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TUPLE)?
                }
                sqltk_parser::keywords::Keyword::TYPE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::TYPE)?
                }
                sqltk_parser::keywords::Keyword::UESCAPE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UESCAPE)?
                }
                sqltk_parser::keywords::Keyword::UINT128 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UINT128)?
                }
                sqltk_parser::keywords::Keyword::UINT16 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UINT16)?
                }
                sqltk_parser::keywords::Keyword::UINT256 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UINT256)?
                }
                sqltk_parser::keywords::Keyword::UINT32 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UINT32)?
                }
                sqltk_parser::keywords::Keyword::UINT64 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UINT64)?
                }
                sqltk_parser::keywords::Keyword::UINT8 => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UINT8)?
                }
                sqltk_parser::keywords::Keyword::UNBOUNDED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::UNBOUNDED,
                        )?
                }
                sqltk_parser::keywords::Keyword::UNCACHE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNCACHE)?
                }
                sqltk_parser::keywords::Keyword::UNCOMMITTED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::UNCOMMITTED,
                        )?
                }
                sqltk_parser::keywords::Keyword::UNFREEZE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNFREEZE)?
                }
                sqltk_parser::keywords::Keyword::UNION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNION)?
                }
                sqltk_parser::keywords::Keyword::UNIQUE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNIQUE)?
                }
                sqltk_parser::keywords::Keyword::UNKNOWN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNKNOWN)?
                }
                sqltk_parser::keywords::Keyword::UNLISTEN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNLISTEN)?
                }
                sqltk_parser::keywords::Keyword::UNLOAD => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNLOAD)?
                }
                sqltk_parser::keywords::Keyword::UNLOCK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNLOCK)?
                }
                sqltk_parser::keywords::Keyword::UNLOGGED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNLOGGED)?
                }
                sqltk_parser::keywords::Keyword::UNMATCHED => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::UNMATCHED,
                        )?
                }
                sqltk_parser::keywords::Keyword::UNNEST => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNNEST)?
                }
                sqltk_parser::keywords::Keyword::UNPIVOT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNPIVOT)?
                }
                sqltk_parser::keywords::Keyword::UNSAFE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNSAFE)?
                }
                sqltk_parser::keywords::Keyword::UNSIGNED => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNSIGNED)?
                }
                sqltk_parser::keywords::Keyword::UNTIL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UNTIL)?
                }
                sqltk_parser::keywords::Keyword::UPDATE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UPDATE)?
                }
                sqltk_parser::keywords::Keyword::UPPER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UPPER)?
                }
                sqltk_parser::keywords::Keyword::URL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::URL)?
                }
                sqltk_parser::keywords::Keyword::USAGE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::USAGE)?
                }
                sqltk_parser::keywords::Keyword::USE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::USE)?
                }
                sqltk_parser::keywords::Keyword::USER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::USER)?
                }
                sqltk_parser::keywords::Keyword::USER_RESOURCES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::USER_RESOURCES,
                        )?
                }
                sqltk_parser::keywords::Keyword::USING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::USING)?
                }
                sqltk_parser::keywords::Keyword::UUID => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::UUID)?
                }
                sqltk_parser::keywords::Keyword::VACUUM => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VACUUM)?
                }
                sqltk_parser::keywords::Keyword::VALID => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VALID)?
                }
                sqltk_parser::keywords::Keyword::VALIDATION_MODE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::VALIDATION_MODE,
                        )?
                }
                sqltk_parser::keywords::Keyword::VALUE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VALUE)?
                }
                sqltk_parser::keywords::Keyword::VALUES => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VALUES)?
                }
                sqltk_parser::keywords::Keyword::VALUE_OF => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VALUE_OF)?
                }
                sqltk_parser::keywords::Keyword::VARBINARY => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::VARBINARY,
                        )?
                }
                sqltk_parser::keywords::Keyword::VARCHAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VARCHAR)?
                }
                sqltk_parser::keywords::Keyword::VARIABLES => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::VARIABLES,
                        )?
                }
                sqltk_parser::keywords::Keyword::VARYING => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VARYING)?
                }
                sqltk_parser::keywords::Keyword::VAR_POP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VAR_POP)?
                }
                sqltk_parser::keywords::Keyword::VAR_SAMP => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VAR_SAMP)?
                }
                sqltk_parser::keywords::Keyword::VERBOSE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VERBOSE)?
                }
                sqltk_parser::keywords::Keyword::VERSION => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VERSION)?
                }
                sqltk_parser::keywords::Keyword::VERSIONING => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::VERSIONING,
                        )?
                }
                sqltk_parser::keywords::Keyword::VIEW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VIEW)?
                }
                sqltk_parser::keywords::Keyword::VIEWS => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VIEWS)?
                }
                sqltk_parser::keywords::Keyword::VIRTUAL => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VIRTUAL)?
                }
                sqltk_parser::keywords::Keyword::VOLATILE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::VOLATILE)?
                }
                sqltk_parser::keywords::Keyword::WAREHOUSE => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::WAREHOUSE,
                        )?
                }
                sqltk_parser::keywords::Keyword::WEEK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WEEK)?
                }
                sqltk_parser::keywords::Keyword::WHEN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WHEN)?
                }
                sqltk_parser::keywords::Keyword::WHENEVER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WHENEVER)?
                }
                sqltk_parser::keywords::Keyword::WHERE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WHERE)?
                }
                sqltk_parser::keywords::Keyword::WIDTH_BUCKET => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::WIDTH_BUCKET,
                        )?
                }
                sqltk_parser::keywords::Keyword::WINDOW => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WINDOW)?
                }
                sqltk_parser::keywords::Keyword::WITH => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WITH)?
                }
                sqltk_parser::keywords::Keyword::WITHIN => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WITHIN)?
                }
                sqltk_parser::keywords::Keyword::WITHOUT => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WITHOUT)?
                }
                sqltk_parser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER,
                        )?
                }
                sqltk_parser::keywords::Keyword::WORK => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WORK)?
                }
                sqltk_parser::keywords::Keyword::WRITE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::WRITE)?
                }
                sqltk_parser::keywords::Keyword::XML => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::XML)?
                }
                sqltk_parser::keywords::Keyword::XOR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::XOR)?
                }
                sqltk_parser::keywords::Keyword::YEAR => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::YEAR)?
                }
                sqltk_parser::keywords::Keyword::ZONE => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ZONE)?
                }
                sqltk_parser::keywords::Keyword::ZORDER => {
                    transformer
                        .transform(node_path, sqltk_parser::keywords::Keyword::ZORDER)?
                }
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::tokenizer::Location {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { line, column } = self;
            Self {
                line: line.apply_transform_with_path(transformer, node_path)?,
                column: column.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::tokenizer::Span {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { start, end } = self;
            Self {
                start: start.apply_transform_with_path(transformer, node_path)?,
                end: end.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::tokenizer::Token {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::tokenizer::Token::EOF => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::EOF)?
                }
                sqltk_parser::tokenizer::Token::Word(field0) => {
                    sqltk_parser::tokenizer::Token::Word(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::Number(field0, field1) => {
                    sqltk_parser::tokenizer::Token::Number(
                        field0.apply_transform_with_path(transformer, node_path)?,
                        field1.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::Char(field0) => {
                    sqltk_parser::tokenizer::Token::Char(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::SingleQuotedString(field0) => {
                    sqltk_parser::tokenizer::Token::SingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::DoubleQuotedString(field0) => {
                    sqltk_parser::tokenizer::Token::DoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::TripleSingleQuotedString(field0) => {
                    sqltk_parser::tokenizer::Token::TripleSingleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::TripleDoubleQuotedString(field0) => {
                    sqltk_parser::tokenizer::Token::TripleDoubleQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::DollarQuotedString(field0) => {
                    sqltk_parser::tokenizer::Token::DollarQuotedString(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::SingleQuotedByteStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::SingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::DoubleQuotedByteStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::DoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                    field0,
                ) => {
                    sqltk_parser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                    field0,
                ) => {
                    sqltk_parser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::SingleQuotedRawStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::SingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::DoubleQuotedRawStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::DoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                    field0,
                ) => {
                    sqltk_parser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                    field0,
                ) => {
                    sqltk_parser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::NationalStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::NationalStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::EscapedStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::EscapedStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::UnicodeStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::UnicodeStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::HexStringLiteral(field0) => {
                    sqltk_parser::tokenizer::Token::HexStringLiteral(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::Comma => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Comma)?
                }
                sqltk_parser::tokenizer::Token::Whitespace(field0) => {
                    sqltk_parser::tokenizer::Token::Whitespace(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::DoubleEq => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::DoubleEq)?
                }
                sqltk_parser::tokenizer::Token::Eq => {
                    transformer.transform(node_path, sqltk_parser::tokenizer::Token::Eq)?
                }
                sqltk_parser::tokenizer::Token::Neq => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Neq)?
                }
                sqltk_parser::tokenizer::Token::Lt => {
                    transformer.transform(node_path, sqltk_parser::tokenizer::Token::Lt)?
                }
                sqltk_parser::tokenizer::Token::Gt => {
                    transformer.transform(node_path, sqltk_parser::tokenizer::Token::Gt)?
                }
                sqltk_parser::tokenizer::Token::LtEq => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::LtEq)?
                }
                sqltk_parser::tokenizer::Token::GtEq => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::GtEq)?
                }
                sqltk_parser::tokenizer::Token::Spaceship => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Spaceship)?
                }
                sqltk_parser::tokenizer::Token::Plus => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Plus)?
                }
                sqltk_parser::tokenizer::Token::Minus => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Minus)?
                }
                sqltk_parser::tokenizer::Token::Mul => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Mul)?
                }
                sqltk_parser::tokenizer::Token::Div => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Div)?
                }
                sqltk_parser::tokenizer::Token::DuckIntDiv => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::DuckIntDiv,
                        )?
                }
                sqltk_parser::tokenizer::Token::Mod => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Mod)?
                }
                sqltk_parser::tokenizer::Token::StringConcat => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::StringConcat,
                        )?
                }
                sqltk_parser::tokenizer::Token::LParen => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::LParen)?
                }
                sqltk_parser::tokenizer::Token::RParen => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::RParen)?
                }
                sqltk_parser::tokenizer::Token::Period => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Period)?
                }
                sqltk_parser::tokenizer::Token::Colon => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Colon)?
                }
                sqltk_parser::tokenizer::Token::DoubleColon => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::DoubleColon,
                        )?
                }
                sqltk_parser::tokenizer::Token::Assignment => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::Assignment,
                        )?
                }
                sqltk_parser::tokenizer::Token::SemiColon => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::SemiColon)?
                }
                sqltk_parser::tokenizer::Token::Backslash => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Backslash)?
                }
                sqltk_parser::tokenizer::Token::LBracket => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::LBracket)?
                }
                sqltk_parser::tokenizer::Token::RBracket => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::RBracket)?
                }
                sqltk_parser::tokenizer::Token::Ampersand => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Ampersand)?
                }
                sqltk_parser::tokenizer::Token::Pipe => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Pipe)?
                }
                sqltk_parser::tokenizer::Token::Caret => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Caret)?
                }
                sqltk_parser::tokenizer::Token::LBrace => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::LBrace)?
                }
                sqltk_parser::tokenizer::Token::RBrace => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::RBrace)?
                }
                sqltk_parser::tokenizer::Token::RArrow => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::RArrow)?
                }
                sqltk_parser::tokenizer::Token::Sharp => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Sharp)?
                }
                sqltk_parser::tokenizer::Token::Tilde => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Tilde)?
                }
                sqltk_parser::tokenizer::Token::TildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::TildeAsterisk,
                        )?
                }
                sqltk_parser::tokenizer::Token::ExclamationMarkTilde => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::ExclamationMarkTilde,
                        )?
                }
                sqltk_parser::tokenizer::Token::ExclamationMarkTildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::ExclamationMarkTildeAsterisk,
                        )?
                }
                sqltk_parser::tokenizer::Token::DoubleTilde => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::DoubleTilde,
                        )?
                }
                sqltk_parser::tokenizer::Token::DoubleTildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::DoubleTildeAsterisk,
                        )?
                }
                sqltk_parser::tokenizer::Token::ExclamationMarkDoubleTilde => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::ExclamationMarkDoubleTilde,
                        )?
                }
                sqltk_parser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk,
                        )?
                }
                sqltk_parser::tokenizer::Token::ShiftLeft => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::ShiftLeft)?
                }
                sqltk_parser::tokenizer::Token::ShiftRight => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::ShiftRight,
                        )?
                }
                sqltk_parser::tokenizer::Token::Overlap => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Overlap)?
                }
                sqltk_parser::tokenizer::Token::ExclamationMark => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::ExclamationMark,
                        )?
                }
                sqltk_parser::tokenizer::Token::DoubleExclamationMark => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::DoubleExclamationMark,
                        )?
                }
                sqltk_parser::tokenizer::Token::AtSign => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::AtSign)?
                }
                sqltk_parser::tokenizer::Token::CaretAt => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::CaretAt)?
                }
                sqltk_parser::tokenizer::Token::PGSquareRoot => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::PGSquareRoot,
                        )?
                }
                sqltk_parser::tokenizer::Token::PGCubeRoot => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::PGCubeRoot,
                        )?
                }
                sqltk_parser::tokenizer::Token::Placeholder(field0) => {
                    sqltk_parser::tokenizer::Token::Placeholder(
                        field0.apply_transform_with_path(transformer, node_path)?,
                    )
                }
                sqltk_parser::tokenizer::Token::Arrow => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Arrow)?
                }
                sqltk_parser::tokenizer::Token::LongArrow => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::LongArrow)?
                }
                sqltk_parser::tokenizer::Token::HashArrow => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::HashArrow)?
                }
                sqltk_parser::tokenizer::Token::HashLongArrow => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::HashLongArrow,
                        )?
                }
                sqltk_parser::tokenizer::Token::AtArrow => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::AtArrow)?
                }
                sqltk_parser::tokenizer::Token::ArrowAt => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::ArrowAt)?
                }
                sqltk_parser::tokenizer::Token::HashMinus => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::HashMinus)?
                }
                sqltk_parser::tokenizer::Token::AtQuestion => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::AtQuestion,
                        )?
                }
                sqltk_parser::tokenizer::Token::AtAt => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::AtAt)?
                }
                sqltk_parser::tokenizer::Token::Question => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Token::Question)?
                }
                sqltk_parser::tokenizer::Token::QuestionAnd => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::QuestionAnd,
                        )?
                }
                sqltk_parser::tokenizer::Token::QuestionPipe => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Token::QuestionPipe,
                        )?
                }
                sqltk_parser::tokenizer::Token::CustomBinaryOperator(field0) => {
                    sqltk_parser::tokenizer::Token::CustomBinaryOperator(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::tokenizer::TokenWithSpan {
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        node_path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: crate::Transform<'ast>,
    {
        node_path.push(self);
        let transformed = {
            let Self { token, span } = self;
            Self {
                token: token.apply_transform_with_path(transformer, node_path)?,
                span: span.apply_transform_with_path(transformer, node_path)?,
            }
        };
        let transformed = transformer.transform(node_path, transformed)?;
        node_path.pop();
        Ok(transformed)
    }
}
#[automatically_derived]
impl<'ast> crate::Transformable<'ast> for sqltk_parser::tokenizer::Whitespace {
    fn apply_transform_with_path<T>(
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
                sqltk_parser::tokenizer::Whitespace::Space => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Whitespace::Space,
                        )?
                }
                sqltk_parser::tokenizer::Whitespace::Newline => {
                    transformer
                        .transform(
                            node_path,
                            sqltk_parser::tokenizer::Whitespace::Newline,
                        )?
                }
                sqltk_parser::tokenizer::Whitespace::Tab => {
                    transformer
                        .transform(node_path, sqltk_parser::tokenizer::Whitespace::Tab)?
                }
                sqltk_parser::tokenizer::Whitespace::SingleLineComment {
                    comment,
                    prefix,
                } => {
                    sqltk_parser::tokenizer::Whitespace::SingleLineComment {
                        comment: comment
                            .apply_transform_with_path(transformer, node_path)?,
                        prefix: prefix.apply_transform_with_path(transformer, node_path)?,
                    }
                }
                sqltk_parser::tokenizer::Whitespace::MultiLineComment(field0) => {
                    sqltk_parser::tokenizer::Whitespace::MultiLineComment(
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
impl<'ast> crate::Transformable<'ast> for sqltk_parser::tokenizer::Word {
    fn apply_transform_with_path<T>(
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
