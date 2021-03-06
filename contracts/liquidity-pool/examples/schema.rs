use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use liquidity_pool::msg::{
    ConfigResponse, HandleMsg, InitMsg, QueryMsg, ReceiveMsg, ReserveResponse,
};

use liquidity_pool::state::Config;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(ReserveResponse), &out_dir);
    export_schema(&schema_for!(ReceiveMsg), &out_dir);
}
