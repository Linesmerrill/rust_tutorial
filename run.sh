#!/bin/sh

for i in {1..100}
do
    mkdir temp
    echo "looping ... number $i"
    filename=$((RANDOM % 1234 + 30))
    echo "filename: $filename"
    for ((j=1; j <=$filename; j++))
    do
        echo "$(($RANDOM % 60 + $RANDOM % 25505))" >> temp/$(($filename + j)).txt
    done
    git add .
    git commit -m "feature-$filename/fix"
    git push
    echo "deleting directory temp/"
    rm -rf temp
    echo "completed, sleeping"
    
    sleep $(($RANDOM % 150 + 30))
    

done