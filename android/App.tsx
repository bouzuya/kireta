import { StatusBar } from "expo-status-bar";
import { useCallback, useState } from "react";
import { Button, StyleSheet, Text, View } from "react-native";
import { WasmContextProvider, useWasm } from "./components/WasmContextProvider";
import { useConfig } from "./hooks/useConfig";

function MyApp(): JSX.Element {
  const { call } = useWasm();
  const [count, setCount] = useState<number>(0);
  const handleOnPress = useCallback((): void => {
    (async () => {
      const result = await call("add", [count, 1]);
      setCount(result as number);
    })();
  }, [count]);
  return (
    <View style={styles.container}>
      <StatusBar style="auto" />
      <Text>{count}</Text>
      <Button onPress={handleOnPress} title="Increment" />
    </View>
  );
}

export default function App(): JSX.Element {
  const { backendBaseUrl } = useConfig();
  const uri = `${backendBaseUrl}/assets/index.html`;
  return (
    <WasmContextProvider uri={uri}>
      <MyApp />
    </WasmContextProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    alignContent: "flex-start",
    alignItems: "stretch",
    display: "flex",
    flexDirection: "column",
    height: "100%",
    justifyContent: "flex-start",
    margin: 0,
    padding: 0,
    width: "100%",
  },
});
