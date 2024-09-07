#!/bin/bash

cd /home/steam/steamcmd/
# Run steamcmd to download game files
./steamcmd.sh +force_install_dir /home/gamefiles +login anonymous +app_update 1026340 validate +quit

mv /home/steam/CondigFiles/Barotrauma /home/steam/.local/share/Daedalic\ Entertainment\ GmbH/Barotrauma -r
# Copy server configurations
cp /home/steam/ConfigFiles/. . -r
# Integrate mods
./integrate_mods

cd /home/steam/Steam/steamapps/common/Barotrauma\ Dedicated\ Server/
./DedicatedServer
