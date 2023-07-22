//! Module for for_test_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use super::for_test_component::{KeyedData, Row};
#[allow(unused_imports)]
use leptos::log;
use leptos::tracing;
use leptos::{component, view, IntoView, RwSignal, Scope};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Used to figure out how the <For...> component works
///
///   * **cx** - Context
///   * **rows** - The collection of data
///   * _return_ - View for for_test_component
#[component]
pub fn ForTestEnumerateComponent(
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

    let signals = store_value(
        cx,
        rows.iter()
            .enumerate()
            .map(|(i, _)| create_rw_signal(cx, i))
            .collect::<Vec<_>>(),
    );

    let rows = create_rw_signal(cx, rows);

    view! { cx,
        <button on:click=move |_| {
            log!("Adding entry");

        }>"Add String"</button>
        <hr/>
        <For
            each=move || { rows.get().into_iter().enumerate() }
            key=move |(_, row)| { row.key().into_owned() }
            view=move |cx, (i, row)| {
                view! { cx,
                    <div>
                        <button on:click=move |_| {
                            log!("Delete clicked on {row:?}");
                        }>"Delete Me"</button>
                        <button on:click=move |_| {
                            log!("Updating {i}");
                            rows.update_untracked(|rows| {
                                if let Some(row) = rows.get_mut(i) {
                                    log!("Component cloning {i}");
                                    row.value.push_str(".");
                                }
                            });
                            let signal = signals.with_value(|signals| signals.get(i).unwrap().clone());
                            log!("Signaling {i}");
                            signal.update(|i| *i = *i);
                        }>"Update Me"</button>
                        <span style="padding-left: 10px;">
                            {move || {
                                //log!("(Re)display {i}");
                                let signal = signals.with_value(|signals| signals.get(i).unwrap().clone());
                                signal.track();
                                rows.with_untracked(|rows| rows.get(i).unwrap().data(cx))
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
