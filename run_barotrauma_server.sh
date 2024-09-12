cp integrate_mods/target/release/integrate_mods int_mods
./int_mods

docker container prune
docker run --net=host --mount type=bind,src=/home/karim/BarotraumaServer/ConfigFiles,dst=/home/steam/ConfigFiles --mount type=bind,src=/home/karim/.local/share/Daedalic\ Entertainment\ GmbH/Barotrauma,dst=/home/steam/.local/share/Daedalic\ Entertainment\ GmbH/Barotrauma --name=Barotrauma -it barotrauma_server bash
