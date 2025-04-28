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
        components::{ParentRoute, Route, Router, Routes, A},
        nested_router::Outlet,
        path,
    };

    pub async fn get_users() -> Result<Vec<crate::db::User>, ServerFnError> {
        #[cfg(feature = "ssr")]
        {
            crate::db::server::get_users_impl().await
        }
        #[cfg(not(feature = "ssr"))]
        {
            unreachable!()
        }
    }

    #[server]
    pub async fn login_user(username: String, password: String) -> Result<bool, ServerFnError> {
        #[cfg(feature = "ssr")]
        {
            use leptos_actix::extract;
            use actix_session::Session;

            dotenvy::dotenv().ok();
            let pool = crate::db::server::connect()
                .await
                .expect("Failed to create database pool");

            let session: Session = extract().await?;

            crate::db::login(&pool, session, username, password).await
        }

        #[cfg(not(feature = "ssr"))]
        {
            Err(ServerFnError::ServerError(
                "Server function called on client".into(),
            ))
        }
    }

    #[server]
    pub async fn logout_user() -> Result<(), ServerFnError> {
        #[cfg(feature = "ssr")]
        {
            use leptos_actix::extract;
            use actix_session::Session;

            dotenvy::dotenv().ok();
            let pool = crate::db::server::connect()
                .await
                .expect("Failed to create database pool");

            let session: Session = extract().await?;

            crate::db::logout(&pool, session).await
        }

        #[cfg(not(feature = "ssr"))]
        {
            Err(ServerFnError::ServerError(
                "Server function called on client".into(),
            ))
        }
    }

    #[component]
    pub fn App() -> impl IntoView {
        provide_meta_context();
        view! {
            <Stylesheet id="leptos" href="/pkg/webapp.css" />
            <Router>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment(Page::Login.path()) view=LoginView />
                    <ParentRoute path=StaticSegment(Page::AdminPanel.path()) view=AdminPanelView>
                        <Route path=StaticSegment("") view=DashboardView />
                        <Route path=StaticSegment(Page::Users.path()) view=UserManagementView />
                        <Route path=StaticSegment(Page::Roles.path()) view=RoleManagementView />
                        <Route path=StaticSegment(Page::Settings.path()) view=SettingsView />
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
            <div class="min-h-screen flex flex-col bg-gray-50">
                <main class="flex-grow">{children()}</main>
                <Footer />
            </div>
        }
    }

    #[component]
    fn PageContent(children: Children) -> impl IntoView {
        view! { <div class="h-screen flex items-center justify-center">{children()}</div> }
    }

    #[component]
    fn LoginFormContainer(children: Children) -> impl IntoView {
        view! { <div class="bg-white rounded-lg shadow-md p-8 w-full max-w-md">{children()}</div> }
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
    fn LoginForm<F, IV>(render_prop: F) -> impl IntoView
    where
        F: Fn() -> IV + Send + Sync + 'static,
        IV: IntoView + 'static,
    {
        let username = RwSignal::new(String::new());
        let password = RwSignal::new(String::new());
        let (error_msg, set_error_msg) = signal(String::new());

        let login_user_request = Action::new(|login_credentials: &(String, String)| {
            let (username, password) = (
                login_credentials.0.to_owned(),
                login_credentials.1.to_owned(),
            );
            async move { login_user(username, password).await }
        });

        let navigate = leptos_router::hooks::use_navigate();

        Effect::new(move |_| match login_user_request.value().get() {
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
        });

        view! {
            {render_prop()}
            <form
                class="space-y-6"
                on:submit=move |ev| {
                    ev.prevent_default();
                    login_user_request.dispatch((username.get(), password.get()));
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

            <div class="flex justify-center mt-6 text-red-700">
                <label>{error_msg}</label>
            </div>

            <div class="mt-6">
                <div class="relative">
                    <div class="absolute inset-0 flex items-center">
                        <div class="w-full border-t border-gray-300"></div>
                    </div>
                    <div class="relative flex justify-center text-sm">
                        <span class="px-2 bg-white text-gray-500">"School Management System"</span>
                    </div>
                </div>
            </div>
        }
    }

    #[component]
    fn AdminPanelView() -> impl IntoView {
        let navigate = leptos_router::hooks::use_navigate();

        let users = Resource::new(
            || (), 
            |_| async move { get_users().await }
        );

        let title = RwSignal::new(String::from("Dashboard"));
        
        provide_context(users);
        provide_context(title);

        view! {
            <PageLayout>
                <PageContent>
                    <div class="flex h-screen">
                        <Sidebar />
                        <div class="flex-1 overflow-auto">
                            <TitleBar />
                            <div class="p-6">
                                <Outlet />
                            </div>
                        </div>
                    </div>
                </PageContent>
            </PageLayout>
        }
    }

    #[component]
    fn Sidebar() -> impl IntoView {
        let logout_user_request = Action::new(|input: &()| {
            async move { logout_user().await }
        });
        view! {
            <div class="w-64 bg-gray-800 text-white flex flex-col h-full">
                <div class="p-4 font-bold text-xl">School Admin</div>
                <nav class="mt-8 flex-grow">
                    <a href="/admin" class="block px-3 py-4 hover:bg-gray-700">
                        "Dashboard"
                    </a>
                    <a href="/admin/users" class="block px-3 py-4 hover:bg-gray-700">
                        "Users"
                    </a>
                    <a href="/admin/roles" class="block px-3 py-4 hover:bg-gray-700">
                        "Roles"
                    </a>
                    <a href="#" class="block px-3 py-4 hover:bg-gray-700">
                        "Audits"
                    </a>
                    <a href="/admin/settings" class="block px-3 py-4 hover:bg-gray-700">
                        "Settings"
                    </a>
                </nav>
                <div class="mt-auto border-t border-gray-700">
                    <a 
                        href="#" 
                        class="block px-3 py-4 hover:bg-gray-700 text-red-400"
                        on:click=move |ev| {
                            ev.prevent_default();
                            logout_user_request.dispatch(());
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate("/", Default::default());
                        }
                    >
                        "Logout"
                    </a>
                </div>
            </div>
        }
    }

    #[component]
    fn TitleBar() -> impl IntoView {
        let title = use_context::<RwSignal<String>>()
            .expect("title context should be provided");
        view! {
            <div class="flex-1 overflow-auto">
                <header class="bg-white shadow p-4 flex justify-between items-center">
                    <h1 class="text-2xl font-bold mb-4">{move || title.get()}</h1>
                    <div class="flex items-center">
                        <div class="w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center text-white">
                            <img src="#" />
                        </div>
                        <span class="mr-2 px-4">"Admin"</span>
                    </div>
                </header>
            </div>
        }
    }

    #[component]
    fn ActionBar() -> impl IntoView {
        view! {
            <div class="flex justify-between mb-4">
                <div class="relative">
                    <input type="text" placeholder="Search users..." class="pl-10 pr-4 py-2 border rounded w-64" />
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </div>
                </div>

                <div class="flex gap-2">
                    <button class="bg-gray-700 text-white px-4 py-2 rounded" disabled>
                        "Delete User"
                    </button>
                    <button class="bg-gray-700 text-white px-4 py-2 rounded" onclick="#">
                        "Add User"
                    </button>
                </div>
            </div>
        }
    }

    #[component]
    fn LoadingSpinner() -> impl IntoView {
        view! {
            <div class="bg-white rounded-lg shadow px-6 py-12 animate-fade-in">
                <div class="flex flex-col items-center space-y-4">
                    <div class="w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-spin" />
                    <p class="text-gray-600 text-sm animate-pulse">"Loading users..."</p>
                </div>
            </div>
        }
    }

    fn DashboardView() -> impl IntoView {
        let title = use_context::<RwSignal<String>>()
            .expect("title context should be provided");

        title.set("Dashboard".to_string());

        view! { <p>"Dashboard view"</p> }
    }

    fn UserManagementView() -> impl IntoView {
        let users = use_context::<Resource<Result<Vec<crate::db::User>, ServerFnError>>>()
            .expect("users context missing");
        let title = use_context::<RwSignal<String>>()
            .expect("title context should be provided");

        title.set("User Management".to_string());
        view! {
            <div class="p-4">

                <Suspense fallback=move || view! { <LoadingSpinner/> }>
                {move || {
                    users.get().map(|result| view! {
                        <ActionBar />
                        <div class="bg-white rounded-lg shadow overflow-auto-x">
                            <table class="min-w-full divide-y divide-gray-200">
                                <thead class="bg-gray-50">
                                    <tr>
                                        <th class="w-12 px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            <input type="checkbox" class="h-4 w-4" />
                                        </th>
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            "Name"
                                        </th>
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            "Username"
                                        </th>
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                            "Role"
                                        </th>
                                    </tr>
                                </thead>

                                <tbody class="bg-white divide-y divide-gray-200">
                                    {match result {
                                        Ok(users) => users.into_iter().map(|user| {
                                            view! {
                                                <tr class="border-t hover:bg-gray-50">
                                                    <td class="px-6 py-4 whitespace-nowrap">
                                                        <input type="checkbox" class="h-4 w-4" />
                                                    </td>
                                                    <td class="py-6 px-4 whitespace-nowrap">
                                                        <div class="text-sm font-medium text-gray-900">
                                                            {move || format!("{} {}", user.first_name, user.last_name)}
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap">
                                                        <div class="text-sm text-gray-500">
                                                            {user.username}
                                                        </div>
                                                    </td>
                                                    <td class="py-2 px-4">
                                                        <span class="px-2 py-1 inline-flex text-xs leading-5 font-medium rounded border border-gray-300">
                                                            {user.role_name}
                                                        </span>
                                                    </td>
                                                </tr>
                                            }.into_any()
                                        })
                                        .collect::<Vec<_>>(),
                                        Err(e) => vec![view! {
                                            <tr>
                                                <td colspan="5" class="py-2 px-4 text-red-500">
                                                    {move || format!("Error loading users: {}", e.to_string())}
                                                </td>
                                            </tr>
                                        }.into_any()]
                                    }}
                                </tbody>
                            </table>
                        </div>
                    })
                }}
                </Suspense>
            </div>
        }
    }

    fn RoleManagementView() -> impl IntoView {
        let title = use_context::<RwSignal<String>>()
            .expect("title context should be provided");

        title.set("Role Management".to_string());
        view! { <p>"Role managment view"</p> }
    }

    fn SettingsView() -> impl IntoView {
        let title = use_context::<RwSignal<String>>()
            .expect("title context should be provided");

        title.set("Settings".to_string());
        view! { <p>"Settings view"</p> }
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
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password_hash: String,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub role_id: i32,
        pub role_name: Option<String>,
        pub is_active: bool,
        pub created_at: chrono::DateTime<chrono::Utc>,
        pub last_updated: chrono::DateTime<chrono::Utc>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserSession {
        pub user_id: i32,
        pub username: String,
        pub session_id: String,
        pub role_id: i32,
        pub role_name: Option<String>,
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        DbConnectionError,
        PasswordError,
        InvalidCredentials,
        TableNotCreated,
        SessionTableNotCreated,
        InsertRolesFailed,
        TransactionFailed,
        RoleNotFound,
        SeedUserFailed,
        DatabaseQueryFailed,
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
                Self::InsertRolesFailed => "Failed to insert roles",
                Self::TransactionFailed => "Failed to commit transaction",
                Self::RoleNotFound => "Failed to retrieve role",
                Self::SeedUserFailed => "Failed to seed user",
                Self::DatabaseQueryFailed => "Failed to make database query",
            };

            write!(f, "{}", error_msg)
        }
    }

    impl std::error::Error for Error {}

    #[cfg(feature = "ssr")]
    pub mod server {
        use super::Error;
        use super::User;
        use argon2::{
            Argon2,
            password_hash::{
                PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
            },
        };
        use dotenvy::dotenv;
        use leptos::prelude::ServerFnError;
        use leptos::server;
        use leptos::context::use_context;
        use sqlx::{PgPool, postgres::PgPoolOptions, Row, Column};
        use std::sync::Arc;

        use actix_session::Session;
        use actix_web::cookie::Key;
        use uuid::Uuid;

        pub async fn connect() -> Result<Arc<PgPool>, Error> {
            dotenv().ok();
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await?;

            Ok(Arc::new(pool))
        }

        pub async fn create_roles_table(pool: &PgPool) -> Result<(), Error> {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS roles (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(100) NOT NULL UNIQUE,
                    description TEXT
                )"
            )
            .execute(pool)
            .await
            .map_err(|e| {
                log::error!("Failed to create roles table: {}", e);
                Error::TableNotCreated
            })?;

            let mut tx = pool.begin().await.map_err(|e| {
                log::error!("Failed to begin transaction: {}", e);
                Error::TransactionFailed
            })?;

            for (name, description) in [
                ("admin", "Full system access"),
                ("teacher", "Can view students and enter grades")
            ] {
                sqlx::query(
                    "INSERT INTO roles (name, description)
                    VALUES ($1, $2)
                    ON CONFLICT (name) DO NOTHING"
                )
                .bind(name)
                .bind(description)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    log::error!("Failed to insert role {}: {}", name, e);
                    Error::InsertRolesFailed
                })?;
            }

            tx.commit().await.map_err(|e| {
                log::error!("Failed to commit transaction: {}", e);
                Error::TransactionFailed
            })?;

            Ok(())
        }
        pub async fn create_users_table(pool: &PgPool) -> Result<(), Error> {
            create_roles_table(pool).await?;

            let query = "
                CREATE TABLE IF NOT EXISTS users (
                    id SERIAL PRIMARY KEY,
                    username VARCHAR(100) NOT NULL UNIQUE,
                    password_hash VARCHAR(255) NOT NULL,
                    first_name VARCHAR(100) NOT NULL,
                    last_name VARCHAR(100) NOT NULL,
                    email VARCHAR(255) NOT NULL UNIQUE,
                    role_id INTEGER NOT NULL REFERENCES roles(id),
                    is_active BOOLEAN DEFAULT TRUE,
                    created_at TIMESTAMPTZ DEFAULT NOW(),
                    last_updated TIMESTAMPTZ DEFAULT NOW()
                )
            ";

            sqlx::query(query)
                .execute(pool)
                .await
                .map_err(|e| {
                    log::error!("Failed to create users table: {}", e);
                    Error::TableNotCreated
                })?;

            Ok(())
        }

        pub async fn seed_admin_user(pool: &PgPool) -> Result<(), Error> {
            dotenv().ok();
            let username = "admin";

            let user_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(username)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    log::error!("Failed to check if admin user exists: {}", e);
                    Error::DatabaseQueryFailed
                })?;
            
            if user_exists {
                log::info!("Admin user already exists, skipping seed");
                return Ok(());
            }

            let password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".into());
            let first_name = "System";
            let last_name = "Administrator";
            let email = "admin@example.com";
            
            let role_id: i32 = sqlx::query_scalar("SELECT id FROM roles WHERE name = $1")
                .bind("admin")
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    log::error!("Failed to get admin role ID: {}", e);
                    Error::RoleNotFound
                })?;

            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

            sqlx::query(
                r#"
                INSERT INTO users (username, password_hash, first_name, last_name, email, role_id, is_active)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(username)
            .bind(password_hash.to_string())
            .bind(first_name)
            .bind(last_name)
            .bind(email)
            .bind(role_id)
            .bind(true)
            .execute(pool)
            .await
            .map_err(|e| {
                log::error!("Failed to seed admin user: {}", e);
                Error::SeedUserFailed
            })?;

            Ok(())
        }


        pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
            let query = sqlx::query(
                r#"
                SELECT
                    u.id,
                    u.username,
                    u.password_hash,
                    u.first_name,
                    u.last_name,
                    u.email,
                    u.role_id, 
                    r.name as "role_name",
                    u.is_active,
                    u.created_at,
                    u.last_updated
                FROM users u
                LEFT JOIN roles r ON u.role_id = r.id
                ORDER BY u.last_name, u.first_name
                "#
            );

            let row = query.fetch_one(pool).await.map_err(|e| {
                log::error!("Column check failed: {}", e);
                Error::DatabaseQueryFailed
            })?;

            log::info!("Columns: {:?}", row.columns().iter().map(|c| c.name()).collect::<Vec<_>>());

            let users = sqlx::query_as::<_, User>(
                r#"
                SELECT 
                    u.id, 
                    u.username, 
                    u.password_hash, 
                    u.first_name, 
                    u.last_name, 
                    u.email, 
                    u.role_id, 
                    r.name as "role_name", 
                    u.is_active, 
                    u.created_at, 
                    u.last_updated
                FROM users u
                LEFT JOIN roles r ON u.role_id = r.id
                ORDER BY u.last_name, u.first_name
                "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| {
                log::error!("Failed to retrieve users: {}", e);
                Error::DatabaseQueryFailed
            })?;

            Ok(users)
        }

        #[cfg_attr(feature = "ssr", server)]
        pub async fn get_users_impl() -> Result<Vec<User>, ServerFnError> {
            #[cfg(feature = "ssr")]
            {
                dotenvy::dotenv().ok();
                let pool = crate::db::server::connect()
                    .await
                    .expect("Failed to create database pool");
                // let pool = use_context::<PgPool>()
                //     .expect("Pool should be in context");
                println!("{}", format!("{:?}", pool.clone()));
                
                get_all_users(&pool)
                    .await
                    .map_err(|e| ServerFnError::ServerError(e.to_string()))
            }
        }

        #[cfg(feature = "ssr")]
        pub async fn login(
            pool: &PgPool,
            session: Session,
            username: String,
            password: String,
        ) -> Result<bool, ServerFnError> {
            let user = sqlx::query_as::<_, User>(
                r#"
                SELECT 
                    u.id, 
                    u.username, 
                    u.password_hash, 
                    u.first_name, 
                    u.last_name, 
                    u.email, 
                    u.role_id, 
                    r.name as "role_name", 
                    u.is_active, 
                    u.created_at, 
                    u.last_updated
                FROM users u
                LEFT JOIN roles r ON u.role_id = r.id
                WHERE u.username = $1
                "#,
            )
            .bind(&username)
            .fetch_optional(pool)
            .await
            .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;

            match user {
                Some(user) => {
                    if !user.is_active {
                        log::warn!("Login attempt for inactive user: {}", username);
                        return Ok(false);
                    }

                    let parsed_hash = PasswordHash::new(&user.password_hash)
                        .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;

                    let argon2 = Argon2::default();
                    let is_valid = argon2
                        .verify_password(password.as_bytes(), &parsed_hash)
                        .is_ok();

                    if is_valid {
                        let session_id = Uuid::new_v4().to_string();

                        crate::db::server::create_user_session(user.id, session_id.clone(), pool)
                            .await
                            .expect("Failed to create a user session");

                        let user_session = crate::db::UserSession {
                            user_id: user.id,
                            username: user.username.clone(),
                            session_id: session_id.clone(),
                            role_id: user.role_id,
                            role_name: user.role_name.clone(),
                            first_name: user.first_name.clone(),
                            last_name: user.last_name.clone(),
                        };

                        session
                            .insert("user_session", user_session)
                            .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))?;

                        Ok(true)
                    } else {
                        log::warn!("Invalid password for user: {}", username);
                        Ok(false)
                    }
                }
                None => {
                    log::warn!("Login attempt for non-existent user: {}", username);
                    Ok(false)
                }
            }
        }

        pub async fn logout(pool: &PgPool, session: Session) -> Result<(), ServerFnError> {
            if let Ok(Some(user_session)) = session
                .get::<crate::db::UserSession>("user_session")
                .map_err(|e| ServerFnError::<Error>::ServerError(e.to_string()))
            {
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
                        eprintln!(
                            "Warning: SESSION_KEY is too short. Using a randomly generated key instead."
                        );
                        Key::generate()
                    } else {
                        Key::from(key.as_bytes())
                    }
                }
                Err(_) => {
                    eprintln!(
                        "SESSION_KEY not found in environment. Using a randomly generated key."
                    );
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
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    expires_at TIMESTAMPTZ NOT NULL,
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
            pool: &PgPool,
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
