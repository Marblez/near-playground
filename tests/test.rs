use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let consumer = sandbox.dev_create_account().await?;
    let producer = sandbox.dev_create_account().await?;

    let consumer_outcome = consumer
        .call(contract.id(), "update_greeting")
        .args_json(json!({}))
        .transact()
        .await?;
    let consumer_message = contract
        .view("get_greeting")
        .args_json(json!({}))
        .await?;
    assert_eq!(consumer_message.json::<String>()?, "Hello");

    let producer_outcome = producer
        .call(contract.id(), "respond")
        .args_json(json!({"new_message": "Goodbye"}))
        .transact()
        .await?;
    let producer_message = contract
        .view("get_greeting")
        .args_json(json!({}))
        .await?;
    assert_eq!(producer_message.json::<String>()?, "Goodbye");

    Ok(())
}
