      | |      (_)    | |      (_)     ___   
      | |__    | |    | |__    | |    / _ \  
      |____|  _|_|_   |____|  _|_|_   \___/  
    _|"""""|_|"""""|_|"""""|_|"""""|_|"""""| 
    "`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-' 

LiLio is a portible gaming device capible of a world class gaming experience powered by Stadia.


## Build your own
[The parts list to build your own LiLio is here](https://docs.google.com/spreadsheets/d/1o2KC0deXbZ5__kjTrOGxbklZBP4nP3jFMt7TPF-E2JY/edit?usp=sharing)

## Instalation
To install LiLio simply install Raspbian Lite on an SD card, and follow the following instructions

`git clone https://github.com/LvInSaNevL/LiLio`

`cd LiLio`

`python3 core/install.py` or `python3 core/install.py -v` if you would live verbose build logs

Lilio will handle the rest of the installation for you and restart your Raspberry Pi when its done!

## Creating a dev environment
If you are cloning LiLio to work on development, you may not need the entire setup, so this section gives you a stripped down version which is just enough to get you up and running. 

`git clone https://github.com/LvInSaNevL/LiLio`

`cd LiLio`

Run the code in `core/install.py` lines 63-71, which is prefixed by `# Building LiLio (finally)`. What this does is create the `market` direactory and the required JSON files. It also adds Stadia to the `device-list.json`, which is needed because without *something* in the file LiLio panics. 