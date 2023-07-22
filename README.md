<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Notes/Questions on `<For...>` Component

## My Requirements

I have a straightforward use-case: _Display items from a collection, allowing
the addition, deletion and edit of the entries._ In doing this the following are
also desired:

- Try to have a single vector of elements, in this case called `Row`, but once
  ironed out a future version will make generic.
- Make all edits to the vector _in-place_.
- Support signalling updates to parent component. This may be via leptos signals
  or simple generic function. That is not the focus here, but parent code will
  give the component a `Vec<Row>` and in the future there will be a way to
  signal out via something like `Fn(&Vec<Row>)` or `FnMut(&Vec<Row>)`.

The following is the documentation for `<For...>`.

> Iterates over children and displays them, KeyedData by the `key` function given.
>
> This is much more efficient than naively iterating over nodes with
> `.iter().map(|n| view! { cx,  ... })...`,
> as it avoids re-creating DOM nodes that are not being changed.
>

That sounds like just what I want. Here is some more of the docs:

```rust
# use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
  id: usize,
  count: RwSignal<i32>
}

#[component]
fn Counters(cx: Scope) -> impl IntoView {
  let (counters, set_counters) = create_signal::<Vec<Counter>>(cx, vec![]);

  view! {
    cx,
    <div>
      <For
        // a function that returns the items we're iterating over; a signal is fine
        each=move || counters.get()
        // a unique key for each item
        key=|counter| counter.id
        // renders each item to a view
        view=move |cx, counter: Counter| {
          view! {
            cx,
            <button>"Value: " {move || counter.count.get()}</button>
          }
        }
      />
    </div>
  }
```

To mimic what I will need to do in a slimmed down example, here is the setup:

```rust

pub trait KeyedData {
    fn key(&self) -> Cow<'_, String>;
}

#[derive(Debug, Clone)]
pub struct Row {
    pub key: String,
    pub value: String,
}

impl KeyedData for Row {
    fn key(&self) -> Cow<'_, String> {
        Cow::Borrowed(&self.key)
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

    let mut rows = rows;
    ...
}
```


In the documented case the first thing to notice is the `each` property is
assigned from a signal using `.get()`. The issue with this approach is that
`.get()` clones the vector of data. The issue with that is not so much the cost
of cloning - but that could be something to think about. What if the content in
the rows are gigantic in terms of memory footprint but what is being displayed
is only a small subset of the row? Ignoring the performance, the more salient
issue for our requirements there is a new copy of the user data that will be
held onto by the framework. Where does that data go? Or more to the point, how
can we correlate updates that we want to make in our `Vec<Row>` data? It looks
like each element iterated has its opportunity to create its own `View`:

```rust
    view=move |cx, counter: Counter| {
        view! {
        cx,
        <button>"Value: " {move || counter.count.get()}</button>
        }
    }
```

That view can be reactive and therefore be updated directly when any tracked
signal like `counter` is updated. I think this can be done without any
modification of the DOM elements except the one being updated. Suppose we try to
mimic this: 

```rust
    let rows = create_signal_rows(cx, rows);
    view!{
        cx,
        <div>
        <For
            each=move || rows.get()
            // a unique key for each item
            key=|row| row.key.clone()
            // renders each item to a view
            view=move |cx, row: Row| {

                let mut editable_row = row;
                // Get all the reactive bits of `Row` to allow edit
                let field_1 = create_signal(cx, row...);
                let field_2 = create_signal(cx, row...);
                let field_3 = create_signal(cx, row...);



                ...
                view! {
                    cx,
                    <input value={field1...} />
                    <input value={field2...} />
                    <input value={field3...} />
                }
            }
        />
        </div>
    }
```

Now the list of elements is shown and each element can be edited. For any
element's `view`, even if it is catching all the events and capable of updating
its local mutable `edit_row` - how can it then signal the parent that the vector
is new/updated? First it would have to clone the updated `edit_row` into the
proper place in the component's mutable `row`. How can it get that place?

The first thought might be to start dealing with enumerated values to get access
to the numeric of the item.



