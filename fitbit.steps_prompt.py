#!/usr/bin/env python

# check usage today. for use in bash shell.

# need to write to file/Redis and read only occasionally.

from oauth import oauth
import datetime 
import httplib
import os
import pymongo
import simplejson as json
import time
import sys
import fitbit

def main():
    now = datetime.datetime.now()       # what time is now?
    datestr = now.strftime("%Y-%m-%d")  # now in date format
    fb = fitbit.make_class()                    # connect to Fitbit
    fbr = fb.activities_date_json( datestr )    # get most recent updates
    summary         = fbr['summary']            # get today's summary
    steps  = summary['steps']                   # pull out steps to now
    sys.stdout.write( str( steps ) + ' steps')

if __name__ == '__main__':
    main()

