#!/bin/bash

ZKBINFO=vps
BINARIES=$(find target/release -maxdepth 1 -type f -executable)
SCRIPTS="public scripts restart.sh stop.sh"

rsync -uP $BINARIES vps:~/zkbinfo/bin/
rsync -urP $SCRIPTS vps:~/zkbinfo/
