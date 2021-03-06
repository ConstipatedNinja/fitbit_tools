#!/usr/bin/Rscript

# R script to graph the average daily steps over the last eight months

library( RMySQL )
library( Cairo )
library( yaml )

my.cnf = yaml.load_file( '~/.my.yaml' )

default = my.cnf$clients$itap

p_cols     <- c( "blue" , "red" , "blue" , "red" , "blue" , "red" , "blue" ) 
con <- dbConnect( MySQL()
    , user=default$user 
    , password=default$password
    , dbname=default$database
    , host=default$host 
    )

sql <- '
SELECT
    CAST( AVG(steps) as DECIMAL ) steps ,
    CAST( AVG(floors) as DECIMAL ) floors ,
    DATE_FORMAT( datestamp , "%Y" ) year ,
    YEARWEEK(datestamp) month 
FROM fitbit_daily
WHERE steps > 100
GROUP BY YEARWEEK(datestamp)
ORDER BY datestamp
DESC LIMIT 40
    '

rs <- dbSendQuery( con , sql )
fields = dbColumnInfo(rs)
data_frame = fetch(rs, n = 40 )

floors = rev(data_frame[[ "floors" ]])
steps  = rev(data_frame[[ "steps"  ]])
month  = rev(data_frame[[ "month"  ]])
year   = rev(data_frame[[ "year"   ]])

table <- matrix( steps
    ,   ncol=40
    ,   byrow=TRUE
    ) 

CairoPNG( filename="/home/jacoby/www/QuantifiedSelf/fitbit_steps_by_week.png" 
    ,   width     = 400 
    ,   height    = 300 
    ,   pointsize = 12 
    )
rstep = range( 0 , steps ) 
plot ( steps
    ,   col         = p_cols[1] 
    ,   type        = "l" ,
    ,   xlab        = "Week"
    ,   ylab        = "Average steps counted by FitBit"
    ,   ylim        = rstep
    ,   xaxt        = "n"
    )
axis( 1 , at=1:40 , labels=month )
title( "Average Daily Steps per Week" )
box()

