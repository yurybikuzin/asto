#!/usr/bin/env bash
set -e
# dir=$(dirname "$0")
# cd "$dir"
# https://stackoverflow.com/questions/9736085/run-a-postgresql-sql-file-using-command-line-arguments
if [[ -e "$1" ]]; then
    export PGPASSWORD=1014103a100
    DB="a100"
    USER="a100"
    
    # HOST="localhost"
    # PORT="5432"

    HOST="v9z.ru"
    PORT="54495"

    cmd=( psql -h $HOST -p $PORT -U $USER -d $DB -a -f $1)
    echo "${cmd[@]}"
    "${cmd[@]}" | grep ERROR
elif [[ "$1" ]]; then
    echo "ERR: not found file '$1'"
else
    echo "ERR: not specified FILE"
fi
