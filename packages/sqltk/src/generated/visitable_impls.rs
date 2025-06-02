use crate::visitable_impls::visit;
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AccessExpr {
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
                    sqltk_parser::ast::AccessExpr::Dot(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AccessExpr::Subscript(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AccessExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Action {
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
                    sqltk_parser::ast::Action::AddSearchOptimization => {}
                    sqltk_parser::ast::Action::Apply { apply_type } => {
                        apply_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::ApplyBudget => {}
                    sqltk_parser::ast::Action::AttachListing => {}
                    sqltk_parser::ast::Action::AttachPolicy => {}
                    sqltk_parser::ast::Action::Audit => {}
                    sqltk_parser::ast::Action::BindServiceEndpoint => {}
                    sqltk_parser::ast::Action::Connect => {}
                    sqltk_parser::ast::Action::Create { obj_type } => {
                        obj_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::DatabaseRole { role } => {
                        role.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Delete => {}
                    sqltk_parser::ast::Action::EvolveSchema => {}
                    sqltk_parser::ast::Action::Execute { obj_type } => {
                        obj_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Failover => {}
                    sqltk_parser::ast::Action::ImportedPrivileges => {}
                    sqltk_parser::ast::Action::ImportShare => {}
                    sqltk_parser::ast::Action::Insert { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Manage { manage_type } => {
                        manage_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::ManageReleases => {}
                    sqltk_parser::ast::Action::ManageVersions => {}
                    sqltk_parser::ast::Action::Modify { modify_type } => {
                        modify_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Monitor { monitor_type } => {
                        monitor_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Operate => {}
                    sqltk_parser::ast::Action::OverrideShareRestrictions => {}
                    sqltk_parser::ast::Action::Ownership => {}
                    sqltk_parser::ast::Action::PurchaseDataExchangeListing => {}
                    sqltk_parser::ast::Action::Read => {}
                    sqltk_parser::ast::Action::ReadSession => {}
                    sqltk_parser::ast::Action::References { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Replicate => {}
                    sqltk_parser::ast::Action::ResolveAll => {}
                    sqltk_parser::ast::Action::Role { role } => {
                        role.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Select { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Temporary => {}
                    sqltk_parser::ast::Action::Trigger => {}
                    sqltk_parser::ast::Action::Truncate => {}
                    sqltk_parser::ast::Action::Update { columns } => {
                        columns.accept(visitor)?;
                    }
                    sqltk_parser::ast::Action::Usage => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Action {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ActionApplyType {
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
                    sqltk_parser::ast::ActionApplyType::AggregationPolicy => {}
                    sqltk_parser::ast::ActionApplyType::AuthenticationPolicy => {}
                    sqltk_parser::ast::ActionApplyType::JoinPolicy => {}
                    sqltk_parser::ast::ActionApplyType::MaskingPolicy => {}
                    sqltk_parser::ast::ActionApplyType::PackagesPolicy => {}
                    sqltk_parser::ast::ActionApplyType::PasswordPolicy => {}
                    sqltk_parser::ast::ActionApplyType::ProjectionPolicy => {}
                    sqltk_parser::ast::ActionApplyType::RowAccessPolicy => {}
                    sqltk_parser::ast::ActionApplyType::SessionPolicy => {}
                    sqltk_parser::ast::ActionApplyType::Tag => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ActionApplyType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ActionCreateObjectType {
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
                    sqltk_parser::ast::ActionCreateObjectType::Account => {}
                    sqltk_parser::ast::ActionCreateObjectType::Application => {}
                    sqltk_parser::ast::ActionCreateObjectType::ApplicationPackage => {}
                    sqltk_parser::ast::ActionCreateObjectType::ComputePool => {}
                    sqltk_parser::ast::ActionCreateObjectType::DataExchangeListing => {}
                    sqltk_parser::ast::ActionCreateObjectType::Database => {}
                    sqltk_parser::ast::ActionCreateObjectType::ExternalVolume => {}
                    sqltk_parser::ast::ActionCreateObjectType::FailoverGroup => {}
                    sqltk_parser::ast::ActionCreateObjectType::Integration => {}
                    sqltk_parser::ast::ActionCreateObjectType::NetworkPolicy => {}
                    sqltk_parser::ast::ActionCreateObjectType::OrganiationListing => {}
                    sqltk_parser::ast::ActionCreateObjectType::ReplicationGroup => {}
                    sqltk_parser::ast::ActionCreateObjectType::Role => {}
                    sqltk_parser::ast::ActionCreateObjectType::Share => {}
                    sqltk_parser::ast::ActionCreateObjectType::User => {}
                    sqltk_parser::ast::ActionCreateObjectType::Warehouse => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ActionCreateObjectType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ActionExecuteObjectType {
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
                    sqltk_parser::ast::ActionExecuteObjectType::Alert => {}
                    sqltk_parser::ast::ActionExecuteObjectType::DataMetricFunction => {}
                    sqltk_parser::ast::ActionExecuteObjectType::ManagedAlert => {}
                    sqltk_parser::ast::ActionExecuteObjectType::ManagedTask => {}
                    sqltk_parser::ast::ActionExecuteObjectType::Task => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ActionExecuteObjectType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ActionManageType {
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
                    sqltk_parser::ast::ActionManageType::AccountSupportCases => {}
                    sqltk_parser::ast::ActionManageType::EventSharing => {}
                    sqltk_parser::ast::ActionManageType::Grants => {}
                    sqltk_parser::ast::ActionManageType::ListingAutoFulfillment => {}
                    sqltk_parser::ast::ActionManageType::OrganizationSupportCases => {}
                    sqltk_parser::ast::ActionManageType::UserSupportCases => {}
                    sqltk_parser::ast::ActionManageType::Warehouses => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ActionManageType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ActionModifyType {
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
                    sqltk_parser::ast::ActionModifyType::LogLevel => {}
                    sqltk_parser::ast::ActionModifyType::TraceLevel => {}
                    sqltk_parser::ast::ActionModifyType::SessionLogLevel => {}
                    sqltk_parser::ast::ActionModifyType::SessionTraceLevel => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ActionModifyType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ActionMonitorType {
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
                    sqltk_parser::ast::ActionMonitorType::Execution => {}
                    sqltk_parser::ast::ActionMonitorType::Security => {}
                    sqltk_parser::ast::ActionMonitorType::Usage => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ActionMonitorType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AddDropSync {
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
                    sqltk_parser::ast::AddDropSync::ADD => {}
                    sqltk_parser::ast::AddDropSync::DROP => {}
                    sqltk_parser::ast::AddDropSync::SYNC => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AddDropSync {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AfterMatchSkip {
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
                    sqltk_parser::ast::AfterMatchSkip::PastLastRow => {}
                    sqltk_parser::ast::AfterMatchSkip::ToNextRow => {}
                    sqltk_parser::ast::AfterMatchSkip::ToFirst(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AfterMatchSkip::ToLast(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AfterMatchSkip {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterColumnOperation {
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
                    sqltk_parser::ast::AlterColumnOperation::SetNotNull => {}
                    sqltk_parser::ast::AlterColumnOperation::DropNotNull => {}
                    sqltk_parser::ast::AlterColumnOperation::SetDefault { value } => {
                        value.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterColumnOperation::DropDefault => {}
                    sqltk_parser::ast::AlterColumnOperation::SetDataType {
                        data_type,
                        using,
                    } => {
                        data_type.accept(visitor)?;
                        using.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterColumnOperation::AddGenerated {
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
impl crate::AsNodeKey for sqltk_parser::ast::AlterColumnOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterConnectorOwner {
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
                    sqltk_parser::ast::AlterConnectorOwner::User(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterConnectorOwner::Role(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterConnectorOwner {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterIndexOperation {
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
                    sqltk_parser::ast::AlterIndexOperation::RenameIndex {
                        index_name,
                    } => {
                        index_name.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterIndexOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterPolicyOperation {
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
                    sqltk_parser::ast::AlterPolicyOperation::Rename { new_name } => {
                        new_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterPolicyOperation::Apply {
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
impl crate::AsNodeKey for sqltk_parser::ast::AlterPolicyOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterRoleOperation {
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
                    sqltk_parser::ast::AlterRoleOperation::RenameRole { role_name } => {
                        role_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterRoleOperation::AddMember { member_name } => {
                        member_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterRoleOperation::DropMember {
                        member_name,
                    } => {
                        member_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterRoleOperation::WithOptions { options } => {
                        options.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterRoleOperation::Set {
                        config_name,
                        config_value,
                        in_database,
                    } => {
                        config_name.accept(visitor)?;
                        config_value.accept(visitor)?;
                        in_database.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterRoleOperation::Reset {
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
impl crate::AsNodeKey for sqltk_parser::ast::AlterRoleOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTableAlgorithm {
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
                    sqltk_parser::ast::AlterTableAlgorithm::Default => {}
                    sqltk_parser::ast::AlterTableAlgorithm::Instant => {}
                    sqltk_parser::ast::AlterTableAlgorithm::Inplace => {}
                    sqltk_parser::ast::AlterTableAlgorithm::Copy => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTableAlgorithm {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTableOperation {
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
                    sqltk_parser::ast::AlterTableOperation::AddConstraint(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::AddColumn {
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
                    sqltk_parser::ast::AlterTableOperation::AddProjection {
                        if_not_exists,
                        name,
                        select,
                    } => {
                        if_not_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        select.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DropProjection {
                        if_exists,
                        name,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::MaterializeProjection {
                        if_exists,
                        name,
                        partition,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        partition.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::ClearProjection {
                        if_exists,
                        name,
                        partition,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        partition.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DisableRowLevelSecurity => {}
                    sqltk_parser::ast::AlterTableOperation::DisableRule { name } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DisableTrigger { name } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DropConstraint {
                        if_exists,
                        name,
                        drop_behavior,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        drop_behavior.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DropColumn {
                        column_name,
                        if_exists,
                        drop_behavior,
                    } => {
                        column_name.accept(visitor)?;
                        if_exists.accept(visitor)?;
                        drop_behavior.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::AttachPartition {
                        partition,
                    } => {
                        partition.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DetachPartition {
                        partition,
                    } => {
                        partition.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::FreezePartition {
                        partition,
                        with_name,
                    } => {
                        partition.accept(visitor)?;
                        with_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::UnfreezePartition {
                        partition,
                        with_name,
                    } => {
                        partition.accept(visitor)?;
                        with_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DropPrimaryKey => {}
                    sqltk_parser::ast::AlterTableOperation::EnableAlwaysRule {
                        name,
                    } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::EnableAlwaysTrigger {
                        name,
                    } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::EnableReplicaRule {
                        name,
                    } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::EnableReplicaTrigger {
                        name,
                    } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::EnableRowLevelSecurity => {}
                    sqltk_parser::ast::AlterTableOperation::EnableRule { name } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::EnableTrigger { name } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::RenamePartitions {
                        old_partitions,
                        new_partitions,
                    } => {
                        old_partitions.accept(visitor)?;
                        new_partitions.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::AddPartitions {
                        if_not_exists,
                        new_partitions,
                    } => {
                        if_not_exists.accept(visitor)?;
                        new_partitions.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DropPartitions {
                        partitions,
                        if_exists,
                    } => {
                        partitions.accept(visitor)?;
                        if_exists.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::RenameColumn {
                        old_column_name,
                        new_column_name,
                    } => {
                        old_column_name.accept(visitor)?;
                        new_column_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::RenameTable {
                        table_name,
                    } => {
                        table_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::ChangeColumn {
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
                    sqltk_parser::ast::AlterTableOperation::ModifyColumn {
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
                    sqltk_parser::ast::AlterTableOperation::RenameConstraint {
                        old_name,
                        new_name,
                    } => {
                        old_name.accept(visitor)?;
                        new_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::AlterColumn {
                        column_name,
                        op,
                    } => {
                        column_name.accept(visitor)?;
                        op.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::SwapWith { table_name } => {
                        table_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::SetTblProperties {
                        table_properties,
                    } => {
                        table_properties.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::OwnerTo { new_owner } => {
                        new_owner.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::ClusterBy { exprs } => {
                        exprs.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::DropClusteringKey => {}
                    sqltk_parser::ast::AlterTableOperation::SuspendRecluster => {}
                    sqltk_parser::ast::AlterTableOperation::ResumeRecluster => {}
                    sqltk_parser::ast::AlterTableOperation::Algorithm {
                        equals,
                        algorithm,
                    } => {
                        equals.accept(visitor)?;
                        algorithm.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTableOperation::AutoIncrement {
                        equals,
                        value,
                    } => {
                        equals.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTableOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterType {
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
                self.operation.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTypeAddValue {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.if_not_exists.accept(visitor)?;
                self.value.accept(visitor)?;
                self.position.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTypeAddValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTypeAddValuePosition {
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
                    sqltk_parser::ast::AlterTypeAddValuePosition::Before(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTypeAddValuePosition::After(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTypeAddValuePosition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTypeOperation {
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
                    sqltk_parser::ast::AlterTypeOperation::Rename(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTypeOperation::AddValue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AlterTypeOperation::RenameValue(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTypeOperation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTypeRename {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.new_name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTypeRename {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AlterTypeRenameValue {
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
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AlterTypeRenameValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AnalyzeFormat {
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
                    sqltk_parser::ast::AnalyzeFormat::TEXT => {}
                    sqltk_parser::ast::AnalyzeFormat::GRAPHVIZ => {}
                    sqltk_parser::ast::AnalyzeFormat::JSON => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AnalyzeFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ArgMode {
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
                    sqltk_parser::ast::ArgMode::In => {}
                    sqltk_parser::ast::ArgMode::Out => {}
                    sqltk_parser::ast::ArgMode::InOut => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ArgMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Array {
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
impl crate::AsNodeKey for sqltk_parser::ast::Array {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ArrayElemTypeDef {
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
                    sqltk_parser::ast::ArrayElemTypeDef::None => {}
                    sqltk_parser::ast::ArrayElemTypeDef::AngleBracket(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ArrayElemTypeDef::SquareBracket(
                        field0,
                        field1,
                    ) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::ArrayElemTypeDef::Parenthesis(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ArrayElemTypeDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Assignment {
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
impl crate::AsNodeKey for sqltk_parser::ast::Assignment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AssignmentTarget {
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
                    sqltk_parser::ast::AssignmentTarget::ColumnName(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AssignmentTarget::Tuple(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AssignmentTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::AttachDuckDBDatabaseOption {
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
                    sqltk_parser::ast::AttachDuckDBDatabaseOption::ReadOnly(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::AttachDuckDBDatabaseOption::Type(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::AttachDuckDBDatabaseOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::BeginTransactionKind {
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
                    sqltk_parser::ast::BeginTransactionKind::Transaction => {}
                    sqltk_parser::ast::BeginTransactionKind::Work => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::BeginTransactionKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::BinaryLength {
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
                    sqltk_parser::ast::BinaryLength::IntegerLength { length } => {
                        length.accept(visitor)?;
                    }
                    sqltk_parser::ast::BinaryLength::Max => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::BinaryLength {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::BinaryOperator {
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
                    sqltk_parser::ast::BinaryOperator::Plus => {}
                    sqltk_parser::ast::BinaryOperator::Minus => {}
                    sqltk_parser::ast::BinaryOperator::Multiply => {}
                    sqltk_parser::ast::BinaryOperator::Divide => {}
                    sqltk_parser::ast::BinaryOperator::Modulo => {}
                    sqltk_parser::ast::BinaryOperator::StringConcat => {}
                    sqltk_parser::ast::BinaryOperator::Gt => {}
                    sqltk_parser::ast::BinaryOperator::Lt => {}
                    sqltk_parser::ast::BinaryOperator::GtEq => {}
                    sqltk_parser::ast::BinaryOperator::LtEq => {}
                    sqltk_parser::ast::BinaryOperator::Spaceship => {}
                    sqltk_parser::ast::BinaryOperator::Eq => {}
                    sqltk_parser::ast::BinaryOperator::NotEq => {}
                    sqltk_parser::ast::BinaryOperator::And => {}
                    sqltk_parser::ast::BinaryOperator::Or => {}
                    sqltk_parser::ast::BinaryOperator::Xor => {}
                    sqltk_parser::ast::BinaryOperator::BitwiseOr => {}
                    sqltk_parser::ast::BinaryOperator::BitwiseAnd => {}
                    sqltk_parser::ast::BinaryOperator::BitwiseXor => {}
                    sqltk_parser::ast::BinaryOperator::DuckIntegerDivide => {}
                    sqltk_parser::ast::BinaryOperator::MyIntegerDivide => {}
                    sqltk_parser::ast::BinaryOperator::Custom(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::BinaryOperator::PGBitwiseXor => {}
                    sqltk_parser::ast::BinaryOperator::PGBitwiseShiftLeft => {}
                    sqltk_parser::ast::BinaryOperator::PGBitwiseShiftRight => {}
                    sqltk_parser::ast::BinaryOperator::PGExp => {}
                    sqltk_parser::ast::BinaryOperator::PGOverlap => {}
                    sqltk_parser::ast::BinaryOperator::PGRegexMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGRegexIMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGRegexNotMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGRegexNotIMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGLikeMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGILikeMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGNotLikeMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGNotILikeMatch => {}
                    sqltk_parser::ast::BinaryOperator::PGStartsWith => {}
                    sqltk_parser::ast::BinaryOperator::Arrow => {}
                    sqltk_parser::ast::BinaryOperator::LongArrow => {}
                    sqltk_parser::ast::BinaryOperator::HashArrow => {}
                    sqltk_parser::ast::BinaryOperator::HashLongArrow => {}
                    sqltk_parser::ast::BinaryOperator::AtAt => {}
                    sqltk_parser::ast::BinaryOperator::AtArrow => {}
                    sqltk_parser::ast::BinaryOperator::ArrowAt => {}
                    sqltk_parser::ast::BinaryOperator::HashMinus => {}
                    sqltk_parser::ast::BinaryOperator::AtQuestion => {}
                    sqltk_parser::ast::BinaryOperator::Question => {}
                    sqltk_parser::ast::BinaryOperator::QuestionAnd => {}
                    sqltk_parser::ast::BinaryOperator::QuestionPipe => {}
                    sqltk_parser::ast::BinaryOperator::PGCustomBinaryOperator(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::BinaryOperator::Overlaps => {}
                    sqltk_parser::ast::BinaryOperator::DoubleHash => {}
                    sqltk_parser::ast::BinaryOperator::LtDashGt => {}
                    sqltk_parser::ast::BinaryOperator::AndLt => {}
                    sqltk_parser::ast::BinaryOperator::AndGt => {}
                    sqltk_parser::ast::BinaryOperator::LtLtPipe => {}
                    sqltk_parser::ast::BinaryOperator::PipeGtGt => {}
                    sqltk_parser::ast::BinaryOperator::AndLtPipe => {}
                    sqltk_parser::ast::BinaryOperator::PipeAndGt => {}
                    sqltk_parser::ast::BinaryOperator::LtCaret => {}
                    sqltk_parser::ast::BinaryOperator::GtCaret => {}
                    sqltk_parser::ast::BinaryOperator::QuestionHash => {}
                    sqltk_parser::ast::BinaryOperator::QuestionDash => {}
                    sqltk_parser::ast::BinaryOperator::QuestionDashPipe => {}
                    sqltk_parser::ast::BinaryOperator::QuestionDoublePipe => {}
                    sqltk_parser::ast::BinaryOperator::At => {}
                    sqltk_parser::ast::BinaryOperator::TildeEq => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::BinaryOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CascadeOption {
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
                    sqltk_parser::ast::CascadeOption::Cascade => {}
                    sqltk_parser::ast::CascadeOption::Restrict => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CascadeOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CaseWhen {
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
                self.result.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CaseWhen {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CastFormat {
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
                    sqltk_parser::ast::CastFormat::Value(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CastFormat::ValueAtTimeZone(field0, field1) => {
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
impl crate::AsNodeKey for sqltk_parser::ast::CastFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CastKind {
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
                    sqltk_parser::ast::CastKind::Cast => {}
                    sqltk_parser::ast::CastKind::TryCast => {}
                    sqltk_parser::ast::CastKind::SafeCast => {}
                    sqltk_parser::ast::CastKind::DoubleColon => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CastKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CeilFloorKind {
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
                    sqltk_parser::ast::CeilFloorKind::DateTimeField(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CeilFloorKind::Scale(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CeilFloorKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CharLengthUnits {
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
                    sqltk_parser::ast::CharLengthUnits::Characters => {}
                    sqltk_parser::ast::CharLengthUnits::Octets => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CharLengthUnits {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CharacterLength {
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
                    sqltk_parser::ast::CharacterLength::IntegerLength {
                        length,
                        unit,
                    } => {
                        length.accept(visitor)?;
                        unit.accept(visitor)?;
                    }
                    sqltk_parser::ast::CharacterLength::Max => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CharacterLength {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CloseCursor {
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
                    sqltk_parser::ast::CloseCursor::All => {}
                    sqltk_parser::ast::CloseCursor::Specific { name } => {
                        name.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CloseCursor {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ClusteredBy {
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
impl crate::AsNodeKey for sqltk_parser::ast::ClusteredBy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ClusteredIndex {
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
impl crate::AsNodeKey for sqltk_parser::ast::ClusteredIndex {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ColumnDef {
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
impl crate::AsNodeKey for sqltk_parser::ast::ColumnDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ColumnOption {
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
                    sqltk_parser::ast::ColumnOption::Null => {}
                    sqltk_parser::ast::ColumnOption::NotNull => {}
                    sqltk_parser::ast::ColumnOption::Default(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Materialized(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Ephemeral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Alias(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Unique {
                        is_primary,
                        characteristics,
                    } => {
                        is_primary.accept(visitor)?;
                        characteristics.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::ForeignKey {
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
                    sqltk_parser::ast::ColumnOption::Check(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::DialectSpecific(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::CharacterSet(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Collation(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Comment(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::OnUpdate(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Generated {
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
                    sqltk_parser::ast::ColumnOption::Options(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Identity(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::OnConflict(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Policy(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnOption::Tags(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ColumnOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ColumnOptionDef {
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
impl crate::AsNodeKey for sqltk_parser::ast::ColumnOptionDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ColumnPolicy {
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
                    sqltk_parser::ast::ColumnPolicy::MaskingPolicy(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ColumnPolicy::ProjectionPolicy(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ColumnPolicy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ColumnPolicyProperty {
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
impl crate::AsNodeKey for sqltk_parser::ast::ColumnPolicyProperty {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CommentDef {
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
                    sqltk_parser::ast::CommentDef::WithEq(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CommentDef::WithoutEq(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CommentDef::AfterColumnDefsWithoutEq(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CommentDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CommentObject {
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
                    sqltk_parser::ast::CommentObject::Column => {}
                    sqltk_parser::ast::CommentObject::Table => {}
                    sqltk_parser::ast::CommentObject::Extension => {}
                    sqltk_parser::ast::CommentObject::Schema => {}
                    sqltk_parser::ast::CommentObject::Database => {}
                    sqltk_parser::ast::CommentObject::User => {}
                    sqltk_parser::ast::CommentObject::Role => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CommentObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ConflictTarget {
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
                    sqltk_parser::ast::ConflictTarget::Columns(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ConflictTarget::OnConstraint(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ConflictTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ConnectBy {
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
impl crate::AsNodeKey for sqltk_parser::ast::ConnectBy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ConstraintCharacteristics {
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
impl crate::AsNodeKey for sqltk_parser::ast::ConstraintCharacteristics {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ContextModifier {
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
                    sqltk_parser::ast::ContextModifier::None => {}
                    sqltk_parser::ast::ContextModifier::Local => {}
                    sqltk_parser::ast::ContextModifier::Session => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ContextModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CopyIntoSnowflakeKind {
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
                    sqltk_parser::ast::CopyIntoSnowflakeKind::Table => {}
                    sqltk_parser::ast::CopyIntoSnowflakeKind::Location => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CopyIntoSnowflakeKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CopyLegacyCsvOption {
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
                    sqltk_parser::ast::CopyLegacyCsvOption::Header => {}
                    sqltk_parser::ast::CopyLegacyCsvOption::Quote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyLegacyCsvOption::Escape(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyLegacyCsvOption::ForceQuote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyLegacyCsvOption::ForceNotNull(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CopyLegacyCsvOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CopyLegacyOption {
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
                    sqltk_parser::ast::CopyLegacyOption::Binary => {}
                    sqltk_parser::ast::CopyLegacyOption::Delimiter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyLegacyOption::Null(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyLegacyOption::Csv(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CopyLegacyOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CopyOption {
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
                    sqltk_parser::ast::CopyOption::Format(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Freeze(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Delimiter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Null(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Header(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Quote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Escape(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::ForceQuote(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::ForceNotNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::ForceNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyOption::Encoding(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CopyOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CopySource {
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
                    sqltk_parser::ast::CopySource::Table { table_name, columns } => {
                        table_name.accept(visitor)?;
                        columns.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopySource::Query(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CopySource {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CopyTarget {
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
                    sqltk_parser::ast::CopyTarget::Stdin => {}
                    sqltk_parser::ast::CopyTarget::Stdout => {}
                    sqltk_parser::ast::CopyTarget::File { filename } => {
                        filename.accept(visitor)?;
                    }
                    sqltk_parser::ast::CopyTarget::Program { command } => {
                        command.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CopyTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateConnector {
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
                self.if_not_exists.accept(visitor)?;
                self.connector_type.accept(visitor)?;
                self.url.accept(visitor)?;
                self.comment.accept(visitor)?;
                self.with_dcproperties.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateConnector {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateFunction {
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
                self.if_not_exists.accept(visitor)?;
                self.name.accept(visitor)?;
                self.args.accept(visitor)?;
                self.return_type.accept(visitor)?;
                self.function_body.accept(visitor)?;
                self.behavior.accept(visitor)?;
                self.called_on_null.accept(visitor)?;
                self.parallel.accept(visitor)?;
                self.using.accept(visitor)?;
                self.language.accept(visitor)?;
                self.determinism_specifier.accept(visitor)?;
                self.options.accept(visitor)?;
                self.remote_connection.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateFunction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateFunctionBody {
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
                    sqltk_parser::ast::CreateFunctionBody::AsBeforeOptions(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CreateFunctionBody::AsAfterOptions(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CreateFunctionBody::Return(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateFunctionBody {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateFunctionUsing {
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
                    sqltk_parser::ast::CreateFunctionUsing::Jar(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CreateFunctionUsing::File(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CreateFunctionUsing::Archive(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateFunctionUsing {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateIndex {
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
impl crate::AsNodeKey for sqltk_parser::ast::CreateIndex {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreatePolicyCommand {
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
                    sqltk_parser::ast::CreatePolicyCommand::All => {}
                    sqltk_parser::ast::CreatePolicyCommand::Select => {}
                    sqltk_parser::ast::CreatePolicyCommand::Insert => {}
                    sqltk_parser::ast::CreatePolicyCommand::Update => {}
                    sqltk_parser::ast::CreatePolicyCommand::Delete => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreatePolicyCommand {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreatePolicyType {
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
                    sqltk_parser::ast::CreatePolicyType::Permissive => {}
                    sqltk_parser::ast::CreatePolicyType::Restrictive => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreatePolicyType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateTable {
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
                self.iceberg.accept(visitor)?;
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
                self.external_volume.accept(visitor)?;
                self.base_location.accept(visitor)?;
                self.catalog.accept(visitor)?;
                self.catalog_sync.accept(visitor)?;
                self.storage_serialization_policy.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateTable {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateTableOptions {
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
                    sqltk_parser::ast::CreateTableOptions::None => {}
                    sqltk_parser::ast::CreateTableOptions::With(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::CreateTableOptions::Options(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateTableOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateViewAlgorithm {
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
                    sqltk_parser::ast::CreateViewAlgorithm::Undefined => {}
                    sqltk_parser::ast::CreateViewAlgorithm::Merge => {}
                    sqltk_parser::ast::CreateViewAlgorithm::TempTable => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateViewAlgorithm {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateViewParams {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.algorithm.accept(visitor)?;
                self.definer.accept(visitor)?;
                self.security.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateViewParams {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CreateViewSecurity {
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
                    sqltk_parser::ast::CreateViewSecurity::Definer => {}
                    sqltk_parser::ast::CreateViewSecurity::Invoker => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CreateViewSecurity {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Cte {
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
                self.closing_paren_token.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Cte {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::CteAsMaterialized {
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
                    sqltk_parser::ast::CteAsMaterialized::Materialized => {}
                    sqltk_parser::ast::CteAsMaterialized::NotMaterialized => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::CteAsMaterialized {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DataType {
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
                    sqltk_parser::ast::DataType::Table(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Character(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Char(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::CharacterVarying(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::CharVarying(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Varchar(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Nvarchar(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Uuid => {}
                    sqltk_parser::ast::DataType::CharacterLargeObject(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::CharLargeObject(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Clob(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Binary(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Varbinary(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Blob(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::TinyBlob => {}
                    sqltk_parser::ast::DataType::MediumBlob => {}
                    sqltk_parser::ast::DataType::LongBlob => {}
                    sqltk_parser::ast::DataType::Bytes(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Numeric(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Decimal(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::BigNumeric(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::BigDecimal(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Dec(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Float(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::TinyInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::TinyIntUnsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int2(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int2Unsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::SmallInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::SmallIntUnsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::MediumInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::MediumIntUnsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int4(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int8(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int16 => {}
                    sqltk_parser::ast::DataType::Int32 => {}
                    sqltk_parser::ast::DataType::Int64 => {}
                    sqltk_parser::ast::DataType::Int128 => {}
                    sqltk_parser::ast::DataType::Int256 => {}
                    sqltk_parser::ast::DataType::Integer(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::IntUnsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int4Unsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::IntegerUnsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::UInt8 => {}
                    sqltk_parser::ast::DataType::UInt16 => {}
                    sqltk_parser::ast::DataType::UInt32 => {}
                    sqltk_parser::ast::DataType::UInt64 => {}
                    sqltk_parser::ast::DataType::UInt128 => {}
                    sqltk_parser::ast::DataType::UInt256 => {}
                    sqltk_parser::ast::DataType::BigInt(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::BigIntUnsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Int8Unsigned(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Signed => {}
                    sqltk_parser::ast::DataType::SignedInteger => {}
                    sqltk_parser::ast::DataType::Unsigned => {}
                    sqltk_parser::ast::DataType::UnsignedInteger => {}
                    sqltk_parser::ast::DataType::Float4 => {}
                    sqltk_parser::ast::DataType::Float32 => {}
                    sqltk_parser::ast::DataType::Float64 => {}
                    sqltk_parser::ast::DataType::Real => {}
                    sqltk_parser::ast::DataType::Float8 => {}
                    sqltk_parser::ast::DataType::Double(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::DoublePrecision => {}
                    sqltk_parser::ast::DataType::Bool => {}
                    sqltk_parser::ast::DataType::Boolean => {}
                    sqltk_parser::ast::DataType::Date => {}
                    sqltk_parser::ast::DataType::Date32 => {}
                    sqltk_parser::ast::DataType::Time(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Datetime(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Datetime64(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Timestamp(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Interval => {}
                    sqltk_parser::ast::DataType::JSON => {}
                    sqltk_parser::ast::DataType::JSONB => {}
                    sqltk_parser::ast::DataType::Regclass => {}
                    sqltk_parser::ast::DataType::Text => {}
                    sqltk_parser::ast::DataType::TinyText => {}
                    sqltk_parser::ast::DataType::MediumText => {}
                    sqltk_parser::ast::DataType::LongText => {}
                    sqltk_parser::ast::DataType::String(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::FixedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Bytea => {}
                    sqltk_parser::ast::DataType::Bit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::BitVarying(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::VarBit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Custom(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Array(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Map(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Tuple(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Nested(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Enum(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Set(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Struct(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Union(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Nullable(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::LowCardinality(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DataType::Unspecified => {}
                    sqltk_parser::ast::DataType::Trigger => {}
                    sqltk_parser::ast::DataType::AnyType => {}
                    sqltk_parser::ast::DataType::GeometricType(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DataType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DateTimeField {
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
                    sqltk_parser::ast::DateTimeField::Year => {}
                    sqltk_parser::ast::DateTimeField::Years => {}
                    sqltk_parser::ast::DateTimeField::Month => {}
                    sqltk_parser::ast::DateTimeField::Months => {}
                    sqltk_parser::ast::DateTimeField::Week(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DateTimeField::Weeks => {}
                    sqltk_parser::ast::DateTimeField::Day => {}
                    sqltk_parser::ast::DateTimeField::DayOfWeek => {}
                    sqltk_parser::ast::DateTimeField::DayOfYear => {}
                    sqltk_parser::ast::DateTimeField::Days => {}
                    sqltk_parser::ast::DateTimeField::Date => {}
                    sqltk_parser::ast::DateTimeField::Datetime => {}
                    sqltk_parser::ast::DateTimeField::Hour => {}
                    sqltk_parser::ast::DateTimeField::Hours => {}
                    sqltk_parser::ast::DateTimeField::Minute => {}
                    sqltk_parser::ast::DateTimeField::Minutes => {}
                    sqltk_parser::ast::DateTimeField::Second => {}
                    sqltk_parser::ast::DateTimeField::Seconds => {}
                    sqltk_parser::ast::DateTimeField::Century => {}
                    sqltk_parser::ast::DateTimeField::Decade => {}
                    sqltk_parser::ast::DateTimeField::Dow => {}
                    sqltk_parser::ast::DateTimeField::Doy => {}
                    sqltk_parser::ast::DateTimeField::Epoch => {}
                    sqltk_parser::ast::DateTimeField::Isodow => {}
                    sqltk_parser::ast::DateTimeField::IsoWeek => {}
                    sqltk_parser::ast::DateTimeField::Isoyear => {}
                    sqltk_parser::ast::DateTimeField::Julian => {}
                    sqltk_parser::ast::DateTimeField::Microsecond => {}
                    sqltk_parser::ast::DateTimeField::Microseconds => {}
                    sqltk_parser::ast::DateTimeField::Millenium => {}
                    sqltk_parser::ast::DateTimeField::Millennium => {}
                    sqltk_parser::ast::DateTimeField::Millisecond => {}
                    sqltk_parser::ast::DateTimeField::Milliseconds => {}
                    sqltk_parser::ast::DateTimeField::Nanosecond => {}
                    sqltk_parser::ast::DateTimeField::Nanoseconds => {}
                    sqltk_parser::ast::DateTimeField::Quarter => {}
                    sqltk_parser::ast::DateTimeField::Time => {}
                    sqltk_parser::ast::DateTimeField::Timezone => {}
                    sqltk_parser::ast::DateTimeField::TimezoneAbbr => {}
                    sqltk_parser::ast::DateTimeField::TimezoneHour => {}
                    sqltk_parser::ast::DateTimeField::TimezoneMinute => {}
                    sqltk_parser::ast::DateTimeField::TimezoneRegion => {}
                    sqltk_parser::ast::DateTimeField::NoDateTime => {}
                    sqltk_parser::ast::DateTimeField::Custom(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DateTimeField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Declare {
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
impl crate::AsNodeKey for sqltk_parser::ast::Declare {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DeclareAssignment {
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
                    sqltk_parser::ast::DeclareAssignment::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DeclareAssignment::Default(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DeclareAssignment::DuckAssignment(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DeclareAssignment::For(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::DeclareAssignment::MsSqlAssignment(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DeclareAssignment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DeclareType {
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
                    sqltk_parser::ast::DeclareType::Cursor => {}
                    sqltk_parser::ast::DeclareType::ResultSet => {}
                    sqltk_parser::ast::DeclareType::Exception => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DeclareType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Deduplicate {
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
                    sqltk_parser::ast::Deduplicate::All => {}
                    sqltk_parser::ast::Deduplicate::ByExpression(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Deduplicate {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DeferrableInitial {
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
                    sqltk_parser::ast::DeferrableInitial::Immediate => {}
                    sqltk_parser::ast::DeferrableInitial::Deferred => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DeferrableInitial {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Delete {
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
impl crate::AsNodeKey for sqltk_parser::ast::Delete {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DescribeAlias {
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
                    sqltk_parser::ast::DescribeAlias::Describe => {}
                    sqltk_parser::ast::DescribeAlias::Explain => {}
                    sqltk_parser::ast::DescribeAlias::Desc => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DescribeAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DictionaryField {
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
impl crate::AsNodeKey for sqltk_parser::ast::DictionaryField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DiscardObject {
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
                    sqltk_parser::ast::DiscardObject::ALL => {}
                    sqltk_parser::ast::DiscardObject::PLANS => {}
                    sqltk_parser::ast::DiscardObject::SEQUENCES => {}
                    sqltk_parser::ast::DiscardObject::TEMP => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DiscardObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Distinct {
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
                    sqltk_parser::ast::Distinct::Distinct => {}
                    sqltk_parser::ast::Distinct::On(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Distinct {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DoUpdate {
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
impl crate::AsNodeKey for sqltk_parser::ast::DoUpdate {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DollarQuotedString {
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
impl crate::AsNodeKey for sqltk_parser::ast::DollarQuotedString {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DropBehavior {
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
                    sqltk_parser::ast::DropBehavior::Restrict => {}
                    sqltk_parser::ast::DropBehavior::Cascade => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DropBehavior {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::DuplicateTreatment {
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
                    sqltk_parser::ast::DuplicateTreatment::Distinct => {}
                    sqltk_parser::ast::DuplicateTreatment::All => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::DuplicateTreatment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::EmptyMatchesMode {
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
                    sqltk_parser::ast::EmptyMatchesMode::Show => {}
                    sqltk_parser::ast::EmptyMatchesMode::Omit => {}
                    sqltk_parser::ast::EmptyMatchesMode::WithUnmatched => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::EmptyMatchesMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::EnumMember {
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
                    sqltk_parser::ast::EnumMember::Name(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::EnumMember::NamedValue(field0, field1) => {
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
impl crate::AsNodeKey for sqltk_parser::ast::EnumMember {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ExactNumberInfo {
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
                    sqltk_parser::ast::ExactNumberInfo::None => {}
                    sqltk_parser::ast::ExactNumberInfo::Precision(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ExactNumberInfo::PrecisionAndScale(
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
impl crate::AsNodeKey for sqltk_parser::ast::ExactNumberInfo {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ExceptSelectItem {
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
impl crate::AsNodeKey for sqltk_parser::ast::ExceptSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ExcludeSelectItem {
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
                    sqltk_parser::ast::ExcludeSelectItem::Single(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ExcludeSelectItem::Multiple(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ExcludeSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Expr {
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
                    sqltk_parser::ast::Expr::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::CompoundIdentifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::CompoundFieldAccess {
                        root,
                        access_chain,
                    } => {
                        root.accept(visitor)?;
                        access_chain.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::JsonAccess { value, path } => {
                        value.accept(visitor)?;
                        path.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsFalse(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNotFalse(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsTrue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNotTrue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNotNull(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsUnknown(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNotUnknown(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsDistinctFrom(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNotDistinctFrom(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IsNormalized { expr, form, negated } => {
                        expr.accept(visitor)?;
                        form.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::InList { expr, list, negated } => {
                        expr.accept(visitor)?;
                        list.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::InSubquery { expr, subquery, negated } => {
                        expr.accept(visitor)?;
                        subquery.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::InUnnest { expr, array_expr, negated } => {
                        expr.accept(visitor)?;
                        array_expr.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Between { expr, negated, low, high } => {
                        expr.accept(visitor)?;
                        negated.accept(visitor)?;
                        low.accept(visitor)?;
                        high.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::BinaryOp { left, op, right } => {
                        left.accept(visitor)?;
                        op.accept(visitor)?;
                        right.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Like {
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
                    sqltk_parser::ast::Expr::ILike {
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
                    sqltk_parser::ast::Expr::SimilarTo {
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
                    sqltk_parser::ast::Expr::RLike {
                        negated,
                        expr,
                        pattern,
                        regexp,
                    } => {
                        negated.accept(visitor)?;
                        expr.accept(visitor)?;
                        pattern.accept(visitor)?;
                        regexp.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::AnyOp {
                        left,
                        compare_op,
                        right,
                        is_some,
                    } => {
                        left.accept(visitor)?;
                        compare_op.accept(visitor)?;
                        right.accept(visitor)?;
                        is_some.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::AllOp { left, compare_op, right } => {
                        left.accept(visitor)?;
                        compare_op.accept(visitor)?;
                        right.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::UnaryOp { op, expr } => {
                        op.accept(visitor)?;
                        expr.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Convert {
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
                    sqltk_parser::ast::Expr::Cast { kind, expr, data_type, format } => {
                        kind.accept(visitor)?;
                        expr.accept(visitor)?;
                        data_type.accept(visitor)?;
                        format.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::AtTimeZone { timestamp, time_zone } => {
                        timestamp.accept(visitor)?;
                        time_zone.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Extract { field, syntax, expr } => {
                        field.accept(visitor)?;
                        syntax.accept(visitor)?;
                        expr.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Ceil { expr, field } => {
                        expr.accept(visitor)?;
                        field.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Floor { expr, field } => {
                        expr.accept(visitor)?;
                        field.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Position { expr, r#in } => {
                        expr.accept(visitor)?;
                        r#in.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Substring {
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
                    sqltk_parser::ast::Expr::Trim {
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
                    sqltk_parser::ast::Expr::Overlay {
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
                    sqltk_parser::ast::Expr::Collate { expr, collation } => {
                        expr.accept(visitor)?;
                        collation.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Nested(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Value(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::IntroducedString { introducer, value } => {
                        introducer.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::TypedString { data_type, value } => {
                        data_type.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Function(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Case {
                        operand,
                        conditions,
                        else_result,
                    } => {
                        operand.accept(visitor)?;
                        conditions.accept(visitor)?;
                        else_result.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Exists { subquery, negated } => {
                        subquery.accept(visitor)?;
                        negated.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Subquery(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::GroupingSets(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Cube(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Rollup(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Tuple(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Struct { values, fields } => {
                        values.accept(visitor)?;
                        fields.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Named { expr, name } => {
                        expr.accept(visitor)?;
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Dictionary(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Map(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Array(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Interval(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::MatchAgainst {
                        columns,
                        match_value,
                        opt_search_modifier,
                    } => {
                        columns.accept(visitor)?;
                        match_value.accept(visitor)?;
                        opt_search_modifier.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Wildcard(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::QualifiedWildcard(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::OuterJoin(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Prior(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Expr::Lambda(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Expr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ExprWithAlias {
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
impl crate::AsNodeKey for sqltk_parser::ast::ExprWithAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ExtractSyntax {
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
                    sqltk_parser::ast::ExtractSyntax::From => {}
                    sqltk_parser::ast::ExtractSyntax::Comma => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ExtractSyntax {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Fetch {
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
impl crate::AsNodeKey for sqltk_parser::ast::Fetch {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FetchDirection {
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
                    sqltk_parser::ast::FetchDirection::Count { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqltk_parser::ast::FetchDirection::Next => {}
                    sqltk_parser::ast::FetchDirection::Prior => {}
                    sqltk_parser::ast::FetchDirection::First => {}
                    sqltk_parser::ast::FetchDirection::Last => {}
                    sqltk_parser::ast::FetchDirection::Absolute { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqltk_parser::ast::FetchDirection::Relative { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqltk_parser::ast::FetchDirection::All => {}
                    sqltk_parser::ast::FetchDirection::Forward { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqltk_parser::ast::FetchDirection::ForwardAll => {}
                    sqltk_parser::ast::FetchDirection::Backward { limit } => {
                        limit.accept(visitor)?;
                    }
                    sqltk_parser::ast::FetchDirection::BackwardAll => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FetchDirection {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FileFormat {
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
                    sqltk_parser::ast::FileFormat::TEXTFILE => {}
                    sqltk_parser::ast::FileFormat::SEQUENCEFILE => {}
                    sqltk_parser::ast::FileFormat::ORC => {}
                    sqltk_parser::ast::FileFormat::PARQUET => {}
                    sqltk_parser::ast::FileFormat::AVRO => {}
                    sqltk_parser::ast::FileFormat::RCFILE => {}
                    sqltk_parser::ast::FileFormat::JSONFILE => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FileFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FlushLocation {
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
                    sqltk_parser::ast::FlushLocation::NoWriteToBinlog => {}
                    sqltk_parser::ast::FlushLocation::Local => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FlushLocation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FlushType {
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
                    sqltk_parser::ast::FlushType::BinaryLogs => {}
                    sqltk_parser::ast::FlushType::EngineLogs => {}
                    sqltk_parser::ast::FlushType::ErrorLogs => {}
                    sqltk_parser::ast::FlushType::GeneralLogs => {}
                    sqltk_parser::ast::FlushType::Hosts => {}
                    sqltk_parser::ast::FlushType::Logs => {}
                    sqltk_parser::ast::FlushType::Privileges => {}
                    sqltk_parser::ast::FlushType::OptimizerCosts => {}
                    sqltk_parser::ast::FlushType::RelayLogs => {}
                    sqltk_parser::ast::FlushType::SlowLogs => {}
                    sqltk_parser::ast::FlushType::Status => {}
                    sqltk_parser::ast::FlushType::UserResources => {}
                    sqltk_parser::ast::FlushType::Tables => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FlushType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ForClause {
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
                    sqltk_parser::ast::ForClause::Browse => {}
                    sqltk_parser::ast::ForClause::Json {
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
                    sqltk_parser::ast::ForClause::Xml {
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
impl crate::AsNodeKey for sqltk_parser::ast::ForClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ForJson {
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
                    sqltk_parser::ast::ForJson::Auto => {}
                    sqltk_parser::ast::ForJson::Path => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ForJson {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ForXml {
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
                    sqltk_parser::ast::ForXml::Raw(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ForXml::Auto => {}
                    sqltk_parser::ast::ForXml::Explicit => {}
                    sqltk_parser::ast::ForXml::Path(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ForXml {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FormatClause {
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
                    sqltk_parser::ast::FormatClause::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FormatClause::Null => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FormatClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FromTable {
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
                    sqltk_parser::ast::FromTable::WithFromKeyword(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FromTable::WithoutKeyword(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FromTable {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Function {
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
                self.uses_odbc_syntax.accept(visitor)?;
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
impl crate::AsNodeKey for sqltk_parser::ast::Function {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionArg {
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
                    sqltk_parser::ast::FunctionArg::Named { name, arg, operator } => {
                        name.accept(visitor)?;
                        arg.accept(visitor)?;
                        operator.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArg::ExprNamed {
                        name,
                        arg,
                        operator,
                    } => {
                        name.accept(visitor)?;
                        arg.accept(visitor)?;
                        operator.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArg::Unnamed(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionArg {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionArgExpr {
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
                    sqltk_parser::ast::FunctionArgExpr::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgExpr::QualifiedWildcard(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgExpr::Wildcard => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionArgExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionArgOperator {
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
                    sqltk_parser::ast::FunctionArgOperator::Equals => {}
                    sqltk_parser::ast::FunctionArgOperator::RightArrow => {}
                    sqltk_parser::ast::FunctionArgOperator::Assignment => {}
                    sqltk_parser::ast::FunctionArgOperator::Colon => {}
                    sqltk_parser::ast::FunctionArgOperator::Value => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionArgOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionArgumentClause {
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
                    sqltk_parser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgumentClause::OrderBy(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgumentClause::Limit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgumentClause::OnOverflow(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgumentClause::Having(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgumentClause::Separator(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArgumentClause::JsonNullClause(
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
impl crate::AsNodeKey for sqltk_parser::ast::FunctionArgumentClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionArgumentList {
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
impl crate::AsNodeKey for sqltk_parser::ast::FunctionArgumentList {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionArguments {
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
                    sqltk_parser::ast::FunctionArguments::None => {}
                    sqltk_parser::ast::FunctionArguments::Subquery(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::FunctionArguments::List(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionArguments {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionBehavior {
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
                    sqltk_parser::ast::FunctionBehavior::Immutable => {}
                    sqltk_parser::ast::FunctionBehavior::Stable => {}
                    sqltk_parser::ast::FunctionBehavior::Volatile => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionBehavior {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionCalledOnNull {
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
                    sqltk_parser::ast::FunctionCalledOnNull::CalledOnNullInput => {}
                    sqltk_parser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput => {}
                    sqltk_parser::ast::FunctionCalledOnNull::Strict => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionCalledOnNull {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionDesc {
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
impl crate::AsNodeKey for sqltk_parser::ast::FunctionDesc {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionDeterminismSpecifier {
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
                    sqltk_parser::ast::FunctionDeterminismSpecifier::Deterministic => {}
                    sqltk_parser::ast::FunctionDeterminismSpecifier::NotDeterministic => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionDeterminismSpecifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::FunctionParallel {
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
                    sqltk_parser::ast::FunctionParallel::Unsafe => {}
                    sqltk_parser::ast::FunctionParallel::Restricted => {}
                    sqltk_parser::ast::FunctionParallel::Safe => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::FunctionParallel {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GeneratedAs {
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
                    sqltk_parser::ast::GeneratedAs::Always => {}
                    sqltk_parser::ast::GeneratedAs::ByDefault => {}
                    sqltk_parser::ast::GeneratedAs::ExpStored => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GeneratedAs {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GeneratedExpressionMode {
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
                    sqltk_parser::ast::GeneratedExpressionMode::Virtual => {}
                    sqltk_parser::ast::GeneratedExpressionMode::Stored => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GeneratedExpressionMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GeometricTypeKind {
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
                    sqltk_parser::ast::GeometricTypeKind::Point => {}
                    sqltk_parser::ast::GeometricTypeKind::Line => {}
                    sqltk_parser::ast::GeometricTypeKind::LineSegment => {}
                    sqltk_parser::ast::GeometricTypeKind::GeometricBox => {}
                    sqltk_parser::ast::GeometricTypeKind::GeometricPath => {}
                    sqltk_parser::ast::GeometricTypeKind::Polygon => {}
                    sqltk_parser::ast::GeometricTypeKind::Circle => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GeometricTypeKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GrantObjects {
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
                    sqltk_parser::ast::GrantObjects::AllSequencesInSchema {
                        schemas,
                    } => {
                        schemas.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::AllTablesInSchema { schemas } => {
                        schemas.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Databases(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Schemas(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Sequences(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Tables(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Views(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Warehouses(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GrantObjects::Integrations(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GrantObjects {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Grantee {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.grantee_type.accept(visitor)?;
                self.name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Grantee {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GranteeName {
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
                    sqltk_parser::ast::GranteeName::ObjectName(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GranteeName::UserHost { user, host } => {
                        user.accept(visitor)?;
                        host.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GranteeName {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GranteesType {
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
                    sqltk_parser::ast::GranteesType::Role => {}
                    sqltk_parser::ast::GranteesType::Share => {}
                    sqltk_parser::ast::GranteesType::User => {}
                    sqltk_parser::ast::GranteesType::Group => {}
                    sqltk_parser::ast::GranteesType::Public => {}
                    sqltk_parser::ast::GranteesType::DatabaseRole => {}
                    sqltk_parser::ast::GranteesType::Application => {}
                    sqltk_parser::ast::GranteesType::ApplicationRole => {}
                    sqltk_parser::ast::GranteesType::None => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GranteesType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GroupByExpr {
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
                    sqltk_parser::ast::GroupByExpr::All(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::GroupByExpr::Expressions(field0, field1) => {
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
impl crate::AsNodeKey for sqltk_parser::ast::GroupByExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::GroupByWithModifier {
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
                    sqltk_parser::ast::GroupByWithModifier::Rollup => {}
                    sqltk_parser::ast::GroupByWithModifier::Cube => {}
                    sqltk_parser::ast::GroupByWithModifier::Totals => {}
                    sqltk_parser::ast::GroupByWithModifier::GroupingSets(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::GroupByWithModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HavingBound {
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
impl crate::AsNodeKey for sqltk_parser::ast::HavingBound {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HavingBoundKind {
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
                    sqltk_parser::ast::HavingBoundKind::Min => {}
                    sqltk_parser::ast::HavingBoundKind::Max => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HavingBoundKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveDelimiter {
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
                    sqltk_parser::ast::HiveDelimiter::FieldsTerminatedBy => {}
                    sqltk_parser::ast::HiveDelimiter::FieldsEscapedBy => {}
                    sqltk_parser::ast::HiveDelimiter::CollectionItemsTerminatedBy => {}
                    sqltk_parser::ast::HiveDelimiter::MapKeysTerminatedBy => {}
                    sqltk_parser::ast::HiveDelimiter::LinesTerminatedBy => {}
                    sqltk_parser::ast::HiveDelimiter::NullDefinedAs => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HiveDelimiter {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveDescribeFormat {
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
                    sqltk_parser::ast::HiveDescribeFormat::Extended => {}
                    sqltk_parser::ast::HiveDescribeFormat::Formatted => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HiveDescribeFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveDistributionStyle {
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
                    sqltk_parser::ast::HiveDistributionStyle::PARTITIONED {
                        columns,
                    } => {
                        columns.accept(visitor)?;
                    }
                    sqltk_parser::ast::HiveDistributionStyle::SKEWED {
                        columns,
                        on,
                        stored_as_directories,
                    } => {
                        columns.accept(visitor)?;
                        on.accept(visitor)?;
                        stored_as_directories.accept(visitor)?;
                    }
                    sqltk_parser::ast::HiveDistributionStyle::NONE => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HiveDistributionStyle {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveFormat {
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
impl crate::AsNodeKey for sqltk_parser::ast::HiveFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveIOFormat {
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
                    sqltk_parser::ast::HiveIOFormat::IOF {
                        input_format,
                        output_format,
                    } => {
                        input_format.accept(visitor)?;
                        output_format.accept(visitor)?;
                    }
                    sqltk_parser::ast::HiveIOFormat::FileFormat { format } => {
                        format.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HiveIOFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveLoadDataFormat {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.serde.accept(visitor)?;
                self.input_format.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HiveLoadDataFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveRowDelimiter {
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
impl crate::AsNodeKey for sqltk_parser::ast::HiveRowDelimiter {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveRowFormat {
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
                    sqltk_parser::ast::HiveRowFormat::SERDE { class } => {
                        class.accept(visitor)?;
                    }
                    sqltk_parser::ast::HiveRowFormat::DELIMITED { delimiters } => {
                        delimiters.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::HiveRowFormat {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::HiveSetLocation {
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
impl crate::AsNodeKey for sqltk_parser::ast::HiveSetLocation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Ident {
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
                self.span.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Ident {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IdentWithAlias {
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
impl crate::AsNodeKey for sqltk_parser::ast::IdentWithAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IdentityParameters {
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
impl crate::AsNodeKey for sqltk_parser::ast::IdentityParameters {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IdentityProperty {
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
impl crate::AsNodeKey for sqltk_parser::ast::IdentityProperty {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IdentityPropertyFormatKind {
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
                    sqltk_parser::ast::IdentityPropertyFormatKind::FunctionCall(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::IdentityPropertyFormatKind::StartAndIncrement(
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
impl crate::AsNodeKey for sqltk_parser::ast::IdentityPropertyFormatKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IdentityPropertyKind {
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
                    sqltk_parser::ast::IdentityPropertyKind::Autoincrement(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::IdentityPropertyKind::Identity(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::IdentityPropertyKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IdentityPropertyOrder {
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
                    sqltk_parser::ast::IdentityPropertyOrder::Order => {}
                    sqltk_parser::ast::IdentityPropertyOrder::NoOrder => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::IdentityPropertyOrder {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IlikeSelectItem {
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
impl crate::AsNodeKey for sqltk_parser::ast::IlikeSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IndexOption {
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
                    sqltk_parser::ast::IndexOption::Using(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::IndexOption::Comment(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::IndexOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::IndexType {
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
                    sqltk_parser::ast::IndexType::BTree => {}
                    sqltk_parser::ast::IndexType::Hash => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::IndexType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::InputFormatClause {
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
                self.values.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::InputFormatClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Insert {
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
                self.table.accept(visitor)?;
                self.table_alias.accept(visitor)?;
                self.columns.accept(visitor)?;
                self.overwrite.accept(visitor)?;
                self.source.accept(visitor)?;
                self.assignments.accept(visitor)?;
                self.partitioned.accept(visitor)?;
                self.after_columns.accept(visitor)?;
                self.has_table_keyword.accept(visitor)?;
                self.on.accept(visitor)?;
                self.returning.accept(visitor)?;
                self.replace_into.accept(visitor)?;
                self.priority.accept(visitor)?;
                self.insert_alias.accept(visitor)?;
                self.settings.accept(visitor)?;
                self.format_clause.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Insert {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::InsertAliases {
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
impl crate::AsNodeKey for sqltk_parser::ast::InsertAliases {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Interpolate {
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
impl crate::AsNodeKey for sqltk_parser::ast::Interpolate {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::InterpolateExpr {
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
impl crate::AsNodeKey for sqltk_parser::ast::InterpolateExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Interval {
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
impl crate::AsNodeKey for sqltk_parser::ast::Interval {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Join {
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
impl crate::AsNodeKey for sqltk_parser::ast::Join {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JoinConstraint {
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
                    sqltk_parser::ast::JoinConstraint::On(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinConstraint::Using(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinConstraint::Natural => {}
                    sqltk_parser::ast::JoinConstraint::None => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::JoinConstraint {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JoinOperator {
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
                    sqltk_parser::ast::JoinOperator::Join(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::Inner(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::Left(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::LeftOuter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::Right(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::RightOuter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::FullOuter(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::CrossJoin => {}
                    sqltk_parser::ast::JoinOperator::Semi(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::LeftSemi(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::RightSemi(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::Anti(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::LeftAnti(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::RightAnti(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JoinOperator::CrossApply => {}
                    sqltk_parser::ast::JoinOperator::OuterApply => {}
                    sqltk_parser::ast::JoinOperator::AsOf {
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
impl crate::AsNodeKey for sqltk_parser::ast::JoinOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonNullClause {
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
                    sqltk_parser::ast::JsonNullClause::NullOnNull => {}
                    sqltk_parser::ast::JsonNullClause::AbsentOnNull => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::JsonNullClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonPath {
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
impl crate::AsNodeKey for sqltk_parser::ast::JsonPath {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonPathElem {
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
                    sqltk_parser::ast::JsonPathElem::Dot { key, quoted } => {
                        key.accept(visitor)?;
                        quoted.accept(visitor)?;
                    }
                    sqltk_parser::ast::JsonPathElem::Bracket { key } => {
                        key.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::JsonPathElem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonTableColumn {
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
                    sqltk_parser::ast::JsonTableColumn::Named(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JsonTableColumn::ForOrdinality(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JsonTableColumn::Nested(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::JsonTableColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonTableColumnErrorHandling {
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
                    sqltk_parser::ast::JsonTableColumnErrorHandling::Null => {}
                    sqltk_parser::ast::JsonTableColumnErrorHandling::Default(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::JsonTableColumnErrorHandling::Error => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::JsonTableColumnErrorHandling {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonTableNamedColumn {
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
impl crate::AsNodeKey for sqltk_parser::ast::JsonTableNamedColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::JsonTableNestedColumn {
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
impl crate::AsNodeKey for sqltk_parser::ast::JsonTableNestedColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::KeyOrIndexDisplay {
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
                    sqltk_parser::ast::KeyOrIndexDisplay::None => {}
                    sqltk_parser::ast::KeyOrIndexDisplay::Key => {}
                    sqltk_parser::ast::KeyOrIndexDisplay::Index => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::KeyOrIndexDisplay {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::KillType {
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
                    sqltk_parser::ast::KillType::Connection => {}
                    sqltk_parser::ast::KillType::Query => {}
                    sqltk_parser::ast::KillType::Mutation => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::KillType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::LambdaFunction {
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
impl crate::AsNodeKey for sqltk_parser::ast::LambdaFunction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::LateralView {
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
impl crate::AsNodeKey for sqltk_parser::ast::LateralView {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ListAggOnOverflow {
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
                    sqltk_parser::ast::ListAggOnOverflow::Error => {}
                    sqltk_parser::ast::ListAggOnOverflow::Truncate {
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
impl crate::AsNodeKey for sqltk_parser::ast::ListAggOnOverflow {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::LockClause {
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
impl crate::AsNodeKey for sqltk_parser::ast::LockClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::LockTableType {
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
                    sqltk_parser::ast::LockTableType::Read { local } => {
                        local.accept(visitor)?;
                    }
                    sqltk_parser::ast::LockTableType::Write { low_priority } => {
                        low_priority.accept(visitor)?;
                    }
                    sqltk_parser::ast::LockTableType::AccessShare => {}
                    sqltk_parser::ast::LockTableType::RowShare => {}
                    sqltk_parser::ast::LockTableType::RowExclusive => {}
                    sqltk_parser::ast::LockTableType::ShareUpdateExclusive => {}
                    sqltk_parser::ast::LockTableType::Share => {}
                    sqltk_parser::ast::LockTableType::ShareRowExclusive => {}
                    sqltk_parser::ast::LockTableType::Exclusive => {}
                    sqltk_parser::ast::LockTableType::AccessExclusive => {}
                    _ => unreachable!(),
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::LockTableType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::LockTables {
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
                    sqltk_parser::ast::LockTables::MySql {
                        pluralized_table_keyword,
                        tables,
                    } => {
                        pluralized_table_keyword.accept(visitor)?;
                        tables.accept(visitor)?;
                    }
                    sqltk_parser::ast::LockTables::Postgres {
                        tables,
                        lock_mode,
                        has_table_keyword,
                        only,
                        no_wait,
                    } => {
                        tables.accept(visitor)?;
                        lock_mode.accept(visitor)?;
                        has_table_keyword.accept(visitor)?;
                        only.accept(visitor)?;
                        no_wait.accept(visitor)?;
                    }
                    _ => unreachable!(),
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::LockTables {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::LockType {
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
                    sqltk_parser::ast::LockType::Share => {}
                    sqltk_parser::ast::LockType::Update => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::LockType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MacroArg {
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
impl crate::AsNodeKey for sqltk_parser::ast::MacroArg {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MacroDefinition {
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
                    sqltk_parser::ast::MacroDefinition::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MacroDefinition::Table(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MacroDefinition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Map {
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
impl crate::AsNodeKey for sqltk_parser::ast::Map {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MapEntry {
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
impl crate::AsNodeKey for sqltk_parser::ast::MapEntry {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MatchRecognizePattern {
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
                    sqltk_parser::ast::MatchRecognizePattern::Symbol(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizePattern::Exclude(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizePattern::Permute(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizePattern::Concat(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizePattern::Group(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizePattern::Alternation(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizePattern::Repetition(
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
impl crate::AsNodeKey for sqltk_parser::ast::MatchRecognizePattern {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MatchRecognizeSymbol {
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
                    sqltk_parser::ast::MatchRecognizeSymbol::Named(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MatchRecognizeSymbol::Start => {}
                    sqltk_parser::ast::MatchRecognizeSymbol::End => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MatchRecognizeSymbol {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Measure {
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
impl crate::AsNodeKey for sqltk_parser::ast::Measure {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MergeAction {
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
                    sqltk_parser::ast::MergeAction::Insert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MergeAction::Update { assignments } => {
                        assignments.accept(visitor)?;
                    }
                    sqltk_parser::ast::MergeAction::Delete => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MergeAction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MergeClause {
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
impl crate::AsNodeKey for sqltk_parser::ast::MergeClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MergeClauseKind {
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
                    sqltk_parser::ast::MergeClauseKind::Matched => {}
                    sqltk_parser::ast::MergeClauseKind::NotMatched => {}
                    sqltk_parser::ast::MergeClauseKind::NotMatchedByTarget => {}
                    sqltk_parser::ast::MergeClauseKind::NotMatchedBySource => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MergeClauseKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MergeInsertExpr {
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
impl crate::AsNodeKey for sqltk_parser::ast::MergeInsertExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MergeInsertKind {
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
                    sqltk_parser::ast::MergeInsertKind::Values(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::MergeInsertKind::Row => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MergeInsertKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Method {
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
                self.method_chain.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Method {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MinMaxValue {
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
                    sqltk_parser::ast::MinMaxValue::Empty => {}
                    sqltk_parser::ast::MinMaxValue::None => {}
                    sqltk_parser::ast::MinMaxValue::Some(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MinMaxValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MySQLColumnPosition {
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
                    sqltk_parser::ast::MySQLColumnPosition::First => {}
                    sqltk_parser::ast::MySQLColumnPosition::After(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MySQLColumnPosition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MySqlTableLock {
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
impl crate::AsNodeKey for sqltk_parser::ast::MySqlTableLock {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::MysqlInsertPriority {
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
                    sqltk_parser::ast::MysqlInsertPriority::LowPriority => {}
                    sqltk_parser::ast::MysqlInsertPriority::Delayed => {}
                    sqltk_parser::ast::MysqlInsertPriority::HighPriority => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::MysqlInsertPriority {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::NamedWindowDefinition {
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
impl crate::AsNodeKey for sqltk_parser::ast::NamedWindowDefinition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::NamedWindowExpr {
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
                    sqltk_parser::ast::NamedWindowExpr::NamedWindow(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::NamedWindowExpr::WindowSpec(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::NamedWindowExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::NonBlock {
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
                    sqltk_parser::ast::NonBlock::Nowait => {}
                    sqltk_parser::ast::NonBlock::SkipLocked => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::NonBlock {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::NormalizationForm {
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
                    sqltk_parser::ast::NormalizationForm::NFC => {}
                    sqltk_parser::ast::NormalizationForm::NFD => {}
                    sqltk_parser::ast::NormalizationForm::NFKC => {}
                    sqltk_parser::ast::NormalizationForm::NFKD => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::NormalizationForm {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::NullTreatment {
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
                    sqltk_parser::ast::NullTreatment::IgnoreNulls => {}
                    sqltk_parser::ast::NullTreatment::RespectNulls => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::NullTreatment {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::NullsDistinctOption {
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
                    sqltk_parser::ast::NullsDistinctOption::None => {}
                    sqltk_parser::ast::NullsDistinctOption::Distinct => {}
                    sqltk_parser::ast::NullsDistinctOption::NotDistinct => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::NullsDistinctOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ObjectName {
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
impl crate::AsNodeKey for sqltk_parser::ast::ObjectName {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ObjectNamePart {
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
                    sqltk_parser::ast::ObjectNamePart::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ObjectNamePart {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ObjectType {
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
                    sqltk_parser::ast::ObjectType::Table => {}
                    sqltk_parser::ast::ObjectType::View => {}
                    sqltk_parser::ast::ObjectType::Index => {}
                    sqltk_parser::ast::ObjectType::Schema => {}
                    sqltk_parser::ast::ObjectType::Database => {}
                    sqltk_parser::ast::ObjectType::Role => {}
                    sqltk_parser::ast::ObjectType::Sequence => {}
                    sqltk_parser::ast::ObjectType::Stage => {}
                    sqltk_parser::ast::ObjectType::Type => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ObjectType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Offset {
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
impl crate::AsNodeKey for sqltk_parser::ast::Offset {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OffsetRows {
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
                    sqltk_parser::ast::OffsetRows::None => {}
                    sqltk_parser::ast::OffsetRows::Row => {}
                    sqltk_parser::ast::OffsetRows::Rows => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OffsetRows {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OnCommit {
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
                    sqltk_parser::ast::OnCommit::DeleteRows => {}
                    sqltk_parser::ast::OnCommit::PreserveRows => {}
                    sqltk_parser::ast::OnCommit::Drop => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OnCommit {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OnConflict {
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
impl crate::AsNodeKey for sqltk_parser::ast::OnConflict {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OnConflictAction {
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
                    sqltk_parser::ast::OnConflictAction::DoNothing => {}
                    sqltk_parser::ast::OnConflictAction::DoUpdate(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OnConflictAction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OnInsert {
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
                    sqltk_parser::ast::OnInsert::DuplicateKeyUpdate(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::OnInsert::OnConflict(field0) => {
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
impl crate::AsNodeKey for sqltk_parser::ast::OnInsert {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OpenJsonTableColumn {
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
                self.as_json.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OpenJsonTableColumn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OperateFunctionArg {
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
impl crate::AsNodeKey for sqltk_parser::ast::OperateFunctionArg {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OrderBy {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.kind.accept(visitor)?;
                self.interpolate.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OrderBy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OrderByExpr {
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
                self.options.accept(visitor)?;
                self.with_fill.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OrderByExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OrderByKind {
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
                    sqltk_parser::ast::OrderByKind::All(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::OrderByKind::Expressions(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OrderByKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::OrderByOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.asc.accept(visitor)?;
                self.nulls_first.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::OrderByOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Owner {
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
                    sqltk_parser::ast::Owner::Ident(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Owner::CurrentRole => {}
                    sqltk_parser::ast::Owner::CurrentUser => {}
                    sqltk_parser::ast::Owner::SessionUser => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Owner {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Partition {
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
                    sqltk_parser::ast::Partition::Identifier(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Partition::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Partition::Part(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Partition::Partitions(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Partition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::PartitionRangeDirection {
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
                    sqltk_parser::ast::PartitionRangeDirection::Left => {}
                    sqltk_parser::ast::PartitionRangeDirection::Right => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::PartitionRangeDirection {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Password {
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
                    sqltk_parser::ast::Password::Password(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Password::NullPassword => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Password {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::PivotValueSource {
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
                    sqltk_parser::ast::PivotValueSource::List(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::PivotValueSource::Any(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::PivotValueSource::Subquery(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::PivotValueSource {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Privileges {
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
                    sqltk_parser::ast::Privileges::All { with_privileges_keyword } => {
                        with_privileges_keyword.accept(visitor)?;
                    }
                    sqltk_parser::ast::Privileges::Actions(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Privileges {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ProcedureParam {
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
impl crate::AsNodeKey for sqltk_parser::ast::ProcedureParam {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ProjectionSelect {
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
impl crate::AsNodeKey for sqltk_parser::ast::ProjectionSelect {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Query {
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
impl crate::AsNodeKey for sqltk_parser::ast::Query {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RaisErrorOption {
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
                    sqltk_parser::ast::RaisErrorOption::Log => {}
                    sqltk_parser::ast::RaisErrorOption::NoWait => {}
                    sqltk_parser::ast::RaisErrorOption::SetError => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::RaisErrorOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ReferentialAction {
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
                    sqltk_parser::ast::ReferentialAction::Restrict => {}
                    sqltk_parser::ast::ReferentialAction::Cascade => {}
                    sqltk_parser::ast::ReferentialAction::SetNull => {}
                    sqltk_parser::ast::ReferentialAction::NoAction => {}
                    sqltk_parser::ast::ReferentialAction::SetDefault => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ReferentialAction {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RenameSelectItem {
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
                    sqltk_parser::ast::RenameSelectItem::Single(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RenameSelectItem::Multiple(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::RenameSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RenameTable {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.old_name.accept(visitor)?;
                self.new_name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::RenameTable {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RepetitionQuantifier {
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
                    sqltk_parser::ast::RepetitionQuantifier::ZeroOrMore => {}
                    sqltk_parser::ast::RepetitionQuantifier::OneOrMore => {}
                    sqltk_parser::ast::RepetitionQuantifier::AtMostOne => {}
                    sqltk_parser::ast::RepetitionQuantifier::Exactly(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RepetitionQuantifier::AtLeast(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RepetitionQuantifier::AtMost(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RepetitionQuantifier::Range(field0, field1) => {
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
impl crate::AsNodeKey for sqltk_parser::ast::RepetitionQuantifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ReplaceSelectElement {
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
impl crate::AsNodeKey for sqltk_parser::ast::ReplaceSelectElement {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ReplaceSelectItem {
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
impl crate::AsNodeKey for sqltk_parser::ast::ReplaceSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ResetConfig {
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
                    sqltk_parser::ast::ResetConfig::ALL => {}
                    sqltk_parser::ast::ResetConfig::ConfigName(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ResetConfig {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RoleOption {
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
                    sqltk_parser::ast::RoleOption::BypassRLS(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::ConnectionLimit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::CreateDB(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::CreateRole(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::Inherit(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::Login(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::Password(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::Replication(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::SuperUser(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::RoleOption::ValidUntil(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::RoleOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RowAccessPolicy {
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
impl crate::AsNodeKey for sqltk_parser::ast::RowAccessPolicy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::RowsPerMatch {
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
                    sqltk_parser::ast::RowsPerMatch::OneRow => {}
                    sqltk_parser::ast::RowsPerMatch::AllRows(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::RowsPerMatch {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SchemaName {
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
                    sqltk_parser::ast::SchemaName::Simple(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SchemaName::UnnamedAuthorization(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SchemaName::NamedAuthorization(
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
impl crate::AsNodeKey for sqltk_parser::ast::SchemaName {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SearchModifier {
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
                    sqltk_parser::ast::SearchModifier::InNaturalLanguageMode => {}
                    sqltk_parser::ast::SearchModifier::InNaturalLanguageModeWithQueryExpansion => {}
                    sqltk_parser::ast::SearchModifier::InBooleanMode => {}
                    sqltk_parser::ast::SearchModifier::WithQueryExpansion => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SearchModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SecondaryRoles {
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
                    sqltk_parser::ast::SecondaryRoles::All => {}
                    sqltk_parser::ast::SecondaryRoles::None => {}
                    sqltk_parser::ast::SecondaryRoles::List(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SecondaryRoles {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SecretOption {
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
impl crate::AsNodeKey for sqltk_parser::ast::SecretOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Select {
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
                self.select_token.accept(visitor)?;
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
                self.flavor.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Select {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SelectFlavor {
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
                    sqltk_parser::ast::SelectFlavor::Standard => {}
                    sqltk_parser::ast::SelectFlavor::FromFirst => {}
                    sqltk_parser::ast::SelectFlavor::FromFirstNoSelect => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SelectFlavor {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SelectInto {
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
impl crate::AsNodeKey for sqltk_parser::ast::SelectInto {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SelectItem {
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
                    sqltk_parser::ast::SelectItem::UnnamedExpr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                        expr.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqltk_parser::ast::SelectItem::QualifiedWildcard(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::SelectItem::Wildcard(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SelectItemQualifiedWildcardKind {
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
                    sqltk_parser::ast::SelectItemQualifiedWildcardKind::ObjectName(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SelectItemQualifiedWildcardKind::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SelectItemQualifiedWildcardKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SequenceOptions {
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
                    sqltk_parser::ast::SequenceOptions::IncrementBy(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::SequenceOptions::MinValue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SequenceOptions::MaxValue(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SequenceOptions::StartWith(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::SequenceOptions::Cache(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SequenceOptions::Cycle(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SequenceOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SessionParamStatsTopic {
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
                    sqltk_parser::ast::SessionParamStatsTopic::IO => {}
                    sqltk_parser::ast::SessionParamStatsTopic::Profile => {}
                    sqltk_parser::ast::SessionParamStatsTopic::Time => {}
                    sqltk_parser::ast::SessionParamStatsTopic::Xml => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SessionParamStatsTopic {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SessionParamValue {
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
                    sqltk_parser::ast::SessionParamValue::On => {}
                    sqltk_parser::ast::SessionParamValue::Off => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SessionParamValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetConfigValue {
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
                    sqltk_parser::ast::SetConfigValue::Default => {}
                    sqltk_parser::ast::SetConfigValue::FromCurrent => {}
                    sqltk_parser::ast::SetConfigValue::Value(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetConfigValue {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetExpr {
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
                    sqltk_parser::ast::SetExpr::Select(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetExpr::Query(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetExpr::SetOperation {
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
                    sqltk_parser::ast::SetExpr::Values(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetExpr::Insert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetExpr::Update(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetExpr::Table(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetExpr {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetOperator {
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
                    sqltk_parser::ast::SetOperator::Union => {}
                    sqltk_parser::ast::SetOperator::Except => {}
                    sqltk_parser::ast::SetOperator::Intersect => {}
                    sqltk_parser::ast::SetOperator::Minus => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetQuantifier {
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
                    sqltk_parser::ast::SetQuantifier::All => {}
                    sqltk_parser::ast::SetQuantifier::Distinct => {}
                    sqltk_parser::ast::SetQuantifier::ByName => {}
                    sqltk_parser::ast::SetQuantifier::AllByName => {}
                    sqltk_parser::ast::SetQuantifier::DistinctByName => {}
                    sqltk_parser::ast::SetQuantifier::None => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetQuantifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetSessionParamGeneric {
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
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetSessionParamGeneric {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetSessionParamIdentityInsert {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.obj.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetSessionParamIdentityInsert {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetSessionParamKind {
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
                    sqltk_parser::ast::SetSessionParamKind::Generic(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetSessionParamKind::IdentityInsert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetSessionParamKind::Offsets(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SetSessionParamKind::Statistics(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetSessionParamKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetSessionParamOffsets {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.keywords.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetSessionParamOffsets {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SetSessionParamStatistics {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.topic.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SetSessionParamStatistics {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Setting {
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
impl crate::AsNodeKey for sqltk_parser::ast::Setting {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowCreateObject {
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
                    sqltk_parser::ast::ShowCreateObject::Event => {}
                    sqltk_parser::ast::ShowCreateObject::Function => {}
                    sqltk_parser::ast::ShowCreateObject::Procedure => {}
                    sqltk_parser::ast::ShowCreateObject::Table => {}
                    sqltk_parser::ast::ShowCreateObject::Trigger => {}
                    sqltk_parser::ast::ShowCreateObject::View => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowCreateObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowObjects {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.terse.accept(visitor)?;
                self.show_options.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowObjects {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowStatementFilter {
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
                    sqltk_parser::ast::ShowStatementFilter::Like(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ShowStatementFilter::ILike(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ShowStatementFilter::Where(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ShowStatementFilter::NoKeyword(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowStatementFilter {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowStatementFilterPosition {
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
                    sqltk_parser::ast::ShowStatementFilterPosition::Infix(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::ShowStatementFilterPosition::Suffix(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowStatementFilterPosition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowStatementIn {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.clause.accept(visitor)?;
                self.parent_type.accept(visitor)?;
                self.parent_name.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowStatementIn {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowStatementInClause {
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
                    sqltk_parser::ast::ShowStatementInClause::IN => {}
                    sqltk_parser::ast::ShowStatementInClause::FROM => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowStatementInClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowStatementInParentType {
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
                    sqltk_parser::ast::ShowStatementInParentType::Account => {}
                    sqltk_parser::ast::ShowStatementInParentType::Database => {}
                    sqltk_parser::ast::ShowStatementInParentType::Schema => {}
                    sqltk_parser::ast::ShowStatementInParentType::Table => {}
                    sqltk_parser::ast::ShowStatementInParentType::View => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowStatementInParentType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ShowStatementOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.show_in.accept(visitor)?;
                self.starts_with.accept(visitor)?;
                self.limit.accept(visitor)?;
                self.limit_from.accept(visitor)?;
                self.filter_position.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ShowStatementOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SqlOption {
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
                    sqltk_parser::ast::SqlOption::Clustered(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SqlOption::Ident(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::SqlOption::KeyValue { key, value } => {
                        key.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqltk_parser::ast::SqlOption::Partition {
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
impl crate::AsNodeKey for sqltk_parser::ast::SqlOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SqliteOnConflict {
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
                    sqltk_parser::ast::SqliteOnConflict::Rollback => {}
                    sqltk_parser::ast::SqliteOnConflict::Abort => {}
                    sqltk_parser::ast::SqliteOnConflict::Fail => {}
                    sqltk_parser::ast::SqliteOnConflict::Ignore => {}
                    sqltk_parser::ast::SqliteOnConflict::Replace => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::SqliteOnConflict {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Statement {
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
                    sqltk_parser::ast::Statement::Analyze {
                        table_name,
                        partitions,
                        for_columns,
                        columns,
                        cache_metadata,
                        noscan,
                        compute_statistics,
                        has_table_keyword,
                    } => {
                        table_name.accept(visitor)?;
                        partitions.accept(visitor)?;
                        for_columns.accept(visitor)?;
                        columns.accept(visitor)?;
                        cache_metadata.accept(visitor)?;
                        noscan.accept(visitor)?;
                        compute_statistics.accept(visitor)?;
                        has_table_keyword.accept(visitor)?;
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
                        table_names.accept(visitor)?;
                        partitions.accept(visitor)?;
                        table.accept(visitor)?;
                        only.accept(visitor)?;
                        identity.accept(visitor)?;
                        cascade.accept(visitor)?;
                        on_cluster.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Msck {
                        table_name,
                        repair,
                        partition_action,
                    } => {
                        table_name.accept(visitor)?;
                        repair.accept(visitor)?;
                        partition_action.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Query(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Insert(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Install { extension_name } => {
                        extension_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Load { extension_name } => {
                        extension_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Directory {
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
                    sqltk_parser::ast::Statement::Call(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Copy {
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
                    sqltk_parser::ast::Statement::CopyIntoSnowflake {
                        kind,
                        into,
                        from_obj,
                        from_obj_alias,
                        stage_params,
                        from_transformations,
                        from_query,
                        files,
                        pattern,
                        file_format,
                        copy_options,
                        validation_mode,
                        partition,
                    } => {
                        kind.accept(visitor)?;
                        into.accept(visitor)?;
                        from_obj.accept(visitor)?;
                        from_obj_alias.accept(visitor)?;
                        stage_params.accept(visitor)?;
                        from_transformations.accept(visitor)?;
                        from_query.accept(visitor)?;
                        files.accept(visitor)?;
                        pattern.accept(visitor)?;
                        file_format.accept(visitor)?;
                        copy_options.accept(visitor)?;
                        validation_mode.accept(visitor)?;
                        partition.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Close { cursor } => {
                        cursor.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Update {
                        table,
                        assignments,
                        from,
                        selection,
                        returning,
                        or,
                    } => {
                        table.accept(visitor)?;
                        assignments.accept(visitor)?;
                        from.accept(visitor)?;
                        selection.accept(visitor)?;
                        returning.accept(visitor)?;
                        or.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Delete(field0) => {
                        field0.accept(visitor)?;
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
                        params,
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
                        params.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::CreateTable(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::CreateVirtualTable {
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
                    sqltk_parser::ast::Statement::CreateIndex(field0) => {
                        field0.accept(visitor)?;
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
                    sqltk_parser::ast::Statement::CreateSecret {
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
                    sqltk_parser::ast::Statement::CreatePolicy {
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
                    sqltk_parser::ast::Statement::CreateConnector(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AlterTable {
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
                    sqltk_parser::ast::Statement::AlterIndex { name, operation } => {
                        name.accept(visitor)?;
                        operation.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AlterView {
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
                    sqltk_parser::ast::Statement::AlterType(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AlterRole { name, operation } => {
                        name.accept(visitor)?;
                        operation.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AlterPolicy {
                        name,
                        table_name,
                        operation,
                    } => {
                        name.accept(visitor)?;
                        table_name.accept(visitor)?;
                        operation.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AlterConnector {
                        name,
                        properties,
                        url,
                        owner,
                    } => {
                        name.accept(visitor)?;
                        properties.accept(visitor)?;
                        url.accept(visitor)?;
                        owner.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AlterSession {
                        set,
                        session_params,
                    } => {
                        set.accept(visitor)?;
                        session_params.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AttachDatabase {
                        schema_name,
                        database_file_name,
                        database,
                    } => {
                        schema_name.accept(visitor)?;
                        database_file_name.accept(visitor)?;
                        database.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::AttachDuckDBDatabase {
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
                    sqltk_parser::ast::Statement::DetachDuckDBDatabase {
                        if_exists,
                        database,
                        database_alias,
                    } => {
                        if_exists.accept(visitor)?;
                        database.accept(visitor)?;
                        database_alias.accept(visitor)?;
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
                        object_type.accept(visitor)?;
                        if_exists.accept(visitor)?;
                        names.accept(visitor)?;
                        cascade.accept(visitor)?;
                        restrict.accept(visitor)?;
                        purge.accept(visitor)?;
                        temporary.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::DropFunction {
                        if_exists,
                        func_desc,
                        drop_behavior,
                    } => {
                        if_exists.accept(visitor)?;
                        func_desc.accept(visitor)?;
                        drop_behavior.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::DropProcedure {
                        if_exists,
                        proc_desc,
                        drop_behavior,
                    } => {
                        if_exists.accept(visitor)?;
                        proc_desc.accept(visitor)?;
                        drop_behavior.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::DropSecret {
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
                    sqltk_parser::ast::Statement::DropPolicy {
                        if_exists,
                        name,
                        table_name,
                        drop_behavior,
                    } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                        table_name.accept(visitor)?;
                        drop_behavior.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::DropConnector { if_exists, name } => {
                        if_exists.accept(visitor)?;
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Declare { stmts } => {
                        stmts.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::CreateExtension {
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
                    sqltk_parser::ast::Statement::DropExtension {
                        names,
                        if_exists,
                        cascade_or_restrict,
                    } => {
                        names.accept(visitor)?;
                        if_exists.accept(visitor)?;
                        cascade_or_restrict.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Fetch { name, direction, into } => {
                        name.accept(visitor)?;
                        direction.accept(visitor)?;
                        into.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Flush {
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
                    sqltk_parser::ast::Statement::Discard { object_type } => {
                        object_type.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::SetRole {
                        context_modifier,
                        role_name,
                    } => {
                        context_modifier.accept(visitor)?;
                        role_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::SetVariable {
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
                    sqltk_parser::ast::Statement::SetTimeZone { local, value } => {
                        local.accept(visitor)?;
                        value.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::SetNames {
                        charset_name,
                        collation_name,
                    } => {
                        charset_name.accept(visitor)?;
                        collation_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::SetNamesDefault {} => {}
                    sqltk_parser::ast::Statement::ShowFunctions { filter } => {
                        filter.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowVariable { variable } => {
                        variable.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowStatus {
                        filter,
                        global,
                        session,
                    } => {
                        filter.accept(visitor)?;
                        global.accept(visitor)?;
                        session.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowVariables {
                        filter,
                        global,
                        session,
                    } => {
                        filter.accept(visitor)?;
                        global.accept(visitor)?;
                        session.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowCreate { obj_type, obj_name } => {
                        obj_type.accept(visitor)?;
                        obj_name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowColumns {
                        extended,
                        full,
                        show_options,
                    } => {
                        extended.accept(visitor)?;
                        full.accept(visitor)?;
                        show_options.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowDatabases {
                        terse,
                        history,
                        show_options,
                    } => {
                        terse.accept(visitor)?;
                        history.accept(visitor)?;
                        show_options.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowSchemas {
                        terse,
                        history,
                        show_options,
                    } => {
                        terse.accept(visitor)?;
                        history.accept(visitor)?;
                        show_options.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowObjects(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowTables {
                        terse,
                        history,
                        extended,
                        full,
                        external,
                        show_options,
                    } => {
                        terse.accept(visitor)?;
                        history.accept(visitor)?;
                        extended.accept(visitor)?;
                        full.accept(visitor)?;
                        external.accept(visitor)?;
                        show_options.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowViews {
                        terse,
                        materialized,
                        show_options,
                    } => {
                        terse.accept(visitor)?;
                        materialized.accept(visitor)?;
                        show_options.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ShowCollation { filter } => {
                        filter.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Use(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::StartTransaction {
                        modes,
                        begin,
                        transaction,
                        modifier,
                        statements,
                        exception_statements,
                        has_end_keyword,
                    } => {
                        statements.accept(visitor)?;
                        exception_statements.accept(visitor)?;
                        modes.accept(visitor)?;
                        begin.accept(visitor)?;
                        transaction.accept(visitor)?;
                        modifier.accept(visitor)?;
                        has_end_keyword.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::SetTransaction {
                        modes,
                        snapshot,
                        session,
                    } => {
                        modes.accept(visitor)?;
                        snapshot.accept(visitor)?;
                        session.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Comment {
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
                    sqltk_parser::ast::Statement::Commit { chain, end, modifier } => {
                        chain.accept(visitor)?;
                        end.accept(visitor)?;
                        modifier.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Rollback { chain, savepoint } => {
                        chain.accept(visitor)?;
                        savepoint.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::CreateSchema {
                        schema_name,
                        if_not_exists,
                    } => {
                        schema_name.accept(visitor)?;
                        if_not_exists.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::CreateDatabase {
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
                    sqltk_parser::ast::Statement::CreateFunction(field0) => {
                        field0.accept(visitor)?;
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
                    sqltk_parser::ast::Statement::DropTrigger {
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
                    sqltk_parser::ast::Statement::CreateProcedure {
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
                    sqltk_parser::ast::Statement::CreateMacro {
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
                    sqltk_parser::ast::Statement::Assert { condition, message } => {
                        condition.accept(visitor)?;
                        message.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Grant {
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
                    sqltk_parser::ast::Statement::Revoke {
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
                    sqltk_parser::ast::Statement::Deallocate { name, prepare } => {
                        name.accept(visitor)?;
                        prepare.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Execute {
                        name,
                        parameters,
                        has_parentheses,
                        immediate,
                        into,
                        using,
                    } => {
                        name.accept(visitor)?;
                        parameters.accept(visitor)?;
                        has_parentheses.accept(visitor)?;
                        immediate.accept(visitor)?;
                        into.accept(visitor)?;
                        using.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Prepare {
                        name,
                        data_types,
                        statement,
                    } => {
                        statement.accept(visitor)?;
                        name.accept(visitor)?;
                        data_types.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Kill { modifier, id } => {
                        modifier.accept(visitor)?;
                        id.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ExplainTable {
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
                    sqltk_parser::ast::Statement::Explain {
                        describe_alias,
                        analyze,
                        verbose,
                        query_plan,
                        estimate,
                        statement,
                        format,
                        options,
                    } => {
                        statement.accept(visitor)?;
                        describe_alias.accept(visitor)?;
                        analyze.accept(visitor)?;
                        verbose.accept(visitor)?;
                        query_plan.accept(visitor)?;
                        estimate.accept(visitor)?;
                        format.accept(visitor)?;
                        options.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Savepoint { name } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::ReleaseSavepoint { name } => {
                        name.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Merge {
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
                    sqltk_parser::ast::Statement::Cache {
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
                    sqltk_parser::ast::Statement::UNCache { table_name, if_exists } => {
                        table_name.accept(visitor)?;
                        if_exists.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::CreateSequence {
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
                    sqltk_parser::ast::Statement::CreateType {
                        name,
                        representation,
                    } => {
                        name.accept(visitor)?;
                        representation.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Pragma { name, value, is_eq } => {
                        name.accept(visitor)?;
                        value.accept(visitor)?;
                        is_eq.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::LockTables(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::UnlockTables(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Unload { query, to, with } => {
                        query.accept(visitor)?;
                        to.accept(visitor)?;
                        with.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::OptimizeTable {
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
                    sqltk_parser::ast::Statement::LISTEN { channel } => {
                        channel.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::UNLISTEN { channel } => {
                        channel.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::NOTIFY { channel, payload } => {
                        channel.accept(visitor)?;
                        payload.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::LoadData {
                        local,
                        inpath,
                        overwrite,
                        table_name,
                        partitioned,
                        table_format,
                    } => {
                        local.accept(visitor)?;
                        inpath.accept(visitor)?;
                        overwrite.accept(visitor)?;
                        table_name.accept(visitor)?;
                        partitioned.accept(visitor)?;
                        table_format.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::RenameTable(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::List(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::Remove(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::SetSessionParam(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Statement::RaisError {
                        message,
                        severity,
                        state,
                        arguments,
                        options,
                    } => {
                        message.accept(visitor)?;
                        severity.accept(visitor)?;
                        state.accept(visitor)?;
                        arguments.accept(visitor)?;
                        options.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Statement {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::StorageSerializationPolicy {
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
                    sqltk_parser::ast::StorageSerializationPolicy::Compatible => {}
                    sqltk_parser::ast::StorageSerializationPolicy::Optimized => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::StorageSerializationPolicy {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::StructBracketKind {
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
                    sqltk_parser::ast::StructBracketKind::Parentheses => {}
                    sqltk_parser::ast::StructBracketKind::AngleBrackets => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::StructBracketKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::StructField {
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
impl crate::AsNodeKey for sqltk_parser::ast::StructField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Subscript {
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
                    sqltk_parser::ast::Subscript::Index { index } => {
                        index.accept(visitor)?;
                    }
                    sqltk_parser::ast::Subscript::Slice {
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
impl crate::AsNodeKey for sqltk_parser::ast::Subscript {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::SymbolDefinition {
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
impl crate::AsNodeKey for sqltk_parser::ast::SymbolDefinition {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Table {
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
impl crate::AsNodeKey for sqltk_parser::ast::Table {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableAlias {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableAlias {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableAliasColumnDef {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableAliasColumnDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableConstraint {
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
                        name.accept(visitor)?;
                        index_name.accept(visitor)?;
                        index_type_display.accept(visitor)?;
                        index_type.accept(visitor)?;
                        columns.accept(visitor)?;
                        index_options.accept(visitor)?;
                        characteristics.accept(visitor)?;
                        nulls_distinct.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableConstraint::PrimaryKey {
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
                    sqltk_parser::ast::TableConstraint::ForeignKey {
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
                    sqltk_parser::ast::TableConstraint::Check { name, expr } => {
                        name.accept(visitor)?;
                        expr.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableConstraint::Index {
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
                    sqltk_parser::ast::TableConstraint::FulltextOrSpatial {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableConstraint {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableEngine {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableEngine {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableFactor {
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
                    sqltk_parser::ast::TableFactor::Table {
                        name,
                        alias,
                        args,
                        with_hints,
                        version,
                        with_ordinality,
                        partitions,
                        json_path,
                        sample,
                        index_hints,
                    } => {
                        name.accept(visitor)?;
                        alias.accept(visitor)?;
                        args.accept(visitor)?;
                        with_hints.accept(visitor)?;
                        version.accept(visitor)?;
                        with_ordinality.accept(visitor)?;
                        partitions.accept(visitor)?;
                        json_path.accept(visitor)?;
                        sample.accept(visitor)?;
                        index_hints.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableFactor::Derived {
                        lateral,
                        subquery,
                        alias,
                    } => {
                        lateral.accept(visitor)?;
                        subquery.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableFactor::TableFunction { expr, alias } => {
                        expr.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableFactor::Function {
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
                    sqltk_parser::ast::TableFactor::UNNEST {
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
                    sqltk_parser::ast::TableFactor::JsonTable {
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
                    sqltk_parser::ast::TableFactor::OpenJsonTable {
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
                    sqltk_parser::ast::TableFactor::NestedJoin {
                        table_with_joins,
                        alias,
                    } => {
                        table_with_joins.accept(visitor)?;
                        alias.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableFactor::Pivot {
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
                    sqltk_parser::ast::TableFactor::Unpivot {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableFactor {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableFunctionArgs {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableFunctionArgs {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableIndexHintForClause {
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
                    sqltk_parser::ast::TableIndexHintForClause::Join => {}
                    sqltk_parser::ast::TableIndexHintForClause::OrderBy => {}
                    sqltk_parser::ast::TableIndexHintForClause::GroupBy => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableIndexHintForClause {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableIndexHintType {
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
                    sqltk_parser::ast::TableIndexHintType::Use => {}
                    sqltk_parser::ast::TableIndexHintType::Ignore => {}
                    sqltk_parser::ast::TableIndexHintType::Force => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableIndexHintType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableIndexHints {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.hint_type.accept(visitor)?;
                self.index_type.accept(visitor)?;
                self.for_clause.accept(visitor)?;
                self.index_names.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableIndexHints {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableIndexType {
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
                    sqltk_parser::ast::TableIndexType::Index => {}
                    sqltk_parser::ast::TableIndexType::Key => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableIndexType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableObject {
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
                    sqltk_parser::ast::TableObject::TableName(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableObject::TableFunction(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableOptionsClustered {
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
                    sqltk_parser::ast::TableOptionsClustered::ColumnstoreIndex => {}
                    sqltk_parser::ast::TableOptionsClustered::ColumnstoreIndexOrder(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableOptionsClustered::Index(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableOptionsClustered {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSample {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.modifier.accept(visitor)?;
                self.name.accept(visitor)?;
                self.quantity.accept(visitor)?;
                self.seed.accept(visitor)?;
                self.bucket.accept(visitor)?;
                self.offset.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSample {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleBucket {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.bucket.accept(visitor)?;
                self.total.accept(visitor)?;
                self.on.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleBucket {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleKind {
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
                    sqltk_parser::ast::TableSampleKind::BeforeTableAlias(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableSampleKind::AfterTableAlias(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleMethod {
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
                    sqltk_parser::ast::TableSampleMethod::Row => {}
                    sqltk_parser::ast::TableSampleMethod::Bernoulli => {}
                    sqltk_parser::ast::TableSampleMethod::System => {}
                    sqltk_parser::ast::TableSampleMethod::Block => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleMethod {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleModifier {
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
                    sqltk_parser::ast::TableSampleModifier::Sample => {}
                    sqltk_parser::ast::TableSampleModifier::TableSample => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleQuantity {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.parenthesized.accept(visitor)?;
                self.value.accept(visitor)?;
                self.unit.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleQuantity {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleSeed {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.modifier.accept(visitor)?;
                self.value.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleSeed {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleSeedModifier {
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
                    sqltk_parser::ast::TableSampleSeedModifier::Repeatable => {}
                    sqltk_parser::ast::TableSampleSeedModifier::Seed => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleSeedModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableSampleUnit {
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
                    sqltk_parser::ast::TableSampleUnit::Rows => {}
                    sqltk_parser::ast::TableSampleUnit::Percent => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableSampleUnit {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableVersion {
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
                    sqltk_parser::ast::TableVersion::ForSystemTimeAsOf(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TableVersion::Function(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TableVersion {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TableWithJoins {
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
impl crate::AsNodeKey for sqltk_parser::ast::TableWithJoins {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Tag {
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
impl crate::AsNodeKey for sqltk_parser::ast::Tag {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TagsColumnOption {
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
impl crate::AsNodeKey for sqltk_parser::ast::TagsColumnOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TimezoneInfo {
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
                    sqltk_parser::ast::TimezoneInfo::None => {}
                    sqltk_parser::ast::TimezoneInfo::WithTimeZone => {}
                    sqltk_parser::ast::TimezoneInfo::WithoutTimeZone => {}
                    sqltk_parser::ast::TimezoneInfo::Tz => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TimezoneInfo {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Top {
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
impl crate::AsNodeKey for sqltk_parser::ast::Top {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TopQuantity {
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
                    sqltk_parser::ast::TopQuantity::Expr(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TopQuantity::Constant(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TopQuantity {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TransactionAccessMode {
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
                    sqltk_parser::ast::TransactionAccessMode::ReadOnly => {}
                    sqltk_parser::ast::TransactionAccessMode::ReadWrite => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TransactionAccessMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TransactionIsolationLevel {
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
                    sqltk_parser::ast::TransactionIsolationLevel::ReadUncommitted => {}
                    sqltk_parser::ast::TransactionIsolationLevel::ReadCommitted => {}
                    sqltk_parser::ast::TransactionIsolationLevel::RepeatableRead => {}
                    sqltk_parser::ast::TransactionIsolationLevel::Serializable => {}
                    sqltk_parser::ast::TransactionIsolationLevel::Snapshot => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TransactionIsolationLevel {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TransactionMode {
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
                    sqltk_parser::ast::TransactionMode::AccessMode(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TransactionMode::IsolationLevel(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TransactionMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TransactionModifier {
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
                    sqltk_parser::ast::TransactionModifier::Deferred => {}
                    sqltk_parser::ast::TransactionModifier::Immediate => {}
                    sqltk_parser::ast::TransactionModifier::Exclusive => {}
                    sqltk_parser::ast::TransactionModifier::Try => {}
                    sqltk_parser::ast::TransactionModifier::Catch => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TransactionModifier {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerEvent {
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
                    sqltk_parser::ast::TriggerEvent::Insert => {}
                    sqltk_parser::ast::TriggerEvent::Update(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::TriggerEvent::Delete => {}
                    sqltk_parser::ast::TriggerEvent::Truncate => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TriggerEvent {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerExecBody {
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
impl crate::AsNodeKey for sqltk_parser::ast::TriggerExecBody {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerExecBodyType {
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
                    sqltk_parser::ast::TriggerExecBodyType::Function => {}
                    sqltk_parser::ast::TriggerExecBodyType::Procedure => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TriggerExecBodyType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerObject {
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
                    sqltk_parser::ast::TriggerObject::Row => {}
                    sqltk_parser::ast::TriggerObject::Statement => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TriggerObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerPeriod {
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
                    sqltk_parser::ast::TriggerPeriod::After => {}
                    sqltk_parser::ast::TriggerPeriod::Before => {}
                    sqltk_parser::ast::TriggerPeriod::InsteadOf => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TriggerPeriod {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerReferencing {
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
impl crate::AsNodeKey for sqltk_parser::ast::TriggerReferencing {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TriggerReferencingType {
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
                    sqltk_parser::ast::TriggerReferencingType::OldTable => {}
                    sqltk_parser::ast::TriggerReferencingType::NewTable => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TriggerReferencingType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TrimWhereField {
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
                    sqltk_parser::ast::TrimWhereField::Both => {}
                    sqltk_parser::ast::TrimWhereField::Leading => {}
                    sqltk_parser::ast::TrimWhereField::Trailing => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TrimWhereField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TruncateIdentityOption {
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
                    sqltk_parser::ast::TruncateIdentityOption::Restart => {}
                    sqltk_parser::ast::TruncateIdentityOption::Continue => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::TruncateIdentityOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::TruncateTableTarget {
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
impl crate::AsNodeKey for sqltk_parser::ast::TruncateTableTarget {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::UnaryOperator {
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
                    sqltk_parser::ast::UnaryOperator::Plus => {}
                    sqltk_parser::ast::UnaryOperator::Minus => {}
                    sqltk_parser::ast::UnaryOperator::Not => {}
                    sqltk_parser::ast::UnaryOperator::PGBitwiseNot => {}
                    sqltk_parser::ast::UnaryOperator::PGSquareRoot => {}
                    sqltk_parser::ast::UnaryOperator::PGCubeRoot => {}
                    sqltk_parser::ast::UnaryOperator::PGPostfixFactorial => {}
                    sqltk_parser::ast::UnaryOperator::PGPrefixFactorial => {}
                    sqltk_parser::ast::UnaryOperator::PGAbs => {}
                    sqltk_parser::ast::UnaryOperator::BangNot => {}
                    sqltk_parser::ast::UnaryOperator::Hash => {}
                    sqltk_parser::ast::UnaryOperator::AtDashAt => {}
                    sqltk_parser::ast::UnaryOperator::DoubleAt => {}
                    sqltk_parser::ast::UnaryOperator::QuestionDash => {}
                    sqltk_parser::ast::UnaryOperator::QuestionPipe => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::UnaryOperator {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::UnionField {
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
impl crate::AsNodeKey for sqltk_parser::ast::UnionField {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::UpdateTableFromKind {
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
                    sqltk_parser::ast::UpdateTableFromKind::BeforeSet(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::UpdateTableFromKind::AfterSet(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::UpdateTableFromKind {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Use {
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
                    sqltk_parser::ast::Use::Catalog(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::Schema(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::Database(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::Warehouse(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::Role(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::SecondaryRoles(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::Object(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Use::Default => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Use {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::UserDefinedTypeCompositeAttributeDef {
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
impl crate::AsNodeKey for sqltk_parser::ast::UserDefinedTypeCompositeAttributeDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::UserDefinedTypeRepresentation {
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
                    sqltk_parser::ast::UserDefinedTypeRepresentation::Composite {
                        attributes,
                    } => {
                        attributes.accept(visitor)?;
                    }
                    sqltk_parser::ast::UserDefinedTypeRepresentation::Enum {
                        labels,
                    } => {
                        labels.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::UserDefinedTypeRepresentation {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::UtilityOption {
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
impl crate::AsNodeKey for sqltk_parser::ast::UtilityOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Value {
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
                    sqltk_parser::ast::Value::Number(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::SingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::DollarQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::TripleSingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::TripleDoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::EscapedStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::UnicodeStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::SingleQuotedByteStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::DoubleQuotedByteStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::TripleSingleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::TripleDoubleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::SingleQuotedRawStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::DoubleQuotedRawStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::TripleSingleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::TripleDoubleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::NationalStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::HexStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::DoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::Boolean(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::Value::Null => {}
                    sqltk_parser::ast::Value::Placeholder(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::Value {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ValueTableMode {
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
                    sqltk_parser::ast::ValueTableMode::AsStruct => {}
                    sqltk_parser::ast::ValueTableMode::AsValue => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ValueTableMode {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ValueWithSpan {
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
                self.span.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::ValueWithSpan {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::Values {
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
impl crate::AsNodeKey for sqltk_parser::ast::Values {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::ViewColumnDef {
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
impl crate::AsNodeKey for sqltk_parser::ast::ViewColumnDef {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WildcardAdditionalOptions {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.wildcard_token.accept(visitor)?;
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
impl crate::AsNodeKey for sqltk_parser::ast::WildcardAdditionalOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WindowFrame {
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
impl crate::AsNodeKey for sqltk_parser::ast::WindowFrame {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WindowFrameBound {
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
                    sqltk_parser::ast::WindowFrameBound::CurrentRow => {}
                    sqltk_parser::ast::WindowFrameBound::Preceding(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::WindowFrameBound::Following(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::WindowFrameBound {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WindowFrameUnits {
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
                    sqltk_parser::ast::WindowFrameUnits::Rows => {}
                    sqltk_parser::ast::WindowFrameUnits::Range => {}
                    sqltk_parser::ast::WindowFrameUnits::Groups => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::WindowFrameUnits {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WindowSpec {
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
impl crate::AsNodeKey for sqltk_parser::ast::WindowSpec {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WindowType {
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
                    sqltk_parser::ast::WindowType::WindowSpec(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::ast::WindowType::NamedWindow(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::WindowType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::With {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.with_token.accept(visitor)?;
                self.recursive.accept(visitor)?;
                self.cte_tables.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::ast::With {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::WithFill {
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
impl crate::AsNodeKey for sqltk_parser::ast::WithFill {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::helpers::attached_token::AttachedToken {
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
impl crate::AsNodeKey for sqltk_parser::ast::helpers::attached_token::AttachedToken {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::ast::helpers::key_value_options::KeyValueOption {
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
impl crate::AsNodeKey for sqltk_parser::ast::helpers::key_value_options::KeyValueOption {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqltk_parser::ast::helpers::key_value_options::KeyValueOptionType {
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
                    sqltk_parser::ast::helpers::key_value_options::KeyValueOptionType::STRING => {}
                    sqltk_parser::ast::helpers::key_value_options::KeyValueOptionType::BOOLEAN => {}
                    sqltk_parser::ast::helpers::key_value_options::KeyValueOptionType::ENUM => {}
                    sqltk_parser::ast::helpers::key_value_options::KeyValueOptionType::NUMBER => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey
for sqltk_parser::ast::helpers::key_value_options::KeyValueOptionType {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqltk_parser::ast::helpers::key_value_options::KeyValueOptions {
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
for sqltk_parser::ast::helpers::key_value_options::KeyValueOptions {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqltk_parser::ast::helpers::stmt_data_loading::FileStagingCommand {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.stage.accept(visitor)?;
                self.pattern.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey
for sqltk_parser::ast::helpers::stmt_data_loading::FileStagingCommand {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqltk_parser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
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
for sqltk_parser::ast::helpers::stmt_data_loading::StageLoadSelectItem {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable
for sqltk_parser::ast::helpers::stmt_data_loading::StageParamsObject {
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
impl crate::AsNodeKey
for sqltk_parser::ast::helpers::stmt_data_loading::StageParamsObject {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::keywords::Keyword {
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
                    sqltk_parser::keywords::Keyword::NoKeyword => {}
                    sqltk_parser::keywords::Keyword::ABORT => {}
                    sqltk_parser::keywords::Keyword::ABS => {}
                    sqltk_parser::keywords::Keyword::ABSENT => {}
                    sqltk_parser::keywords::Keyword::ABSOLUTE => {}
                    sqltk_parser::keywords::Keyword::ACCESS => {}
                    sqltk_parser::keywords::Keyword::ACCOUNT => {}
                    sqltk_parser::keywords::Keyword::ACTION => {}
                    sqltk_parser::keywords::Keyword::ADD => {}
                    sqltk_parser::keywords::Keyword::ADMIN => {}
                    sqltk_parser::keywords::Keyword::AFTER => {}
                    sqltk_parser::keywords::Keyword::AGAINST => {}
                    sqltk_parser::keywords::Keyword::AGGREGATION => {}
                    sqltk_parser::keywords::Keyword::ALERT => {}
                    sqltk_parser::keywords::Keyword::ALGORITHM => {}
                    sqltk_parser::keywords::Keyword::ALIAS => {}
                    sqltk_parser::keywords::Keyword::ALL => {}
                    sqltk_parser::keywords::Keyword::ALLOCATE => {}
                    sqltk_parser::keywords::Keyword::ALTER => {}
                    sqltk_parser::keywords::Keyword::ALWAYS => {}
                    sqltk_parser::keywords::Keyword::ANALYZE => {}
                    sqltk_parser::keywords::Keyword::AND => {}
                    sqltk_parser::keywords::Keyword::ANTI => {}
                    sqltk_parser::keywords::Keyword::ANY => {}
                    sqltk_parser::keywords::Keyword::APPLICATION => {}
                    sqltk_parser::keywords::Keyword::APPLY => {}
                    sqltk_parser::keywords::Keyword::APPLYBUDGET => {}
                    sqltk_parser::keywords::Keyword::ARCHIVE => {}
                    sqltk_parser::keywords::Keyword::ARE => {}
                    sqltk_parser::keywords::Keyword::ARRAY => {}
                    sqltk_parser::keywords::Keyword::ARRAY_MAX_CARDINALITY => {}
                    sqltk_parser::keywords::Keyword::AS => {}
                    sqltk_parser::keywords::Keyword::ASC => {}
                    sqltk_parser::keywords::Keyword::ASENSITIVE => {}
                    sqltk_parser::keywords::Keyword::ASOF => {}
                    sqltk_parser::keywords::Keyword::ASSERT => {}
                    sqltk_parser::keywords::Keyword::ASYMMETRIC => {}
                    sqltk_parser::keywords::Keyword::AT => {}
                    sqltk_parser::keywords::Keyword::ATOMIC => {}
                    sqltk_parser::keywords::Keyword::ATTACH => {}
                    sqltk_parser::keywords::Keyword::AUDIT => {}
                    sqltk_parser::keywords::Keyword::AUTHENTICATION => {}
                    sqltk_parser::keywords::Keyword::AUTHORIZATION => {}
                    sqltk_parser::keywords::Keyword::AUTO => {}
                    sqltk_parser::keywords::Keyword::AUTOINCREMENT => {}
                    sqltk_parser::keywords::Keyword::AUTO_INCREMENT => {}
                    sqltk_parser::keywords::Keyword::AVG => {}
                    sqltk_parser::keywords::Keyword::AVRO => {}
                    sqltk_parser::keywords::Keyword::BACKWARD => {}
                    sqltk_parser::keywords::Keyword::BASE64 => {}
                    sqltk_parser::keywords::Keyword::BASE_LOCATION => {}
                    sqltk_parser::keywords::Keyword::BEFORE => {}
                    sqltk_parser::keywords::Keyword::BEGIN => {}
                    sqltk_parser::keywords::Keyword::BEGIN_FRAME => {}
                    sqltk_parser::keywords::Keyword::BEGIN_PARTITION => {}
                    sqltk_parser::keywords::Keyword::BERNOULLI => {}
                    sqltk_parser::keywords::Keyword::BETWEEN => {}
                    sqltk_parser::keywords::Keyword::BIGDECIMAL => {}
                    sqltk_parser::keywords::Keyword::BIGINT => {}
                    sqltk_parser::keywords::Keyword::BIGNUMERIC => {}
                    sqltk_parser::keywords::Keyword::BINARY => {}
                    sqltk_parser::keywords::Keyword::BIND => {}
                    sqltk_parser::keywords::Keyword::BINDING => {}
                    sqltk_parser::keywords::Keyword::BIT => {}
                    sqltk_parser::keywords::Keyword::BLOB => {}
                    sqltk_parser::keywords::Keyword::BLOCK => {}
                    sqltk_parser::keywords::Keyword::BLOOMFILTER => {}
                    sqltk_parser::keywords::Keyword::BOOL => {}
                    sqltk_parser::keywords::Keyword::BOOLEAN => {}
                    sqltk_parser::keywords::Keyword::BOTH => {}
                    sqltk_parser::keywords::Keyword::BOX => {}
                    sqltk_parser::keywords::Keyword::BROWSE => {}
                    sqltk_parser::keywords::Keyword::BTREE => {}
                    sqltk_parser::keywords::Keyword::BUCKET => {}
                    sqltk_parser::keywords::Keyword::BUCKETS => {}
                    sqltk_parser::keywords::Keyword::BY => {}
                    sqltk_parser::keywords::Keyword::BYPASSRLS => {}
                    sqltk_parser::keywords::Keyword::BYTEA => {}
                    sqltk_parser::keywords::Keyword::BYTES => {}
                    sqltk_parser::keywords::Keyword::CACHE => {}
                    sqltk_parser::keywords::Keyword::CALL => {}
                    sqltk_parser::keywords::Keyword::CALLED => {}
                    sqltk_parser::keywords::Keyword::CARDINALITY => {}
                    sqltk_parser::keywords::Keyword::CASCADE => {}
                    sqltk_parser::keywords::Keyword::CASCADED => {}
                    sqltk_parser::keywords::Keyword::CASE => {}
                    sqltk_parser::keywords::Keyword::CASES => {}
                    sqltk_parser::keywords::Keyword::CAST => {}
                    sqltk_parser::keywords::Keyword::CATALOG => {}
                    sqltk_parser::keywords::Keyword::CATALOG_SYNC => {}
                    sqltk_parser::keywords::Keyword::CATCH => {}
                    sqltk_parser::keywords::Keyword::CEIL => {}
                    sqltk_parser::keywords::Keyword::CEILING => {}
                    sqltk_parser::keywords::Keyword::CENTURY => {}
                    sqltk_parser::keywords::Keyword::CHAIN => {}
                    sqltk_parser::keywords::Keyword::CHANGE => {}
                    sqltk_parser::keywords::Keyword::CHANGE_TRACKING => {}
                    sqltk_parser::keywords::Keyword::CHANNEL => {}
                    sqltk_parser::keywords::Keyword::CHAR => {}
                    sqltk_parser::keywords::Keyword::CHARACTER => {}
                    sqltk_parser::keywords::Keyword::CHARACTERS => {}
                    sqltk_parser::keywords::Keyword::CHARACTER_LENGTH => {}
                    sqltk_parser::keywords::Keyword::CHARSET => {}
                    sqltk_parser::keywords::Keyword::CHAR_LENGTH => {}
                    sqltk_parser::keywords::Keyword::CHECK => {}
                    sqltk_parser::keywords::Keyword::CIRCLE => {}
                    sqltk_parser::keywords::Keyword::CLEAR => {}
                    sqltk_parser::keywords::Keyword::CLOB => {}
                    sqltk_parser::keywords::Keyword::CLONE => {}
                    sqltk_parser::keywords::Keyword::CLOSE => {}
                    sqltk_parser::keywords::Keyword::CLUSTER => {}
                    sqltk_parser::keywords::Keyword::CLUSTERED => {}
                    sqltk_parser::keywords::Keyword::CLUSTERING => {}
                    sqltk_parser::keywords::Keyword::COALESCE => {}
                    sqltk_parser::keywords::Keyword::COLLATE => {}
                    sqltk_parser::keywords::Keyword::COLLATION => {}
                    sqltk_parser::keywords::Keyword::COLLECT => {}
                    sqltk_parser::keywords::Keyword::COLLECTION => {}
                    sqltk_parser::keywords::Keyword::COLUMN => {}
                    sqltk_parser::keywords::Keyword::COLUMNS => {}
                    sqltk_parser::keywords::Keyword::COLUMNSTORE => {}
                    sqltk_parser::keywords::Keyword::COMMENT => {}
                    sqltk_parser::keywords::Keyword::COMMIT => {}
                    sqltk_parser::keywords::Keyword::COMMITTED => {}
                    sqltk_parser::keywords::Keyword::COMPATIBLE => {}
                    sqltk_parser::keywords::Keyword::COMPRESSION => {}
                    sqltk_parser::keywords::Keyword::COMPUTE => {}
                    sqltk_parser::keywords::Keyword::CONCURRENTLY => {}
                    sqltk_parser::keywords::Keyword::CONDITION => {}
                    sqltk_parser::keywords::Keyword::CONFLICT => {}
                    sqltk_parser::keywords::Keyword::CONNECT => {}
                    sqltk_parser::keywords::Keyword::CONNECTION => {}
                    sqltk_parser::keywords::Keyword::CONNECTOR => {}
                    sqltk_parser::keywords::Keyword::CONSTRAINT => {}
                    sqltk_parser::keywords::Keyword::CONTAINS => {}
                    sqltk_parser::keywords::Keyword::CONTINUE => {}
                    sqltk_parser::keywords::Keyword::CONVERT => {}
                    sqltk_parser::keywords::Keyword::COPY => {}
                    sqltk_parser::keywords::Keyword::COPY_OPTIONS => {}
                    sqltk_parser::keywords::Keyword::CORR => {}
                    sqltk_parser::keywords::Keyword::CORRESPONDING => {}
                    sqltk_parser::keywords::Keyword::COUNT => {}
                    sqltk_parser::keywords::Keyword::COVAR_POP => {}
                    sqltk_parser::keywords::Keyword::COVAR_SAMP => {}
                    sqltk_parser::keywords::Keyword::CREATE => {}
                    sqltk_parser::keywords::Keyword::CREATEDB => {}
                    sqltk_parser::keywords::Keyword::CREATEROLE => {}
                    sqltk_parser::keywords::Keyword::CREDENTIALS => {}
                    sqltk_parser::keywords::Keyword::CROSS => {}
                    sqltk_parser::keywords::Keyword::CSV => {}
                    sqltk_parser::keywords::Keyword::CUBE => {}
                    sqltk_parser::keywords::Keyword::CUME_DIST => {}
                    sqltk_parser::keywords::Keyword::CURRENT => {}
                    sqltk_parser::keywords::Keyword::CURRENT_CATALOG => {}
                    sqltk_parser::keywords::Keyword::CURRENT_DATE => {}
                    sqltk_parser::keywords::Keyword::CURRENT_DEFAULT_TRANSFORM_GROUP => {}
                    sqltk_parser::keywords::Keyword::CURRENT_PATH => {}
                    sqltk_parser::keywords::Keyword::CURRENT_ROLE => {}
                    sqltk_parser::keywords::Keyword::CURRENT_ROW => {}
                    sqltk_parser::keywords::Keyword::CURRENT_SCHEMA => {}
                    sqltk_parser::keywords::Keyword::CURRENT_TIME => {}
                    sqltk_parser::keywords::Keyword::CURRENT_TIMESTAMP => {}
                    sqltk_parser::keywords::Keyword::CURRENT_TRANSFORM_GROUP_FOR_TYPE => {}
                    sqltk_parser::keywords::Keyword::CURRENT_USER => {}
                    sqltk_parser::keywords::Keyword::CURSOR => {}
                    sqltk_parser::keywords::Keyword::CYCLE => {}
                    sqltk_parser::keywords::Keyword::DATA => {}
                    sqltk_parser::keywords::Keyword::DATABASE => {}
                    sqltk_parser::keywords::Keyword::DATABASES => {}
                    sqltk_parser::keywords::Keyword::DATA_RETENTION_TIME_IN_DAYS => {}
                    sqltk_parser::keywords::Keyword::DATE => {}
                    sqltk_parser::keywords::Keyword::DATE32 => {}
                    sqltk_parser::keywords::Keyword::DATETIME => {}
                    sqltk_parser::keywords::Keyword::DATETIME64 => {}
                    sqltk_parser::keywords::Keyword::DAY => {}
                    sqltk_parser::keywords::Keyword::DAYOFWEEK => {}
                    sqltk_parser::keywords::Keyword::DAYOFYEAR => {}
                    sqltk_parser::keywords::Keyword::DAYS => {}
                    sqltk_parser::keywords::Keyword::DCPROPERTIES => {}
                    sqltk_parser::keywords::Keyword::DEALLOCATE => {}
                    sqltk_parser::keywords::Keyword::DEC => {}
                    sqltk_parser::keywords::Keyword::DECADE => {}
                    sqltk_parser::keywords::Keyword::DECIMAL => {}
                    sqltk_parser::keywords::Keyword::DECLARE => {}
                    sqltk_parser::keywords::Keyword::DEDUPLICATE => {}
                    sqltk_parser::keywords::Keyword::DEFAULT => {}
                    sqltk_parser::keywords::Keyword::DEFAULT_DDL_COLLATION => {}
                    sqltk_parser::keywords::Keyword::DEFERRABLE => {}
                    sqltk_parser::keywords::Keyword::DEFERRED => {}
                    sqltk_parser::keywords::Keyword::DEFINE => {}
                    sqltk_parser::keywords::Keyword::DEFINED => {}
                    sqltk_parser::keywords::Keyword::DEFINER => {}
                    sqltk_parser::keywords::Keyword::DELAYED => {}
                    sqltk_parser::keywords::Keyword::DELETE => {}
                    sqltk_parser::keywords::Keyword::DELIMITED => {}
                    sqltk_parser::keywords::Keyword::DELIMITER => {}
                    sqltk_parser::keywords::Keyword::DELTA => {}
                    sqltk_parser::keywords::Keyword::DENSE_RANK => {}
                    sqltk_parser::keywords::Keyword::DEREF => {}
                    sqltk_parser::keywords::Keyword::DESC => {}
                    sqltk_parser::keywords::Keyword::DESCRIBE => {}
                    sqltk_parser::keywords::Keyword::DETACH => {}
                    sqltk_parser::keywords::Keyword::DETAIL => {}
                    sqltk_parser::keywords::Keyword::DETERMINISTIC => {}
                    sqltk_parser::keywords::Keyword::DIRECTORY => {}
                    sqltk_parser::keywords::Keyword::DISABLE => {}
                    sqltk_parser::keywords::Keyword::DISCARD => {}
                    sqltk_parser::keywords::Keyword::DISCONNECT => {}
                    sqltk_parser::keywords::Keyword::DISTINCT => {}
                    sqltk_parser::keywords::Keyword::DISTRIBUTE => {}
                    sqltk_parser::keywords::Keyword::DIV => {}
                    sqltk_parser::keywords::Keyword::DO => {}
                    sqltk_parser::keywords::Keyword::DOUBLE => {}
                    sqltk_parser::keywords::Keyword::DOW => {}
                    sqltk_parser::keywords::Keyword::DOY => {}
                    sqltk_parser::keywords::Keyword::DROP => {}
                    sqltk_parser::keywords::Keyword::DRY => {}
                    sqltk_parser::keywords::Keyword::DUPLICATE => {}
                    sqltk_parser::keywords::Keyword::DYNAMIC => {}
                    sqltk_parser::keywords::Keyword::EACH => {}
                    sqltk_parser::keywords::Keyword::ELEMENT => {}
                    sqltk_parser::keywords::Keyword::ELEMENTS => {}
                    sqltk_parser::keywords::Keyword::ELSE => {}
                    sqltk_parser::keywords::Keyword::EMPTY => {}
                    sqltk_parser::keywords::Keyword::ENABLE => {}
                    sqltk_parser::keywords::Keyword::ENABLE_SCHEMA_EVOLUTION => {}
                    sqltk_parser::keywords::Keyword::ENCODING => {}
                    sqltk_parser::keywords::Keyword::ENCRYPTION => {}
                    sqltk_parser::keywords::Keyword::END => {}
                    sqltk_parser::keywords::Keyword::END_EXEC => {}
                    sqltk_parser::keywords::Keyword::ENDPOINT => {}
                    sqltk_parser::keywords::Keyword::END_FRAME => {}
                    sqltk_parser::keywords::Keyword::END_PARTITION => {}
                    sqltk_parser::keywords::Keyword::ENFORCED => {}
                    sqltk_parser::keywords::Keyword::ENGINE => {}
                    sqltk_parser::keywords::Keyword::ENUM => {}
                    sqltk_parser::keywords::Keyword::ENUM16 => {}
                    sqltk_parser::keywords::Keyword::ENUM8 => {}
                    sqltk_parser::keywords::Keyword::EPHEMERAL => {}
                    sqltk_parser::keywords::Keyword::EPOCH => {}
                    sqltk_parser::keywords::Keyword::EQUALS => {}
                    sqltk_parser::keywords::Keyword::ERROR => {}
                    sqltk_parser::keywords::Keyword::ESCAPE => {}
                    sqltk_parser::keywords::Keyword::ESCAPED => {}
                    sqltk_parser::keywords::Keyword::ESTIMATE => {}
                    sqltk_parser::keywords::Keyword::EVENT => {}
                    sqltk_parser::keywords::Keyword::EVERY => {}
                    sqltk_parser::keywords::Keyword::EVOLVE => {}
                    sqltk_parser::keywords::Keyword::EXCEPT => {}
                    sqltk_parser::keywords::Keyword::EXCEPTION => {}
                    sqltk_parser::keywords::Keyword::EXCHANGE => {}
                    sqltk_parser::keywords::Keyword::EXCLUDE => {}
                    sqltk_parser::keywords::Keyword::EXCLUSIVE => {}
                    sqltk_parser::keywords::Keyword::EXEC => {}
                    sqltk_parser::keywords::Keyword::EXECUTE => {}
                    sqltk_parser::keywords::Keyword::EXECUTION => {}
                    sqltk_parser::keywords::Keyword::EXISTS => {}
                    sqltk_parser::keywords::Keyword::EXP => {}
                    sqltk_parser::keywords::Keyword::EXPANSION => {}
                    sqltk_parser::keywords::Keyword::EXPLAIN => {}
                    sqltk_parser::keywords::Keyword::EXPLICIT => {}
                    sqltk_parser::keywords::Keyword::EXPORT => {}
                    sqltk_parser::keywords::Keyword::EXTENDED => {}
                    sqltk_parser::keywords::Keyword::EXTENSION => {}
                    sqltk_parser::keywords::Keyword::EXTERNAL => {}
                    sqltk_parser::keywords::Keyword::EXTERNAL_VOLUME => {}
                    sqltk_parser::keywords::Keyword::EXTRACT => {}
                    sqltk_parser::keywords::Keyword::FAIL => {}
                    sqltk_parser::keywords::Keyword::FAILOVER => {}
                    sqltk_parser::keywords::Keyword::FALSE => {}
                    sqltk_parser::keywords::Keyword::FETCH => {}
                    sqltk_parser::keywords::Keyword::FIELDS => {}
                    sqltk_parser::keywords::Keyword::FILE => {}
                    sqltk_parser::keywords::Keyword::FILES => {}
                    sqltk_parser::keywords::Keyword::FILE_FORMAT => {}
                    sqltk_parser::keywords::Keyword::FILL => {}
                    sqltk_parser::keywords::Keyword::FILTER => {}
                    sqltk_parser::keywords::Keyword::FINAL => {}
                    sqltk_parser::keywords::Keyword::FIRST => {}
                    sqltk_parser::keywords::Keyword::FIRST_VALUE => {}
                    sqltk_parser::keywords::Keyword::FIXEDSTRING => {}
                    sqltk_parser::keywords::Keyword::FLOAT => {}
                    sqltk_parser::keywords::Keyword::FLOAT32 => {}
                    sqltk_parser::keywords::Keyword::FLOAT4 => {}
                    sqltk_parser::keywords::Keyword::FLOAT64 => {}
                    sqltk_parser::keywords::Keyword::FLOAT8 => {}
                    sqltk_parser::keywords::Keyword::FLOOR => {}
                    sqltk_parser::keywords::Keyword::FLUSH => {}
                    sqltk_parser::keywords::Keyword::FN => {}
                    sqltk_parser::keywords::Keyword::FOLLOWING => {}
                    sqltk_parser::keywords::Keyword::FOR => {}
                    sqltk_parser::keywords::Keyword::FORCE => {}
                    sqltk_parser::keywords::Keyword::FORCE_NOT_NULL => {}
                    sqltk_parser::keywords::Keyword::FORCE_NULL => {}
                    sqltk_parser::keywords::Keyword::FORCE_QUOTE => {}
                    sqltk_parser::keywords::Keyword::FOREIGN => {}
                    sqltk_parser::keywords::Keyword::FORMAT => {}
                    sqltk_parser::keywords::Keyword::FORMATTED => {}
                    sqltk_parser::keywords::Keyword::FORWARD => {}
                    sqltk_parser::keywords::Keyword::FRAME_ROW => {}
                    sqltk_parser::keywords::Keyword::FREE => {}
                    sqltk_parser::keywords::Keyword::FREEZE => {}
                    sqltk_parser::keywords::Keyword::FROM => {}
                    sqltk_parser::keywords::Keyword::FSCK => {}
                    sqltk_parser::keywords::Keyword::FULFILLMENT => {}
                    sqltk_parser::keywords::Keyword::FULL => {}
                    sqltk_parser::keywords::Keyword::FULLTEXT => {}
                    sqltk_parser::keywords::Keyword::FUNCTION => {}
                    sqltk_parser::keywords::Keyword::FUNCTIONS => {}
                    sqltk_parser::keywords::Keyword::FUSION => {}
                    sqltk_parser::keywords::Keyword::GENERAL => {}
                    sqltk_parser::keywords::Keyword::GENERATE => {}
                    sqltk_parser::keywords::Keyword::GENERATED => {}
                    sqltk_parser::keywords::Keyword::GEOGRAPHY => {}
                    sqltk_parser::keywords::Keyword::GET => {}
                    sqltk_parser::keywords::Keyword::GLOBAL => {}
                    sqltk_parser::keywords::Keyword::GRANT => {}
                    sqltk_parser::keywords::Keyword::GRANTED => {}
                    sqltk_parser::keywords::Keyword::GRANTS => {}
                    sqltk_parser::keywords::Keyword::GRAPHVIZ => {}
                    sqltk_parser::keywords::Keyword::GROUP => {}
                    sqltk_parser::keywords::Keyword::GROUPING => {}
                    sqltk_parser::keywords::Keyword::GROUPS => {}
                    sqltk_parser::keywords::Keyword::HASH => {}
                    sqltk_parser::keywords::Keyword::HAVING => {}
                    sqltk_parser::keywords::Keyword::HEADER => {}
                    sqltk_parser::keywords::Keyword::HEAP => {}
                    sqltk_parser::keywords::Keyword::HIGH_PRIORITY => {}
                    sqltk_parser::keywords::Keyword::HISTORY => {}
                    sqltk_parser::keywords::Keyword::HIVEVAR => {}
                    sqltk_parser::keywords::Keyword::HOLD => {}
                    sqltk_parser::keywords::Keyword::HOSTS => {}
                    sqltk_parser::keywords::Keyword::HOUR => {}
                    sqltk_parser::keywords::Keyword::HOURS => {}
                    sqltk_parser::keywords::Keyword::ICEBERG => {}
                    sqltk_parser::keywords::Keyword::ID => {}
                    sqltk_parser::keywords::Keyword::IDENTITY => {}
                    sqltk_parser::keywords::Keyword::IDENTITY_INSERT => {}
                    sqltk_parser::keywords::Keyword::IF => {}
                    sqltk_parser::keywords::Keyword::IGNORE => {}
                    sqltk_parser::keywords::Keyword::ILIKE => {}
                    sqltk_parser::keywords::Keyword::IMMEDIATE => {}
                    sqltk_parser::keywords::Keyword::IMMUTABLE => {}
                    sqltk_parser::keywords::Keyword::IMPORT => {}
                    sqltk_parser::keywords::Keyword::IMPORTED => {}
                    sqltk_parser::keywords::Keyword::IN => {}
                    sqltk_parser::keywords::Keyword::INCLUDE => {}
                    sqltk_parser::keywords::Keyword::INCLUDE_NULL_VALUES => {}
                    sqltk_parser::keywords::Keyword::INCREMENT => {}
                    sqltk_parser::keywords::Keyword::INDEX => {}
                    sqltk_parser::keywords::Keyword::INDICATOR => {}
                    sqltk_parser::keywords::Keyword::INHERIT => {}
                    sqltk_parser::keywords::Keyword::INITIALLY => {}
                    sqltk_parser::keywords::Keyword::INNER => {}
                    sqltk_parser::keywords::Keyword::INOUT => {}
                    sqltk_parser::keywords::Keyword::INPATH => {}
                    sqltk_parser::keywords::Keyword::INPLACE => {}
                    sqltk_parser::keywords::Keyword::INPUT => {}
                    sqltk_parser::keywords::Keyword::INPUTFORMAT => {}
                    sqltk_parser::keywords::Keyword::INSENSITIVE => {}
                    sqltk_parser::keywords::Keyword::INSERT => {}
                    sqltk_parser::keywords::Keyword::INSTALL => {}
                    sqltk_parser::keywords::Keyword::INSTANT => {}
                    sqltk_parser::keywords::Keyword::INSTEAD => {}
                    sqltk_parser::keywords::Keyword::INT => {}
                    sqltk_parser::keywords::Keyword::INT128 => {}
                    sqltk_parser::keywords::Keyword::INT16 => {}
                    sqltk_parser::keywords::Keyword::INT2 => {}
                    sqltk_parser::keywords::Keyword::INT256 => {}
                    sqltk_parser::keywords::Keyword::INT32 => {}
                    sqltk_parser::keywords::Keyword::INT4 => {}
                    sqltk_parser::keywords::Keyword::INT64 => {}
                    sqltk_parser::keywords::Keyword::INT8 => {}
                    sqltk_parser::keywords::Keyword::INTEGER => {}
                    sqltk_parser::keywords::Keyword::INTEGRATION => {}
                    sqltk_parser::keywords::Keyword::INTERPOLATE => {}
                    sqltk_parser::keywords::Keyword::INTERSECT => {}
                    sqltk_parser::keywords::Keyword::INTERSECTION => {}
                    sqltk_parser::keywords::Keyword::INTERVAL => {}
                    sqltk_parser::keywords::Keyword::INTO => {}
                    sqltk_parser::keywords::Keyword::INVOKER => {}
                    sqltk_parser::keywords::Keyword::IO => {}
                    sqltk_parser::keywords::Keyword::IS => {}
                    sqltk_parser::keywords::Keyword::ISODOW => {}
                    sqltk_parser::keywords::Keyword::ISOLATION => {}
                    sqltk_parser::keywords::Keyword::ISOWEEK => {}
                    sqltk_parser::keywords::Keyword::ISOYEAR => {}
                    sqltk_parser::keywords::Keyword::ITEMS => {}
                    sqltk_parser::keywords::Keyword::JAR => {}
                    sqltk_parser::keywords::Keyword::JOIN => {}
                    sqltk_parser::keywords::Keyword::JSON => {}
                    sqltk_parser::keywords::Keyword::JSONB => {}
                    sqltk_parser::keywords::Keyword::JSONFILE => {}
                    sqltk_parser::keywords::Keyword::JSON_TABLE => {}
                    sqltk_parser::keywords::Keyword::JULIAN => {}
                    sqltk_parser::keywords::Keyword::KEY => {}
                    sqltk_parser::keywords::Keyword::KEYS => {}
                    sqltk_parser::keywords::Keyword::KILL => {}
                    sqltk_parser::keywords::Keyword::LAG => {}
                    sqltk_parser::keywords::Keyword::LANGUAGE => {}
                    sqltk_parser::keywords::Keyword::LARGE => {}
                    sqltk_parser::keywords::Keyword::LAST => {}
                    sqltk_parser::keywords::Keyword::LAST_VALUE => {}
                    sqltk_parser::keywords::Keyword::LATERAL => {}
                    sqltk_parser::keywords::Keyword::LEAD => {}
                    sqltk_parser::keywords::Keyword::LEADING => {}
                    sqltk_parser::keywords::Keyword::LEFT => {}
                    sqltk_parser::keywords::Keyword::LEVEL => {}
                    sqltk_parser::keywords::Keyword::LIKE => {}
                    sqltk_parser::keywords::Keyword::LIKE_REGEX => {}
                    sqltk_parser::keywords::Keyword::LIMIT => {}
                    sqltk_parser::keywords::Keyword::LINE => {}
                    sqltk_parser::keywords::Keyword::LINES => {}
                    sqltk_parser::keywords::Keyword::LIST => {}
                    sqltk_parser::keywords::Keyword::LISTEN => {}
                    sqltk_parser::keywords::Keyword::LISTING => {}
                    sqltk_parser::keywords::Keyword::LN => {}
                    sqltk_parser::keywords::Keyword::LOAD => {}
                    sqltk_parser::keywords::Keyword::LOCAL => {}
                    sqltk_parser::keywords::Keyword::LOCALTIME => {}
                    sqltk_parser::keywords::Keyword::LOCALTIMESTAMP => {}
                    sqltk_parser::keywords::Keyword::LOCATION => {}
                    sqltk_parser::keywords::Keyword::LOCK => {}
                    sqltk_parser::keywords::Keyword::LOCKED => {}
                    sqltk_parser::keywords::Keyword::LOG => {}
                    sqltk_parser::keywords::Keyword::LOGIN => {}
                    sqltk_parser::keywords::Keyword::LOGS => {}
                    sqltk_parser::keywords::Keyword::LONGBLOB => {}
                    sqltk_parser::keywords::Keyword::LONGTEXT => {}
                    sqltk_parser::keywords::Keyword::LOWCARDINALITY => {}
                    sqltk_parser::keywords::Keyword::LOWER => {}
                    sqltk_parser::keywords::Keyword::LOW_PRIORITY => {}
                    sqltk_parser::keywords::Keyword::LS => {}
                    sqltk_parser::keywords::Keyword::LSEG => {}
                    sqltk_parser::keywords::Keyword::MACRO => {}
                    sqltk_parser::keywords::Keyword::MANAGE => {}
                    sqltk_parser::keywords::Keyword::MANAGED => {}
                    sqltk_parser::keywords::Keyword::MANAGEDLOCATION => {}
                    sqltk_parser::keywords::Keyword::MAP => {}
                    sqltk_parser::keywords::Keyword::MASKING => {}
                    sqltk_parser::keywords::Keyword::MATCH => {}
                    sqltk_parser::keywords::Keyword::MATCHED => {}
                    sqltk_parser::keywords::Keyword::MATCHES => {}
                    sqltk_parser::keywords::Keyword::MATCH_CONDITION => {}
                    sqltk_parser::keywords::Keyword::MATCH_RECOGNIZE => {}
                    sqltk_parser::keywords::Keyword::MATERIALIZE => {}
                    sqltk_parser::keywords::Keyword::MATERIALIZED => {}
                    sqltk_parser::keywords::Keyword::MAX => {}
                    sqltk_parser::keywords::Keyword::MAXVALUE => {}
                    sqltk_parser::keywords::Keyword::MAX_DATA_EXTENSION_TIME_IN_DAYS => {}
                    sqltk_parser::keywords::Keyword::MEASURES => {}
                    sqltk_parser::keywords::Keyword::MEDIUMBLOB => {}
                    sqltk_parser::keywords::Keyword::MEDIUMINT => {}
                    sqltk_parser::keywords::Keyword::MEDIUMTEXT => {}
                    sqltk_parser::keywords::Keyword::MEMBER => {}
                    sqltk_parser::keywords::Keyword::MERGE => {}
                    sqltk_parser::keywords::Keyword::METADATA => {}
                    sqltk_parser::keywords::Keyword::METHOD => {}
                    sqltk_parser::keywords::Keyword::METRIC => {}
                    sqltk_parser::keywords::Keyword::MICROSECOND => {}
                    sqltk_parser::keywords::Keyword::MICROSECONDS => {}
                    sqltk_parser::keywords::Keyword::MILLENIUM => {}
                    sqltk_parser::keywords::Keyword::MILLENNIUM => {}
                    sqltk_parser::keywords::Keyword::MILLISECOND => {}
                    sqltk_parser::keywords::Keyword::MILLISECONDS => {}
                    sqltk_parser::keywords::Keyword::MIN => {}
                    sqltk_parser::keywords::Keyword::MINUS => {}
                    sqltk_parser::keywords::Keyword::MINUTE => {}
                    sqltk_parser::keywords::Keyword::MINUTES => {}
                    sqltk_parser::keywords::Keyword::MINVALUE => {}
                    sqltk_parser::keywords::Keyword::MOD => {}
                    sqltk_parser::keywords::Keyword::MODE => {}
                    sqltk_parser::keywords::Keyword::MODIFIES => {}
                    sqltk_parser::keywords::Keyword::MODIFY => {}
                    sqltk_parser::keywords::Keyword::MODULE => {}
                    sqltk_parser::keywords::Keyword::MONITOR => {}
                    sqltk_parser::keywords::Keyword::MONTH => {}
                    sqltk_parser::keywords::Keyword::MONTHS => {}
                    sqltk_parser::keywords::Keyword::MSCK => {}
                    sqltk_parser::keywords::Keyword::MULTISET => {}
                    sqltk_parser::keywords::Keyword::MUTATION => {}
                    sqltk_parser::keywords::Keyword::NAME => {}
                    sqltk_parser::keywords::Keyword::NANOSECOND => {}
                    sqltk_parser::keywords::Keyword::NANOSECONDS => {}
                    sqltk_parser::keywords::Keyword::NATIONAL => {}
                    sqltk_parser::keywords::Keyword::NATURAL => {}
                    sqltk_parser::keywords::Keyword::NCHAR => {}
                    sqltk_parser::keywords::Keyword::NCLOB => {}
                    sqltk_parser::keywords::Keyword::NESTED => {}
                    sqltk_parser::keywords::Keyword::NETWORK => {}
                    sqltk_parser::keywords::Keyword::NEW => {}
                    sqltk_parser::keywords::Keyword::NEXT => {}
                    sqltk_parser::keywords::Keyword::NFC => {}
                    sqltk_parser::keywords::Keyword::NFD => {}
                    sqltk_parser::keywords::Keyword::NFKC => {}
                    sqltk_parser::keywords::Keyword::NFKD => {}
                    sqltk_parser::keywords::Keyword::NO => {}
                    sqltk_parser::keywords::Keyword::NOBYPASSRLS => {}
                    sqltk_parser::keywords::Keyword::NOCREATEDB => {}
                    sqltk_parser::keywords::Keyword::NOCREATEROLE => {}
                    sqltk_parser::keywords::Keyword::NOINHERIT => {}
                    sqltk_parser::keywords::Keyword::NOLOGIN => {}
                    sqltk_parser::keywords::Keyword::NONE => {}
                    sqltk_parser::keywords::Keyword::NOORDER => {}
                    sqltk_parser::keywords::Keyword::NOREPLICATION => {}
                    sqltk_parser::keywords::Keyword::NORMALIZE => {}
                    sqltk_parser::keywords::Keyword::NORMALIZED => {}
                    sqltk_parser::keywords::Keyword::NOSCAN => {}
                    sqltk_parser::keywords::Keyword::NOSUPERUSER => {}
                    sqltk_parser::keywords::Keyword::NOT => {}
                    sqltk_parser::keywords::Keyword::NOTHING => {}
                    sqltk_parser::keywords::Keyword::NOTIFY => {}
                    sqltk_parser::keywords::Keyword::NOWAIT => {}
                    sqltk_parser::keywords::Keyword::NO_WRITE_TO_BINLOG => {}
                    sqltk_parser::keywords::Keyword::NTH_VALUE => {}
                    sqltk_parser::keywords::Keyword::NTILE => {}
                    sqltk_parser::keywords::Keyword::NULL => {}
                    sqltk_parser::keywords::Keyword::NULLABLE => {}
                    sqltk_parser::keywords::Keyword::NULLIF => {}
                    sqltk_parser::keywords::Keyword::NULLS => {}
                    sqltk_parser::keywords::Keyword::NUMERIC => {}
                    sqltk_parser::keywords::Keyword::NVARCHAR => {}
                    sqltk_parser::keywords::Keyword::OBJECT => {}
                    sqltk_parser::keywords::Keyword::OBJECTS => {}
                    sqltk_parser::keywords::Keyword::OCCURRENCES_REGEX => {}
                    sqltk_parser::keywords::Keyword::OCTETS => {}
                    sqltk_parser::keywords::Keyword::OCTET_LENGTH => {}
                    sqltk_parser::keywords::Keyword::OF => {}
                    sqltk_parser::keywords::Keyword::OFF => {}
                    sqltk_parser::keywords::Keyword::OFFSET => {}
                    sqltk_parser::keywords::Keyword::OFFSETS => {}
                    sqltk_parser::keywords::Keyword::OLD => {}
                    sqltk_parser::keywords::Keyword::OMIT => {}
                    sqltk_parser::keywords::Keyword::ON => {}
                    sqltk_parser::keywords::Keyword::ONE => {}
                    sqltk_parser::keywords::Keyword::ONLY => {}
                    sqltk_parser::keywords::Keyword::OPEN => {}
                    sqltk_parser::keywords::Keyword::OPENJSON => {}
                    sqltk_parser::keywords::Keyword::OPERATE => {}
                    sqltk_parser::keywords::Keyword::OPERATOR => {}
                    sqltk_parser::keywords::Keyword::OPTIMIZATION => {}
                    sqltk_parser::keywords::Keyword::OPTIMIZE => {}
                    sqltk_parser::keywords::Keyword::OPTIMIZED => {}
                    sqltk_parser::keywords::Keyword::OPTIMIZER_COSTS => {}
                    sqltk_parser::keywords::Keyword::OPTION => {}
                    sqltk_parser::keywords::Keyword::OPTIONS => {}
                    sqltk_parser::keywords::Keyword::OR => {}
                    sqltk_parser::keywords::Keyword::ORC => {}
                    sqltk_parser::keywords::Keyword::ORDER => {}
                    sqltk_parser::keywords::Keyword::ORDINALITY => {}
                    sqltk_parser::keywords::Keyword::ORGANIZATION => {}
                    sqltk_parser::keywords::Keyword::OUT => {}
                    sqltk_parser::keywords::Keyword::OUTER => {}
                    sqltk_parser::keywords::Keyword::OUTPUTFORMAT => {}
                    sqltk_parser::keywords::Keyword::OVER => {}
                    sqltk_parser::keywords::Keyword::OVERFLOW => {}
                    sqltk_parser::keywords::Keyword::OVERLAPS => {}
                    sqltk_parser::keywords::Keyword::OVERLAY => {}
                    sqltk_parser::keywords::Keyword::OVERRIDE => {}
                    sqltk_parser::keywords::Keyword::OVERWRITE => {}
                    sqltk_parser::keywords::Keyword::OWNED => {}
                    sqltk_parser::keywords::Keyword::OWNER => {}
                    sqltk_parser::keywords::Keyword::OWNERSHIP => {}
                    sqltk_parser::keywords::Keyword::PACKAGE => {}
                    sqltk_parser::keywords::Keyword::PACKAGES => {}
                    sqltk_parser::keywords::Keyword::PARALLEL => {}
                    sqltk_parser::keywords::Keyword::PARAMETER => {}
                    sqltk_parser::keywords::Keyword::PARQUET => {}
                    sqltk_parser::keywords::Keyword::PART => {}
                    sqltk_parser::keywords::Keyword::PARTITION => {}
                    sqltk_parser::keywords::Keyword::PARTITIONED => {}
                    sqltk_parser::keywords::Keyword::PARTITIONS => {}
                    sqltk_parser::keywords::Keyword::PASSWORD => {}
                    sqltk_parser::keywords::Keyword::PAST => {}
                    sqltk_parser::keywords::Keyword::PATH => {}
                    sqltk_parser::keywords::Keyword::PATTERN => {}
                    sqltk_parser::keywords::Keyword::PER => {}
                    sqltk_parser::keywords::Keyword::PERCENT => {}
                    sqltk_parser::keywords::Keyword::PERCENTILE_CONT => {}
                    sqltk_parser::keywords::Keyword::PERCENTILE_DISC => {}
                    sqltk_parser::keywords::Keyword::PERCENT_RANK => {}
                    sqltk_parser::keywords::Keyword::PERIOD => {}
                    sqltk_parser::keywords::Keyword::PERMISSIVE => {}
                    sqltk_parser::keywords::Keyword::PERSISTENT => {}
                    sqltk_parser::keywords::Keyword::PIVOT => {}
                    sqltk_parser::keywords::Keyword::PLACING => {}
                    sqltk_parser::keywords::Keyword::PLAN => {}
                    sqltk_parser::keywords::Keyword::PLANS => {}
                    sqltk_parser::keywords::Keyword::POINT => {}
                    sqltk_parser::keywords::Keyword::POLICY => {}
                    sqltk_parser::keywords::Keyword::POLYGON => {}
                    sqltk_parser::keywords::Keyword::POOL => {}
                    sqltk_parser::keywords::Keyword::PORTION => {}
                    sqltk_parser::keywords::Keyword::POSITION => {}
                    sqltk_parser::keywords::Keyword::POSITION_REGEX => {}
                    sqltk_parser::keywords::Keyword::POWER => {}
                    sqltk_parser::keywords::Keyword::PRAGMA => {}
                    sqltk_parser::keywords::Keyword::PRECEDES => {}
                    sqltk_parser::keywords::Keyword::PRECEDING => {}
                    sqltk_parser::keywords::Keyword::PRECISION => {}
                    sqltk_parser::keywords::Keyword::PREPARE => {}
                    sqltk_parser::keywords::Keyword::PRESERVE => {}
                    sqltk_parser::keywords::Keyword::PREWHERE => {}
                    sqltk_parser::keywords::Keyword::PRIMARY => {}
                    sqltk_parser::keywords::Keyword::PRIOR => {}
                    sqltk_parser::keywords::Keyword::PRIVILEGES => {}
                    sqltk_parser::keywords::Keyword::PROCEDURE => {}
                    sqltk_parser::keywords::Keyword::PROFILE => {}
                    sqltk_parser::keywords::Keyword::PROGRAM => {}
                    sqltk_parser::keywords::Keyword::PROJECTION => {}
                    sqltk_parser::keywords::Keyword::PUBLIC => {}
                    sqltk_parser::keywords::Keyword::PURCHASE => {}
                    sqltk_parser::keywords::Keyword::PURGE => {}
                    sqltk_parser::keywords::Keyword::QUALIFY => {}
                    sqltk_parser::keywords::Keyword::QUARTER => {}
                    sqltk_parser::keywords::Keyword::QUERY => {}
                    sqltk_parser::keywords::Keyword::QUOTE => {}
                    sqltk_parser::keywords::Keyword::RAISERROR => {}
                    sqltk_parser::keywords::Keyword::RANGE => {}
                    sqltk_parser::keywords::Keyword::RANK => {}
                    sqltk_parser::keywords::Keyword::RAW => {}
                    sqltk_parser::keywords::Keyword::RCFILE => {}
                    sqltk_parser::keywords::Keyword::READ => {}
                    sqltk_parser::keywords::Keyword::READS => {}
                    sqltk_parser::keywords::Keyword::READ_ONLY => {}
                    sqltk_parser::keywords::Keyword::REAL => {}
                    sqltk_parser::keywords::Keyword::RECLUSTER => {}
                    sqltk_parser::keywords::Keyword::RECURSIVE => {}
                    sqltk_parser::keywords::Keyword::REF => {}
                    sqltk_parser::keywords::Keyword::REFERENCES => {}
                    sqltk_parser::keywords::Keyword::REFERENCING => {}
                    sqltk_parser::keywords::Keyword::REGCLASS => {}
                    sqltk_parser::keywords::Keyword::REGEXP => {}
                    sqltk_parser::keywords::Keyword::REGR_AVGX => {}
                    sqltk_parser::keywords::Keyword::REGR_AVGY => {}
                    sqltk_parser::keywords::Keyword::REGR_COUNT => {}
                    sqltk_parser::keywords::Keyword::REGR_INTERCEPT => {}
                    sqltk_parser::keywords::Keyword::REGR_R2 => {}
                    sqltk_parser::keywords::Keyword::REGR_SLOPE => {}
                    sqltk_parser::keywords::Keyword::REGR_SXX => {}
                    sqltk_parser::keywords::Keyword::REGR_SXY => {}
                    sqltk_parser::keywords::Keyword::REGR_SYY => {}
                    sqltk_parser::keywords::Keyword::RELATIVE => {}
                    sqltk_parser::keywords::Keyword::RELAY => {}
                    sqltk_parser::keywords::Keyword::RELEASE => {}
                    sqltk_parser::keywords::Keyword::RELEASES => {}
                    sqltk_parser::keywords::Keyword::REMOTE => {}
                    sqltk_parser::keywords::Keyword::REMOVE => {}
                    sqltk_parser::keywords::Keyword::RENAME => {}
                    sqltk_parser::keywords::Keyword::REORG => {}
                    sqltk_parser::keywords::Keyword::REPAIR => {}
                    sqltk_parser::keywords::Keyword::REPEATABLE => {}
                    sqltk_parser::keywords::Keyword::REPLACE => {}
                    sqltk_parser::keywords::Keyword::REPLICA => {}
                    sqltk_parser::keywords::Keyword::REPLICATE => {}
                    sqltk_parser::keywords::Keyword::REPLICATION => {}
                    sqltk_parser::keywords::Keyword::RESET => {}
                    sqltk_parser::keywords::Keyword::RESOLVE => {}
                    sqltk_parser::keywords::Keyword::RESPECT => {}
                    sqltk_parser::keywords::Keyword::RESTART => {}
                    sqltk_parser::keywords::Keyword::RESTRICT => {}
                    sqltk_parser::keywords::Keyword::RESTRICTED => {}
                    sqltk_parser::keywords::Keyword::RESTRICTIONS => {}
                    sqltk_parser::keywords::Keyword::RESTRICTIVE => {}
                    sqltk_parser::keywords::Keyword::RESULT => {}
                    sqltk_parser::keywords::Keyword::RESULTSET => {}
                    sqltk_parser::keywords::Keyword::RESUME => {}
                    sqltk_parser::keywords::Keyword::RETAIN => {}
                    sqltk_parser::keywords::Keyword::RETURN => {}
                    sqltk_parser::keywords::Keyword::RETURNING => {}
                    sqltk_parser::keywords::Keyword::RETURNS => {}
                    sqltk_parser::keywords::Keyword::REVOKE => {}
                    sqltk_parser::keywords::Keyword::RIGHT => {}
                    sqltk_parser::keywords::Keyword::RLIKE => {}
                    sqltk_parser::keywords::Keyword::RM => {}
                    sqltk_parser::keywords::Keyword::ROLE => {}
                    sqltk_parser::keywords::Keyword::ROLES => {}
                    sqltk_parser::keywords::Keyword::ROLLBACK => {}
                    sqltk_parser::keywords::Keyword::ROLLUP => {}
                    sqltk_parser::keywords::Keyword::ROOT => {}
                    sqltk_parser::keywords::Keyword::ROW => {}
                    sqltk_parser::keywords::Keyword::ROWID => {}
                    sqltk_parser::keywords::Keyword::ROWS => {}
                    sqltk_parser::keywords::Keyword::ROW_NUMBER => {}
                    sqltk_parser::keywords::Keyword::RULE => {}
                    sqltk_parser::keywords::Keyword::RUN => {}
                    sqltk_parser::keywords::Keyword::SAFE => {}
                    sqltk_parser::keywords::Keyword::SAFE_CAST => {}
                    sqltk_parser::keywords::Keyword::SAMPLE => {}
                    sqltk_parser::keywords::Keyword::SAVEPOINT => {}
                    sqltk_parser::keywords::Keyword::SCHEMA => {}
                    sqltk_parser::keywords::Keyword::SCHEMAS => {}
                    sqltk_parser::keywords::Keyword::SCOPE => {}
                    sqltk_parser::keywords::Keyword::SCROLL => {}
                    sqltk_parser::keywords::Keyword::SEARCH => {}
                    sqltk_parser::keywords::Keyword::SECOND => {}
                    sqltk_parser::keywords::Keyword::SECONDARY => {}
                    sqltk_parser::keywords::Keyword::SECONDS => {}
                    sqltk_parser::keywords::Keyword::SECRET => {}
                    sqltk_parser::keywords::Keyword::SECURITY => {}
                    sqltk_parser::keywords::Keyword::SEED => {}
                    sqltk_parser::keywords::Keyword::SELECT => {}
                    sqltk_parser::keywords::Keyword::SEMI => {}
                    sqltk_parser::keywords::Keyword::SENSITIVE => {}
                    sqltk_parser::keywords::Keyword::SEPARATOR => {}
                    sqltk_parser::keywords::Keyword::SEQUENCE => {}
                    sqltk_parser::keywords::Keyword::SEQUENCEFILE => {}
                    sqltk_parser::keywords::Keyword::SEQUENCES => {}
                    sqltk_parser::keywords::Keyword::SERDE => {}
                    sqltk_parser::keywords::Keyword::SERDEPROPERTIES => {}
                    sqltk_parser::keywords::Keyword::SERIALIZABLE => {}
                    sqltk_parser::keywords::Keyword::SERVICE => {}
                    sqltk_parser::keywords::Keyword::SESSION => {}
                    sqltk_parser::keywords::Keyword::SESSION_USER => {}
                    sqltk_parser::keywords::Keyword::SET => {}
                    sqltk_parser::keywords::Keyword::SETERROR => {}
                    sqltk_parser::keywords::Keyword::SETS => {}
                    sqltk_parser::keywords::Keyword::SETTINGS => {}
                    sqltk_parser::keywords::Keyword::SHARE => {}
                    sqltk_parser::keywords::Keyword::SHARING => {}
                    sqltk_parser::keywords::Keyword::SHOW => {}
                    sqltk_parser::keywords::Keyword::SIGNED => {}
                    sqltk_parser::keywords::Keyword::SIMILAR => {}
                    sqltk_parser::keywords::Keyword::SKIP => {}
                    sqltk_parser::keywords::Keyword::SLOW => {}
                    sqltk_parser::keywords::Keyword::SMALLINT => {}
                    sqltk_parser::keywords::Keyword::SNAPSHOT => {}
                    sqltk_parser::keywords::Keyword::SOME => {}
                    sqltk_parser::keywords::Keyword::SORT => {}
                    sqltk_parser::keywords::Keyword::SORTED => {}
                    sqltk_parser::keywords::Keyword::SOURCE => {}
                    sqltk_parser::keywords::Keyword::SPATIAL => {}
                    sqltk_parser::keywords::Keyword::SPECIFIC => {}
                    sqltk_parser::keywords::Keyword::SPECIFICTYPE => {}
                    sqltk_parser::keywords::Keyword::SQL => {}
                    sqltk_parser::keywords::Keyword::SQLEXCEPTION => {}
                    sqltk_parser::keywords::Keyword::SQLSTATE => {}
                    sqltk_parser::keywords::Keyword::SQLWARNING => {}
                    sqltk_parser::keywords::Keyword::SQRT => {}
                    sqltk_parser::keywords::Keyword::STABLE => {}
                    sqltk_parser::keywords::Keyword::STAGE => {}
                    sqltk_parser::keywords::Keyword::START => {}
                    sqltk_parser::keywords::Keyword::STARTS => {}
                    sqltk_parser::keywords::Keyword::STATEMENT => {}
                    sqltk_parser::keywords::Keyword::STATIC => {}
                    sqltk_parser::keywords::Keyword::STATISTICS => {}
                    sqltk_parser::keywords::Keyword::STATUS => {}
                    sqltk_parser::keywords::Keyword::STDDEV_POP => {}
                    sqltk_parser::keywords::Keyword::STDDEV_SAMP => {}
                    sqltk_parser::keywords::Keyword::STDIN => {}
                    sqltk_parser::keywords::Keyword::STDOUT => {}
                    sqltk_parser::keywords::Keyword::STEP => {}
                    sqltk_parser::keywords::Keyword::STORAGE_INTEGRATION => {}
                    sqltk_parser::keywords::Keyword::STORAGE_SERIALIZATION_POLICY => {}
                    sqltk_parser::keywords::Keyword::STORED => {}
                    sqltk_parser::keywords::Keyword::STRICT => {}
                    sqltk_parser::keywords::Keyword::STRING => {}
                    sqltk_parser::keywords::Keyword::STRUCT => {}
                    sqltk_parser::keywords::Keyword::SUBMULTISET => {}
                    sqltk_parser::keywords::Keyword::SUBSTRING => {}
                    sqltk_parser::keywords::Keyword::SUBSTRING_REGEX => {}
                    sqltk_parser::keywords::Keyword::SUCCEEDS => {}
                    sqltk_parser::keywords::Keyword::SUM => {}
                    sqltk_parser::keywords::Keyword::SUPER => {}
                    sqltk_parser::keywords::Keyword::SUPERUSER => {}
                    sqltk_parser::keywords::Keyword::SUPPORT => {}
                    sqltk_parser::keywords::Keyword::SUSPEND => {}
                    sqltk_parser::keywords::Keyword::SWAP => {}
                    sqltk_parser::keywords::Keyword::SYMMETRIC => {}
                    sqltk_parser::keywords::Keyword::SYNC => {}
                    sqltk_parser::keywords::Keyword::SYSTEM => {}
                    sqltk_parser::keywords::Keyword::SYSTEM_TIME => {}
                    sqltk_parser::keywords::Keyword::SYSTEM_USER => {}
                    sqltk_parser::keywords::Keyword::TABLE => {}
                    sqltk_parser::keywords::Keyword::TABLES => {}
                    sqltk_parser::keywords::Keyword::TABLESAMPLE => {}
                    sqltk_parser::keywords::Keyword::TAG => {}
                    sqltk_parser::keywords::Keyword::TARGET => {}
                    sqltk_parser::keywords::Keyword::TASK => {}
                    sqltk_parser::keywords::Keyword::TBLPROPERTIES => {}
                    sqltk_parser::keywords::Keyword::TEMP => {}
                    sqltk_parser::keywords::Keyword::TEMPORARY => {}
                    sqltk_parser::keywords::Keyword::TEMPTABLE => {}
                    sqltk_parser::keywords::Keyword::TERMINATED => {}
                    sqltk_parser::keywords::Keyword::TERSE => {}
                    sqltk_parser::keywords::Keyword::TEXT => {}
                    sqltk_parser::keywords::Keyword::TEXTFILE => {}
                    sqltk_parser::keywords::Keyword::THEN => {}
                    sqltk_parser::keywords::Keyword::TIES => {}
                    sqltk_parser::keywords::Keyword::TIME => {}
                    sqltk_parser::keywords::Keyword::TIMESTAMP => {}
                    sqltk_parser::keywords::Keyword::TIMESTAMPTZ => {}
                    sqltk_parser::keywords::Keyword::TIMETZ => {}
                    sqltk_parser::keywords::Keyword::TIMEZONE => {}
                    sqltk_parser::keywords::Keyword::TIMEZONE_ABBR => {}
                    sqltk_parser::keywords::Keyword::TIMEZONE_HOUR => {}
                    sqltk_parser::keywords::Keyword::TIMEZONE_MINUTE => {}
                    sqltk_parser::keywords::Keyword::TIMEZONE_REGION => {}
                    sqltk_parser::keywords::Keyword::TINYBLOB => {}
                    sqltk_parser::keywords::Keyword::TINYINT => {}
                    sqltk_parser::keywords::Keyword::TINYTEXT => {}
                    sqltk_parser::keywords::Keyword::TO => {}
                    sqltk_parser::keywords::Keyword::TOP => {}
                    sqltk_parser::keywords::Keyword::TOTALS => {}
                    sqltk_parser::keywords::Keyword::TRACE => {}
                    sqltk_parser::keywords::Keyword::TRAILING => {}
                    sqltk_parser::keywords::Keyword::TRANSACTION => {}
                    sqltk_parser::keywords::Keyword::TRANSIENT => {}
                    sqltk_parser::keywords::Keyword::TRANSLATE => {}
                    sqltk_parser::keywords::Keyword::TRANSLATE_REGEX => {}
                    sqltk_parser::keywords::Keyword::TRANSLATION => {}
                    sqltk_parser::keywords::Keyword::TREAT => {}
                    sqltk_parser::keywords::Keyword::TRIGGER => {}
                    sqltk_parser::keywords::Keyword::TRIM => {}
                    sqltk_parser::keywords::Keyword::TRIM_ARRAY => {}
                    sqltk_parser::keywords::Keyword::TRUE => {}
                    sqltk_parser::keywords::Keyword::TRUNCATE => {}
                    sqltk_parser::keywords::Keyword::TRY => {}
                    sqltk_parser::keywords::Keyword::TRY_CAST => {}
                    sqltk_parser::keywords::Keyword::TRY_CONVERT => {}
                    sqltk_parser::keywords::Keyword::TUPLE => {}
                    sqltk_parser::keywords::Keyword::TYPE => {}
                    sqltk_parser::keywords::Keyword::UESCAPE => {}
                    sqltk_parser::keywords::Keyword::UINT128 => {}
                    sqltk_parser::keywords::Keyword::UINT16 => {}
                    sqltk_parser::keywords::Keyword::UINT256 => {}
                    sqltk_parser::keywords::Keyword::UINT32 => {}
                    sqltk_parser::keywords::Keyword::UINT64 => {}
                    sqltk_parser::keywords::Keyword::UINT8 => {}
                    sqltk_parser::keywords::Keyword::UNBOUNDED => {}
                    sqltk_parser::keywords::Keyword::UNCACHE => {}
                    sqltk_parser::keywords::Keyword::UNCOMMITTED => {}
                    sqltk_parser::keywords::Keyword::UNDEFINED => {}
                    sqltk_parser::keywords::Keyword::UNFREEZE => {}
                    sqltk_parser::keywords::Keyword::UNION => {}
                    sqltk_parser::keywords::Keyword::UNIQUE => {}
                    sqltk_parser::keywords::Keyword::UNKNOWN => {}
                    sqltk_parser::keywords::Keyword::UNLISTEN => {}
                    sqltk_parser::keywords::Keyword::UNLOAD => {}
                    sqltk_parser::keywords::Keyword::UNLOCK => {}
                    sqltk_parser::keywords::Keyword::UNLOGGED => {}
                    sqltk_parser::keywords::Keyword::UNMATCHED => {}
                    sqltk_parser::keywords::Keyword::UNNEST => {}
                    sqltk_parser::keywords::Keyword::UNPIVOT => {}
                    sqltk_parser::keywords::Keyword::UNSAFE => {}
                    sqltk_parser::keywords::Keyword::UNSET => {}
                    sqltk_parser::keywords::Keyword::UNSIGNED => {}
                    sqltk_parser::keywords::Keyword::UNTIL => {}
                    sqltk_parser::keywords::Keyword::UPDATE => {}
                    sqltk_parser::keywords::Keyword::UPPER => {}
                    sqltk_parser::keywords::Keyword::URL => {}
                    sqltk_parser::keywords::Keyword::USAGE => {}
                    sqltk_parser::keywords::Keyword::USE => {}
                    sqltk_parser::keywords::Keyword::USER => {}
                    sqltk_parser::keywords::Keyword::USER_RESOURCES => {}
                    sqltk_parser::keywords::Keyword::USING => {}
                    sqltk_parser::keywords::Keyword::UUID => {}
                    sqltk_parser::keywords::Keyword::VACUUM => {}
                    sqltk_parser::keywords::Keyword::VALID => {}
                    sqltk_parser::keywords::Keyword::VALIDATION_MODE => {}
                    sqltk_parser::keywords::Keyword::VALUE => {}
                    sqltk_parser::keywords::Keyword::VALUES => {}
                    sqltk_parser::keywords::Keyword::VALUE_OF => {}
                    sqltk_parser::keywords::Keyword::VARBINARY => {}
                    sqltk_parser::keywords::Keyword::VARBIT => {}
                    sqltk_parser::keywords::Keyword::VARCHAR => {}
                    sqltk_parser::keywords::Keyword::VARIABLES => {}
                    sqltk_parser::keywords::Keyword::VARYING => {}
                    sqltk_parser::keywords::Keyword::VAR_POP => {}
                    sqltk_parser::keywords::Keyword::VAR_SAMP => {}
                    sqltk_parser::keywords::Keyword::VERBOSE => {}
                    sqltk_parser::keywords::Keyword::VERSION => {}
                    sqltk_parser::keywords::Keyword::VERSIONING => {}
                    sqltk_parser::keywords::Keyword::VERSIONS => {}
                    sqltk_parser::keywords::Keyword::VIEW => {}
                    sqltk_parser::keywords::Keyword::VIEWS => {}
                    sqltk_parser::keywords::Keyword::VIRTUAL => {}
                    sqltk_parser::keywords::Keyword::VOLATILE => {}
                    sqltk_parser::keywords::Keyword::VOLUME => {}
                    sqltk_parser::keywords::Keyword::WAREHOUSE => {}
                    sqltk_parser::keywords::Keyword::WAREHOUSES => {}
                    sqltk_parser::keywords::Keyword::WEEK => {}
                    sqltk_parser::keywords::Keyword::WEEKS => {}
                    sqltk_parser::keywords::Keyword::WHEN => {}
                    sqltk_parser::keywords::Keyword::WHENEVER => {}
                    sqltk_parser::keywords::Keyword::WHERE => {}
                    sqltk_parser::keywords::Keyword::WIDTH_BUCKET => {}
                    sqltk_parser::keywords::Keyword::WINDOW => {}
                    sqltk_parser::keywords::Keyword::WITH => {}
                    sqltk_parser::keywords::Keyword::WITHIN => {}
                    sqltk_parser::keywords::Keyword::WITHOUT => {}
                    sqltk_parser::keywords::Keyword::WITHOUT_ARRAY_WRAPPER => {}
                    sqltk_parser::keywords::Keyword::WORK => {}
                    sqltk_parser::keywords::Keyword::WRITE => {}
                    sqltk_parser::keywords::Keyword::XML => {}
                    sqltk_parser::keywords::Keyword::XOR => {}
                    sqltk_parser::keywords::Keyword::YEAR => {}
                    sqltk_parser::keywords::Keyword::YEARS => {}
                    sqltk_parser::keywords::Keyword::ZONE => {}
                    sqltk_parser::keywords::Keyword::ZORDER => {}
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::keywords::Keyword {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::tokenizer::Location {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.line.accept(visitor)?;
                self.column.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::tokenizer::Location {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::tokenizer::Span {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.start.accept(visitor)?;
                self.end.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::tokenizer::Span {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::tokenizer::Token {
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
                    sqltk_parser::tokenizer::Token::EOF => {}
                    sqltk_parser::tokenizer::Token::Word(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::Number(field0, field1) => {
                        field0.accept(visitor)?;
                        field1.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::Char(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::SingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::DoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::TripleSingleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::TripleDoubleQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::DollarQuotedString(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::SingleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::DoubleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::TripleSingleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::SingleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::DoubleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::TripleSingleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral(
                        field0,
                    ) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::NationalStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::EscapedStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::UnicodeStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::HexStringLiteral(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::Comma => {}
                    sqltk_parser::tokenizer::Token::Whitespace(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::DoubleEq => {}
                    sqltk_parser::tokenizer::Token::Eq => {}
                    sqltk_parser::tokenizer::Token::Neq => {}
                    sqltk_parser::tokenizer::Token::Lt => {}
                    sqltk_parser::tokenizer::Token::Gt => {}
                    sqltk_parser::tokenizer::Token::LtEq => {}
                    sqltk_parser::tokenizer::Token::GtEq => {}
                    sqltk_parser::tokenizer::Token::Spaceship => {}
                    sqltk_parser::tokenizer::Token::Plus => {}
                    sqltk_parser::tokenizer::Token::Minus => {}
                    sqltk_parser::tokenizer::Token::Mul => {}
                    sqltk_parser::tokenizer::Token::Div => {}
                    sqltk_parser::tokenizer::Token::DuckIntDiv => {}
                    sqltk_parser::tokenizer::Token::Mod => {}
                    sqltk_parser::tokenizer::Token::StringConcat => {}
                    sqltk_parser::tokenizer::Token::LParen => {}
                    sqltk_parser::tokenizer::Token::RParen => {}
                    sqltk_parser::tokenizer::Token::Period => {}
                    sqltk_parser::tokenizer::Token::Colon => {}
                    sqltk_parser::tokenizer::Token::DoubleColon => {}
                    sqltk_parser::tokenizer::Token::Assignment => {}
                    sqltk_parser::tokenizer::Token::SemiColon => {}
                    sqltk_parser::tokenizer::Token::Backslash => {}
                    sqltk_parser::tokenizer::Token::LBracket => {}
                    sqltk_parser::tokenizer::Token::RBracket => {}
                    sqltk_parser::tokenizer::Token::Ampersand => {}
                    sqltk_parser::tokenizer::Token::Pipe => {}
                    sqltk_parser::tokenizer::Token::Caret => {}
                    sqltk_parser::tokenizer::Token::LBrace => {}
                    sqltk_parser::tokenizer::Token::RBrace => {}
                    sqltk_parser::tokenizer::Token::RArrow => {}
                    sqltk_parser::tokenizer::Token::Sharp => {}
                    sqltk_parser::tokenizer::Token::DoubleSharp => {}
                    sqltk_parser::tokenizer::Token::Tilde => {}
                    sqltk_parser::tokenizer::Token::TildeAsterisk => {}
                    sqltk_parser::tokenizer::Token::ExclamationMarkTilde => {}
                    sqltk_parser::tokenizer::Token::ExclamationMarkTildeAsterisk => {}
                    sqltk_parser::tokenizer::Token::DoubleTilde => {}
                    sqltk_parser::tokenizer::Token::DoubleTildeAsterisk => {}
                    sqltk_parser::tokenizer::Token::ExclamationMarkDoubleTilde => {}
                    sqltk_parser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk => {}
                    sqltk_parser::tokenizer::Token::ShiftLeft => {}
                    sqltk_parser::tokenizer::Token::ShiftRight => {}
                    sqltk_parser::tokenizer::Token::Overlap => {}
                    sqltk_parser::tokenizer::Token::ExclamationMark => {}
                    sqltk_parser::tokenizer::Token::DoubleExclamationMark => {}
                    sqltk_parser::tokenizer::Token::AtSign => {}
                    sqltk_parser::tokenizer::Token::CaretAt => {}
                    sqltk_parser::tokenizer::Token::PGSquareRoot => {}
                    sqltk_parser::tokenizer::Token::PGCubeRoot => {}
                    sqltk_parser::tokenizer::Token::Placeholder(field0) => {
                        field0.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Token::Arrow => {}
                    sqltk_parser::tokenizer::Token::LongArrow => {}
                    sqltk_parser::tokenizer::Token::HashArrow => {}
                    sqltk_parser::tokenizer::Token::AtDashAt => {}
                    sqltk_parser::tokenizer::Token::QuestionMarkDash => {}
                    sqltk_parser::tokenizer::Token::AmpersandLeftAngleBracket => {}
                    sqltk_parser::tokenizer::Token::AmpersandRightAngleBracket => {}
                    sqltk_parser::tokenizer::Token::AmpersandLeftAngleBracketVerticalBar => {}
                    sqltk_parser::tokenizer::Token::VerticalBarAmpersandRightAngleBracket => {}
                    sqltk_parser::tokenizer::Token::TwoWayArrow => {}
                    sqltk_parser::tokenizer::Token::LeftAngleBracketCaret => {}
                    sqltk_parser::tokenizer::Token::RightAngleBracketCaret => {}
                    sqltk_parser::tokenizer::Token::QuestionMarkSharp => {}
                    sqltk_parser::tokenizer::Token::QuestionMarkDashVerticalBar => {}
                    sqltk_parser::tokenizer::Token::QuestionMarkDoubleVerticalBar => {}
                    sqltk_parser::tokenizer::Token::TildeEqual => {}
                    sqltk_parser::tokenizer::Token::ShiftLeftVerticalBar => {}
                    sqltk_parser::tokenizer::Token::VerticalBarShiftRight => {}
                    sqltk_parser::tokenizer::Token::HashLongArrow => {}
                    sqltk_parser::tokenizer::Token::AtArrow => {}
                    sqltk_parser::tokenizer::Token::ArrowAt => {}
                    sqltk_parser::tokenizer::Token::HashMinus => {}
                    sqltk_parser::tokenizer::Token::AtQuestion => {}
                    sqltk_parser::tokenizer::Token::AtAt => {}
                    sqltk_parser::tokenizer::Token::Question => {}
                    sqltk_parser::tokenizer::Token::QuestionAnd => {}
                    sqltk_parser::tokenizer::Token::QuestionPipe => {}
                    sqltk_parser::tokenizer::Token::CustomBinaryOperator(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::tokenizer::Token {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::tokenizer::TokenWithSpan {
    fn accept<'ast, V: crate::Visitor<'ast>>(
        &'ast self,
        visitor: &mut V,
    ) -> std::ops::ControlFlow<crate::Break<V::Error>> {
        visit(
            self,
            visitor,
            #[allow(unused_variables)]
            |visitor| {
                self.token.accept(visitor)?;
                self.span.accept(visitor)?;
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::tokenizer::TokenWithSpan {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::tokenizer::Whitespace {
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
                    sqltk_parser::tokenizer::Whitespace::Space => {}
                    sqltk_parser::tokenizer::Whitespace::Newline => {}
                    sqltk_parser::tokenizer::Whitespace::Tab => {}
                    sqltk_parser::tokenizer::Whitespace::SingleLineComment {
                        comment,
                        prefix,
                    } => {
                        comment.accept(visitor)?;
                        prefix.accept(visitor)?;
                    }
                    sqltk_parser::tokenizer::Whitespace::MultiLineComment(field0) => {
                        field0.accept(visitor)?;
                    }
                }
                std::ops::ControlFlow::Continue(())
            },
        )
    }
}
#[automatically_derived]
impl crate::AsNodeKey for sqltk_parser::tokenizer::Whitespace {
    fn as_node_key(&self) -> crate::NodeKey<'_> {
        crate::NodeKey::new(self)
    }
}
#[automatically_derived]
impl crate::Visitable for sqltk_parser::tokenizer::Word {
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
impl crate::AsNodeKey for sqltk_parser::tokenizer::Word {
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
