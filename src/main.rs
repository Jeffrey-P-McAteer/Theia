use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    // Create an instance
    let instance = Instance::new().unwrap();
    
    // Create a media from a file
    
    //let md = Media::new_path(&instance, "/j/downloads/trailer_400p.ogg").unwrap();
    //let md = Media::new_location(&instance, "https://www.youtube.com/watch?v=Y226GeS8iAg").unwrap();
	let md = Media::new_location(&instance, "https://download.blender.org/peach/trailer/trailer_400p.ogg").unwrap();
    
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(15));
}

