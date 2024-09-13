use regex::Regex;
use std::env;
use std::process::Command;
use log::{info, warn, error};

static MODS_DEST_DIR: &str = r#"LocalMods/"#;

fn main() {
    //make_barotrauma_symlink();
    //make_player_config_file();
    run_barotrauma_docker_server();
    loop {
        
    }
}

fn run_barotrauma_docker_server(){
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .arg("/C")
            .arg("docker container remove Barotrauma")
            .spawn();

        Command::new("cmd")
            .arg("/C")
            .arg(r#"docker run --net=host --mount type=bind,src=ConfigFiles,dst=/home/steam/ConfigFiles --mount type=bind,src=Barotrauma,dst=/home/steam/.local/share/Daedalic\ Entertainment\ GmbH/Barotrauma --name=Barotrauma -it barotrauma_server bash"#)
            .spawn().unwrap();
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .spawn().unwrap();
    }
}

pub fn make_barotrauma_symlink() {

    // Get Barotrauma appdata folder
    let bara_data_dir = get_barotrauma_data_dir();

    // Symlink barotrauma Multiplayer amd Installed Mods dir
    let bara_data_symlink = std::path::Path::new("Barotrauma");
    if bara_data_symlink.exists() {
        println!("Barotrauma data dymlink already exists, skipping...");
    } else {
        let bara_data_path = std::path::Path::new(&bara_data_dir);

        if !bara_data_path.exists() {
            println!("Path does not exist");
        } else {
            #[cfg(target_os = "windows")]
            {
                std::os::windows::fs::symlink_dir(&bara_data_dir, "Barotrauma").unwrap()
            }
            #[cfg(target_os = "linux")]
            {
                std::os::unix::fs::symlink(&bara_data_dir, "Barotrauma").unwrap()
            }
        }
    }
}

fn get_barotrauma_data_dir() -> String{
    let vars = env::vars();
    let target_os = TargetOs::new();

    let mut data_dir = String::new();
    for var in vars.into_iter() {
        match target_os{
            TargetOs::Win => {
                if var.0 == "APPDATA" {
                    let appdata_roaming = std::path::Path::new(&var.1);
                    data_dir = appdata_roaming.parent().unwrap().to_str().unwrap().to_string();

                    data_dir.push_str(r#"\Local"#);
                    data_dir.push_str(r#"\Daedalic Entertainment GmbH\Barotrauma"#);
                    break;
                }
            },
            TargetOs::Unix => {
                if var.0 == "XDG_DATA_DIRS" {
                    let data_dirs = var.1;
                    let regex = Regex::new(r#"(?<data>\/home\/.+\/\.local\/share)"#).unwrap();
                    let caps = regex.captures(&data_dirs).unwrap();

                    data_dir = caps["data"].to_string();
                    data_dir.push_str(r#"/Daedalic Entertainment GmbH/Barotrauma"#);
                    break;
                }
            }
        }
    }

    data_dir
}

pub fn make_player_config_file() {
    let mods_dir = r#"Barotrauma/WorkshopMods/Installed/"#.to_string();

    let mods = get_mods_dirs(&mods_dir);

    let mut config_player = CONFIG_PLAYER_START.to_string();
    for m in mods {
        let insert_mod_line = format!(
            "\n<!-- {} -->\n<package path=\"{}\" enabled=\"true\"/>",
            m.name, m.path
        );
        config_player.push_str(&insert_mod_line);
    }

    config_player.push_str(CONFIG_PLAYER_END);

    std::fs::write("ConfigFiles/config_player.xml", config_player).unwrap();
}

#[derive(Debug, Clone)]
struct BarotraumaMod {
    pub path: String,
    pub name: String,
}

fn get_mods_dirs(mods_dir: &str) -> Vec<BarotraumaMod> {
    let mods_list = std::fs::read_dir(mods_dir).unwrap();

    let mods_list: String = mods_list
        .into_iter()
        .map(|read_dir| {
            let path = read_dir.unwrap().path();
            let path_file_name = path.file_name().unwrap();
            format!("{}\n", path_file_name.to_str().unwrap().to_string())
        })
        .collect();

    let mut mod_list = Vec::new();

    for mod_index in mods_list.lines() {
        let mut mod_origin_path = mods_dir.to_string();
        let mut mod_dest_path = MODS_DEST_DIR.to_string();

        mod_origin_path.push_str(mod_index);
        mod_dest_path.push_str(mod_index);
        mod_dest_path.push_str("/filelist.xml");

        let mut filelist = mod_origin_path.clone();
        filelist.push_str("/filelist.xml");

        let filelist_contents = std::fs::read_to_string(&filelist).unwrap();

        let regex = Regex::new("contentpackage name=\"(?<mod_name>.+)\" steamworkshopid").unwrap();
        let cap = regex.captures(&filelist_contents);

        if cap.is_none() {
            eprintln!(
                "No contentpackage name field in filelist.xml for mod path {}, skipping",
                &mod_origin_path
            );
            continue;
        }

        let cap = cap.unwrap();

        let mod_name: &str = &cap["mod_name"];

        mod_list.push(BarotraumaMod {
            path: mod_dest_path,
            name: mod_name.to_string(),
        });
    }

    mod_list
}

const CONFIG_PLAYER_START: &str = r##"<config
language="Russian"
  verboselogging="false"
  savedebugconsolelogs="false"
  savepath=""
  subeditorundobuffer="32"
  maxautosaves="8"
  autosaveintervalseconds="300"
  subeditorbackground="#0D2545"
  enablesplashscreen="true"
  pauseonfocuslost="true"
  aimassistamount="0.05"
  enablemouselook="true"
  showenemyhealthbars="ShowAll"
  chatspeechbubbles="true"
  interactionlabeldisplaymode="Everything"
  chatopen="true"
  crewmenuopen="true"
  showoffensiveserverprompt="false"
  tutorialskipwarning="false"
  corpsedespawndelay="600"
  corpsespersubdespawnthreshold="5"
  usedualmodesockets="true"
  disableingamehints="true"
  enablesubmarineautosave="true"
  quickstartsub=""
  remotemainmenucontenturl="https://www.barotraumagame.com/gamedata/"
  crossplaychoice="Disabled"
  savedcampaignsettings="&lt;campaignsettings PresetName=&quot;Normal&quot; TutorialEnabled=&quot;True&quot; RadiationEnabled=&quot;False&quot; MaxMissionCount=&quot;2&quot; WorldHostility=&quot;Medium&quot; StartItemSet=&quot;normal&quot; StartingBalanceAmount=&quot;Medium&quot; CrewVitalityMultiplier=&quot;1&quot; NonCrewVitalityMultiplier=&quot;1&quot; OxygenMultiplier=&quot;1&quot; FuelMultiplier=&quot;1&quot; MissionRewardMultiplier=&quot;1&quot; ShopPriceMultiplier=&quot;1&quot; ShipyardPriceMultiplier=&quot;1&quot; RepairFailMultiplier=&quot;1&quot; ShowHuskWarning=&quot;True&quot; PatdownProbability=&quot;Medium&quot; /&gt;"
  disableglobalspamlist="false">
  <contentpackages>
    <!--Vanilla-->
    <corepackage
      path="Content/ContentPackages/Vanilla.xml" />
    <regularpackages>"##;
const CONFIG_PLAYER_END: &str = r#"
</regularpackages>
  </contentpackages>
  <serverfilters
    FilterSameVersion="True"
    FilterPassword="False"
    FilterFullServers="False"
    FilterEmptyServers="False"
    FilterOffensiveServers="True"
    servertag.Serious="True"
    servertag.Casual="True"
    servertag.Roleplay="True"
    servertag.Rampage="True"
    servertag.SomethingDifferent="True"
    sandbox="True"
    mission="True"
    pvp="True"
    multiplayercampaign="True"
    karma="Any"
    traitors="Any"
    friendlyfire="Any"
    voip="Any"
    modded="Any" />
</config>"#;

enum TargetOs {
    Win,
    Unix,
}

impl TargetOs {
    pub fn new() -> Self {
        if cfg!(target_os = "windows") {
            Self::Win
        } else {
            Self::Unix
        }
    }
}
