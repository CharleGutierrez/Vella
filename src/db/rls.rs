use tracing::info;

#[derive(Clone, Debug)]
pub struct RlsPolicy {
    pub table: String,
    pub tenant_column: String,
}

impl RlsPolicy {
    pub fn new(table: &str, tenant_column: &str) -> Self {
        Self {
            table: table.to_string(),
            tenant_column: tenant_column.to_string(),
        }
    }

    pub fn apply_to_query(&self, base_query: &str, tenant_id: &str) -> String {
        // A real implementation would parse the AST or use a query builder.
        // For demonstration, we safely append a WHERE clause if it doesn't exist,
        // or an AND clause if it does.
        info!("Applying RLS Policy for table {} (Tenant: {})", self.table, tenant_id);
        
        let safe_tenant_id = tenant_id.replace("'", "''"); // Basic SQL escaping

        if base_query.to_uppercase().contains("WHERE") {
            format!("{} AND {} = '{}'", base_query, self.tenant_column, safe_tenant_id)
        } else {
            format!("{} WHERE {} = '{}'", base_query, self.tenant_column, safe_tenant_id)
        }
    }
}
