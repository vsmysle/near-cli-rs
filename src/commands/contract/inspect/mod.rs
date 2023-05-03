use color_eyre::eyre::Context;
use inquire::Text;

use crate::common::{JsonRpcClientExt, RpcQueryResponseExt};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = crate::GlobalContext)]
#[interactive_clap(output_context = ContractAccountContext)]
pub struct ContractAccount {
    /// What is the contract account ID?
    account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    /// Select a folder to download the contract
    to_folder: InspectContract,
}

#[derive(Debug, Clone)]
pub struct ContractAccountContext {
    config: crate::config::Config,
    account_id: near_primitives::types::AccountId,
}

impl ContractAccountContext {
    pub fn from_previous_context(
        previous_context: crate::GlobalContext,
        scope: &<ContractAccount as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            config: previous_context.0,
            account_id: scope.account_id.clone().into(),
        })
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ContractAccountContext)]
#[interactive_clap(output_context = InspectContractContext)]
pub struct InspectContract {
    #[interactive_clap(skip_default_input_arg)]
    /// Where to download the contract file?
    folder_path: crate::types::path_buf::PathBuf,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: crate::network_view_at_block::NetworkViewAtBlockArgs,
}

#[derive(Clone)]
pub struct InspectContractContext(crate::network_view_at_block::ArgsForViewContext);

impl InspectContractContext {
    pub fn from_previous_context(
        previous_context: ContractAccountContext,
        _scope: &<InspectContract as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let account_id = previous_context.account_id;

        let on_after_getting_block_reference_callback: crate::network_view_at_block::OnAfterGettingBlockReferenceCallback = std::sync::Arc::new({
            move |network_config, block_reference| {
                let call_access_view = network_config
                    .json_rpc_client()
                    .blocking_call_view_code(&account_id, block_reference.clone())
                    .wrap_err_with(|| format!("Failed to fetch query ViewCode for <{}>", &account_id))?
                    .contract_code_view()
                    .wrap_err_with(|| format!("Error call result for <{}>", &account_id))?;

                for payload in wasmparser::Parser::new(0).parse_all(call_access_view.code.as_slice()) {
                    if let wasmparser::Payload::ExportSection(s) = payload? {
                        for export in s {
                            let export = export?;

                            if export.kind == wasmparser::ExternalKind::Func {
                                eprintln!("{:?}", export.name);
                            }
                        }
                    }
                }
                Ok(())
            }
        });
        Ok(Self(crate::network_view_at_block::ArgsForViewContext {
            config: previous_context.config,
            on_after_getting_block_reference_callback,
        }))
    }
}

impl From<InspectContractContext> for crate::network_view_at_block::ArgsForViewContext {
    fn from(item: InspectContractContext) -> Self {
        item.0
    }
}

impl InspectContract {
    fn input_folder_path(
        _context: &ContractAccountContext,
    ) -> color_eyre::eyre::Result<Option<crate::types::path_buf::PathBuf>> {
        let home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
        let mut folder_path = std::path::PathBuf::from(&home_dir);
        folder_path.push("Downloads");
        eprintln!();
        let input_folder_path = Text::new("Where to download the contract file?")
            .with_initial_value(&format!("{}", folder_path.to_string_lossy()))
            .prompt()?;
        let folder_path = shellexpand::tilde(&input_folder_path).as_ref().parse()?;
        Ok(Some(folder_path))
    }
}
