#!/usr/bin/env sh

. "$(dirname $0)"/read_env_file.sh $1
"$(dirname $0)"/../target/release/tape_checker

