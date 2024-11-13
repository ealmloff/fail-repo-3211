use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {  
    #[layout(HomeNavBar)]  
    #[route("/")]  
    Home {},  
    #[end_layout]  

    #[layout(LoginNavBar)]  
    #[route("/login")]  
    Login {},  
    #[end_layout]  

    #[route("/dashboard")]  
    Dashboard {},  
}  

fn HomeNavBar() -> Element {
    rsx! {
        div { "HomeNavBar" }
        Outlet::<Route> {}
    }
}

fn LoginNavBar() -> Element {
    rsx! {
        div { "LoginNavBar" }
        Outlet::<Route> {}
    }
}

fn Dashboard() -> Element {
    rsx! {
        div { "Dashboard" }
    }
}

fn Home() -> Element {
    rsx! {
        div { "Home" }
    }
}

fn Login() -> Element {
    rsx! {
        div { "Login" }
    }
}

fn main() {
    launch(|| rsx! { Router::<Route> {} } )
}
