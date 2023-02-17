use crate::components::{
    alert::{AlertComponent, Props as AlertProps},
    spinner::Spinner,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::router::{switch, Route};
use crate::store::Store;

#[function_component(App)]
pub fn app() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let is_page_loading = store.page_loading.clone();

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };
    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
                if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
                if is_page_loading {
                    <div class="pt-4 pl-2 top-[5.5rem] fixed">
                        <Spinner width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
                    </div>
                }
        </BrowserRouter>
    }
}
