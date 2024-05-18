# Replex

### In progress...

## How to run
```bash
$ cargo watch -x run
```

## Config Yaml
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
disabale_leaf_count: true

# Hide user state on items
disable_user_state: true

# Better "Continue Watching" rows, will disable the default "Continue Watching" and "On Deck" hubs.
# Provide the collection names for your `in_progress` and `next_up` hubs.
# Replex will handle episode sorting and hub order. 
better_on_deck:
  enabled: true
  in_progress: "Continue watching"
  next_up: "Jump back in"


# Either exclude all watched items from collections,
# or specify a list of collections to exclude watched items from.
exclude_watched:
  all: true
  collections:

# List of Plex default collections that should be shown as hero rows
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