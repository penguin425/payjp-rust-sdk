//! Platform API resources for multi-tenant functionality.

pub mod tenant;
pub mod tenant_transfer;

pub use tenant::{CreateTenantParams, Tenant, TenantService, UpdateTenantParams};
pub use tenant_transfer::{TenantTransfer, TenantTransferService};
