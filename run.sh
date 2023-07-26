#!/bin/sh
set -a
source .env
set +a

if [ "$1" = "dev" ];then
  cargo run --package rocket-boilerplate --bin rocket-boilerplate
else
  cargo run -r --package rocket-boilerplate --bin rocket-boilerplate
fi