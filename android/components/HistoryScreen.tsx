import { StackActions, useNavigation } from "@react-navigation/native";
import React from "react";
import { StyleSheet, View } from "react-native";
import { Button } from "react-native-paper";

export function HistoryScreen(): JSX.Element {
  const navigation = useNavigation();
  return (
    <View style={styles.container}>
      <Button
        onPress={() => { navigation.dispatch(StackActions.replace("TodayScreen")); }}
      >
        Today
      </Button>
      <Button
        onPress={() => { navigation.dispatch(StackActions.push("ListScreen")); }}
      >
        2023-09-06
      </Button>
      <Button
        onPress={() => { navigation.dispatch(StackActions.push("ListScreen")); }}
      >
        2023-09-05
      </Button>
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
});
