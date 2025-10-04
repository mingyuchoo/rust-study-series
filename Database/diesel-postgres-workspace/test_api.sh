#!/bin/bash

# API Test Script for Todo Service

BASE_URL="http://localhost:8000/api/todos"

echo "🧪 Testing Todo API..."
echo ""

echo "1️⃣ Creating a new todo..."
CREATE_RESPONSE=$(curl -s -X POST $BASE_URL \
  -H "Content-Type: application/json" \
  -d '{"title":"Test from API"}')
echo "Response: $CREATE_RESPONSE"
echo ""

echo "2️⃣ Listing all todos..."
curl -s $BASE_URL | jq '.'
echo ""

echo "3️⃣ Getting todo #1..."
curl -s $BASE_URL/1 | jq '.'
echo ""

echo "4️⃣ Updating todo #1..."
UPDATE_RESPONSE=$(curl -s -X PUT $BASE_URL/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Updated Todo"}')
echo "Response: $UPDATE_RESPONSE"
echo ""

echo "5️⃣ Listing all todos again..."
curl -s $BASE_URL | jq '.'
echo ""

echo "✅ API tests completed!"
