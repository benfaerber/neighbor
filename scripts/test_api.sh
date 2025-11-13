#!/bin/bash

# TODO: Put in the prod endpoint when I deploy
ENDPOINT="http://127.0.0.1:8080/"

# Test 1: Simple case - single vehicle (from README)
echo "Test 1: Single 10ft vehicle"
echo "-------------------------------------------"
curl -X POST "$ENDPOINT" \
    -H "Content-Type: application/json" \
    -d '[
            {
                "length": 10,
                "quantity": 1
            }
        ]' 
echo ""

# Test 2: 3 vehicles
echo "Test 2: 3 vehicles"
echo "-------------------------------------------"
curl -X POST "$ENDPOINT" \
    -H "Content-Type: application/json" \
    -d '[
            {
                "length": 10,
                "quantity": 1
            },
            {
                "length": 20,
                "quantity": 2
            },
            {
                "length": 25,
                "quantity": 1
            }
        ]'
echo ""
