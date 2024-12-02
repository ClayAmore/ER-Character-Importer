use std::sync::Mutex;

use er_character_importer_models_lib::{CharacterSummary, JsResult};
use er_save_lib::SaveApi;
use tauri::{AppHandle, Runtime, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Default)]
pub(crate) struct Save1 {
    pub(crate) save: Option<SaveApi>,
}

#[derive(Default)]
pub(crate) struct Save2 {
    pub(crate) save: Option<SaveApi>,
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn open<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, (Mutex<Save1>, Mutex<Save2>)>,
    index: usize,
) -> String {
    if let Some(path) = app
        .dialog()
        .file()
        .add_filter("PC Save", &["sl2"])
        .add_filter("Playstation", &["txt"])
        .add_filter("All files", &["*"])
        .blocking_pick_file()
    {
        let save_api = SaveApi::from_path(&path.to_string());
        if let Err(err) = save_api {
            return JsResult::error(err.to_string());
        }
        let save_api = save_api.unwrap();
        let characters_summary = &characters_summary(&save_api);
        match index {
            0 => {
                state.0.lock().unwrap().save = Some(save_api);
            }
            1 => {
                state.1.lock().unwrap().save = Some(save_api);
            }
            _ => {}
        }
        return JsResult::ok(characters_summary);
    }
    JsResult::ok(Vec::<CharacterSummary>::new())
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn save<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, (Mutex<Save1>, Mutex<Save2>)>,
    index: usize,
) -> Result<String, String> {
    if let Some(path) = app
        .dialog()
        .file()
        .add_filter("PC Save", &["sl2"])
        .add_filter("Playstation", &["txt"])
        .add_filter("All files", &["*"])
        .blocking_save_file()
    {
        match index {
            0 => {
                if let Err(err) = state
                    .0
                    .lock()
                    .unwrap()
                    .save
                    .as_ref()
                    .unwrap()
                    .write_to_path(&path.as_path().unwrap())
                {
                    return Err(err.to_string());
                }
                Ok(format!("File saved to: {}", path))
            }
            1 => {
                if let Err(err) = state
                    .1
                    .lock()
                    .unwrap()
                    .save
                    .as_ref()
                    .unwrap()
                    .write_to_path(&path.as_path().unwrap())
                {
                    return Err(err.to_string());
                }
                Ok(format!("File saved to: {}", path))
            }
            _ => Ok(format!("None")),
        }
    } else {
        Ok(format!("None"))
    }
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) fn transfer_character<R: Runtime>(
    _: AppHandle<R>,
    state: State<'_, (Mutex<Save1>, Mutex<Save2>)>,
    save1_to_save2: bool,
    save1_index: usize,
    save2_index: usize,
) -> String {
    if save1_to_save2 {
        state
            .1
            .lock()
            .as_mut()
            .unwrap()
            .save
            .as_mut()
            .unwrap()
            .import_character(
                save2_index,
                state.0.lock().as_ref().unwrap().save.as_ref().unwrap(),
                save1_index,
            )
            .unwrap();

        JsResult::ok(&characters_summary(
            state.1.lock().as_ref().unwrap().save.as_ref().unwrap(),
        ))
    } else {
        state
            .0
            .lock()
            .as_mut()
            .unwrap()
            .save
            .as_mut()
            .unwrap()
            .import_character(
                save1_index,
                state.1.lock().as_ref().unwrap().save.as_ref().unwrap(),
                save2_index,
            )
            .unwrap();

        JsResult::ok(&characters_summary(
            state.0.lock().as_ref().unwrap().save.as_ref().unwrap(),
        ))
    }
}

pub(crate) fn characters_summary(save_api: &SaveApi) -> Vec<CharacterSummary> {
    let mut characters = Vec::new();
    save_api
        .active_characters()
        .iter()
        .enumerate()
        .for_each(|(index, active)| {
            if *active {
                characters.push(CharacterSummary {
                    character_name: save_api.character_name(index),
                    location: "".to_string(),
                    level: save_api.level(index),
                    vigor: save_api.vigor(index),
                    mind: save_api.mind(index),
                    endurance: save_api.endurance(index),
                    strength: save_api.strength(index),
                    dexterity: save_api.dexterity(index),
                    intelligence: save_api.intelligence(index),
                    faith: save_api.faith(index),
                    arcane: save_api.arcane(index),
                });
            }
        });
    characters
}
