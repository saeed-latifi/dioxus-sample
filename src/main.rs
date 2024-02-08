#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    let mut tasksList: Vec<Task> = vec![];

    let task_one = Task {
        id: 1,
        title: "task one",
    };
    let task_two = Task {
        id: 2,
        title: "task two",
    };

    let task_three = Task {
        id: 3,
        title: "task three",
    };

    tasksList.push(task_one);
    tasksList.push(task_two);
    tasksList.push(task_three);

    render! {
        div{class:"flex flex-col w-full bg-pink-200 gap-4 p-4 text-white",
            {tasksList.into_iter().map( |t| rsx!{ TaskCard { task:t } })}
        }
    }
}

#[derive(Props)]
struct Task<'a> {
    pub id: i64,
    pub title: &'a str,
}

#[component]
fn TaskCard<'a>(cx: Scope<'a>, task: Task<'a>) -> Element {
    render! {
        div { class:"flex items-center justify-between  gap-2 rounded bg-pink-600 p-2",
            span{"{task.title}"}
            span{"{task.id}"}
        }
    }
}
