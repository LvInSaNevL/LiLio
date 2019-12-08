import os

env = ["sudo dpkg --add-architecture i386",
       "sudo apt update && sudo apt upgrade"]

preqs = ["libssl-dev",
         "pkg-config",
         "libgtk-3-dev",
         "libssl-dev:i386"]

print('\033[92m' + "Configuring the system environment" + '\033[0m')
for arg in env:
    os.system(arg)

print('\033[92m' + "Installing required packages" + '\033[0m')
for package in preqs:
    os.system("sudo apt install -y %s" %(package))

print('\033[92m' + "Building LiLio" + '\033[0m')
os.system("cargo build")