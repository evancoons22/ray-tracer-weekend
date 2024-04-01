#! /bin/bash
rm test.ppm && cargo run >> test.ppm && open test.ppm
