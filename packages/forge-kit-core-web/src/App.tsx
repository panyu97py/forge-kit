// import {useState} from "react";
// import reactLogo from "./assets/react.svg";
// import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import "@radix-ui/themes/styles.css";
import {Theme, ThemePanel} from "@radix-ui/themes";

function App() {
    // const [greetMsg, setGreetMsg] = useState("");
    // const [name, setName] = useState("");
    //
    // async function greet() {
    //     Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // setGreetMsg(await invoke("greet", {name}));
    // }

    return (
        <main>
            <Theme>
                <ThemePanel/>
            </Theme>
        </main>
    );
}

export default App;
