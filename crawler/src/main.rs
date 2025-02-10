mod crawler;

use common::io::*;
use std::collections::HashSet;
use std::fs::File;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let google_search = String::from("https://www.google.com/search?q=example&sca_esv=c7459735fc04b658&sxsrf=AHTn8zqGT-0sGAk8FJaHdx0zmT3oIWAnlg%3A1738186356673&ei=dJ6aZ7HtKO-akdUPpbSf2Qc&ved=0ahUKEwix997i8JuLAxVvTaQEHSXaJ3sQ4dUDCBI&uact=5&oq=example&gs_lp=Egxnd3Mtd2l6LXNlcnAiB2V4YW1wbGUyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQFI5AVQiwJY0ARwAXgBkAEAmAE5oAHCAaoBATS4AQPIAQD4AQGYAgWgAucBwgIHECMYsQIYJ8ICBxAAGIAEGArCAgoQIxiABBgnGIoFwgIKEAAYgAQYQxiKBcICChAAGIAEGBQYhwLCAg0QABiABBixAxiDARgKwgIFEAAYgATCAgoQABiABBixAxgKmAMA4gMFEgExIECIBgGQBhG6BgYIARABGAiSBwE1oAfdKA&sclient=gws-wiz-serp");
    let file: File = create_f("../test.txt");

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let tx_clone = tx.clone();

    let mut visited: HashSet<String> = HashSet::new();
    catch_sig(file, &visited);

    let raw: Vec<String> = std::env::args().collect();

    let depth = raw[1].parse::<u8>().unwrap();
    let crawler_thread = thread::spawn(move || {
        crawler::crawl(google_search.as_str(), depth, &mut visited, tx_clone);
    });

    let dumper_thread = thread::spawn(move || {
        while let Ok(url) = rx.recv() {
            println!("Recieved URL: {}", url);
        }
    });

    crawler_thread.join();
    dumper_thread.join();
}
