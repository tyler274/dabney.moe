use leptos::prelude::*;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => RwSignal::new(Errors::default()),
        },
    };

    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold text-red-600 mb-4">"Error"</h1>
            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                {move || {
                    errors.with(|errors| {
                        errors
                            .iter()
                            .map(|(_, e)| view! { <p>{e.to_string()}</p> })
                            .collect_view()
                    })
                }}
            </div>
        </div>
    }
}
