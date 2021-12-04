#!/bin/sh

DAY=$1
YEAR=$2

# Default to current year
if [ -z "${YEAR}" ]; then
    YEAR=`date +%Y`
fi

# Default to current day
if [ -z "${DAY}" ]; then
    DAY=`date +%d`
fi

echo "${YEAR} ${DAY}"

mkdir -p "src/$YEAR/$DAY"

touch "testdata/${YEAR}_${DAY}.txt"

cat gen/main_template.rs | sed "s/%%YEAR%%/${YEAR}/g" | sed "s/%%DAY%%/${DAY}/g" > "src/${YEAR}/${DAY}/main.rs"

echo >> Cargo.toml
echo "[[bin]]" >> Cargo.toml
echo "name = \"aoc${YEAR}_${DAY}\"" >> Cargo.toml
echo "path = \"src/${YEAR}/${DAY}/main.rs\"" >> Cargo.toml