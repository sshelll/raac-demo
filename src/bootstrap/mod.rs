#![allow(dead_code)]

mod actions;
mod roles;

pub async fn boot() {
    tokio::join!(roles::boot(), actions::boot());
}
