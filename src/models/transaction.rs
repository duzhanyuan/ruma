//! Matrix transaction.

use diesel::{
    FindDsl,
    LoadDsl,
    insert,
};
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;

use error::ApiError;
use schema::transactions;

/// A Transaction.
#[derive(AsChangeset, Clone, Debug, Identifiable, Insertable, Queryable)]
#[primary_key(path, access_token)]
#[table_name = "transactions"]
pub struct Transaction {
    /// The full path of the endpoint used for the transaction.
    pub path: String,
    /// The access token used.
    pub access_token: String,
    /// The serialized response of the endpoint. It should be used
    /// as the response on future requests.
    pub response: String,
}

impl Transaction {
    /// Create a new transaction entry.
    pub fn create(
        connection: &PgConnection,
        path: String,
        access_token: String,
        response: String
    ) -> Result<Transaction, ApiError> {
        let new_transaction = Transaction {
            path: path,
            access_token: access_token,
            response: response,
        };

        insert(&new_transaction)
            .into(transactions::table)
            .get_result(connection)
            .map_err(ApiError::from)
    }

    /// Look up a transaction with the url path of the endpoint and the access token.
    pub fn find(
        connection: &PgConnection,
        path: &str,
        access_token: &str
    ) -> Result<Option<Transaction>, ApiError> {
        let transaction = transactions::table
            .find((path, access_token))
            .get_result(connection);

        match transaction {
            Ok(transaction) => Ok(Some(transaction)),
            Err(DieselError::NotFound) => Ok(None),
            Err(err) => Err(ApiError::from(err)),
        }
    }
}
