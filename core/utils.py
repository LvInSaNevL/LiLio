#!/usr/bin/python3

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