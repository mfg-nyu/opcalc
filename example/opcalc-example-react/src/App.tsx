import React, { useEffect, useMemo, useState } from "react";
import "./App.css";

const getSecondTimestamp = (dt: Date): number => {
  return Math.floor(dt.getTime() / 1000);
};

interface OptionDefinition {
  assetPrice: number;
  strike: number;
  volatility: number;
  interest: number;
  currTime: Date;
  expiryTime: Date;
}

interface OptionOutputs {
  value: number;
  delta: number;
  gamma: number;
  vega: number;
  theta: number;
}

const useOpCalc = (initialOptionDef: OptionDefinition) => {
  type OpCalcType = typeof import("opcalc");
  const [opcalc, setOpCalc] = useState<OpCalcType | undefined>(undefined);
  const [optionDef, setOptionDef] = useState<OptionDefinition>(
    initialOptionDef
  );

  // load opcalc on launch
  useEffect(() => {
    import("opcalc")
      .then((instance) => setOpCalc(instance))
      .catch((e) => console.error(e));
  }, []);

  const outputs = useMemo<
    { call: OptionOutputs; put: OptionOutputs } | undefined
  >(() => {
    if (!opcalc) return;

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
      const option = opcalc.WasmBSOptionBuilder.new()
        .with_asset_price(assetPrice)
        .with_strike(strike)
        .with_volatility(volatility)
        .with_interest(interest)
        .with_current_time(getSecondTimestamp(currTime))
        .with_maturity_time(getSecondTimestamp(expiryTime))
        .create();

      return {
        call: {
          value: option.call_value(),
          delta: option.call_delta(),
          gamma: option.call_gamma(),
          vega: option.call_vega(),
          theta: option.call_theta(),
        },
        put: {
          value: option.put_value(),
          delta: option.put_delta(),
          gamma: option.put_gamma(),
          vega: option.put_vega(),
          theta: option.put_theta(),
        },
      };
    } catch (error) {
      console.error(error);
      return;
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

  const DECIMAL_COUNT = 5;

  return (
    <div className="App">
      <main>
        <section className="demo-body">
          <h1>
            <code className="header">OpCalc</code>
          </h1>

          <div className="option-entry">
            <span className="label">Call value</span>
            <span className="content">
              {outputs?.call.value.toFixed(DECIMAL_COUNT)}
            </span>
          </div>
          <div className="option-entry">
            <span className="label">Put value</span>
            <span className="content">
              {outputs?.put.value.toFixed(DECIMAL_COUNT)}
            </span>
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
