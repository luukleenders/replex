use anyhow::Result;
use async_trait::async_trait;

use crate::config::Config;
use crate::models::MediaContainer;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;

use super::Transform;

#[derive(Default, Debug)]
pub struct HideInProgressTransform;

#[async_trait]
impl Transform for HideInProgressTransform {
    async fn transform_mediacontainer(
        &self,
        container: &mut MediaContainer,
        _plex_client: &PlexClient,
        _options: &PlexContext,
    ) -> Result<()> {
        let config = Config::load();

        if container.hub.is_empty() || !config.better_on_deck {
            return Ok(());
        }

        println!("Hiding in progress items");

        let hubs = container.children_mut();
        let context = Some("hub.tv.inprogress".to_string());

        hubs.retain(|x| x.context != context);

        Ok(())
    }
}
