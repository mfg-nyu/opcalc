import React, { useEffect, useState } from "react";
import "./App.css";

const getSecondTimestamp = (dt: Date): number => {
  return Math.floor(dt.getTime() / 1000);
};

type OpCalcType = typeof import("opcalc");

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

function computeOptionOutputs(
  input: OptionDefinition,
  opcalc: OpCalcType
): Promise<CallPutOutputs> {
  return new Promise((resolve, reject) => {
    try {
      const option = opcalc
        .create_option()
        .with_asset_price(input.assetPrice)
        .with_strike(input.strike)
        .with_volatility(input.volatility)
        .with_interest(input.interest)
        .with_current_time(getSecondTimestamp(input.currTime))
        .with_maturity_time(getSecondTimestamp(input.expiryTime))
        .finalize();

      resolve({
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
      });
    } catch (e) {
      reject(e);
    }
  });
}

enum OpCalcLoadStatus {
  NotLoaded,
  Loaded,
  LoadErred,
}

/** Load the `opcalc` module, or report loading error status. */
const useOpCalcModuleImport = () => {
  const [{ instance: opcalc, loadStatus }, setOpCalcModule] = useState<{
    instance: OpCalcType | undefined;
    loadStatus: OpCalcLoadStatus;
  }>({ instance: undefined, loadStatus: OpCalcLoadStatus.NotLoaded });

  // use the 'opcalc' string literal here to prevent a webpack bundle splitting
  // bailout. Imports with variable path names cannot be statically analyzed
  // and therefore cannot be used for webpack optimizations.
  useEffect(() => {
    import("opcalc")
      .then((instance) =>
        setOpCalcModule({ instance, loadStatus: OpCalcLoadStatus.Loaded })
      )
      .catch((e) => {
        console.error(e);
        setOpCalcModule({
          instance: undefined,
          loadStatus: OpCalcLoadStatus.LoadErred,
        });
      });
  }, []);

  return {
    opcalc,
    loadStatus: {
      loaded: loadStatus === OpCalcLoadStatus.Loaded,
      erred: loadStatus === OpCalcLoadStatus.LoadErred,
    },
  };
};

const useOpCalc = (initialOptionInput: OptionDefinition) => {
  const { opcalc, loadStatus } = useOpCalcModuleImport();

  const [optionInput, updateInput] = useState<OptionDefinition>(
    initialOptionInput
  );

  const [optionOutputs, setOptionOutputs] = useState<
    CallPutOutputs | undefined
  >(undefined);

  useEffect(() => {
    if (!opcalc) return;

    computeOptionOutputs(optionInput, opcalc)
      .then((res) => setOptionOutputs(res))
      .catch((e) => console.error(e));
  }, [opcalc, optionInput]);

  return {
    currentOptionInput: optionInput,
    updateInput,
    outputs: optionOutputs,
    loadStatus,
  };
};

const initialOptionInput: OptionDefinition = {
  assetPrice: 100,
  strike: 105,
  volatility: 0.23,
  interest: 0.005,
  currTime: new Date(2020, 11, 1),
  expiryTime: new Date(2021, 0, 15),
};

function App() {
  const { outputs, loadStatus, currentOptionInput, updateInput } = useOpCalc(
    initialOptionInput
  );

  return (
    <div className="App">
      <main>
        <section className="demo-body">
          <h1>
            <code className="header">OpCalc</code>
          </h1>

          <OptionInput input={currentOptionInput} onChange={updateInput} />

          {!loadStatus.loaded ? (
            <Loading />
          ) : loadStatus.erred ? (
            <LoadErred />
          ) : (
            <OutputTable data={outputs} />
          )}
        </section>
      </main>
    </div>
  );
}

const Loading: React.FC = () => {
  return <span className="user-message">Loading option calculator...</span>;
};

const LoadErred: React.FC = () => {
  return (
    <span className="user-message">
      An error has occurred during calculation.
    </span>
  );
};

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
  min = Number.NEGATIVE_INFINITY,
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
          <DecimalCell value={data?.call.value} />
          <DecimalCell value={data?.call.delta} />
          <DecimalCell value={data?.call.gamma} />
          <DecimalCell value={data?.call.vega} />
          <DecimalCell value={data?.call.theta} />
        </tr>

        <tr>
          <th>Put</th>
          <DecimalCell value={data?.put.value} />
          <DecimalCell value={data?.put.delta} />
          <DecimalCell value={data?.put.gamma} />
          <DecimalCell value={data?.put.vega} />
          <DecimalCell value={data?.put.theta} />
        </tr>
      </tbody>
    </table>
  );
};

const DecimalCell: React.FC<{
  value: number | undefined;
  decimalCount?: number;
}> = ({ value, decimalCount = 5 }) => {
  return <td>{value?.toFixed(decimalCount)}</td>;
};

export default App;
