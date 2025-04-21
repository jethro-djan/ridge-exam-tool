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
        path,
        StaticSegment,
        components::{Route, Router, Routes, ParentRoute},
        nested_router::Outlet,
    };

    #[server]
    pub async fn login_user(
        username: String, 
        password: String
    ) -> Result<bool, ServerFnError> {
        #[cfg(feature = "ssr")]
        {
            use leptos_actix::extract;
            // use sqlx::PgPool;
            use actix_session::Session;

            dotenvy::dotenv().ok();
            let pool = crate::db::server::connect().await.expect("Failed to create database pool");

            // let pool_data: web::Data<Pool<Postgres>> = extract().await?;
            // let pool = pool_data.get_ref();
            
            let session: Session = extract().await?;

            crate::db::login(&pool, session, username, password).await 
        }

        #[cfg(not(feature = "ssr"))]
        {
            Err(ServerFnError::ServerError("Server function called on client".into()))
        }
    }

    #[component]
    pub fn App() -> impl IntoView {
        provide_meta_context();
        view! {
            <Stylesheet id="leptos" href="/pkg/webapp.css" />
            <Router>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment(Page::Login.path()) view=LoginView/>
                    <ParentRoute path=StaticSegment(Page::AdminPanel.path()) view=AdminPanelView>
                        <Route path=StaticSegment(Page::Users.path()) view=UserManagementView/>
                        <Route path=StaticSegment(Page::Roles.path()) view=RoleManagementView/>
                        <Route path=StaticSegment(Page::Settings.path()) view=SettingsView/>
                        <Route path=path!("") view=DashboardView/>
                    </ParentRoute>
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
            </PageLayout>

        }
    }

    #[component]
    fn PageLayout(children: Children) -> impl IntoView {
        view! {
            <div class="min-h-screen flex flex-col bg-gray-100">
                <main class="flex-grow">
                    {children()}
                </main>
                <Footer />
            </div>
        }
    }

    #[component]
    fn PageContent(children: Children) -> impl IntoView {
        view! {
            <div class="h-screen flex items-center justify-center">
                {children()}
            </div>
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


        let login_user_request = Action::new(|login_credentials: &(String, String)| {
            let (username, password) = (
                login_credentials.0.to_owned(), login_credentials.1.to_owned()
            );
            async move { login_user(username, password).await }
        });

        let navigate = leptos_router::hooks::use_navigate();

        Effect::new(move |_| {
            match login_user_request.value().get() {
                Some(Ok(true)) => {
                    navigate("/admin", Default::default());
                }
                Some(Ok(false)) => {
                    set_error_msg.set(String::from("Invalid credentials"));
                }
                Some(Err(e)) => {
                    set_error_msg.set(format!("Login error: {}", e));
                }
                _ => {}
            }
        });

        view! {
            {render_prop()}
            <form 
                class="space-y-6"
                on:submit=move |ev| {
                    ev.prevent_default();
                    login_user_request
                       .dispatch((username.get(), password.get()));
                }
            >
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
                        type="password"
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
                        prop:value=error_msg
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
            <div class="bg-gray-100 font-sans">
                <div class="flex h-screen">
                    <Sidebar />
                    <Outlet />
                </div>
            </div>
        }
    }

    #[component]
    fn Sidebar() -> impl IntoView {
        view! {
            <div class="w-64 bg-gray-600 text-white">
                <div class="h-16 bg-blue-600 flex items-center justify-center">
                    <h1 class="text-xl font-bold">"Admin Panel"</h1>
                </div>
                <nav class="mt-8">
                    <ul>
                        <li class="px-6 py-3 hover:bg-gray-700">
                            <a href="/admin" class="block">"Dashboard"</a>
                        </li>
                        <li class="px-6 py-3 hover:bg-gray-700">
                            <a href="/admin/users" class="block font-medium">User Management</a>
                        </li>
                        <li class="px-6 py-3 hover:bg-gray-700">
                            <a href="/admin/roles" class="block">Role Management</a>
                        </li>
                        <li class="px-6 py-3 hover:bg-gray-700">
                            <a href="#" class="block">Audit Logs</a>
                        </li>
                        <li class="px-6 py-3 hover:bg-gray-700">
                            <a href="/admin/settings" class="block">Settings</a>
                        </li>
                    </ul>
                </nav>
            </div>
        }
    }

    fn DashboardView() -> impl IntoView {
        view! {<p>"Dashboard view"</p>}
    }

    fn UserManagementView() -> impl IntoView {
        view! {<p>"User managment view"</p>}
    }
    fn RoleManagementView() -> impl IntoView {
        view! {<p>"Role managment view"</p>}
    }

    fn SettingsView() -> impl IntoView {
        view! {<p>"Settings view"</p>}
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
        Users,
        Roles,
        Settings,
    }

    impl Page {
        pub fn path(&self) -> &'static str {
            match self {
                Self::Login => "/",
                Self::AdminPanel => "/admin",
                Self::Users => "users",
                Self::Roles => "roles",
                Self::Settings => "settings",
            }
        }
    }
}

pub mod db {
    use serde::{Deserialize, Serialize};

    #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password_hash: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserSession {
        pub user_id: i32,
        pub username: String,
        pub session_id: String,
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        DbConnectionError,
        PasswordError,
        InvalidCredentials,
        TableNotCreated,
        SessionTableNotCreated,
    }
 
    #[cfg(feature = "ssr")]
    impl From<sqlx::Error> for Error {
        fn from(error: sqlx::Error) -> Error {
            dbg!(error);

            Error::DbConnectionError
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let error_msg = match self {
                Self::DbConnectionError => "Failed to connect to database",
                Self::PasswordError => "DbConnectionError: Failed to connect to database",
                Self::InvalidCredentials => "Incorrect username and password. Please try again.",
                Self::TableNotCreated => "Database users table not created.",
                Self::SessionTableNotCreated => "Session users table not created.",
            }; 
            
            write!(f, "{}", error_msg)
        }
    }

    impl std::error::Error for Error {}

    #[cfg(feature = "ssr")]
    pub mod server {
        use super::User;
        use super::Error;
        use std::sync::Arc;
        use sqlx::{PgPool, postgres::PgPoolOptions};
        use dotenvy::dotenv;
        use leptos::prelude::ServerFnError;
        use argon2::{ 
            password_hash::{
                rand_core::OsRng,
                SaltString, PasswordHash, PasswordHasher, PasswordVerifier,
            },
            Argon2,
        };

        use actix_web::{cookie::Key};
        use actix_session::{Session};
        use uuid::Uuid;


        pub async fn connect() -> Result<Arc<PgPool>, Error> {
            dotenv().ok();
            let database_url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await?;

            Ok(Arc::new(pool))
        }

        pub async fn create_users_table(pool: &PgPool) -> Result<(), Error> {
            let query = "
                CREATE TABLE IF NOT EXISTS users (
                    id SERIAL PRIMARY KEY,
                    username VARCHAR(100) NOT NULL UNIQUE,
                    password_hash VARCHAR(255) NOT NULL,
                    role VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            ";

            sqlx::query(query)
                .execute(pool)
                .await
                .map_err(|_| Error::TableNotCreated)?;

            Ok(())
        }

        pub async fn seed_admin_user(pool: &PgPool) -> Result<(), Error> {
            dotenv().ok();
            let username = "admin";
            let password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".into());
            let role = "admin";
            
            let salt = SaltString::generate(&mut OsRng);

            let argon2 = Argon2::default();
            let password_hash = argon2.hash_password(
                password.as_bytes(),
                &salt
            ).unwrap();

            sqlx::query(
                r#"
                INSERT INTO users (username, password_hash, role)
                VALUES ($1, $2, $3)
                ON CONFLICT (username) DO NOTHING
                "#,
            )
            .bind(username)
            .bind(password_hash.to_string())
            .bind(role)
            .execute(pool)
            .await?;

            Ok(())
        }

        #[cfg(feature = "ssr")]
        pub async fn login(
            pool: &PgPool,
            session: Session,
            username: String, 
            password: String
        ) -> Result<bool, ServerFnError> {

            let user = sqlx::query_as::<_, User>(
                "SELECT id, username, password_hash FROM users where username = $1",
            )
            .bind(&username)
            .fetch_optional(pool)
            .await
            .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;

            match user {
                Some(user) => {
                    let parsed_hash = PasswordHash::new(&user.password_hash)
                        .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;
                    
                    let argon2 = Argon2::default();
                    let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash)
                        .is_ok();
                    
                    if is_valid {
                        let session_id = Uuid::new_v4().to_string();

                        crate::db::server::create_user_session(user.id, session_id.clone(), pool)
                            .await
                            .expect("Failed to create a user session");

                        let user_session = crate::db::UserSession {
                            user_id: user.id,
                            username: username,
                            session_id: session_id.clone(),
                        };

                        session.insert("user_session", user_session)
                            .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;

                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                None => Ok(false)
            }
        }

        pub async fn logout(
            pool: &PgPool,
            session: Session
        ) -> Result<(), ServerFnError> {
            if let Ok(Some(user_session)) = session.get::<crate::db::UserSession>("user_session")
                .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string())) {
                 sqlx::query("DELETE FROM user_sessions WHERE session_id = $1")
                    .bind(&user_session.session_id)
                    .execute(pool)
                    .await
                    .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;
            }

            session.purge();
            Ok(())
        }

        pub fn get_secret_session_key() -> Key {
            dotenvy::dotenv().ok();

            match std::env::var("SESSION_KEY") {
                Ok(key) => {
                    if key.len() < 32 {
                        eprintln!("Warning: SESSION_KEY is too short. Using a randomly generated key instead.");
                        Key::generate()
                    } else {
                        Key::from(key.as_bytes())
                    }
                }
                Err(_) => {
                    eprintln!("SESSION_KEY not found in environment. Using a randomly generated key.");
                    eprintln!("Note: Sessions will be invalidated on server restart");
                    Key::generate()
                }
            }
        }

        pub async fn create_sessions_table(pool: &PgPool) -> Result<(), ServerFnError> {
            let query = "
                 CREATE TABLE IF NOT EXISTS user_sessions (
                    id SERIAL PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    session_id VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP NOT NULL,
                    expires_at TIMESTAMP NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                )
            ";

            sqlx::query(query)
                .execute(pool)
                .await
                .map_err(|_| Error::SessionTableNotCreated)?;

            Ok(())
        }

        pub async fn create_user_session(
            user_id: i32, 
            session_id: String, 
            pool: &PgPool
        ) -> Result<(), ServerFnError> {
            let query = "
                INSERT INTO user_sessions (user_id, session_id, created_at, expires_at)
                VALUES ($1, $2, NOW(), NOW() + INTERVAL '7 days')
            ";
            sqlx::query(query)
                .bind(user_id)
                .bind(&session_id)
                .execute(pool)
                .await
                .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;

            Ok(())
        }
    }

    #[cfg(feature = "ssr")]
    pub use server::*;
}
