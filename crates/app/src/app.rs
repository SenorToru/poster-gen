use leptos::{ev, html::Input, prelude::*};
use poster_gen_core::{remix_gate, PeopleConfig, ProviderId, RemixGate, RemixParams};
use wasm_bindgen_futures::spawn_local;

use crate::api::{portrait_request, post_job, remix_request};
use crate::copy::{
    aspect_label, copy, Choice, Language, ASPECTS, ETHNICITIES, EXPRESSIONS, GENDERS, HAIR,
    RESOLUTIONS, SKIN, STYLES,
};
use crate::image::compress_image;
use crate::store::{forget_credential, load_credential, save_credential, Credential};

const SELECT: &str = "w-full rounded-sm border border-border bg-card px-3 py-2 text-sm text-foreground transition-colors focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary";

#[component]
pub fn App() -> impl IntoView {
    let language = RwSignal::new(Language::Zh);
    let provider = RwSignal::new(ProviderId::Grok);
    let grok = RwSignal::new(load_credential(ProviderId::Grok));
    let gemini = RwSignal::new(load_credential(ProviderId::Gemini));
    let show_key = RwSignal::new(false);

    let base = RwSignal::new(None::<String>);
    let product = RwSignal::new(None::<String>);
    let people = RwSignal::new(None::<String>);
    let people_mode = RwSignal::new(PeopleMode::Upload);
    let people_config = RwSignal::new(PeopleConfig {
        gender: "Female".into(),
        hair_color: "Brown".into(),
        skin_tone: "Medium".into(),
        ethnicity: "Mixed".into(),
        expression: "Neutral".into(),
    });
    let model_preview = RwSignal::new(None::<String>);
    let generating_model = RwSignal::new(false);

    let aspect = RwSignal::new("1:1".to_string());
    let resolution = RwSignal::new("1K".to_string());
    let style_id = RwSignal::new(STYLES[0].id.to_string());
    let custom = RwSignal::new(String::new());
    let generated = RwSignal::new(None::<String>);
    let generating = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("lang", language.get().html_lang());
        }
    });

    let slot = move || match provider.get() {
        ProviderId::Grok => grok,
        ProviderId::Gemini => gemini,
    };

    let persist = move || {
        let provider = provider.get_untracked();
        save_credential(provider, &slot().get_untracked());
    };

    let remix_disabled = move || {
        generating.get()
            || remix_gate(
                base.get().is_some(),
                product.get().is_some(),
                people.get().is_some(),
            ) != RemixGate::Ready
    };

    let on_remix = move |_| {
        let lang = language.get_untracked();
        let text = copy(lang);
        if slot().get_untracked().api_key.trim().is_empty() {
            error.set(Some(text.missing_key.to_string()));
            return;
        }
        let gate = remix_gate(
            base.get_untracked().is_some(),
            product.get_untracked().is_some(),
            people.get_untracked().is_some(),
        );
        let message = match gate {
            RemixGate::Ready => None,
            RemixGate::MissingBase => Some(text.missing_base),
            RemixGate::MissingReference => Some(text.missing_reference),
        };
        if let Some(message) = message {
            error.set(Some(message.to_string()));
            return;
        }
        let style = STYLES
            .iter()
            .find(|style| style.id == style_id.get_untracked())
            .unwrap_or(&STYLES[0]);
        let request = remix_request(
            provider.get_untracked(),
            slot().get_untracked().model_id.clone(),
            RemixParams {
                base_image: base.get_untracked().unwrap_or_default(),
                product_image: product.get_untracked(),
                people_image: people.get_untracked(),
                style_description: style.prompt.to_string(),
                custom_prompt: custom.get_untracked(),
                aspect_ratio: aspect.get_untracked(),
                resolution: resolution.get_untracked(),
            },
        );
        let key = slot().get_untracked().api_key.clone();
        generating.set(true);
        error.set(None);
        generated.set(None);
        spawn_local(async move {
            match post_job("/api/remix", key.trim(), &request).await {
                Ok(image) => generated.set(Some(image)),
                Err(message) => error.set(Some(message)),
            }
            generating.set(false);
        });
    };

    let on_model = move |_| {
        let text = copy(language.get_untracked());
        if slot().get_untracked().api_key.trim().is_empty() {
            error.set(Some(text.missing_key.to_string()));
            return;
        }
        let request = portrait_request(
            provider.get_untracked(),
            slot().get_untracked().model_id.clone(),
            people_config.get_untracked(),
        );
        let key = slot().get_untracked().api_key.clone();
        generating_model.set(true);
        error.set(None);
        spawn_local(async move {
            match post_job("/api/generate-model", key.trim(), &request).await {
                Ok(image) => {
                    model_preview.set(Some(image.clone()));
                    people.set(Some(image));
                }
                Err(message) => error.set(Some(message)),
            }
            generating_model.set(false);
        });
    };

    let switch_mode = move |mode: PeopleMode| {
        people_mode.set(mode);
        people.set(None);
        model_preview.set(None);
    };

    view! {
        <div class="min-h-screen bg-background text-foreground">
            <header class="sticky top-0 z-10 border-b border-border/70 bg-background/85 backdrop-blur">
                <div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-5 lg:px-10">
                    <div class="flex items-center gap-4">
                        <div class="flex size-10 items-center justify-center rounded-sm bg-primary text-primary-foreground">
                            <span class="font-serif text-xl leading-none">"P"</span>
                        </div>
                        <div class="flex flex-col">
                            <span class="font-serif text-lg leading-tight text-foreground">"PosterGen"</span>
                            <span class="hidden text-xs tracking-wide text-muted-foreground sm:block">
                                {move || copy(language.get()).subtitle}
                            </span>
                        </div>
                    </div>
                    <button
                        type="button"
                        class="rounded-sm border border-border bg-card px-3 py-1.5 text-sm text-secondary-foreground transition-colors hover:bg-accent"
                        on:click=move |_| language.update(|lang| *lang = lang.cycle())
                    >
                        {move || language.get().cycle().name()}
                    </button>
                </div>
            </header>

            <main class="mx-auto max-w-6xl px-6 py-10 lg:px-10 lg:py-12">
                <div class="grid grid-cols-1 gap-x-12 gap-y-12 lg:grid-cols-2">
                    <section class="flex flex-col gap-8">
                        <SectionMark index="01" title=Signal::derive(move || copy(language.get()).sources_title.to_string()) hint=Signal::derive(move || copy(language.get()).sources_hint.to_string()) />
                        <Uploader label=Signal::derive(move || copy(language.get()).upload_base.to_string()) image=base required=true language />
                        <Uploader label=Signal::derive(move || copy(language.get()).upload_product.to_string()) image=product required=false language />

                        <div class="flex flex-col gap-3">
                            <label class="text-sm font-medium text-foreground">{move || copy(language.get()).upload_people}</label>
                            <div class="grid grid-cols-2 gap-1 rounded-sm border border-border bg-secondary/50 p-1">
                                <button
                                    type="button"
                                    class=move || mode_tab(people_mode.get() == PeopleMode::Upload)
                                    on:click=move |_| switch_mode(PeopleMode::Upload)
                                >
                                    {move || copy(language.get()).mode_upload}
                                </button>
                                <button
                                    type="button"
                                    class=move || mode_tab(people_mode.get() == PeopleMode::Generate)
                                    on:click=move |_| switch_mode(PeopleMode::Generate)
                                >
                                    {move || copy(language.get()).mode_generate}
                                </button>
                            </div>
                            {move || if people_mode.get() == PeopleMode::Upload {
                                view! { <Uploader label=Signal::derive(|| String::new()) image=people required=false language /> }.into_any()
                            } else {
                                view! {
                                    <div class="flex flex-col gap-5 rounded-sm border border-border bg-card/60 p-5">
                                        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                                            <Field label=Signal::derive(move || copy(language.get()).gender.to_string())>
                                                <ChoiceSelect choices=GENDERS language value=Signal::derive(move || people_config.get().gender) on_change=move |value| people_config.update(|cfg| cfg.gender = value) />
                                            </Field>
                                            <Field label=Signal::derive(move || copy(language.get()).ethnicity.to_string())>
                                                <ChoiceSelect choices=ETHNICITIES language value=Signal::derive(move || people_config.get().ethnicity) on_change=move |value| people_config.update(|cfg| cfg.ethnicity = value) />
                                            </Field>
                                            <Field label=Signal::derive(move || copy(language.get()).skin_tone.to_string())>
                                                <ChoiceSelect choices=SKIN language value=Signal::derive(move || people_config.get().skin_tone) on_change=move |value| people_config.update(|cfg| cfg.skin_tone = value) />
                                            </Field>
                                            <Field label=Signal::derive(move || copy(language.get()).hair_color.to_string())>
                                                <ChoiceSelect choices=HAIR language value=Signal::derive(move || people_config.get().hair_color) on_change=move |value| people_config.update(|cfg| cfg.hair_color = value) />
                                            </Field>
                                            <Field label=Signal::derive(move || copy(language.get()).expression.to_string())>
                                                <ChoiceSelect choices=EXPRESSIONS language value=Signal::derive(move || people_config.get().expression) on_change=move |value| people_config.update(|cfg| cfg.expression = value) />
                                            </Field>
                                        </div>
                                        {move || model_preview.get().map(|src| {
                                            let href = src.clone();
                                            view! {
                                                <div class="relative overflow-hidden rounded-sm border border-border bg-secondary/40">
                                                    <div class="absolute left-2 top-2 z-10 rounded-sm bg-foreground/80 px-2 py-0.5 text-[10px] text-background backdrop-blur-sm">
                                                        {move || copy(language.get()).model_preview}
                                                    </div>
                                                    <img src=src alt="Generated model" class="mx-auto max-h-64 w-full object-contain p-2" />
                                                    <a
                                                        href=href
                                                        download=stamp("poster-gen-model")
                                                        class="absolute bottom-2 right-2 z-10 rounded-full bg-card/90 p-2 text-foreground shadow-md"
                                                        title=move || copy(language.get()).save_model
                                                    >
                                                        "↓"
                                                    </a>
                                                </div>
                                            }
                                        })}
                                        <button
                                            type="button"
                                            class="inline-flex items-center justify-center gap-2 rounded-sm border border-primary/40 bg-accent/60 px-4 py-2.5 text-sm font-medium text-primary transition-colors hover:bg-accent disabled:opacity-60"
                                            disabled=move || generating_model.get()
                                            on:click=on_model
                                        >
                                            {move || if generating_model.get() { copy(language.get()).generating_model } else { copy(language.get()).generate_model }}
                                        </button>
                                    </div>
                                }.into_any()
                            }}
                            <p class="text-xs leading-relaxed text-muted-foreground">{move || copy(language.get()).replacement_hint}</p>
                        </div>
                    </section>

                    <section class="flex flex-col gap-8">
                        <SectionMark index="02" title=Signal::derive(move || copy(language.get()).config_title.to_string()) hint=Signal::derive(move || copy(language.get()).config_hint.to_string()) />

                        <div class="flex flex-col gap-2">
                            <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).provider}</label>
                            <select class=SELECT prop:value=move || provider.get().as_str() on:change=move |ev| {
                                if let Some(next) = parse_provider(&event_target_value(&ev)) {
                                    provider.set(next);
                                }
                            }>
                                <option value="grok">{ProviderId::Grok.label()}</option>
                                <option value="gemini">{ProviderId::Gemini.label()}</option>
                            </select>
                        </div>

                        <div class="flex flex-col gap-2">
                            <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).api_key}</label>
                            <div class="flex gap-2">
                                <input
                                    class=SELECT
                                    type=move || if show_key.get() { "text" } else { "password" }
                                    prop:value=move || slot().get().api_key
                                    autocomplete="off"
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev);
                                        slot().update(|cred| cred.api_key = value);
                                        persist();
                                    }
                                />
                                <button type="button" class="shrink-0 rounded-sm border border-border bg-card px-3 text-sm" on:click=move |_| show_key.update(|v| *v = !*v)>
                                    {move || if show_key.get() { copy(language.get()).hide_key } else { copy(language.get()).show_key }}
                                </button>
                            </div>
                            <div class="flex items-center justify-between gap-3">
                                <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).model_id}</label>
                                <button
                                    type="button"
                                    class="text-xs text-primary"
                                    on:click=move |_| {
                                        let provider = provider.get();
                                        match provider {
                                            ProviderId::Grok => grok.set(Credential::fresh(provider)),
                                            ProviderId::Gemini => gemini.set(Credential::fresh(provider)),
                                        }
                                        forget_credential(provider);
                                    }
                                >
                                    {move || copy(language.get()).forget_key}
                                </button>
                            </div>
                            <input
                                class=SELECT
                                prop:value=move || slot().get().model_id
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    slot().update(|cred| cred.model_id = value);
                                    persist();
                                }
                            />
                        </div>

                        <div class="flex flex-col gap-3">
                            <span class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).style}</span>
                            <div class="grid grid-cols-2 gap-2">
                                {STYLES.iter().copied().map(|style| {
                                    view! {
                                        <button
                                            type="button"
                                            class=move || style_tab(style_id.get() == style.id)
                                            on:click=move |_| style_id.set(style.id.to_string())
                                        >
                                            {move || style.label(language.get())}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div class="flex flex-col gap-2">
                                <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).aspect_ratio}</label>
                                <select class=SELECT prop:value=move || aspect.get() on:change=move |ev| aspect.set(event_target_value(&ev))>
                                    {ASPECTS.iter().map(|value| view! { <option value=*value>{aspect_label(value)}</option> }).collect_view()}
                                </select>
                            </div>
                            <div class="flex flex-col gap-2">
                                <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).resolution}</label>
                                <select class=SELECT prop:value=move || resolution.get() on:change=move |ev| resolution.set(event_target_value(&ev))>
                                    {RESOLUTIONS.iter().map(|value| view! { <option value=*value>{*value}</option> }).collect_view()}
                                </select>
                            </div>
                        </div>
                        <p class="-mt-4 font-serif text-sm italic text-muted-foreground">{move || copy(language.get()).resolution_info}</p>

                        <div class="flex flex-col gap-2">
                            <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || copy(language.get()).custom_instruction}</label>
                            <textarea
                                class="min-h-28 w-full resize-none rounded-sm border border-border bg-card px-3 py-2 text-sm text-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
                                rows="3"
                                prop:value=move || custom.get()
                                placeholder=move || copy(language.get()).custom_placeholder
                                on:input=move |ev| custom.set(event_target_value(&ev))
                            />
                        </div>

                        <button
                            type="button"
                            class=move || if remix_disabled() {
                                "inline-flex cursor-not-allowed items-center justify-center gap-2 rounded-sm bg-muted px-6 py-4 text-sm font-medium tracking-wide text-muted-foreground"
                            } else {
                                "inline-flex items-center justify-center gap-2 rounded-sm bg-primary px-6 py-4 text-sm font-medium tracking-wide text-primary-foreground hover:bg-primary/90"
                            }
                            disabled=move || remix_disabled()
                            on:click=on_remix
                        >
                            {move || if generating.get() { copy(language.get()).generating } else { copy(language.get()).generate }}
                        </button>
                        {move || error.get().map(|message| view! {
                            <div class="rounded-sm border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive">
                                <span class="font-semibold">{copy(language.get()).error}": "</span>
                                {message}
                            </div>
                        })}
                    </section>
                </div>

                <section class="mt-12 flex flex-col gap-6">
                    <div class="flex items-center justify-between">
                        <SectionMark index="03" title=Signal::derive(move || copy(language.get()).result_title.to_string()) hint=Signal::derive(move || copy(language.get()).result_hint.to_string()) />
                        {move || generated.get().map(|src| view! {
                            <a href=src download=stamp("poster-gen-remix") class="inline-flex items-center gap-1 text-sm font-medium text-primary">
                                {copy(language.get()).download}
                            </a>
                        })}
                    </div>
                    <div class="flex min-h-[480px] w-full items-center justify-center overflow-hidden rounded-sm border border-border bg-card/60 p-6">
                        {move || if let Some(src) = generated.get() {
                            view! { <img src=src alt="Generated advertisement" class="max-h-[700px] max-w-full rounded-sm object-contain shadow-lg" /> }.into_any()
                        } else if generating.get() {
                            view! {
                                <div class="flex flex-col items-center gap-4 text-muted-foreground">
                                    <p class="font-serif text-base italic">{copy(language.get()).composing}</p>
                                    <p class="text-xs">{copy(language.get()).composing_hint}</p>
                                </div>
                            }.into_any()
                        } else {
                            view! { <p class="font-serif text-base italic text-muted-foreground">{copy(language.get()).ready}</p> }.into_any()
                        }}
                    </div>
                </section>
            </main>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PeopleMode {
    Upload,
    Generate,
}

fn mode_tab(active: bool) -> &'static str {
    if active {
        "rounded-sm bg-card px-3 py-2 text-sm text-foreground shadow-sm"
    } else {
        "rounded-sm px-3 py-2 text-sm text-muted-foreground hover:text-foreground"
    }
}

fn style_tab(active: bool) -> &'static str {
    if active {
        "rounded-sm border border-primary/60 bg-accent/70 px-4 py-3 text-left text-sm text-accent-foreground"
    } else {
        "rounded-sm border border-border bg-card px-4 py-3 text-left text-sm text-secondary-foreground hover:bg-secondary/60"
    }
}

fn parse_provider(value: &str) -> Option<ProviderId> {
    match value {
        "grok" => Some(ProviderId::Grok),
        "gemini" => Some(ProviderId::Gemini),
        _ => None,
    }
}

fn stamp(prefix: &str) -> String {
    format!("{prefix}-{}.png", js_sys::Date::now() as u64)
}

#[component]
fn SectionMark(
    index: &'static str,
    title: Signal<String>,
    hint: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-baseline gap-3">
            <span class="font-mono text-xs text-primary/70">{index}</span>
            <div class="flex items-baseline gap-2">
                <h2 class="font-serif text-lg leading-none text-foreground">{move || title.get()}</h2>
                <span class="text-xs text-muted-foreground">{move || hint.get()}</span>
            </div>
        </div>
    }
}

#[component]
fn Field(label: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium uppercase tracking-wider text-muted-foreground">{move || label.get()}</label>
            {children()}
        </div>
    }
}

#[component]
fn ChoiceSelect(
    choices: &'static [Choice],
    language: RwSignal<Language>,
    value: Signal<String>,
    on_change: impl Fn(String) + 'static + Copy,
) -> impl IntoView {
    view! {
        <select class=SELECT prop:value=move || value.get() on:change=move |ev| on_change(event_target_value(&ev))>
            {choices.iter().copied().map(|choice| view! {
                <option value=choice.value>{move || choice.label(language.get())}</option>
            }).collect_view()}
        </select>
    }
}

#[component]
fn Uploader(
    label: Signal<String>,
    image: RwSignal<Option<String>>,
    required: bool,
    language: RwSignal<Language>,
) -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let pick = move |_| {
        if let Some(input) = input_ref.get() {
            input.click();
        }
    };
    let on_file = move |ev: ev::Event| {
        let input: web_sys::HtmlInputElement = event_target(&ev);
        let Some(files) = input.files() else { return };
        let Some(file) = files.get(0) else { return };
        let file = gloo_file::File::from(file);
        spawn_local(async move {
            if let Ok(data) = gloo_file::futures::read_as_data_url(&file).await {
                image.set(Some(compress_image(data).await));
            }
        });
    };
    let remove = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        image.set(None);
        if let Some(input) = input_ref.get() {
            input.set_value("");
        }
    };

    view! {
        <div class="flex w-full flex-col gap-2">
            {move || {
                let label = label.get();
                (!label.is_empty()).then(|| view! {
                    <label class="text-sm font-medium text-foreground">
                        {label}
                        {required.then(|| view! { <span class="text-primary">" *"</span> })}
                    </label>
                })
            }}
            <div
                on:click=pick
                class=move || if image.get().is_some() {
                    "group relative flex h-40 w-full cursor-pointer flex-col items-center justify-center overflow-hidden rounded-sm border border-dashed border-primary/60 bg-accent/40"
                } else {
                    "group relative flex h-40 w-full cursor-pointer flex-col items-center justify-center overflow-hidden rounded-sm border border-dashed border-border bg-secondary/40 hover:border-primary/50 hover:bg-accent/40"
                }
            >
                {move || image.get().map(|src| view! {
                    <div class="relative h-full w-full p-2">
                        <img src=src alt="" class="h-full w-full rounded-sm object-contain" />
                        <button
                            type="button"
                            title=move || copy(language.get()).remove
                            class="absolute right-2 top-2 inline-flex size-7 items-center justify-center rounded-full bg-foreground/80 text-background"
                            on:click=remove
                        >
                            "×"
                        </button>
                    </div>
                })}
                {move || image.get().is_none().then(|| view! {
                    <p class="text-sm text-muted-foreground">{copy(language.get()).drag_drop}</p>
                })}
                <input
                    node_ref=input_ref
                    type="file"
                    class="hidden"
                    accept="image/png, image/jpeg, image/webp"
                    on:change=on_file
                />
            </div>
        </div>
    }
}
