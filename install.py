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