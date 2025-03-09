use crate::error_template::ErrorTemplate;
use crate::pages::about::AboutPage;
use crate::pages::games::GamesPage;
use crate::pages::home::HomePage;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/dabney_engineering.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Title text="Dabney Engineering Inc."/>

        <Router>
            <main>
                <nav class="bg-gray-800 text-white p-4">
                    <div class="container mx-auto flex justify-between items-center">
                        <A href="/">
                            <span class="text-xl font-bold">"Dabney Engineering"</span>
                        </A>
                        <div class="space-x-4">
                            <A href="/">
                                <span class="hover:text-gray-300">"Home"</span>
                            </A>
                            <A href="/games">
                                <span class="hover:text-gray-300">"Games"</span>
                            </A>
                            <A href="/about">
                                <span class="hover:text-gray-300">"About"</span>
                            </A>
                        </div>
                    </div>
                </nav>

                <Routes fallback=|| view! { <ErrorTemplate/> }>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/games") view=GamesPage/>
                    <Route path=path!("/about") view=AboutPage/>
                </Routes>

                <footer class="bg-gray-800 text-white p-4 mt-8">
                    <div class="container mx-auto text-center">
                        <p>"© 2024 Dabney Engineering Inc. All rights reserved."</p>
                    </div>
                </footer>
            </main>
        </Router>
    }
}
