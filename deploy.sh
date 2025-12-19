#! /bin/bash

build=true

while getopts "s" flag; do
  case "$flag" in
  s) build=false ;; # static only
  esac
done

if [[ $build == true ]]; then
  cargo build --release
fi

cd ..
if [[ $build == true ]]; then
  scp usmaanwahab-co-uk/target/release/usmaanwahab-co-uk webserver:/root/
fi
ssh -t webserver 'rm -r /root/templates /root/static'
scp usmaanwahab-co-uk/.env webserver:/root/
scp usmaanwahab-co-uk/courses.json webserver:/root/
scp -r usmaanwahab-co-uk/templates/ webserver:/root/
scp -r usmaanwahab-co-uk/static/ webserver:/root/
ssh -t webserver 'sleep 1 && ./usmaanwahab-co-uk'
