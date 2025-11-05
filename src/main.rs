mod structs;
mod web;
mod file;
mod settings;
mod process;

//TODO: Wrap in Docker?
//TODO: Build a web app front end
    // With ability to check for updates now
    // replace current version with latest now
    // modify schedule
    // modify rentension policy
    // provide backup of world data
    // add mods to game or fetch latest
    // show log data from service

fn main() {
    // load settings from file
    let settings = settings::load_settings();

    // find the latest copy
    let current_version_url = web::get_manifest_url();
    let meta = web::get_download_details(current_version_url);

    // download the file
    file::download_file(meta.url, &settings.local_server_location);

    // check integrity
    let download_is_safe = file::check_integrity(&settings.local_server_location, 
        &meta.sha1, 
        meta.size);

    println!("Download integrity: {download_is_safe}");
    
    // check the local version - either contents of the zip or record in json file
    // if local is less than latest then replace
    // schedule replacement
    
    // find running instances
    // exit process
    let found = process::find_and_terminate_process(&settings.command_fragment);
    println!("Terminated existing instances: {}", found);

    // move old version to archive folder
    // check archive for limit and delete versions outside of retention policy
    // restart service
    // record newest version in json file 

}