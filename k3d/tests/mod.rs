use k3d::{cluster::K3dClusterCmd, Error, K3d, K3dCmd, K3dRun};

#[test]
fn main() -> Result<(), Error> {
    let cluster = "my-cluster".to_string();
    let mut k3d_create = K3d::new().cluster().create(cluster.clone());
    k3d_create.run().unwrap();

    let mut k3d_stop = K3d::new().cluster().stop(vec![cluster.clone()]);
    k3d_stop.run().unwrap();

    let mut k3d_start = K3d::new().cluster().start(vec![cluster.clone()]);
    k3d_start.run().unwrap();

    let mut k3d_delete = K3d::new().cluster().delete(vec![cluster]);
    k3d_delete.run().unwrap();

    Ok(())
}
