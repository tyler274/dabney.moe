# Dabney Engineering Website

[![Deploy to GitHub Pages](https://github.com/tyler274/dabney.moe/actions/workflows/deploy.yml/badge.svg)](https://github.com/tyler274/dabney.moe/actions/workflows/deploy.yml)

This is the official website for Dabney Engineering Inc., built with Rust and WebAssembly using the Leptos framework.

Visit the site at: https://tyler274.github.io/dabney.moe/

## Features

- Fast, responsive web experience powered by Rust and WebAssembly
- Modern UI with Tailwind CSS
- Component-based architecture
- Type-safe development

## Development

To run the development server:

```bash
trunk serve --open
```

## Building

To build the project:

```bash
trunk build --release
```

## Deployment

The website is automatically deployed to GitHub Pages when changes are pushed to the main branch. The deployment process is handled by GitHub Actions.

To deploy manually:

1. Push changes to the main branch:
```bash
git push origin main
```

2. Monitor the deployment in the GitHub Actions tab.

3. The site will be available at https://tyler274.github.io/dabney.moe/

## Local Setup

1. Install Rust and the wasm32 target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Install Trunk:
```bash
cargo install trunk
```

3. Install dependencies:
```bash
cargo build
```

## Technology Stack

- Rust
- WebAssembly (WASM)
- Leptos Framework
- Tailwind CSS
- GitHub Pages

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