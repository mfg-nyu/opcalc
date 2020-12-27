import React, { useEffect, useMemo, useState } from "react";
import "./App.css";

enum LoadStatus {
  NotLoaded,
  Loaded,
  LoadErred,
}

const getSecondTimestamp = (dt: Date): number => {
  return Math.floor(dt.getTime() / 1000);
};

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
    import("opcalc")
      .then((instance: any) => {
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

interface OptionDefinition {
  assetPrice: number;
  strike: number;
  volatility: number;
  interest: number;
  currTime: Date;
  expiryTime: Date;
}

const useOpCalc = (initialOptionDef: OptionDefinition) => {
  type OpCalcType = typeof import("opcalc");
  const { instance: opcalc } = useWasm<OpCalcType>("opcalc");

  const [optionDef, setOptionDef] = useState<OptionDefinition>(
    initialOptionDef
  );

  const outputs = useMemo(() => {
    if (!opcalc) return {};
    const {
      assetPrice,
      strike,
      volatility,
      interest,
      currTime,
      expiryTime,
    } = optionDef;

    console.log(optionDef);

    try {
      // const option = opcalc.WasmBSOptionBuilder.new()
      //   .with_asset_price(assetPrice)
      //   .with_strike(strike)
      //   .with_volatility(volatility)
      //   .with_interest(interest)
      //   .with_current_time(getSecondTimestamp(currTime))
      //   .with_maturity_time(getSecondTimestamp(expiryTime))
      //   .create();

      const option = opcalc.BSOption.new(
        getSecondTimestamp(currTime),
        getSecondTimestamp(expiryTime),
        assetPrice,
        strike,
        interest,
        volatility,
        0
      );

      return {
        call: {
          value: option.call_value(),
        },
        put: {
          value: option.put_value(),
        },
      };
    } catch (error) {
      console.error(error);
      return {};
    }
  }, [opcalc, optionDef]);

  return {
    setOptionDef,
    outputs,
  };
};

const optionDef: OptionDefinition = {
  assetPrice: 100,
  strike: 105,
  volatility: 0.23,
  interest: 0.005,
  currTime: new Date(2020, 11, 1),
  expiryTime: new Date(2021, 0, 15),
};

function App() {
  const { outputs } = useOpCalc(optionDef);

  return (
    <div className="App">
      <main>
        <div>
          <div className="option-entry">
            <span className="label">Call value</span>
            <span className="content">{outputs.call?.value}</span>
          </div>
          <div className="option-entry">
            <span className="label">Put value</span>
            <span className="content">{outputs.put?.value}</span>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
