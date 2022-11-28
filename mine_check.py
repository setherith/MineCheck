import requests
import os, stat
from server import Server
from bs4 import BeautifulSoup

def download(server: Server):
    print("downloading server jar...")
    server_stream = requests.get(server._location)
    with open('server.jar', 'wb') as server_file:
        server_file.write(server_stream.content)
        print("saved server jar to filesystem...")
        print("making executable...")
        os.chmod('server.jar', stat.S_IRUSR | stat.S_IWUSR | stat.S_IXUSR | stat.S_IROTH | stat.S_IRGRP)

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
                print (found_server)
                download(found_server)
                break
else:
    print("Server page unavailable, perhaps down?")
    print("Consider checking: " + url)
