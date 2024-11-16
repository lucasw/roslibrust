#[cfg(feature = "ros1")]
roslibrust_codegen_macro::find_and_generate_ros_messages!("assets/ros1_common_interfaces");

#[cfg(feature = "ros1")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use roslibrust::ros1::NodeHandle;

    env_logger::init();

    let nh = NodeHandle::new("http://localhost:11311", "talker_rs")
        .await
        .map_err(|err| err)?;
    let publisher = nh
        .advertise::<geometry_msgs::PointStamped>("/my_point", 1, false)
        .await?;

    let mut count = 0;
    loop {
        let mut msg = geometry_msgs::PointStamped::default();
        msg.header.seq = count;
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time went backwards");
        msg.header.stamp = roslibrust_codegen::Time { secs: time.as_secs() as u32, nsecs: time.subsec_nanos() };
        msg.point.x = ((count as f64) / 200.0).sin();
        publisher.publish(&msg).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        count += 1;
    }

    // Ok(())
}

#[cfg(not(feature = "ros1"))]
fn main() {
    eprintln!("This example does nothing without compiling with the feature 'ros1'");
}
