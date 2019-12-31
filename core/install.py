#!/usr/bin/python3

import os
import sys
import socket
import requests
from utils import *


#  # -- # -- #
#  The actual installation begins here
#  # -- # -- #

# Checks to see if the user passed the verbose flag
if len(sys.argv) > 1:
    if sys.argv[1].lower() == "--verbose" or sys.argv[1].lower() == "-v":
        verbose = True

# Verifies the user is connected to the internet
if (is_connected):
    yellowText("Devices is connected to the internet")
else:
    yellowText("No internet connection. Please connect to the internet and try again.")

# Creates the working directory
yellowText("Creating the main working directory at ~/.LiLio")
os.system("mkdir ~/.LiLio")

# Starts the actaul process by installing Chromium
yellowText("Installing Chromium")
verboseText("Downloading most recent version of Chromium")
os.system("wget https://download-chromium.appspot.com/dl/Linux_x64?type=snapshots -O ~/.LiLio/chromium.zip")
os.system("unzip ~/.LiLio/chromium.zip -d ~/.LiLio/chromium && rm -rf ~/.LiLio/chromium.zip")
os.system("chmod +x ~/.LiLio/chromium/chrome-linux/chrome")

# Modifies some Chromium defaults
arguments=['export GOOGLE_API_KEY="no"',
           'export GOOGLE_DEFAULT_CLIENT_ID="no"',
           'export GOOGLE_DEFAULT_CLIENT_SECRET="no"']
for arg in arguments:
    os.system("(cd ~/.LiLio/chromium/chrome-linux && %s)" %(arg))

# Configuring the system to the proper spec
env = ["sudo dpkg --add-architecture i386",
       "sudo apt update && sudo apt upgrade"]
yellowText("Configuring the system environment")
for package in env:
    os.system("sudo apt install -y %s" %(package))

# Verifies all the prerequisites are installed
prereqs=["pkg-config",
         "libgtk-3-dev",
         "libssl-dev:i386",
         "x11-xserver-utils"]
yellowText("Installing needed prerequisites")
for req in prereqs:
    verboseText("Installing %s" %(req))
    os.system("sudo apt install %s -y" %(req))
verboseText("Updating the software and OS, and cleaning up the install")
os.system("sudo apt-get update && sudo apt-get upgrade -y && sudo apt-get upgrade -y && sudo apt-get dist-upgrade -y && sudo apt-get autoremove -y")

# Building LiLio (finally)
yellowText("Building LiLio")
os.system("cargo build")

# Copies the program files to the working directory to the home directory
yellowText("Installing LiLio")
if verbose:
    os.system("cp -rv . ~/.LiLio")
else:
    os.system("cp -r . ~/.LiLio")

# Shows the user the LiLio splash and informs them that LiLio is installed and they need to restart their device
splash=[
'   _        _      _        _            ',
'  | |      (_)    | |      (_)     ___   ',
'  | |__    | |    | |__    | |    / _ \  ',
'  |____|  _|_|_   |____|  _|_|_   \___/  ',
'_|"""""|_|"""""|_|"""""|_|"""""|_|"""""| ',
'\"`-0-0-\'\"`-0-0-\'\"`-0-0-\'\"`-0-0-\'\"`-0-0-\' ',
' ',
'Welcome to LiLio, your own pocket sized gaming consle!',
'LiLio has successfully installed on your device. All thats standing between you and a world class gaming experience is a quick restart.',
' ',
'Press any button to reboot your LiLio console...'
]

for line in splash:
    yellowText(line)

input()
#os.system("restart")