use serde_json::json;

use crate::primitives::{
    AccountInPoolResponse, AccountInPoolResult, Block, BlockResponse, Response, Validators,
    ValidatorsResponse, ViewAccountResponse,
};

#[derive(Debug)]
pub(crate) struct NearJsonRpcClient {
    endpoint: String,
}

impl NearJsonRpcClient {
    pub fn new(endpoint: String) -> Self {
        NearJsonRpcClient { endpoint }
    }

    pub async fn get_locked_amount(
        &self,
        account_id: String,
        block_height: u64,
    ) -> Result<u128, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": json!({
                "request_type": "call_function",
                "block_id": block_height,
                "account_id": account_id,
                "method_name": "get_locked_amount",
                "args_base64": ""
            })
        });

        let client = reqwest::Client::new();
        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: Response = res.json().await?;

        Ok(body.result.get_amount())
    }

    pub async fn get_liquid_owners_balance(
        &self,
        account_id: String,
        block_height: u64,
    ) -> Result<u128, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": json!({
                "request_type": "call_function",
                "block_id": block_height,
                "account_id": account_id,
                "method_name": "get_liquid_owners_balance",
                "args_base64": ""
            })
        });

        let client = reqwest::Client::new();
        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: Response = res.json().await?;

        Ok(body.result.get_amount())
    }

    pub async fn get_account_in_pool(
        &self,
        account_id: String,
        pool_account_id: String,
        block_height: u64,
    ) -> Result<AccountInPoolResult, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": json!({
                "request_type": "call_function",
                "block_id": block_height,
                "account_id": pool_account_id,
                "method_name": "get_account",
                "args_base64": base64::encode(json!({"account_id": account_id}).to_string()),
            })
        });

        let client = reqwest::Client::new();
        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: AccountInPoolResponse = res.json().await?;

        let account_in_pool: AccountInPoolResult =
            serde_json::from_slice(&body.result.result[..]).unwrap();

        Ok(account_in_pool)
    }

    pub async fn get_native_balance(
        &self,
        account_id: String,
        block_height: u64,
    ) -> Result<u128, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": json!({
                "request_type": "view_account",
                "block_id": block_height,
                "account_id": account_id,
            })
        });

        let client = reqwest::Client::new();
        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: ViewAccountResponse = res.json().await?;

        Ok(body.result.get_amount())
    }

    pub async fn get_validators(&self) -> Result<Validators, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "validators",
            "params": json!({"latest": null}),
        });

        let client = reqwest::Client::new();

        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: ValidatorsResponse = res.json().await?;

        Ok(body.result)
    }

    pub async fn get_block(&self, block_height: u64) -> Result<Block, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "block",
            "params": json!({"block_id": block_height}),
        });

        let client = reqwest::Client::new();

        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: BlockResponse = res.json().await?;

        Ok(body.result)
    }

    pub async fn get_final_block(&self) -> Result<Block, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "block",
            "params": json!({"finality": "final"}),
        });

        let client = reqwest::Client::new();

        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: BlockResponse = res.json().await?;

        Ok(body.result)
    }
    pub async fn get_staking_pool_account_id(
        &self,
        account_id: String,
    ) -> Result<String, reqwest::Error> {
        let params = json!({
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": json!({
                "request_type": "call_function",
                "finality": "final",
                "account_id": account_id,
                "method_name": "get_staking_pool_account_id",
                "args_base64": "e30="
            })
        });

        let client = reqwest::Client::new();
        let res = client.post(&self.endpoint).json(&params).send().await?;

        let body: AccountInPoolResponse = res.json().await?;

        let pool_account_id: String = serde_json::from_slice(&body.result.result[..]).unwrap();
        Ok(pool_account_id)
    }
}
