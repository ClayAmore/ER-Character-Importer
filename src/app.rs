use er_character_importer_models_lib::{CharacterSummary, JsResult};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{List, Toolbar};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub(crate) async fn invoke_without_args(cmd: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub(crate) async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct TransferArgs {
    save1_to_save2: bool,
    save1_index: usize,
    save2_index: usize,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct Save {
    pub(crate) id: usize,
    pub(crate) filter_value: String,
    pub(crate) characters: Vec<CharacterSummary>,
    pub(crate) selected_index: Option<usize>,
}

impl Save {
    pub(crate) fn new(id: usize) -> Self {
        let mut save = Self::default();
        save.id = id;
        save
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let is_dark = use_state(|| false);
    let save1 = use_state(|| Save::new(0));
    let save2 = use_state(|| Save::new(1));

    let transfer_right = {
        let save1 = save1.clone();
        let save2 = save2.clone();
        Callback::from(move |_| {
            let save1 = save1.clone();
            let save2 = save2.clone();
            spawn_local(async move {
                if let Some(result) = invoke(
                    "transfer_character",
                    to_value(&TransferArgs {
                        save1_to_save2: true,
                        save1_index: save1.selected_index.unwrap(),
                        save2_index: save2.selected_index.unwrap(),
                    })
                    .unwrap(),
                )
                .await
                .as_string()
                {
                    let result: JsResult = serde_json::from_str(&result).unwrap();
                    match result {
                        JsResult::Ok(characters_json) => {
                            let new_characters: Vec<CharacterSummary> =
                                serde_json::from_str(&characters_json).unwrap();
                            if !new_characters.is_empty() {
                                let mut new_value = (*save2).clone();
                                new_value.characters = new_characters;
                                save2.set(new_value);
                            }
                        }
                        JsResult::Err(_) => {}
                    }
                }
            });
        })
    };

    let transfer_left = {
        let save1 = save1.clone();
        let save2 = save2.clone();
        Callback::from(move |_| {
            let save1 = save1.clone();
            let save2 = save2.clone();
            spawn_local(async move {
                let save1 = save1.clone();
                let save2 = save2.clone();
                if let Some(result) = invoke(
                    "transfer_character",
                    to_value(&TransferArgs {
                        save1_to_save2: false,
                        save1_index: save1.selected_index.unwrap(),
                        save2_index: save2.selected_index.unwrap(),
                    })
                    .unwrap(),
                )
                .await
                .as_string()
                {
                    let result: JsResult = serde_json::from_str(&result).unwrap();
                    match result {
                        JsResult::Ok(characters_json) => {
                            let new_characters: Vec<CharacterSummary> =
                                serde_json::from_str(&characters_json).unwrap();
                            if !new_characters.is_empty() {
                                let mut new_value = (*save1).clone();
                                new_value.characters = new_characters;
                                save1.set(new_value);
                            }
                        }
                        JsResult::Err(_) => {}
                    }
                }
            });
        })
    };

    html! {
        <main class="w-screen h-screen overflow-hidden flex" data-theme={
        if *is_dark {
            "business"
        } else {
            "corporate"
        }}>
            <Toolbar is_dark={is_dark}/>
            <div class="container mx-auto place-items-start pt-20 pb-8 self-stretch grid grid-cols-[5fr_1fr_5fr] gap-4 place-items-stretch">
                <List save={save1.clone()} />
                <div class="flex flex-col justify-center items-center">
                    <button onclick={transfer_right} class={format!("btn mb-1 {}", if save1.selected_index == None || save2.selected_index == None {"btn-disabled"} else {""})}>
                        <svg class={format!("size-6 transition duration-250 ease-out")} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3" />
                        </svg>
                    </button>
                    <button onclick={transfer_left} class={format!("btn mt-1 {}", if save1.selected_index == None || save2.selected_index == None {"btn-disabled"} else {""})}>
                        <svg class={format!("size-6 transition duration-250 ease-out")} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" />
                        </svg>
                    </button>
                </div>
                <List save={save2.clone()}  />
            </div>
        </main>
    }
}
