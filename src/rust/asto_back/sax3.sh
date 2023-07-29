#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir/.."
# cargo run -p asto_back -- -w asto_back sax xml/2022-11-06/svd.xml xml/2022-11-06/smm.xml
sources=(
    # xml/2022-10-09/smm.xml
    # xml/2022-10-09/svd.xml # non_registered: 50/131
    #
    # xml/2022-11-06/smm.xml
    # xml/2022-11-06/svd.xml # non_registered: 38/123
    #
    # xml/2022-12-04/smm.xml
    # xml/2022-12-04/svd.xml # non_registered: 53/136
    #
    # xml/2022-12-25/s6.xml # non_registered: 22/112
    #
    # xml/2023-01-29/s6.xml # non_registered: 135/234

    # xml/2023-02-19/s6.xml  # non_registered: 44/136
    # xml/2023-02-19/svd-test.xml 

    # xml/2022-10-09/smm-test.xml
    # xml/2022-11-06/smm-test2.xml
    # xml/2022-11-06/svd-test2.xml
    # xml/2022-11-06/svd-test3.xml
    # xml/2022-11-06/svd-test2.xml
    # xml/2023-03-19/s6.xml 

    # xml/2023-04-16/smm.xml
    # xml/2023-04-16/svd.xml
    # xml/2023-04-16/s6_edited.xml

    # xml/2023-04-23/smm.xml
    # xml/2023-04-23/svd_edited.xml

    xml/2023-05-13/s6_edited.xml
)
cargo_run_opts=(
    # --release
)
app_opts=(
    --db-url "postgres://a100:1014103a100@v9z.ru:54495/a100"
    # --db-url "postgres://a100:1014103a100@v9z.ru:54495/a100"
)
cargo run -p asto_back "${cargo_run_opts[@]}" -- -w asto_back sax3 "${sources[@]}" "${app_opts[@]}" "$@" 


