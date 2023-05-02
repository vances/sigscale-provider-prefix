//! prefixtables-provider capability provider
//!
//!
use wasmbus_rpc::provider::prelude::*;
use sigscale_interface_prefix::*;

// Start the provider and run until stopped by the host
fn main() -> Result<(), Box<dyn std::error::Error>> {
	provider_main(PrefixTablesProvider::default(), Some("PrefixTables".to_string()))?;
	eprintln!("PrefixTables provider exiting");
	Ok(())
}

/// prefixtables-provider capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(PrefixTables)]
struct PrefixTablesProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for PrefixTablesProvider {}
impl ProviderHandler for PrefixTablesProvider {}

/// Handle Prefix methods
#[async_trait]
impl PrefixTables for PrefixTablesProvider {
	// Create a new prefix table
	async fn create_table (
		&self,
		_ctx: &Context,
		_arg: &CreateTableRequest,
	) -> RpcResult<CreateTableResponse> {
		todo!()
	}

	// Destroy an existing prefix table
	async fn destroy_table (
		&self,
		_ctx: &Context,
		_arg: &DestroyTableRequest,
	) -> RpcResult<DestroyTableResponse> {
		todo!()
	}

	// Insert a prefix into a table
	async fn insert_prefix (
		&self,
		_ctx: &Context,
		_arg: &InsertPrefixRequest,
	) -> RpcResult<InsertPrefixResponse> {
		todo!()
	}

	// Remove a prefix from a table
	async fn remove_prefix (
		&self,
		_ctx: &Context,
		_arg: &RemovePrefixRequest,
	) -> RpcResult<RemovePrefixResponse> {
		todo!()
	}

	// Match an address in a prefix table
	async fn match_prefix (
		&self,
		_ctx: &Context,
		_arg: &MatchPrefixRequest,
	) -> RpcResult<MatchPrefixResponse> {
		Ok(MatchPrefixResponse {success: true, value: 0u32, ..Default::default() })
	}
}
