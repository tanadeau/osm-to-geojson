#!/bin/bash

APP_NAME="osm-to-geojson"
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker build -t $DOCKER_USERNAME/$APP_NAME:$TRAVIS_TAG .
ID="$(docker images | grep $APP_NAME | head -n 1 | awk '{print $3}')"
docker tag "$ID" $DOCKER_USERNAME/$APP_NAME:latest
docker push $DOCKER_USERNAME/$APP_NAME
