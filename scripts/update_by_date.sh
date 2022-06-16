#/bin/bash

CURRENT=2022-06-01
FINISH=2022-06-17
while [ "$CURRENT" != "$FINISH" ]; do
  echo $CURRENT
  CURRENT=$(date -I -d "$CURRENT + 1 day")
  ZKBINFO_HOST=zkbinfo ./target/release/fetch_by_date $CURRENT

  # mac option for d decl (the +1d is equivalent to + 1 day)
  # d=$(date -j -v +1d -f "%Y-%m-%d" "2020-12-12" +%Y-%m-%d)
done
