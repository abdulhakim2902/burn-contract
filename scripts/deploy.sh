#!/bin/bash

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`

NEAR_ENV=$2 near state $1

NEAR_ENV=$2 near deploy $1 out/burn_token.wasm --initFunction new --initArgs '{}'

NEAR_ENV=$2 near state  $1
