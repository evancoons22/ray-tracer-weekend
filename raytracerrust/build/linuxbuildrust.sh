#! /bin/bash
# time the program
start=`date +%s`
rm test.ppm && cargo run >> test.ppm 
end=`date +%s`
runtime=$((end-start))
echo "Rendered in $runtime seconds."
feh test.ppm
