# Waybackwhen - Daily screenshots of old webpages via waybackmachine

## Current status
Waiting on access in GCP to set secrets correctly. The slack token was initially
set as a normal environment variable, but is not removed. You have to change the
channel id when things are ready

## Deployment
The application run as a scheduled job using Google Cloud Run. Its set to a
specific image, so to deploy a new version you have to change this url. You can
most likely do this with the CLI but I've not tried yet.

The docker images are hosted on Google Artifact Registry. You have to be
particaly about how you tag it.

```bash
> ./tag-for-deployment.sh
> #optionally you can add a tag 
> ./tag-for-deployment.sh 2b1c708
```
Then you push to the registry as long as you have set up auth:

```bash
> gcloud init                                               # one time if gcloud is not set up
> gcloud auth configure-docker europe-north1-docker.pkg.dev # one time
> docker push europe-north1-docker.pkg.dev/marine-cycle-97212/cloud-run-source-deploy/waybackwhen:YOUR_TAG
```

Make sure you update the tag above ^

## Development

### Run the application locally

Remember to set up the environment variables as illustrated in `env.example`.

```bash
> cargo run
```

### Debug docker

This is a nice command for testing out things inside docker (I had some issues
with this, but it should all be solved now)

```bash
> docker run --name wayback --rm --env-file .env waybackwhen:YOUR_TAG
```

and if you want to peek inside the container instead of running the built binary:

```bash
> docker run --name wayback --rm --env-file .env  -it waybackwhen:YOUR_TAG /bin/sh
```

# Possible features

* Custom timelines for different sites
* Dashboard for configuring websites and timelines

# TODO

* Add ci/cd for deploying new versions. Good luck!
* Add monitoring so that error are reported

# Bugs

* When an image upload fails the script doesnt report an error. This often
  happens if the bot is not in the channel.
