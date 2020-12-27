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

interface CallPutOutputs {
  call: OptionOutputs;
  put: OptionOutputs;
}

const useOpCalc = (initialOptionDef: OptionDefinition) => {
  type OpCalcType = typeof import("opcalc");
  const [{ instance: opcalc, loadErred }, setOpCalcModule] = useState<
    | { instance: undefined; loadErred: boolean }
    | {
        instance: OpCalcType;
        loadErred: false;
      }
  >({ instance: undefined, loadErred: false });

  const [optionDef, setOptionDef] = useState<OptionDefinition>(
    initialOptionDef
  );

  // load opcalc on launch
  useEffect(() => {
    import("opcalc")
      .then((instance) => setOpCalcModule({ instance, loadErred: false }))
      .catch((e) => {
        console.error(e);
        setOpCalcModule({ instance: undefined, loadErred: true });
      });
  }, []);

  const outputs = useMemo<CallPutOutputs | undefined>(() => {
    if (!opcalc) return;

    const {
      assetPrice,
      strike,
      volatility,
      interest,
      currTime,
      expiryTime,
    } = optionDef;

    try {
      const option = opcalc
        .create_option()
        .with_asset_price(assetPrice)
        .with_strike(strike)
        .with_volatility(volatility)
        .with_interest(interest)
        .with_current_time(getSecondTimestamp(currTime))
        .with_maturity_time(getSecondTimestamp(expiryTime))
        .finalize();

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
    currentOptionDef: optionDef,
    setOptionDef,
    outputs,
    loadErred,
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
  const { outputs, loadErred, currentOptionDef, setOptionDef } = useOpCalc(
    optionDef
  );

  return (
    <div className="App">
      <main>
        <section className="demo-body">
          <h1>
            <code className="header">OpCalc</code>
          </h1>

          <OptionInput input={currentOptionDef} onChange={setOptionDef} />

          {loadErred ? <LoadErred /> : <OutputTable data={outputs} />}
        </section>
      </main>
    </div>
  );
}

const OptionInput: React.FC<{
  input: OptionDefinition;
  onChange: (input: OptionDefinition) => void;
}> = ({ input, onChange: onInputChange }) => {
  const daysBetweenDates = (start: Date, end: Date) => {
    return Math.floor(
      (end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24)
    );
  };

  return (
    <div className="option-input-container">
      <NumericInput
        name="Asset Price"
        input={input.assetPrice}
        min={0}
        onChange={(assetPrice) => onInputChange({ ...input, assetPrice })}
      />

      <NumericInput
        name="Strike Price"
        input={input.strike}
        min={0}
        onChange={(strike) => onInputChange({ ...input, strike })}
      />

      <NumericInput
        name="Interest Rate"
        input={input.interest}
        step={0.001}
        onChange={(interest) => onInputChange({ ...input, interest })}
      />

      <NumericInput
        name="Volatility"
        input={input.volatility}
        step={0.01}
        onChange={(volatility) => onInputChange({ ...input, volatility })}
      />

      <NumericInput
        name="Days to maturity"
        input={daysBetweenDates(input.currTime, input.expiryTime)}
        step={1}
        min={0}
        onChange={(newDayDiff) => {
          const newExpiryTime = new Date(
            input.currTime.getTime() + newDayDiff * 24 * 60 * 60 * 1000
          );
          onInputChange({ ...input, expiryTime: newExpiryTime });
        }}
      />
    </div>
  );
};

const NumericInput: React.FC<{
  name: string;
  input: number;
  min?: number;
  max?: number;
  step?: number;
  onChange: (newValue: number) => unknown;
}> = ({
  name,
  input,
  min = -Infinity,
  max = Number.POSITIVE_INFINITY,
  step = 1,
  onChange,
}) => {
  const formatInputId = (inputName: string) => {
    return "option-input-form__" + inputName.toLowerCase().split(" ").join("-");
  };

  return (
    <div className="form-entry">
      <label htmlFor={formatInputId(name)}>{name}</label>
      <input
        type="number"
        id={formatInputId(name)}
        min={min}
        max={max}
        step={step}
        value={input}
        onChange={(e) => onChange(Number.parseFloat(e.target.value))}
      ></input>
    </div>
  );
};

const LoadErred: React.FC = () => {
  return <span>An error has occurred during calculation.</span>;
};

const OutputTable: React.FC<{ data: CallPutOutputs | undefined }> = ({
  data,
}) => {
  return (
    <table className="output">
      <tbody>
        <tr>
          <th></th>
          <th>Value</th>
          <th>Delta</th>
          <th>Gamma</th>
          <th>Vega</th>
          <th>Theta</th>
        </tr>

        <tr>
          <th>Call</th>
          <DataCell value={data?.call.value} />
          <DataCell value={data?.call.delta} />
          <DataCell value={data?.call.gamma} />
          <DataCell value={data?.call.vega} />
          <DataCell value={data?.call.theta} />
        </tr>

        <tr>
          <th>Put</th>
          <DataCell value={data?.put.value} />
          <DataCell value={data?.put.delta} />
          <DataCell value={data?.put.gamma} />
          <DataCell value={data?.put.vega} />
          <DataCell value={data?.put.theta} />
        </tr>
      </tbody>
    </table>
  );
};

const DataCell: React.FC<{ value: number | undefined }> = ({ value }) => {
  const DECIMAL_COUNT = 5;

  return <td>{value?.toFixed(DECIMAL_COUNT)}</td>;
};

export default App;
