#!/bin/bash

set -e

DOC_DIR="/tmp/plotpy/doc_tests"

DOC_TESTS="
    contour
    curve
    histogram
    legend
    plot
    shapes
    surface
    text
"

for t in $DOC_TESTS; do
    cp -v $DOC_DIR/"doc_${t}.svg" ./figures/
done
