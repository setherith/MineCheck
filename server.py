from packaging import version

class Server:

    _version: version
    _location: str

    def __init__(self, title: str, location: str):
        if title.startswith('minecraft_server'):
            self._version = version.parse(title[17:-4])
            self._location = location

    def is_newer(self, current) -> bool:
        return self.ver > current.ver

    def __repr__(self) -> str:
        return f'Version: {self._version}\nLocation: {self._location}'
