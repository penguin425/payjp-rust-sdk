# Pull Request: Add public key support and comprehensive token creation methods

## Summary

This PR adds comprehensive support for secure token creation using public keys, along with multiple methods for creating tokens to accommodate different security settings and use cases.

## Key Features

### 1. PayjpPublicClient - Public Key Authentication
- New client type for public key operations (pk_test_/pk_live_)
- Supports token creation only (security constraint)
- Requires public key + password authentication
- Same retry logic and error handling as PayjpClient

### 2. Multiple Token Creation Methods
Provides 4 different approaches to create tokens:

**Method 1: SDK with Public Key** (Recommended for SDK users)
```bash
export PAYJP_PUBLIC_KEY="pk_test_xxxxx"
export PAYJP_PUBLIC_PASSWORD="your_password"
cargo run --example create_token_public
```

**Method 2: Browser-based HTML Tool** (Most compatible)
- `create_token.html` - Uses PAY.JP.js for client-side token creation
- Works with strict security settings
- Japanese UI with pre-filled test card data

**Method 3: Shell Script**
- `create_test_token.sh` - Automated token creation via API
- Supports both jq and grep for JSON parsing

**Method 4: Direct curl** (For reference)
- Documented curl commands for manual testing

### 3. Security Improvements
- Comprehensive security warnings in all examples
- Clear documentation about PCI DSS compliance
- Proper separation of public/secret key usage
- Card name validation fixes (romanized names)

### 4. Form Encoding and Deserialization Fixes
- Fixed nested structure serialization with serde_urlencoded
- Added `card[field]` format for PAY.JP API compatibility
- Added `CardOrId` enum to handle expandable card fields
- Properly deserialize card fields that can be either ID strings or full objects
- Unit tests for form encoding

## Changes

### New Files
- `src/client.rs` - Added PayjpPublicClient (186 lines)
- `src/resources/token.rs` - Added PublicTokenService
- `examples/create_token_public.rs` - Public key token creation example
- `examples/charge_with_token.rs` - Token-based payment example
- `create_token.html` - Browser-based token creation tool
- `create_test_token.sh` - Automated token creation script

### Modified Files
- All example files - Added security warnings and romanized names
- `README.md` & `README.ja.md` - Comprehensive documentation updates
- `src/lib.rs` - Export PayjpPublicClient and PublicTokenService
- `Cargo.toml` - Added new examples and serde_urlencoded dependency

### Statistics
```
17 files changed, 1154 insertions(+), 91 deletions(-)
```

## Testing

- ✅ All examples compile successfully
- ✅ Public key authentication header verified (Base64 encoding matches curl)
- ✅ Form encoding unit tests pass
- ✅ Manual testing with actual PAY.JP credentials:
  - ✅ `create_token_public` - Token creation with public key + password works
  - ✅ `charge_with_token` - Charge creation with token works
  - ✅ `create_customer` - Customer creation with token works (CardOrId enum handles card IDs correctly)
  - ✅ `subscription` - Full subscription lifecycle works (create plan, customer, subscription, pause, resume, delete)
  - ✅ `three_d_secure` - Fixed API implementation to use correct parameters (resource_id only)

## Architecture

Demonstrates proper payment architecture:
1. **Client-side**: Public key creates tokens (card data never sent to server)
2. **Server-side**: Secret key processes payments with tokens
3. **Security**: Follows PAY.JP and PCI DSS best practices

## API Compatibility

Matches PAY.JP API exactly:
```bash
# curl equivalent
curl -u "pk_test_xxxxx:password" ...

# SDK equivalent
PayjpPublicClient::new("pk_test_xxxxx", "password")?
```

## Documentation

- Bilingual README (English + Japanese)
- Multiple token creation methods documented
- Security warnings and best practices
- Code examples for all scenarios

## Backwards Compatibility

All existing APIs unchanged. This PR only adds new features:
- New `PayjpPublicClient` type
- New `PublicTokenService`
- New examples and tools

## Resolves

This PR enables token creation for accounts with strict security settings that block raw card data submission (`unsafe_credit_card_param` error).

## Commit History

```
e01a302 Fix 3D Secure API implementation to match PAY.JP specification
29fe1fb Update PR description with final testing results and statistics
8af1f4a Fix metadata serialization issue in examples
e502412 Update all examples to use pre-created tokens instead of raw card data
c09a5b1 Fix metadata serialization issue in examples (earlier attempt)
1a020e6 Add PR description template for GitHub PR creation
45e06dd Add password parameter to PayjpPublicClient for authentication
1358fa3 Add PayjpPublicClient for secure token creation with public keys
8f8e23a Add browser-based token creation tool and update documentation
ba22020 Add script to create test tokens via API
8124647 Add token-based example for accounts without unsafe card param setting
4ea6068 Add security warnings about unsafe card parameters
e0d3d51 Fix card name validation errors in examples
9fd5615 Update src/client.rs
d81f839 Add unit test for form encoding with nested card structures
```

## Branch Information

- **Source Branch**: `claude/payjp-rust-sdk-01L9YAHXakzMazjAaZcCErC3`
- **Target Branch**: `main` (or default branch)

## How to Create the PR

Use the GitHub CLI or web interface:

```bash
# Using GitHub CLI (if available)
gh pr create --title "Add public key support and comprehensive token creation methods" --body-file PR_DESCRIPTION.md

# Or use the GitHub web interface:
# 1. Go to https://github.com/penguin425/payjp-rust-sdk/compare
# 2. Select your branch: claude/payjp-rust-sdk-01L9YAHXakzMazjAaZcCErC3
# 3. Copy the content from this file into the PR description
```
