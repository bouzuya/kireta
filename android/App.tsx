import { StatusBar } from "expo-status-bar";
import { useRef } from "react";
import { StyleSheet, Text, View } from "react-native";
import WebView from "react-native-webview";
import Constants from "expo-constants";

export default function App() {
  const backendBaseUrl = Constants.expoConfig?.extra?.backendBaseUrl ?? null;
  if (backendBaseUrl === null)
    throw new Error("BACKEND_BASE_URL is not defined");
  const ref = useRef<WebView>(null);
  const uri = `${backendBaseUrl}/assets/index.html`;
  return (
    <View style={styles.container}>
      <StatusBar style="auto" />
      <WebView
        cacheEnabled={false}
        ref={ref}
        source={{ uri }}
        containerStyle={styles.webView}
      />
      <Text>Open up App.tsx to start working on your app!</Text>
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
