#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
files=(
    # eventlar.sql
    # event_resultlar.sql
    # get_init_data.sql
    # export_dancerlar_for_anton.sql
    add_event_result.sql
)
for file in "${files[@]}"; do
    ./psql-f.sh ../sql/$file
done
