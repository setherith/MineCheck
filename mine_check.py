import requests
import os, stat
from server import Server
from peek import Peek
from bs4 import BeautifulSoup
from packaging import version

class MineChecker: 

    def __init__(self):
        remote = self.get_online_version_and_location()
        local = Peek('/home/setherith/backup/server.jar').get_version()
        print(f'Remote = {remote}\nLocal = {local}')
        remote_version = version.parse(remote.get_version())
        local_version = version.parse(local)
        print (remote_version, local_version, remote_version > local_version)
        if remote_version > local_version:
            print ("newer version online found...")
            self.download(remote)
        else:
            print ("version is current or newer?")

    def download(self, server: Server):
        print("downloading server jar...")
        server_stream = requests.get(server.location)
        with open('server.jar', 'wb') as server_file:
            server_file.write(server_stream.content)
            print("saved server jar to filesystem...")
            print("making executable...")
            os.chmod('server.jar', stat.S_IRUSR | stat.S_IWUSR | stat.S_IXUSR | stat.S_IROTH | stat.S_IRGRP)

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
            print("Server page unavailable, perhaps down?")
            print("Consider checking: " + url)
        return None

if __name__ == "__main__":
    MineChecker()
