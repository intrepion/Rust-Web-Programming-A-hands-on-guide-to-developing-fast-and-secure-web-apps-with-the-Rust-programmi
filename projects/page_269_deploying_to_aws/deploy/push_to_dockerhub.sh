#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

sudo docker build -t rust_app .

sudo docker tag rust_app:latest intrepion/actix_web_application:latest

sudo docker login
sudo docker push intrepion/actix_web_application:latest
