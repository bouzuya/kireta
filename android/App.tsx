import { StatusBar } from "expo-status-bar";
import { useCallback, useRef, useState } from "react";
import { Button, StyleSheet, Text, View } from "react-native";
import WebView, { WebViewMessageEvent } from "react-native-webview";
import Constants from "expo-constants";

export default function App() {
  const [count, setCount] = useState<number>(0);
  const backendBaseUrl = Constants.expoConfig?.extra?.backendBaseUrl ?? null;
  if (backendBaseUrl === null)
    throw new Error("BACKEND_BASE_URL is not defined");
  const ref = useRef<WebView>(null);
  const uri = `${backendBaseUrl}/assets/index.html`;
  const handleOnMessage = useCallback((event: WebViewMessageEvent): void => {
    const message = JSON.parse(event.nativeEvent.data);
    console.log("onMessage", message);
    setCount(message.result);
  }, []);
  const handleOnPress = useCallback((): void => {
    const message = {
      name: "add",
      args: [count, 1],
    };
    console.log("postMessage", message);
    ref.current?.postMessage(JSON.stringify(message));
  }, [count, ref]);
  return (
    <View style={styles.container}>
      <StatusBar style="auto" />
      <WebView
        cacheEnabled={false}
        containerStyle={styles.webView}
        onMessage={handleOnMessage}
        ref={ref}
        source={{ uri }}
      />
      <Text>{count}</Text>
      <Button onPress={handleOnPress} title="Increment" />
    </View>
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
  webView: {
    height: "100%",
    margin: 0,
    padding: 0,
    width: "100%",
  },
});
