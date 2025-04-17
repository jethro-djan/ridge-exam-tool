// #[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use webapp::app::App;
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use leptos_meta::MetaTags;

    dotenvy::dotenv().ok();
    let pool = webapp::db::connect().await.expect("Failed to create database pool");

    let config = get_configuration(None).unwrap();
    let addr = config.leptos_options.site_addr;

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &config.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        let pool = pool.clone();

        println!("Listening on http::/{}", &addr);

        App::new()
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
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(&addr)?
    .run()
    .await
}

// #[cfg(not(feature = "ssr"))]
// pub fn main() {
//     use webapp::app::*;
//     use leptos::mount::mount_to_body;
// 
//     console_error_panic_hook::set_once();
//     mount_to_body(App);
// }
