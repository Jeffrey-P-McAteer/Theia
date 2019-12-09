
use web_view::*;
use vlc::{Instance, Media, MediaPlayer};
use url::{Url};

pub fn open(mrl: Option<Url>) {
	if let Some(mrl) = mrl {
		match mrl.scheme() {
			"youtube" | "yt" => {
				open_youtube(
					&youtube_url_to_embed_url(mrl)
				);
			}
			unk => {
				open_vlc(mrl.as_str());
			}
		}
	}
	else {
		std::unimplemented!();
	}
}

fn youtube_url_to_embed_url(yt_url: Url) -> String {
	"https://www.youtube.com/embed/L3ug_ZRNJiw?rel=0&autoplay=1".to_string()
}

fn open_youtube(url: &str) {
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
    instance.wait();

}