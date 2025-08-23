#! /bin/bash

cargo build --release

cd ..
scp usmaanwahab-co-uk/target/release/usmaanwahab-co-uk webserver:/root/
scp -r usmaanwahab-co-uk/templates/ webserver:/root/

ssh -t webserver 'sleep 1 && export RUST_BACKTRACE=1 && ./usmaanwahab-co-uk'
