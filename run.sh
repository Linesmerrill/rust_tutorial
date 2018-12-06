#!/bin/sh

for i in {1..100}
do
    echo "looping ... number $i"
    filename=$((RANDOM % 60 + 30))
    echo "filename: $filename"
    for ((j=1; j <=$filename; j++))
    do
        echo "internal loop $j"
        echo "$(($RANDOM % 60 + $RANDOM % 25505))" | tee -a $filename.txt
    done
    git add .
    git commit -m "feature-$filename/fix"
    git push
    echo "deleting file"
    rm $filename.txt
    echo "completed, sleeping"
    
    sleep $(($RANDOM % 60 + 30))
    

done