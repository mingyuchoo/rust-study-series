import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import './App.css';

function App() {
  const [ answer, setAnswer ] = useState(0);
  useEffect(() => {
      init().then(() => {
          setAnswer(add(1, 1));
      })
  }, []);

  return (
    <div className="App">
      <p>
        1 + 1 = {answer}
      </p>
    </div>
  );
}

export default App;
