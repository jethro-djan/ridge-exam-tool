use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {

    view! {
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
                    <LoginFormTitle />
                    <LoginForm />
                </LoginFormContainer>
                <Footer />
            </PageContent>
        </PageLayout>

    }
}

#[component]
fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <main class="">
            {children()}
        </main>
        <Footer/>
    }
}

#[component]
fn PageContent(children: Children) -> impl IntoView {
    view! {
        <body class="">
            {children()}
        </body>
    }
}

#[component]
fn LoginFormContainer(children: Children) -> impl IntoView {
    view! {
        <div class="bg-gray-100 flex justify-center h-screen items-center">
            <div class="bg-white rounded-lg shadow-md p-8 w-full max-w-md">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn LoginFormTitle() -> impl IntoView {
    view! {
        <div class="text-center mb-8">
            <div class="flex justify-center">
                <img src="/public/Ridge_School_Kumasi_Logo.png" />
            </div>
            <h1 class="text-2xl font-semibold text-gray-800">
                "Examination Management Portal"
            </h1>
        </div>
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    view! {
        <form class="space-y-6">
            <div>
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
                >

                    "Sign in"
                </button>
            </div>
        </form>

        <div class="mt-6">
            <div class="relative">
                <div class="absolute inset-0 flex items-center">
                    <div class="w-full border-t border-gray-300"></div>
                </div>
                <div class="relative flex justify-center text-sm">
                    <span class="px-2 bg-white text-gray-500">
                        School Management System
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
