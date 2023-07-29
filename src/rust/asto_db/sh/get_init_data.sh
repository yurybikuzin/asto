#!/usr/bin/env bash
set -e
dir=$(dirname "$0")
cd "$dir"
PGPASSWORD=1014103a100 psql -h v9z.ru -p 54495 -U a100 a100 -c "select get_init_data(-1::smallint)" -A -t | jq . > init_data.json
