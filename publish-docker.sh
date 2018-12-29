#!/bin/bash

APP_NAME="osm-to-geojson"
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker build -t $APP_NAME .
docker tag $APP_NAME $DOCKER_USERNAME/$APP_NAME:$TRAVIS_TAG
docker push $DOCKER_USERNAME/$APP_NAME
