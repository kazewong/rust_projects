import './App.css'
import init, {greet} from "../pkg/test_wasm.js"

function App() {

  init().then(() => {
    greet();
  }
  )

  return (
    <>
    </>
  )
}

export default App
