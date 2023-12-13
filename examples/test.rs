use do_sdk::client::{droplets::DropletHelper, model::CreateOneDropletReq, DoClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // test_do.await?;

    Ok(())
}
async fn test_do() -> anyhow::Result<()> {
    let token = std::env::var("DO_TOKEN").expect("not set DO_TOKEN");
    let client = DoClient::new(token);
    let r = client.droplets().list().await?;
    println!("{:?}", r);

    client.droplets().delete("123").await?;

    let create_req = CreateOneDropletReq {
        name: uuid::Uuid::new_v4().to_string(),
        region: Some("nyc1".into()),
        size: "s-1vcpu-1gb".into(),
        image: "ubuntu-22-04-x64".into(),
        ssh_keys: vec![],
    };
    let r = client.droplets().create(create_req).await?;
    let id = r.droplet.id;
    println!("id: {:?}, {:?}", id, r);
    let r = client.droplets().wait_creating(id).await?;
    println!("{:?}", r);

    let r = client.ssh_keys().list_all_ssh_keys().await?;
    println!("{:?}", r);

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let r = client.droplets().list().await?;
    println!("{:?}", r);

    Ok(())
}
