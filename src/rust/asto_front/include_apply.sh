#!/usr/bin/env bash
set -e
sudo nginx -t && sudo nginx -s reload
