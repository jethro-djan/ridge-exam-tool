#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

pub mod app {
    use leptos::prelude::*;
    use leptos_meta::*;
    use leptos_router::{
        StaticSegment,
        components::{Route, Router, Routes},
    };
    use leptos::task::spawn_local;

    #[component]
    pub fn App() -> impl IntoView {
        provide_meta_context();
        view! {
            <Stylesheet id="leptos" href="/pkg/webapp.css" />
            <Router>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment(Page::Login.path()) view=LoginView/>
                    <Route path=StaticSegment(Page::AdminPanel.path()) view=AdminPanelView/>
                </Routes>
            </Router>
        }
    }

    #[component]
    fn LoginView() -> impl IntoView {
        view! {
            <PageLayout>
                <PageContent>
                    <LoginFormContainer>
                        <LoginForm render_prop=|| LoginFormTitle />
                    </LoginFormContainer>
                </PageContent>
                <Footer />
            </PageLayout>

        }
    }

    #[component]
    fn PageLayout(children: Children) -> impl IntoView {
        view! {
            <body class="bg-gray-100">
                {children()}
            </body>
        }
    }

    #[component]
    fn PageContent(children: Children) -> impl IntoView {
        view! {
            <main class="h-screen flex items-center justify-center">
                {children()}
            </main>
        }
    }

    #[component]
    fn LoginFormContainer(children: Children) -> impl IntoView {
        view! {
            <div class="bg-white rounded-lg shadow-md p-8 w-full max-w-md">
                {children()}
            </div>
        }
    }

    #[component]
    fn LoginFormTitle() -> impl IntoView {
        view! {
            <div class="text-center mb-8">
                <div class="flex justify-center">
                    <img src="/assets/Ridge_School_Kumasi_Logo.png" />
                </div>
                <h1 class="text-2xl font-semibold text-gray-800">
                    "Examination Management Portal"
                </h1>
            </div>
        }
    }

    #[component]
    fn LoginForm<F, IV>(
        render_prop: F,
    ) -> impl IntoView
    where 
        F: Fn() -> IV + std::marker::Send + 'static,
        IV: IntoView + 'static,
    {
        let username = RwSignal::new(String::new());
        let password = RwSignal::new(String::new());
        let (error_msg, set_error_msg) = signal(String::new());

        // let (_user, set_user) = signal(None::<shared::db::User>);

        // let get_db_pool = Action::new(|_| async move { shared::db::connect(database_url) });

        // let login_user_request = Action::new(|username: &String, password: &String| {
        //     let username = username.to_owned();
        //     let password = password.to_owned();
        //     let pool = get_db_pool.dispatch();
        //     async move { shared::db::login_user(pool, &username, &password).await }
        // });

        view! {
            {render_prop()}
            <form class="space-y-6">
                <div>
                    <label for="username" class="block text-sm font-medium text-gray-700 mb-1">
                        "Username" 
                    </label>
                    <input
                        type="text"
                        name="username"
                        placeholder="Enter your username"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        required
                        bind:value=username
                    />
                </div>

                <div>
                    <label for="password" class="block text-sm font-medium text-gray-700 mb-1">
                        "Password" 
                    </label>
                    <input
                        type="text"
                        name="password"
                        placeholder="Enter your password"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        required
                        bind:value=password
                    />
                </div>

                <div>
                    <button
                        type="submit"
                        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        // on:submit=move |ev| {
                        //     ev.prevent_default();
                        //     spawn_local(async move {
                        //         match helpers::login(username.get(), password.get()).await {
                        //             Ok(user) => set_user.set(Some(user)),
                        //             Err(e) => println!("{:?}", e),
                        //         }
                        //     })
                        // }
                        // prop:value=error_msg
                    >

                        "Sign in"
                    </button>
                </div>
            </form>

            <div class="flex justify-center mt-6">
                <label>{error_msg}</label>
            </div>

            <div class="mt-6">
                <div class="relative">
                    <div class="absolute inset-0 flex items-center">
                        <div class="w-full border-t border-gray-300"></div>
                    </div>
                    <div class="relative flex justify-center text-sm">
                        <span class="px-2 bg-white text-gray-500">
                            "School Management System"
                        </span>
                    </div>
                </div>
            </div>
        }
    }

    #[component]
    fn AdminPanelView() -> impl IntoView {
        view! {
            <p>"Hello, there"</p>
        }
    }

    #[component]
    fn Footer() -> impl IntoView {
        view! {
            <div class="mt-6 text-center text-xs text-gray-500">
                <p>"© 2025 School Examination Portal. All rights reserved."</p>
            </div>
        }
    }

    pub enum Page {
        Login,
        AdminPanel,
    }

    impl Page {
        pub fn path(&self) -> &'static str {
            match self {
                Self::AdminPanel => "/admin",
                Self::Login => "/",
            }
        }
    }
}

// pub mod helpers {
//     use std::env;
//     use leptos::*;
//     use leptos::server_fn::ServerFnError;
// 
//     #[server]
//     pub async fn login(username: String, password: String) -> Result<shared::db::User, ServerFnError> {
//         let database_url =
//             env::var("DATABASE_URL").expect("Something went wrong with the database URL");
//         let pool = shared::db::connect(&database_url)
//             .await
//             .map_err(|e| ServerFnError::ServerError::<shared::db::Error>(e.to_string()))?;
//         let user = shared::db::login_user(&pool, &username, &password).await;
//         match user {
//             Ok(user) => Ok(user),
//             Err(e) => Err(ServerFnError::ServerError(e.to_string())),
//         }
//         // async move { shared::db::login_user(pool, &username, &password).await }
//         //     .map_err(|_| ServerFnError::ServerError("Something went wrong".into()))?
//     }
// 
//     // #[server]
//     // pub async fn get_db_connection() -> Result<Pool, ServerFnError> {
//     //     dotenv().ok();
//     //     let database_url =
//     //         env::var("DATABASE_URL").expect("Something went wrong with the database URL");
//     //     async move { shared::db::connect(&database_url).await }
//     //         .map_err(|_| ServerFnError::ServerError("DATABASE_URL not set".into()))?
//     // }
// }

#[cfg(feature = "ssr")]
pub mod db {
    use argon2::Argon2;
    use argon2::PasswordHash;
    use argon2::PasswordVerifier;
    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;
    use serde::{Deserialize, Serialize};
    use leptos::server;
    use leptos::prelude::ServerFnError;
    use dotenvy::dotenv;
    use leptos::context::use_context;

    use std::sync::Arc;
    use std::fmt;

    #[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password_hash: String,
    }

    pub async fn connect() -> Result<Arc<PgPool>, Error> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
            // .map_err(|error| Error::from(error))?;

        Ok(Arc::new(pool))
    }

    #[server]
    pub async fn create_users_table() -> Result<(), ServerFnError> {
        let pool = use_context::<PgPool>().expect("Missing pool");
        let query = "
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username VARCHAR(100) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL
                role VARCHAR(255) NOT NULL
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ";

        sqlx::query(query)
            .execute(&pool)
            .await
            .map_err(|_| Error::TableNotCreated)?;

        Ok(())
    }

    #[server]
    pub async fn login_user(username: String, password: String) -> Result<bool, ServerFnError> {
        let pool = use_context::<PgPool>().expect("Missing pool");
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash FROM users where username = $1",
        )
        .bind(username)
        .fetch_optional(&pool)
        .await?;

        match user {
            Some(user) => {
                let parsed_hash = PasswordHash::new(&user.password_hash)
                    .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;
                
                let argon2 = Argon2::default();
                let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok();
                
                if is_valid {
                    // In a real app, you would create a session here
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            None => Ok(false)
        }
    }

    // #[server]
    // async fn verify_password(password: &str, hash: &str) -> Result<bool, ServerFnError> {
    //     let parsed_hash = PasswordHash::new(hash).map_err(|_| Error::PasswordError)?;

    //     Ok(Argon2::default()
    //         .verify_password(password.as_bytes(), &parsed_hash)
    //         .is_ok())
    //         .map_err(|_| ServerFnError::ServerError)
    // }

    #[derive(Debug, Clone)]
    pub enum Error {
        DbConnectionError,
        PasswordError,
        InvalidCredentials,
        TableNotCreated,
    }

    impl From<sqlx::Error> for Error {
        fn from(error: sqlx::Error) -> Error {
            dbg!(error);

            Error::DbConnectionError
        }
    }

    // impl From<io::Error> for Error {
    //     fn from(error: io::Error) -> Self {
    //         match error {
    //             Error::InvalidCrene
    //         }

    // }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let error_msg = match self {
                Self::DbConnectionError => "Failed to connect to database",
                Self::PasswordError => "DbConnectionError: Failed to connect to database",
                Self::InvalidCredentials => "Incorrect username and password. Please try again.",
                Self::TableNotCreated => "Database error.",
            }; 
            
            write!(f, "{}", error_msg)
        }
    }

    impl std::error::Error for Error {}

    // impl std::fmt::Debug for Error {
    //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    //     }
    // }
}
