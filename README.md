# Replex
This project is a hard fork of the original [lostb1t/replex project](https://github.com/lostb1t/replex/).
All credit goes to the original author, this hard fork was my way of learning Rust.

The project is in a working state at the moment, but I have not tested it extensively with different clients.

## How to run
```bash
$ cargo watch -x run
```

## Config Yaml
Configuring the project can still be done via environment variables, but doing so using a yaml file is also supported.

The config file is located at `config/config.yaml`. Here is an example config file:
```yaml
# Plex host
host: "http://localhost:32400"

# Port that the Replex server will run on
port: 3001

# Rust log level
rust_log: "info"

# Caching
cache:
  enabled: true
  ttl: 3600
  auto_refresh: true

# Redirect streams directly to the Plex server, bypassing Replex.
# Optionally specify a host to redirect streams to.
redirect_streams: 
  enabled: true
  host:
  
# Auto select the media version according to the client's resolution
auto_select_version: true

# Set the quality of a stream to the maximum available quality,
# does not prevent transcoding if the client does not support the codec.
force_maximum_quality: true

# If a transcode for this quality is triggered, fall back to a lower quality
transcode_fallback_for: "4K"

# Disable related content
disable_related: true

# Hide leaf count on items
disable_leaf_count: true

# Hide user state on items
disable_user_state: true

# Better "Continue Watching" rows, will disable the default "Continue Watching" and "On Deck" hubs.
# Provide the collection names for your `in_progress` and `next_up` hubs.
# Replex will handle episode sorting and hub order. 
better_on_deck:
  enabled: true
  in_progress: "Continue watching"
  next_up: "Jump back in"


# NOT AVAILABLE YET
# Either exclude all watched items from collections,
# or specify a list of collections to exclude watched items from.
# exclude_watched:
#   all: true
#   collections:

# List of Plex default collections that should be shown as hero rows
# To have custom collecitons display as hero rows, add the label "REPLEXHERO" to the collection.
hero_rows:
  - home.movies.recent
  - movies.recent
  - movie.recentlyadded
  - movie.topunwatched
  - movie.recentlyviewed
  - hub.movie.recentlyreleased
  - home.television.recent
  - tv.inprogress
  - tv.recentlyaired

# List of hubs that will be sorted to the top of the home screen
priority_hubs:
```

## Features 
Note: for all features see the config.yaml example

### Interleaved rows
Collection hubs with the same name from different libraries will be merged into one on the home screen.
So a collection named "Trending" in the Movie library will be merged with a collection named "Trending" from a TV Show library on the home screen.

### Redirect streams
Useful for when you're on an app box, in which case it might not be ideal to stream media through Replex.

Note: Plex doesn't handle redirects well, and will not remember them.
So every chuck of a stream will first hit Replex before being redirected to the redirect url.
It is recommended to run Replex on the same machine as the Plex server.

### Auto select version
If you have multiple versions of a media item then this setting will choose the one that's closest to the client resolution. 
So a 1080p TV will get the 1080P version while 4k gets the 4k version. 
A user can still override this by selecting a different version from the client.

### Force maximum quality
This will force clients to use the maximum quality. 
Meaning that if a client requests anything other than the maximum quality this will be ignored,
and the maximum quality (direct play/stream when server allows for original) is used instead. 
This doesn't prevent transcoding. It only sets the bitrate to original quality. 
So if a client needs a different codec, container or audio it should still transcode.

### Transcode fallback for
If the selected media triggers a video transcode, fallback to another version of the media. 
Only triggers on video transcoding. Remuxing is still allowed.
Options are "4k" and "1080".

Example: if `transcode_fallback_for` is set to "4k" then 4k transcodes will fall back to another version if available.

### Disable related content
See: https://github.com/lostb1t/replex/issues/26

### Better on Deck
The `better_on_deck` feature is an attempt to improve (subjective) the "Continue Watching" and "On Deck" hubs on the Plex home screen.
This feature will disable the default "Continue Watching" and "On Deck" hubs and replace them with custom hubs.

One improvement is that Plex sometimes "forgets" that you've watched something, for instance when switching between Plex servers.
This feature will try to find shows that you've watched but not finished and add them to the "Continue Watching" hub (sorting may be off until a "last_watched" date is available).

The `in_progress` and `next_up` fields are the names of the collections that will be used to populate the new hubs.
If you're using Kometa (formerly Plex Meta Manager) you can use the following collections for each library:

#### Movie libraries
```yaml
Continue watching:
  summary: A list of movies that are in progress
  smart_filter:
    all:
      progress: true
    sort_by: viewed.desc
  label.sync: "REPLEXHERO"
  visible_home: true
  visible_shared: true
  visible_library: true
  collection_filtering: user
  minimum_items: 0
````
#### TV show libraries
```yaml
collections:
  Continue watching: 
    summary: A list of episodes that are in progress
    builder_level: episode
    smart_filter:
      all:
        episode_progress: true
      sort_by: viewed.desc
    label.sync: "REPLEXHERO"
    visible_home: true
    visible_shared: true
    visible_library: true
    collection_filtering: user
    minimum_items: 0
  Jump back in:
    summary: Jump back into shows you are watching
    smart_filter:
      all:
        episode_last_watched: 365 * 5
        episode_plays.gt: 0
        unplayed_episodes: true
      sort_by: viewed.desc
    label.sync: "REPLEXCARD"
    visible_home: true
    visible_shared: true
    visible_library: true
    collection_filtering: user
```

### Exclude watched items (in progress)
Currently, you can hide watched items from collections by adding the `REPLEX_EXCLUDE_WATCHED` label to the collection.

### Hero style rows
For custom collections you can change the hub style to hero by setting the label "REPLEXHERO" on a collection.

For built-in rows you can use the config file, these are the options:
- home.movies.recent
- movies.recent
- movie.recentlyadded
- movie.topunwatched
- movie.recentlyviewed
- hub.movie.recentlyreleased
- movie.recentlyreleased
- home.television.recent
- tv.recentlyadded
- tv.toprated
- tv.inprogress
- tv.recentlyaired

### Priority hubs
You can set a list of hubs that will be sorted to the top of the home screen.

Note: the better on deck will ignore this list and still sort `in_progress` and `next_up` to the top.

## Remote access
Because this app sits in front of Plex, the built-in remote access (and auto SSL) will not work and needs to be disabled.

For testing purposes you can access through the browser at http://[replexip]:[replexport] (ex: http://localhost:3001).
But if you want other clients to connect to Replex you need to set up a reverse proxy with a domain and preferably ssl.

Some easy to set up reverse proxy managers are: https://caddyserver.com or https://nginxproxymanager.com

Once you have your domain hooked up to Replex add your Replex url to 'Custom server access URLs' 
field under the network settings in Plex, and lastly disable remote access under remote access.

Clear you client's cache to force Plex reloading the custom server url.

Note: SSL is highly suggested, some clients default to not allowing insecure connections. 
And some clients don't even support insecure connections (app.plex.tv).

## Known limitations
- Hero hubs on Android devices don't load more content. So hero hubs have a maximum of 100 items on Android.
- On Android mobile, hero elements in libraries are slightly cut-off. This seems to be a Plex limitation.
- When exclude_watched is true a maximum item limit per library is opposed of 250 items. 
  So if you have a mixed row of 2 libraries the max results of that row will be 500 items.
- disable_user_state: For movies this works in the webapp. Shows work across clients.