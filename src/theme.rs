use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::time::Duration;
use rocket::fs::NamedFile;
use std::path::PathBuf;

pub async fn get_theme(path: PathBuf, jar: &CookieJar<'_>) -> Option<NamedFile>
{
    let theme = match jar.get("theme").map(|cookie| cookie.value()) {
        Some("light") => "light",
        Some("dark") => "dark",
        _ => { set_theme(jar, "light"); "light" }
    };
    let file = path.file_name()?.to_str()?;
    NamedFile::open(format!("public/res/themed/{}/{}", theme, file)).await.ok()
}

pub async fn toggle_theme(path: PathBuf, jar: &CookieJar<'_>) -> Option<NamedFile>
{
    let switch = match jar.get("theme").map(|cookie| cookie.value()) {
        Some("light") => "dark",
        Some("dark") => "light",
        _ => "dark"
    };
    set_theme(jar, switch);
    let file = path.file_name()?.to_str()?;
    NamedFile::open(format!("public/res/themed/{}/{}", switch, file)).await.ok()
}

fn set_theme(jar: &CookieJar<'_>, theme: &'static str)
{
    jar.add(Cookie::build("theme", theme)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::weeks(4))
        .finish()
    )
}