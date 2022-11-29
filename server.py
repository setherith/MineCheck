class Server:

    version: str
    location: str

    def __init__(self, title: str, location: str):
        if title.startswith('minecraft_server'):
            self.version = title[17:-4]
            self.location = location

    def get_version(self) -> str:
        return self.version

    def __repr__(self) -> str:
        return f'Version: {self.version}\nLocation: {self.location}'
