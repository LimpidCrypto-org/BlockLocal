use k3d::{
    cluster::K3dNodeCmd,
    node::{create::K3dNodeCreateArgs, K3dClusterCmd},
    Error, K3d, K3dCmd, K3dRun,
};

#[test]
fn test_cluster_cycle() -> Result<(), Error> {
    let cluster = "my-cluster";
    let mut k3d_create = K3d::new().cluster().create(cluster);
    k3d_create.run().unwrap();

    let mut k3d_stop = K3d::new().cluster().stop(vec![cluster]);
    k3d_stop.run().unwrap();

    let mut k3d_start = K3d::new().cluster().start(vec![cluster]);
    k3d_start.run().unwrap();

    let mut k3d_delete = K3d::new().cluster().delete(vec![cluster]);
    k3d_delete.run().unwrap();

    Ok(())
}

#[test]
fn test_node_cycle() -> Result<(), Error> {
    let cluster = "my-cluster";
    let node = "my-node";
    let mut k3d_create = K3d::new().cluster().create(cluster);
    k3d_create.run().unwrap();

    let mut k3d_node_create = K3d::new().node().create(&node).cluster(&cluster);
    k3d_node_create.run().unwrap();

    let mut k3d_node_stop = K3d::new()
        .node()
        .stop(&format!("{}-{}", K3d::build_name(node), 0));
    k3d_node_stop.run().unwrap();

    let mut k3d_node_start = K3d::new()
        .node()
        .start(&format!("{}-{}", K3d::build_name(node), 0));
    k3d_node_start.run().unwrap();

    let mut k3d_node_delete = K3d::new()
        .node()
        .delete(&format!("{}-{}", K3d::build_name(node), 0));
    k3d_node_delete.run().unwrap();

    let mut k3d_delete = K3d::new().cluster().delete(vec![cluster]);
    k3d_delete.run().unwrap();

    Ok(())
}
