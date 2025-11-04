mod structs;
mod web;
mod file;

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
    // find the latest copy
    let current_version_url = web::get_manifest_url();
    let meta = web::get_download_details(current_version_url);

    // download the file
    let path = String::from("./server.jar");
    file::download_file(meta.url, &path);

    // check integrity
    let download_is_safe = file::check_integrity(&path, 
        &meta.sha1, 
        meta.size);

    println!("Download integrity: {download_is_safe}");

    // check the local version - either contents of the zip or record in json file
    // if local is less than latest then replace
        // schedule replacement
            // find running instances
            // exit process
            // move old version to archive folder
            // check archive for limit and delete versions outside of retention policy
            // chmod new version
            // restart service
            // record newest version in json file 

}