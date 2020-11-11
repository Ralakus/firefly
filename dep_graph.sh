#!/usr/bin/env bash

cargo deps \
    --include-orphans \
    --subgraph firefly \
    --subgraph-name "Firefly" \
    | dot -Tpng > dep_graph.png 