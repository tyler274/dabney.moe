# Dabney Engineering Website

A modern website for Dabney Engineering Inc., built with Rust and WebAssembly using the Leptos framework.

## Features

- Fast, responsive web experience powered by Rust and WebAssembly
- Modern UI with Tailwind CSS
- Component-based architecture
- Type-safe development

## Prerequisites

- Rust (latest stable)
- wasm32-unknown-unknown target
- Trunk

## Setup

1. Install Rust and the required tools:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

2. Install Tailwind CSS:

```bash
npm install -D tailwindcss
npx tailwindcss -i ./style/input.css -o ./style/output.css
```

## Development

To start the development server:

```bash
trunk serve
```

This will compile the Rust code to WebAssembly and serve it at `http://localhost:8080`.

## Building for Production

To build for production:

```bash
trunk build --release
```

The output will be in the `dist` directory.

## Project Structure

- `src/` - Rust source code
  - `app.rs` - Main application component
  - `components/` - Reusable UI components
  - `pages/` - Page components
- `public/` - Static assets
- `style/` - CSS files
- `index.html` - HTML entry point

## License

All rights reserved © 2024 Dabney Engineering Inc. 