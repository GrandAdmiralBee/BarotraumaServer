FROM cm2network/steamcmd

# Make all directories for barotrauma files
RUN mkdir /home/steam/.local/share/Daedalic\ Entertainment\ GmbH/ -p
RUN mkdir /home/steam/ConfigFiles
WORKDIR /home/steam/.local/share/Daedalic\ Entertainment\ GmbH/
COPY Barotrauma Barotrauma

# Copy programm to integrate mods into config_player.xml file
WORKDIR /home/steam/Steam/steamapps/common/Barotrauma\ Dedicated\ Server/
COPY integrate_mods/target/release/integrate_mods integrate_mods
COPY entrypoint.sh entrypoint.sh

USER root
RUN chown steam:steam /home/steam/. -R
RUN chmod +x entrypoint.sh
USER steam

# ENTRYPOINT [ "./entrypoint.sh" ]
