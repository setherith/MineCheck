import json
import zipfile

class Peek:
    """Will find the version number of server.jar"""

    _version: str

    def __init__(self, location: str):
        with zipfile.ZipFile(location) as server_zip:
            with server_zip.open('version.json') as server_json:
                version_content = json.loads(server_json.read())
                self._version = version_content['name']

    def get_version(self) -> str:
        return self._version

if __name__ == "__main__":
    p = Peek('/home/setherith/server/server.jar')
    print (p.get_version())
