use leptos::{create_resource, create_rw_signal, create_signal, ReadSignal, RwSignal, WriteSignal};

pub fn use_signal<T>(s: T) -> (ReadSignal<T>, WriteSignal<T>) {
    create_signal(s)
}

pub fn use_rw_signal<T>(s: T) -> RwSignal<T> {
    create_rw_signal(s)
}

// pub fn use_resource<T>(s: T) -> T {
//     create_resource(s)
// }
