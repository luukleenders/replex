use anyhow::Result;
use async_trait::async_trait;

use crate::config::Config;
use crate::models::MediaContainer;
use crate::plex::client::PlexClient;
use crate::plex::models::PlexContext;
use crate::plex::traits::User;
use super::Transform;

#[derive(Default, Debug)]
pub struct MaxResolutionTransform;

#[async_trait]
impl Transform for MaxResolutionTransform {
    async fn transform_mediacontainer(
        &self, container: &mut MediaContainer, plex_client: &PlexClient,
        options: &PlexContext,
    ) -> Result<()> {
        if container.hub.is_empty() || options.token.is_none() {
            return Ok(());
        }

        let config = Config::load();
        let token = options.token.clone().unwrap();
        let user = User::get(plex_client, token).await?;

        let max_1080p = config
            .force_resolution
            .max_1080p
            .as_ref()
            .map_or(false, |vec| vec.contains(&user));

        let max_4k = config
            .force_resolution
            .max_4k
            .as_ref()
            .map_or(false, |vec| vec.contains(&user));


        if !max_1080p && !max_4k {
            return Ok(());
        }

        for hub in &mut container.hub {
            if hub.size.unwrap_or_default() == 0 {
                continue;
            }

            println!("Hub: {:?}", hub);
        }

        Ok(())
    }
}
