#!/bin/sh
curl "https://gitlab.com/Dimbreath/AnimeGameData/-/raw/master/ExcelBinOutput/AvatarExcelConfigData.json" >./resources/AvatarExcelConfigData.json
curl "https://gitlab.com/Dimbreath/AnimeGameData/-/raw/master/TextMap/TextMapEN.json" >./resources/TextMapEN.json
curl "https://docs.google.com/spreadsheets/d/1EBSxkR_fjX2kckY0G-lcZGCp5ugnDp83lWg2bxTgsL8/gviz/tq?tqx=out:csv&gid=163318763" >./resources/CharacterUsefulRollMap.csv
