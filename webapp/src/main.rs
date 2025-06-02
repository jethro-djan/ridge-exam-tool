#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_session::{SessionMiddleware, storage::CookieSessionStore, config::PersistentSession};
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use leptos_meta::MetaTags;
    use webapp::app::App;

    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // Move all database setup outside of the block
    use webapp::app::db::server;
    dotenvy::dotenv().ok();
    
    let pool = server::connect()
        .await
        .expect("Failed to create database pool");

    server::create_users_table(&pool)
        .await
        .expect("Failed to create users table");

    server::seed_admin_user(&pool)
        .await
        .expect("Failed to seed admin user");

    server::create_sessions_table(&pool)
        .await
        .expect("Failed to create sessions table");

    let secret_key = server::get_secret_session_key();

    let config = get_configuration(None).unwrap();
    let addr = config.leptos_options.site_addr;

    let leptos_options_data = web::Data::new(config.leptos_options.clone());

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &config.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        let pool = pool.clone();

        println!("Listening on http://{}", &addr);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(leptos_options_data.clone())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                let pool = pool.clone();
                move || {
                    provide_context(pool.clone());
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1.0"
                                />
                                <meta charset="Utf-8" />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_name(String::from("webapp_session"))
                .cookie_http_only(true)
                .cookie_secure(false) 
                .cookie_same_site(cookie::SameSite::Lax)
                .cookie_path("/".to_string()) 
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(cookie::time::Duration::days(30))
                )
                .build()
            )
    })
    .bind(&addr)?
    .run()
    .await
}


#[cfg(not(feature = "ssr"))]
pub fn main() {}
