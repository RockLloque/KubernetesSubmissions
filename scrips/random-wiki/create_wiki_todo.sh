#!/bin/sh
set -e

# Get a random Wikipedia URL by following the redirect
WIKI_URL=$(curl -sL -o /dev/null -w '%{url_effective}' https://en.wikipedia.org/wiki/Special:Random)

echo "Random Wikipedia page: $WIKI_URL"

# Extract the page title from the URL (after /wiki/)
PAGE_TITLE=$(echo "$WIKI_URL" | sed 's|.*/wiki/||' | sed 's/_/ /g')

# Create the todo using the backend service within the cluster
TODO_BACKEND_RESPONSE=$(curl -X POST http://$TODO_BACKEND_SERVICE_HOST:$PORT_BACKEND/todos \
  -H "Content-Type: application/json" \
  -d "{
    \"title\": \"$PAGE_TITLE\",
    \"description\": \"$WIKI_URL\"
  }")

echo ""
echo "Todo created for: $PAGE_TITLE"
echo "POST response: $TODO_BACKEND_RESPONSE"
