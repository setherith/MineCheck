# MineCheck

## Problem

I host my own Minecraft server for my kids, and the server goes through long periods of not being used. While the client is automatically updated via the launcher, the server is not. This causes the server to refuse the connection because of the mismatch in version numbers.

## Solution

Build a service that will periodically check for the latest version of the server hosted on the Mojang servers. If it finds a new version:

1. Fetch the latest copy from the website
2. Mark it as executable (`chmod 755`)
2. Halt the Minecraft server locally
4. Modify the MOTD to say that the new version was downloaded and when
5. Restart the local server