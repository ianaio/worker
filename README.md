# worker
Convenience crate for working with Web Workers

<div align="center">

  <h1><code>ianaio-worker</code></h1>

  <p>
    <a href="https://crates.io/crates/ianaio-worker"><img src="https://img.shields.io/crates/v/ianaio-worker.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/ianaio-worker"><img src="https://img.shields.io/crates/d/ianaio-worker.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/ianaio-worker"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://docs.rs/ianaio-worker">API Docs</a>
    <span> | </span>
    <a href="https://github.com/ianaio/worker/blob/main/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://discord.com/channels/1247475712001314857/1247475712001314860">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.iana.io/">The IanaIO Rust and WebAssembly Working Group</a></sub>
</div>

IanaIO workers are a way to offload tasks to web workers. These are run concurrently using
[web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers).
It provides a neat abstraction over the browser's Web Workers API which can be consumed from anywhere.

