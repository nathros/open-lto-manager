#!/bin/bash

for i in {1..30}
do
   echo "Message $i"
   sleep 0.3

   # >&2 echo "stderr Message $i"
   # sleep 0.33
done