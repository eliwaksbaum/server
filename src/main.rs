#[macro_use] extern crate rocket;
use std::path::PathBuf;

use rocket::fs::{FileServer, NamedFile};
use rocket::response::{Redirect, content::RawHtml};
use rocket::http::{Status, CookieJar};
use rocket::form::Form;

pub mod blog;
pub mod projects;
pub mod mail;
pub mod theme;

#[catch(404)]
async fn not_found() -> Option<NamedFile>
{
    NamedFile::open("assets/404.html").await.ok()
}

#[catch(403)]
async fn permission_denied() -> Option<NamedFile>
{
    NamedFile::open("assets/403.html").await.ok()
}

#[get("/projects")]
fn projects_home() -> Result<RawHtml<String>, Status>
{
    projects::shuffle_cards()
}

#[get("/blog")]
fn blog_home() -> Result<RawHtml<String>, Status>
{
    blog::blog(None)
}

#[get("/blog/tag/<tag>")]
fn blog_tag(tag: &str) -> Result<Result<RawHtml<String>, Status>, Redirect>
{
    if tag.find(char::is_uppercase).is_some()
    {
        return Err(Redirect::to(uri!(blog_tag(blog::normalize(tag)))))
    }
    Ok(blog::blog(Some(tag)))
}

#[post("/contact.html", data="<email>")]
async fn send_mail(email: Form<mail::Email<'_>>) -> Option<NamedFile>
{
    let path = match mail::contact(email) {
        Ok(_) => "assets/email-ok.html",
        Err(_) => "assets/email-err.html"
    };
    NamedFile::open(path).await.ok()
}

#[get("/res/themed/cur/<path..>")]
async fn get_theme(path: PathBuf, jar: &CookieJar<'_>) -> Option<NamedFile>
{
    theme::get_theme(path, jar).await
}

#[get("/res/themed/toggle/<path..>")]
async fn toggle_theme(path: PathBuf, jar: &CookieJar<'_>) -> Option<NamedFile>
{
    theme::toggle_theme(path, jar).await
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", FileServer::from("public/"))
        .mount("/", routes![projects_home, blog_home, blog_tag, send_mail, get_theme, toggle_theme])
        .register("/", catchers![not_found, permission_denied])
}