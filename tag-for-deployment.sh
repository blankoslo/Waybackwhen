#!/bin/bash
# tag docker image for pushing to artifact registry on google cloud
TAG=${1:-latest}
docker tag waybackwhen:$TAG europe-north1-docker.pkg.dev/marine-cycle-97212/cloud-run-source-deploy/waybackwhen:$TAG\
    && echo -e "\nImage waybackwhen:$TAG was tagged\n" && docker image ls | grep -E 'waybackwhen|REPOSITORY'
