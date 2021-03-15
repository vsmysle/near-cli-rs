use dialoguer::{theme::ColorfulTheme, Input, Select};
use structopt::StructOpt;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

use super::sign_transaction::{CliSignTransaction, SignTransaction};
use super::transaction_actions::transfer_near_tokens_type::{
    CliTransferNEARTokensAction, NearBalance, TransferNEARTokensAction,
};

use super::transaction_actions::add_access_key_type::{
    AccessKeyPermission, AddAccessKeyAction, CliAddAccessKeyAction,
};
use super::transaction_actions::create_account_type::{
    CliCreateAccountAction, CreateAccountAction,
};
use super::transaction_actions::delete_access_key_type::{
    CliDeleteAccessKeyAction, DeleteAccessKeyAction,
};
use super::transaction_actions::delete_account_type::{
    CliDeleteAccountAction, DeleteAccountAction,
};

#[derive(Debug)]
pub struct Receiver {
    pub receiver_account_id: String,
    pub action: NextAction,
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum NextAction {
    #[strum_discriminants(strum(message = "Select a new action"))]
    AddAction(SelectAction),
    #[strum_discriminants(strum(message = "Skip adding a new action"))]
    Skip(SkipAction),
}

#[derive(Debug)]
pub struct SelectAction {
    transaction_subcommand: ActionSubcommand,
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum ActionSubcommand {
    #[strum_discriminants(strum(message = "Transfer NEAR Tokens"))]
    TransferNEARTokens(TransferNEARTokensAction),
    #[strum_discriminants(strum(message = "Call a Function"))]
    CallFunction,
    #[strum_discriminants(strum(message = "Stake NEAR Tokens"))]
    StakeNEARTokens,
    #[strum_discriminants(strum(message = "Create an Account"))]
    CreateAccount(CreateAccountAction),
    #[strum_discriminants(strum(message = "Delete an Account"))]
    DeleteAccount(DeleteAccountAction),
    #[strum_discriminants(strum(message = "Add an Access Key"))]
    AddAccessKey(AddAccessKeyAction),
    #[strum_discriminants(strum(message = "Detete an Access Key"))]
    DeleteAccessKey(DeleteAccessKeyAction),
}

#[derive(Debug, StructOpt)]
pub struct CliReceiver {
    receiver_account_id: Option<String>,
    #[structopt(subcommand)]
    action: Option<CliNextAction>,
}

#[derive(Debug, StructOpt)]
pub enum CliNextAction {
    AddAction(CliSelectAction),
    Skip(CliSkipAction),
}

#[derive(Debug, StructOpt)]
pub struct CliSelectAction {
    #[structopt(subcommand)]
    transaction_subcommand: Option<CliActionSubcommand>,
}

#[derive(Debug, StructOpt)]
pub enum CliActionSubcommand {
    TransferNEARTokens(CliTransferNEARTokensAction),
    CallFunction,
    StakeNEARTokens,
    CreateAccount(CliCreateAccountAction),
    DeleteAccount(CliDeleteAccountAction),
    AddAccessKey(CliAddAccessKeyAction),
    DeleteAccessKey(CliDeleteAccessKeyAction),
}

#[derive(Debug, StructOpt)]
pub enum CliSkipNextAction {
    Skip(CliSkipAction),
}

impl NextAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) {
        println!("Receiver process: self:\n       {:?}", &self);
        match self {
            NextAction::AddAction(select_action) => {
                select_action
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
            NextAction::Skip(skip_action) => {
                skip_action
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
        }
    }
}

impl SelectAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) {
        println!("Receiver process: self:\n       {:?}", &self);
        self.transaction_subcommand
            .process(prepopulated_unsigned_transaction, selected_server_url)
            .await;
    }
}

impl ActionSubcommand {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) {
        match self {
            ActionSubcommand::TransferNEARTokens(args_transfer) => {
                args_transfer
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
            // ActionSubcommand::CallFunction(args_function) => {},
            // ActionSubcommand::StakeNEARTokens(args_stake) => {},
            ActionSubcommand::CreateAccount(args_create_account) => {
                args_create_account
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
            ActionSubcommand::DeleteAccount(args_delete_account) => {
                args_delete_account
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
            ActionSubcommand::AddAccessKey(args_add_access_key) => {
                args_add_access_key
                    .process(
                        prepopulated_unsigned_transaction,
                        selected_server_url,
                        "".to_string(),
                    )
                    .await
            }
            ActionSubcommand::DeleteAccessKey(args_delete_access_key) => {
                args_delete_access_key
                    .process(prepopulated_unsigned_transaction, selected_server_url)
                    .await
            }
            _ => unreachable!("Error"),
        }
    }
    pub fn choose_action_command() -> Self {
        println!();
        let variants = ActionSubcommandDiscriminants::iter().collect::<Vec<_>>();
        let action_subcommands = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_action_subcommand = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&action_subcommands)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_action_subcommand] {
            ActionSubcommandDiscriminants::TransferNEARTokens => {
                let amount: NearBalance = NearBalance::input_amount();
                let next_action: Box<NextAction> = Box::new(NextAction::input_next_action());
                ActionSubcommand::TransferNEARTokens(TransferNEARTokensAction {
                    amount,
                    next_action,
                })
            }
            ActionSubcommandDiscriminants::CallFunction => ActionSubcommand::CallFunction,
            ActionSubcommandDiscriminants::StakeNEARTokens => ActionSubcommand::StakeNEARTokens,
            ActionSubcommandDiscriminants::CreateAccount => {
                let next_action: Box<NextAction> = Box::new(NextAction::input_next_action());
                ActionSubcommand::CreateAccount(CreateAccountAction { next_action })
            }
            ActionSubcommandDiscriminants::DeleteAccount => {
                let beneficiary_id: String = DeleteAccountAction::input_beneficiary_id();
                let next_action: Box<NextAction> = Box::new(NextAction::input_next_action());
                ActionSubcommand::DeleteAccount(DeleteAccountAction {
                    beneficiary_id,
                    next_action,
                })
            }
            ActionSubcommandDiscriminants::AddAccessKey => {
                let public_key: String = AddAccessKeyAction::input_public_key();
                let nonce: near_primitives::types::Nonce = AddAccessKeyAction::input_nonce();
                let permission: AccessKeyPermission = AccessKeyPermission::choose_permission();
                ActionSubcommand::AddAccessKey(AddAccessKeyAction {
                    public_key,
                    nonce,
                    permission,
                })
            }
            ActionSubcommandDiscriminants::DeleteAccessKey => {
                let public_key: String = DeleteAccessKeyAction::input_public_key();
                let next_action: Box<NextAction> = Box::new(NextAction::input_next_action());
                ActionSubcommand::DeleteAccessKey(DeleteAccessKeyAction {
                    public_key,
                    next_action,
                })
            }
        }
    }
}

impl Receiver {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) {
        println!("Receiver process: self:\n       {:?}", &self);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            receiver_id: self.receiver_account_id.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.action
            .process(unsigned_transaction, selected_server_url)
            .await;
    }
    pub fn input_receiver_account_id() -> String {
        Input::new()
            .with_prompt("What is the account ID of the receiver?")
            .interact_text()
            .unwrap()
    }
}

impl From<CliReceiver> for Receiver {
    fn from(item: CliReceiver) -> Self {
        let receiver_account_id: String = match item.receiver_account_id {
            Some(cli_receiver_account_id) => cli_receiver_account_id,
            None => Receiver::input_receiver_account_id(),
        };
        let action: NextAction = match item.action {
            Some(cli_next_action) => NextAction::from(cli_next_action),
            None => NextAction::input_next_action(),
        };
        Receiver {
            receiver_account_id,
            action,
        }
    }
}

impl NextAction {
    pub fn input_next_action() -> Self {
        println!();
        let variants = NextActionDiscriminants::iter().collect::<Vec<_>>();
        let next_action = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_next_action = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&next_action)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_next_action] {
            NextActionDiscriminants::AddAction => {
                let transaction_subcommand: ActionSubcommand =
                    ActionSubcommand::choose_action_command();
                NextAction::AddAction(SelectAction {
                    transaction_subcommand,
                })
            }
            NextActionDiscriminants::Skip => {
                let sign_option: SignTransaction = SignTransaction::choose_sign_option();
                NextAction::Skip(SkipAction { sign_option })
            }
        }
    }
}

impl From<CliNextAction> for NextAction {
    fn from(item: CliNextAction) -> Self {
        match item {
            CliNextAction::AddAction(cli_select_action) => {
                let select_action: SelectAction = SelectAction::from(cli_select_action);
                NextAction::AddAction(select_action)
            }
            CliNextAction::Skip(cli_skip_action) => {
                let skip_action: SkipAction = SkipAction::from(cli_skip_action);
                NextAction::Skip(skip_action)
            }
        }
    }
}

impl From<CliSelectAction> for SelectAction {
    fn from(item: CliSelectAction) -> Self {
        let transaction_subcommand: ActionSubcommand = match item.transaction_subcommand {
            Some(cli_transaction_subcommand) => ActionSubcommand::from(cli_transaction_subcommand),
            None => ActionSubcommand::choose_action_command(),
        };
        SelectAction {
            transaction_subcommand,
        }
    }
}

impl From<CliActionSubcommand> for ActionSubcommand {
    fn from(item: CliActionSubcommand) -> Self {
        match item {
            CliActionSubcommand::TransferNEARTokens(cli_transfer_near_token) => {
                let transfer_near_token: TransferNEARTokensAction =
                    TransferNEARTokensAction::from(cli_transfer_near_token);
                ActionSubcommand::TransferNEARTokens(transfer_near_token)
            }
            CliActionSubcommand::CreateAccount(cli_create_account) => {
                let create_account: CreateAccountAction =
                    CreateAccountAction::from(cli_create_account);
                ActionSubcommand::CreateAccount(create_account)
            }
            CliActionSubcommand::DeleteAccount(cli_delete_account) => {
                let delete_account: DeleteAccountAction =
                    DeleteAccountAction::from(cli_delete_account);
                ActionSubcommand::DeleteAccount(delete_account)
            }
            CliActionSubcommand::AddAccessKey(cli_add_access_key) => {
                let add_access_key: AddAccessKeyAction =
                    AddAccessKeyAction::from(cli_add_access_key);
                ActionSubcommand::AddAccessKey(add_access_key)
            }
            CliActionSubcommand::DeleteAccessKey(cli_delete_access_key) => {
                let delete_access_key: DeleteAccessKeyAction =
                    DeleteAccessKeyAction::from(cli_delete_access_key);
                ActionSubcommand::DeleteAccessKey(delete_access_key)
            }
            _ => unreachable!("Error"),
        }
    }
}

impl From<CliSkipNextAction> for NextAction {
    fn from(item: CliSkipNextAction) -> Self {
        match item {
            CliSkipNextAction::Skip(cli_skip_action) => {
                let skip_action: SkipAction = SkipAction::from(cli_skip_action);
                NextAction::Skip(skip_action)
            }
        }
    }
}

#[derive(Debug)]
pub struct SkipAction {
    pub sign_option: SignTransaction,
}

#[derive(Debug, StructOpt)]
pub struct CliSkipAction {
    #[structopt(subcommand)]
    sign_option: Option<CliSignTransaction>,
}

impl SkipAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        selected_server_url: Option<url::Url>,
    ) {
        println!("Skip process:\n       {:?}", &self);
        println!(
            "Skip process: prepopulated_unsigned_transaction:\n       {:?}",
            &prepopulated_unsigned_transaction
        );
        self.sign_option
            .process(prepopulated_unsigned_transaction, selected_server_url)
            .await;
    }
}

impl From<CliSkipAction> for SkipAction {
    fn from(item: CliSkipAction) -> Self {
        let sign_option: SignTransaction = match item.sign_option {
            Some(cli_sign_transaction) => SignTransaction::from(cli_sign_transaction),
            None => SignTransaction::choose_sign_option(),
        };
        SkipAction { sign_option }
    }
}