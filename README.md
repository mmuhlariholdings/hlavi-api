# hlavi-api

HTTP API server for Hlavi kanban task management.

## Table of Contents

- [Getting Started](#getting-started)
- [Documentation](#documentation)
- [Development](#development)
- [Contributing](#contributing)
- [Contact](#contact)

## Getting Started

A quick guide on how you can get started running and working on the applicatoin on your local machine.

### Requirements

- Rust 1.75 or higher
- Cargo

### Clone

```bash
git clone https://github.com/mmuhlariholdings/hlavi-api.git
cd hlavi-api
```

### Build and Run

```bash
cargo run
```

The API server will start on `http://localhost:3000`

### API Endpoints

- `GET /health` - Health check
- `GET /api/v1/tickets` - List all tickets
- `POST /api/v1/tickets` - Create a new ticket
- `GET /api/v1/tickets/:id` - Get ticket by ID
- `PUT /api/v1/tickets/:id` - Update a ticket
- `DELETE /api/v1/tickets/:id` - Delete a ticket
- `GET /api/v1/board` - Get board configuration

## Development

Information on how to go about your development workflow.

## Contributing

Take a moment to review our [contribution guide](CONTRIBUTING.md) before submitting your first pull request.

Make sure that you check for open issues and pull requests to see if someone else is working on something similar.

## Contact

For feedback, requests or enquiries:

🌐 [http://www.mmuhlariholdings.co.za](http://www.mmuhlariholdings.co.za)