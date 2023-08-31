import { randomUUID } from "expo-crypto";
import {
  RefObject,
  createContext,
  createRef,
  useCallback,
  useContext,
  useRef,
} from "react";
import { StyleSheet } from "react-native";
import WebView, { WebViewMessageEvent } from "react-native-webview";

type Call = (name: string, args: unknown[]) => Promise<unknown>;
type Calls = Record<string, Deferred | undefined>;

type Deferred = {
  promise: Promise<unknown>;
  reject: (reason: unknown) => void;
  resolve: (value: unknown) => void;
};

function defer(): Deferred {
  let reject: ((reason: unknown) => void) | null = null;
  let resolve: ((value: unknown) => void) | null = null;
  const promise = new Promise((ok, ng) => {
    reject = ng;
    resolve = ok;
  });
  if (reject === null) throw new Error("outerReject is null");
  if (resolve === null) throw new Error("outerResolve is null");
  return { promise, reject, resolve };
}

const WasmContext = createContext<{
  call: Call;
  calls: RefObject<Calls>;
}>({ call: () => Promise.resolve(null), calls: createRef<Calls>() });

export function useWasm(): {
  call: Call;
} {
  const { call } = useContext(WasmContext);
  return { call };
}

type Props = {
  children: React.ReactNode;
  uri: string;
};

export function WasmContextProvider({ children, uri }: Props): JSX.Element {
  const ref = useRef<WebView>(null);
  const calls = useRef<Calls>({});
  const handleOnMessage = useCallback(
    (event: WebViewMessageEvent): void => {
      const message = JSON.parse(event.nativeEvent.data);
      console.log("onMessage", message);
      const call = calls.current[message.id];
      if (typeof call === "undefined") throw new Error("Unknown call");
      call.resolve(message.result);
      calls.current[message.id] = undefined;
    },
    [calls]
  );
  const call = useCallback(
    (name: string, args: unknown[]): Promise<unknown> => {
      const id = randomUUID();
      const message = {
        id,
        name,
        args,
      };
      const deferred = defer();
      calls.current[id] = deferred;
      console.log("postMessage", message);
      ref.current?.postMessage(JSON.stringify(message));
      return deferred.promise;
    },
    [ref]
  );
  return (
    <WasmContext.Provider value={{ call, calls }}>
      <WebView
        cacheEnabled={false}
        containerStyle={styles.webView}
        onMessage={handleOnMessage}
        ref={ref}
        source={{ uri }}
      />
      {children}
    </WasmContext.Provider>
  );
}

const styles = StyleSheet.create({
  webView: {
    display: "none",
  },
});
