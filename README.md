# Luffy
it hook pirate hook it luffy

## Code
### Service
Service defines a Git repository service provider.
Each has their own header for signifying the type of the event,
as well as their own types of event, although they can often be similar.

An implementation is provided for Gitea.
Service exposes a function to parse the header and body of a web hook request
into it's variants, which is then passed to a Handler which can handle that
type of variant.

### Handler
This defines the way you want to handle the service event.
An implementation provided assigns environment variables to names derived from
the structure of the web hook event.
The implementation provided reads a config file when a hook event is received
and calls the function from there. The function there can take advantage of
the environment variables as they wish.

### Main
Currently, this utilizes `warp` to spin up a web server.
It accepts an address and port to bind to,
as well hostnames which are allowed to hit the endpoint.
This will likely move into some `examples` folder in the future,
and an implementation using the Rust AWS Lambda runtime would be nice over using `warp`.

### Container
The Dockerfile essentially a placeholder at the moment with how garbage it is.
You will need to build and target musl for the time being for the executable to
be available to the docker build.

### Similar projects
From a cursory glance, it looks like [`adnanh/webhook`](https://github.com/adnanh/webhook)
is a much more mature project with much the same idea. I did not look further to see if it
supports Gitea events, but I would guess it would be a simple contribution if not yet available.
I would strongly suggest using other projects such as that at the moment, until this project
is more mature. (if it gets there)
