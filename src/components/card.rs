use er_character_importer_models_lib::CharacterSummary;
use yew::html::onclick::Event;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Card)]
pub fn card(character: &CharacterSummary, selected: bool, on_click: Callback<MouseEvent>) -> Html {
    let is_expanded = use_state(|| false);

    let on_expand = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |e: Event| {
            e.stop_propagation();
            is_expanded.set(!(*is_expanded.clone()))
        })
    };

    html! {
        <div onclick={on_click} class={format!(
            "card p-4 card-bordered mb-2 cursor-pointer transition  {}"
        , if selected {"border-accent border-2 font-medium"} else {"hover:border-base-content/40 hover:border-1 card-compact border-base-content/20"})}>
            <div class="flex justify-between items-end">
                <div class="flex flex-col select-none">
                    <p class="text-sm">{format!("Level: {}", character.level)}</p>
                    <p class="font text-xl">{character.character_name.clone()}</p>
                </div>
                <button onclick={on_expand} class="p-1 rounded transition hover:scale-150">
                    <svg class={format!("size-5 transition duration-250 ease-out {}", if *is_expanded {"rotate-180"} else {""})} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                    </svg>
                </button>
            </div>
            <div class={format!("overflow-y-hidden transition-[max-height] duration-250 ease-out {}", if *is_expanded {"max-h-screen"} else {"max-h-0"})}>
                <div class="divider"></div>
                <div class="flex flex-col">
                    <StatsRow label={"Vigor"} value={character.vigor.to_string()}  />
                    <StatsRow label={"Mind"} value={character.mind.to_string()} />
                    <StatsRow label={"Endurance"} value={character.endurance.to_string()}   />
                    <StatsRow label={"Strength"} value={character.strength.to_string()} />
                    <StatsRow label={"Dexterity"} value={character.dexterity.to_string()}   />
                    <StatsRow label={"Intelligence"} value={character.intelligence.to_string()} />
                    <StatsRow label={"Faith"} value={character.faith.to_string()}   />
                    <StatsRow label={"Arcane"} value={character.arcane.to_string()} />
                </div>
            </div>
        </div>
    }
}

#[autoprops]
#[function_component(StatsRow)]
fn stats_row(
    #[prop_or(AttrValue::Static("Label"))] label: &AttrValue,
    #[prop_or(AttrValue::Static("Value"))] value: &AttrValue,
) -> Html {
    html! {
        <div class={"flex justify-between py-2"}>
            <div>{label}</div>
            <div class="justify-self-end">{value}</div>
        </div>
    }
}

#[autoprops]
#[function_component(EmptyCard)]
pub fn empty_card(selected: bool, on_click: Callback<MouseEvent>, num: isize) -> Html {
    html! {
        <div onclick={on_click} class={format!(
            "card p-4 card-bordered border-dashed mb-2 cursor-pointer transition  {}"
        , if selected {"border-neutral border-2 font-medium"} else {"hover:border-neutral hover:border-1 card-compact"})}>
            <div class="flex justify-between items-end">
                <div class="flex flex-col select-none">
                    <p class="text-xs">{format!("Slot: {}", num)}</p>
                    <p class="font text-xl">{"Empty"}</p>
                </div>
            </div>
        </div>
    }
}
