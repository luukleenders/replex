build:
	cargo build --release

fix:
	cargo fix

run:
	REPLEX_HOST="http://plex" \
	REPLEX_VIDEO_TRANSCODE_FALLBACK_FOR="4k" \
	REPLEX_AUTO_SELECT_VERSION=0 \
	REPLEX_FORCE_MAXIMUM_QUALITY=1 \
	REPLEX_HERO_ROWS="home.movies.recent,movies.recent,movie.recentlyadded,movie.topunwatched,movie.recentlyviewed,hub.movie.recentlyreleased,home.television.recent,tv.inprogress,tv.recentlyaired" \
	REPLEX_EXCLUDE_WATCHED=0 \
	REPLEX_DISABLE_RELATED=0 \
	REPLEX_DISABLE_LEAF_COUNT=0 \
	REPLEX_DISABLE_USER_STATE=0 \
	REPLEX_REDIRECT_STREAMS=1 \
	REPLEX_CACHE_ROWS=1 \
	REPLEX_CACHE_TTL=10000 \
	REPLEX_PORT=3001 \
	RUST_LOG="info,replex=debug" \
	cargo watch -x run

run-tests:
	cargo test -- --nocapture