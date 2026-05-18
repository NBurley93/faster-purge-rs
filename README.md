# faster-purge-rs

## What is this?

faster-purge-rs, is a rewrite of an older script I made in python.

I use a tool called FASTER for Arma 3 operations hosting, and figuring out which profile has the mods I need, while also retiring older profiles and removing older mods can be a hassle.

Enter FASTER Purge. It's purpose is simple. It looks for the latest version of FASTER's config file,
scrapes all the profiles and downloaded mods from it, and lists out what profiles you have, what mods they load, and finally,
it lists any mods that have been orphaned, or not loaded in ANY profile.
