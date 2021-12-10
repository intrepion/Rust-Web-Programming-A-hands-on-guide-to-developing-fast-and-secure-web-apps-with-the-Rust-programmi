#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

scp -i "./rust_app.pem" ./docker-compose.yml ec2-user@34.217.22.197:/home/ec2-user/docker-compose.yml
scp -i "./rust_app.pem" -r ./nginx ec2-user@34.217.22.197:/home/ec2-user/nginx

ssh -i "./rust_app.pem" -t ec2-user@34.217.22.197 << EOF
    docker-compose stop
    docker container rm rust_app
    docker image rm intrepion/actix_web_application

    docker-compose up -d
    docker container exec -t rust_app diesel migration run
    rm -r nginx/
    rm docker-compose.yml
EOF

curl --header "Content-Type: application/json" --request POST --data '{"name":"intrepion", "email":"intrepion@gmail.com", "password":"test"}' 34.217.22.197/api/v1/user/create
