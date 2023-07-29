#!/usr/bin/env bash
set -e
# dir=$(dirname "$0")
# cd "$dir"
# https://stackoverflow.com/questions/9736085/run-a-postgresql-sql-file-using-command-line-arguments
    # echo "OK: '$1'"
export PGPASSWORD=1014103a100
# cmd=( psql -h localhost -U a100 -d a100 "$@")
cmd=( psql -h v9z.ru -p 54495 -U a100 -d a100 "$@")
echo "${cmd[@]}"
"${cmd[@]}"
