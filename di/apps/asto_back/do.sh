#!/usr/bin/env bash

source "$(realpath "$(dirname "${BASH_SOURCE[0]}")/../../sh/core/do.common.sh")"

src_rust_dir="$proj_dir/src/rust"
target="x86_64-unknown-linux-musl"
exe="target/$target/release/./$app"
dependencies_for_deploy=( # https://askubuntu.com/questions/552120/preserve-directory-tree-while-copying-with-rsync
    "$src_rust_dir/$exe" 
    "$src_rust_dir/$app/./.env"
    "$src_rust_dir/$app/./settings.toml"
    "$src_rust_dir/$app/src/./css/index.css"
    "$src_rust_dir/$app/src/./css/style.css"
    "$src_rust_dir/$app/src/./css/portrait.css"
    "$src_rust_dir/$app/src/./css/assets/favicon.ico"
    "$src_rust_dir/$app/./sales@yury.bikuzin.42.json"

    "$src_rust_dir/$app/./xml/2022-10-09/smm.xml"
    "$src_rust_dir/$app/./xml/2022-10-09/svd.xml"

    "$src_rust_dir/$app/./xml/2022-11-06/smm.xml"
    "$src_rust_dir/$app/./xml/2022-11-06/svd.xml"

    "$src_rust_dir/$app/./xml/2022-12-04/smm.xml"
    "$src_rust_dir/$app/./xml/2022-12-04/svd.xml"

    "$src_rust_dir/$app/./xml/2022-12-25/s6.xml"
    "$src_rust_dir/$app/./xml/2023-01-29/s6.xml"
    "$src_rust_dir/$app/./xml/2023-02-19/s6.xml"
    "$src_rust_dir/$app/./xml/2023-03-19/s6.xml"

    # "$src_rust_dir/$app/./xml/2023-04-16/smm.xml"
    # "$src_rust_dir/$app/./xml/2023-04-16/svd.xml"
    "$src_rust_dir/$app/./xml/2023-04-16/s6_edited.xml"

    "$src_rust_dir/$app/./xml/2023-04-23/smm.xml"
    "$src_rust_dir/$app/./xml/2023-04-23/svd.xml"

    "$src_rust_dir/$app/./xml/2023-05-13/s6.xml"
    "$src_rust_dir/$app/./xml/2023-05-13/s6_edited.xml"
)

case $cmd in
    build )
        set -e
        pushd "$src_rust_dir" 
        x cargo build --release --target $target -p $app 
        x ls -lah $exe 
        target_files=()
        popd 
    ;;
    get-dependencies-for-deploy )
        echo "${dependencies_for_deploy[@]}"
    ;;
    deploy )
        [[ $dry_run ]] || set -e
        x $dry_run $src_rust_dir/$exe -w "$src_rust_dir/$app" -t 
        x $dry_run ssh "$host" "mkdir -p $proj/$kind/$app" 
        x $dry_run rsync -avz --relative "${dependencies_for_deploy[@]}" $host:$proj/$kind/$app/ # https://askubuntu.com/questions/552120/preserve-directory-tree-while-copying-with-rsync
    ;;
    after-deploy )
        service_name="${app}_$kind"
        if [[ $(ssh $host "ls /etc/systemd/system/${service_name}.service") ]]; then
            cmd="sudo systemctl restart ${app}_$kind && sudo systemctl enable ${app}_$kind"
            x $dry_run ssh $host "cd $proj/$kind/$app/ && $cmd"
            url="https://asto.dance"
            # case $host in 
            #     abc ) url=https://abc.baza-winner.ru ;;
            #     v9z ) url=https://z9v.ru ;;
            #     bikuzin18 ) url=https://bikuzin18.baza-winner.ru ;;
            #     wgate ) url=https://export.baza-winner.ru ;;
            #     *) echoerr "Unexpected host: $host"
            # esac
            route=/$app
            prefix=
            if [[ $kind == 'prod' ]]; then
                prefix=""
            else
                prefix="/$kind"
            fi
            url="$url$prefix$route/health"
            x $dry_run curl "$url"
            cat << EOM
== DID DEPLOY AND $cmd
EOM
        elif [[ -e $di_dir/apps/$app/systemd.service ]]; then
            cat << EOM
== AFTER DEPLOY NOTE:
    run $di_dir/$kind/$app/systemd.sh
    OR
    Enter to '$host' via ssh and run 'cd $proj/$kind/$app && ./$app server' in tmux session
    To leave ssh session use Enter-tilda-dot sequence (Enter ~ .)
EOM
        else
            ls 
            cat << EOM
== AFTER DEPLOY NOTE:
    Enter to '$host' via ssh and run 'cd $proj/$kind/$app && ./$app server' in tmux session
    To leave ssh session use Enter-tilda-dot sequence (Enter ~ .)
EOM
        fi
    ;;
esac

