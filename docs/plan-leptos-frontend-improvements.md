# Leptos Frontend Improvements — From `feat/leptos-frontend`

## Source Branch

`feat/leptos-frontend` — contains infinite scroll, Leptos 0.8 API updates, and styling improvements.

## Status

CSS files (`styles.css`, `tailwind.css`) and logo (`assets/logo.png`) are **already identical** to main. No extraction needed.

## Changes to Extract

### 1. Infinite Scroll for Document List

**Current:** Offset-based pagination with manual page buttons.

**From branch:** Resource-based infinite scroll using `leptos-use` scroll detection.

**Key changes in `frontend/src/routes/document.rs`:**

```rust
// Signal pattern (Leptos 0.8)
let (page, page_set) = create_signal(1u32);
let (data, data_set) = create_signal(vec![]);
let (error, error_set) = create_signal(false);

// Resource-based fetching
let documents = create_resource(
    move || page.get(),
    move |page| async move {
        api_call::<Vec<Doc>, FetchError>(
            "api/doc",
            Method::GET,
            None,
            Some(vec![("page", &page.to_string())]),
        )
        .await
    },
);

// Infinite scroll effect
create_effect(move |_| {
    if user_scroll_return.arrived_state.get().bottom && !user_scroll_return.is_scrolling.get() {
        page_set.update(|page| *page = page.saturating_add(1));
        (user_scroll_return.measure)();
        if error.get_untracked() {
            documents.refetch();
        }
    }
});
```

### 2. Abort Controller for Fetch Cleanup

**From `frontend/src/api.rs`:**

```rust
pub async fn api_call<T, P>(
    path: &str,
    method: Method,
    query_params: Option<Vec<(&str, &str)>>,
) -> Result<T, FetchError>
where
    T: Serializable,
    P: Serialize,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(AbortController::signal);

    // abort in-flight requests if we navigate away from this page
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort();
        }
    });

    let mut builder = RequestBuilder::new(&format!("http://localhost:4000/{path}"))
        .method(method)
        .abort_signal(abort_signal.as_ref());
    // ...
}
```

### 3. SignalToggle Trait

**From `frontend/src/lib.rs`:**

```rust
pub trait SignalToggle {
    fn toggle(&mut self);
}

impl<T> SignalToggle for T
where
    T: SignalUpdate<Value = T> + Not<Output = T>,
{
    fn toggle(&mut self) {
        self.update(|&mut mut value| value = !value);
    }
}
```

### 4. Sidebar Uses `<A>` Component

**From `frontend/src/components/sidebar.rs`:**

```rust
pub fn MenuEntry<H: ToHref + 'static>(href: H, #[prop()] label: &'static str) -> impl IntoView {
    view! {
        <li class="list-none">
            <A
                href=href
                class=r"
                transition-all duration-100 ease-in-out
                flex items-center p-2 group
                rounded-md hover:bg-slate-900
                text-white hover:text-amber-600 hover:font-semibold"
            >
                <span class="ml-2">{label}</span>
            </A>
        </li>
    }
}
```

### 5. Upload Component Updates

- `MaybeSignal` instead of `Signal` for props
- `create_node_ref` instead of `NodeRef::new()`
- `Callback::call` instead of direct callback invocation

## Implementation Order

1. Add `SignalToggle` trait to `lib.rs`
2. Update `api.rs` with abort controller (requires `web-sys` feature for `AbortController`)
3. Update `sidebar.rs` to use `<A>` component from `leptos_router`
4. Rewrite `document.rs` with infinite scroll pattern
5. Update `upload.rs` with Leptos 0.8 APIs
