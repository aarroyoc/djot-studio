use std::io::prelude::*;
use std::path::PathBuf;
use std::env;
use std::process::Command;
use rfd::FileDialog;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Djot Studio")
        .build(&event_loop)?;

    let port = portpicker::pick_unused_port().expect("No ports free");
    env::set_var("DJOT_STUDIO_PORT", &format!("{port}"));

    let _scryer = Command::new("./scryer-prolog")
        .arg("djot-studio.pl")
	.arg(&format!("{port}"))
        .spawn()
        .expect("Failed to launch Scryer Prolog");

    std::thread::sleep(std::time::Duration::from_millis(500));

    let _webview = WebViewBuilder::new(window)?
        .with_download_started_handler(|_url, path_buf| {
	    let file = FileDialog::new()
	    	.save_file();
	    
	    match file {
		Some(path) => {
		    dbg!(&path_buf);
		    dbg!(&path);
		    *path_buf = path;
		    true
		},
		None => false
	    }
	})
        .with_download_completed_handler(|url, path_buf, success| {
	    dbg!(url);
	    dbg!(path_buf);
	    dbg!(success);
	})
	.with_url(&format!("http://localhost:{port}/static/index.html"))?
	.build()?;

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;

      match event {
	Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
	Event::WindowEvent {
	  event: WindowEvent::CloseRequested,
	  ..
	} => *control_flow = ControlFlow::Exit,
	_ => (),
      }
    });
}
