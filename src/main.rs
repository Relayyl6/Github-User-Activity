use std::io;
use reqwest;
// use serde;
use serde::Deserialize;
// use serde_json::Result;
use std::error::Error;

use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Actor {
    id: u64,
    login: String,
    display_login: String,
    gravatar_id: String,
    url: String,
    avatar_url: String,
}

#[derive(Debug, Deserialize)]
struct Repo {
    #[serde(rename = "id")]
    id: u64,
    name: String,
    #[serde(rename = "url")]
    url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Event {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    actor: Actor,
    repo: Repo,
    payload: Value, // Use Value for flexible payload structure
    public: bool,
    created_at: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    println!("Enter your GitHub username");

    let mut user = String::new();

    io::stdin()
        .read_line(&mut user)
        .expect("Failed to Get any logs from the entered list of users");

    let entered_username = user.trim();

    // let username = String::from("relayyl6"); // Only thing to change in code per user
    let url = format!("https://api.github.com/users/{}/events", entered_username);

    // Create a client
    let client = reqwest::Client::new();

    let res: reqwest::Response = client
        .get(&url)
        .header("User-Agent", "rust-client")
        .send()
        .await?;

    if !res.status().is_success() {
        println!("Request Failed, {}", res.status());
    }

    let events: Vec<Event> = res.json().await?;

    let latest_events = events.iter().take(15);
    println!("Showing up to 15 latest events for {}", entered_username);
    print!("Output: \n");
    for event in latest_events {
        // println!("{:#?}", event);
        
        println!(" - {}", describe_event(event));
    }


    
    // Example: deserialize the provided JSON array into Vec<Event>
    let example_json = r#"[
        {
            "id": "22249084947",
            "type": "WatchEvent",
            "actor": {
                "id": 583231,
                "login": "octocat",
                "display_login": "octocat",
                "gravatar_id": "",
                "url": "https://api.github.com/users/octocat",
                "avatar_url": "https://avatars.githubusercontent.com/u/583231?v=4"
            },
            "repo": {
                "id": 1296269,
                "name": "octocat/Hello-World",
                "url": "https://api.github.com/repos/octocat/Hello-World"
            },
            "payload": {
                "action": "started"
            },
            "public": true,
            "created_at": "2022-06-09T12:47:28Z"
        },
        {
            "id": "22249084964",
            "type": "PushEvent",
            "actor": {
                "id": 583231,
                "login": "octocat",
                "display_login": "octocat",
                "gravatar_id": "",
                "url": "https://api.github.com/users/octocat",
                "avatar_url": "https://avatars.githubusercontent.com/u/583231?v=4"
            },
            "repo": {
                "id": 1296269,
                "name": "octocat/Hello-World",
                "url": "https://api.github.com/repos/octocat/Hello-World"
            },
            "payload": {
                "repository_id": 1296269,
                "push_id": 10115855396,
                "ref": "refs/heads/master",
                "head": "7a8f3ac80e2ad2f6842cb86f576d4bfe2c03e300",
                "before": "883efe034920928c47fe18598c01249d1a9fdabd"
            },
            "public": true,
            "created_at": "2022-06-07T07:50:26Z"
        }
    ]"#;

    let events: Vec<Event> = serde_json::from_str(example_json)?;
    println!("Loaded {} events from example JSON", events.len()); // Ignore this
    Ok(())
}


fn describe_event(event: &Event) -> String {
    match event.event_type.as_str() {
        "PushEvent" => {
            // Try to get commit count from payload if available
            let commit_count = event.payload.get("size")
                .and_then(|v| v.as_u64())
                .unwrap_or(1); // Default to 1 if not present

            format!(
                "Pushed {} commit{} to {}",
                commit_count,
                if commit_count == 1 { "" } else { "s" },
                event.repo.name
            )
        }
        "IssuesEvent" => {
            let action = event.payload.get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("performed an action on");
            format!(
                "{} a(n) issue in {}",
                capitalize_first(action),
                event.repo.name
            )
        }
        "WatchEvent" => {
            format!(
                "Starred {}",
                event.repo.name
            )
        }
        "CreateEvent" => {
            let ref_type = event.payload.get("ref_type")
                .and_then(|v| v.as_str())
                .unwrap_or("repository");
            format!(
                "Created a new {} in {}",
                ref_type,
                event.repo.name
            )
        }
        "ForkEvent" => {
            format!(
                "Forked {}",
                event.repo.name
            )
        }
        // Add more event types as needed
        other => format!("Did {} in {}", other, event.repo.name),
    }
}

// Helper to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// // Example usage in your main loop:
// for event in latest_events {
//     println!(" - {}", describe_event(event));
// }