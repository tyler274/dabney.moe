use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="min-h-screen py-16">
            <div class="container mx-auto px-4">
                // Company Overview Section
                <section class="mb-16">
                    <h1 class="text-4xl font-bold text-center mb-12">"About Dabney Engineering"</h1>
                    <div class="max-w-3xl mx-auto text-lg text-gray-700 leading-relaxed">
                        <p class="mb-6">
                            "Founded in 2024, Dabney Engineering is a cutting-edge game development studio focused on creating innovative and engaging gaming experiences. Our team combines technical expertise with creative vision to push the boundaries of what's possible in modern gaming."
                        </p>
                        <p class="mb-6">
                            "We specialize in developing games that not only entertain but also challenge conventional thinking. Our commitment to excellence and innovation drives everything we do, from concept to execution."
                        </p>
                    </div>
                </section>

                // Mission & Values Section
                <section class="mb-16 bg-gray-100 py-12 rounded-lg">
                    <div class="container mx-auto px-4">
                        <h2 class="text-3xl font-bold text-center mb-8">"Our Mission & Values"</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
                            <div class="bg-white p-6 rounded-lg shadow-md">
                                <h3 class="text-xl font-bold mb-4">"Mission"</h3>
                                <p class="text-gray-700">
                                    "To create exceptional gaming experiences that inspire creativity and bring joy to players worldwide."
                                </p>
                            </div>
                            <div class="bg-white p-6 rounded-lg shadow-md">
                                <h3 class="text-xl font-bold mb-4">"Values"</h3>
                                <ul class="list-disc list-inside text-gray-700">
                                    <li>"Innovation in Technology"</li>
                                    <li>"Player-Centric Design"</li>
                                    <li>"Quality Excellence"</li>
                                    <li>"Continuous Learning"</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </section>

                // Team Section
                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-center mb-12">"Our Team"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <div class="text-center">
                            <div class="w-32 h-32 bg-gray-300 rounded-full mx-auto mb-4"></div>
                            <h3 class="text-xl font-bold">"John Doe"</h3>
                            <p class="text-gray-600">"Founder & CEO"</p>
                        </div>
                        <div class="text-center">
                            <div class="w-32 h-32 bg-gray-300 rounded-full mx-auto mb-4"></div>
                            <h3 class="text-xl font-bold">"Jane Smith"</h3>
                            <p class="text-gray-600">"Lead Game Designer"</p>
                        </div>
                        <div class="text-center">
                            <div class="w-32 h-32 bg-gray-300 rounded-full mx-auto mb-4"></div>
                            <h3 class="text-xl font-bold">"Mike Johnson"</h3>
                            <p class="text-gray-600">"Technical Director"</p>
                        </div>
                    </div>
                </section>

                // Contact Section
                <section class="bg-gray-900 text-white py-12 rounded-lg">
                    <div class="text-center">
                        <h2 class="text-3xl font-bold mb-6">"Get in Touch"</h2>
                        <p class="mb-8">"Interested in learning more about Dabney Engineering or want to join our team?"</p>
                        <a href="mailto:contact@dabney.engineering" class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-lg transition duration-300">
                            "Contact Us"
                        </a>
                    </div>
                </section>
            </div>
        </div>
    }
}
