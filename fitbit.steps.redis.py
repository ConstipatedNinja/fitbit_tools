#!/usr/bin/env python

# check usage today. for use in bash shell.

# need to write to file/Redis and read only occasionally.

from oauth import oauth
import calendar
import datetime 
import fitbit
import os
import redis
import sys
import time

def main():
    # time-handling part
    now = datetime.datetime.now()       # what time is now?
    datestr = now.strftime("%Y-%m-%d")  # now in date format
    # fitbit part
    fb = fitbit.make_class()                    # connect to Fitbit
    fbr = fb.activities_date_json( datestr )    # ask for today's updates
    summary         = fbr['summary']            # get the summary
    steps  = summary['steps']                   # pull out steps to now
    # redis part
    rs = redis.Redis()                      # create Redis object
    rs.hset( 'fitbit' , 'steps' , steps )   # put step count into Redis

if __name__ == '__main__':
    main()

