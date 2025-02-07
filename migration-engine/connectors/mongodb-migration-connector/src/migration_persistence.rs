use crate::MongoDbMigrationConnector;
use migration_connector::MigrationPersistence;

#[async_trait::async_trait]
impl MigrationPersistence for MongoDbMigrationConnector {
    async fn baseline_initialize(&mut self) -> migration_connector::ConnectorResult<()> {
        Err(crate::unsupported_command_error())
    }

    async fn initialize(&mut self) -> migration_connector::ConnectorResult<()> {
        Err(crate::unsupported_command_error())
    }

    async fn mark_migration_applied_impl(
        &mut self,
        _migration_name: &str,
        _checksum: &str,
    ) -> migration_connector::ConnectorResult<String> {
        Err(crate::unsupported_command_error())
    }

    async fn mark_migration_rolled_back_by_id(
        &mut self,
        _migration_id: &str,
    ) -> migration_connector::ConnectorResult<()> {
        Err(crate::unsupported_command_error())
    }

    async fn record_migration_started_impl(
        &mut self,
        _migration_name: &str,
        _checksum: &str,
    ) -> migration_connector::ConnectorResult<String> {
        Err(crate::unsupported_command_error())
    }

    async fn record_successful_step(&mut self, _id: &str) -> migration_connector::ConnectorResult<()> {
        Err(crate::unsupported_command_error())
    }

    async fn record_failed_step(&mut self, _id: &str, _logs: &str) -> migration_connector::ConnectorResult<()> {
        Err(crate::unsupported_command_error())
    }

    async fn record_migration_finished(&mut self, _id: &str) -> migration_connector::ConnectorResult<()> {
        Err(crate::unsupported_command_error())
    }

    async fn list_migrations(
        &mut self,
    ) -> migration_connector::ConnectorResult<
        Result<Vec<migration_connector::MigrationRecord>, migration_connector::PersistenceNotInitializedError>,
    > {
        Err(crate::unsupported_command_error())
    }
}
