use yew::{prelude::*, Renderer};

fn main() {
    Renderer::<Index>::new().render();
}

#[function_component]
fn Index() -> Html {
    let counter = use_state(|| 0);
    let add_one_to_counter = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1);
        }
    };
    let remove_one_to_counter = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter - 1);
        }
    };

    html!(
        <div>
            <h1 class={"title"}>{"The best JS framework is written in Rust!"}</h1>
            <button onclick={add_one_to_counter} class={"btn"}>{"+ 1"}</button>
            <button onclick={remove_one_to_counter} class={"btn"}>{"- 1"}</button>
            <p>{*counter}</p>
            <a href={"https://yew.rs"}>{"Yew"}</a>

            <style>
            { "
                @import url('https://fonts.googleapis.com/css2?family=Fira+Code&display=swap');
                html {
                    text-align: center;
                    font-family: 'Fira Code', monospace;
                }

                .title {
                    margin-top: 20px;
                }

                .btn {
                    display: inline-block;
                    margin: 10px;
                    outline: 0;
                    border: 0;
                    cursor: pointer;
                    background-color: white;
                    border-radius: 4px;
                    padding: 8px 16px;
                    font-size: 16px;
                    font-weight: 600;
                    color: #2d3748;
                    border: 1px solid #cbd5e0;
                    line-height: 26px;
                    box-shadow: 0 1px 3px 0 rgba(0,0,0,.1),0 1px 2px 0 rgba(0,0,0,.06);
                }
             "}
            </style>
        </div>
    )
}
