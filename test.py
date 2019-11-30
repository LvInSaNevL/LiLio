import os

arguments=['export GOOGLE_API_KEY="no"',
           'export GOOGLE_DEFAULT_CLIENT_ID="no"',
           'export GOOGLE_DEFAULT_CLIENT_SECRET="no"']
for arg in arguments:
    os.system(arg)