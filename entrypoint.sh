#!/bin/bash

cd /home/steam/steamcmd/
# Run steamcmd to download game files
./steamcmd.sh +force_install_dir /home/gamefiles +login anonymous +app_update 1026340 validate +quit

cd /home/steam/Steam/steamapps/common/Barotrauma\ Dedicated\ Server/
# Copy server configurations
cp /home/steam/ConfigFiles/. . -r
cp /home/steam/.local/share/Daedalic\ Entertainment\ GmbH/Barotrauma/WorkshopMods/Installed/. LocalMods/. -r
cp clientpermissions.xml Data/clientpermissions.xml
cp FactionEvents.xml Content/FactionEvents.xml
cp config_player.xml Data/config_player.xml

./DedicatedServer
bash
