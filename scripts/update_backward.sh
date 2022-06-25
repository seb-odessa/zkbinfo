#!/bin/bash

CURRENT=${1:-$(date +'%Y-%m-%d' -d "1 day ago")}
FINISH=${2:-$(date +'%Y-%m-%d' -d "1 year ago")}
if [ "$CURRENT" != "$TODAY" ]; then
    while [ "$CURRENT" != "$FINISH" ]; do
      echo $CURRENT
      ZKBINFO_HOST=zkbinfo ~/zkbinfo/target/release/fetch_by_date $CURRENT
      CURRENT=$(date -I -d "$CURRENT - 1 day")
    done
fi