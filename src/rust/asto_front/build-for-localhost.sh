#!/usr/bin/env bash
set -e
dir=$(dirname "$0")
cd "$dir"
pushd src/css
rm -f "index.css"
scss=( "style.scss" "print.scss" "portrait.scss" "landscape.scss" "admin.scss" )
# scss=( "style.scss" "print.scss" )
for style in "${scss[@]}"; do
    if [[ ! -f "$style" ]]; then
        echo "ERR: scss file not found: $style"
        exit 1
    fi
    target_file="${style%.scss}.css"
    echo "$target_file"
    if [[ -f "$target_file" ]]; then
        chmod u+w "$target_file"
    fi
    grass "$style" > "$target_file"
	chmod 444 "$target_file"
    if [[ "${style%.scss}" == "portrait" ]]; then
        echo "@import '$target_file' screen and (max-width: 1023px);" >> "index.css";
    elif [[ "${style%.scss}" == "landscape" ]]; then
        echo "@import '$target_file' screen and (min-width: 1024px);" >> "index.css";
    elif [[ "${style%.scss}" == "print" ]]; then
        echo "@import '$target_file' print;" >> "index.css";
    else
        echo "@import '$target_file';" >> "index.css"
    fi
done
chmod 444 "index.css"
popd
if [[ ! $1 ]]; then
    wasm-pack build --target web --no-typescript --dev
fi
