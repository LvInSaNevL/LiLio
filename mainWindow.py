from Tkinter import *
import os

# Intial setup
window = Tk()
windowWidth = 800
windowHeight = 480
buttonWidth = windowWidth / 24
buttonHeight = windowHeight / 30
rootDir = "~/.LiLio"

# Program commands
def Stadia():
    os.system("%s/chromium/chrome-linux/chrome --start-fullscreen --new-window https://stadia.google.com/home" %rootDir)
def YouTube():
    os.system("%s/chromium/chrome-linux/chrome --start-fullscreen --new-window https://www.youtube.com/"  %rootDir)
def Plex():
    os.system("%s/chromium/chrome-linux/chrome --start-fullscreen --new-window https://app.plex.tv/desktop"  %rootDir)

# Program buttons
Stadia = Button(window, text="Stadia", command=Stadia, padx=0, pady=0, borderwidth=0, highlightthickness=0, height=buttonHeight, width=buttonWidth).grid(row=0, column=0, sticky=W+E+N+S)
YouTube = Button(window, text="YouTube", command=YouTube, padx=0, pady=0, borderwidth=0, highlightthickness=0, height=buttonHeight, width=buttonWidth).grid(row=0, column=1, sticky=W+E+N+S)
Plex = Button(window, text="Plex", command=Plex, padx=0, pady=0, borderwidth=0, highlightthickness=0, height=buttonHeight, width=buttonWidth).grid(row=0, column=2, sticky=W+E+N+S)

# Calls the main window creator
window.title("LiLio")
window.geometry("%sx%s" %(windowWidth, windowHeight))
window.resizable(0, 0)
window.mainloop()