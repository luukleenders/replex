# Replex

![plot](./examplewithhero.png)

Plex proxy with the following features:

- Merge recommendations on home into one from different libraries. Aka have movies and shows in a single row.
- Hide watched items from recommendations.
- Choose between styles, shelf (default) or hero.
- Plex doesnt apply user restrictions to collection recommendations. this proxy fixes that so you can have unique rows for every user.
- Works on every client!

### Usage example

**sMake sure that the collections you want to merge have the same name as it will be merged by name.**s

Run the docker image with REPLEX_HOST set to your plex instance.

```
docker run --rm -it -p 80:80 -e REPLEX_HOST="http://10.0.0.3:42405" ghcr.io/sarendsen/replex-nginx:latest
```

add your proxy url to plex "Custom server access URLs" (ex http://0.0.0.0:80)

then access your proxy url http://0.0.0.0:80

NOTICE: this isnt a fully fledged proxy and doesnt aim to be. I suggest putting it behind a proper (reverse) proxy and only route the following paths (and it subpaths) to this app. 

- /hubs
- /replex

If you dont have a reverse proxy an docker image including nginx exists at ghcr.io/sarendsen/replex-nginx and a version without nginx at ghcr.io/sarendsen/replex.

### Settings
Settings are set via [environment variables](https://kinsta.com/knowledgebase/what-is-an-environment-variable/) 

| Setting        	       | Default 	| Description                                                            	|
|--------------------------|------------|---------------------------------------------------------------------------|
| REPLEX_HOST              | -      	| Plex target host to proxy                                             	|
| REPLEX_INCLUDE_WATCHED   | false    	| If set to false, hide watched items.                        	|

### Hub style

You can change the hub style to hero elements by setting the label "REPLEXHERO" on an collection. 

### Force clients to use the proxy

To force plex clients to use your proxy you can either use a custom domain with or without ssl or use your server ip.

if using a domain set 'Custom certificate domain' to your domain (without http/https) and to your 'Custom server access URLs' (with http/https)
if using an ip only set 'Custom server access URLs'
Disable remote access and it will route everything properly