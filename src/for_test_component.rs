//! Module for for_test_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::tracing;
use leptos::{component, view, IntoView, RwSignal, Scope, View};
use std::borrow::Cow;
use std::clone::Clone;

pub trait KeyedData {
    fn key(&self) -> Cow<'_, String>;
    fn data(&self, cx: Scope) -> View;
}

const BIG_DATA_SIZE: usize = 8*1024;

#[derive(Debug)]
pub struct Row {
    key: String,
    pub value: String,
    big_data: [u32; BIG_DATA_SIZE],
}

impl Row {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            big_data: [1; BIG_DATA_SIZE],
        }
    }
}

impl Clone for Row {
    fn clone(&self) -> Self {
        log!("Cloning {}", self.key);
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
            big_data: self.big_data
        }
    }
}

impl KeyedData for Row {
    fn key(&self) -> Cow<'_, String> {
        Cow::Borrowed(&self.key)
    }

    fn data(&self, cx: Scope) -> View {
        view! { cx,
            <span>
                {
                    let mut text = self.key.clone();
                    text.push_str(" -> ... ");
                    text.push_str(&self.value);
                    text
                }
            </span>
        }
        .into_view(cx)
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Used to figure out how the <For...> component works
///
///   * **cx** - Context
///   * **rows** - The collection of data
///   * _return_ - View for for_test_component
#[component]
pub fn ForTestComponent(
    /// Context
    cx: Scope,
    /// The collection of data
    rows: Vec<Row>,
) -> impl IntoView {
    // α <fn for_test_component>

    tracing::debug!("This should go somewhere");
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::For;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalUpdate;
    use leptos::SignalUpdateUntracked;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    let mut signals = HashMap::<String, RwSignal<usize>>::with_capacity(rows.len());

    let entries = {
        rows.iter().enumerate().for_each(|(i, s)| {
            signals.insert(s.key().into_owned(), create_rw_signal(cx, i));
        });
        create_rw_signal(cx, Rc::new(RefCell::new(rows)))
    };

    let num_elements = move || entries.with(|entries| entries.borrow().len());

    let nth_key = move |n: usize| {
        entries.with_untracked(|entries| entries.borrow().get(n).map(|entry| entry.key.clone()))
    };

    let signals = store_value(cx, signals);

    let key_signal =
        move |key: &String| signals.with_value(|signals| signals.get(key).unwrap().clone());

    let delete_by_key = move |key: &String| {
        let position = entries
            .with_untracked(|entries| entries.borrow().iter().position(|entry| &entry.key == key));
        if let Some(position) = position {
            entries.update(|entries| {
                //let index_changes = (position..(entries.len()-1)).iter().map(|i| )
                signals.update_value(|signals| {
                    let mut entries = entries.as_ref().borrow_mut();
                    entries.remove(position);
                    let len = entries.len();
                    let elements_after = &entries[position..len];
                    for (i, element) in elements_after.iter().enumerate() {
                        if let Some(entry) = signals.get_mut(&element.key) {
                            log!(
                                "Updating i from {:?} to {}",
                                entry.get_untracked(),
                                position + i
                            );
                            entry.update_untracked(|e| *e = position + i);
                        }
                    }
                });
            });
        }
    };

    let i_by_key =
        move |key: &String| signals.with_value(|signals| signals.get(key).unwrap().get_untracked());

    let (counter, set_i) = create_signal(cx, 1);

    view! { cx,
        <h3>{move || { format!("There are {} entries", num_elements()) }}</h3>
        <button on:click=move |_| {
            log!("Adding entry");
            let next_i = num_elements();
            entries
                .update(|entries| {
                    let key = format!("auto-added key {}", counter.get());
                    entries
                        .borrow_mut()
                        .push(Row::new(&key, &format!("value {}", counter.get())));
                    signals
                        .update_value(|signals| {
                            signals.insert(key, create_rw_signal(cx, next_i));
                        });
                    set_i.update(|i_| *i_ += 1);
                })
        }>"Add String"</button>
        <hr/>
        <For
            each=move || { 0..num_elements() }
            key=move |&i| { nth_key(i) }
            view=move |cx, i| {
                let key = Rc::new(nth_key(i).unwrap());
                let key_for_update = Rc::clone(&key);
                let key_for_delete = Rc::clone(&key);
                let key_for_display = Rc::clone(&key);
                view! { cx,
                    <div>
                        <button on:click=move |_| {
                            log!("Delete clicked on {:?}", key);
                            delete_by_key(&key_for_delete);
                        }>"Delete Me"</button>
                        <button on:click=move |_| {
                            let i = i_by_key(&*key_for_update);
                            log!("Updating {i}");

                            entries.with_untracked(|entries| {
                                if let Some(entry) = entries.borrow_mut().get_mut(i) {
                                    entry.value.push_str(".");
                                }
                                key_signal(&key_for_update).update(|_| {});
                            });
                        }>"Update Me"</button>
                        <span style="padding-left: 10px;">
                            {move || {
                                key_signal(&key_for_display)
                                    .with(|&i| {
                                        //log!("(Re)display {i}");
                                        entries
                                            .with_untracked(|entries| {
                                                if let Some(row) = entries.borrow().get(i) {
                                                    row.data(cx)
                                                } else {
                                                    view! { cx, <h4>"Error"</h4> }
                                                        .into_view(cx)
                                                }
                                            })
                                    })
                            }}
                        </span>
                    </div>
                }
                    .into_view(cx)
            }
        />
    }

    // ω <fn for_test_component>
}

// α <mod-def for_test_component>
// ω <mod-def for_test_component>
