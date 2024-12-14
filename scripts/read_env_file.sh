#!/usr/bin/env sh

ENV_FILE="${1:-.env}"

if [ -z $1 ] ; then
    if [ "$0" = "-bash" ] ; then
        path_file="$(dirname "${BASH_SOURCE[0]}")"/".."/"$ENV_FILE"
    else
        path_file="$(dirname $0)"/".."/"$ENV_FILE"
    fi
else
    path_file="$PWD"/"$1"
fi

if [ -f "$path_file" ]; then
    echo "Use env file: $path_file"

    # Load environment variables from the file
    set -a
    . "$path_file" || {
        echo "Error sourcing environment file: $path_file"
        exit 1
    }
    set +a
else
    echo "File: $path_file - is not found."
fi

