import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";

enum LoadStatus {
  NotLoaded,
  Loaded,
  LoadErred,
}

/**
 * Imports a wasm module, or signals error if an error has occured during import.
 *
 * @typeParam T the type of module to be imported.
 * @param loadPath Specify the path where wasm module should be imported from
 *   (e.g. 'opcalc').
 */
const useWasm = function <T>(loadPath: string) {
  const [state, setState] = useState<{
    instance: T | undefined;
    loadStatus: LoadStatus;
    error: Error | undefined;
  }>({
    instance: undefined,
    loadStatus: LoadStatus.NotLoaded,
    error: undefined,
  });

  useEffect(() => {
    import(loadPath)
      .then((instance) => {
        setState({ instance, loadStatus: LoadStatus.Loaded, error: undefined });
      })
      .catch((e) => {
        console.error(e);
        setState((prevState) => ({
          ...prevState,
          error: e,
          loadStatus: LoadStatus.LoadErred,
        }));
      });
  }, [loadPath]);

  return {
    instance: state.instance,
    error: state.error,
    loaded: state.loadStatus === LoadStatus.Loaded,
    loadErred: state.loadStatus === LoadStatus.LoadErred,
  };
};

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
