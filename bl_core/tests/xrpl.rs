use bl_core::blockchain::{Blockchain, BlockchainCluster, BlockchainNode};

#[tokio::test]
async fn test_xrpl_cluster() {
    let xrpl = Blockchain::new(
        "xrpl".to_string(),
        "https://example.com".parse().unwrap(),
        None,
        None,
    );
    BlockchainCluster::create(&xrpl, None).await.unwrap();
    BlockchainNode::create(&xrpl, "validator", None)
        .await
        .unwrap();
}
