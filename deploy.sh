#! /bin/bash

cargo build --release

cd ..
scp usmaanwahab-co-uk/target/release/usmaanwahab-co-uk webserver:/root/
scp -r usmaanwahab-co-uk/templates/ webserver:/root/

ssh -t webserver './usmaanwahab-co-uk'
