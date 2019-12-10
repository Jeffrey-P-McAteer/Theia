
use web_view::*;
use vlc::{Instance, Media, MediaPlayer};
use url::{Url};
use regex::{Regex};
use base64;
use serde_json;
use glob;
use nfd;

use std::thread;
use std::time;
use std::fs;

use crate::config;

pub fn open(c: &config::Config, mrl: Option<Url>) {
	if let Some(mrl) = mrl {
		match mrl.scheme() {
			"youtube" | "yt" => {
				if youtube_url_is_video(&mrl) {
					open_youtube(
						&youtube_url_to_embed_url(&mrl)
					);
				}
				else {
					open_search_gui(c, &mrl); // we'd want to open the GUI to some channel overview listing videos
				}
			}
			"file" => {
				// If single file, play. If directory, open to search GUI
				if let Ok(meta) = fs::metadata( mrl.path() ) {
					if meta.is_file() {
						open_vlc(mrl.as_str());
					}
					else {
						open_search_gui(c, &mrl); // we'd want to open the GUI to search all videos in dir
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
		open_search_gui(c, &mrl);
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

fn open_search_gui(c: &config::Config, mrl: &Url) {
	// We already know mrl should be interpreted as a directory of some sort
	println!("open_search_gui({:?})", mrl);
	
	let webview = web_view::builder()
        .title("Theia")
        .content(Content::Html( include_str!("html/gui.html") ))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
        	do_from_js(c, mrl, webview, arg);
        	Ok(())
        })
        .build()
        .unwrap();

    let handle = webview.handle();

    thread::spawn(move || {
		loop {
			// In leiu of a reasonable way to get 3rd-party domains to phone
			// home we instead spawn a thread to inject every possible modification every 250ms
			thread::sleep(time::Duration::from_millis(250));
			handle.dispatch(|webview| {
			webview.eval(r#"
// Define once, and only once, any utility functions.
if (!window.addStyleString) {
	window.addStyleString = function (str) {
		var node = document.createElement('style');
		node.innerHTML = str;
		document.body.appendChild(node);
	};
}
// Check if this is the first time the loop has executed on this page...
if (!window.theia_loaded) {
	// Prevent future loops from running...
	window.theia_loaded = true;
	// Schedule a task IN the browser to poll for app JS to load..
	window.theia_int = setInterval(function() {
		
		// If this is a YT tab we will have one of these...
		var yt_vid_play_app = document.querySelector('video.video-stream');
		if (yt_vid_play_app) {
			
			// Stop polling for app JS load...
			clearInterval(window.theia_int);

			// Do the YT tab thing.
			addStyleString('.ytp-pause-overlay, .ytp-chrome-bottom, .ytp-chrome-top { display: none !important; }');
		}

		// If this is a YT results page we will have one of these...
		var yt_results_app = document.querySelector('ytd-search.style-scope');
		if (yt_results_app) {

			// Stop polling for app JS load...
			clearInterval(window.theia_int);

			// Do the YT search thing.
			document.body.innerHTML = "";
			document.body.appendChild(yt_results_app);
			// Modify yt_results_app to go fullscreen on click
			var app_fix_fn = function() {
				var links = document.getElementsByTagName("a");
				for(var i=0; i<links.length; i++) {
					try {
						console.log(links[i].href);
						if (links[i].href.includes("/watch")) {
							links[i].href = "https://www.youtube.com/embed/"+links[i].href.split("=")[1]+"?rel=0&autoplay=1";
							console.log("https://www.youtube.com/embed/"+links[i].href.split("=")[1]+"?rel=0&autoplay=1");
						}
					}
					catch (e) {
						console.log(e);
					}
				}

				addStyleString('body { all: initial; * { all: unset; } }');
				addStyleString('@media (prefers-color-scheme: dark) { p, a, div, span, yt-formatted-string { color: white !important; } }');
				addStyleString('@media (prefers-color-scheme: light) { p, a, div, span, yt-formatted-string { color: black !important; } }');
				addStyleString('.ytp-pause-overlay, .ytp-chrome-bottom, .ytp-chrome-top { display: none !important; }');

			};
			// Schedule quickly...
			setTimeout(app_fix_fn, 20);
			// And again in 1.2 seconds
			setTimeout(app_fix_fn, 1200);
		}


	}, 250);
}
"#);
				Ok(())
			});
		}
	});

    webview.run();

    loop {
    	// We may have more threads running... TODO ctrl+c is not viable thread management
    }

}

fn do_from_js(c: &config::Config, mrl: &Url, webview: &mut web_view::WebView<'_, ()>, arg: &str) {
	if arg == "__main__" {
		// Setup routine to push state into JS app
		match mrl.scheme() {
			"youtube" | "yt" => {
				let b64_query = base64::encode(&mrl.path()[1..]);
				webview.eval(&format!("window.location = \"https://youtube.com/results?search_query=\"+atob(\"{}\");", &b64_query));
			}
			"file" => {
				let options = glob::MatchOptions {
				    case_sensitive: false,
				    require_literal_separator: false,
				    require_literal_leading_dot: false,
				};

				let mut vid_files: Vec<String> = vec![];

				for ext in &["mp4", "ogg", "avi", "flv", "wmv", "mov"] {
					for entry in glob::glob_with( &format!("{}/**/*.{}", mrl.path(), ext), options ).expect("Failed to read glob pattern") {
						if let Ok(entry) = entry {
							vid_files.push(
								entry.into_os_string().into_string().unwrap()
							);
						}
					}
				}

				let mut html = String::new();
				html += &format!(r#"
<h2>{}</h2>
<hr>
"#, mrl.path() );
				for file in vid_files {
					html += &format!(r#"
						<div class="vid-entry">
							<p>{}</p>
							<button onclick="external.invoke('play_file;'+'{}');">Play</button>
						</div>
					"#, &file[mrl.path().len()+1..], &file);
				}

				_js_assign_body(
					webview,
					&html
				);
			}
			"default" => {
				let config_json = serde_json::to_string(c).unwrap();
				let app_html = include_str!("html/default.html").replace("__CONFIG__", &config_json);
				_js_assign_body(
					webview,
					&app_html
				);
			}
			unk => {
				_js_assign_body(
					webview,
					&format!("<em>Error: unknown scheme for URL: {}</em>", mrl.as_str() )
				);
			}
		}
		return;
	}

	if arg.starts_with("open_file;") {
		if let Ok(nfd::Response::Okay(result)) = nfd::open_file_dialog(None, None) {
			thread::spawn(move || {
				open_vlc(&format!("file://{}", result));
			});
		}
	}

	if arg.starts_with("open_dir;") {
		if let Ok(nfd::Response::Okay(result)) = nfd::open_file_dialog(None, None) {
			// First close _this_ web view
			webview.terminate();
			let c = c.clone();
			thread::spawn(move || {
				let dir_url: Url = format!("file://{}", result).parse().unwrap();
				open_search_gui(&c, &dir_url);
			});
		}
	}

	if arg.starts_with("search_yt;") {
		let query = format!("{}", &arg[10..]);
		let b64_query = base64::encode(&query);
		webview.eval(&format!("window.location = \"https://youtube.com/results?search_query=\"+atob(\"{}\");", &b64_query));
	}

	println!("do_from_js.arg={}", arg);
	

}

fn _js_assign_body(webview: &mut web_view::WebView<'_, ()>, html: &str) {
	// Encode to base64 to ensure string-iness
	let b64_html = base64::encode(html);
	webview.eval(&format!(r#"
document.body.innerHTML = "";
var range = document.createRange();
document.body.appendChild(
	range.createContextualFragment(atob("{}"))
);
"#, b64_html));
	// document.body.innerHTML = atob(\"{}\"); does not run script tags
}
