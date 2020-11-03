use serde::{Serialize, Deserialize};

use crate::contracts;
use crate::types::TxRef;
use crate::TransactionStatus;

/// Secret code contract states.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SecretCode {
    codev: String
}

/// The commands that the contract accepts from the blockchain. Also called transactions.
/// Commands are supposed to update the states of the contract.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Show {
        codestr: String
    }
}

/// The errors that the contract could throw for some queries
#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotAuthorized,
    SomeOtherError,
}

/// Query requests. The end users can only query the contract states by sending requests.
/// Queries are not supposed to write to the contract states.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    GetCode,
}

/// Query responses.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    GetCode {
        code: String
    },
    /// Something wrong happened
    Error(Error)
}


impl SecretCode {
    /// Initializes the contract
    pub fn new() -> Self {
        Default::default()
    }
}

impl contracts::Contract<Command, Request, Response> for SecretCode {
    // Returns the contract id
    fn id(&self) -> contracts::ContractId { contracts::SECRET_CODE }

    // Handles the commands from transactions on the blockchain. This method doesn't respond.
    fn handle_command(&mut self, _origin: &chain::AccountId, _txref: &TxRef, cmd: Command) -> TransactionStatus {
        match cmd {
            Command::Show { codestr } => {
                self.codev = codestr;
                TransactionStatus::Ok
            }
        }
    }

    // Handles a direct query and responds to the query. It shouldn't modify the contract states.
    fn handle_query(&mut self, _origin: Option<&chain::AccountId>, req: Request) -> Response {
        let inner = || -> Result<Response, Error> {
            match req {
                Request::GetCode => {
                    Ok(Response::GetCode { code: self.codev })
                },
            }
        };
        match inner() {
            Err(error) => Response::Error(error),
            Ok(resp) => resp
        }
    }
}

