#![no_std]

use gstd::{msg, prelude::*, ActorId};
use collections::HashMap;
use scale::{Encode, Decode};

// Define the main state as a static variable
static mut SOCIAL_MEDIA: Option<SocialMediaState> = None;

// Define the main state struct
#[derive(Default)]
pub struct SocialMediaState {
    pub users: HashMap<ActorId, String>,
    pub posts: Vec<Post>,
    pub next_post_id: u64,
}

// Define structs for Post and User
#[derive(Encode, Decode, Clone)]
pub struct Post {
    pub post_id: u64,
    pub user_id: ActorId,
    pub title: String,
    pub content: String,
}

#[derive(Encode, Decode)]
pub struct User {
    pub user_id: ActorId,
    pub username: String,
}

// Define functions to interact with the social media state
impl SocialMediaState {
    // Function to create a new user
    pub fn create_user(&mut self, user_id: ActorId, username: String) -> bool {
        if self.users.contains_key(&user_id) {
            return false; // User already exists
        }
        self.users.insert(user_id, username);
        true
    }

    // Function to create a new post
    pub fn create_post(&mut self, user_id: ActorId, title: String, content: String) -> bool {
        if !self.users.contains_key(&user_id) {
            return false; // User must exist to create a post
        }
        let post = Post {
            post_id: self.next_post_id,
            user_id,
            title,
            content,
        };
        self.next_post_id += 1;
        self.posts.push(post);
        true
    }

    // Function to retrieve posts by user
    pub fn get_posts_by_user(&self, user_id: ActorId) -> Vec<&Post> {
        self.posts.iter().filter(|p| p.user_id == user_id).collect()
    }
}

// Initialization function
#[no_mangle]
pub unsafe extern "C" fn init() {
    SOCIAL_MEDIA = Some(SocialMediaState::default());
}

// Main function
#[gstd::async_main]
async fn main() {
    unsafe {
        let social_media = SOCIAL_MEDIA.as_mut().expect("Social media state not initialized");

        match msg::load::<String>() {
            Ok(username) => {
                let user_id = msg::source();
                let success = social_media.create_user(user_id, username);
                msg::reply(success, 0).expect("Failed to reply");
            }
            Err(_) => {
                // Handle other messages such as creating posts, comments, etc.
            }
        }
    }
}

// Metadata function
#[no_mangle]
pub extern "C" fn meta() -> *const u8 {
    static META: Meta = Meta {
        meta: Out::meta::<SocialMediaState>(),
        input: Out::input::<String>(),
        output: Out::output::<bool>(),
        // Define other metadata properties
    };

    &META as *const Meta as *const u8
}
