import init, {greet} from "../pkg";

import './App.css'

function App() {

  init().then((_exports) => {
    greet();
    });
  return (
    <>

    </>
  )
}

export default App
