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
use std::collections::HashMap;

/// Models a piece of data keyed (uniquely identified) by some `key` function
pub trait KeyedData {
    fn key(&self) -> Cow<'_, String>;
    fn data(&self, cx: Scope) -> View;
}

const BIG_DATA_SIZE: usize = 8 * 1024;

/// Models a row of data which includes a key and the data portion
#[derive(Debug)]
pub struct Row {
    /// The unique identifier for the data
    key: String,
    /// The value associated with the row
    pub value: String,
    // Just to feel the impact of large rows and the effect of cloning, additional data
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
            big_data: self.big_data,
        }
    }
}

impl KeyedData for Row {
    /// Return the unique identifier
    fn key(&self) -> Cow<'_, String> {
        Cow::Borrowed(&self.key)
    }

    /// Creates a view of the data for a row
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
    use leptos::SignalUpdate;
    use leptos::SignalUpdateUntracked;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;
    use std::collections::HashMap;
    use std::rc::Rc;

    let signals = rows
        .iter()
        .enumerate()
        .map(|(i, row)| (row.key().into_owned(), create_rw_signal(cx, i)))
        .collect::<HashMap<String, RwSignal<usize>>>();

    let entries = create_rw_signal(cx, rows);

    let signals = store_value(cx, signals);

    // Reactive number of elements
    let num_elements = move || entries.with(|entries| entries.len());

    // Get the nth key by accessing the vector in a non-reactive way to get the element
    // and then get a copy of the key
    let nth_key = move |n: usize| {
        entries.with_untracked(|entries| entries.get(n).map(|entry| entry.key.clone()))
    };

    // Reactively grab the signal associated with a key. Note the convenient clone of
    // the signal which is just a few id's used to lookup the actual data in leptos
    let key_signal =
        move |key: &String| signals.with_value(|signals| signals.get(key).unwrap().clone());

    // Delete a row by key. In addition to deleting the row it patches the index of all
    // rows after the row identified by key. Since those rows will shift down to accommodate
    // the removal, their indices must be updated.
    let delete_by_key = move |key: &String| {
        if let Some(position) =
            signals.with_value(|signals| signals.get(key).cloned().map(|signal| signal.get()))
        {
            entries.update(|entries| {
                signals.update_value(|signals| {
                    entries.remove(position);
                    let len = entries.len();
                    let elements_after = &entries[position..len];
                    for (i, element) in elements_after.iter().enumerate() {
                        if let Some(entry) = signals.get_mut(&element.key) {
                            entry.update_untracked(|e| *e = position + i);
                        }
                    }
                });
            });
        }
    };

    // A counter to label rows that are added so can have distinct keys.
    let (add_item_counter, set_add_item_counter) = create_signal(cx, 1);

    view! { cx,
        <h3>{move || { format!("There are {} entries", num_elements()) }}</h3>
        <button on:click=move |_| {
            let next_i = num_elements();
            entries
                .update(|entries| {
                    let count = add_item_counter.get();
                    let key = format!("auto-added key {}", count);
                    entries.push(Row::new(&key, &format!("value {}", count)));
                    signals
                        .update_value(|signals| {
                            signals.insert(key, create_rw_signal(cx, next_i));
                        });
                    set_add_item_counter.update(|i| *i += 1);
                })
        }>"Add Row"</button>
        <hr/>
        <For
            each=move || 0..num_elements()
            key=move |&i| nth_key(i)
            view=move |cx, i| {
                let key = Rc::new(nth_key(i).unwrap());
                let key_for_update = Rc::clone(&key);
                let key_for_delete = Rc::clone(&key);
                let key_for_display = Rc::clone(&key);
                view! { cx,
                    <div>
                        <button on:click=move |_| {
                            delete_by_key(&key_for_delete);
                        }>"Delete Me"</button>
                        <button on:click=move |_| {
                            let key_signal = key_signal(&key_for_update);
                            let reactive_i = key_signal.get();
                            entries
                                .update_untracked(|entries| {
                                    if let Some(entry) = entries.get_mut(reactive_i) {
                                        entry.value.push_str(".");
                                    }
                                });
                            key_signal.update(|_| {});
                        }>"Update Me"</button>
                        <span style="padding-left: 10px;">
                            {move || {
                                key_signal(&key_for_display)
                                    .with(|&i| {
                                        entries
                                            .with_untracked(|entries| {
                                                if let Some(row) = entries.get(i) {
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
