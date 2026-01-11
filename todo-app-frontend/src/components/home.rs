use crate::{api, components::hero_image::HeroImage, core::models::{Todo, CreateTodo}, core::utils::linkify};
use leptos::task::spawn_local;
use leptos::prelude::*;

#[component]
pub fn Home(hero_image_url: String) -> impl IntoView {
    let (title, title_writer) = signal(String::new());
    let (description, description_writer) = signal(String::new());
    let (tasks, tasks_writer) = signal(Vec::<Todo>::new());
    let (error, error_writer) = signal(Option::<String>::None);
    let (loading, loading_writer) = signal(false);

    // Fetch todos on mount
    Effect::new(move |_| {
        spawn_local(async move {
            loading_writer.set(true);
            match api::fetch_todos().await {
                Ok(todos) => {
                    tasks_writer.set(todos);
                    error_writer.set(None);
                }
                Err(e) => {
                    error_writer.set(Some(format!("Failed to load todos: {}", e)));
                }
            }
            loading_writer.set(false);
        });
    });

    let on_add_todo = move |_| {
        let title_val = title.get();
        let description_val = description.get();

        if title_val.trim().is_empty() {
            error_writer.set(Some("Title cannot be empty".to_string()));
            return;
        }

        spawn_local(async move {
            loading_writer.set(true);
            let new_todo = CreateTodo {
                title: title_val.clone(),
                description: description_val.clone(),
            };

            match api::create_todo(new_todo).await {
                Ok(_) => {
                    // Refresh the todo list after creation
                    match api::fetch_todos().await {
                        Ok(todos) => {
                            tasks_writer.set(todos);
                            title_writer.set(String::new());
                            description_writer.set(String::new());
                            error_writer.set(None);
                        }
                        Err(e) => {
                            error_writer.set(Some(format!("Failed to refresh todos: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    error_writer.set(Some(format!("Failed to create todo: {}", e)));
                }
            }
            loading_writer.set(false);
        });
    };

    view! {
        <div>
            <HeroImage image_url=hero_image_url />
            <div>List of Todos</div>

            <Show when=move || error.get().is_some()>
                <div style="color: red; padding: 10px;">
                    {move || error.get().unwrap_or_default()}
                </div>
            </Show>

            <Show when=move || loading.get()>
                <div style="padding: 10px;">Loading...</div>
            </Show>

            <div>
                <input
                    type="text"
                    placeholder="Title"
                    prop:value=move || title.get()
                    on:input=move |event| {
                        title_writer.set(event_target_value(&event));
                    }
                />
            </div>
            <div>
                <textarea
                    placeholder="Description"
                    prop:value=move || description.get()
                    on:input=move |event| {
                        description_writer.set(event_target_value(&event));
                    }
                />
            </div>
            <button
                on:click=on_add_todo
                prop:disabled=move || loading.get()
            >
                Add Todo
            </button>
        </div>
        <For
            each=move || tasks.get()
            key=|task| task.id.clone()
            children=move |task| {
                let title_content = linkify(&task.title);
                let description_content = linkify(&task.description);
                view! {
                    <div style="border: 1px solid #ccc; margin: 10px; padding: 10px;">
                        <div><strong>{title_content}</strong></div>
                        <p>{description_content}</p>
                    </div>
                }
            }
        />
    }
}
