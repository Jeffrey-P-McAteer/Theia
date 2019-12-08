
use std::thread;

fn main() {
    //libvlc_experiment();
    gui_experiment();
}

fn gui_experiment() {
	use web_view::*;

	web_view::builder()
        .title("theia")
        .content(Content::Url( "https://www.youtube.com/embed/L3ug_ZRNJiw?rel=0&autoplay=1" ))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

}

fn libvlc_experiment() {
	use vlc::{Instance, Media, MediaPlayer};

	// Create an instance
    let instance = Instance::new().unwrap();
    
    // Create a media from a file
    
    //let md = Media::new_path(&instance, "/j/downloads/trailer_400p.ogg").unwrap();
    //let md = Media::new_location(&instance, "https://www.youtube.com/watch?v=L3ug_ZRNJiw").unwrap();
	let md = Media::new_location(&instance, "https://download.blender.org/peach/trailer/trailer_400p.ogg").unwrap();
    
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(30));
}

