# Swiss Army Knife CLI (sak-cli)

A versatile command-line tool for API interactions and system operations.

## Features

- Mimecast API Integration
  - Message tracking
  - TTP URL Protection management
- Shell command execution
- Extensible architecture for adding more APIs and functionality

## Installation

### From Source

```bash
cargo install --path .
```

## Usage

### Configuration

#### Mimecast API

Configure Mimecast API credentials:
```bash
sak-cli config mimecast \
  --base-url "https://eu-api.mimecast.com" \
  --app-id "your-app-id" \
  --app-key "your-app-key"

# Note: Get your app_id and app_key from Mimecast Administration Console:
# Administration > Services > API & Platform Integrations > Available Integrations > Mimecast API 2.0

#### Microsoft Graph API

Configure Microsoft Graph API credentials:
```bash
# For interactive authentication (recommended)
sak-cli config graph --client-id "your-client-id"

# For daemon/service applications (optional)
sak-cli config graph --client-id "your-client-id" --client-secret "your-client-secret"
```

To register an application in Azure AD:
1. Go to Azure Portal > Azure Active Directory > App registrations
2. Click "New registration"
3. Enter a name for your application
4. Select "Single tenant" or "Multitenant" based on your needs
5. Set the redirect URI to "http://localhost:8888/oauth/callback" (type: "Web")
6. Note the "Application (client) ID" for configuration
7. Under "API permissions", add the required Microsoft Graph permissions:
   - User.Read
   - Mail.Read
   - Calendars.Read
8. Click "Grant admin consent" if you're an admin
```

### Mimecast Commands

Track messages:
```bash
# Search for messages with a specific subject
sak-cli mimecast track 'subject:"Important Email"'

# Search for messages from a specific sender
sak-cli mimecast track 'from:user@example.com'
```

Manage TTP URL Protection:
```bash
# Get information about a managed URL
sak-cli mimecast url get "http://example.com"

# Create a managed URL with block action
sak-cli mimecast url create "http://malicious.com" block --comment "Known malicious site"

# Create a managed URL with permit action
sak-cli mimecast url create "http://trusted.com" permit --comment "Trusted partner site"

# Manage Configuration Snapshots
sak-cli mimecast snapshot create "Backup before major changes"
sak-cli mimecast snapshot list
sak-cli mimecast snapshot list --start "2024-01-01" --limit 10
sak-cli mimecast snapshot restore "snapshot-id-123"
sak-cli mimecast snapshot export "snapshot-id-123"
```

### Shell Commands

Execute shell commands:
```bash
sak-cli exec -- ls -la
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.