#!/usr/bin/env bash
set -e

source "$(realpath "$(dirname "${BASH_SOURCE[0]}")/../../sh/core/do.common.sh")"
source "$di_dir/sh/start_prepare_Makefile_build.2.sh"
source "$di_dir/sh/build.css.2.sh"
# source "$di_dir/sh/build.js.sh"

src_rust_dir="$proj_dir/src/rust"
src_dir="$src_rust_dir/$app"
dependencies_for_deploy=(
    $(fd -I '\.(html|css|svg|webp|jpg|png|ico|js|wasm|json)$' -a "$src_dir")
)

rsync_sources=(
    $src_dir/pkg 
    $src_dir/src/assets 
    $src_dir/src/css 
    $src_dir/src/index.html
)
case $cmd in
    build )
        pushd "$src_dir" 

		styles=( 
            "$src_dir/src/css/admin.scss" 
            "$src_dir/src/css/style.scss" 
            "$src_dir/src/css/portrait.scss" 
            "$src_dir/src/css/landscape.scss" 
        )
        fill_target_files_with_styles
        target_file_wasm="$(pwd)/pkg/${app}_bg.wasm"
        target_files+=( "$target_file_wasm" )
        {
            start_prepare_Makefile_build
            build_css 

            wasm_deps=( $(fd -I '\.(rs|toml)$' -a .) )
        cat << EOM
$target_file_wasm: ${wasm_deps[@]}
EOM
        printf "\twasm-pack build --target web --no-typescript --release\n\n"

        } > "$Makefile_build"

        if [[ $force ]]; then
            make --always-make -f "$Makefile_build"
        else
            make -f "$Makefile_build" 
        fi

        popd 
    ;;
    get-dependencies-for-deploy )
        echo "${dependencies_for_deploy[@]}"
    ;;
    deploy )
        if [[ false ]]; then
        [[ $dry_run ]] || set -e
        x $dry_run ssh "$host" "mkdir -p $proj/$kind/$app" 
        # x $dry_run rsync -avz "${rsync_sources[@]}" $host:$proj/$kind/$app/
        rsync_sources=(
            $( 
                printf "%s\n" "${dependencies_for_deploy[@]}"\
                | rust-script --loop '|l| print!("{}", &l'[${#src_dir}' + 1..])'\
                | grep -vE "^src/" 
            )
        )
        cd $src_dir
        x $dry_run rsync -avz -R "${rsync_sources[@]}" "$host:$proj/$kind/$app/"
        sub_dir="src/"
        rsync_sources=(
            $( 
                printf "%s\n" "${dependencies_for_deploy[@]}" \
                | rust-script --loop '|l| print!("{}", &l'[${#src_dir}' + 1..])' \
                | grep -E "^$sub_dir" \
                | rust-script --loop '|l| print!("{}", &l'[${#sub_dir}'..])'
            )
        )
        cd $sub_dir
        x $dry_run rsync -avz -R "${rsync_sources[@]}" "$host:$proj/$kind/$app/"
        fi
    ;;
    after-deploy )
        source "$di_dir/sh/$cmd.sh"
#         cat << EOM
# == AFTER DEPLOY NOTE:
#     Enter to '$host' via ssh and run 'cd $proj/$kind/$app && ./$app' in tmux session
#     To leave ssh session use Enter-tilda-dot sequence (Enter ~ .)
# EOM
#         cat << EOM
# == AFTER DEPLOY NOTE:
#     Enter to '$host' via ssh and run 'cd $proj/$kind/$app && ./$app' in tmux session
#     To leave ssh session use Enter-tilda-dot sequence (Enter ~ .)
# EOM
    ;;
    * )
        echoerr "unexpected \$cmd '$cmd'"
    ;;
esac


