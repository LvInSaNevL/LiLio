import os
import sys
import socket
import requests

#  # -- # -- #
#  Functions for the rest of the program
#  # -- # -- #

# A simple way to print distinctive text for important events
def yellowText(input):
    print("\033[33m" + input + "\033[m")

# A simple way to allow people to optionally print information
verbose = False
def verboseText(input):
    if verbose:
        print("[[  %s  ]]" %input)

# Checks if the devices is connected to the internet
def is_connected():
    try:
        # connect to the host -- tells us if the host is actually reachable
        socket.create_connection(("www.google.com", 80))
        return True
    except OSError:
        pass
    return False

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

# Verifies all the prerequisites are installed
prereqs=['python-tk']
yellowText("Installing needed prerequisites")
for req in prereqs:
    verboseText("Installing %s" %(req))
    os.system("sudo apt install %s -y" %(req))
verboseText("Updating the software and OS, and cleaning up the install")
os.system("sudo apt-get update && sudo apt-get upgrade -y && sudo apt-get upgrade -y && sudo apt-get dist-upgrade -y && sudo apt-get autoremove -y")

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
os.system("restart")