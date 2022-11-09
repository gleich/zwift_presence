use discord_rich_presence::{
    activity::{Activity, Assets},
    DiscordIpc, DiscordIpcClient,
};

use crate::parse::Data;

pub fn connect() -> Result<DiscordIpcClient, Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1039963166362320927")?;
    client.connect()?;
    Ok(client)
}

pub fn update(data: Data, client: &mut DiscordIpcClient) -> Result<(), Box<dyn std::error::Error>> {
    client.set_activity(
        Activity::new()
            .state("Cycling")
            .details("foo bar")
            .assets(Assets::new().large_image("zwift")),
    )?;
    Ok(())
}
