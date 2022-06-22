#!/usr/bin/bash

START=$(date +'%Y-%m-%d' -d "$1")
. ~/zkbinfo/scripts/update_by_date.sh $START

