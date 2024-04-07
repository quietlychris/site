#!/bin/sh

orthophoto_path=$1
orthophoto_mbtiles_path=$2
gdal_translate -outsize 90% 0 -of MBTILES $orthophoto_path $orthophoto_mbtiles_path
echo "- Converted to MBTILES"
gdaladdo -r bilinear $orthophoto_mbtiles_path 2 4 8 16 32 64 128 256 512 1024 #2048 #4096 #8192 #16384
