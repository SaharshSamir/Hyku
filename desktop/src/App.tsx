import {clipboard} from "@tauri-apps/api";
import {CopiedTextItem} from "./components/CopiedTextItem";
import {initSystemTray} from "./core";

function App() {
    return (
        <div className="container mx-auto flex justify-center p-6">
            <CopiedTextItem
                text={"My text"}
                onCopy={(text) => clipboard.writeText(text)}
            />

            <button
                className="bg-purple-500 ring-4 ring-purple-400 text-white rounded-md w-full py-2 mt-4 hover:bg-opacity-90 active:bg-purple-500"
                onClick={() => initSystemTray()}>init sys tray
                Copy
            </button>
        </div>
    );
}

export default App;
