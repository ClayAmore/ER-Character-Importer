use er_character_importer_models_lib::{CharacterSummary, JsResult};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use strsim::sorensen_dice;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::app::{invoke, Save};
use crate::components::Card;

#[allow(unused)]
enum Alert {
    Error(String),
    Success(String),
    Warning(String),
    Loading(String),
}

#[derive(Serialize, Deserialize)]
struct OpenSaveArgs {
    index: usize,
}

#[autoprops]
#[function_component(List)]
pub fn list(save: &UseStateHandle<Save>) -> Html {
    let filter_value = use_state(|| String::new());
    let alert: UseStateHandle<Option<Alert>> = use_state(|| None);

    let on_open_clicked = {
        let save = save.clone();
        let alert = alert.clone();
        Callback::from(move |_| {
            let save = save.clone();
            let alert = alert.clone();
            spawn_local(async move {
                if let Some(result) =
                    invoke("open", to_value(&OpenSaveArgs { index: save.id }).unwrap())
                        .await
                        .as_string()
                {
                    let result: JsResult = serde_json::from_str(&result).unwrap();
                    match result {
                        JsResult::Ok(characters_json) => {
                            let new_characters: Vec<CharacterSummary> =
                                serde_json::from_str(&characters_json).unwrap();
                            if !new_characters.is_empty() {
                                let mut new_value = (*save).clone();
                                new_value.characters = new_characters;
                                new_value.selected_index = None;
                                save.set(new_value);
                            }
                        }
                        JsResult::Err(err) => alert.set(Some(Alert::Error(err))),
                    }
                }
            });
        })
    };

    let on_save_clicked = {
        let alert = alert.clone();
        let index = save.id;
        Callback::from(move |_| {
            let alert = alert.clone();
            alert.set(Some(Alert::Loading(format!("Saving..."))));
            spawn_local(async move {
                if let Some(result) = invoke("save", to_value(&OpenSaveArgs { index }).unwrap())
                    .await
                    .as_string()
                {
                    if result != "None" {
                        alert.set(Some(Alert::Success(result)));
                    } else {
                        alert.set(None);
                    }
                }
            });
        })
    };

    let onchange = {
        let filter_value = filter_value.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(chr) = input {
                filter_value.set(chr.value().to_string());
            }
        })
    };

    html! {
        <div class="p-6 card card-bordered border-base-content/20 overflow-hidden flex flex-col">
            <div class="grid grid-cols-2 gap-2 mb-2">
                <button onclick={on_open_clicked} class="btn">
                    <svg  class="size-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 0 0-1.883 2.542l.857 6a2.25 2.25 0 0 0 2.227 1.932H19.05a2.25 2.25 0 0 0 2.227-1.932l.857-6a2.25 2.25 0 0 0-1.883-2.542m-16.5 0V6A2.25 2.25 0 0 1 6 3.75h3.879a1.5 1.5 0 0 1 1.06.44l2.122 2.12a1.5 1.5 0 0 0 1.06.44H18A2.25 2.25 0 0 1 20.25 9v.776" />
                    </svg>
                    {"Open"}
                </button>
                <button onclick={on_save_clicked} class={format!("btn {}", if save.characters.is_empty() {"btn-disabled"} else {""})}>
                    <svg  class="size-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
                        <path d="M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"/>
                        <path d="M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"/>
                        <path d="M7 3v4a1 1 0 0 0 1 1h7"/>
                    </svg>
                    {"Save"}
                </button>
            </div>

            {
                if alert.as_ref().is_some_and(|alert| {
                    match alert {
                        Alert::Error(text) => text != "null",
                        Alert::Success(text) => text != "null",
                        Alert::Warning(text) => text != "null",
                        Alert::Loading(text) => text != "null",
                    }
                }) {
                    match alert.as_ref().unwrap() {
                        Alert::Error(text) =>
                            html!(
                                <div role="alert" class="alert alert-error mb-2">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-6 w-6 shrink-0 stroke-current"
                                        fill="none"
                                        viewBox="0 0 24 24">
                                        <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                    <span class="break-all">{text}</span>
                                    <div>
                                    <button onclick={
                                        let alert = alert.clone();
                                        Callback::from(
                                            move |_| {
                                                alert.set(None);
                                            }
                                        )
                                    } class="btn btn-sm">{"Close"}</button>
                                </div>
                                </div>
                            ),
                        Alert::Success(text) =>
                            html!(
                                <div role="alert" class="alert alert-success mb-2">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-6 w-6 shrink-0 stroke-current"
                                        fill="none"
                                        viewBox="0 0 24 24">
                                        <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                    <span class="break-all">{text}</span>
                                    <div>
                                    <button onclick={
                                        let alert = alert.clone();
                                        Callback::from(
                                            move |_| {
                                                alert.set(None);
                                            }
                                        )
                                    } class="btn btn-sm">{"Close"}</button>
                                </div>
                                </div>
                            ),
                        Alert::Warning(text) =>
                            html!(
                                <div role="alert" class="alert alert-warning mb-2">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-6 w-6 shrink-0 stroke-current"
                                        fill="none"
                                        viewBox="0 0 24 24">
                                        <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                    </svg>
                                    <span class="word-wrap">{text}</span>
                                    <div>
                                    <button onclick={
                                        let alert = alert.clone();
                                        Callback::from(
                                            move |_| {
                                                alert.set(None);
                                            }
                                        )
                                    } class="btn btn-sm">{"Close"}</button>
                                </div>
                                </div>
                            ),
                        Alert::Loading(text) =>
                            html!(
                                <div role="alert" class="alert alert-info mb-2">
                                    <span class="loading loading-spinner loading-xs"></span>
                                    <span class="word-wrap">{text}</span>
                                    <div>
                                </div>
                                </div>
                            ),
                    }
                }
                else {
                    html!()
                }
            }

            <label class="input input-bordered flex items-center gap-2 mb-2 shrink-0">
                <input value={(*filter_value).clone()} oninput={onchange} type="text" class="grow" placeholder="Filter by character name" />
                <svg  class="size-5 text-slate-300" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 3c2.755 0 5.455.232 8.083.678.533.09.917.556.917 1.096v1.044a2.25 2.25 0 0 1-.659 1.591l-5.432 5.432a2.25 2.25 0 0 0-.659 1.591v2.927a2.25 2.25 0 0 1-1.244 2.013L9.75 21v-6.568a2.25 2.25 0 0 0-.659-1.591L3.659 7.409A2.25 2.25 0 0 1 3 5.818V4.774c0-.54.384-1.006.917-1.096A48.32 48.32 0 0 1 12 3Z" />
                </svg>
            </label>

            <div class="overflow-y-auto grow">
            {
                if save.characters.len() > 0 {
                    save.characters.iter().filter(|character|{
                        if filter_value.is_empty() {
                            return true;
                        }
                        let distance = sorensen_dice(
                            &character.character_name.clone().to_lowercase(),
                            &(*filter_value).to_lowercase(),
                        );
                        distance > 0.3
                    }).enumerate().map(|(i, character)| {
                        html!(<Card character={character.clone()} selected={save.selected_index == Some(i)} on_click={
                            if save.selected_index == Some(i) {
                                let save = save.clone();
                                Callback::from(move |_|{
                                    let mut new_value = (*save).clone();
                                    new_value.selected_index = None;
                                    save.set(new_value)
                                })
                            }
                            else {
                                let save = save.clone();
                                Callback::from(move |_| {
                                    let mut new_value = (*save).clone();
                                    new_value.selected_index = Some(i);
                                    save.set(new_value)
                                })
                            }
                        } />)
                    }).collect()
                }
                else {
                    html!(
                        <div class="card card-bordered border-base-content/20 p-2 pt-6 flex flex-col items-center h-full">
                            <div class="card-body items-center">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H6.911a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661Z" />
                                </svg>
                                <p>{"No file loaded"}</p>
                            </div>
                        </div>
                    )
                }
            }
        </div>
        </div>
    }
}
