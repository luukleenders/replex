# Replex

![plot](./examplewithhero.png)

Plex proxy with the following features:

- Merge recommendations on home into one from different libraries. Aka have movies and shows in a single row.
- Hide watched items from recommendations.
- Choose between styles, shelf (default) or hero.
- Plex doesnt apply user restrictions to collection recommendations. this proxy fixes that so you can have unique rows for every user.
- Works on every client/app not only plex web!
- Plays nice with PMM (and without).

!!This does not alter your plex data in anyway. it only alters outgoing api requests. All your collections or rows are kept intact!!

### Usage example

_Make sure that the collections you want to merge have the same name in your different libraries as the merge happens by name. 
Example: an collection named "Trending" in a Movie library will be merged with an collection named "Trending" from a shows library on home._

Run the docker image with REPLEX_HOST set to your plex instance.

```
docker run --rm -it -p 80:80 -e REPLEX_HOST="http://10.0.0.3:42405" ghcr.io/sarendsen/replex:latest
```

add your proxy url to plex "Custom server access URLs" (ex http://0.0.0.0:80)

then access your proxy url http://0.0.0.0:80

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

if using a domain set 'Custom certificate domain' to your domain (without http/https) and to your 'Custom server access URLs' (with http/https).
if using an ip only set 'Custom server access URLs'. By using your IP you will not have SSL tho.

Disable remote access and it will route everything properly

## Docker

2 images are provided:

- `replex`
- `replex-standalone`

This isnt a fully fledged proxy and doesnt aim to be. The default docker image `replex` includes nginx. 
If you already have a reverse proxy running and want to ax nginx you can route the following paths (and it subpaths) to this app and use the `replex-standalone` image. 

- /hubs (excluding /hubs/search)
- /replex

But there is no real reason to use the standalone version. And i advice against using it, as there is no harm or real overhead in using the main and is already configured properly. 
