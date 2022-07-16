import { clipboard } from "@tauri-apps/api";
import { CopiedTextItem } from "./components/CopiedTextItem";

function App() {
  return (
    <div className="container mx-auto flex justify-center p-6">
      <CopiedTextItem
        text={"My text"}
        onCopy={(text) => clipboard.writeText(text)}
      />
    </div>
  );
}

export default App;
