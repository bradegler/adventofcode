#!/bin/sh

DAY=$1
YEAR=$2

# Default to current year
if [ -z "${YEAR}" ]; then
    YEAR=$(date +%Y)
fi

# Default to current day
if [ -z "${DAY}" ]; then
    DAY=$(date +%d)
fi

echo "${YEAR} ${DAY}"

mkdir -p "src/$YEAR/$DAY"
mkdir -p "testdata/${YEAR}"

touch "testdata/${YEAR}/${YEAR}_${DAY}.txt"

sed "s/%%YEAR%%/${YEAR}/g" < gen/main_template.rs | sed "s/%%DAY%%/${DAY}/g" > "src/${YEAR}/${DAY}/main.rs"

{
    echo 
    echo "[[bin]]" 
    echo "name = \"aoc${YEAR}_${DAY}\""
    echo "path = \"src/${YEAR}/${DAY}/main.rs\""
} >> Cargo.toml


agy "src/${YEAR}/${DAY}/main.rs" "testdata/${YEAR}/${YEAR}_${DAY}.txt"
