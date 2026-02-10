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
- `GET /api/v1/tasks` - List all tasks
- `POST /api/v1/tasks` - Create a new task
- `GET /api/v1/tasks/:id` - Get task by ID
- `PUT /api/v1/tasks/:id` - Update a task
- `DELETE /api/v1/tasks/:id` - Delete a task
- `GET /api/v1/board` - Get board configuration

## Development

Information on how to go about your development workflow.

## Contributing

Take a moment to review our [contribution guide](CONTRIBUTING.md) before submitting your first pull request.

Make sure that you check for open issues and pull requests to see if someone else is working on something similar.

## Contact

For feedback, requests or enquiries:

🌐 [http://www.mmuhlariholdings.co.za](http://www.mmuhlariholdings.co.za)