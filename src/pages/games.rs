use leptos::*;
use leptos::prelude::*;

#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <div class="min-h-screen py-16">
            <div class="container mx-auto px-4">
                <h1 class="text-4xl font-bold text-center mb-12">"Our Games"</h1>

                // Featured Game Section
                <section class="mb-16">
                    <div class="bg-gray-900 text-white rounded-lg overflow-hidden">
                        <div class="grid grid-cols-1 md:grid-cols-2">
                            <div class="bg-gray-800 h-96"></div>
                            <div class="p-8">
                                <h2 class="text-3xl font-bold mb-4">"Coming Soon: Project Nova"</h2>
                                <p class="text-gray-300 mb-6">
                                    "An innovative space exploration game that combines strategy, resource management, and epic space battles. Be among the first to colonize the galaxy!"
                                </p>
                                <div class="mb-6">
                                    <h3 class="text-xl font-bold mb-2">"Key Features:"</h3>
                                    <ul class="list-disc list-inside text-gray-300">
                                        <li>"Procedurally generated galaxies"</li>
                                        <li>"Advanced AI civilizations"</li>
                                        <li>"Real-time space combat"</li>
                                        <li>"Complex economy system"</li>
                                    </ul>
                                </div>
                                <button class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300">
                                    "Join the Waitlist"
                                </button>
                            </div>
                        </div>
                    </div>
                </section>

                // Games in Development
                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-center mb-8">"Games in Development"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="bg-white rounded-lg shadow-lg overflow-hidden">
                            <div class="bg-gray-200 h-48"></div>
                            <div class="p-6">
                                <h3 class="text-xl font-bold mb-2">"Quantum Breach"</h3>
                                <p class="text-gray-600 mb-4">
                                    "A cyberpunk thriller where reality meets virtual worlds. Hack, fight, and survive in a dystopian future."
                                </p>
                                <span class="inline-block bg-yellow-100 text-yellow-800 px-3 py-1 rounded-full text-sm">
                                    "In Development"
                                </span>
                            </div>
                        </div>

                        <div class="bg-white rounded-lg shadow-lg overflow-hidden">
                            <div class="bg-gray-200 h-48"></div>
                            <div class="p-6">
                                <h3 class="text-xl font-bold mb-2">"Forest Keeper"</h3>
                                <p class="text-gray-600 mb-4">
                                    "An enchanting adventure game where you protect magical creatures and solve environmental puzzles."
                                </p>
                                <span class="inline-block bg-yellow-100 text-yellow-800 px-3 py-1 rounded-full text-sm">
                                    "Concept Phase"
                                </span>
                            </div>
                        </div>

                        <div class="bg-white rounded-lg shadow-lg overflow-hidden">
                            <div class="bg-gray-200 h-48"></div>
                            <div class="p-6">
                                <h3 class="text-xl font-bold mb-2">"Mech Warriors: Uprising"</h3>
                                <p class="text-gray-600 mb-4">
                                    "Command powerful mechs in intense tactical battles across diverse landscapes."
                                </p>
                                <span class="inline-block bg-yellow-100 text-yellow-800 px-3 py-1 rounded-full text-sm">
                                    "Early Development"
                                </span>
                            </div>
                        </div>
                    </div>
                </section>

                // Newsletter Signup
                <section class="bg-gray-100 rounded-lg p-8 text-center">
                    <h2 class="text-2xl font-bold mb-4">"Stay Updated"</h2>
                    <p class="text-gray-600 mb-6">"Subscribe to our newsletter for the latest updates on our games and exclusive content."</p>
                    <div class="max-w-md mx-auto flex gap-4">
                        <input
                            type="email"
                            placeholder="Enter your email"
                            class="flex-1 px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:border-blue-500"
                        />
                        <button class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg transition duration-300">
                            "Subscribe"
                        </button>
                    </div>
                </section>
            </div>
        </div>
    }
}
