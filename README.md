# MineCheck

## Problem

I host my own Minecraft server for my kids, and the server goes through periods of not being used for months sometimes. When the client is updated and the server is not the server refuses the connection because of the mismatch in version numbers.

## Bonus Hassle

Because the server is not used regularly I sometimes forget the details on how to connect to it:
- "I forgot my password"
- "I forgot my username!"
- "What IP address was the server again?"

Even when I do have my brain switched on, the process of performing a wget with the long janky url they now have was starting to get frustrating...

## Solution

Build a service that will periodically check the version of the server hosted on the Minecraft website. If it finds a new version:

1. Halt the Minecraft server locally
2. Fetch the latest copy from the website
3. Mark it as executable (`chmod +x`)
4. Modify the MOTD to say that the new version was downloaded and when
5. Restart the local server

Thus, keeping me free from having to update this every X months!
