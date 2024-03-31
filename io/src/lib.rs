#![no_std]

extern crate gstd;

use gstd::{exec, msg, prelude::*, ActorId};
use gmeta::{Meta, Out};
use scale_info::TypeInfo;
use scale::{Decode, Encode};

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct User {
    user_id: ActorId,
    username: String,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Post {
    post_id: u64,
    user_id: ActorId,
    title: String,
    body: String,
    image_url: Option<String>,
    caption: Option<String>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Comment {
    comment_id: u64,
    post_id: u64,
    user_id: ActorId,
    content: String,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Like {
    like_id: u64,
    post_id: u64,
    user_id: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Default)]
pub struct SocialMedia {
    users: Vec<User>,
    posts: Vec<Post>,
    comments: Vec<Comment>,
    likes: Vec<Like>,
    next_post_id: u64,
    next_comment_id: u64,
    next_like_id: u64,
}

impl SocialMedia {
    pub fn create_user(&mut self, username: String) -> bool {
        let user_id = exec::origin();
        if self.users.iter().any(|u| u.user_id == user_id) {
            return false; // User already exists
        }
        let new_user = User { user_id, username };
        self.users.push(new_user);
        true
    }

    pub fn create_post(&mut self, title: String, body: String, image_url: Option<String>, caption: Option<String>) -> bool {
        let user_id = exec::origin();
        if self.users.iter().all(|u| u.user_id != user_id) {
            return false; // User must exist to create a post
        }

        let new_post = Post {
            post_id: self.next_post_id,
            user_id,
            title,
            body,
            image_url,
            caption,
        };
        self.next_post_id += 1;
        self.posts.push(new_post);
        true
    }

    pub fn create_comment(&mut self, post_id: u64, content: String) -> bool {
        let user_id = exec::origin();
        if self.users.iter().all(|u| u.user_id != user_id) {
            return false; // User must exist to create a comment
        }
        if self.posts.iter().all(|p| p.post_id != post_id) {
            return false; // Post must exist to comment on
        }

        let new_comment = Comment {
            comment_id: self.next_comment_id,
            post_id,
            user_id,
            content,
        };
        self.next_comment_id += 1;
        self.comments.push(new_comment);
        true
    }

    pub fn create_like(&mut self, post_id: u64) -> bool {
        let user_id = exec::origin();
        if self.users.iter().all(|u| u.user_id != user_id) {
            return false; // User must exist to like a post
        }
        if self.posts.iter().all(|p| p.post_id != post_id) {
            return false; // Post must exist to like
        }

        let new_like = Like {
            like_id: self.next_like_id,
            post_id,
            user_id,
        };
        self.next_like_id += 1;
        self.likes.push(new_like);
        true
    }

    pub fn get_comments(&self, post_id: u64) -> Vec<Comment> {
        self.comments.iter().filter(|c| c.post_id == post_id).cloned().collect()
    }

    pub fn get_likes(&self, post_id: u64) -> Vec<Like> {
        self.likes.iter().filter(|l| l.post_id == post_id).cloned().collect()
    }

    pub fn get_posts_by_user(&self, user_id: ActorId) -> Vec<Post> {
        self.posts.iter().filter(|p| p.user_id == user_id).cloned().collect()
    }
}

static mut SOCIAL_MEDIA: Option<SocialMedia> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    SOCIAL_MEDIA = Some(SocialMedia::default());
}

#[gstd::async_main]
async fn main() {
    unsafe {
        let social_media = SOCIAL_MEDIA.get_or_insert_with(SocialMedia::default);

        match msg::load::<String>() {
            Ok(username) => {
                // Handle message as user creation request
                let success = social_media.create_user(username);
                msg::reply(success, 0).expect("Failed to reply");
            }
            Err(_) => {
                // Handle other messages such as creating posts, comments, etc.
            }
        }
    }
}

// Add metadata for the contract
#[no_mangle]
pub extern "C" fn meta() -> *const u8 {
    static META: Meta = Meta {
        meta: Out::meta::<SocialMedia>(),
        input: Out::input::<String>(),
        output: Out::output::<bool>(),
        // Define other metadata properties
    };

    &META as *const Meta as *const u8
}
