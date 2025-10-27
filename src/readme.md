# GitHub Activity Tracker

A minimal command-line Rust app that fetches and displays the latest public activity from any GitHub user using the **GitHub REST API**.
This project is a simple, async Rust application built with [`reqwest`](https://docs.rs/reqwest), [`tokio`](https://tokio.rs/), and [`serde`](https://serde.rs/).

---

## Project Page

(If applicable, add your project page or learning roadmap link here)

---

## Features

* Fetch and display the latest **GitHub events** for any username
* Supports multiple event types like:

  * Push events
  * Watch (Star) events
  * Issue creation or updates
  * Repository forks and creations
* Prints **up to 15** of the user’s most recent public activities
* Graceful error handling and friendly CLI prompts
* Uses **asynchronous I/O** for fast HTTP requests

---

## Requirements

* **Rust** 1.70+ (recommended)
* **Cargo** (included with Rustup)

Dependencies used:

```toml
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1.48.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio-macros = "2.5.0"
```

---

## Install & Build

Clone the repository and navigate into the folder:

```bash
git clone https://github.com/<your-username>/github-activity-tracker.git
cd github-activity-tracker
```

Build the project:

```bash
cargo build
```

Or build and run directly:

```bash
cargo run
```

---

## Run & Usage

Run the binary from your terminal:

```bash
cargo run
```

When prompted, enter a GitHub username:

```text
Enter your GitHub username
> octocat
```

Example output:

```text
Showing up to 15 latest events for octocat
Output:
 - Starred octocat/Hello-World
 - Pushed 2 commits to octocat/Spoon-Knife
 - Created a new branch in octocat/RepoX
 - Forked octocat/Hello-World
```

---

## Example: Dummy API Learning Script

Before connecting to GitHub, this project started as an experiment using a **dummy JSON API** (`https://dummyjson.com/products`) to understand async Rust and JSON deserialization:

```rust
let response: ProductsResponse = reqwest::get("https://dummyjson.com/products")
    .await?
    .json::<ProductsResponse>()
    .await?;

println!("Loaded {} products", response.products.len());
```

This served as a learning foundation before adapting the same concepts to the **GitHub Events API**.

---

## Notes & Tips

* The app fetches events from:
  `https://api.github.com/users/<username>/events`
* If you get a `403` or `rate limit` error, GitHub’s API might be temporarily restricting requests — try again later or use an authenticated token.
* Only **public events** are displayed.
* Modify the `take(15)` limit in the code to show more or fewer events.
* The app uses `serde_json::Value` for flexible event payload parsing — making it easy to add new event types later.

---

## Troubleshooting

* **Network errors:** Ensure your internet connection is active and GitHub API is reachable.
* **Request failed (403 or 404):** Check that the entered username exists and that you’re not rate-limited.
* **Cargo not found:** Install Rust via [rustup.rs](https://rustup.rs).

---

## License

MIT

---

Basic API request for the dummy JSON data to learn first how requesting works

```rust
use reqwest;
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductsResponse {
    products: Vec<Product>,
    total: Option<u64>,
    skip: Option<u64>,
    limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Dimensions {
    width: Option<f64>,
    height: Option<f64>,
    depth: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Review {
    rating: Option<u8>,
    comment: Option<String>,
    date: Option<String>,
    reviewer_name: Option<String>,
    reviewer_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Product {
    id: i32,
    title: String,
    description: String,
    category: Option<String>,
    price: Option<f64>,
    rating: Option<f64>,
    stock: Option<u64>,
    dimensions: Option<Dimensions>,
    #[serde(default)]
    reviews: Vec<Review>,
    // flexible fields
    meta: Option<HashMap<String, serde_json::Value>>,

    #[serde(default)]
    images: Vec<String>,

    // catch-all for any extra fields you don't want to model explicitly
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let response: ProductsResponse = reqwest::get("https://dummyjson.com/products")
        .await?
        .json::<ProductsResponse>()
        .await?;

    println!("loaded {} products", response.products.len());

    // print first product for inspection
    if let Some(first) = response.products.get(2) {
        println!("first product: {:#?}", first);
    }
    
    Ok(())
}
```

---