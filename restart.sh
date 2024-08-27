#!/bin/bash

killall websocket_client
killall zkbinfo
killall zkbgui
sleep 5

tar --remove-files -czf logs-$(date +"%Y%m%d_%H%M%S").tar.gz logs
mkdir -p logs

IP=$(hostname -I | awk '{print $1}')

echo
export ZKBINFO_HOST=$IP
export ZKBINFO_PORT=8080
export ZKBGUI_HOST=$IP
export ZKBGUI_PORT=8088

nohup ~/zkbinfo/bin/zkbinfo > ~/zkbinfo/logs/zkbinfo.log 2>/dev/null &
nohup ~/zkbinfo/bin/zkbgui > ~/zkbinfo/logs/zkbgui.log  2>/dev/null&
nohup ~/zkbinfo/bin/websocket_client > ~/zkbinfo/logs/websocket_client.log 2>/dev/null &

sleep 1
echo
