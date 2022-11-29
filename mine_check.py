import requests
import os, stat
import logging
import json
from server import Server
from peek import Peek
from bs4 import BeautifulSoup
from packaging import version

class MineChecker: 

    logger: logging.Logger = None
    config = None

    def __init__(self):
        # read config file...
        self.read_config()

        # setting up logging...
        if self.config['logging_level'] == "INFO":
            logging.basicConfig(level=logging.INFO, \
                    format='%(asctime)s %(message)s', \
                    datefmt='%Y-%m-%d %H:%M:%S')
        self.logger = logging.getLogger('MineChecker')

        remote = self.get_online_version_and_location()
        local = Peek(self.config['local_server_location']).get_version()
        
        remote_version = version.parse(remote.get_version())
        local_version = version.parse(local)
        
        if remote_version > local_version:
            self.logger.info(f"Local version is at {local_version}")
            self.logger.info(f"Online version is at {remote_version}")
            self.download(remote)
        else:
            self.logger.info("Local version is up to date!")

    def read_config(self):
        with open('config.txt', 'r') as config_file:
            self.config = json.loads(config_file.read())


    def download(self, server: Server):
        self.logger.info("Downloading latest version...")
        self.logger.debug(f"URL: {server.location}")
        server_stream = requests.get(server.location)
        with open(self.config['local_server_location'], 'wb') as server_file:
            server_file.write(server_stream.content)
            self.logger.info("Download complete!")
            self.logger.info("Making file executable...")
            os.chmod(self.config['local_server_location'], stat.S_IRUSR | stat.S_IWUSR | stat.S_IXUSR | stat.S_IROTH | stat.S_IXOTH | stat.S_IRGRP | stat.S_IWGRP | stat.S_IXGRP)

    def get_online_version_and_location(self):
        headers = {"User-Agent": "Chrome/107.0.0.0"}
        url = 'https://www.minecraft.net/en-us/download/server'
        server_page = requests.get(url, headers=headers)

        if server_page.status_code == 200:
            soup = BeautifulSoup(server_page.text, 'html.parser')
 
            all_links = soup.select('a')
            for link in all_links:
                if link.has_attr('href'):
                    if link['href'].endswith('server.jar'):
                        title, location = str(link.contents[0]), link['href']
                        found_server = Server(title, location)
                        return found_server
        else:
            self.logger.warn("Server page unavailable")
            self.logger.debug(f"URL: {url}")
        return None

if __name__ == "__main__":
    MineChecker()
