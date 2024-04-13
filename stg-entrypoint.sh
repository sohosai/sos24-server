#!/bin/bash

echo "$(TZ=JST-9 date +%Y-%m-%d\ %H:%M:%S) (JST) - SOS24 STAGING SERVER: create database"
cargo sqlx database create
echo "$(TZ=JST-9 date +%Y-%m-%d\ %H:%M:%S) (JST) - SOS24 STAGING SERVER: run migration and payload demo data"
cargo sqlx migrate run

echo "$(TZ=JST-9 date +%Y-%m-%d\ %H:%M:%S) (JST) - SOS24 STAGING SERVER: start sos24 server"
echo "$(TZ=JST-9 date +%Y-%m-%d\ %H:%M:%S) (JST) - Warning: This is the staging environment."

exec /usr/local/bin/sos24-presentation
