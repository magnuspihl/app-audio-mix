use windows_volume_control::AudioController;
use clap::Parser;

#[derive(Default, Parser, Debug)]
#[command(about, long_about = None)]
struct Arguments {
    #[clap(short, long)]
    list: Option<bool>,
    #[clap(short, long)]
    include: Option<String>,
    #[clap(short = 'x', long)]
    exclude: Option<String>,
    #[clap(short, long)]
    volume: Option<f32>,
    #[clap(short, long)]
    other_volume: Option<f32>,
    #[clap(short, long)]
    adjust: Option<f32>,
}

fn main() {
    let args = Arguments::parse();

    unsafe {
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        let sessions = controller.get_all_session_names();

        if args.list.unwrap_or_default() == true {
            list_sessions(&sessions);
        }

        let filtered_sessions = &match_sessions(&sessions, args.include.unwrap_or_default(), args.exclude.unwrap_or_default());
        if args.volume.unwrap_or_default() != 0.0 {
            if args.volume.unwrap() > 1.0 || args.volume.unwrap() < 0.0 {
                println!("Volume must be between 0.0 and 1.0");
                return;
            }
            if args.other_volume.is_some() {
                set_volume(&controller, filtered_sessions, args.other_volume.unwrap());
            }
            set_volume(&controller, filtered_sessions, args.volume.unwrap());
        }
        else if args.adjust.unwrap_or_default() != 0.0 {
            if args.adjust.unwrap() > 1.0 || args.adjust.unwrap() < -1.0 {
                println!("Volume adjustment must be between -1.0 and 1.0");
                return;
            }
            adjust_volume(&controller, filtered_sessions, args.adjust.unwrap());
        }
    }
}

fn list_sessions(sessions: &Vec<String>) {
    for session in sessions.iter() {
        println!("{:?}", session);
    }
}

fn match_sessions(sessions: &Vec<String>, include: String, exclude: String) -> Vec<String> {
    let include_vec: Vec<String> = include.split(",").map(|x| x.to_string()).collect();
    let exclude_vec: Vec<String> = exclude.split(",").map(|x| x.to_string()).collect();

    let mut result: Vec<String>;
    if include == "" || include == "*" {
        result = sessions.clone();
    }
    else {
        result = Vec::new();
        for i in include_vec.iter() {
            if is_match(sessions, i.to_string()) {
                result.push(i.to_string());
            }
        }
    }

    if exclude != "" {
        if exclude == "*" {
            result.clear();
        }
        else if exclude_vec.len() > 0 {
            result.retain(|x| !is_match(&exclude_vec, x.to_string()));
        }
    }

    result.retain(|x| x.to_string() != "master");

    return result;
}

fn is_match(sessions: &Vec<String>, test: String) -> bool {
    for session in sessions.iter() {
        if session.to_lowercase().contains(&test.to_lowercase()) {
            return true;
        }
    }
    return false;
}

fn set_volume(controller: &AudioController, sessions: &Vec<String>, volume: f32) {
    unsafe  {
       for session in sessions.iter() {
            let current_session = controller.get_session_by_name(session.to_string()).unwrap();
            current_session.setVolume(volume);
        }
    }
}

fn adjust_volume(controller: &AudioController, sessions: &Vec<String>, adjust: f32) {
    let mut volume = -10.0;
    unsafe  {
       for session in sessions.iter() {
            let current_session = controller.get_session_by_name(session.to_string()).unwrap();
            if volume < 1.0 {
                volume = current_session.getVolume() + adjust;
                if volume > 1.0 {
                    volume = 1.0;
                }
                else if volume < 0.0 {
                    volume = 0.0;
                }
            }
            current_session.setVolume(volume);
        }
    }
}