#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

bash <(curl https://raw.githubusercontent.com/xd009642/tarpaulin/master/travis-install.sh)
cargo tarpaulin --all-features --out Xml
bash <(curl -s https://codecov.io/bash)
