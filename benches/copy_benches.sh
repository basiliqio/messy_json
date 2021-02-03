#!/bin/bash -eux
cargo bench
cp target/criterion/Dummy\ object/report/violin.svg benches/dummy_violin.svg
cp target/criterion/Partial\ object/report/violin.svg benches/partial_violin.svg
cp target/criterion/Simple\ object/report/violin.svg benches/simple_violin.svg

