use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    let (task, task_writer) = signal(Item::default());
    let (tasks, tasks_writer) = signal(Vec::<Item>::new());
    let (counter, counter_writer) = signal(0u32);

    view! {
        <main>
            <div>
                <div>List of Items</div>
                <div>
                <input type="text"  on:input= move |event| {
                    task_writer.write().title = event_target_value(&event);
                }/>
                </div>
                <div>
                <textarea on:input= move |event| {
                    task_writer.write().description = event_target_value(&event);
                }></textarea>
                </div>
                <button on:click=move |_| {
                    tasks_writer.write().push(Item {
                        id: counter.get(),
                        title:  task.get().title,
                        description: task.get().description,
                    });
                    counter_writer.set(counter.get() +1);
                }>Add Item</button>
            </div>
            <For
                each = move || tasks.get()
                key = |task| task.id
                children = move |task| {
                    view! {
                        <div>{task.title}</div>
                        <p>{task.description}</p>
                    }
                }
            />
        </main>
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct Item {
    id: u32,
    title: String,
    description: String,
}
