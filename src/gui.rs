
use web_view::*;
use vlc::{Instance, Media, MediaPlayer};
use url::{Url};
use regex::{Regex};

use std::thread;
use std::time;
use std::fs;

pub fn open(mrl: Option<Url>) {
	if let Some(mrl) = mrl {
		match mrl.scheme() {
			"youtube" | "yt" => {
				if youtube_url_is_video(&mrl) {
					open_youtube(
						&youtube_url_to_embed_url(&mrl)
					);
				}
				else {
					open_search_gui(&mrl); // we'd want to open the GUI to some channel overview listing videos
				}
			}
			"file" => {
				// If single file, play. If directory, open to search GUI
				if let Ok(meta) = fs::metadata( mrl.path() ) {
					if meta.is_file() {
						open_vlc(mrl.as_str());
					}
					else {
						open_search_gui(&mrl); // we'd want to open the GUI to search all videos in dir
					}
				}
				else {
					panic!("Invalid file given; cannot get metadata");
				}
			}
			unk => {
				open_vlc(mrl.as_str());
			}
		}
	}
	else {
		// we'd want to open the GUI to a list of all backends w/ config options to
		// add new backends and search for videos
		let mrl = Url::parse("default://").unwrap();
		open_search_gui(&mrl);
	}
}

static YT_VID_REGEX: &'static str = "^.*((youtu.be/)|(v/)|(/u/w/)|(embed/)|(watch?))??v?=([^#&?]*).*";

fn youtube_url_is_video(yt_url: &Url) -> bool {
	let yt_url = yt_url.as_str();
	let video_id_regex = Regex::new(YT_VID_REGEX).unwrap();
	return video_id_regex.is_match(yt_url);
}

fn youtube_url_to_embed_url(yt_url: &Url) -> String {
	let yt_url = yt_url.as_str();
	let video_id_regex = Regex::new(YT_VID_REGEX).unwrap();
	for cap in video_id_regex.captures(yt_url) {
		println!("cap = {:?}", cap );
	}
	let video_id = video_id_regex.captures(yt_url).unwrap().get(7).map_or("ERROR", |m| m.as_str());
	if video_id == "ERROR" {
		panic!("Invalid youtube URL: cannot parse video ID");
	}
	return format!("https://www.youtube.com/embed/{}?rel=0&autoplay=1", video_id); // constructs url like "https://www.youtube.com/embed/L3ug_ZRNJiw?rel=0&autoplay=1".to_string()
}

fn open_youtube(url: &str) {
	println!("url={}", url);
	web_view::builder()
        .title("theia")
        .content(Content::Url( url ))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

fn open_vlc(mrl: &str) {
	// Create an instance
    let instance = Instance::new().unwrap();
    
    // Create a media from a mrl
	let md = Media::new_location(&instance, mrl).unwrap();
    
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait until user closes
    // instance.wait(); // Does not work, hangs after video ends
    // Poll and wait until video stops playback
    loop {
    	thread::sleep(time::Duration::from_millis(250));
    	if ! mdp.is_playing() {
    		break;
    	}
    }

}

fn open_search_gui(mrl: &Url) {
	// We already know mrl should be interpreted as a directory of some sort
	println!("open_search_gui({:?})", mrl);
	web_view::builder()
        .title("Minimal webview example")
        .content(Content::Html( include_str!("html/gui.html") ))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
        	println!("arg = {:?}", arg);

        	if arg == "call" {
        		webview.eval("document.body.innerHTML += \"<p>stuff</p>\";");
        	}

        	Ok(())
        })
        .run()
        .unwrap();
}
