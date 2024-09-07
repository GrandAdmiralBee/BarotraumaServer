use regex::Regex;
use std::process::Command;

fn main() {
    let mods_dir =
        r#"/home/steam/.local/share/Daedalic\ Entertainment\ GmbH/Barotrauma/WorkshopMods/Installed/"#.to_string();

    let mods = get_mods_dirs(&mods_dir);

    let mut config_player = CONFIG_PLAYER_START.to_string();
    for m in mods {
        let insert_mod_line = format!("\n<!-- {} -->\n<package path=\"{}\"/>", m.name, m.path);
        config_player.push_str(&insert_mod_line);
    }

    config_player.push_str(CONFIG_PLAYER_END);

    std::fs::write("config_player.xml", config_player).unwrap();
}

#[derive(Debug, Clone)]
struct BarotraumaMod {
    pub path: String,
    pub name: String,
}

fn get_mods_dirs(mods_dir: &str) -> Vec<BarotraumaMod> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!("ls {mods_dir}"))
        .output()
        .unwrap();

    let mods_list = String::from_utf8(output.stdout).unwrap();

    dbg!(&mods_list);

    let mut mod_list = Vec::new();

    for mod_index in mods_list.lines() {
        let mut mod_path = mods_dir.to_string();
        mod_path.push_str(mod_index);

        let mut filelist = mod_path.clone();
        filelist.push_str("/filelist.xml");

        let output = Command::new("sh")
            .arg("-c")
            .arg(&format!("cat {}", filelist))
            .output()
            .unwrap();

        let filelist_contents = String::from_utf8(output.stdout).unwrap();
        let regex = Regex::new("contentpackage name=\"(?<mod_name>.+)\" steamworkshopid").unwrap();
        let cap = regex.captures(&filelist_contents);

        if cap.is_none() {
            eprintln!(
                "No contentpackage name field in filelist.xml for mod path {}, skipping",
                &mod_path
            );
            continue;
        }

        let cap = cap.unwrap();

        let mod_name: &str = &cap["mod_name"];

        mod_list.push(BarotraumaMod {
            path: mod_path,
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
