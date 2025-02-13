#!/bin/sh
mkdir -p resources/gi/
curl "https://docs.google.com/spreadsheets/d/1EBSxkR_fjX2kckY0G-lcZGCp5ugnDp83lWg2bxTgsL8/gviz/tq?tqx=out:csv&gid=163318763" | sed -E 's/"//g' >./resources/gi/char_usefulroll_map.csv
UPDATE_DATAMINE=true cargo check --offline
