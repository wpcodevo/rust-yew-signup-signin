use crate::{
    api::user_api::api_user_info,
    components::header::Header,
    router,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let response = api_user_info().await;
                match response {
                    Ok(user) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(Some(user), dispatch);
                    }
                    Err(e) => {
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                        navigator.push(&router::Route::LoginPage);
                    }
                }
            });
        },
        (),
    );

    html! {
    <>
      <Header />
      <section class="bg-ct-blue-600 min-h-screen pt-20">
        <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
          <div>
            <p class="text-5xl font-semibold">{"Profile Page"}</p>
            if let Some(user) = user {
              <div class="mt-8">
                <p class="mb-4">{format!("ID: {}", user.id)}</p>
                <p class="mb-4">{format!("Name: {}", user.name)}</p>
                <p class="mb-4">{format!("Email: {}", user.email)}</p>
                <p class="mb-4">{format!("Role: {}", user.role)}</p>
              </div>
            }else {
              <p class="mb-4">{"Loading..."}</p>
            }
          </div>
        </div>
      </section>
    </>
    }
}
