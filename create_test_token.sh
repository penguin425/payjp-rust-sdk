#!/bin/bash
#
# Create a test token using PAY.JP API
#
# Usage:
#   export PAYJP_SECRET_KEY="sk_test_xxxxx"
#   ./create_test_token.sh

if [ -z "$PAYJP_SECRET_KEY" ]; then
    echo "Error: PAYJP_SECRET_KEY environment variable not set"
    echo "Usage: export PAYJP_SECRET_KEY=\"sk_test_xxxxx\""
    exit 1
fi

echo "Creating test token with PAY.JP API..."
echo ""

# Create token with test card data
response=$(curl -s -X POST https://api.pay.jp/v1/tokens \
  -u "${PAYJP_SECRET_KEY}:" \
  -d "card[number]=4242424242424242" \
  -d "card[exp_month]=12" \
  -d "card[exp_year]=2030" \
  -d "card[cvc]=123" \
  -d "card[name]=Test User")

# Check if request was successful
if echo "$response" | grep -q '"error"'; then
    echo "Error creating token:"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
    exit 1
fi

# Extract token ID - use jq if available, otherwise grep
if command -v jq &> /dev/null; then
    token_id=$(echo "$response" | jq -r '.id')
else
    token_id=$(echo "$response" | grep -o '"id":"tok_[^"]*"' | cut -d'"' -f4)
fi

if [ -z "$token_id" ] || [ "$token_id" = "null" ]; then
    echo "Error: Could not extract token ID from response"
    echo "Response:"
    echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
    exit 1
fi

echo "✓ Token created successfully!"
echo ""
echo "Token ID: $token_id"
echo ""
echo "Now you can run the example:"
echo "  export PAYJP_TOKEN_ID=\"$token_id\""
echo "  cargo run --example charge_with_token"
