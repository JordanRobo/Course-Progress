# ![Icon](/src-tauri/icons/32x32.png) Course Progress Tracker

## Project Overview

Course Progress Tracker is a desktop application designed to help users monitor and manage their progress through online courses. This project was born out of a personal need to keep track of completed coursework across various online learning platforms.

## Key Features

- Submit and store course completion information
- View progress statistics
- Embedded database for offline use and data persistence

## Tech Stack

This project leverages a modern and efficient tech stack:

- **Frontend**: 
  - Svelte: A lightweight and reactive JavaScript framework
  - SvelteKit: The official Svelte application framework, providing routing and server-side rendering capabilities

- **Backend**:
  - Rust: A systems programming language known for its performance and safety
  - Diesel: A safe, extensible ORM and Query Builder for Rust
  - SQLite: A C-language library that implements a small, fast, self-contained SQL database engine

- **Desktop Application Framework**:
  - Tauri: A toolkit for building small, fast, and secure desktop applications with web technologies

- **Build Tool**:
  - Bun: A fast all-in-one JavaScript runtime and toolkit

## Project Philosophy

This application was conceived as a personal tool to enhance the online learning experience. By providing a centralized location to log and visualize course progress, it aims to motivate continued learning and offer insights into one's educational journey.

The use of Tauri with a Svelte frontend allows for a responsive and native-feeling desktop application, while the Rust backend with Diesel ORM ensures robust data handling and storage using SQLite. This combination results in a fast, efficient, and user-friendly tool for tracking educational progress.