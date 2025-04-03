# fsbot-rs

An AI-powered file system assistant that helps you manage and interact with your files through natural language commands.

## Features

- 🤖 Interactive AI assistant for file operations
- 📁 File system operations (create, delete, search, read)
- 💬 Natural language command processing
- 🎯 Context-aware responses
- 🔍 Smart file searching
- 📝 File content reading and analysis

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fsbot-rs = "0.1.0"
```

Or install it directly:

```bash
cargo install fsbot-rs
```

## Usage

Run the application:

```bash
fsbot-rs
```

### Available Commands

1. Create files:
```
create file notes.txt
```

2. Delete files:
```
delete notes.txt
```

3. Search for files:
```
find Python files
search documents
```

4. Read file contents:
```
read notes.txt
```

5. Get help:
```
help
```

## Examples

```bash
# Create a new file
create file todo.txt

# Search for Python files
find Python files

# Read a file's contents
read todo.txt

# Delete a file
delete todo.txt
```

## Configuration

The assistant can be configured through environment variables:

- `FSBOT_API_KEY`: Your API key for AI services (if using external AI)
- `FSBOT_MODEL`: The AI model to use (default: "gpt-3.5-turbo")

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 