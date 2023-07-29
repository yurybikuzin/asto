#!/usr/bin/env bash
set -e
dir=$(dirname "$0")
if [[ ! $(which nginx) ]]; then
    sudo mkdir -p /etc/letsencrypt
    sudo chmod a+rwx /etc/letsencrypt
    sudo apt update && sudo apt install -y nginx
fi;
sudo rsync -avz "$dir/nginx/" /etc/nginx/
set +e
rsync -avz -R --rsync-path="sudo rsync" z9v:/etc/letsencrypt /
set -e
sudo nginx -t && sudo nginx -s reload
