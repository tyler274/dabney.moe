use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            // Hero Section
            <section class="bg-gray-900 text-white py-20">
                <div class="container mx-auto px-4">
                    <div class="max-w-3xl mx-auto text-center">
                        <h1 class="text-5xl font-bold mb-6">"Welcome to Dabney Engineering"</h1>
                        <p class="text-xl mb-8">"Crafting immersive gaming experiences through innovative technology"</p>
                        <a href="/games" class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300">
                            "Explore Our Games"
                        </a>
                    </div>
                </div>
            </section>

            // Features Section
            <section class="py-16">
                <div class="container mx-auto px-4">
                    <h2 class="text-3xl font-bold text-center mb-12">"Why Choose Dabney Engineering?"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <div class="text-center p-6">
                            <div class="text-4xl mb-4">"🎮"</div>
                            <h3 class="text-xl font-bold mb-2">"Innovative Gameplay"</h3>
                            <p class="text-gray-600">"We create unique gaming experiences that push the boundaries of what's possible."</p>
                        </div>
                        <div class="text-center p-6">
                            <div class="text-4xl mb-4">"🚀"</div>
                            <h3 class="text-xl font-bold mb-2">"Cutting-edge Technology"</h3>
                            <p class="text-gray-600">"Using the latest tech to deliver smooth, responsive, and immersive games."</p>
                        </div>
                        <div class="text-center p-6">
                            <div class="text-4xl mb-4">"💡"</div>
                            <h3 class="text-xl font-bold mb-2">"Creative Vision"</h3>
                            <p class="text-gray-600">"Bringing fresh ideas and unique perspectives to every project we undertake."</p>
                        </div>
                    </div>
                </div>
            </section>

            // Latest News Section
            <section class="bg-gray-100 py-16">
                <div class="container mx-auto px-4">
                    <h2 class="text-3xl font-bold text-center mb-12">"Latest News"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="bg-white rounded-lg shadow-md overflow-hidden">
                            <div class="p-6">
                                <h3 class="text-xl font-bold mb-2">"New Game Announcement Coming Soon"</h3>
                                <p class="text-gray-600 mb-4">"Stay tuned for our exciting new project announcement!"</p>
                                <p class="text-sm text-gray-500">"March 15, 2024"</p>
                            </div>
                        </div>
                        <div class="bg-white rounded-lg shadow-md overflow-hidden">
                            <div class="p-6">
                                <h3 class="text-xl font-bold mb-2">"Join Our Team"</h3>
                                <p class="text-gray-600 mb-4">"We're looking for talented developers and artists."</p>
                                <p class="text-sm text-gray-500">"March 10, 2024"</p>
                            </div>
                        </div>
                        <div class="bg-white rounded-lg shadow-md overflow-hidden">
                            <div class="p-6">
                                <h3 class="text-xl font-bold mb-2">"Tech Blog Update"</h3>
                                <p class="text-gray-600 mb-4">"Read about our latest technological innovations."</p>
                                <p class="text-sm text-gray-500">"March 5, 2024"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
