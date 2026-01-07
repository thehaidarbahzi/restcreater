# Restcreater

A command-line tool for easily scaffolding REST API projects in multiple programming languages and frameworks.

## Overview

**restcreater** is designed to eliminate the repetitive setup process when starting new REST API projects. Whether you're prototyping, learning a new framework, or starting a production project, restcreater provides you with clean and minimal boilerplate code to get started immediately.

The tool includes carefully crafted templates for popular web frameworks across Rust, Go, and JavaScript ecosystems, each featuring a basic "Hello World" endpoint and a clear structure for building upon.

## Features

- **Multiple Language Support**: Create REST APIs in Rust, Go, or JavaScript
- **Framework Templates**: Pre-configured templates for popular web frameworks
- **Fast Setup**: Generate a complete project structure in seconds
- **Minimal Boilerplate**: Clean code without complicated mess

## Supported Languages & Frameworks

### Rust
- **Actix-web**
- **Axum**
- **Rocket**
- **Warp**

### Go
- **Beego**
- **Buffalo**
- **Echo**
- **Gin**
- **Gorm**

### JavaScript
- **Expressjs**
- **Fastify**
- **Hapijs**
- **Koajs**

## Installation

### From Source

Ensure you have Rust and Cargo installed on your system.  If not, install them from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/thehaidarbahzi/restcreater.git

# Navigate to the project directory
cd restcreater

# Build the project
cargo build --release

# Install the binary
cargo install --path . 
```

## Usage

### Creating a New Project

#### Interactive Mode (Recommended)

Simply run the command without arguments to use the interactive prompts:

```bash
restcreater new
```

You'll be guided through selecting:
1. Project name
2. Programming language
3. Framework
4. Project template

#### Command-Line Arguments

Alternatively, specify all parameters directly:

```bash
restcreater new --name <PROJECT_NAME> --lang <LANGUAGE> --framework <FRAMEWORK> --template <TEMPLATE>
```

**Arguments:**
- `-n, --name <NAME>` - Project name
- `-l, --lang <LANG>` - Programming language (rust, go, javascript)
- `-f --framework <FRAMEWORK>` - Framework for the selected language
- `-t, --template <TEMPLATE>` - Project template

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**! 

### Adding New Templates

To add support for a new framework:

1. Create a new directory under `src/templates/<language>/<framework-name>/`
2. Add the necessary template files with proper variable substitution (e.g., `{{name}}`)
3. Test the template generation thoroughly
4. Update this README with the new framework

## License

This project is currently unlicensed.

## Roadmap

- [ ] Add more languages
- [ ] Add more framework templates
- [ ] Publish to github releases
- [ ] Implement update feature
